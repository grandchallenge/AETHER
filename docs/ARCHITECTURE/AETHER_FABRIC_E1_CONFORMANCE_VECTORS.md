# AETHER/FABRIC E1 Conformance Vectors

Status: E1 candidate, Revision 1 after Formalist review
Issue: #85
Protocol: `aether-fabric/1.0`

## 1. Purpose

Define concrete specification vectors for the later E2 executable harness.
Every mechanically realized vector uses an explicit `mechanical_attempt_id`.
Every AETHER semantic lifecycle uses `semantic_attempt_id`.

Notation:

```text
+ EVENT(...)   valid event
! EVENT(...)   event/record that must be rejected or quarantined
=>             expected result
!=>            forbidden implication
```

## V01 — valid pre-start realization

```text
+ MechanicalEnvelopeAuthorized(E1, authorization_ref=A1, scope_ref=SR1,
  scope_digest=sha256(bytes(SR1)), permitted_actions=[dispatch_payload], ...)
+ RouteRealized(E1, mechanical_attempt=M1, endpoint=EP1)
+ QueueAdmitted(E1, mechanical_attempt=M1, endpoint=EP1)
```

Expected: mechanical `queue_admitted`; semantic `not_started`.

Forbidden:

```text
QueueAdmitted(M1) !=> SemanticExecutionStarted
QueueAdmitted(M1) !=> SemanticAdmissionAccepted
```

## V02 — delivery does not admit

```text
+ RouteRealized(E2,M2,EP2)
+ QueueAdmitted(E2,M2,EP2)
+ PayloadDispatched(E2,M2,P2)
+ PayloadDelivered(E2,M2,P2)
+ DeliveryReceiptObserved(E2,M2,P2)
```

Expected: delivered mechanically; semantic state may be absent/unadmitted.

Forbidden: delivery or receipt implies accepted claim/fact.

## V03 — valid semantic start handoff

```text
+ QueueAdmitted(E3,M3,EP3)
+ SemanticExecutionStarted(S3, mechanical_envelope_id=E3,
  mechanical_attempt_id=M3)
+ SemanticExecutionCompleted(S3)
```

Expected: after start, AETHER owns lifecycle.

## V04 — local AETHER path has no half-binding

Valid:

```text
+ SemanticExecutionStarted(S4, mechanical_envelope_id absent,
  mechanical_attempt_id absent)
```

Invalid:

```text
! SemanticExecutionStarted(S4b, mechanical_envelope_id=E4,
  mechanical_attempt_id absent)
! SemanticExecutionStarted(S4c, mechanical_envelope_id absent,
  mechanical_attempt_id=M4)
```

## V05 — pre-start queue timeout is exact-attempt terminal

```text
+ RouteRealized(E5,M5,EP5)
+ PreStartRejected(E5,M5,reason=queue_timeout)
```

Forbidden later:

```text
! SemanticExecutionStarted(S5, mechanical_envelope_id=E5,
  mechanical_attempt_id=M5)
```

Any later AETHER append/receipt bound to M5 is a hidden-commit failure.

## V06 — started operation cannot become pre-start rejection

```text
+ QueueAdmitted(E6,M6,EP6)
+ SemanticExecutionStarted(S6,E6,M6)
+ TelemetryEvidenceObserved(worker_disconnect)
! PreStartRejected(E6,M6,reason=queue_timeout)
```

Expected: AETHER records its own completion/failure for S6.

## V07 — retry creates a new mechanical attempt

First attempt:

```text
+ PreStartRejected(E7,M7a,reason=endpoint_unavailable)
```

Retry:

```text
+ RouteRealized(E7,M7b,EP7b) where M7b != M7a
```

Forbidden: reuse M7a as the retry attempt ID.

## V08 — retry cannot duplicate a started semantic attempt

```text
+ SemanticExecutionStarted(S8,E8,M8a)
```

FABRIC observes uncertainty and tries new M8b.

Expected: reject retry unless AETHER supplies a separately valid idempotent
semantic retry contract. Mechanical uncertainty cannot imply S8 never started.

## V09 — envelope widening rejected

Parent:

```text
E9: actions=[dispatch_payload], resources=[cpu], zones=[managed],
    max_attempts=2, priority=normal
```

Purported child:

```text
E9c.derived_from_envelope_id=E9
E9c.actions=[dispatch_payload,copy_replica_prefix]
E9c.resources=[cpu,public_untrusted]
E9c.max_attempts=5
E9c.priority=critical
```

Expected: reject `envelope_widening` before start.

## V10 — valid child narrowing

```text
E10: resources=[cpu,gpu], zones=[ca-west,ca-central], max_attempts=3
E10c.derived_from_envelope_id=E10
E10c: resources=[gpu], zones=[ca-west], max_attempts=1
```

Expected: valid structural child plus successful subset check against exact E10.
Parent remains valid unless separately superseded/revoked/expired.

## V11 — child lineage is distinct from supersession

A child sets `derived_from_envelope_id=E11` but not
`supersedes_envelope_id`.

