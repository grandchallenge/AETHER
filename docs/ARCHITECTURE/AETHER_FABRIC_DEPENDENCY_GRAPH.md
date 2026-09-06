# AETHER/FABRIC Dependency Graph

Status: E0 candidate, revised after Adversary pass
Issue: #83

## Purpose

Record allowed, forbidden, and evidence-return dependency edges before any code
is extracted. This is a semantic/control graph, not a deployment diagram.

## Canonical planes

```text
INTELLECT
  │ constitutional policy / office obligations / authority schedules
  ▼
POL / institutional applications
  │ work objects / claims / desired allocation / decisions
  ▼
AETHER
  │ authoritative semantic state within admitted domain
  │ append / cuts / replay / policy / provenance / derivation / admission
  ├────────────────────────────┐
  │                            │
  ▼                            ▼
FABRIC                       GHOS
mechanical realization       protected execution/control
  │                            │
  └──────────────┬─────────────┘
                 ▼
       endpoints / stores / GPUs / services
```

This does not make AETHER an operational parent of FABRIC or GHOS. It states
only that mechanical/execution events cannot manufacture semantic or
constitutional authority.

## Allowed edges

### INTELLECT -> POL/AETHER

May supply constitutional policy, office obligations, authority schedules,
work-package/application contracts, and approved semantic boundary rules.
Repository/process location does not itself create semantic truth.

### POL -> AETHER

May project typed institutional objects: work, claims, evidence, critiques,
verifications, decisions, charters, offices/guilds, and desired allocations.
AETHER admits/derives their semantic representation.

### AETHER -> FABRIC

May provide a versioned `MechanicalEnvelopeAuthorized` containing only bounded
mechanical realization constraints, for example:

- eligible endpoint/resource class already authorized upstream;
- payload reference;
- correlation ID;
- trust/locality zone;
- cost/latency ceiling;
- retry budget;
- redundancy;
- TTL/deadline;
- bounded mechanical priority.

FABRIC may optimize within the envelope. It may not widen eligibility,
permission, policy visibility or institutional objective.

### FABRIC -> AETHER

May return operational evidence:

- route realization;
- delivery/failure;
- endpoint liveness;
- replica lag;
- queue depth/wait/backpressure;
- capacity/locality/cost/latency.

Required path:

```text
FABRIC observation
  -> provenance-bearing semantic submission
  -> AETHER admission/rejection
  -> optional later POL/AETHER-Learn use
```

Forbidden shortcut:

```text
FABRIC telemetry -> direct policy/authority mutation
```

### AETHER/POL -> GHOS

May submit an already governed protected-execution request with exact subject,
artifact/command contract and preconditions. GHOS retains controller admission,
credential and execution policy.

### GHOS -> AETHER

May return run/controller/attempt/artifact/outcome evidence. It becomes AETHER
state only through normal semantic submission/admission.

### FABRIC <-> GHOS

A future reviewed bridge may transport payloads/results. Transport never implies
controller admission or semantic authority.

## Resource scheduling split

The current AETHER service resource-control contract is intentionally split
across the AETHER/FABRIC boundary rather than assigned wholesale to FABRIC.

### Mechanical side — possible FABRIC

Under a fixed semantic namespace and authorized mechanical envelope:

- choose a concrete worker;
- enforce global worker-pool capacity;
- observe queue occupancy/wait;
- apply pre-start queue timeout/backpressure;
- report capacity/unavailability.

These may change latency or whether work starts. They may not change what an
operation means once it starts.

### Semantic side — remains AETHER

AETHER retains:

- semantic namespace resolution;
- deterministic same-namespace serialization where required;
- the `cancel_before_start_complete_after_start` lifecycle;
- the rule that a pre-start timeout cannot later produce a hidden semantic
  commit;
- the rule that once semantic execution starts, a mechanical timeout cannot
  return failure while a mutation may still commit in the background;
- no-partial-authority/no-semantic-receipt guarantees on resource rejection;
- semantic runtime/rule/tuple boundedness requirements.

Normative realization sequence:

```text
NamespaceResolved                 [AETHER]
MechanicalEnvelopeAuthorized      [AETHER/POL contract]
WorkerRouteRealized               [FABRIC]
QueueAdmitted | PreStartRejected  [FABRIC]
SemanticExecutionStarted          [AETHER]
SemanticExecutionCompleted        [AETHER]
SemanticReceiptPersisted          [AETHER]
```

Once `SemanticExecutionStarted` exists, FABRIC may observe or wait; it cannot
reinterpret the operation as cancelled. If cooperative in-evaluation
cancellation is ever added, it requires a separate AETHER semantic checkpoint
contract and ADR, matching the live `RESOURCE_CONTROL_CONTRACT.md`.

## Explicitly forbidden implications

