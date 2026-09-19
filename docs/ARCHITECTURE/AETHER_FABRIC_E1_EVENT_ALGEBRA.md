# AETHER/FABRIC E1 Event Algebra

Status: E1 candidate, Revision 1 after Formalist review
Issue: #85
Protocol: `aether-fabric/1.0`

## 1. Purpose

Define legal event kinds, ownership, attempt identity, ordering constraints,
mutually exclusive states, revocation semantics and forbidden implications
across POL/AETHER/FABRIC/GHOS.

## 2. Event domains

Events belong to exactly one primary domain:

- `INSTITUTIONAL` — POL/governed allocation/decision meaning;
- `SEMANTIC` — AETHER admission/evaluation/authority meaning;
- `MECHANICAL` — FABRIC physical realization/observation;
- `EXECUTION` — GHOS protected external execution meaning;
- `CONTROL_BRIDGE` — upstream-governed control records that constrain FABRIC
  but are not issued by FABRIC.

Cross-domain references do not transfer authority.

## 3. Identity vocabulary

Every cross-plane operation may contain:

```text
correlation_id          # groups the higher-level operation; not authority
mechanical_attempt_id   # one FABRIC realization attempt
semantic_attempt_id     # one AETHER semantic attempt
event_id                # one event record
envelope_id             # one closed mechanical authorization envelope
```

Rules:

1. A retry creates a new `mechanical_attempt_id`.
2. A retry retains the higher-level `correlation_id`.
3. All mechanical route/queue/dispatch/delivery records for one attempt carry
   the same `mechanical_attempt_id`.
4. If AETHER semantic execution is reached through FABRIC, the
   `SemanticExecutionStarted` event binds both `mechanical_envelope_id` and
   `mechanical_attempt_id`.
5. A local/no-FABRIC AETHER execution carries neither mechanical binding field.
6. It is invalid for `SemanticExecutionStarted` to carry only one of those two
   mechanical fields.

## 4. Event vocabulary

| Event | Domain | Owner/producer | Meaning |
| --- | --- | --- | --- |
| `AllocationDesired` | INSTITUTIONAL | POL/governed allocator | Desired institutional allocation. |
| `MechanicalEnvelopeAuthorized` | CONTROL_BRIDGE | upstream governed issuer | Closed mechanical realization envelope. |
| `MechanicalEnvelopeRevoked` | CONTROL_BRIDGE | upstream governed issuer | Stops future/pre-start use before expiry. |
| `RouteRealized` | MECHANICAL | FABRIC | Concrete endpoint/route selected within envelope. |
| `QueueAdmitted` | MECHANICAL | FABRIC | Pre-start capacity accepted attempt. |
| `PreStartRejected` | MECHANICAL | FABRIC | Exact mechanical attempt did not cross semantic start. |
| `PayloadDispatched` | MECHANICAL | FABRIC | Payload movement began. |
| `PayloadDelivered` | MECHANICAL | FABRIC | Payload reached target. |
| `DeliveryReceiptObserved` | MECHANICAL | FABRIC | Delivery observation; not semantic receipt. |
| `ReplicaMovementObserved` | MECHANICAL | FABRIC | Physical replica/prefix movement. |
| `PhysicalObjectLocationObserved` | MECHANICAL | FABRIC/storage | Physical object/vector locality observation. |
| `TelemetryEvidenceObserved` | MECHANICAL | FABRIC | Operational observation; not yet AETHER evidence. |
| `SemanticExecutionStarted` | SEMANTIC | AETHER | AETHER crossed semantic start boundary. |
| `SemanticExecutionCompleted` | SEMANTIC | AETHER | AETHER completed semantic lifecycle for attempt. |
| `SemanticSubmissionProposed` | SEMANTIC | submitter/AETHER edge | Result/evidence proposed for admission. |
| `SemanticAdmissionAccepted` | SEMANTIC | AETHER | Submission became admitted state. |
| `SemanticAdmissionRejected` | SEMANTIC | AETHER | Submission was semantically rejected. |
| `ExternalExecutionRequested` | EXECUTION | GHOS client/route | Protected execution requested. |
| `ExternalExecutionStarted` | EXECUTION | GHOS | Admitted controller started work. |
| `ExternalExecutionCompleted` | EXECUTION | GHOS | Protected execution completed. |
| `ExternalExecutionFailed` | EXECUTION | GHOS | Protected execution failed. |
| `InstitutionalDecisionRecorded` | INSTITUTIONAL | POL/INTELLECT application | Governed decision recorded. |

