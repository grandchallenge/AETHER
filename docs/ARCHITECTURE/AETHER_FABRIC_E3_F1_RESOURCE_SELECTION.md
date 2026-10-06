# AETHER/FABRIC E3/F1 — Local Resource-Selection Extraction

Status: protected specification milestone; implementation evidence pending
Issue: #94
Protected E2 basis: `fce5095cfd0db8766ae0c7d5e80c091b41284862`
Protected specification basis: `1ccce8756f7e79fe16753f2c8b3da182addc94c5`
E1 protocol basis: `aether-fabric/1.0`
E3/F1 extension candidate: `aether-fabric/1.1`

## 1. Objective

E3/F1 is the first bounded extraction of a real mechanical responsibility from AETHER into FABRIC.

The responsibility in scope is intentionally narrow:

> Given an already-authorized E1 mechanical envelope, an upstream-governed placement-constraint record, an exact resource snapshot, and an exact selector implementation/policy identity, compute zero or one preferred eligible placement and return replayable non-operative evidence of that computation.

FABRIC does not acquire authority to queue, lease, dispatch, start, retry, admit, accept, promote, or semantically interpret the work.

The principal safety properties are:

```text
Selected(r, e, s, c) => r in Eligible_AETHER(e, s, c)
FABRICDecision !=> SemanticAuthorityChange
```

For the admitted live extraction domain, the FABRIC permitted-placement set MUST equal the AETHER reference permitted-placement set. FABRIC MAY add mechanical error states, but MUST NOT enlarge the authorized placement set.

## 2. Live extraction baseline

The protected AETHER implementation does not currently expose an identityful multi-resource selector. The live resource-control path is the bounded local blocking executor in `crates/aether_http/src/http.rs`, governed by `docs/RESOURCE_CONTROL_CONTRACT.md`:

- a configured local blocking-worker pool;
- bounded global worker permits;
- bounded queue admission;
- per-namespace semantic admission/ordering retained by AETHER;
- anonymous worker permits rather than stable per-worker identities.

Therefore the first live E3A equivalence domain SHALL model the current local blocking executor as **one mechanical resource pool**, not fabricate identities for individual semaphore permits.

F1A SHALL define one stable resource-pool identity for this existing executor surface. It SHALL NOT assign synthetic stable identities to anonymous worker permits.

Multi-resource snapshots MAY be used in pure contract/hostile tests, but E3A MUST NOT claim live multi-resource equivalence until AETHER has an independently governed reference surface containing multiple resource pools or targets.

This baseline distinction is mandatory:

```text
contract test domain    may contain multiple synthetic resources
live extraction domain  initially contains one current local resource pool
```

## 3. Protocol/version discipline

E1 is a closed `aether-fabric/1.0` contract. Its JSON Schemas use closed-world records and `additionalProperties: false`.

E3/F1 MUST NOT add fields such as resource requirements, snapshot freshness, placement evidence, or selector identity to an E1 `1.0` envelope and then still call that record E1-compatible.

F1A SHALL define a negotiated E3/F1 extension candidate, provisionally `aether-fabric/1.1`, for any new placement-context, resource-snapshot, or placement-evidence records.

Required rules:

- the exact E1 `MechanicalEnvelopeAuthorized` bytes remain valid `1.0` bytes and are not rewritten;
- an E3 placement-constraint record references the exact E1 envelope by `envelope_id` and digest;
- the E3 constraint record may narrow E1 placement eligibility but may never widen it;
- a `1.0`-only participant presented with a required F1 capability/record fails closed;
- no implicit downgrade from `1.1` placement semantics to `1.0` is permitted;
- E3/F1 extension schemas remain candidate/non-runtime until exact-head evidence closes them.

## 4. Authority boundary

AETHER retains exclusive authority over request admission, semantic identity, principal and actor authority, namespace resolution, semantic attempt creation, policy interpretation, authorization, envelope issuance and revocation, semantic start/completion, result admission, provenance, replay, authoritative epochs/fencing, same-namespace semantic ordering, and institutional state.

FABRIC may only compute a preferred resource from a set already bounded by upstream authority.

A placement decision is observation/evidence. It is not permission to execute.

