# AETHER/FABRIC E1 Conformance Vectors

Status: E1 candidate
Issue: #85
Protocol: `aether-fabric/1.0`

## 1. Purpose

Define concrete E1 examples that a later E2 harness must encode as executable
positive and negative tests.

E1 vectors are specification evidence only. They do not claim a FABRIC runtime
exists.

Each vector declares:

- initial authority/semantic state;
- input record sequence;
- expected mechanical/semantic result;
- forbidden result;
- E0 ambiguity or Council condition exercised.

## 2. Vector notation

```text
+ EVENT(...)   valid observed event
! EVENT(...)   event that must be rejected/quarantined as invalid
=>             expected consequence
!=>            forbidden implication
```

IDs such as `A1`, `E1`, `EP1`, `C1` are illustrative typed identifiers, not
production UUID requirements.

## V01 — minimal valid pre-start mechanical realization

Initial:

```text
AuthorizationRef=A1 exists upstream.
No semantic execution has started.
```

Trace:

```text
+ MechanicalEnvelopeAuthorized(E1, authorization_ref=A1,
  permitted_actions=[dispatch], eligible_resources=[cpu-worker],
  expires_at=T2, required_capabilities=[supports_payload_transport])
+ RouteRealized(E1, endpoint=EP1, resource_class=cpu-worker)
+ QueueAdmitted(E1, endpoint=EP1)
```

Expected:

```text
mechanical_state = queue_admitted
semantic_state = not_started
```

Forbidden:

```text
QueueAdmitted !=> SemanticExecutionStarted
QueueAdmitted !=> SemanticAdmissionAccepted
```

## V02 — delivery does not admit

Trace:

```text
+ MechanicalEnvelopeAuthorized(E2, ...)
+ RouteRealized(E2, EP2)
+ QueueAdmitted(E2, EP2)
+ PayloadDispatched(E2, payload=P1)
+ PayloadDelivered(E2, payload=P1)
+ DeliveryReceiptObserved(E2, payload=P1)
```

Expected:

```text
mechanical_state = delivered
semantic_state = not_started OR unadmitted
```

Forbidden:

```text
PayloadDelivered !=> SemanticAdmissionAccepted
DeliveryReceiptObserved !=> ClaimAccepted
```

## V03 — valid AETHER semantic start handoff

Trace:

```text
+ MechanicalEnvelopeAuthorized(E3, ...)
+ RouteRealized(E3, EP3)
+ QueueAdmitted(E3, EP3)
+ SemanticExecutionStarted(attempt=S3, correlation=C3)
+ SemanticExecutionCompleted(attempt=S3, correlation=C3)
```

Expected:

- no mechanical timeout/rejection may erase `S3`;
- AETHER completion semantics control the attempt after start.

## V04 — pre-start queue timeout is terminal non-start

Trace:

```text
+ MechanicalEnvelopeAuthorized(E4, ...)
+ RouteRealized(E4, EP4)
+ PreStartRejected(E4, reason=queue_timeout)
```

Expected:

```text
semantic_state = not_started
```

Forbidden later event:

```text
! SemanticExecutionStarted(attempt=S4, bound_to=E4)
```

If a later accepted append/semantic receipt appears for the supposedly
non-started attempt, the trace fails as a hidden commit.

## V05 — started operation cannot be converted to timeout

Trace:

```text
+ MechanicalEnvelopeAuthorized(E5, ...)
+ QueueAdmitted(E5, EP5)
+ SemanticExecutionStarted(S5)
+ TelemetryEvidenceObserved(observation=worker_disconnect)
```

Forbidden:

```text
! PreStartRejected(E5, reason=queue_timeout)
```

Expected:

AETHER must eventually record its own completion/failure state. FABRIC may
report operational failure but cannot reinterpret semantic lifecycle.

## V06 — envelope widening rejected

Authorized envelope:

```text
E6:
  permitted_actions=[dispatch]
  eligible_resources=[cpu-worker]
  trust_zones=[gcl-managed]
  retry_budget=2
  priority_ceiling=normal
```

Attempted derived envelope:

```text
E6b:
  permitted_actions=[dispatch, promote_replica_authority]
  eligible_resources=[cpu-worker, public-untrusted]
  retry_budget=5
  priority=critical
```

Expected:

```text
! E6b
reason = envelope_widening
```

No dispatch occurs.

## V07 — narrowed downstream envelope valid

Authorized:

```text
E7: resources=[cpu,gpu], zones=[ca-west,ca-central], retry<=3
```

Derived:

```text
E7b: resources=[gpu], zones=[ca-west], retry=1
```

Expected: structurally valid narrowing, subject to integrity/security profile.

## V08 — expired envelope cannot start new work

Trace:

```text
now > E8.expires_at
! QueueAdmitted(E8)
```

Expected `PreStartRejected(reason=envelope_expired)`.

An existing `SemanticExecutionStarted` bound before expiry is not retroactively
cancelled.

## V09 — superseded envelope cannot start new work

