# AETHER/FABRIC E3/F1 — Local Resource-Selection Extraction

Status: candidate specification
Issue: #94
Protected E2 basis: `fce5095cfd0db8766ae0c7d5e80c091b41284862`
Protocol family: `aether-fabric/1.x`

## 1. Objective

E3/F1 is the first bounded extraction of a real mechanical responsibility from AETHER into FABRIC.

The only responsibility in scope is:

> Given an already-authorized mechanical envelope and an exact resource snapshot, compute zero or one preferred eligible placement and return replayable evidence of that computation.

FABRIC does not acquire authority to queue, lease, dispatch, start, retry, admit, accept, promote, or semantically interpret the work.

The principal safety properties are:

```text
Selected(r, e, s) => r in Eligible_AETHER(e, s)
FABRICDecision !=> SemanticAuthorityChange
```

For the admitted extraction domain, the new permitted-placement set MUST equal the AETHER reference permitted-placement set. FABRIC MAY add mechanical error states, but MUST NOT enlarge the authorized placement set.

## 2. Authority boundary

AETHER retains exclusive authority over request admission, semantic identity, principal and actor authority, namespace resolution, semantic attempt creation, policy interpretation, authorization, envelope issuance and revocation, semantic start/completion, result admission, provenance, replay, authoritative epochs/fencing, and institutional state.

FABRIC may only compute a preferred resource from a set already bounded by upstream authority.

A placement decision is observation/evidence. It is not permission to execute.

F1 explicitly excludes queue admission, dispatch, resource leasing, process start, retry realization, transport, autoscaling, replica movement, telemetry-driven policy mutation, learned scheduling, distributed consensus, and multi-region failover.

## 3. Decision input

The replay identity SHALL bind four exact inputs:

```text
DecisionInput
├── mechanical_envelope_bytes
├── canonical_resource_snapshot_bytes
├── scheduler_policy_artifact_bytes
└── decision_time
```

The implementation SHALL record SHA-256 digests for the first three byte surfaces plus a digest over the complete canonical DecisionInput.

The envelope SHALL carry or immutably reference explicit `resource_requirements`; capacity eligibility MUST NOT depend on an unstated requirement.

`decision_time` is immutable evidence supplied to the decision procedure. Wall-clock time read during replay MUST NOT alter the decision.

## 4. Resource snapshot contract

A resource snapshot SHALL include at minimum:

```text
ResourceSnapshot
├── snapshot_id
├── schema_version
├── snapshot_digest
├── observed_at
├── observation_source
├── canonicalization_version
└── resources[]
    ├── resource_id
    ├── resource_class
    ├── capabilities[]
    ├── trust_zone
    ├── health
    ├── available_capacity
    ├── queue_depth
    ├── locality
    └── scheduler_metadata
```

Interpretation MUST be independent of input array order. Canonicalization and stable resource identity MUST make semantically identical snapshots byte-stable before hashing.

Snapshot freshness is upstream-governed. A maximum snapshot age or equivalent freshness rule MUST come from the envelope or versioned AETHER→FABRIC contract. FABRIC may enforce this rule; it may not invent or relax it.

## 5. Eligibility

A resource is eligible only when every hard constraint holds.

At minimum:

- resource class is in the envelope's eligible resource classes;
- required capabilities are a subset of resource capabilities;
- trust zone is permitted;
- health state is eligible;
- available capacity satisfies the explicit resource requirements;
- snapshot freshness satisfies the upstream rule;
- decision time is within the envelope's permitted temporal boundary;
- the envelope/control revision observed for the decision is not revoked.

Optimization happens only after eligibility is established.

Policy determines the admissible set. FABRIC optimization selects within that set.

## 6. Deterministic selector

F1 SHALL use a pure deterministic selector. The initial policy SHOULD remain intentionally simple and explainable, for example lexicographic ordering across admissible locality, queue depth, and stable resource identity after hard eligibility filtering.

Given identical:

- envelope bytes;
- canonical snapshot bytes;
- scheduler-policy artifact bytes; and
- decision time,

the selector SHALL return the same decision evidence.

A scheduler version label alone is insufficient. Replay binds the exact scheduler-policy artifact digest.

## 7. Placement evidence

Successful output is `PlacementSelected`, not `ExecutionAuthorized`.

Minimum logical shape:

```text
PlacementSelected
├── placement_decision_id
├── event_id
├── envelope_id
├── mechanical_attempt_id
├── envelope_digest
├── resource_snapshot_id
├── resource_snapshot_digest
├── scheduler_policy_id
├── scheduler_policy_digest
├── decision_time
├── decision_input_digest
├── control_revision
├── envelope_status_revision
├── resource_id
├── eligibility_evidence
├── selection_rank
└── authority_effect = none
```

