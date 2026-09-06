# AETHER/FABRIC E1 Event Algebra

Status: E1 candidate
Issue: #85
Protocol: `aether-fabric/1.0`

## 1. Purpose

Define legal event kinds, ownership, ordering constraints, mutually exclusive
states, and forbidden implications across POL/AETHER/FABRIC/GHOS.

The event algebra prevents a mechanical implementation from acquiring authority
by collapsing several qualitatively different transitions into one status flag.

## 2. Event domains

Events belong to exactly one primary domain:

- `INSTITUTIONAL`: POL/governed allocation/decision meaning;
- `SEMANTIC`: AETHER admission/evaluation/authority meaning;
- `MECHANICAL`: FABRIC physical realization/observation;
- `EXECUTION`: GHOS protected external execution meaning.

An event may reference events from another domain but does not inherit their
authority.

## 3. Event vocabulary

| Event | Domain | Producer | Meaning |
| --- | --- | --- | --- |
| `AllocationDesired` | INSTITUTIONAL | POL/governed allocator | Declares desired actor/guild/resource class for work. |
| `MechanicalEnvelopeAuthorized` | SEMANTIC/INSTITUTIONAL bridge | authorized upstream producer, recorded/referenced by AETHER | Closes the permitted mechanical realization space. |
| `RouteRealized` | MECHANICAL | FABRIC | Concrete route/endpoint chosen within envelope. |
| `QueueAdmitted` | MECHANICAL | FABRIC | Pre-start mechanical capacity accepted operation. |
| `PreStartRejected` | MECHANICAL | FABRIC | Operation did not cross AETHER semantic-start boundary. |
| `PayloadDispatched` | MECHANICAL | FABRIC | Payload movement began. |
| `PayloadDelivered` | MECHANICAL | FABRIC | Payload reached target endpoint. |
| `DeliveryReceiptObserved` | MECHANICAL | FABRIC | Mechanical delivery evidence observed. |
| `ExternalExecutionRequested` | EXECUTION | GHOS client/route | Protected execution requested under GHOS contract. |
| `ExternalExecutionStarted` | EXECUTION | GHOS | Admitted controller started protected work. |
| `ExternalExecutionCompleted` | EXECUTION | GHOS | Protected execution completed. |
| `ExternalExecutionFailed` | EXECUTION | GHOS | Protected execution failed under GHOS semantics. |
| `SemanticExecutionStarted` | SEMANTIC | AETHER | AETHER crossed semantic start boundary. |
| `SemanticExecutionCompleted` | SEMANTIC | AETHER | AETHER evaluation/mutation completed under semantic lifecycle. |
| `SemanticSubmissionProposed` | SEMANTIC | submitter/AETHER edge | Result/evidence proposed for semantic admission. |
| `SemanticAdmissionAccepted` | SEMANTIC | AETHER | Submission became admitted AETHER state. |
| `SemanticAdmissionRejected` | SEMANTIC | AETHER | Submission was semantically rejected. |
| `InstitutionalDecisionRecorded` | INSTITUTIONAL | POL/INTELLECT application | Governed decision recorded using admitted evidence. |
| `TelemetryEvidenceObserved` | MECHANICAL | FABRIC | Operational observation emitted; not yet AETHER evidence. |
| `ReplicaMovementObserved` | MECHANICAL | FABRIC | Physical replica/prefix movement observed. |
| `PhysicalObjectLocationObserved` | MECHANICAL | FABRIC/storage | Object/vector locality observation. |

## 4. Minimal partial order

For a mechanically realized direct AETHER operation, the typical partial order
is:

```text
AllocationDesired?
      |
MechanicalEnvelopeAuthorized
      |
RouteRealized
      |
QueueAdmitted  OR  PreStartRejected
      |
      +-- PreStartRejected -> terminal mechanical non-start
      |
SemanticExecutionStarted
      |
SemanticExecutionCompleted
      |
SemanticSubmissionProposed?   [if a result re-enters via admission]
      |
SemanticAdmissionAccepted OR SemanticAdmissionRejected
      |
InstitutionalDecisionRecorded?
```

`AllocationDesired` is optional for purely mechanical system operations that
already have a separately authorized semantic source. Its absence does not give
FABRIC discretion to invent institutional purpose.

For remote payload transport:

```text
MechanicalEnvelopeAuthorized
 -> RouteRealized
 -> QueueAdmitted
 -> PayloadDispatched
 -> PayloadDelivered
 -> DeliveryReceiptObserved
```

A later semantic path may begin, but delivery alone does not create it.

For GHOS-protected execution:

```text
MechanicalEnvelopeAuthorized?     [if FABRIC transports]
 -> PayloadDelivered?
 -> ExternalExecutionRequested
 -> ExternalExecutionStarted
 -> ExternalExecutionCompleted | ExternalExecutionFailed
 -> SemanticSubmissionProposed
 -> SemanticAdmissionAccepted | SemanticAdmissionRejected
```

FABRIC cannot substitute its own endpoint for `ExternalExecutionStarted`.

## 5. State machines

### 5.1 Mechanical attempt state

```text
M0 = unknown
M1 = envelope_authorized
M2 = route_realized
M3 = queue_admitted
M4 = dispatched
M5 = delivered
MX = pre_start_rejected
MF = transport_failed
```

Allowed transitions:

```text
M0 -> M1
M1 -> M2 | MX
M2 -> M3 | MX
M3 -> M4 | MF
M4 -> M5 | MF
```

A pure local worker realization may omit `M4/M5` if no transport occurs; it
must still preserve `M1/M2/M3` or an equivalent mechanically inspectable start
path.

