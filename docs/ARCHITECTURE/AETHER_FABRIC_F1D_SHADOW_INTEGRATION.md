# AETHER/FABRIC F1D — Non-operative Shadow Integration

Status: protected — merge `ebe535885ae73b7950a2f540a137f2836bdb2941`
Issue: #94
Protected source head: `b19d3a736696dcfdd3737848d7b1654136410cca`
Protected F1D merge: `ebe535885ae73b7950a2f540a137f2836bdb2941`
Protected F1C documentary basis: `1bf7f2accc1cd4dae15bffbd016654e838d5e452`
Protected F1C implementation/evidence merge: `c21aebe269a83fe729c226189f6643de7f6fc1da`

## Objective

F1D connects the protected F1B selector to the current AETHER local blocking-pool reference surface in shadow mode only.

The integration captures one read-only `ResourceSnapshot` from the current AETHER blocking executor and supplies that exact snapshot to both:

1. the protected AETHER reference-admissibility predicate; and
2. the protected F1B deterministic selector.

The result is a `FabricShadowPlacementComparison` containing the exact snapshot, the AETHER reference permitted resource set, the FABRIC shadow decision, the FABRIC-selected resource identity when one exists, and a boolean indicating whether the two observed sets agree.

## Authority rule

The AETHER reference path remains authoritative. The FABRIC decision and the comparison record are evidence only and carry `authority_effect = none`.

F1D does not:

- acquire an admission or worker permit;
- enqueue or reserve work;
- dispatch a payload;
- create or bind a `mechanical_attempt_id`;
- create `RouteRealized`;
- authorize retry;
- start or complete a semantic attempt;
- admit a result;
- mutate policy, provenance, namespace state, or institutional state; or
- activate live FABRIC routing.

A disagreement is therefore observable comparison evidence, not a routing decision or authority change.

## Exact-input rule

F1D deliberately captures the local-pool snapshot once. It does not call two convenience adapters that could observe semaphore counters at different instants and then compare those observations as if they were the same input.

The one captured snapshot is passed unchanged to the AETHER reference predicate and the F1B selector together with the same exact E1 envelope bytes, placement constraint, control-state witness, policy artifact, and selector implementation identity.

## Evidence in this tranche

F1D tests establish that:

1. an eligible current local pool yields the same single-resource outcome on both shadow arms;
2. an intentionally unavailable current local pool yields the same empty outcome on both arms;
3. the comparison path leaves the read-only local-pool snapshot unchanged before and after evaluation; and
4. a revoked exact-time control witness fails closed rather than producing a comparison decision.

These are hookup and non-interference tests. They are not the full differential-equivalence proof required by F1E.

## Protected completion

F1D was reviewed on exact source head `b19d3a736696dcfdd3737848d7b1654136410cca` and protected by squash merge `ebe535885ae73b7950a2f540a137f2836bdb2941` on 6 October 2026.

Exact-head audit passes:

- Formalist PASS — review `5436389258`;
- Adversary PASS — review `5436390581`;
- Referee COMPLETE — review `5436391792`.

Exact-head protected checks:

- Required CI gate `112579107752` — success;
- Required Supply Chain gate `112578632536` — success;
- policy / policy `112578588220` — success;
- security / action-policy `112578588548` — success;
- routing-enforcement `112578586809` — success.

No inline review threads remained at merge.

## Rollback

F1D owns no semantic state. Removing or disabling calls to `fabric_shadow_compare_placement` requires no semantic-state migration and leaves the existing AETHER resource-control path unchanged.

## Successor boundary

F1E is the next bounded tranche: differential-equivalence evidence over the admitted live extraction domain, including hostile mismatch/no-widening evidence.

E3B live routing/cutover remains separately governed and is not authorized by F1D.