## 5. Minimal partial order

A mechanically realized direct AETHER operation:

```text
AllocationDesired?
  -> MechanicalEnvelopeAuthorized(E)
  -> RouteRealized(E, mechanical_attempt=M)
  -> QueueAdmitted(M) | PreStartRejected(M)
  -> SemanticExecutionStarted(S, envelope=E, mechanical_attempt=M)
  -> SemanticExecutionCompleted(S)
  -> SemanticSubmissionProposed?      [when separate admission is required]
  -> SemanticAdmissionAccepted | SemanticAdmissionRejected
  -> InstitutionalDecisionRecorded?
```

The `PreStartRejected(M)` branch is terminal with respect to semantic start for
that exact `mechanical_attempt_id`.

A local/no-FABRIC AETHER path is:

```text
SemanticExecutionStarted(S, no mechanical binding)
  -> SemanticExecutionCompleted(S)
```

Transport-only path:

```text
MechanicalEnvelopeAuthorized(E)
 -> RouteRealized(E,M)
 -> QueueAdmitted(M)
 -> PayloadDispatched(M)
 -> PayloadDelivered(M)
 -> DeliveryReceiptObserved(M)
```

Delivery does not imply a semantic path exists.

GHOS-protected path:

```text
MechanicalEnvelopeAuthorized? / transport?
 -> ExternalExecutionRequested
 -> ExternalExecutionStarted        [GHOS admitted controller only]
 -> ExternalExecutionCompleted | ExternalExecutionFailed
 -> SemanticSubmissionProposed
 -> SemanticAdmissionAccepted | SemanticAdmissionRejected
```

## 6. Mechanical attempt state machine

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

`MX` is terminal for semantic start for that `mechanical_attempt_id`.

A pure local worker realization may omit dispatch/delivery while still using an
explicit route/queue attempt identity if FABRIC participates.

## 7. AETHER semantic attempt state machine

```text
S0 = not_started
S1 = started
S2 = completed
S3 = submission_proposed
S4 = admitted
SX = rejected
```

Typical transitions:

```text
S0 -> S1
S1 -> S2
S2 -> S3?          # when separate semantic admission is required
S3 -> S4 | SX
```

AETHER-native accepted append may itself be the semantic mutation/admission;
E1 does not add a redundant second admission layer to existing journal law.

## 8. Envelope control state machine

Envelope states:

```text
E0 = authorized
E1 = expired
E2 = superseded
E3 = revoked
```

Only an upstream governed control record may produce `revoked` or a new
replacement envelope. FABRIC may observe the state and reject new attempts; it
cannot self-issue authority-bearing control transitions.

### Revocation

```text
MechanicalEnvelopeAuthorized(E)
 -> MechanicalEnvelopeRevoked(E)
```

After revocation:

- no new `RouteRealized`/`QueueAdmitted` attempt may begin under E;
- queued-but-not-semantic-started attempts may end with
  `PreStartRejected(reason=envelope_revoked)`;
- an attempt already bound to `SemanticExecutionStarted` continues under
  AETHER semantic lifecycle;
- revocation does not create a replacement envelope.

### Supersession

A new upstream envelope may set `supersedes_envelope_id=E`.

Supersession stops new/pre-start attempts under E according to policy but does
not erase started semantic work.

### Derivation

A child envelope may set `derived_from_envelope_id=E`. Derivation is not a
state transition of E and does not invalidate E. The child must be a verified
narrowing of E.

## 9. Critical invariants

### A1 — exact-attempt mutual exclusion

For the same `mechanical_attempt_id=M`:

```text
PreStartRejected(M) xor SemanticExecutionStarted(... mechanical_attempt=M)
```

Both cannot exist in a valid trace.

If both are observed, quarantine the trace; do not discard one event to make it
appear consistent.

### A2 — retry identity

A retry uses `M2 != M1`. Reusing M1 for a later attempt is invalid.

### A3 — mechanical failure after semantic start is not semantic cancellation

After `SemanticExecutionStarted(S,...M)`, worker disconnect, caller timeout,
transport failure or revocation may be observed. AETHER remains responsible for
completing/failing S under its semantic contract.

### A4 — delivery is orthogonal to semantic acceptance

`PayloadDelivered` may coexist with accepted, rejected, unadmitted or absent
semantic submission.

### A5 — semantic completion is orthogonal to institutional judgment

A completed AETHER evaluation does not itself select an institutional decision.

### A6 — GHOS success is orthogonal to AETHER admission

Protected external work may complete and still be rejected/unadmitted by
AETHER.

### A7 — telemetry is observation-only

`TelemetryEvidenceObserved` cannot directly transition institutional authority,
AETHER policy, actor eligibility or semantic admission.

### A8 — mechanical envelope control is upstream-owned

FABRIC cannot create `MechanicalEnvelopeAuthorized` or
`MechanicalEnvelopeRevoked` merely from local scheduler/liveness state.

## 10. Duplicate/idempotency law

Mechanical delivery is not assumed exactly-once.

Each event has a stable `event_id`; re-observing the same ID is idempotent at
the event-record level.

Distinct event IDs may describe duplicate physical delivery. AETHER semantic
admission must remain idempotent/deduplicated under AETHER identity rules.
FABRIC does not fabricate semantic deduplication identity.

## 11. Retry law

A retry record set must identify:

- same or narrower valid envelope;
- stable `correlation_id`;
- new `mechanical_attempt_id`;
- predecessor failed/rejected attempt;
- remaining retry budget.

Retry cannot:

- widen eligibility or scope;
- reset expiry/revocation/supersession;
- resurrect an invalid envelope;
- duplicate a semantic attempt already started unless AETHER explicitly
  authorizes an idempotent semantic retry.

## 12. Scope and lineage law

A child envelope identifies the exact parent through
`derived_from_envelope_id`. Its subset relation is a conformance property, not
something schema validation alone can prove.

`scope_ref` identifies an immutable byte object and `scope_digest` is SHA-256
of that exact referenced byte sequence. This avoids implementation-dependent
conceptual canonicalization.

## 13. Telemetry return algebra

```text
TelemetryEvidenceObserved          [MECHANICAL]
 -> SemanticSubmissionProposed     [SEMANTIC edge]
 -> SemanticAdmissionAccepted | Rejected
 -> AllocationDesired?             [later governed decision]
```

Direct unadmitted telemetry-to-allocation policy mutation is outside E1.

## 14. Replica movement algebra

```text
MechanicalEnvelopeAuthorized
 -> ReplicaMovementObserved(M)
 -> AETHER validates cut/prefix/epoch/fencing
 -> AETHER accepts/rejects semantic replica state
```

No mechanical event transitions to `LeaderEpochChanged` or semantic promotion.

## 15. Invalid trace classes

A conformance harness must reject or quarantine:

- pre-start rejection and semantic start for same mechanical attempt;
- retry that reuses mechanical attempt ID;
- semantic start with only one of envelope/attempt binding fields;
- new mechanical start after expiry/revocation/supersession;
- purported child envelope without verified narrowing from named parent;
- FABRIC-issued revocation/authorization;
- mechanical event followed by authority transition without owner-domain event;
- implicit protocol downgrade;
- hidden commit after pre-start rejection;
- delivered GHOS payload treated as controller admission;
- unadmitted telemetry directly changing governed allocation.

## 16. E2 use

E2 must implement executable traces keyed by exact `mechanical_attempt_id` and
`semantic_attempt_id`. E1 completion requires every event/control record to have
an owner, required identity, legal predecessor/successor set and explicit
forbidden implications.