### 5.2 AETHER semantic attempt state

```text
S0 = not_started
S1 = started
S2 = completed
S3 = submission_proposed
S4 = admitted
SX = rejected
```

Allowed transitions depend on operation type:

```text
S0 -> S1
S1 -> S2
S2 -> S3?          [only if a separate admission step is required]
S3 -> S4 | SX
```

Some AETHER-native mutation operations may make their accepted append itself
the semantic completion/admission; E1 does not replace existing AETHER journal
semantics with a second admission layer.

### 5.3 Protected execution state

```text
G0 = not_requested
G1 = requested
G2 = started_by_admitted_controller
G3 = completed
GX = failed
```

Only GHOS may produce the authoritative G2 transition for protected execution.

## 6. Cross-machine invariants

### E1 — mutual exclusion of pre-start rejection and semantic start

For a single semantic attempt ID:

```text
PreStartRejected xor SemanticExecutionStarted
```

Both may not exist for the same attempt.

If evidence of both is observed, the trace is invalid and must be quarantined;
neither event is silently discarded to make the trace consistent.

### E2 — mechanical failure after semantic start is not semantic cancellation

After `SemanticExecutionStarted`, a transport/worker observation may report
failure, disconnect or timeout. The AETHER semantic attempt remains `started`
until AETHER records its own completion/failure according to existing semantic
law.

### E3 — delivery is orthogonal to semantic acceptance

`PayloadDelivered` can coexist with either:

```text
SemanticAdmissionAccepted
SemanticAdmissionRejected
no semantic submission at all
```

### E4 — semantic completion is orthogonal to institutional judgment

A correctly completed AETHER evaluation does not imply an institutional
decision to accept a claim or allocate further work.

### E5 — GHOS success is orthogonal to AETHER admission

Protected external work may complete and still yield a result that AETHER
rejects or never admits.

### E6 — telemetry is observation-only

`TelemetryEvidenceObserved` cannot directly transition any institutional or
AETHER authority state.

## 7. Idempotency and duplicate events

Mechanical delivery is not assumed exactly-once.

Each record has a stable `event_id`. Re-observing the same `event_id` is
idempotent at the event-log level.

If two distinct event IDs claim the same mechanical operation and payload, both
may be preserved as observations; semantic admission must remain idempotent or
explicitly deduplicated under AETHER's own identity rules.

FABRIC must never fabricate a semantic deduplication identity.

## 8. Causation and correlation

Every event contains:

```text
correlation_id
```

and may contain:

```text
causation_id
```

`correlation_id` groups an operation for reconstruction. `causation_id` names a
specific predecessor event.

Neither is an authority token.

A malicious actor must not gain authority by selecting a correlation ID that
matches an authorized operation.

## 9. Retry algebra

A retry creates a new mechanical attempt ID, not a new institutional purpose.

A retry must reference:

- the same or a narrower authorized envelope;
- a stable higher-level correlation ID;
- the predecessor failed/pre-start-rejected attempt;
- the remaining retry budget.

A retry may not:

- widen endpoint/resource eligibility;
- reset an expired semantic authority lease;
- resurrect a superseded envelope;
- cause an already-started semantic attempt to be duplicated unless AETHER's
  semantic idempotency contract explicitly permits it.

## 10. Supersession algebra

If envelope `E2` supersedes `E1`:

- new pre-start mechanical attempts under E1 are rejected;
- queued-but-not-started E1 attempts may be rejected according to policy;
- `SemanticExecutionStarted` attempts bound to E1 continue under AETHER's
  semantic lifecycle;
- E2 may narrow or otherwise replace E1 only under a fresh upstream authority
  decision; FABRIC cannot self-issue E2.

## 11. Replica movement algebra

```text
ReplicationMovementAuthorized   [AETHER or admitted upstream contract]
 -> ReplicaMovementObserved     [MECHANICAL]
 -> ReplicaStateValidated       [AETHER semantic check]
```

`ReplicaMovementObserved` has no transition to `LeaderEpochChanged` or
`ReplicaPromoted`.

## 12. Telemetry return algebra

```text
TelemetryEvidenceObserved       [MECHANICAL]
 -> SemanticSubmissionProposed  [AETHER edge]
 -> SemanticAdmissionAccepted | Rejected
 -> AllocationDesired?          [later governed decision]
```

The direct edge:

```text
TelemetryEvidenceObserved -> AllocationDesired
```

is forbidden for governed allocation unless a separately admitted allocator
contract consumes the corresponding **admitted** evidence.

## 13. Failure consistency classes

A trace parser/conformance harness must distinguish:

- **valid terminal non-start**: `PreStartRejected`, no semantic start;
- **valid started completion**: semantic start then semantic completion;
- **valid delivered/unadmitted**: payload delivered, no semantic acceptance;
- **valid external failure**: GHOS failure followed by optional semantic failure
  evidence submission;
- **invalid dual-start/reject**: pre-start rejection and semantic start for same
  attempt;
- **invalid authority leap**: mechanical event followed by authority transition
  without AETHER/GHOS/INTELLECT owner event;
- **invalid downgrade**: work begins under unsupported/implicitly downgraded
  protocol/capability set;
- **invalid hidden commit**: pre-start timeout/failure reported, later AETHER
  append/receipt appears for the same supposedly non-started attempt.

## 14. Conformance use

E2 must turn these event classes into executable traces. E1 completion requires
that every event has an owner, legal predecessor/successor set, and explicit
forbidden implications.
