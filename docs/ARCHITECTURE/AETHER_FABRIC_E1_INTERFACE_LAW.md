# AETHER/FABRIC E1 Interface Law

Status: E1 candidate, Revision 1 after Formalist review
Issue: `AETHER-FABRIC-ENCAP-E1-001` / #85
AETHER basis: protected `main` `411411dfcc29757bbf68589b817b8bffeceb7bcb`
Council basis: INTELLECT `18df1c3ef89712fe00af37a484cc03ce270e99bd`
E0 basis: AETHER ADR 0020 and E0 architecture packet merged at `411411df...`
Protocol candidate: `aether-fabric/1.0`

## 1. Purpose

Define the smallest versioned boundary by which AETHER may later request an
independently encapsulated mechanical FABRIC to realize already-authorized
movement, placement, rendezvous, queue admission, replication movement, or
physical object locality.

E1 is interface law only. It does not move implementation, create a FABRIC
runtime, change Article IX, activate POL/AETHER-Learn, or change GHOS controller
admission.

The governing statement is:

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

Schemas in `schemas/aether_fabric/e1/` are specification/conformance artifacts,
not production activation.

## 3. Plane ownership

### AETHER owns

- semantic namespace identity;
- append/admission decisions;
- journal cuts and replay;
- policy visibility;
- provenance and proof identity;
- semantic coordination facts;
- same-namespace semantic ordering where the live resource contract requires it;
- `cancel_before_start_complete_after_start` lifecycle;
- semantic execution start/completion;
- semantic lease/fence meaning;
- partition/cut identity;
- leader epoch, promotion authority, stale-epoch rejection and divergent-prefix
  fencing;
- semantic sidecar identity/provenance/policy;
- interpretation of admitted telemetry as evidence.

### FABRIC may later own after E2/E3 evidence

- endpoint/resource observation;
- concrete route/placement realization;
- global worker-pool capacity realization;
- pre-start queue admission/rejection;
- queue depth/wait/backpressure observation;
- delivery/retry mechanics;
- replica byte/prefix movement beneath AETHER fencing;
- physical artifact/vector locality and movement;
- liveness/capacity/locality/cost/latency observations.

### POL / governed allocator owns

- desired institutional allocation;
- institutional actor/guild eligibility;
- institutional objective/utility;
- priority among ends;
- interpretation of admitted evidence for later allocation.

### GHOS owns

- persistent-controller admission;
- protected execution routing;
- execution credential authority;
- protected external mutation semantics.

### INTELLECT owns

- constitutional policy;
- office powers/obligations;
- authority schedules;
- Article IX and related boundary law.

## 4. Core invariants

### I1 — delivery is not admission

```text
PayloadDelivered !=> SemanticAdmissionAccepted
```

### I2 — liveness is not permission

```text
EndpointHealthy !=> ActorAuthorized
```

### I3 — replication is not semantic authority

```text
ReplicaBytesEqual !=> ReplicaAuthorityEqual
```

### I4 — capability/resource possession is not institutional eligibility

```text
ResourceAvailable !=> InstitutionallyEligible
```

### I5 — transport failure is not negative evidence

```text
Undelivered !=> ClaimFalse
PreStartRejected !=> SemanticAdmissionRejected
```

### I6 — started semantic work cannot be mechanically erased

Once AETHER records `SemanticExecutionStarted`, FABRIC cannot convert that
attempt into `PreStartRejected`, cannot report it safely cancelled, and cannot
cause AETHER to claim non-start while the operation may still commit.

### I7 — mechanical optimization is bounded

FABRIC may optimize only within an immutable `MechanicalEnvelopeAuthorized`.
It may not widen endpoint/resource eligibility, trust zones, retry budget,
priority ceiling, deadline, redundancy, payload scope or permitted mechanical
actions.

### I8 — mechanical policy is attributable

Priority, fairness, retry, eligibility and locality policies used for
consequential realization must be identified by stable policy references bound
to the envelope. Anonymous scheduler defaults cannot silently act as
institutional policy.

### I9 — telemetry is evidence, not policy mutation

FABRIC telemetry must travel through provenance-bearing AETHER
submission/admission before a governed allocator uses it as semantic evidence.

### I10 — version/capability downgrade is fail-closed

Unsupported protocol major, required capability, required record type or
selected integrity profile rejects before start. No constraint may be silently
dropped to make work routable.

### I11 — attempt identity is explicit

Every FABRIC realization attempt has a required `mechanical_attempt_id` that is
stable across route/queue/dispatch/delivery records for that attempt. A retry
uses a new `mechanical_attempt_id` while retaining the higher-level
`correlation_id`.

If AETHER crosses semantic start through a FABRIC realization, the
`SemanticExecutionStarted` record binds both `mechanical_envelope_id` and
`mechanical_attempt_id`.

A local/no-FABRIC AETHER execution is the only exception: it omits both fields.
It is invalid to provide one without the other.

## 5. Mechanical envelope

### 5.1 `MechanicalEnvelopeAuthorized`

