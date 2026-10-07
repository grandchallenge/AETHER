# ADR 0026 — E3B live control-path cutover boundary

Status: runtime-readiness protected; activation reserved and blocked
Date: 2026-10-07
Issue: #110
Protected basis: `8ca561ea18bc96aba942aeae5e1392f4862b7e1a`
Protected E3A merge: `23353cf01e5c8cefef18eedc8b878c2290fb50f0`

## Context

E3A proved that the protected F1 selector computes the same permitted placement
set as AETHER's current reference selector for the admitted one-resource live
domain. It deliberately stopped before routing.

The remaining problem is no longer selector equivalence. It is the control
boundary between historical non-authoritative placement evidence and a real E1
mechanical attempt.

The current AETHER HTTP path does not yet carry runtime E1/E3 control records.
It authenticates and admits a request, then directly uses the bounded blocking
executor. FABRIC must not infer or self-issue upstream authorization from that
fact.

## Decision

### 1. Split readiness from activation

E3B-R may implement and protect cutover-readiness mechanisms while the default
and actual live path remain the AETHER reference route.

E3B-A is a later reserved activation event.

### 2. Revalidate at realization

A historical `PlacementSelected` does not authorize `RouteRealized`.

The realization bridge must consume a fresh exact-time control witness and a
fresh resource snapshot. Revoked, superseded, expired, stale, unavailable or
binding-mismatched inputs fail closed before a mechanical attempt exists.

### 3. Mint attempt identity only at realization

`mechanical_attempt_id` begins at the realization boundary and is deterministic
over the exact canonical realization-request record plus exact
placement/current-state evidence.

Repeated delivery of the same realization request is idempotent. A retry uses a
different realization-request identity.

### 4. Preserve E1 event shape

B1 emits the existing E1 `RouteRealized` shape. It does not silently extend or
rewrite E1 `1.0`.

The placement decision is linked through `causation_id`; no new semantic
authority field is introduced.

### 5. Default to reference routing

The only readiness modes are:

- `reference_only` — default and current behavior;
- `candidate_readiness` — enables the non-live realization-evidence surface.

There is no live-FABRIC mode in E3B-R.

### 6. Do not invent the upstream producer

The current live HTTP path lacks runtime `MechanicalEnvelopeAuthorized`,
`PlacementConstraintSet`, current `ControlStateWitness`, and
`PlacementSelected` records.

Until AETHER or another governed upstream issuer supplies those records, no
safe live cutover can occur. FABRIC may not synthesize them from authentication,
liveness, queue state, or local configuration.

### 7. Preserve constitutional semantics

The E3B-R design does not change Article IX or AETHER semantic authority.

If a future exact activation candidate does change effective authority, Article
XI becomes a prerequisite. Regardless, live production activation remains a
separate reserved control-plane disposition.

## Consequences

Benefits:

- closes the placement-to-attempt revocation race;
- creates exact retry/idempotency semantics;
- preserves E1 schema identity;
- keeps rollback semantic-state-free;
- prevents successful HTTP authentication from being laundered into FABRIC
  authorization;
- makes the actual activation delta small and auditable.

Costs:

- E3B-R cannot yet route ordinary live HTTP operations through FABRIC;
- a governed upstream control-record producer is now an explicit prerequisite;
- activation needs a separate exact packet and reserved disposition.

## Rejected alternatives

- treating `PlacementSelected` as execution authority;
- reusing its historical control witness at realization;
- minting a mechanical attempt during placement;
- auto-generating E1/E3 authority records inside FABRIC;
- enabling FABRIC routing by default;
- treating E3A equivalence proof as activation approval;
- changing Article IX implicitly through runtime behavior.

## Non-authority

This ADR authorizes no production cutover. E3B-A remains separately governed.


## Protected readiness completion

The E3B-R readiness implementation was reviewed on exact source head
`209ec0cbfa49af8bb3827650f2c1f2b7640253d1` and protected as
`04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`.

This ADR still does not authorize E3B-A. The missing upstream-governed runtime
control-record producer and exact live-routing/rollback delta are governance
dependencies, not FABRIC implementation details that the scheduler may infer.
