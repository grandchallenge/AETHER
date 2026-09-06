# ADR 0020 — AETHER/FABRIC E0 Boundary Classification Before Extraction

Status: Proposed for E0 review
Date: 2026-09-05
Issue: #83

## Context

INTELLECT Council matter `GI-COUNCIL-AETHER-FABRIC-ENCAP-001` approved with
conditions the architectural direction of independently encapsulating AETHER
and FABRIC while preserving asymmetric semantics.

The Council's durable record is merged in `grandchallenge/INTELLECT` at
`18df1c3ef89712fe00af37a484cc03ce270e99bd`.

The approved doctrine is:

- AETHER remains authoritative semantic state within its admitted domain;
- FABRIC may become an independently encapsulated mechanical
  coordination/data plane;
- POL supplies institutional semantics over AETHER;
- GHOS retains protected execution/controller governance;
- INTELLECT retains constitutional authority.

The Referee required E0 before any code movement. In particular, existing
AETHER crates must not be moved wholesale merely because their names or current
implementation mix semantic and operational responsibilities.

AETHER protected `main` at E0 intake is
`159cf930ae9130060f16d9fbc2608ad8a4069fae`.

## Decision

Perform a responsibility-level classification of the live AETHER estate before
introducing an AETHER/FABRIC interface, moving code, creating a FABRIC runtime,
or changing repository custody.

Each material responsibility is assigned exactly one E0 classification:

- `SEMANTIC`;
- `INSTITUTIONAL`;
- `EXECUTION`;
- `FABRIC`;
- `AMBIGUOUS`.

`AMBIGUOUS` is a first-class result. It records that evidence or a finer
contract boundary is still required.

The canonical E0 artifacts are:

- `docs/ARCHITECTURE/AETHER_FABRIC_E0_BOUNDARY_MATRIX.md`;
- `docs/ARCHITECTURE/AETHER_FABRIC_DEPENDENCY_GRAPH.md`;
- `docs/ARCHITECTURE/AETHER_FABRIC_AMBIGUITY_REGISTER.md`;
- this ADR.

## Classification principles

### Semantic retention principle

A responsibility remains in AETHER when it determines or validates:

- semantic identity;
- append order or admitted history;
- semantic cuts/replay;
- schema admission;
- policy visibility;
- provenance;
- recursive derivation;
- proof/explanation;
- semantic lease/fence authority;
- partition/cut/import meaning;
- semantic leader epoch/fencing;
- semantic admission/rejection.

### Mechanical candidate principle

A responsibility may be a FABRIC candidate when it can be specified entirely
as mechanical realization under an already authorized envelope, such as:

- endpoint/resource discovery;
- physical placement;
- delivery;
- queueing/backpressure;
- retry mechanics;
- replica byte movement;
- endpoint/resource liveness;
- locality/capacity observation.

Mechanical success must not create semantic authority.

### Influence-without-authority principle

A responsibility is not safely mechanical merely because it never writes a
semantic fact directly. Scheduling, priority, selective delivery, starvation,
retry asymmetry, endpoint eligibility, or locality policy can materially shape
what the polity gets to observe.

A future FABRIC may optimize only inside a bounded, attributable mechanical
envelope supplied by an authorized upstream decision.

### Execution separation principle

GHOS controller admission, protected execution, credential authority, release
promotion, and similar protected execution responsibilities do not become
FABRIC responsibilities merely because FABRIC might later transport their
payloads.

## Live-main findings

### Clean semantic center

The following current crates/responsibilities are strongly semantic and are not
E0 extraction candidates:

- `aether_ast` canonical semantic types;
- `aether_schema` schema/type contracts;
- resolver replay/merge/dependency certification;
- rules/plan/runtime recursive semantics;
- explanation/provenance;
- semantic service admission and execution receipts;
- semantic coordination facts and lease/fence meaning;
- partition/cut/federated-import semantics;
- authority epochs, stale-epoch rejection, and divergent-prefix fencing.

### Clean mechanical candidates

The current estate already contains a smaller set of clearly mechanical
responsibilities:

- replica lag/health observations;
- concrete replica endpoint/path configuration;
- queue/backpressure mechanics under fixed semantic namespace identity;
- audit-event buffering/delivery;
- process/service liveness;
- host/capacity observations.

These remain in place during E0.

### Mixed seams

The most important E1 candidates are not whole crates. They are mixed seams
inside or across:

- storage backend mechanics versus authoritative journal semantics;
- sidecar payload locality versus semantic reference/provenance/policy;
- replica movement versus authority-partition epoch/fencing;
- HTTP transport versus AETHER auth/policy/admission;
- namespace semantic resolution versus worker/queue realization;
- semantic safety limits versus operational resource limits;
- compatibility facade ownership;
- cross-plane telemetry/performance measurement.

## AETHER-POL lineage

PR #10, `Introduce AETHER-POL semantic layer`, is preserved as historical
lineage and is not present in current protected `main`.

Its explicit non-goals excluded scheduler, workflow engine, queue, graph runner,
model-serving layer, and autonomous agent runtime. That separation is
compatible with the Council doctrine, but the stale PR is not itself an
admitted current implementation.

Any future POL candidate must be freshly reviewed against current AETHER
schema/policy/provenance contracts and the AETHER/FABRIC separation.

## AETHER-Learn finding

No live AETHER-Learn implementation was found on current AETHER `main` using
the E0 router/allocation search vocabulary.

Desired allocation therefore remains a future institutional/learning contract,
not a responsibility that can be inferred from existing HTTP namespace routing
or partition mechanics.

## Consequences

### Positive

- prevents premature repository surgery;
- preserves semantic authority around cuts, replay, admission, and fencing;
- identifies genuine mechanical extraction candidates;
- makes mixed responsibilities explicit rather than hiding them behind crate
  names;
- provides concrete E1 contract and E2 falsification targets;
- preserves a local AETHER correctness path.

### Costs

- E1 must decompose existing outer-layer responsibilities before extraction;
- some apparently operational code remains inside AETHER until tests prove a
  narrower boundary;
- future FABRIC work cannot begin as a broad rewrite of `aether_partition` or
  `aether_http`;
- historical terminology requires careful migration.

## Forbidden effects of this ADR

This ADR does not:

- create a FABRIC repository or runtime;
- move code;
- change the AETHER production semantic authority boundary;
- amend INTELLECT Article IX;
- activate AETHER-POL;
- establish AETHER-Learn;
- admit AETHER/FABRIC as GHOS controller infrastructure;
- authorize a production deployment.

## E0 acceptance

E0 may close only if:

1. the matrix represents all material current responsibility families;
2. every responsibility has exactly one classification;
3. every `AMBIGUOUS` entry has a discriminating evidence requirement;
4. semantic fencing/cuts/admission are not assigned to FABRIC;
5. GHOS/INTELLECT boundaries remain explicit;
6. historical lineage is preserved;
7. no code/runtime responsibility moved during E0;
8. a non-authoring Adversary pass attacks authority leakage and selective
   influence;
9. a non-authoring Referee pass confirms completeness.

## Next decision

If E0 is accepted, E1 may specify narrow versioned interface contracts for the
mixed seams.

E1 is not authorized to extract code merely because it can name an interface.
Any E2/E3/E4 transition remains subject to the Council's semantic-equivalence,
hostile-testing, non-AETHER utility, maintenance, and rollback conditions.
