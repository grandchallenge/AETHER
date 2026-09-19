# AETHER/FABRIC E1 Interface Law

Status: E1 candidate, Revision 1 after Formalist review
Issue: `AETHER-FABRIC-ENCAP-E1-001` / #85
AETHER basis: protected `main` `411411dfcc29757bbf68589b817b8bffeceb7bcb`
Council basis: INTELLECT `18df1c3ef89712fe00af37a484cc03ce270e99bd`
E0 basis: AETHER ADR 0020 and E0 packet merged at `411411df...`
Protocol candidate: `aether-fabric/1.0`

## 1. Purpose

Define the smallest versioned boundary by which AETHER may later request an
independently encapsulated mechanical FABRIC to realize already-authorized
movement, placement, rendezvous, pre-start queueing, replica movement, or
physical object locality.

E1 is interface law only. It does not move implementation, create a FABRIC
runtime, change Article IX, activate POL/AETHER-Learn, or change GHOS controller
admission.

> AETHER owns admitted semantic state and semantic lifecycle. FABRIC may realize
> bounded mechanical work. Mechanical success, liveness, delivery, replication,
> queue admission or endpoint capability never manufacture semantic,
> institutional, constitutional or GHOS-controller authority.

## 2. Protocol identity

```text
protocol_family = aether-fabric
protocol_major  = 1
protocol_minor  = 0
canonical       = aether-fabric/1.0
```

Schemas under `schemas/aether_fabric/e1/` are specification/conformance
artifacts, not production activation.

## 3. Plane ownership

AETHER retains semantic namespace identity, append/admission, cuts/replay,
policy visibility, provenance/proof identity, semantic coordination facts,
same-namespace semantic ordering where required, started-operation lifecycle,
semantic lease/fence meaning, partition/cut identity, leader epoch/promotion and
stale/divergent-prefix fencing, sidecar semantic identity/provenance/policy, and
admission of returned evidence.

FABRIC may later own—only after corresponding E2/E3 evidence—endpoint/resource
observation, concrete route/placement, global worker-capacity realization,
pre-start queue admission/rejection, delivery/retry mechanics, replica
byte/prefix movement beneath AETHER fencing, physical object/vector locality,
and operational liveness/capacity/locality/cost/latency observations.

POL/governed allocation owns institutional eligibility, desired allocation,
utility and priority among ends. GHOS owns protected controller admission and
execution. INTELLECT owns constitutional authority and Article IX.

## 4. Core invariants

```text
PayloadDelivered            !=> SemanticAdmissionAccepted
EndpointHealthy              !=> ActorAuthorized
ReplicaBytesEqual            !=> ReplicaAuthorityEqual
ResourceAvailable            !=> InstitutionallyEligible
Undelivered                  !=> ClaimFalse
PreStartRejected             !=> SemanticAdmissionRejected
```

Once AETHER records `SemanticExecutionStarted`, FABRIC cannot convert the exact
attempt into pre-start rejection or safe cancellation.

FABRIC may optimize only inside a closed mechanical envelope. Priority,
fairness, retry, eligibility and locality policy must be attributable through
stable references. Unadmitted telemetry cannot directly mutate governed policy
or allocation. Unsupported protocol/capabilities fail before start with no
implicit downgrade.

## 5. Attempt identity

Every FABRIC realization has a required:

```text
mechanical_attempt_id
```

All route/queue/dispatch/delivery records for that attempt carry the same ID. A
retry creates a new mechanical attempt ID while retaining the higher-level
`correlation_id`.

If FABRIC participates in reaching AETHER semantic start,
`SemanticExecutionStarted` binds both:

```text
mechanical_envelope_id
mechanical_attempt_id
```

The local/no-FABRIC AETHER path omits both. It is invalid to carry only one.

## 6. `MechanicalEnvelopeAuthorized`

This record is a `control_bridge`, not a FABRIC-produced mechanical event.

Required conceptual identity:

