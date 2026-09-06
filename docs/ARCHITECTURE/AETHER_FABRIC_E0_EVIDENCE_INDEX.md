# AETHER/FABRIC E0 Evidence Index

Status: E0 evidence map, revised after Adversary pass
Issue: #83
AETHER protected-main basis: `159cf930ae9130060f16d9fbc2608ad8a4069fae`
INTELLECT Council basis: `18df1c3ef89712fe00af37a484cc03ce270e99bd`

## Scope statement

This index binds the E0 classifications to inspectable live repository
surfaces. E0 claims coverage of **material responsibility families**, not a
line-by-line permanent assignment of every function. Mixed surfaces remain
`AMBIGUOUS` precisely to avoid inferring architecture from file/crate names.

## Governing sources

| Evidence | E0 use |
| --- | --- |
| INTELLECT `GI-COUNCIL-AETHER-FABRIC-ENCAP-001` at `18df1c3e...` | Governing separation doctrine, influence-without-authority threat, E0-before-extraction and semantic-equivalence conditions. |
| AETHER `AGENTS.md` | Rust semantic kernel is authoritative; service/API concerns may not dictate rule semantics; production-semantic activation remains gated. |
| AETHER `SPEC.md` | Authoritative datom/replay/rule/provenance/coordination semantics, lease fencing and sidecar subordination. |
| `Cargo.toml` | Live workspace crate inventory. |
| `docs/ARCHITECTURE_BOUNDARIES.md` | Current R6 crate ownership/dependency contract. |
| ADR 0019 | Responsibility-crate extraction already separated service, HTTP, sidecar, partition, perf and pilot from the `aether_api` catch-all; facade remains temporary. |
| `docs/STATUS.md` | Current controlled-alpha scope and implemented partition/resource/release surfaces. |

## Semantic authority evidence

| Family | Primary evidence | E0 conclusion |
| --- | --- | --- |
| canonical types/DSL | `AGENTS.md`, `SPEC.md`, AST/rules crates | `SEMANTIC`. |
| schema/admission | schema crate, ADR 0012 | `SEMANTIC`; validation belongs before authoritative commit. |
| journal append/cuts/receipts | storage/service, ADR 0012 | Accepted history is `SEMANTIC`; backend I/O remains mixed. |
| replay/policy projection | resolver, ADR 0010 | `SEMANTIC`; policy is part of input before replay/compile. |
| rule plan/closure | rules/plan/runtime, ADR 0019 | `SEMANTIC`; executable schedule participates in deterministic meaning. |
| proof identity | explain crate, ADR 0011 | `SEMANTIC`; trace identity binds exact namespace/cut/schema/program/policy/import material. |
| coordination lease/fence meaning | `SPEC.md` §10/10.1, pilot/tests | `SEMANTIC`; stale holder fencing is authority, not resource reservation. |

ADR 0010 is especially dispositive: hidden facts can change negation,
aggregation, recursion and tuple allocation, so policy filtering after the fact
is not authorization-safe. A transport/fabric layer therefore cannot own or
reconstruct policy-scoped semantics.

ADR 0012 is similarly dispositive: the journal is described as immutable
semantic authority; followers receive leader-admitted revisions/datoms/receipts
and verify identity rather than make independent semantic decisions.

## Partition / replication evidence

| Family | Evidence | E0 conclusion |
| --- | --- | --- |
| partition/cut/federated identity | AST + `aether_partition` | `SEMANTIC`. |
| imported-fact source-cut provenance | partition service | `SEMANTIC`. |
| leader epoch/promotion | `LeaderEpoch`, `PromoteReplica*` | `SEMANTIC`; changes append authority. |
| stale-epoch/divergent-prefix fencing | partition implementation/docs | `SEMANTIC`. |
| follower byte/prefix movement | partition prototype | `AMBIGUOUS`; physical movement may become mechanical only beneath AETHER validation. |
| replica lag/health | `ReplicaStatus` | `FABRIC` candidate as operational evidence. |
| endpoint/database-path realization | `ReplicaConfig` | `FABRIC` candidate. |

## Sidecar / memory evidence

ADR 0007 explicitly rejects sidecars as independent truth or orchestration
planes. Identity, provenance, policy, orchestration state and replay scope stay
anchored in the kernel.

Therefore:

