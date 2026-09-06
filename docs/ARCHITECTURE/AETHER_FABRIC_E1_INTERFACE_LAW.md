# AETHER/FABRIC E1 Interface Law

Status: E1 candidate
Issue: `AETHER-FABRIC-ENCAP-E1-001` / #85
AETHER basis: protected `main` `411411dfcc29757bbf68589b817b8bffeceb7bcb`
Council basis: INTELLECT `18df1c3ef89712fe00af37a484cc03ce270e99bd`
E0 basis: AETHER ADR 0020 and E0 architecture packet merged at `411411df...`

## 1. Purpose

This document defines the smallest versioned contract boundary by which AETHER
may later ask a separately encapsulated mechanical FABRIC to realize already
authorized movement, placement, rendezvous, queue admission, replication
movement, or similar physical work.

E1 is interface law only. It does not move implementation, create a FABRIC
runtime, change Article IX, or grant any new authority.

The design target is:

> AETHER owns admitted semantic state and semantic lifecycle. FABRIC may realize
> bounded mechanical work. A successful mechanical event never manufactures a
> semantic, institutional, constitutional, or GHOS-controller event.

## 2. Protocol identity

The E1 contract family is identified as:

```text
protocol_family = aether-fabric
protocol_major  = 1
protocol_minor  = 0
```

Canonical textual form:

```text
aether-fabric/1.0
```

E1 records are **candidate schemas for conformance work**. Their presence in the
repository does not make them a production transport protocol.

## 3. Plane ownership

### 3.1 AETHER owns

- semantic namespace identity;
- append/admission decisions;
- journal cuts and replay;
- policy visibility;
- provenance and proof identity;
- semantic coordination facts;
- same-namespace semantic ordering where the live contract requires it;
- `cancel_before_start_complete_after_start` semantic lifecycle;
- semantic execution start/completion;
- semantic lease/fence meaning;
- partition/cut identity;
- leader epoch, promotion authority, stale-epoch rejection and divergent-prefix
  fencing;
- semantic sidecar identity/provenance/policy;
- interpretation of returned telemetry as evidence after admission.

### 3.2 FABRIC may later own

Only after E2/E3 evidence for the corresponding seam:

- endpoint/resource observation;
- concrete route/placement realization;
- global worker-pool capacity realization;
- pre-start queue admission/rejection;
- queue depth/wait/backpressure observation;
- delivery and retry mechanics;
- replica byte/prefix movement beneath AETHER fencing;
- physical artifact/vector locality and movement;
- operational liveness/capacity/locality/cost/latency observations.

### 3.3 POL / governed allocator owns

- desired institutional allocation;
- eligible actor/guild class as an institutional choice;
- institutional objective/utility;
- priority among ends;
- interpretation of accepted evidence for future allocation.

AETHER may record these decisions and provide the authorized semantic reference
from which a mechanical envelope is derived.

### 3.4 GHOS owns

- persistent-controller admission;
- protected execution routing;
- execution credential authority;
- protected external mutation semantics.

FABRIC transport cannot create GHOS controller admission.

### 3.5 INTELLECT owns

- constitutional policy;
- office powers/obligations;
- constitutional authority schedules;
- Article IX and related boundary law.

## 4. Core invariants

The E1 interface is invalid if any implementation requires violating these
invariants.

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

### I4 — resource possession is not institutional eligibility

```text
ResourceAvailable !=> InstitutionallyEligible
```

### I5 — transport failure is not negative evidence

```text
Undelivered !=> ClaimFalse
PreStartRejected !=> SemanticRejected
```

### I6 — started semantic work cannot be mechanically erased

Once AETHER emits `SemanticExecutionStarted`, FABRIC cannot convert the
operation into `PreStartRejected`, cannot report it safely cancelled, and cannot
cause AETHER to return a timeout while the operation may still commit in the
background.

### I7 — mechanical optimization is bounded

FABRIC may optimize only within an immutable `MechanicalEnvelopeAuthorized`.
It may not widen endpoint eligibility, trust zones, retry budget, priority
class, deadline, redundancy, payload scope, or permitted mechanical action.