Purpose: bind one upstream governed authorization to a closed mechanical scope.

Required conceptual fields:

```text
protocol_version
record_type = MechanicalEnvelopeAuthorized
authority_effect = none
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
derived_from_envelope_id?      # narrowing lineage only
supersedes_envelope_id?        # replacement lifecycle only
integrity_profile_ref?
```

`authorization_ref` and `issuer_ref` are opaque cross-plane references. FABRIC
may structurally require them and verify a separately selected integrity
profile, but schema validation does not prove the referenced office/actor had
institutional authority.

The contract admits no semantic equivalent of:

- grant role/permission;
- accept claim;
- promote replica authority;
- admit GHOS controller;
- widen policy visibility;
- amend constitutional authority.

### 5.2 Scope byte identity

`scope_ref` identifies an immutable byte object containing the exact scope
manifest used to derive the envelope.

`scope_digest` is:

```text
algorithm = sha256
value = SHA256(exact byte sequence identified by scope_ref)
```

No conceptual-object canonicalization is implied. If the exact referenced bytes
cannot be retrieved/verified under the selected profile, the scope binding is
not established.

### 5.3 Closed-world interpretation

Only declared actions/resources/zones/capabilities are permitted. Unknown
required capability, unknown protocol major, unknown required action or missing
required constraint causes pre-start rejection.

### 5.4 Child-envelope narrowing

A downstream component may derive a **narrower** envelope only by setting:

```text
derived_from_envelope_id = parent.envelope_id
```

A derived child must satisfy:

```text
permitted_actions(child)          subseteq permitted_actions(parent)
eligible_resources(child)         subseteq eligible_resources(parent)
trust_zones(child)                subseteq trust_zones(parent)
locality(child)                    no weaker than parent constraints
retry_budget(child)               <= retry_budget(parent)
parallel_redundancy(child)         <= parent ceiling
priority(child)                    <= parent priority ceiling
deadline(child)                    <= parent deadline
security/integrity requirements    not weaker than parent
```

A child-envelope schema proves only structural form; subset validation requires
the referenced parent envelope and is a conformance check.

`derived_from_envelope_id` does not mean supersession. The parent may remain
valid for other attempts.

### 5.5 Supersession

`supersedes_envelope_id` denotes replacement lifecycle, not derivation. An
upstream-authorized replacement may stop new pre-start work under the
superseded envelope. It cannot retroactively erase an attempt that already
crossed `SemanticExecutionStarted`.

### 5.6 Expiry

Expiry prevents new mechanical start/admission. It does not cancel AETHER
semantic execution that already started.

## 6. Envelope control records

### 6.1 `MechanicalEnvelopeRevoked`

Purpose: allow upstream governed authority to terminate future/pre-start use of
an envelope before natural expiry without inventing out-of-contract behavior.

Required conceptual fields:

```text
protocol_version
record_type = MechanicalEnvelopeRevoked
authority_owner = upstream_governed_record
control_id
envelope_id
authorization_ref
issuer_ref
revoked_at
reason
correlation_id?
integrity_profile_ref?
```

Rules:

1. FABRIC cannot self-issue a valid revocation merely because an endpoint is
   unhealthy or a queue is congested.
2. Once a conforming revocation is observed, no new/pre-start attempt may be
   admitted under the revoked envelope.
3. A revocation may cause queued-but-not-started attempts to end as
   `PreStartRejected(reason=envelope_revoked)`.
4. A revocation cannot convert an already recorded `SemanticExecutionStarted`
   into non-start or mechanical cancellation.
5. Revocation does not grant a replacement envelope. Replacement requires a new
   upstream `MechanicalEnvelopeAuthorized`.

The candidate schema is `envelope_control.schema.json`.

## 7. Mechanical events and attempt binding

Each mechanical event in a realization path contains:

```text
event_id
correlation_id
mechanical_attempt_id
envelope_id              # where the event is envelope-bound
occurred_at
```

The same mechanical attempt ID is used for:

```text
RouteRealized
QueueAdmitted | PreStartRejected
PayloadDispatched
PayloadDelivered
DeliveryReceiptObserved
```

Replica/object movement likewise requires a mechanical attempt ID when the
movement is envelope-authorized.

### `PreStartRejected`

Allowed reasons include:

```text
capacity_exhausted
queue_timeout
endpoint_unavailable
envelope_expired
envelope_revoked
envelope_superseded
protocol_incompatible
required_capability_missing
trust_zone_unsatisfied
mechanical_policy_unsatisfied
payload_unavailable
integrity_validation_failed
envelope_widening
```

A `PreStartRejected` record is invalid if AETHER already recorded
`SemanticExecutionStarted` bound to the same mechanical attempt.

## 8. AETHER semantic lifecycle handoff

The canonical handoff is:

```text
AETHER: NamespaceResolved
AETHER/POL: MechanicalEnvelopeAuthorized
FABRIC: RouteRealized(mechanical_attempt_id=M)
FABRIC: QueueAdmitted(M) | PreStartRejected(M)
AETHER: SemanticExecutionStarted(
          semantic_attempt_id=S,
          mechanical_envelope_id=E,
          mechanical_attempt_id=M)
AETHER: SemanticExecutionCompleted(S)
```

