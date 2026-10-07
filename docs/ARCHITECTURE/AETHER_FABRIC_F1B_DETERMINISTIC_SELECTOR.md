# AETHER/FABRIC F1B — Pure Deterministic Resource Selector

Status: protected — merge `04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3`
Issue: #94
Protected source head: `a010ad94672f32aac582f91bf73fc7bdec93340d`
Protected F1B merge: `04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3`
Protected F1A basis: `24589252ca5584c1ac4385f8fd57b9c680eebbf3`
Protected documentary basis: `397a1c8be6a6cf69fc1e01271ed40f575e58e6b0`

## Objective

F1B adds one pure function over the protected F1A contract:

> filter resources exclusively through F1A admissibility, deterministically rank
> the eligible resources, and emit non-operative placement evidence.

F1B does not connect that decision to AETHER routing, queueing, dispatch,
`RouteRealized`, retries, semantic start, result admission, or institutional
state.

## Exact policy artifact

The policy artifact is:

`schemas/aether_fabric/e3/f1b_selector_policy.json`

Policy ID:

`aether-fabric-f1b-lexicographic-v1`

The selector accepts only the exact artifact digest returned by
`f1b_scheduler_policy_identity()`. Policy substitution fails closed.

## Eligibility

F1B does not implement a second authority predicate. Every candidate resource is
filtered through protected F1A `reference_resource_admissible(...)` using the
same exact E1 envelope, E3 placement constraint, resource snapshot, and
control-state witness.

Therefore:

`Selected(r,e,s,c) => reference_resource_admissible(r,e,s,c)`.

Malformed, stale, widened, revoked, superseded, mismatched, or otherwise invalid
governed input returns an error rather than a permissive placement.

## Ranking

Among eligible resources only, ranking is lexicographic:

1. lower `queue_depth`;
2. greater `available_capacity.available`;
3. lexicographically smaller stable `resource_id`.

The final resource-ID tie-break makes the result independent of resource-array
ordering. The protected live F1A domain currently contains one local blocking
resource pool, so multi-resource ranking remains contract evidence rather than a
claim of live multi-resource extraction.

## Decision evidence

F1B emits one of:

- `PlacementSelected`; or
- `PlacementUnavailable` with reason `no_eligible_resource`.

Both bind the F1A decision-input digest, exact policy identity, exact selector
implementation identity, exact control witness, and exact resource snapshot.
`placement_decision_id` and `event_id` are deterministic for identical exact
inputs.

Placement remains pre-attempt evidence: it carries no `mechanical_attempt_id`
and `authority_effect = none`.

## Idempotency and replay

For identical exact F1A decision inputs, exact policy artifact, and exact
selector implementation identity, F1B returns byte-equivalent logical decision
content.

Changing snapshot, constraints, control witness, policy artifact, or selector
implementation identity changes the replay identity according to F1A digest
rules.

## Non-authority

F1B is not wired into `aether_http` or any execution path.

It does not:

- acquire permits;
- enqueue work;
- reserve a resource;
- dispatch a payload;
- create a mechanical attempt;
- authorize semantic start;
- mutate policy or semantic state;
- activate FABRIC routing.

F1D remains the first tranche that may introduce a non-operative shadow hookup.
E3B live routing/cutover remains separately governed.

## Acceptance

F1B is complete only when the exact candidate head has:

- deterministic replay tests;
- ranking-order and tie-break tests;
- hostile ineligible-resource exclusion;
- exact policy-artifact substitution rejection;
- no-eligible-resource evidence;
- pre-attempt/non-authority evidence;
- full workspace regression and clippy passes;
- Formalist, Adversary, and Referee exact-head dispositions;
- all required protected checks green;
- no unresolved review threads;
- protected merge and protected-main readback.
