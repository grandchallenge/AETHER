# AETHER/FABRIC E3B — Live Control-Path Governance

Status: runtime-readiness candidate; activation not authorized
Issue: #110
Protected basis: `8ca561ea18bc96aba942aeae5e1392f4862b7e1a`
Protected E3A merge: `23353cf01e5c8cefef18eedc8b878c2290fb50f0`

## 1. Purpose

E3B is the first separately governed control-path tranche after E3A proved
implementation equivalence.

Its objective is narrow:

> make a FABRIC placement eligible to become an E1 `RouteRealized`
> mechanical attempt only after a fresh realization-time AETHER/governed
> control-state recheck, while keeping AETHER semantic authority unchanged.

E3B is split into readiness and activation. Repository code may prove cutover
readiness without activating live production routing.

## 2. Authority classification

E3B changes mechanical control flow only. It MUST NOT change:

- semantic admission or semantic attempt identity;
- namespace authority or same-namespace semantic ordering;
- policy interpretation or visibility;
- provenance, replay, cuts, leader epochs or fencing;
- institutional or constitutional authority;
- GHOS controller admission.

Current Constitution 1.2.0 / Article IX remains controlling. If an exact E3B
candidate changes the effective AETHER authority boundary, activation is blocked
until the Article XI process is effective.

## 3. Protected predecessor

F1A-F1E and E3A are complete and protected. E3A proved:

- E1 `aether-fabric/1.0` stayed byte/schema stable;
- E3/F1 placement records are explicit `1.1` evidence;
- the admitted live domain is exactly `aether-local-blocking-pool`;
- shadow selection equals the AETHER reference permitted set;
- FABRIC cannot widen the authorized placement set.

E3B may rely on those exact protected results but may not reinterpret them as
live activation.

## 4. Placement-to-realization law

`PlacementSelected` remains historical, non-authoritative evidence.

A later realization request SHALL create a mechanical attempt only when all of
the following hold at realization time:

1. the placement binds the exact E1 envelope and exact E3 placement constraint;
2. the placement itself has `authority_effect = none`;
3. a fresh `ControlStateWitness` names the same envelope and exact envelope
   digest;
4. the fresh witness is observed at the exact realization time;
5. the fresh witness is `Active`;
6. the selected resource is still admissible under a fresh exact resource
   snapshot and the same upstream placement constraints;
7. the selected endpoint is the same endpoint/resource named by the placement;
8. E3 constraints still narrow E1;
9. no implicit protocol downgrade occurs.

A historical placement control witness is never accepted as the current
realization witness merely because the placement was once valid.

## 5. Mechanical attempt identity

A `mechanical_attempt_id` is created only at the realization boundary.

For readiness v1 it is a deterministic digest over:

- the exact canonical `RealizationRequest` record, including request ID,
  realization time, and occurred-at evidence time;
- the placement-decision identity;
- the fresh control-witness digest;
- the fresh resource-snapshot digest.

Therefore:

- repeating the exact same realization request is logically idempotent;
- a retry uses a distinct realization-request identity and yields a new
  mechanical attempt;
- a changed current witness or resource snapshot yields a different attempt.

The emitted record is an E1 `RouteRealized` mechanical event with
`authority_effect = none`.

## 6. B2 routing-mode guard

The readiness implementation defines only:

- `reference_only` — default;
- `candidate_readiness` — explicitly enables the E3B realization-evidence
  surface for tests/readiness exercises.

There is intentionally no `live_fabric` mode in E3B-R.

`candidate_readiness` does not replace any current HTTP execution path and
does not call the blocking executor. It captures the current read-only pool
snapshot and runs the pure realization bridge.

A later E3B-A activation delta must be separately reviewed and must add the
actual live-routing configuration only after reserved activation authority is
present.

## 7. Current live integration seam

The protected live HTTP path performs:

```text
HTTP authentication / namespace authorization
 -> rate limit
 -> namespace admission
 -> BoundedBlockingExecutor::run(...)
 -> AETHER semantic operation
```

At the protected E3B basis, that path does **not** carry runtime instances of:

- `MechanicalEnvelopeAuthorized`;
- `PlacementConstraintSet`;
- current `ControlStateWitness`;
- `PlacementSelected`.

Therefore the scheduler cannot safely self-mint those records. Doing so inside
FABRIC would collapse upstream authorization into mechanical routing.

This missing upstream control-record producer is an explicit E3B readiness
dependency, not something FABRIC may infer from successful HTTP authentication.

## 8. Rollback / kill switch

Before activation, rollback is simply `reference_only`, which is the default
and current behavior.

After any future activation, the exact activation packet must prove that
returning to `reference_only`:

- requires no semantic-state migration;
- does not rewrite semantic attempts or provenance;
- cannot strand a started semantic attempt;
- preserves historical mechanical evidence;
- stops new FABRIC realization requests.

## 9. B3 hostile/replay obligations

At minimum readiness evidence SHALL cover:

- default mode rejects candidate realization;
- exact identical realization request is idempotent;
- retry identity yields a new mechanical attempt;
- revocation after placement blocks realization;
- supersession after placement blocks realization;
- expiry after placement blocks realization;
- forged placement/constraint binding is rejected;
- current selected resource disappearance/unavailability is rejected;
- candidate realization does not acquire permits or mutate queue state;
- endpoint/resource substitution is rejected;
- synthetic multi-resource placement cannot be promoted into the one-resource
  live domain;
- E2 mechanical-authority/no-work-after-revocation invariants remain green.

## 10. B4 readiness acceptance

E3B-R is complete only when:

- B0 specification/ADR are protected;
- B1 realization bridge is exact-head reviewed and tested;
- B2 default-OFF guard is exact-head reviewed and tested;
- B3 hostile/replay evidence is green;
- applicable E2/F1A-F1E evidence replays;
- full Rust workspace and clippy are green;
- no unresolved review threads remain;
- protected merge/readback is exact;
- the readiness record names every remaining activation dependency.

E3B-R completion does not authorize live production routing.

## 11. E3B-A activation gate

Before live FABRIC routing may be represented as active, a separate exact
activation packet must bind:

- the protected E3B-R commit;
- the exact upstream control-record producer;
- the exact live routing code/config delta;
- the exact rollback delta;
- current Constitution/authority analysis;
- fresh Formalist, Adversary and Referee reviews;
- required protected checks;
- production canary plan and abort condition;
- authentic reserved Human Steward/control-plane disposition.

Automation may prepare and validate this packet. It may not manufacture the
reserved activation disposition.

## 12. Non-authority

E3B-R does not authorize E3B-A, general FABRIC activation, semantic extraction,
queue ownership, autonomous retries, transport extraction, replica authority,
distributed FABRIC, or generalized GCL migration.