### I8 — mechanical policy must be attributable

Priority, fairness, retry, eligibility and locality policies used for a
consequential mechanical realization must be referenced by stable policy IDs
and bound to the envelope. An anonymous scheduler default cannot silently act as
institutional policy.

### I9 — telemetry is evidence, not a policy write

FABRIC telemetry may be submitted to AETHER with provenance. It must not mutate
POL/AETHER-Learn objective, actor eligibility, semantic authority, or policy
visibility directly.

### I10 — downgrade is fail-closed

A participant that cannot satisfy the required protocol major version or
required capability set rejects the envelope before mechanical work begins. It
does not silently reinterpret or downgrade the request.

## 5. Canonical record classes

The E1 protocol defines the following record classes.

### 5.1 `MechanicalEnvelopeAuthorized`

Purpose: bind one already-authorized semantic/institutional decision to a closed
mechanical realization envelope.

Required conceptual fields:

```text
protocol_version
record_type = MechanicalEnvelopeAuthorized
envelope_id
correlation_id
authorization_ref
issuer_ref
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
deadline_or_ttl
expires_at
supersedes_envelope_id?
required_capabilities[]
```

`authorization_ref` and `issuer_ref` are opaque cross-plane references. FABRIC
may structurally require them and may verify an external integrity binding if a
future security contract supplies one, but E1 does not authorize FABRIC to
interpret those references as new institutional powers.

The envelope contains no field named or semantically equivalent to:

- `grant_role`;
- `grant_permission`;
- `accept_claim`;
- `promote_replica_authority`;
- `admit_controller`;
- `override_policy_visibility`;
- `amend_constitution`.

Such effects are outside the contract.

### 5.2 `RouteRealized`

Purpose: state the concrete mechanical route/endpoint selected inside the
closed envelope.

Conceptual fields:

```text
record_type = RouteRealized
protocol_version
event_id
correlation_id
envelope_id
endpoint_id
resource_class
route_class
mechanical_policy_ref
realized_at
```

`RouteRealized` is an observation of realization, not proof that upstream
institutional allocation was valid. Its legitimacy is evaluated by checking it
against the referenced envelope.

### 5.3 `QueueAdmitted`

Purpose: record that mechanical pre-start capacity accepted the operation.

```text
record_type = QueueAdmitted
event_id
correlation_id
envelope_id
endpoint_id
queue_class
admitted_at
```

This event does not mean semantic execution has started.

### 5.4 `PreStartRejected`

Purpose: record that mechanical realization failed **before** AETHER semantic
execution began.

Required reason vocabulary:

```text
capacity_exhausted
queue_timeout
endpoint_unavailable
envelope_expired
envelope_superseded
protocol_incompatible
required_capability_missing
trust_zone_unsatisfied
mechanical_policy_unsatisfied
payload_unavailable
```

A `PreStartRejected` event is invalid if `SemanticExecutionStarted` has already
been recorded for the same semantic attempt.

### 5.5 `PayloadDispatched`, `PayloadDelivered`, `DeliveryReceiptObserved`

These records describe mechanical movement only. They must preserve the same
`correlation_id`, `envelope_id`, and opaque `payload_ref`.

No delivery record may contain or imply a semantic disposition.

### 5.6 `SemanticExecutionStarted` / `SemanticExecutionCompleted`

These are AETHER-owned lifecycle events, not FABRIC events.

They bind the semantic attempt to the AETHER namespace, semantic input identity,
and AETHER execution/trace identity defined by existing semantic contracts.

Required law:

```text
QueueAdmitted may precede SemanticExecutionStarted.
PreStartRejected and SemanticExecutionStarted are mutually exclusive for the
same semantic attempt.
SemanticExecutionStarted requires eventual AETHER-owned completion/failure
handling under the semantic contract; a FABRIC timeout cannot erase it.
```

E1 does not add cooperative in-evaluation cancellation. That would require a new
AETHER semantic checkpoint ADR.