The local/no-FABRIC path is:

```text
AETHER: SemanticExecutionStarted(
          semantic_attempt_id=S,
          mechanical_envelope_id absent,
          mechanical_attempt_id absent)
```

The semantic-start schema requires the two mechanical binding fields together
or neither.

E1 does not add cooperative in-evaluation cancellation. That would require a
new AETHER semantic checkpoint ADR.

## 9. Telemetry evidence return

`TelemetryEvidenceObserved` may represent:

```text
endpoint_liveness
queue_depth
queue_wait
replication_lag
capacity
locality
latency
cost
transport_failure
backpressure
```

It is an observation, not an AETHER fact. Required governed path:

```text
TelemetryEvidenceObserved
 -> SemanticSubmissionProposed
 -> SemanticAdmissionAccepted | SemanticAdmissionRejected
 -> later allocator may consume admitted evidence
```

No direct FABRIC telemetry-to-policy mutation exists in E1.

## 10. Replica movement

Future FABRIC may move opaque bytes/prefixes only.

```text
AETHER: ReplicationMovementAuthorized / envelope
FABRIC: ReplicaMovementObserved(mechanical_attempt_id=M)
AETHER: validate source cut/prefix/leader epoch/fencing
AETHER: accept or reject semantic replica state
```

FABRIC cannot mint/modify `LeaderEpoch`, partition authority role, promotion
state, accepted cut, stale-epoch disposition or divergent-prefix disposition.

## 11. Sidecar/object locality

```text
AETHER: semantic object/vector identity + policy/provenance
FABRIC/storage: physical placement/cache/shard movement
FABRIC/storage: PhysicalObjectLocationObserved(mechanical_attempt_id=M)
AETHER: result submitted/admitted with provenance
```

Cache hit, nearest-neighbor rank and physical availability are not semantic
relevance, visibility or claim support.

## 12. Failure vocabulary

Mechanical states:

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

AETHER semantic lifecycle states:

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

## 13. Retry law

A retry:

- retains the higher-level `correlation_id`;
- creates a new `mechanical_attempt_id`;
- references the same or narrower valid envelope;
- consumes the remaining retry budget;
- does not reset expiry/supersession/revocation;
- cannot duplicate a semantic attempt that already started unless AETHER
  explicitly supplies an idempotent semantic retry contract.

## 14. Version negotiation

- unsupported protocol major: reject before start;
- unsupported required capability: reject before start;
- no implicit major downgrade;
- no dropping required constraints/capabilities;
- unknown required record type: fail closed;
- unknown optional audit material may be retained only if it causes no state
  transition.

## 15. Correlation law

`correlation_id` groups a cross-plane operation. `event_id`,
`mechanical_attempt_id`, `semantic_attempt_id`, `envelope_id`, `payload_ref`
and identity-stratum references remain distinct types.

Correlation is not an authority token.

## 16. Integrity boundary

Schema validation proves only structural conformance. Authority verification and
integrity verification are separate operations.

A future integrity profile must bind at least:

- protocol version;
- envelope/control ID;
- upstream authorization/issuer references;
- `scope_ref` and exact-byte `scope_digest`;
- payload reference;
- permitted actions/resources/zones;
- mechanical policy/fairness references;
- retry/redundancy/priority/deadline;
- derivation/supersession lineage;
- expiry/revocation identity;
- required capabilities.

Integrity failure rejects before start. There is no weaker fallback.

## 17. E0 ambiguity disposition

| E0 ambiguity | E1 candidate boundary | Status |
| --- | --- | --- |
| storage backend vs journal semantics | no FABRIC boundary yet | unresolved; D1/backend substitution required |
| sidecar locality vs semantics | `PhysicalObjectLocationObserved` + opaque semantic ref | candidate |
| replica movement vs fencing | `ReplicaMovementObserved`; AETHER validates epoch/prefix/cut | candidate |
| HTTP transport vs service semantics | transport-neutral protocol; HTTP remains AETHER gateway for now | unresolved; D1 required |
| namespace vs worker realization | semantic namespace before envelope; worker route inside envelope | candidate |
| queue/resource vs semantic lifecycle | pre-start mechanics vs explicit semantic start/completion | candidate |
| semantic vs operational limits | semantic limits stay AETHER; pure capacity may be envelope constraint | partially unresolved per limit |
| compatibility facade | explicitly not protocol owner | resolved as non-boundary |
| telemetry/performance | `TelemetryEvidenceObserved` plus admission path | candidate |

## 18. E1 completion boundary

E1 completes only when normative prose, schemas and conformance vectors agree;
Formalist/Adversary/Referee review finds no material contract defect; and exact-
head protected checks close.

E1 completion authorizes only E2 conformance-harness work. It does not authorize
runtime extraction, FABRIC activation, Article IX change, POL/AETHER-Learn
activation, GHOS-controller change, deployment or claim promotion.