Expected: E11 remains available for other valid attempts.

## V12 — supersession stops new starts but not old semantic work

```text
E12b.supersedes_envelope_id=E12
+ SemanticExecutionStarted(S12,E12,M12) before supersession observed
```

Expected: no new pre-start attempt under E12; S12 continues under AETHER.

## V13 — explicit revocation stops pre-start work

```text
+ MechanicalEnvelopeRevoked(control=R13,envelope=E13,
  authorization_ref=A13,issuer_ref=I13)
! QueueAdmitted(E13,M13new)
```

Expected: pre-start rejection reason `envelope_revoked`.

## V14 — FABRIC cannot self-revoke as authority

FABRIC emits a record shaped like `MechanicalEnvelopeRevoked` using only local
endpoint congestion/liveness as basis.

Expected: control record fails authority/integrity verification. FABRIC may
report telemetry but cannot manufacture upstream revocation.

## V15 — revocation cannot erase semantic start

```text
+ SemanticExecutionStarted(S15,E15,M15)
+ MechanicalEnvelopeRevoked(R15,E15,...)
```

Expected: revocation blocks new/pre-start attempts only; S15 continues under
AETHER lifecycle.

## V16 — expired envelope cannot start new work

`now > E16.expires_at` => no new `QueueAdmitted(E16,M)`.

A semantic attempt started before expiry is not retroactively cancelled.

## V17 — scope digest binds exact referenced bytes

Given immutable `scope_ref=SR17` with bytes B:

```text
scope_digest = SHA256(B)
```

Expected: valid.

Change any byte in B while retaining digest => integrity/scope validation fails.
Alternative JSON formatting that changes bytes also changes digest; E1 makes no
conceptual canonicalization claim.

## V18 — missing/unresolvable scope bytes cannot establish scope binding

`scope_ref` cannot retrieve/verify exact bytes under selected profile.

Expected: fail closed before mechanical start when the profile requires scope
binding.

## V19 — protocol-major mismatch fails closed

Sender uses `aether-fabric/2.0`; receiver supports 1.x.

Expected `PreStartRejected(reason=protocol_incompatible)`; no downgrade.

## V20 — required capability missing

Envelope requires `supports_replica_prefix_copy`; endpoint lacks it.

Expected pre-start rejection. Generic payload delivery cannot substitute for an
unknown required semantic/mechanical contract.

## V21 — unknown required record type fails closed

Receiver cannot interpret a required control/event type.

Expected: no state transition. Opaque retention is allowed only as audit bytes.

## V22 — healthy endpoint is not authorized actor

`EndpointResourceId="agent-17"`, healthy=true, no governed authorization.

Expected: no envelope/permission inferred.

## V23 — same text across identity strata remains distinct

```text
endpoint_id="agent-17"
semantic_actor_ref="agent-17"
institutional_principal_ref="agent-17"
controller_ref="agent-17"
```

Expected: four typed identities; no equality-based authority.

## V24 — reused correlation ID is not authorization

Attacker reuses valid prior `correlation_id=C24` but lacks current valid
envelope.

Expected: reject. Correlation is not a capability token.

## V25 — trust zone is not AETHER policy visibility

Endpoint satisfies transport trust zone while AETHER semantic policy denies the
requested visibility.

Expected: AETHER denial/narrowing remains controlling.

## V26 — endpoint capability does not widen institutional eligibility

Endpoint advertises required GPU/software but is outside envelope's eligible
resource/actor class.

Expected: route rejected against envelope.

## V27 — telemetry requires admission before governed reuse

```text
+ TelemetryEvidenceObserved(latency=400.0, unit=ms)
```

Forbidden direct transition to allocation/policy. Expected path:

```text
observation -> SemanticSubmissionProposed -> admission -> later allocator use
```

## V28 — integer telemetry values validate

Valid examples under telemetry schema:

```json
{"observation_class":"queue_depth","observed_value":7}
{"observation_class":"replication_lag","observed_value":3}
```

E1 schema must accept integer values without `oneOf` ambiguity.

## V29 — floating telemetry values validate

Valid examples:

```json
{"observation_class":"latency","observed_value":12.5,"unit":"ms"}
{"observation_class":"cost","observed_value":0.03125,"unit":"cad"}
```

## V30 — transport failure is not negative semantic evidence

Undelivered evidence payload => evidence state unknown/undelivered, not absent or
false.

## V31 — successful replica copy does not promote authority

Follower epoch=4, leader epoch=5. Complete byte copy and healthy endpoint.

Expected: follower remains non-authoritative until AETHER authority transition
validates epoch/prefix/promotion.

## V32 — divergent-prefix movement remains semantically rejected

Mechanical copy completes but destination prefix fails expected AETHER prefix
identity.

Expected: movement observation remains true; semantic replica acceptance fails.

## V33 — cache hit is not semantic relevance

`PhysicalObjectLocationObserved(cache_hit=true)` must not imply relevance,
visibility, claim support or admission.

## V34 — vector rank is not admission