```text
E9b supersedes E9
! QueueAdmitted(E9, new_attempt)
```

Existing started semantic attempt under E9 follows AETHER lifecycle.

## V10 — major-version incompatibility fails closed

Sender:

```text
protocol = aether-fabric/2.0
```

Receiver supports only `1.x`.

Expected:

```text
PreStartRejected(reason=protocol_incompatible)
```

Forbidden:

```text
silent downgrade to 1.x
```

## V11 — required capability missing

Envelope requires:

```text
required_capabilities=[supports_replica_prefix_copy]
```

Endpoint lacks it.

Expected pre-start rejection. Endpoint may not emulate an unknown operation by
reinterpreting it as generic payload transport.

## V12 — unknown required record type fails closed

A required transition record is unknown to receiver.

Expected: reject before state transition. Receiver may store opaque bytes for
audit only if it makes no operational/semantic decision from them.

## V13 — healthy endpoint is not authorized actor

Initial:

```text
EndpointResourceId=agent-17, healthy=true
No matching institutional authorization.
```

Forbidden:

```text
endpoint healthy !=> actor authorized
```

No envelope may be minted by FABRIC from liveness alone.

## V14 — equal names across identity strata do not collapse

```text
endpoint_id="agent-17"
semantic_actor_ref="agent-17"
institutional_principal_ref="agent-17"
controller_ref="agent-17"
```

Expected: four typed identities remain distinct. No authority inference occurs
without explicit bridge/governing record.

## V15 — reused correlation ID is not authorization

An attacker reuses `correlation_id=C15` from a prior valid operation but lacks a
valid current envelope.

Expected: reject. Correlation groups traces; it is not a capability token.

## V16 — trust-zone permission does not imply semantic visibility

Endpoint belongs to allowed transport zone `gcl-managed` but AETHER policy scope
does not permit the semantic actor to see the requested namespace/facts.

Expected:

- transport security constraint may be satisfied;
- semantic operation remains blocked/narrowed by AETHER policy;
- FABRIC cannot widen visibility.

## V17 — resource capability does not imply institutional eligibility

Endpoint advertises GPU and required software capability but is not within the
upstream eligible actor/resource class.

Expected: route rejected against envelope. Capability cannot widen eligibility.

## V18 — telemetry requires admission before governed reuse

Trace:

```text
+ TelemetryEvidenceObserved(endpoint=EP18, observation=latency, value=400ms)
```

Forbidden direct transition:

```text
! AllocationDesired(avoid=EP18) solely from unadmitted telemetry
```

Expected governed path:

```text
TelemetryEvidenceObserved
 -> SemanticSubmissionProposed
 -> SemanticAdmissionAccepted
 -> later allocator consumes admitted evidence
```

## V19 — transport failure is not negative claim evidence

A payload containing requested evidence is undelivered.

Forbidden:

```text
transport_failed !=> evidence_absent
transport_failed !=> claim_false
```

Expected: evidence state is unknown/undelivered.

## V20 — mechanically successful replica copy does not promote authority

Initial:

```text
follower epoch=4
leader epoch=5
```

Trace:

```text
+ ReplicaMovementObserved(source=L, destination=F, bytes=complete)
+ endpoint_health(F)=healthy
```

Expected:

- physical state may become byte-equal;
- follower remains non-authoritative under stale epoch until AETHER authority
  transition says otherwise.

Forbidden:

```text
ReplicaMovementObserved !=> LeaderEpochChanged
ReplicaMovementObserved !=> ReplicaPromoted
```

## V21 — divergent-prefix copy fails semantic validation

Mechanical transfer completes, but destination prefix digest does not satisfy
AETHER's expected authority prefix.

Expected:

- mechanical observation remains true;
- AETHER rejects semantic replica acceptance/fencing transition;
- FABRIC may not rewrite prefix identity to make validation pass.

## V22 — cache hit is not semantic relevance

Trace:

```text
+ PhysicalObjectLocationObserved(object_ref=O22, cache_hit=true)
```

Forbidden:

```text
cache_hit !=> semantically_relevant
cache_hit !=> policy_visible
cache_hit !=> supports_claim
```

## V23 — vector rank is not admission

A mechanical/vector service returns nearest-neighbor rank 1 for object O23.

Expected: rank/result is an observation/result payload requiring provenance and
AETHER submission/admission. No semantic acceptance from rank alone.

## V24 — principal fairness starvation attack

Two valid envelopes share a mechanical queue. Scheduler indefinitely delays one
principal despite both remaining within envelope constraints.

Expected E2 test outcome:

- violation is visible as attributable mechanical policy/fairness failure;
- no semantic rejection is inferred;
- the scheduler cannot hide behind an anonymous default;
- envelope/policy contract determines whether the behavior is nonconformant.

## V25 — head-of-line blocking attack

A mechanically slow request blocks unrelated eligible work beyond declared
fairness/latency bounds.

Expected: operational conformance failure; semantic state remains
unknown/not-started for blocked work rather than rejected.

## V26 — retry budget cannot reset by reroute

