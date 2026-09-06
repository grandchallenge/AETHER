# ADR 0021 — AETHER/FABRIC E1 Versioned Interface Law

Status: Proposed for E1 review
Date: 2026-09-05
Issue: #85
Protocol candidate: `aether-fabric/1.0`

## Context

E0 classified the live AETHER estate at responsibility granularity and was
merged at `411411dfcc29757bbf68589b817b8bffeceb7bcb`.

The central E0 result was that the AETHER/FABRIC seam is real but cuts through
existing outer responsibilities rather than aligning with whole crates.

E0 retained in AETHER:

- append/admission, cuts/replay, policy, provenance and proof identity;
- semantic coordination lease/fence meaning;
- partition/federation semantics and leader epoch/fencing;
- semantic namespace identity;
- same-namespace semantic serialization and started-operation lifecycle;
- sidecar semantic identity/provenance/policy;
- no-partial-authority guarantees.

E0 identified mechanical candidates:

- endpoint/resource liveness and location;
- concrete route/worker realization;
- global capacity and pre-start queue/backpressure;
- replica byte/prefix movement beneath AETHER fencing;
- physical artifact/vector locality;
- audit-event transport;
- host/capacity/locality/cost/latency observations.

The Council required versioned contracts before any extraction and required
FABRIC influence to remain bounded by attributable mechanical envelopes.

## Decision

Adopt a candidate E1 contract family named:

```text
aether-fabric/1.0
```

The contract is transport-neutral and non-runtime in E1.

It defines:

1. a closed `MechanicalEnvelopeAuthorized` that may be narrowed but not widened;
2. distinct identity strata for endpoint/resource, semantic actor,
   institutional principal and GHOS controller;
3. a typed event algebra separating institutional allocation, mechanical
   realization, semantic execution/admission and protected execution;
4. a resource handoff boundary at `SemanticExecutionStarted`;
5. evidence-return telemetry rather than direct policy mutation;
6. replica movement without authority promotion;
7. physical object/vector locality without semantic relevance/visibility;
8. fail-closed protocol/capability negotiation;
9. stable cross-plane correlation without treating correlation as authority;
10. conformance vectors to be executed in E2.

## Mechanical envelope law

The mechanical envelope binds an already-authorized upstream decision to an
immutable mechanical scope.

A downstream hop may narrow the envelope but cannot widen:

- permitted actions;
- eligible resource classes;
- trust/locality constraints;
- retry budget;
- priority ceiling;
- deadline/TTL;
- security requirements.

FABRIC cannot self-issue a broader superseding envelope.

Expiry/supersession prevents new pre-start work but does not erase AETHER
semantic execution that already started.

## Semantic-start law

The resource-control boundary is:

```text
AETHER: semantic namespace resolution
AETHER/POL: mechanical envelope authorization
FABRIC: route + pre-start queue admission/rejection
AETHER: SemanticExecutionStarted
AETHER: SemanticExecutionCompleted
```

Before semantic start, FABRIC may reject for capacity, queue timeout,
unsupported capability, trust-zone failure, expiry or other declared mechanical
reason.

After semantic start, FABRIC may report operational failure but cannot
reinterpret the operation as cancelled or safely not-started.

This preserves the existing AETHER
`cancel_before_start_complete_after_start` contract.

## Identity law

Four identity strata remain non-collapsible:

```text
EndpointResourceId
SemanticActorRef
InstitutionalPrincipalRef
ControllerRef
```

Textual equality, co-location, reachability, liveness, capability, common
credential material or matching correlation IDs do not collapse identity kinds
or create authority.

Cross-stratum relationships require explicit bounded bridge/governing records.

## Event law

The core event families are:

```text
AllocationDesired                         INSTITUTIONAL
MechanicalEnvelopeAuthorized              bridge/authorized upstream
RouteRealized                             MECHANICAL
QueueAdmitted | PreStartRejected          MECHANICAL
PayloadDispatched / PayloadDelivered      MECHANICAL
DeliveryReceiptObserved                   MECHANICAL
SemanticExecutionStarted/Completed        SEMANTIC
SemanticSubmissionProposed                SEMANTIC
SemanticAdmissionAccepted/Rejected        SEMANTIC
ExternalExecutionRequested/Started/...    EXECUTION / GHOS
InstitutionalDecisionRecorded             INSTITUTIONAL
TelemetryEvidenceObserved                 MECHANICAL observation
ReplicaMovementObserved                   MECHANICAL observation
PhysicalObjectLocationObserved            MECHANICAL observation
```