F1 explicitly excludes queue admission, dispatch, resource leasing, process start, retry realization, transport, autoscaling, replica movement, telemetry-driven policy mutation, learned scheduling, distributed consensus, and multi-region failover.

## 5. Upstream placement constraints

Capacity/freshness eligibility cannot depend on requirements absent from the closed E1 envelope.

F1A SHALL therefore define a separate upstream-governed `PlacementConstraintSet` (name may be refined by ADR/schema, but semantics may not be weakened) that references the exact E1 envelope and carries only additional mechanical narrowing constraints.

Minimum conceptual shape:

```text
PlacementConstraintSet
├── schema_version
├── protocol_family
├── protocol_major/minor
├── constraint_set_id
├── envelope_id
├── envelope_digest
├── correlation_id
├── resource_requirements
├── max_snapshot_age_ms
├── decision_time_unix_ms
├── control_state_ref
├── control_state_digest
├── source_authority_ref
└── authority_effect = mechanical_narrowing_only
```

The constraint set:

- is upstream-produced and integrity-bound;
- cannot create permission absent from the E1 envelope;
- cannot widen eligible resource classes, trust zones, capabilities, locality, deadline, retry, redundancy, priority, or security constraints;
- supplies the exact decision time used for deterministic freshness/deadline evaluation;
- supplies an exact control-state witness for revocation/supersession/expiry interpretation.

FABRIC MUST NOT choose a more favorable decision time or freshness window.

## 6. Exact decision input

The replay identity SHALL bind all exact inputs that can affect selection:

```text
DecisionInput
├── exact_e1_envelope_bytes
├── exact_placement_constraint_bytes
├── canonical_resource_snapshot_bytes
├── scheduler_policy_artifact_bytes
├── selector_implementation_identity
└── control_state_witness_bytes_or_digest
```

`decision_time_unix_ms` is carried inside the exact placement-constraint bytes and is therefore replay-bound.

The implementation SHALL record SHA-256 digests for:

- exact E1 envelope bytes;
- exact placement-constraint bytes;
- canonical resource-snapshot bytes;
- exact scheduler-policy artifact bytes;
- exact selector implementation artifact/build identity;
- control-state witness when represented by bytes;
- the complete canonical DecisionInput.

A scheduler policy version string or repository branch name alone is insufficient.

The exact selector implementation MUST be bound by an immutable build/artifact digest or an equivalently exact commit/tree plus reproducible artifact identity accepted by the tranche's evidence contract.

## 7. Resource snapshot contract

A resource snapshot SHALL include at minimum:

```text
ResourceSnapshot
├── snapshot_id
├── schema_version
├── snapshot_digest
├── observed_at_unix_ms
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

### Canonicalization profile

F1A SHALL define one deterministic JSON canonicalization profile.

At minimum it MUST:

- encode UTF-8 JSON;
- reject duplicate object keys;
- normalize object member ordering deterministically;
- sort `resources[]` by stable `resource_id`;
- sort set-valued arrays such as capabilities where order has no semantic meaning;
- prohibit non-finite numeric values;
- specify integer units for all capacity and timestamp values used in selection;
- produce byte-identical output for semantically identical snapshots.

A versioned RFC 8785/JCS-based profile with the required pre-sorting of semantically unordered arrays is acceptable.

The exact profile identifier is part of `canonicalization_version`.

Snapshot freshness is upstream-governed. FABRIC enforces `max_snapshot_age_ms` from the exact placement constraint; it may not invent or relax that value.

## 8. Eligibility

A resource is eligible only when every hard constraint holds.

At minimum:

- resource class is in the E1 envelope's eligible resource classes;
- required E1 capabilities are a subset of resource capabilities;
- trust zone is permitted by E1;
- locality is no weaker than the E1 envelope permits;
- health state is eligible under the exact scheduler policy;
- available capacity satisfies the explicit E3 placement requirements;
- snapshot freshness satisfies upstream `max_snapshot_age_ms`;
- `decision_time_unix_ms` is before the E1 envelope expiry/deadline;
- the exact control-state witness shows no revocation/supersession state that forbids new work;
- every E3 constraint is equal to or narrower than E1.

Optimization happens only after eligibility is established.

Policy determines the admissible set. FABRIC optimization selects within that set.

## 9. Deterministic selector

F1 SHALL use a pure deterministic selector.

The initial policy SHOULD remain intentionally simple and explainable. For the first live extraction domain, selection is effectively between:

- the one current local blocking resource pool, when eligible; or
- no placement, when ineligible/unavailable.

Synthetic multi-resource tests MAY exercise lexicographic ranking across locality, queue depth, capacity and stable resource identity, but those tests do not prove live multi-resource extraction.

Given identical exact DecisionInput bytes and selector implementation identity, the selector SHALL return the same decision evidence.

## 10. Placement evidence

Successful shadow output is `PlacementSelected`, not `ExecutionAuthorized`.

Placement occurs **before** E1 `RouteRealized(E,M)` creates a mechanical realization attempt. Therefore a shadow placement decision MUST NOT mint or require a `mechanical_attempt_id`.

Minimum logical shape:

```text
PlacementSelected
├── placement_decision_id
├── event_id
├── correlation_id
├── envelope_id
├── envelope_digest
├── placement_constraint_id
├── placement_constraint_digest
├── resource_snapshot_id
├── resource_snapshot_digest
├── scheduler_policy_id
├── scheduler_policy_digest
├── selector_implementation_digest
├── decision_time_unix_ms
├── decision_input_digest
├── control_state_ref
├── control_state_digest
├── resource_id
├── eligibility_evidence
├── selection_rank
└── authority_effect = none
```

No eligible target produces `PlacementUnavailable` with the same exact input bindings and `authority_effect = none`.

Neither output denotes route realization, queue admission, dispatch, semantic start, semantic completion, result acceptance, or authority change.

If a future E3B cutover is authorized, the exact relationship between `PlacementSelected` and the later E1/E3 `RouteRealized` mechanical attempt MUST be separately specified and versioned. E3A does not create that link.

## 11. Idempotency

Repeated delivery of an identical DecisionInput MUST NOT manufacture distinct logical placement history.

The logical `placement_decision_id` SHALL be a deterministic function of the complete `decision_input_digest` and the exact selector implementation identity.

Protocol retries may re-emit the same logical decision evidence.

Distinct decision inputs, including a changed snapshot, decision time, control witness, policy artifact, or selector implementation, MUST yield distinguishable decision identity.

## 12. Revocation race and linearization

The placement decision binds the exact control-state witness against which it was computed.

If revocation races with decision computation, a placement record may remain as historical evidence, but it cannot authorize later realization.

Any future downstream mechanical realization MUST independently re-check current governed envelope/control state at its own realization boundary. A stale placement decision cannot override a newer revocation, supersession, expiry, or other upstream prohibition.

F1 itself performs no realization, so a late shadow decision is harmless evidence rather than an execution side effect.

## 13. Reference behavior and shadow mode

Shadow execution is mandatory before any routing cutover.

For the first live extraction domain, the current AETHER resource-control path remains authoritative. F1A SHALL expose a read-only reference predicate/adapter over the existing local blocking-pool admission surface without changing its behavior.

The adapter SHALL answer only the bounded question needed for differential evidence:

```text
Would the current AETHER resource-control configuration admit the modeled
local blocking resource pool under this exact envelope/constraint/snapshot input?
```

It MUST NOT fabricate per-worker identities or change semaphore ordering.

Conceptually:

```text
current AETHER resource-control path ---> authoritative behavior
                 |
                 +--- exact modeled input ---> reference admissibility evidence
                 |
                 +--- same exact input ------> FABRIC shadow selector
                                                |
                                                +--> non-operative placement evidence