Authorized retry budget=2.

Two failures consume both attempts. A new endpoint route under same envelope
attempts a third retry.

Expected: reject. Endpoint change does not reset budget.

## V27 — retry cannot duplicate started semantic attempt

First mechanical attempt crossed `SemanticExecutionStarted`; transport
observation becomes uncertain. FABRIC attempts retry as if first attempt never
started.

Expected: reject unless AETHER explicitly supplies an idempotent semantic retry
contract for the same semantic operation.

## V28 — GHOS endpoint reachability does not confer controller admission

Payload is delivered to endpoint X. X is not GHOS-admitted controller for the
protected operation.

Forbidden:

```text
PayloadDelivered(X) !=> ExternalExecutionStarted(X)
```

Protected execution remains blocked.

## V29 — GHOS execution success still requires AETHER admission

GHOS returns successful artifact/result.

Expected path:

```text
ExternalExecutionCompleted
 -> SemanticSubmissionProposed
 -> SemanticAdmissionAccepted | Rejected
```

Forbidden: execution success directly writes semantic truth.

## V30 — anonymous mechanical policy rejected for consequential envelope

A consequential envelope refers to no stable `mechanical_policy_ref` or
`fairness_policy_ref` while scheduler behavior can affect priority/selection.

Expected: pre-start conformance rejection for deployments whose selected profile
requires attributable mechanical policy.

## V31 — audit sink backpressure is fail-visible

Audit event is formed but sink queue is full/unavailable.

Expected behavior must follow the declared audit policy and remain visible.

Forbidden:

- silently drop required accountability evidence;
- report a stronger completed/accountable state than can be reconstructed.

## V32 — semantic safety limit cannot be relaxed by mechanical envelope

Mechanical envelope asks for a runtime/rule/tuple limit larger than AETHER's
semantic safety contract.

Expected: AETHER limit wins; envelope cannot widen semantic boundedness.

## V33 — operational capacity limit may narrow safely

FABRIC/endpoint imposes a lower queue/body transport capacity than the maximum
AETHER semantic contract.

Expected: pre-start mechanical rejection/backpressure is permitted if it does
not partially mutate semantic state or reinterpret rejection as semantic
failure.

## V34 — same-namespace ordering survives worker-count changes

Vary global worker count and endpoint selection while submitting same-namespace
semantic operations whose AETHER contract requires serialization.

Expected: canonical admitted operation order obeys AETHER serialization
contract. Mechanical parallelism may not reorder semantic commits.

## V35 — no hidden commit after pre-start rejection

Force queue timeout before semantic start, persist `PreStartRejected`, then
continue observing system long enough to detect delayed execution.

Expected: no corresponding AETHER append/semantic receipt/trace handle appears.
Any such later commit is a critical conformance failure.

## V36 — started operation completes despite caller/mechanical timeout

Allow semantic start, then force caller disconnect/transport timeout.

Expected: AETHER operation follows started-operation completion/failure semantics
and remains queryable/auditable; it is not treated as a pre-start rejection.

## V37 — envelope integrity mismatch

Payload/envelope bytes change after the integrity binding was produced.

Expected: pre-start rejection under the selected integrity profile. No fallback
removes constraints.

## V38 — supersession cannot widen via FABRIC

FABRIC attempts to self-issue E38b superseding E38 with broader resources.

Expected: reject because FABRIC is not authorized envelope issuer and because
scope widens.

## V39 — stale endpoint binding

`ActorEndpointBindingObserved` expired, but endpoint remains healthy.

Expected: health does not extend binding. New route requires current authorized
binding/envelope evidence.

## V40 — unknown mechanical observation stays non-semantic

FABRIC emits an observation class unknown to AETHER admission schema.

Expected: AETHER may reject/quarantine the submission; allocator cannot consume
it as admitted evidence by default.

## 3. Required E2 suites

E2 should group the vectors into executable suites:

- `D1_SEMANTIC_EQUIVALENCE`: V01-V05, V34-V36;
- `D2_INFLUENCE_WITHOUT_AUTHORITY`: V18-V19, V24-V27, V30, V34-V36;
- `D3_REPLICA_FENCING`: V20-V21;
- `D4_SIDECAR_LOCALITY`: V22-V23;
- `D5_RESOURCE_LIFECYCLE`: V04-V05, V24-V27, V32-V36;
- `IDENTITY_AUTHORITY`: V13-V17, V28, V39;
- `VERSION_INTEGRITY`: V06-V12, V37-V38, V40.

## 4. Severity

Critical failures include:

- mechanical event creates/widens authority;
- pre-start failure later produces hidden semantic commit;
- started semantic operation is erased by mechanical timeout;
- replica movement changes leader epoch/promotion authority;
- implicit protocol downgrade begins work;
- unadmitted telemetry changes governed allocation directly;
- unadmitted endpoint executes as GHOS controller;
- required audit evidence disappears silently while stronger completion is
  claimed.

A critical failure blocks corresponding extraction regardless of performance
benefit.