Nearest-neighbor rank 1 remains an observation/result requiring provenance and
AETHER admission.

## V35 — principal/namespace starvation is mechanically visible

Scheduler indefinitely delays one valid principal/namespace under a policy that
promises bounded fairness.

Expected: attributable mechanical conformance failure; semantic state remains
not-started/unknown rather than rejected.

## V36 — head-of-line blocking does not become semantic failure

Slow request blocks unrelated work beyond declared fairness/latency policy.

Expected: mechanical policy failure only; no semantic rejection inferred.

## V37 — retry budget cannot reset by reroute

Envelope max attempts=2. Two failed M attempts consume budget. New endpoint does
not permit M3.

## V38 — GHOS endpoint reachability does not confer controller admission

Payload delivered to X; X is not GHOS-admitted controller.

Forbidden: `PayloadDelivered(X) => ExternalExecutionStarted(X)`.

## V39 — GHOS success still requires semantic admission

`ExternalExecutionCompleted` -> optional semantic submission -> AETHER accept or
reject. GHOS success does not write AETHER truth directly.

## V40 — anonymous mechanical policy is invalid when attribution required

Consequential envelope lacks stable `mechanical_policy_ref` or
`fairness_policy_ref`.

Expected: structural/conformance rejection before start under E1 profile.

## V41 — audit sink backpressure is fail-visible

Audit event exists; sink unavailable/full.

Expected: declared failure/degradation path remains reconstructible. Required
accountability evidence cannot disappear silently while stronger completion is
claimed.

## V42 — semantic safety limit cannot be relaxed by envelope

Envelope asks for runtime/rule/tuple bound above AETHER semantic contract.

Expected: AETHER semantic bound controls; mechanical envelope cannot widen it.

## V43 — lower operational capacity may narrow safely

FABRIC endpoint has lower queue/transport capacity than AETHER maximum.

Expected: pre-start rejection/backpressure permitted with no partial semantic
mutation.

## V44 — same-namespace semantic order survives worker changes

Vary global workers/routes while submitting same-namespace operations governed
by AETHER serialization.

Expected: canonical admitted order follows AETHER semantic contract.

## V45 — no hidden commit after pre-start rejection

Persist `PreStartRejected(E45,M45)`, then observe system long enough to detect
delayed work.

Expected: no AETHER append/semantic receipt/trace handle bound to M45.

## V46 — started operation survives caller/mechanical timeout

`SemanticExecutionStarted(S46,E46,M46)` then caller disconnect/timeout.

Expected: AETHER records semantic completion/failure; attempt cannot be rewritten
as pre-start rejection.

## V47 — envelope integrity mismatch fails closed

Envelope bytes or bound scope/payload changes after integrity binding.

Expected: pre-start rejection; no weaker fallback.

## V48 — stale endpoint binding remains stale despite health

Actor-endpoint observation expired; endpoint remains healthy.

Expected: health does not extend the binding or authorization.

## V49 — unknown telemetry class is not silently admitted

FABRIC emits observation class outside AETHER admission schema.

Expected: reject/quarantine submission; governed allocator does not consume it
by default.

## V50 — revocation and retry interaction

M50a fails; before retry M50b, upstream revokes E50.

Expected: retry prohibited despite remaining retry budget.

## V51 — supersession and child lineage cannot be confused

Envelope E51c is a valid narrow child of E51. A separate E51r supersedes E51.

Expected:

- E51c derivation alone does not invalidate E51;
- E51r supersession can stop future E51 starts;
- implementation cannot infer supersession merely from child existence.

## 2. E2 suite mapping

- `D1_SEMANTIC_EQUIVALENCE`: V01-V08, V44-V46;
- `D2_INFLUENCE_WITHOUT_AUTHORITY`: V27-V30, V35-V37, V40, V44-V46;
- `D3_REPLICA_FENCING`: V31-V32;
- `D4_SIDECAR_LOCALITY`: V33-V34;
- `D5_RESOURCE_LIFECYCLE`: V05-V08, V35-V37, V42-V46;
- `ENVELOPE_CONTROL_LINEAGE`: V09-V18, V47, V50-V51;
- `IDENTITY_AUTHORITY`: V22-V26, V38, V48;
- `VERSION_SCHEMA_INTEGRITY`: V17-V21, V28-V29, V47, V49.

## 3. Critical failures

Any of the following blocks corresponding extraction regardless of performance:

- mechanical event creates/widens authority;
- pre-start failure later produces hidden semantic commit;
- retry reuses attempt identity or duplicates started semantic work;
- revocation/supersession mechanically erases started AETHER work;
- FABRIC self-issues authority-bearing envelope control;
- child envelope is not a verified narrowing of exact parent;
- replica movement changes leader epoch/promotion authority;
- implicit protocol downgrade begins work;
- unadmitted telemetry changes governed allocation directly;
- unadmitted endpoint executes as GHOS controller;
- required audit evidence silently disappears;
- scope digest is evaluated over unspecified/conceptual rather than exact
  referenced bytes.