### 5.7 `TelemetryEvidenceObserved`

Purpose: transport operational observations into a provenance-bearing evidence
submission path.

Observation classes may include:

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

Required conceptual fields:

```text
record_type = TelemetryEvidenceObserved
event_id
correlation_id?
endpoint_id?
observation_class
observed_value
unit?
observed_at
observer_id
source_ref?
```

This record is **not** an AETHER fact merely because FABRIC emitted it. It must
be submitted/admitted through the ordinary AETHER provenance path before a
governed allocator may treat it as institutional evidence.

### 5.8 `ReplicaMovementObserved`

Purpose: report physical prefix/data movement without asserting semantic
authority.

It may name:

- source endpoint;
- destination endpoint;
- opaque payload/prefix reference;
- bytes/items transferred;
- transport result;
- observed lag after movement.

It must not mint or modify:

- AETHER `LeaderEpoch`;
- partition authority role;
- semantic promotion state;
- accepted partition cut;
- stale-epoch disposition;
- divergent-prefix disposition.

### 5.9 `PhysicalObjectLocationObserved`

Purpose: represent physical artifact/vector location, shard presence, cache
presence or movement.

It must bind to an opaque AETHER semantic object reference but must not assert:

- semantic relevance;
- visibility authorization;
- claim support;
- accepted nearest-neighbor meaning;
- semantic provenance beyond the opaque source reference.

## 6. Mechanical envelope semantics

### 6.1 Closed-world interpretation

For E1, the envelope is closed-world: only declared actions/resources/zones and
bounded policies are permitted.

Unknown `permitted_action`, unknown required capability, unknown protocol major,
or missing required constraint is a pre-start failure.

### 6.2 Monotonic restriction

A downstream mechanical hop may **narrow** but never widen an envelope.

If `E0` is the authorized envelope and `E1` is a derived downstream envelope,
then:

```text
permitted_actions(E1)           subseteq permitted_actions(E0)
eligible_resources(E1)          subseteq eligible_resources(E0)
trust_zones(E1)                 subseteq trust_zones(E0)
retry_budget(E1)                <= retry_budget(E0)
redundancy(E1)                  <= redundancy(E0), unless E0 states a range
priority(E1)                    cannot outrank E0's declared ceiling
deadline(E1)                    <= deadline(E0)
required_security_constraints   cannot be removed
```

A widening attempt fails before dispatch.

### 6.3 Supersession

A superseding envelope must reference the previous `envelope_id`. FABRIC may
stop admitting new pre-start work under the superseded envelope. It cannot
retroactively erase `SemanticExecutionStarted` under the prior envelope.

### 6.4 Expiry

Expiry prevents new mechanical start/admission. It does not cancel AETHER
semantic execution that already started.

## 7. Scheduling handoff

E1 formalizes the E0 resource-control correction.

```text
AETHER: NamespaceResolved
AETHER/POL: MechanicalEnvelopeAuthorized
FABRIC: RouteRealized
FABRIC: QueueAdmitted | PreStartRejected
AETHER: SemanticExecutionStarted
AETHER: SemanticExecutionCompleted
AETHER: semantic receipt/trace persistence
```

FABRIC owns capacity realization only up to the semantic start boundary.

The following remain AETHER invariants:

- deterministic same-namespace order where current service law requires one
  active semantic operation;
- no partial append on resource rejection;
- no semantic execution receipt/trace handle on a pre-start rejection;
- once started, complete according to the AETHER semantic lifecycle.

## 8. Replica movement handoff

Future mechanical replication may move opaque bytes/prefixes only.

Required sequence:

```text
AETHER: ReplicationMovementAuthorized
FABRIC: ReplicaMovementObserved
AETHER: validate source cut/prefix/epoch/fencing
AETHER: accept or reject semantic replica state
```

A mechanically healthy, byte-equal follower remains non-authoritative unless
AETHER's own epoch/promotion/fencing contract says otherwise.

## 9. Sidecar/object-location handoff

Future physical locality may be mechanically realized as:

```text
AETHER: semantic object/vector reference + visibility/provenance contract
FABRIC/storage: physical placement/cache/shard movement
FABRIC/storage: PhysicalObjectLocationObserved
AETHER: provenance-bearing result submission/admission
```

Cache hit, nearest-neighbor rank and physical availability are observations,
not semantic judgments.

## 10. Failure vocabulary

E1 distinguishes mechanical and semantic state explicitly.

### Mechanical states

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

### AETHER semantic lifecycle states

```text
not_started
started
completed
submission_proposed
unadmitted
admitted
rejected
```

The vocabularies must not be collapsed. In particular:

```text
transport_failed != rejected
pre_start_rejected != rejected
completed != admitted
delivered != admitted
```

## 11. Version negotiation

### 11.1 Major version

A participant must reject an envelope whose `protocol_major` it does not
support exactly.

### 11.2 Minor version

Minor versions may add optional non-authority-bearing fields or optional
capabilities. A sender declares `required_capabilities`. A receiver must reject
before start if any required capability is unknown or unsupported.

### 11.3 No implicit downgrade

A receiver may not silently transform `aether-fabric/2.x` into `1.x`, remove a
required capability, or omit a security/mechanical constraint to make a request
routable.

### 11.4 Unknown records

Unknown `record_type` values fail closed for required execution paths. They may
be retained as opaque audit material only if the retaining component makes no
state transition based on them.

## 12. Correlation law

All records participating in one cross-plane operation share a stable
`correlation_id`.

Each event has its own `event_id`. A later event may carry a `causation_id` that
references the immediate prior event, but correlation does not imply authority.

A complete audit can reconstruct:

```text
institutional allocation
 -> mechanical authorization
 -> route/queue/delivery
 -> optional GHOS execution
 -> semantic submission/admission
 -> institutional decision
```

without treating these as one undifferentiated workflow state.

## 13. Security/integrity boundary

E1 defines what must be bound, not the final cryptographic mechanism.

A future integrity profile must bind at least:

- protocol version;
- envelope ID;
- authorization reference;
- issuer reference;
- scope digest;
- payload reference;
- permitted actions;
- eligible resource classes;
- trust/locality constraints;
- mechanical policy reference;
- expiry/supersession;
- required capabilities.

If integrity cannot be verified under the selected profile, FABRIC rejects
before start. No fallback may widen the envelope.

## 14. E0 ambiguity disposition under E1

| E0 ambiguity | E1 candidate boundary | Status after E1 drafting |
| --- | --- | --- |
| storage backend mechanics vs journal semantics | storage adapter remains outside FABRIC protocol for now; append/cut/receipt law stays AETHER | unresolved; E2 backend substitution needed |
| sidecar locality vs semantic contract | `PhysicalObjectLocationObserved` + opaque semantic ref | candidate boundary |
| replica movement vs epoch/fencing | `ReplicaMovementObserved`; AETHER validates epoch/prefix/cut | candidate boundary |
| HTTP transport vs semantic service | protocol is transport-neutral; HTTP remains AETHER gateway until reference adapter exists | unresolved; D1 needed |
| namespace vs worker realization | `NamespaceResolved` before mechanical envelope; worker route inside envelope | candidate boundary |
| resource scheduling vs semantic lifecycle | queue/pre-start mechanical; start/completion semantic | candidate boundary |
| semantic vs operational limits | semantic limits remain AETHER; pure transport/capacity limits may be envelope/FABRIC constraints | partially resolved; per-limit E2 evidence still needed |
| compatibility facade | explicitly not a protocol owner | resolved as non-boundary |
| cross-plane perf/telemetry | `TelemetryEvidenceObserved` with admitted evidence return | candidate boundary |

## 15. E1 completion boundary

E1 may be considered complete when the interface law, event algebra, identity
law, conformance vectors, candidate schemas and ADR agree on these invariants and
independent review finds no material authority laundering or event collapse.

E1 completion is **not** code-extraction authority. E2 must execute the D1-D5
conformance/falsification programme before any corresponding responsibility is
moved behind a FABRIC runtime boundary.