```text
record_type = MechanicalEnvelopeAuthorized
domain = control_bridge
authority_owner = upstream_governed_record
authority_effect = mechanical_only
envelope_id
correlation_id
authorization_ref
issuer_ref
scope_ref
scope_digest
payload_ref
permitted_actions[]
eligible_resource_classes[]
trust_zones[]
locality_constraints[]
mechanical_policy_ref
priority_class
fairness_policy_ref
retry_policy
redundancy_policy
expires_at
required_capabilities[]
integrity_profile_ref
derived_from_envelope_id?   # narrowing lineage
supersedes_envelope_id?     # replacement lifecycle
```

`mechanical_only` means the envelope authorizes only the bounded physical actions
expressed by the record. It does **not** confer semantic admission, institutional
power, constitutional authority or GHOS-controller admission.

Schema validation proves structural conformance only. A selected integrity and
authority profile must separately verify the upstream authorization before the
envelope is acted upon.

The envelope admits no semantic equivalent of grant-role, accept-claim,
promote-authority, admit-controller, widen-policy-visibility or amend-
constitution.

## 7. Scope byte identity

`scope_ref` identifies an immutable byte object containing the exact scope
manifest.

```text
scope_digest.algorithm = sha256
scope_digest.value = SHA256(exact byte sequence identified by scope_ref)
```

No conceptual-object canonicalization is implied. If exact bytes cannot be
retrieved/verified under the selected profile, the scope binding is not
established.

## 8. Closed-world and narrowing law

Unknown required action/capability/version/constraint is a pre-start failure.

A child envelope sets `derived_from_envelope_id=parent.envelope_id` and must be a
verified narrowing:

```text
actions(child)             subseteq actions(parent)
resources(child)           subseteq resources(parent)
trust_zones(child)         subseteq trust_zones(parent)
locality(child)            no weaker than parent
retry_budget(child)        <= parent
parallel_redundancy(child) <= parent ceiling
priority(child)            <= parent ceiling
deadline(child)            <= parent deadline
security requirements      no weaker than parent
```

Child derivation does not invalidate the parent.

`supersedes_envelope_id` is separate replacement lifecycle and is mutually
exclusive with child derivation in one envelope record.

Expiry or supersession prevents new/pre-start work but cannot retroactively
erase AETHER semantic execution already started.

## 9. `MechanicalEnvelopeRevoked`

Early revocation is explicit and upstream-owned:

```text
record_type = MechanicalEnvelopeRevoked
domain = control_bridge
authority_owner = upstream_governed_record
control_id
envelope_id
authorization_ref
issuer_ref
revoked_at
reason
integrity_profile_ref
```

A separately verified revocation stops new/pre-start attempts. Queued-but-not-
started work may terminate as `PreStartRejected(reason=envelope_revoked)`.

FABRIC cannot self-issue revocation from congestion/liveness state. Revocation
cannot mechanically erase an already recorded `SemanticExecutionStarted` and
does not itself create a replacement envelope.

## 10. Mechanical event law

Every envelope-bound mechanical event carries:

```text
event_id
correlation_id
mechanical_attempt_id
envelope_id
occurred_at
```

Core mechanical events:

```text
RouteRealized
QueueAdmitted
PreStartRejected
PayloadDispatched
PayloadDelivered
DeliveryReceiptObserved
ReplicaMovementObserved
PhysicalObjectLocationObserved
```

Mechanical events are produced in domain `mechanical` with
`authority_effect=none`.

`PreStartRejected` reasons include capacity/queue/endpoint failure, expiry,
revocation, supersession, protocol/capability/trust/policy failure, payload
unavailability, integrity failure and envelope widening.

It is invalid to record `PreStartRejected(E,M)` after AETHER has recorded
`SemanticExecutionStarted(...E,M)`.

## 11. Resource scheduling handoff

```text
AETHER: NamespaceResolved
AETHER/POL: MechanicalEnvelopeAuthorized(E)
FABRIC: RouteRealized(E,M)
FABRIC: QueueAdmitted(E,M) | PreStartRejected(E,M)
AETHER: SemanticExecutionStarted(S,E,M)
AETHER: SemanticExecutionCompleted(S)
```