- semantic artifact/vector references and result re-entry are `SEMANTIC`;
- physical payload/cache/shard locality is `AMBIGUOUS` until E1/E2 proves it can
  vary without changing identity, visibility, provenance or replay.

## Service edge and resource-control evidence

### HTTP

R6/ADR 0019 makes `aether_http` an outward consumer of service contracts, but
that crate still combines:

- mechanical HTTP framing;
- principal/auth binding;
- policy narrowing;
- semantic namespace selection;
- audit content;
- worker/queue/resource mechanics.

Therefore `aether_http` cannot move wholesale.

### Resource control — Adversary correction

`docs/RESOURCE_CONTROL_CONTRACT.md` proves the resource-control surface contains
both mechanical and semantic obligations.

Mechanical candidates:

- global blocking-worker capacity;
- queue occupancy and queue wait;
- pre-start backpressure/timeout observations;
- concrete worker realization.

Semantic obligations retained in AETHER:

- one active semantic operation per namespace to preserve deterministic
  same-namespace order;
- `cancel_before_start_complete_after_start`;
- once synchronous semantic work starts, the service must not return timeout
  while a mutation may still commit in the background;
- resource rejection must not partially append authority, publish an AETHER
  execution receipt, or retain a trace handle;
- semantic runtime/rule/tuple limits are checked before affected semantic
  metadata persists.

Accordingly, queueing/backpressure is **not** classified wholesale as FABRIC.
The E0 matrix splits mechanical capacity from semantic serialization,
cancellation and atomicity. AF-A07/D2/D5 record the discriminating tests.

### Audit

- audit event semantic content is `SEMANTIC`;
- audit-event buffering/delivery may be mechanical;
- backpressure cannot silently erase audit evidence required for accountability.

### Health/capacity

Service health, replica lag and host/capacity facts are operational evidence and
may be `FABRIC` candidates. They create no permission or institutional
allocation by themselves.

## GHOS / execution evidence

`.ghos-routing/workflows.json` records `GITHUB_ACTIONS` as the
`PERSISTENT_CONTROLLER` for the governed AETHER workflows. The registry's claim
boundaries explicitly set `constitutional`, `merge`, `certification`,
`production`, `publication`, `claim_promotion` and related flags to `false`.

E0 therefore classifies workflow execution/controller routing as `EXECUTION`,
not FABRIC. A future FABRIC may transport payloads only under a separately
reviewed bridge; endpoint delivery cannot mint GHOS controller authority.

## POL lineage

PR #10 (`Introduce AETHER-POL semantic layer`) is not on current protected
`main`. It proposed typed polity/guild/contract/work/claim/evidence/decision
objects projected into AETHER facts and explicitly excluded scheduler, workflow
engine, queue, graph runner, model serving and autonomous runtime.

E0 retains this as `INSTITUTIONAL` lineage only. It is not current authority or
implementation.

## AETHER-Learn evidence status

Search of current protected `main` under `AETHER-Learn`, regret-router,
route-proposal and allocation vocabulary found no live implementation. This is
a bounded negative search finding, not a claim about every historical GCL
artifact.

Desired allocation is therefore recorded as a future institutional/learning
contract, not inferred from HTTP namespace routing, resource scheduling,
partition replication or capacity tooling.

## Material responsibility-family coverage

E0 covers:

1. semantic representation/DSL;
2. schema/admission;
3. journal/persistence;
4. replay/rules/closure;
5. policy/provenance/proof identity;
6. semantic coordination lease/fence state;
7. sidecar memory/reference boundaries;
8. partition/federation/replication;
9. HTTP/auth/namespace/audit;
10. worker/queue/resource-control semantics and mechanics;
11. health/performance/capacity telemetry;
12. compatibility/client/tooling surfaces;
13. GHOS/workflow execution integration;
14. POL and AETHER-Learn lineage/boundaries.

The packet does not claim every implementation function has a permanent owner.
Every mixed family has an ambiguity entry and discriminating E1/E2 evidence
requirement.

## E0 evidence sufficiency rule

A classification is E0-sufficient when the material responsibility is named,
its current evidence surface is identified, authority implications are explicit,
mixed responsibility is not hidden by a crate-level label, and unresolved
extraction questions have a discriminating test.

E0 does not execute the future interface. Semantic-equivalence execution belongs
to E2. E0's job is to make premature extraction demonstrably unjustified.