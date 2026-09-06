# ADR 0020 — AETHER/FABRIC E0 Boundary Classification Before Extraction

Status: Proposed for E0 review, revised after Adversary pass
Date: 2026-09-05
Issue: #83

## Context

INTELLECT Council matter `GI-COUNCIL-AETHER-FABRIC-ENCAP-001` approved with
conditions the architectural direction of independently encapsulating AETHER
and FABRIC while preserving asymmetric semantics. Its durable Council record is
merged at `grandchallenge/INTELLECT` commit
`18df1c3ef89712fe00af37a484cc03ce270e99bd`.

The approved doctrine is:

- AETHER remains authoritative semantic state within its admitted domain;
- FABRIC may become an independently encapsulated mechanical
  coordination/data plane;
- POL supplies institutional semantics over AETHER;
- GHOS retains protected execution/controller governance;
- INTELLECT retains constitutional authority.

The Council requires E0 before code movement. AETHER protected `main` at E0
intake is `159cf930ae9130060f16d9fbc2608ad8a4069fae`.

## Decision

Classify material live AETHER **responsibilities**, not crates, before defining
an AETHER/FABRIC interface or moving implementation.

Each responsibility receives one E0 class:

- `SEMANTIC`;
- `INSTITUTIONAL`;
- `EXECUTION`;
- `FABRIC`;
- `AMBIGUOUS`.

`AMBIGUOUS` is a first-class result and prohibits extraction until a narrower
contract and discriminating evidence resolve the seam.

Canonical E0 artifacts:

- `docs/ARCHITECTURE/AETHER_FABRIC_E0_BOUNDARY_MATRIX.md`;
- `docs/ARCHITECTURE/AETHER_FABRIC_DEPENDENCY_GRAPH.md`;
- `docs/ARCHITECTURE/AETHER_FABRIC_AMBIGUITY_REGISTER.md`;
- `docs/ARCHITECTURE/AETHER_FABRIC_E0_EVIDENCE_INDEX.md`;
- this ADR.

## Semantic retention rule

A responsibility remains in AETHER when it determines or validates:

- semantic identity;
- append order or admitted history;
- semantic cuts/replay;
- schema admission;
- policy visibility;
- provenance;
- recursive derivation;
- proof identity/explanation;
- semantic lease/fence authority;
- semantic operation ordering/atomicity;
- partition/cut/import meaning;
- semantic leader epoch/fencing;
- semantic admission/rejection.

## Mechanical candidate rule

A responsibility may be a FABRIC candidate only when it can be specified as
non-authoritative physical realization inside an already authorized envelope,
for example:

- endpoint/resource observation;
- concrete placement;
- delivery;
- replica byte movement;
- pre-start worker/queue capacity;
- pre-start backpressure/timeout;
- locality/capacity observation.

Mechanical success or failure must not create semantic authority or reinterpret
an already-started semantic operation.

## Influence-without-authority rule

FABRIC can shape what the polity gets to observe even without writing semantic
facts. Scheduling, starvation, selective delivery, retry asymmetry, endpoint
eligibility, locality, and priority are therefore governed influence channels.

FABRIC may optimize only inside a bounded, attributable mechanical envelope
supplied by an authorized upstream decision.

## Resource-control refinement

The first Adversary pass found that current AETHER queue/resource control is
mixed, not purely mechanical. `docs/RESOURCE_CONTROL_CONTRACT.md` combines:

### Mechanical candidates

- global worker-pool capacity;
- concrete worker realization;
- queue occupancy/wait;
- pre-start timeout/backpressure observations.

### Semantic obligations retained in AETHER

- one-active-operation same-namespace serialization where required for
  deterministic semantic order;
- `cancel_before_start_complete_after_start`;
- a pre-start timeout may prevent start, but cannot later produce a hidden
  semantic commit;
- once semantic execution starts, a mechanical timeout cannot report failure
  while an authority mutation may still commit in the background;
- resource rejection cannot partially append authority or publish an AETHER
  execution receipt/trace handle;
- semantic runtime/rule/tuple boundedness remains an AETHER safety contract.

