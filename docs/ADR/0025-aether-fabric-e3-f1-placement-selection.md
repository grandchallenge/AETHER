# ADR 0025 — AETHER/FABRIC E3/F1 Placement-Selection Extraction

Status: F1E protected — implementation merge `47bec0af7823c0295f55a6b5f6ae3dac832bfeae`; E3A adjudication pending
Date: 2026-10-06
Issue: #94
E1 basis: `aether-fabric/1.0`
Protected E2 basis: `fce5095cfd0db8766ae0c7d5e80c091b41284862`
Protected E3/F1 specification basis: `1ccce8756f7e79fe16753f2c8b3da182addc94c5`

## Context

E1 deliberately closed the AETHER/FABRIC interface at protocol `1.0`.
Its schemas reject unknown fields. E2 proved the reference contract is executable
but did not move implementation.

The first extraction target is resource placement. The live AETHER resource
surface is not an identityful multi-resource scheduler. It is the bounded local
blocking executor in `crates/aether_http/src/http.rs`, with global worker
permits, queue admission and AETHER-owned per-namespace semantic ordering.

A first E3/F1 draft incorrectly left four ambiguities:

1. it implied new resource-requirement/freshness fields could be attached to the
   closed E1 `1.0` envelope;
2. it required `mechanical_attempt_id` on a placement decision that occurs
   before E1 `RouteRealized` creates a mechanical attempt;
3. it assumed an identityful AETHER reference selector that does not currently
   exist; and
4. it bound scheduler policy but not the exact selector implementation that
   executes that policy.

## Decision

### 1. Preserve E1 `1.0` byte/schema identity

Do not mutate or silently extend E1 envelope records.

F1A will define candidate E3 placement-context/evidence records under a
negotiated minor extension, provisionally `aether-fabric/1.1`.

A `1.0`-only participant required to consume F1 records fails closed. No
implicit downgrade is allowed.

### 2. Add an upstream narrowing record instead of widening the envelope

Resource requirements, snapshot-freshness bounds, decision time and the exact
control-state witness belong in an upstream-governed E3
`PlacementConstraintSet` that references the exact E1 envelope and may only
narrow its mechanical permissions.

### 3. Treat placement as pre-attempt evidence

`PlacementSelected` is non-operative evidence and does not carry or mint a
`mechanical_attempt_id`.

E1 attempt identity begins when actual route realization begins. A future E3B
cutover must separately specify and version the relation between a placement
decision and `RouteRealized(E,M)`.

### 4. Use the actual current AETHER baseline

The first live equivalence domain models the current local blocking executor as
one resource pool. It must not fabricate stable identities for anonymous
semaphore permits.

Synthetic multi-resource tests are useful contract tests but are not live
equivalence evidence.

### 5. Bind implementation as well as policy

Replay identity includes:

- exact E1 envelope bytes;
- exact E3 placement-constraint bytes;
- canonical resource snapshot bytes;
- scheduler-policy artifact bytes;
- exact selector implementation/build identity;
- exact control-state witness.

The upstream placement constraint supplies the replay-bound decision time.

### 6. Keep E3A shadow-only

E3A may construct a read-only reference-admissibility adapter and a FABRIC
shadow selector. Neither may alter queue/permit state or semantic state.

Live routing remains E3B and is separately governed.

## Consequences

Benefits:

- no silent mutation of the protected E1 protocol;
- attempt identity remains consistent with the E1 event algebra;
- equivalence is measured against the implementation that actually exists;
- synthetic multi-resource evidence cannot be mistaken for live extraction;
- replay binds both policy and executable selector identity;
- revocation/freshness checks have explicit upstream evidence.

Costs:

- the first live extraction domain is intentionally narrow;
- F1A must introduce candidate E3 schemas and a reference adapter before the
  selector itself is meaningful;
- live multi-resource routing requires a later independently governed widening.

## Rejected alternatives

- adding new fields to the E1 `1.0` envelope while retaining the `1.0` label;
- treating a placement decision as a mechanical attempt;
- inventing stable worker identities for anonymous semaphore permits;
- claiming multi-resource equivalence from synthetic fixtures;
- binding only a human-readable scheduler version;
- using FABRIC's wall clock as freshness authority;
- allowing placement evidence to authorize queueing or dispatch.

## Non-authority

This ADR does not authorize E3B live routing, queue/dispatch authority, general
FABRIC activation, semantic extraction, replica authority, autonomous policy
adaptation, distributed FABRIC, or generalized GCL migration.


## F1A protected completion

F1A was reviewed on exact source head
`92fdb90d93bb7eadb0ff7036422d9320f8c155f9` and protected by squash merge
`24589252ca5584c1ac4385f8fd57b9c680eebbf3` on 6 October 2026.

The protected F1A implementation provides the negotiated E3 contract records,
canonicalization/digest machinery, exact-decision-time non-authoritative
control witness, and read-only local blocking-pool reference
snapshot/admissibility surfaces. It does not perform placement selection,
queueing, dispatch, route realization, or live FABRIC routing.