```text
PayloadDelivered => SemanticAdmissionAccepted             [forbidden]
EndpointHealthy => ActorAuthorized                         [forbidden]
ReplicaBytesEqual => ReplicaAuthorityEqual                 [forbidden]
QueueTimedOutAfterSemanticStart => SemanticOperationGone   [forbidden]
ResourceAvailable => InstitutionallyEligible               [forbidden]
GHOSExecutionCompleted => AETHERFactAccepted               [forbidden]
AETHERFactSaysController(x) => GHOSControllerAdmitted(x)   [forbidden]
```

FABRIC also may not use scheduling, starvation, priority, retry asymmetry,
locality or selective delivery to implement undeclared institutional policy.

## Normative cross-plane event chain

```text
1.  AllocationDesired
2.  MechanicalEnvelopeAuthorized
3.  RouteRealized
4.  QueueAdmitted | PreStartRejected
5.  PayloadDispatched
6.  PayloadDelivered
7.  DeliveryReceiptObserved
8.  ExternalExecutionRequested        [GHOS path only]
9.  ExternalExecutionCompleted/Failed [GHOS path only]
10. SemanticSubmissionProposed
11. SemanticAdmissionAccepted | SemanticAdmissionRejected
12. InstitutionalDecisionRecorded
```

For a direct AETHER semantic operation, `SemanticExecutionStarted` and
`SemanticExecutionCompleted` additionally bracket the semantic lifecycle. A
mechanical timeout can prevent start; it cannot erase a start that already
occurred.

## Current live-main dependency structure

### Semantic center

```text
aether_ast
  -> aether_schema
  -> aether_resolver
  -> aether_plan / aether_rules
  -> aether_runtime
  -> aether_explain
  -> aether_service_core
```

`aether_storage` supplies authoritative persistence beneath semantic admission.
Backend I/O remains an E0 ambiguity rather than a presumed FABRIC layer.

### Mixed outer surfaces

```text
aether_service_core
  -> aether_pilot       institutional/product proof layer
  -> aether_partition   semantic partition/federation + mixed replica mechanics
  -> aether_http        semantic service edge + mixed worker/queue mechanics
  -> aether_sidecar     semantic reference contract + mixed payload locality
```

ADR 0019 already establishes these as responsibility crates rather than one API
catch-all. The AETHER/FABRIC split requires a **finer** decomposition; it does
not reverse ADR 0019 by moving whole outer crates.

## Desired future law

Subject to E1/E2 evidence:

```text
POL / AETHER-Learn
       │ desired allocation
       ▼
AETHER
       │ authorized mechanical envelope
       ▼
FABRIC
       │ route / pre-start queue / delivery / placement / telemetry
       ├───────────────┐
       │               │
       ▼               ▼
ordinary endpoint     GHOS-admitted execution path
       │               │
       └──────┬────────┘
              ▼
       result / evidence
              │
              ▼
AETHER semantic submission + admission
              │
              ▼
POL judgment / memory / later allocation
```

AETHER semantic execution lifecycle constraints continue to apply after FABRIC
has mechanically realized a route.

## AETHER-MEM / sidecar law

Allowed:

```text
AETHER memory identity/provenance/policy
 -> opaque physical object/shard reference
 -> FABRIC/storage locality and movement
 -> retrieved result
 -> AETHER semantic submission/admission
```

Forbidden:

```text
cache_hit => semantically_relevant
nearest_neighbor_rank => accepted_claim
object_available => authorized_visibility
```

## AETHER-Learn law

No live implementation was found on current AETHER main under E0 search. A
future contract should be:

```text
work + capability evidence + outcome ledger + admitted FABRIC telemetry
 -> governed allocator / no-regret learner
 -> AllocationDesired
 -> AETHER record/admission
 -> FABRIC physical realization
```

FABRIC may supply evidence; it cannot silently change the institutional
objective or eligible actor set.

## Failure containment

### FABRIC unavailable

- distributed movement/placement/queue admission may degrade;
- pending work is unknown/undelivered, not false;
- local AETHER append/replay/derivation remains semantically correct;
- started AETHER operations obey AETHER completion/atomicity semantics.

### AETHER unavailable

A future FABRIC may continue only already bounded mechanical work allowed by its
contract. It may not invent semantic admission, authority or policy relaxation.
Buffered results remain unadmitted.

### GHOS unavailable

Protected external execution remains blocked/pending. FABRIC or AETHER cannot
silently substitute themselves as controllers.

## E1 targets

1. `MechanicalEnvelopeAuthorized` schema.
2. Endpoint/resource identity distinct from semantic actor/office/controller.
3. Route/pre-start queue/delivery event schemas.
4. `SemanticExecutionStarted/Completed` handoff contract.
5. Telemetry evidence-return schema.
6. Replica movement contract excluding epoch/fencing authority.
7. Namespace-resolution versus worker-realization contract.
8. Sidecar physical-location contract.
9. Cross-plane correlation identity.
10. Failure vocabulary: unknown, undelivered, pre-start rejected, started,
    completed, submitted, unadmitted, admitted, rejected.
11. Fail-closed version negotiation.
12. D2/D5 hostile scheduling and resource-control conformance tests.