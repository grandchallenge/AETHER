# AETHER/FABRIC E3B-R — Runtime Cutover Readiness Adjudication

Status: protected readiness complete — activation BLOCKED
Issue: #110
Protected basis: `8ca561ea18bc96aba942aeae5e1392f4862b7e1a`
Protected E3A merge: `23353cf01e5c8cefef18eedc8b878c2290fb50f0`

## Purpose

E3B-R adjudicates whether the protected F1 selector can safely cross the
placement-to-mechanical-realization boundary without changing AETHER semantic
authority.

E3B-R is not production activation.

## Readiness implementation

### B0 — control-path law

`AETHER_FABRIC_E3B_LIVE_CONTROL_PATH.md` and ADR 0026 define:

- placement evidence remains non-authoritative;
- realization requires a fresh exact-time governed control witness;
- realization rechecks the selected resource against a fresh snapshot;
- `mechanical_attempt_id` begins only at `RouteRealized`;
- reference routing remains default;
- activation is a separate reserved event.

### B1 — pure realization bridge

`aether_fabric::realize_selected_placement` validates the exact historical
placement/envelope/constraint binding, constructs a fresh realization-time
constraint projection, reuses the protected F1A admissibility law against the
fresh witness/snapshot, and only then emits E1-shaped `RouteRealized`
evidence.

The logical attempt identity is digest-bound to:

- the exact canonical realization-request record;
- placement decision identity;
- fresh control-witness digest;
- fresh resource-snapshot digest.

Exact replay is idempotent. A retry request receives a different attempt
identity.

### B2 — default-OFF integration guard

`HttpKernelOptions::fabric_routing_mode` has exactly two readiness values:

- `reference_only` — default;
- `candidate_readiness`.

There is no live-FABRIC mode.

The candidate-readiness surface captures the current resource snapshot
read-only and invokes the B1 bridge. It does not acquire a worker permit,
enqueue work, execute an HTTP operation, or alter semantic state.

### B3 — hostile/replay evidence

The readiness tests fail closed on:

- late revocation;
- late supersession;
- late expiry;
- stale realization snapshot;
- selected resource no longer available;
- forged placement/constraint binding;
- synthetic resource substitution outside the admitted live domain;
- candidate realization while the routing mode remains default/reference-only.

They also prove:

- exact realization replay is idempotent;
- retry request identity produces a distinct mechanical attempt;
- candidate readiness leaves the blocking-pool snapshot unchanged.

Protected E2/F1A conformance is replayed alongside these tests.

## Live integration finding

The ordinary protected HTTP execution path currently proceeds:

```text
authentication + namespace authorization
 -> rate limiting
 -> namespace admission
 -> BoundedBlockingExecutor::run(...)
 -> AETHER semantic operation
```

That path does not currently receive runtime instances of:

- `MechanicalEnvelopeAuthorized`;
- `PlacementConstraintSet`;
- a fresh governed `ControlStateWitness`;
- `PlacementSelected`.

This is decisive for activation.

FABRIC cannot safely synthesize those records from successful HTTP
authentication, namespace admission, local capacity, or liveness. Such
synthesis would make the mechanical scheduler its own authorization issuer and
would violate the E1/E3 authority boundary.

## Readiness acceptance

1. Placement-to-`RouteRealized` relationship explicit/versioned — **PASS**.
2. Fresh realization-time control-state recheck — **PASS**.
3. Revocation/supersession/expiry between placement and realization block new
   work — **PASS**.
4. Attempt identity created only at realization — **PASS**.
5. Exact replay idempotent; retry identity distinct — **PASS**.
6. Selected endpoint/resource revalidated at realization — **PASS**.
7. E3 narrowing remains enforced through protected F1A law — **PASS**.
8. Mechanical evidence has `authority_effect = none` — **PASS**.
9. Reference routing remains default — **PASS**.
10. Candidate-readiness surface has no queue/permit/semantic side effect —
    **PASS**.
11. E2/F1A conformance and F1 hostile/replay evidence remain green —
    **PASS subject to final exact-head replay**.
12. Full workspace/clippy/format integrity — **PASS subject to final exact-head
    replay**.
13. Upstream runtime control-record producer identified and protected —
    **NOT SATISFIED FOR ACTIVATION**.
14. Exact live-routing delta exists — **NOT SATISFIED FOR ACTIVATION**.
15. Reserved production/control-plane disposition exists — **NOT SATISFIED FOR
    ACTIVATION**.

## Candidate disposition

### E3B-R

**READINESS PASS**, subject to exact-head replay, reviews, protected checks,
merge, and readback of this candidate.

### E3B-A

**BLOCKED / NOT AUTHORIZED.**

The blocking dependencies are substantive:

1. an AETHER/upstream-governed runtime producer must supply the exact E1/E3
   control records to the live execution boundary;
2. the exact live-routing and rollback delta must then be reviewed against that
   producer;
3. the reserved production/control-plane activation disposition must be
   authentically present on the exact activation packet.

If item 1 or 2 changes the effective Article IX/AETHER authority boundary, an
effective Article XI amendment becomes a prerequisite as well.

## Rollback

The readiness candidate requires no semantic-state migration. The default
`reference_only` mode is the existing behavior, and candidate-readiness
evidence owns no semantic state.

## Stop boundary

After protected E3B-R completion and preparation of the exact A0 activation
packet, execution SHALL stop at the **E3B-A RESERVED LIVE ACTIVATION BOUNDARY**
until the named activation prerequisites exist.

No readiness success may be converted into activation authority.


## Protected E3B-R completion

E3B-R was reviewed on exact source head
`209ec0cbfa49af8bb3827650f2c1f2b7640253d1` and protected by squash merge
`04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`.

Exact-head reviews:

- Formalist PASS — review `PRR_kwDORqA9d88AAAABRFGqIQ`;
- Adversary PASS — review `PRR_kwDORqA9d88AAAABRFGtoA`;
- Referee COMPLETE — review `PRR_kwDORqA9d88AAAABRFGweA`.

Exact-head protected gates:

- Required CI gate `112754487840` — success;
- Required Supply Chain gate `112753765429` — success;
- policy / policy `112753690518` — success;
- security / action-policy `112753690733` — success;
- routing-enforcement `112753679790` — success;
- unresolved review threads — none.

Protected `main` read back exactly at
`04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`.

The E3B-R readiness transaction is therefore complete. E3B-A remains blocked
and unauthorized for the reasons already recorded above.