```

The reference AETHER path remains authoritative throughout E3A.

Differential evidence SHALL establish:

- FABRIC never selects outside the AETHER-permitted live set;
- for the admitted live extraction domain, FABRIC and the reference adapter compute the same permitted-placement set;
- contract-level synthetic multi-resource tests are clearly separated from live-equivalence evidence;
- mechanical failures introduced by FABRIC fail closed;
- no shadow output changes queue state, semantic lifecycle, or authority.

## 14. Rollback

F1 SHALL be removable by disabling the shadow path without semantic-state migration.

FABRIC owns no semantic state. Disabling shadow evaluation or any later separately approved selector route MUST NOT require rewriting provenance, semantic attempts, namespace state, control state, or institutional authority records.

## 15. F1A required deliverables

F1A is complete only when it provides, at minimum:

1. candidate E3/F1 extension schemas under a versioned `schemas/aether_fabric/e3/` surface for:
   - placement constraints;
   - resource snapshots;
   - placement decisions/unavailability;
2. exact canonicalization and digest rules;
3. the stable current-resource-pool mapping for the live AETHER blocking executor;
4. a read-only AETHER reference-admissibility adapter with no scheduling side effect;
5. explicit narrowing validation from E3 placement constraints to the exact E1 envelope;
6. exact selector implementation/build identity rules;
7. decision-time, snapshot-freshness and control-state witness rules;
8. idempotent placement-decision identity rules;
9. hostile tests for malformed/widening constraints and protocol downgrade;
10. an ADR recording why the E1 `1.0` envelope remains immutable and why E3/F1 uses a negotiated extension rather than silent field accretion.

## 16. Work structure

- **F1A — typed extension contract, baseline adapter, canonical decision inputs**
- **F1B — pure deterministic selector**
- **F1C — hostile/replay tests**
- **F1D — non-operative shadow integration beside the AETHER reference path**
- **F1E — differential-equivalence evidence**
- **E3A — exact-head adjudication and protected merge**
- **E3B — separately governed routing/cutover decision**

E3A does not authorize E3B.

## 17. Required hostile evidence

At minimum:

- current single local resource pool eligible;
- current pool unavailable/ineligible;
- one synthetic eligible resource;
- multiple synthetic eligible resources;
- no eligible resources;
- wrong resource class;
- missing capability;
- wrong trust zone;
- weakened locality constraint;
- unhealthy resource;
- insufficient capacity;
- missing or malformed placement requirements;
- E3 placement constraint that widens E1;
- stale snapshot;
- non-canonical snapshot ordering;
- duplicate snapshot resource IDs;
- expired temporal boundary;
- revoked envelope;
- superseded envelope;
- revocation race;
- incompatible protocol major;
- `1.0` participant presented with required `1.1` placement capability;
- implicit protocol downgrade;
- scheduler-policy digest mismatch;
- selector-implementation digest mismatch;
- corrupt E1 scope digest;
- deterministic tie-break in synthetic domain;
- identical replay;
- duplicate protocol delivery/idempotency;
- shadow path mints no `mechanical_attempt_id`;
- no semantic-start side effect;
- no queue/permit side effect from shadow/reference adapters;
- no authority mutation;
- no selection outside the E1 envelope;
- envelope narrowing preserved;
- telemetry cannot broaden eligibility;
- capability laundering;
- trust-zone laundering;
- priority escalation;
- resource-ID/semantic-ID collapse;
- malformed snapshot cannot trigger permissive fallback.

Applicable E2 vectors SHALL also replay against the extracted path where their vocabulary remains applicable.

## 18. Exact-head governance

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

## 19. E3A acceptance

E3A completes only when:

- the exact implementation responsibility is limited to non-operative placement selection;
- E1 `1.0` remains byte/schema-stable;
- negotiated E3/F1 extension records are explicit and fail closed;
- the live baseline is the actual current AETHER local blocking-pool surface, not synthetic per-worker identities;
- exact decision inputs and evidence digests include the selector implementation identity;
- snapshot canonicalization/freshness rules are explicit;
- control-state witnessing, revocation linearization and idempotency are tested;
- the shadow path mints no mechanical attempt and has no queue/permit/dispatch side effect;
- differential evidence proves no authorization-set widening on the live extraction domain;
- synthetic multi-resource contract evidence is not misreported as live equivalence;
- applicable E2 invariants remain green;
- hostile F1 vectors pass;
- rollback requires no semantic-state migration;
- Formalist, Adversary, and Referee exact-head passes exist;
- protected checks are green;
- no unresolved review threads remain;
- protected main is read back after merge; and
- a completion receipt records exact source head and protected merge.

## 20. Non-authority

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
