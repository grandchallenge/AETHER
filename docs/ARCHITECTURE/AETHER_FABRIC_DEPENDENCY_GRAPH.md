# AETHER/FABRIC Dependency Graph

Status: E0 candidate
Issue: #83

## Purpose

Record the allowed and forbidden dependency edges implied by the approved
AETHER/FABRIC separation doctrine before any code is extracted.

This is a semantic/control graph, not a deployment diagram.

## Canonical planes

```text
INTELLECT
  │ constitutional policy / office obligations / authority schedules
  ▼
POL / institutional applications
  │ work objects / claims / desired allocations / decisions
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

The graph does not imply that AETHER is an operational parent of FABRIC or
GHOS. It means only that semantic meaning and authority are not inferred from
mechanical or execution events.

## Allowed edges

### INTELLECT -> POL/AETHER

Allowed:

- constitutional policy;
- office obligations;
- admitted authority schedules;
- work-package and application contracts;
- approved semantic boundary rules.

Not implied:

- INTELLECT implementation code must run inside AETHER;
- a GitHub event is itself semantic truth merely because INTELLECT records it.

### POL -> AETHER

Allowed:

- typed institutional objects projected as semantic facts;
- work-object identity;
- claims/evidence/critique/verification/decision records;
- desired allocation decisions;
- charter/office/guild semantics.

Required property:

> POL describes institutional meaning; AETHER provides the admitted semantic
> state and derivation substrate on which those objects can be represented.

### AETHER -> FABRIC

Allowed:

- an opaque or mechanically checkable `MechanicalEnvelopeAuthorized` record;
- endpoint eligibility constraints already determined by upstream authority;
- payload references;
- delivery class, redundancy, TTL, locality/trust-zone constraints;
- correlation identifiers;
- retry budget and bounded mechanical priority class.

Forbidden content in this edge:

- permission expansion;
- semantic-admission decisions;
- institutional utility functions invented by FABRIC;
- authority epochs minted by FABRIC;
- claim acceptance/rejection delegated merely for convenience.

### FABRIC -> AETHER

Allowed only as **evidence return**, never self-authenticating truth:

- delivery observations;
- route realization;
- endpoint/resource liveness;
- replication lag;
- queue depth/backpressure;
- capacity observations;
- transport failure;
- locality/cost/latency observations.

Required path:

```text
FABRIC telemetry
  -> provenance-bearing semantic submission
  -> AETHER admission/rejection
  -> optional POL/AETHER-Learn use in later allocation
```

Forbidden shortcut:

```text
FABRIC telemetry -> direct institutional policy mutation
```

### AETHER/POL -> GHOS

Allowed:

- an already governed request for protected execution;
- artifact/command contract references;
- exact subject identity;
- execution preconditions.

GHOS retains its own controller-admission and credential rules.

### GHOS -> AETHER

Allowed:

- execution evidence;
- artifact identity/digest;
- controller/run/attempt identity;
- bounded execution outcome;
- failure state.

Required path:

> execution evidence must still be semantically admitted before it becomes an
> AETHER fact or POL decision input.

### FABRIC <-> GHOS

Potentially allowed for mechanical movement of execution payloads/results only
if a future reviewed contract exists.

Neither direction may imply controller admission or semantic authority.

## Explicitly forbidden edges

### FABRIC -> semantic admission

Forbidden:

```text
PayloadDelivered => SemanticAdmissionAccepted
```

Delivery success establishes only an operational observation.

### FABRIC liveness -> institutional authority

Forbidden:

```text
endpoint_healthy(x) => actor_authorized(x)
```

A resource may be available and still unauthorized.

### FABRIC replica -> authority replica

Forbidden:

```text
bytes_equal(replica_a, replica_b) => authority_equal(replica_a, replica_b)
```

AETHER leader epoch/cut/fencing semantics remain authoritative.

### FABRIC scheduling -> institutional choice

Forbidden:

- changing guild/agent eligibility;
- suppressing an evidence class;
- starving work because of an undeclared institutional preference;
- using locality/cost policy to override a POL allocation;
- treating retry exhaustion as evidence that a claim is false.

### AETHER -> GHOS controller admission

Forbidden:

AETHER cannot make itself, FABRIC, or another service an admitted persistent
controller by emitting a semantic fact.

### GHOS -> AETHER semantic bypass

Forbidden:

A protected controller completing work does not directly mutate AETHER truth
without normal admission/provenance.

### INTELLECT authority -> implementation inference

Forbidden:

No component may infer constitutional power from repository location, process
ownership, service identity, or co-location with INTELLECT artifacts.

## Normative event chain

The architecture should preserve the following typed separation:

```text
1. AllocationDesired
2. MechanicalEnvelopeAuthorized
3. RouteRealized
4. PayloadDispatched
5. PayloadDelivered
6. DeliveryReceiptObserved
7. ExternalExecutionRequested        [only when GHOS path is required]
8. ExternalExecutionCompleted/Failed [only when GHOS path is required]
9. SemanticSubmissionProposed
10. SemanticAdmissionAccepted | SemanticAdmissionRejected
11. InstitutionalDecisionRecorded
```

No later event retroactively proves an earlier authorization unless a normative
contract explicitly states the derivation and the required evidence is present.

## Current live-main mapping

### AETHER semantic core

Current strong inward semantic dependency chain:

```text
aether_ast
  -> aether_schema
  -> aether_resolver
  -> aether_plan / aether_rules
  -> aether_runtime
  -> aether_explain
  -> aether_service_core
