# ADR 0025 — AETHER/FABRIC E3/F1 Placement-Selection Extraction

Status: F1A protected — implementation merge `24589252ca5584c1ac4385f8fd57b9c680eebbf3`; F1B pending
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