No eligible target produces `PlacementUnavailable` with the same input bindings and `authority_effect = none`.

Neither output denotes queue admission, dispatch, semantic start, semantic completion, result acceptance, or authority change.

## 8. Idempotency

Repeated delivery of an identical DecisionInput MUST NOT manufacture distinct logical placement history.

The logical `placement_decision_id` SHALL be a deterministic function of the exact decision identity, including envelope/mechanical-attempt identity, snapshot digest, scheduler-policy digest, and decision time or the complete decision-input digest.

Protocol retries may re-emit the same logical decision evidence.

## 9. Revocation race and linearization

A placement decision binds the exact control and envelope-status revisions against which it was computed.

If revocation races with decision computation, a placement record may remain as historical evidence, but it cannot authorize later realization. Any downstream mechanical realization MUST re-check the current governed envelope state and reject a decision whose envelope became revoked before realization authority was independently established.

F1 itself performs no realization, so a late decision is harmless evidence rather than an execution side effect.

## 10. Shadow mode

Shadow execution is mandatory before any routing cutover.

```text
AETHER reference selector ---> authoritative placement
            |
            +---- same exact input ----> FABRIC shadow selector
                                         |
                                         +--> non-operative placement evidence
```

The reference AETHER selector remains authoritative throughout E3A.

Differential evidence SHALL establish:

- FABRIC never selects outside the AETHER-permitted set;
- for the admitted extraction domain, FABRIC and the reference selector compute the same permitted-placement set;
- any different preferred target is still inside that identical authorized set;
- mechanical failures introduced by FABRIC fail closed;
- no FABRIC output changes semantic lifecycle or authority.

## 11. Rollback

F1 SHALL be removable by routing back to the AETHER reference selector without semantic-state migration.

FABRIC owns no semantic state. Disabling shadow evaluation or a later approved selector route MUST NOT require rewriting provenance, semantic attempts, namespace state, or institutional authority records.

## 12. Work structure

- **F1A — contract + canonical decision inputs**
- **F1B — pure deterministic selector**
- **F1C — hostile/replay tests**
- **F1D — shadow integration with existing AETHER selector**
- **F1E — differential-equivalence evidence**
- **E3A — exact-head adjudication and protected merge**
- **E3B — separately governed routing/cutover decision**

E3A does not authorize E3B.

## 13. Required hostile evidence

At minimum:

- one eligible resource;
- multiple eligible resources;
- no eligible resources;
- wrong resource class;
- missing capability;
- wrong trust zone;
- unhealthy resource;
- insufficient capacity;
- missing or malformed resource requirements;
- stale snapshot;
- non-canonical snapshot ordering;
- expired temporal boundary;
- revoked envelope;
- revocation race;
- incompatible protocol major;
- scheduler-policy digest mismatch;
- corrupt scope digest;
- deterministic tie-break;
- identical replay;
- duplicate protocol delivery/idempotency;
- attempt-identity preservation;
- no semantic-start side effect;
- no authority mutation;
- no selection outside the envelope;
- envelope narrowing preserved;
- telemetry cannot broaden eligibility;
- capability laundering;
- trust-zone laundering;
- priority escalation;
- resource-ID/semantic-ID collapse;
- malformed snapshot cannot trigger permissive fallback.

Applicable E2 vectors SHALL also replay against the extracted path.

## 14. Exact-head governance

Candidate completion follows:

```text
exact candidate head
-> Formalist
-> Adversary
-> Referee
-> required protected checks
-> no unresolved review threads
-> protected merge
-> protected-main readback
-> completion receipt
```

Any candidate-head change invalidates earlier exact-head review/check authority and requires fresh evidence for the changed head.

A valid protected transition is not erased by later documentary/runtime failure. Recovery must bind the real protected transition and repair readback rather than fabricate a cleaner history.

## 15. E3A acceptance

E3A completes only when:

- the exact implementation responsibility is limited to placement selection;
- exact decision inputs and evidence digests are implemented;
- snapshot canonicalization/freshness rules are explicit;
- revocation linearization and idempotency are tested;
- the shadow path is non-operative;
- differential evidence proves no authorization-set widening;
- applicable E2 invariants remain green;
- hostile F1 vectors pass;
- rollback requires no semantic-state migration;
- Formalist, Adversary, and Referee exact-head passes exist;
- protected checks are green;
- no unresolved review threads remain;
- protected main is read back after merge; and
- a completion receipt records exact source head and protected merge.

## 16. Non-authority

Completion of E3A SHALL NOT authorize:

- live FABRIC routing/cutover;
- general FABRIC activation;
- queue or dispatch authority;
- semantic extraction;
- autonomous policy adaptation;
- replica authority;
- distributed FABRIC;
- generalized GCL migration.

E3B is a separate control-path decision under the live governance boundary.