```

`aether_storage` supplies authoritative journal persistence beneath resolver and
service admission. Its local backend mechanics remain an E0 ambiguity rather
than a FABRIC assignment.

### Current mixed outer crates

```text
aether_service_core
  -> aether_pilot       [application/institutional proof layer]
  -> aether_partition   [semantic partition/federation + mixed replication mechanics]
  -> aether_http        [semantic service edge + mixed queue/routing mechanics]
  -> aether_sidecar     [semantic reference contract + mixed payload locality]
```

The approved architecture does not authorize moving those crates wholesale.
E1 must first expose narrower contracts inside the mixed edges.

## Desired future dependency law

Subject to E1/E2 evidence, the intended architecture is:

```text
POL / AETHER-Learn
       │ desired allocation
       ▼
AETHER
       │ authorized mechanical envelope
       ▼
FABRIC
       │ route / delivery / placement / telemetry
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

## AETHER-MEM / sidecar dependency law

Allowed:

```text
AETHER semantic memory identity/provenance/policy
  -> opaque physical object/shard reference
  -> FABRIC/storage locality and movement
  -> retrieved payload/result
  -> semantic submission/admission
```

Forbidden:

```text
cache_hit => semantically_relevant
nearest_neighbor_rank => accepted_claim
object_available => authorized_visibility
```

## AETHER-Learn dependency law

No live AETHER-Learn implementation is present on current AETHER `main` under
E0 search. The future contract should nevertheless preserve:

```text
context + work + capability evidence + outcome ledger
   -> governed allocator / no-regret learner
   -> AllocationDesired
   -> AETHER record/admission
   -> FABRIC physical realization
```

FABRIC may return cost/latency/liveness evidence but must not directly mutate
the allocator's institutional objective.

## Failure containment invariants

### FABRIC unavailable

Required outcome:

- distributed movement/placement may fail or degrade;
- pending work is marked unknown/undelivered as appropriate;
- local AETHER append/replay/derivation remains correct;
- AETHER does not convert transport silence into negative evidence.

### AETHER unavailable

Required outcome for a future independent FABRIC:

- FABRIC may continue or fail its bounded mechanical operations according to
  contract;
- it cannot invent semantic admission, new authority, or policy relaxation;
- buffered results remain unadmitted until AETHER returns.

### GHOS unavailable

Required outcome:

- protected external execution cannot be silently substituted with FABRIC or
  an AETHER process;
- requests remain blocked/pending according to the execution contract.

## E1 contract targets emerging from this graph

1. `MechanicalEnvelopeAuthorized` schema.
2. Endpoint/resource identity schema distinct from semantic actor identity.
3. `RouteRealized` / delivery event schema.
4. Telemetry evidence-return schema.
5. Replica-movement contract that excludes epoch/fencing authority.
6. Namespace-resolution versus worker-realization split.
7. Sidecar physical-location contract.
8. Cross-plane correlation/trace identity.
9. Failure vocabulary distinguishing unknown, undelivered, delivered,
   unadmitted, admitted, and rejected.
10. Version negotiation and fail-closed incompatibility behavior.