FABRIC owns capacity realization only up to semantic start.

AETHER retains deterministic same-namespace order where required, no-partial-
append/no-semantic-receipt guarantees on pre-start rejection, and
`cancel_before_start_complete_after_start`.

E1 adds no cooperative in-evaluation cancellation. Such a feature requires a
new AETHER semantic checkpoint ADR.

## 12. Retry law

A retry:

- keeps `correlation_id`;
- creates a new `mechanical_attempt_id`;
- uses the same or narrower still-valid envelope;
- consumes remaining retry budget;
- does not reset expiry/revocation/supersession;
- cannot duplicate a semantic attempt already started unless AETHER explicitly
  provides an idempotent semantic retry contract.

## 13. Telemetry evidence return

`TelemetryEvidenceObserved` may report liveness, queue depth/wait, replication
lag, capacity, locality, latency, cost, transport failure or backpressure.

It remains observation-only:

```text
TelemetryEvidenceObserved
 -> SemanticSubmissionProposed
 -> SemanticAdmissionAccepted | SemanticAdmissionRejected
 -> later governed allocator may use admitted evidence
```

No direct unadmitted telemetry-to-policy edge exists.

## 14. Replica movement

Future FABRIC may move bytes/prefixes. AETHER retains source/partition cuts,
leader epoch, promotion authority, stale-epoch rejection, divergent-prefix
fencing and semantic acceptance of replica state.

```text
MechanicalEnvelopeAuthorized
 -> ReplicaMovementObserved(E,M)
 -> AETHER validates cut/prefix/epoch/fencing
 -> AETHER accepts/rejects semantic replica state
```

## 15. Sidecar/object locality

Future FABRIC/storage may report physical placement/cache/shard movement against
an opaque AETHER semantic object reference. Cache hit, vector rank and physical
availability are observations, not semantic relevance, policy visibility or
claim support.

## 16. Identity strata

At least four identity types remain non-collapsible:

```text
EndpointResourceId
SemanticActorRef
InstitutionalPrincipalRef
ControllerRef
```

String equality, co-location, reachability, liveness, capability, common
credentials or reused correlation IDs do not create cross-stratum authority.
Explicit observation/reference records are required.

## 17. Failure vocabulary

Mechanical:

```text
unknown
undelivered
route_realized
queue_admitted
pre_start_rejected
dispatched
delivered
transport_failed
```

Semantic:

```text
not_started
started
completed
submission_proposed
unadmitted
admitted
rejected
```

Forbidden collapses:

```text
transport_failed != rejected
pre_start_rejected != rejected
completed != admitted
delivered != admitted
```

## 18. Version and integrity law

- unknown protocol major: reject before start;
- unsupported required capability: reject before start;
- unknown required record type: fail closed;
- no implicit downgrade or constraint removal;
- schema validation is structural only;
- authority/integrity verification is separate;
- integrity failure rejects before start with no weaker fallback.

A future integrity profile must bind exact protocol, control/envelope identity,
upstream authorization/issuer refs, `scope_ref`+exact-byte digest, payload,
actions/resources/zones, policy/fairness refs, retry/redundancy/priority/deadline,
child/supersession lineage, expiry/revocation identity and required
capabilities.

## 19. E0 ambiguity disposition

Candidate E1 boundaries now exist for sidecar locality, replica movement,
namespace-to-worker realization, pre-start queue/resource realization and
telemetry evidence return.

Still unresolved pending E2 evidence:

- durable storage adapter extraction;
- generic transport extraction from `aether_http`;
- per-limit classification where semantic safety and operational capacity remain
  mixed.

The temporary `aether_api` facade is not a protocol owner.

## 20. E1 completion boundary

E1 completes only when the interface law, event algebra, identity law, schemas,
fixtures, conformance vectors and ADR agree; Formalist/Adversary/Referee review
finds no material defect; and exact-head protected checks close.

E1 completion authorizes only E2 conformance-harness work. It does not authorize
runtime extraction, FABRIC activation, Article IX change, POL/AETHER-Learn
activation, GHOS-controller change, deployment or claim promotion.