Mechanical events never imply semantic or institutional dispositions.

## Version law

- unknown protocol major: reject before start;
- unsupported required capability: reject before start;
- no implicit major downgrade;
- no dropping required capabilities/constraints to make work routable;
- unknown required record type: fail closed;
- unknown optional audit material may be retained only if it causes no state
  transition.

## Telemetry law

Operational observations may include liveness, queue depth/wait, replication
lag, capacity, locality, latency, cost, backpressure and transport failure.

They enter governed reasoning only through:

```text
mechanical observation
 -> provenance-bearing semantic submission
 -> AETHER admission/rejection
 -> later governed allocation/decision
```

No direct telemetry-to-policy edge is admitted in E1.

## Replica law

FABRIC may later move bytes/prefixes. AETHER retains:

- source/partition cut identity;
- leader epoch;
- promotion authority;
- stale-epoch rejection;
- divergent-prefix fencing;
- semantic acceptance of replica state.

Byte equality and endpoint health do not confer authority.

## Sidecar law

FABRIC/storage may later coordinate physical locality for artifacts/vectors.
AETHER retains semantic identity, provenance, visibility/policy and result
admission.

Cache hit, object availability and vector rank are observations only.

## Integrity law

E1 defines the integrity binding surface but not the final cryptographic
profile. A future profile must bind the exact envelope version, IDs, upstream
authorization references, scope/payload, mechanical constraints, policy refs,
expiry/supersession and required capabilities.

Integrity failure rejects before start. It cannot trigger a weaker fallback.

## E0 ambiguity treatment

E1 proposes boundaries for:

- sidecar physical locality;
- replica movement;
- namespace-to-worker handoff;
- pre-start queue/resource realization;
- telemetry evidence return.

E1 intentionally leaves these unresolved pending E2 evidence:

- durable storage adapter extraction;
- generic transport adapter extraction from `aether_http`;
- per-limit classification where semantic safety and operational capacity are
  still mixed.

The temporary `aether_api` facade is explicitly not the AETHER/FABRIC boundary.

## Consequences

Positive:

- future FABRIC can optimize mechanics without acquiring semantic power;
- AETHER local correctness remains possible;
- event and identity collapse become mechanically testable;
- influence-without-authority is attributable via policy references;
- retry/version/downgrade behavior is fail-closed;
- existing resource-control and partition-fencing semantics are preserved.

Costs:

- more typed records and explicit handoffs;
- mechanical systems cannot infer authority from convenient local state;
- E2 must build a nontrivial conformance harness before extraction;
- final integrity/security profile remains separate work;
- some current code remains mixed until tests justify decomposition.

## Rejected alternatives

### A. Use one generic `status` object

Rejected because it collapses delivered/completed/admitted/approved and makes
authority laundering difficult to detect.

### B. Treat endpoint capability as permission

Rejected because resource availability is not institutional authorization.

### C. Let FABRIC own lease/epoch failover generically

Rejected because current AETHER lease epochs and partition epochs are semantic
authority/fencing concepts, not merely liveness locks.

### D. Exactly-once transport as the semantic solution

Rejected as an E1 assumption. Mechanical delivery may be at-least-once or
otherwise defined later; AETHER must preserve idempotent semantic admission and
explicit identity rather than delegating truth to transport guarantees.

### E. Move `aether_http` or `aether_partition` wholesale

Rejected by E0: both contain mixed semantic and mechanical responsibilities.

## Non-authority

This ADR does not:

- create or activate FABRIC;
- move implementation;
- modify Article IX/AETHER authority;
- activate POL/AETHER-Learn;
- change GHOS controller admission;
- authorize deployment/claim promotion.

## Verification

E1 verification is specification-level:

- Formalist checks closed contracts and transition completeness;
- Adversary attacks widening, downgrade, identity collapse, starvation, hidden
  commit, replica authority laundering and telemetry policy bypass;
- Referee decides whether E1 is complete enough to authorize E2 test
  implementation.

E2 must implement the vectors in
`AETHER_FABRIC_E1_CONFORMANCE_VECTORS.md`, including D1-D5, before any
corresponding responsibility can be extracted.