Therefore E0 does **not** classify queueing/backpressure wholesale as FABRIC.
The mechanical scheduler can realize capacity, but AETHER owns the semantic
start/completion/atomicity contract.

## Execution separation rule

GHOS controller admission, protected execution, credential authority, release
promotion, and protected workflow execution do not become FABRIC
responsibilities merely because FABRIC may later transport payloads.

`.ghos-routing/workflows.json` currently binds governed workflows to the
`GITHUB_ACTIONS` persistent controller and does not grant AETHER or FABRIC
controller authority.

## Live-main findings

### Clean AETHER semantic center

Strongly semantic responsibilities include:

- AST/DSL/schema;
- append admission/order/cuts/receipts;
- replay and merge;
- policy projection;
- rule plan/runtime/closure;
- provenance/proof identity;
- semantic coordination facts and lease fencing;
- semantic resource ordering/atomicity;
- partition/cut/import/federated meaning;
- leader epoch, semantic promotion, stale/divergent-prefix fencing;
- sidecar identity/provenance/policy.

### Clear mechanical candidates

Subject to the authorized-envelope constraints:

- replica lag/health observations;
- concrete endpoint/location configuration;
- worker-pool capacity and concrete worker realization;
- queue occupancy/wait/backpressure before semantic execution starts;
- audit-event transport;
- process/service liveness;
- host/capacity observations.

These remain in place during E0.

### Mixed seams

E1 must decompose:

- storage backend mechanics vs authoritative journal semantics;
- sidecar locality vs semantic reference/provenance/policy;
- replica movement vs epoch/prefix fencing;
- HTTP transport vs auth/policy/admission;
- namespace resolution vs worker realization;
- mechanical queue capacity vs semantic serialization/cancellation;
- semantic safety limits vs operational limits;
- compatibility facade ownership;
- cross-plane telemetry/performance.

## AETHER-POL lineage

PR #10 (`Introduce AETHER-POL semantic layer`) is historical lineage and is not
present on current protected `main`. Its explicit non-goals excluded scheduler,
workflow engine, queue, graph runner, model serving and autonomous runtime.

Any new POL realization requires fresh review against current AETHER contracts
and the approved AETHER/FABRIC doctrine.

## AETHER-Learn finding

No live AETHER-Learn implementation was found on current protected AETHER main
under the E0 searched router/allocation vocabulary. Desired allocation remains
a future governed institutional/learning contract, not something inferred from
HTTP namespace routing, queueing, partitions or capacity tooling.

## Consequences

Positive:

- semantic authority is protected around replay/admission/fencing/atomicity;
- genuine mechanical candidates are exposed without crate-level overreach;
- mixed seams receive concrete E1/E2 tests;
- local AETHER correctness remains a required path;
- FABRIC can later evolve for performance without becoming policy/authority.

Costs:

- operational code may remain inside AETHER until a narrower interface is
  proven;
- E1 must decompose responsibilities before extraction;
- resource scheduling needs an explicit start/completion handoff contract;
- historical terminology requires migration discipline.

## Forbidden effects

This ADR does not:

- create a FABRIC repository/runtime;
- move code/APIs;
- alter AETHER production semantic authority;
- amend INTELLECT Article IX;
- activate AETHER-POL or AETHER-Learn;
- admit AETHER/FABRIC as GHOS controllers;
- authorize production deployment or claim promotion.

## E0 acceptance

E0 may close only if:

1. material live responsibility families are represented;
2. each responsibility has one classification;
3. every `AMBIGUOUS` item has discriminating evidence requirements;
4. semantic cuts/fencing/admission/order/atomicity are not assigned to FABRIC;
5. GHOS and INTELLECT authority boundaries remain explicit;
6. historical lineage is preserved;
7. no code/runtime responsibility moved;
8. a non-authoring Adversary pass finds no unresolved material authority leak;
9. a non-authoring Referee pass confirms completeness.

## Next decision

If E0 is accepted, E1 may specify narrow versioned interface contracts for the
mixed seams. E1 is not itself authority to extract code. E2/E3/E4 remain gated
by semantic-equivalence, hostile scheduling/failure tests, non-AETHER FABRIC
utility, maintenance ownership, rollback and the Council's remaining
conditions.