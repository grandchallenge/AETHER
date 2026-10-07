# AETHER/FABRIC F1C — Hostile and Replay Evidence

Status: protected — merge `c21aebe269a83fe729c226189f6643de7f6fc1da`
Issue: #94
Protected source head: `2ac11f579e9862c0d1da74674b05decd7b658bb4`
Protected F1C merge: `c21aebe269a83fe729c226189f6643de7f6fc1da`
Protected F1B basis: `04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3`
Protected documentary basis: `4bf216cfc96a2c5a8c238be6e0f2c23f89cf91eb`

## Objective

F1C adds evidence only. It does not change selector behavior. The tranche attacks the protected F1B selector at the exact replay, revocation-race, integrity, and logical-idempotency boundaries required by issue #94.

## Evidence obligations

The F1C test pack proves that:

1. repeated identical selected and unavailable inputs are logically idempotent;
2. canonically equivalent resource snapshots with different input ordering produce the same snapshot digest and placement decision;
3. a revoked or superseded exact-time control witness blocks a new placement even when the placement constraint is rebound to that exact witness;
4. a stale snapshot cannot be replayed into a placement;
5. mutation of a resource snapshot without recomputing its digest is rejected;
6. changed resource-snapshot state changes placement identity;
7. changed active control revision changes placement identity; and
8. changed selector implementation identity changes placement identity.

These tests complement the protected F1B evidence for policy substitution, ineligible-resource exclusion, no-eligible-resource handling, deterministic ranking, and pre-attempt/non-authority semantics.

## Revocation race boundary

F1C verifies the decision-time half of the race rule: placement requires an exact-time `ControlStateWitness` in `Active` state. A later revocation may leave older placement evidence as historical evidence, but F1C does not authorize realization from it. The placement-to-`RouteRealized` recheck belongs to the later shadow/integration lane and remains outside this tranche.

## Non-authority

F1C changes tests and documentation only. It does not wire FABRIC into `aether_http`, acquire permits, enqueue work, reserve resources, dispatch payloads, create attempts, authorize retries, start semantics, admit results, or activate routing.

F1D remains the first authorized non-operative shadow-hookup tranche. E3B live routing/cutover remains separately governed.

## Protected completion

F1C was reviewed on exact source head `2ac11f579e9862c0d1da74674b05decd7b658bb4` and protected by squash merge
`c21aebe269a83fe729c226189f6643de7f6fc1da` on 6 October 2026.

The protected evidence demonstrates exact replay/idempotency, canonical-equivalent
snapshot ordering, stale and digest-tampered snapshot rejection, exact-time revoked
and superseded control-state rejection, and replay-identity separation when the
resource snapshot, active control revision, or selector implementation changes.

The next bounded successor is F1D: non-operative shadow integration beside the
AETHER reference selector. F1D must not transfer routing authority; F1E differential
equivalence and E3B live routing/cutover remain separately governed.