The next bounded successor is F1B: a pure deterministic selector over the F1A
contract. E3B live routing/cutover remains separately governed.


## F1B deterministic-selector decision

F1B adds exactly one pure selection policy over the protected F1A contract.
Eligibility is delegated exclusively to F1A `reference_resource_admissible`; F1B
must not introduce an independent permission predicate.

The exact policy artifact is
`schemas/aether_fabric/e3/f1b_selector_policy.json`, policy ID
`aether-fabric-f1b-lexicographic-v1`. Among eligible resources it ranks by:

1. lower queue depth;
2. greater available `blocking_admission_slot` capacity;
3. stable lexicographic resource ID.

The final tie-break removes resource-array-order dependence. Policy substitution
fails closed because the selector accepts only the exact artifact digest.

F1B emits only non-operative `PlacementSelected` or `PlacementUnavailable`
evidence. It remains pre-`RouteRealized`, carries no `mechanical_attempt_id`, and
has `authority_effect = none`. It is not connected to `aether_http` or any live
routing path. F1D remains the first shadow-integration tranche; E3B live cutover
remains separately governed.


## F1B protected completion

F1B was reviewed on exact source head `a010ad94672f32aac582f91bf73fc7bdec93340d` and protected by squash merge
`04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3` on 6 October 2026.

The protected selector filters exclusively through F1A admissibility and then
ranks eligible resources by lower queue depth, greater available admission
capacity, and stable resource ID. The exact policy artifact is digest-bound.
Placement evidence remains pre-`RouteRealized`, non-operative, and
`authority_effect = none`.

F1C hostile/replay evidence is protected at `c21aebe269a83fe729c226189f6643de7f6fc1da`. F1D non-operative shadow hookup is protected at `ebe535885ae73b7950a2f540a137f2836bdb2941`. F1E differential equivalence is the next bounded successor; E3B live routing remains separately governed.


## F1C protected completion

F1C was reviewed on exact source head `2ac11f579e9862c0d1da74674b05decd7b658bb4` and protected by squash merge
`c21aebe269a83fe729c226189f6643de7f6fc1da` on 6 October 2026. The tranche adds hostile/replay evidence only: exact
idempotency, canonical-equivalent ordering, stale/tampered snapshot rejection,
revoked/superseded exact-time witness rejection, and decision-identity separation
for changed governed inputs.

F1C does not perform the later placement-to-`RouteRealized` revocation recheck and
does not authorize shadow or live routing. F1D is the first non-operative
shadow-integration tranche.


## F1D protected completion

F1D was reviewed on exact source head `b19d3a736696dcfdd3737848d7b1654136410cca` and protected by squash merge `ebe535885ae73b7950a2f540a137f2836bdb2941` on 6 October 2026.

The protected hookup captures the current local blocking-pool `ResourceSnapshot` exactly once and supplies that same exact modeled input to the AETHER reference-admissibility predicate and the F1B selector. The AETHER reference path remains authoritative. FABRIC emits comparison evidence only with `authority_effect = none`.

The hookup acquires no permit, mutates no queue, reserves or dispatches no work, creates no attempt or `RouteRealized` event, and changes no semantic state. Its one-resource permitted-set comparison is valid only for the protected initial live extraction domain and is not a generalized multi-resource equivalence claim.

F1E differential-equivalence evidence is the next bounded successor. E3B live routing/cutover remains separately governed and unauthorized by F1D.


## F1E differential-equivalence candidate

F1E adds only an evidence adjudicator over the protected F1D comparison record.
It is explicitly limited to the admitted live domain containing exactly one
resource pool, `aether-local-blocking-pool`.

For that domain, F1E requires both:

```text
FABRIC permitted set == AETHER reference permitted set
FABRIC permitted set subset-of AETHER reference permitted set
```

The second predicate is recorded separately as the no-authorization-widening
property even though exact equality implies it. Any mismatch fails closed as an
evidence error and is not returned as a placement result.

The adjudicator additionally rejects:

- synthetic multi-resource evidence presented as live equivalence;
- authority-effect laundering;
- FABRIC decisions not bound to the exact compared snapshot digest; and
- selected-resource projections inconsistent with the underlying placement
  decision.

F1E changes no scheduler, semaphore, queue, route, retry, dispatch, attempt,
semantic, policy, provenance, or institutional-state behavior. The current
AETHER reference path remains authoritative.

F1E is protected on exact reviewed source head
`0ded819f4a46e50333e5c54c313c204f8006493a` by squash merge
`47bec0af7823c0295f55a6b5f6ae3dac832bfeae`.

The bounded F1A-F1E implementation/evidence sequence is therefore complete.
E3A exact-head adjudication/readback is the next governed step. E3B live
routing/cutover remains separately governed and unauthorized.
