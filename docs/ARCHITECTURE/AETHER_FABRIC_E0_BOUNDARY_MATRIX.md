# AETHER/FABRIC E0 Boundary Matrix

Status: E0 inventory candidate
Issue: `AETHER-FABRIC-ENCAP-E0-001` / #83
AETHER basis: protected `main` `159cf930ae9130060f16d9fbc2608ad8a4069fae`
Council basis: INTELLECT `18df1c3ef89712fe00af37a484cc03ce270e99bd`

## Purpose

This document classifies the current AETHER estate before any code movement.
It implements the first condition of `GI-COUNCIL-AETHER-FABRIC-ENCAP-001`:
prove the responsibility seam before extracting machinery.

The classifications are deliberately about **responsibilities**, not crates.
One crate may currently contain responsibilities that belong to different
planes. `AMBIGUOUS` is therefore a valid and expected E0 result.

Classification vocabulary:

- `SEMANTIC`: authority over admitted AETHER state, replay, derivation,
  provenance, policy visibility, semantic fencing, or semantic identity.
- `INSTITUTIONAL`: polity/application semantics such as offices, guilds,
  work objects, governed allocation, or decision structures above AETHER.
- `EXECUTION`: persistent-controller/external execution responsibility or
  integration metadata that belongs to GHOS/executor governance rather than
  semantic admission or transport.
- `FABRIC`: mechanically distributed movement, placement, rendezvous,
  buffering, backpressure, replication movement, endpoint/resource liveness,
  or equivalent non-authoritative realization.
- `AMBIGUOUS`: a mixed or insufficiently evidenced seam that must remain
  unresolved until a discriminating test or decomposition exists.

## Non-negotiable interpretation rules

1. Delivery does not imply semantic admission.
2. Liveness does not imply institutional authority.
3. Resource capability does not imply permission.
4. Physical replication does not imply semantic authority replication.
5. Missing transport is not negative evidence.
6. AETHER semantic correctness must remain possible without FABRIC.
7. FABRIC mechanics must remain useful without importing AETHER semantics.
8. GHOS controller admission and INTELLECT constitutional authority remain
   outside both AETHER and FABRIC implementation convenience.

## Matrix

| # | Responsibility | Current owner / evidence surface | Class | Semantic/control dependencies | E0 rationale | Candidate future owner | Open question |
| ---: | --- | --- | --- | --- | --- | --- | --- |
| 1 | Canonical entity/value/query/rule/provenance types | `crates/aether_ast` | `SEMANTIC` | foundational libraries | These types define the objects whose meaning is replayed and derived. | AETHER | None material. |
| 2 | DSL semantic surface | `SPEC.md`, `crates/aether_ast`, `crates/aether_rules` | `SEMANTIC` | schema, rule compiler | The DSL is the canonical semantics surface under `AGENTS.md` and `SPEC.md`. | AETHER | POL vocabulary may later project into it but must not redefine kernel semantics. |
| 3 | Schema registry and type contracts | `crates/aether_schema` | `SEMANTIC` | AST | Schema admission determines which facts are valid semantic inputs. | AETHER | None material. |
| 4 | Namespace schema activation and append admission | `aether_service_core::admission`, ADR 0012 | `SEMANTIC` | schema, journal cut, policy/provenance | Admission is explicitly semantic authority, not transport. | AETHER | None material. |
| 5 | Append-only journal order and semantic element identity | `crates/aether_storage`, `aether_ast::ElementId` | `SEMANTIC` | AST, durable backend | Append order is constitutional AETHER authority. | AETHER | Backend transport/persistence adapters are classified separately below. |
| 6 | Transactional expected-cut/idempotent append contract | `aether_storage`, `aether_service_core` | `SEMANTIC` | journal, schema admission | Atomic semantic mutation and receipt identity define accepted history. | AETHER | None material. |
| 7 | Local durable journal adapter I/O | SQLite/Postgres implementations under storage/service | `AMBIGUOUS` | semantic append contract; filesystem/database I/O | Persistence mechanics are mechanical, but correctness of the authoritative journal depends on exact transactional semantics. Extracting them as FABRIC would conflate storage durability with distributed movement unless a narrower storage-adapter contract is proven. | AETHER or separate storage adapter; not presumed FABRIC | Can a storage adapter be mechanically replaceable while preserving byte-/cut-exact semantic guarantees with no policy interpretation? |
| 8 | Current-state and `AsOf` materialization | `crates/aether_resolver` | `SEMANTIC` | journal, schema | Deterministic replay is core semantic authority. | AETHER | None material. |
| 9 | CRDT/cardinality merge semantics | `aether_resolver`, schema merge classes | `SEMANTIC` | schema, journal | Merge rules determine meaning of admitted history. | AETHER | None material. |
| 10 | Dependency certification for replay/program inputs | `aether_resolver`, remediation R1/R2 | `SEMANTIC` | policy scope, schema, rule program | Determines exact semantic dependency closure. | AETHER | None material. |
| 11 | Rule parsing, safety, type validation, stratification | `crates/aether_rules` | `SEMANTIC` | AST, schema, plan | Determines which recursive claims are admissible/executable. | AETHER | None material. |
| 12 | SCC/stratum executable plan | `crates/aether_plan`, `aether-executable-plan-v1` | `SEMANTIC` | AST | Plan ordering is part of deterministic semantic closure. | AETHER | None material. |
| 13 | Semi-naive recursive execution and derived-set maintenance | `crates/aether_runtime` | `SEMANTIC` | resolver, executable plan | Produces deterministic derived truth from admitted inputs. | AETHER | None material. |
| 14 | Derivation/proof/plan explanation | `crates/aether_explain` | `SEMANTIC` | plan, runtime, provenance | Proof traces explain why an admitted/derived fact is present. | AETHER | Rendering transport may remain at service edge. |
| 15 | Policy scope as pre-replay/pre-compilation semantic input | ADR 0010, `aether_service_core`, resolver/runtime | `SEMANTIC` | auth principal, policy context | Visibility changes the semantic snapshot before evaluation. | AETHER | Identity authentication mechanism itself is separately classified. |
| 16 | Execution receipt identity and trace-handle semantics | ADR 0011, `aether_service_core::execution` | `SEMANTIC` | namespace, cut, policy, program, provenance | Receipts bind an AETHER evaluation to exact semantic inputs; they are not generic process-execution receipts. | AETHER | External GHOS execution receipts must remain a separate type/domain. |
| 17 | AETHER evaluation orchestration | `aether_service_core` | `SEMANTIC` | admission, resolver, rules, runtime, explain, sidecars | Orchestration orders semantic preparation/evaluation and is not general job scheduling. | AETHER | Must not absorb future endpoint placement merely for convenience. |
| 18 | Tasks, claims, lease facts, heartbeats, expiries, fences, outcomes as semantic facts | `SPEC.md` §10, `aether_pilot`, coordination demos | `SEMANTIC` | journal, rules, policy | The live implementation represents coordination state as facts and derives readiness/authority from them. Their meaning remains semantic even if later physical dispatch uses FABRIC. | AETHER | Physical worker dispatch and resource reservation are not yet separated in live main. |
| 19 | Lease epoch and stale-holder semantic fencing | `SPEC.md` §10.1, pilot/service tests | `SEMANTIC` | claim/lease facts, accepted append | A stale holder being unable to commit a fenced action is an authority rule, not a transport lease. | AETHER | A future FABRIC operational lease must use a distinct type/name. |
| 20 | Coordination application DSL, reports, deltas, proof fixtures | `crates/aether_pilot` | `INSTITUTIONAL` | service-core semantic APIs | This is an application/proof layer expressing work coordination above the kernel. | POL / application layer over AETHER | Live main predates an admitted `aether_pol` crate; migration target must remain evidence-driven. |
| 21 | AETHER-POL typed institutional layer | preserved PR #10 only; not in live `main` | `INSTITUTIONAL` | would project typed POL facts into AETHER | PR #10 explicitly proposed polity/guild/contract/work/claim/evidence/decision objects without scheduler/runtime semantics. It is historical lineage, not current authority. | AETHER-POL above AETHER | Reintroduce only through a fresh reviewed candidate compatible with the Council architecture. |
| 22 | AETHER-Learn desired allocation / no-regret router semantics | no live implementation found on AETHER `main` | `INSTITUTIONAL` | future POL work object/capability/evidence inputs; AETHER outcome ledger | Desired institutional allocation belongs upstream of physical realization. Current live-main absence must not be papered over by assigning current routing code to it. | AETHER-Learn/POL | Exact repo/module and contract are not yet live; requires separate admission. |
| 23 | Artifact/vector semantic identity, provenance, policy, and references | `crates/aether_sidecar`, ADR 0007, `SPEC.md` §11 | `SEMANTIC` | AST, journal references, policy | Sidecars are explicitly subordinate; AETHER is source of truth for identity/provenance/policy. | AETHER | None material. |
| 24 | Local artifact/vector catalog bytes, cache locality, vector payload placement | `crates/aether_sidecar` | `AMBIGUOUS` | semantic reference/policy; local SQLite/catalog mechanics | Physical location and payload movement are mechanical candidates, but current sidecar contracts are local and tightly journal-subordinated. | possible FABRIC/storage sidecar | Can payload locality/movement be separated without allowing retrieval rank/availability to become semantic judgment? |
| 25 | Vector search result admission as provenance-bearing facts | sidecar-to-semantic return path | `SEMANTIC` | sidecar result, provenance, policy, schema | Search output acquires meaning only when represented/admitted through AETHER. | AETHER | Physical nearest-neighbor execution may later live elsewhere. |
| 26 | Partition identity and explicit partition cuts | `aether_ast::{PartitionId, PartitionCut, FederatedCut}`, `aether_partition` | `SEMANTIC` | journal/cut semantics | Partition/cut identity defines which committed prefixes participate in meaning. | AETHER | None material. |
| 27 | Federated import semantics and source-cut provenance | `aether_partition::import_partition_facts`, federation docs | `SEMANTIC` | explicit source cuts, policy scope, provenance | Imported facts must carry source partition/cut identity and cannot be reduced to message delivery. | AETHER | Physical transfer of import payloads may later use FABRIC. |
| 28 | Federated semantic execution/explain over explicit cuts | `aether_partition` | `SEMANTIC` | imported facts, runtime, execution store | Determines derived meaning over multiple authoritative partitions without inventing a global clock. | AETHER | None material. |
| 29 | Authority-partition leader epoch and manual authority promotion | `aether_partition::{LeaderEpoch, PromoteReplica*}` | `SEMANTIC` | partition metadata, accepted append authority | Promotion changes which replica may authoritatively append; it is semantic/control authority, not liveness routing. | AETHER | Future automatic election would require separate semantic-authority governance. |
| 30 | Stale-epoch rejection and divergent-prefix fencing | `aether_partition` replicated authority prototype | `SEMANTIC` | leader epoch, prefix digest/cut | These mechanisms prevent mechanically live replicas from becoming semantic authority. | AETHER | None material. |
| 31 | Replica byte/data movement and follower catch-up | `aether_partition` single-host replicated prototype | `AMBIGUOUS` | semantic prefix/epoch fencing; storage copying | The movement itself is a FABRIC-shaped responsibility, but current implementation is entangled with semantic prefix validation and leadership metadata. | likely FABRIC adapter later | Can copy/apply be expressed as opaque prefix movement whose success never changes authority until AETHER validates epoch/prefix? |
| 32 | Replica lag/health observation | `ReplicaStatus::{replication_lag, healthy, detail}` | `FABRIC` | endpoint/replica process state; AETHER may later ingest telemetry | Lag/health are operational observations, not semantic authority. | FABRIC | Must be admitted into AETHER as evidence before affecting governed allocation. |
| 33 | Replica endpoint/database path configuration | `ReplicaConfig` | `FABRIC` | deployment topology | Concrete endpoint/storage location is physical realization. | FABRIC / deployment config | Current database paths are single-host; future endpoint identity needs a stable non-semantic contract. |
| 34 | Network/HTTP byte transport | `crates/aether_http` / Axum boundary | `AMBIGUOUS` | service traits, auth, audit, namespaces | HTTP is an AETHER service edge, not automatically the future internal FABRIC. Moving it wholesale would incorrectly move policy/auth semantics. | likely AETHER gateway plus optional FABRIC transport beneath/alongside | Which portions are generic transport versus AETHER-specific semantic API? |
| 35 | Bearer-token authentication and principal binding | `aether_http`, service auth | `SEMANTIC` | principal identity, maximum policy visibility | Authentication binds requests to semantic policy ceilings; delivery alone cannot confer this authority. | AETHER identity/service edge | Credential issuance remains outside the kernel and may involve external governance. |
| 36 | Request policy narrowing | HTTP/service policy path | `SEMANTIC` | authenticated principal policy ceiling | Determines admitted semantic visibility. | AETHER | None material. |
| 37 | Namespace-to-service routing | `aether_http` | `AMBIGUOUS` | namespace semantic identity, worker/queue realization | Selecting the semantic namespace is meaning-bearing; choosing a worker/queue is mechanical. Current code combines both concerns. | split AETHER namespace resolution / possible FABRIC realization | Can route resolution yield an opaque authorized mechanical envelope before worker selection? |
| 38 | Per-namespace queueing, worker bounds, queue timeouts, backpressure | resource-control service/HTTP paths | `FABRIC` | authorized namespace/resource envelope; no semantic policy choice | These are mechanical resource-control functions provided they cannot change institutional priority/eligibility. | FABRIC candidate | Must test starvation, priority inversion, selective suppression, and same-namespace ordering. |
| 39 | Request/document/rule/result size limits | resource-control contract | `AMBIGUOUS` | semantic parser/runtime safety; service resource budgets | Some limits are semantic safety invariants, others are purely operational backpressure. | split AETHER/FABRIC | Classify each limit by whether changing it changes meaning or only resource realization. |
| 40 | Audit record content for semantic requests/results | HTTP audit path | `SEMANTIC` | principal, namespace, cut, query, tuple/provenance | Audit records preserve semantic/provenance lineage. | AETHER | Physical log delivery/storage may be externalized separately. |
| 41 | Audit sink buffering/backpressure | HTTP/resource control | `FABRIC` | audit event stream | Mechanical delivery/buffering of already formed audit events is not semantic admission. | FABRIC / logging substrate | Failure must not silently erase required semantic audit evidence. |
| 42 | Service health/status endpoints | `aether_http`, partition status | `FABRIC` | process/runtime state | Liveness/status are operational observations only. | FABRIC / service ops | AETHER may admit snapshots as evidence with provenance. |
| 43 | Release qualification, immutable evidence, promotion computation | release workflows, evidence schemas, commercial ledger | `EXECUTION` | GitHub/GHOS/protected workflow authority; AETHER artifact identity where applicable | These govern release execution/evidence, not semantic truth or transport. | GHOS/release governance | Must not be moved into FABRIC because it executes protected authority gates. |
| 44 | `.ghos-routing` controller routing metadata | `.ghos-routing/` | `EXECUTION` | GHOS admission/routing | This is explicitly part of the protected execution plane. | GHOS | No AETHER/FABRIC implementation may infer controller admission from this integration. |
| 45 | Go shell/admin/deployment tooling | `go/` | `EXECUTION` | AETHER public APIs; deployment environment | Shell/tooling realizes operator actions but does not own semantics. | operator/GHOS tooling | Some generic service discovery may later use FABRIC, but authority remains external. |
| 46 | Python SDK/research harness | `python/` | `EXECUTION` | AETHER public APIs | Client/harness code invokes semantics but is non-authoritative. | client/tooling | No FABRIC extraction required merely because it is external to Rust. |
| 47 | Compatibility re-export facade | `crates/aether_api` | `AMBIGUOUS` | all public surfaces during R6 migration | It is a temporary compatibility mechanism, not a stable architectural plane. | shrink/deprecate according to existing R6 plan | Future AETHER/FABRIC interfaces must not use this facade as their normative boundary. |
| 48 | Performance benchmarks and drift measurements | `crates/aether_perf` | `AMBIGUOUS` | measured crates, host/runtime observations | Measurement spans semantic runtime and future mechanical fabric. It should remain leaf tooling, not be assigned wholesale to either plane. | cross-plane measurement tooling | Split metrics into semantic correctness/performance versus FABRIC transport/capacity once E1 exists. |
| 49 | Capacity curves, node classes, host facts | `aether_perf`, capacity planning | `FABRIC` | host/resource observations | Resource availability and capacity are mechanical evidence, not permission. | FABRIC/ops telemetry | Institutional allocation may consume this only through an admitted evidence path. |
| 50 | Commercial/demo application packaging | docs/examples/reference app packs | `INSTITUTIONAL` | pilot/service APIs | These express workflows/products above the kernel; they should not drive core AETHER/FABRIC boundaries. | application layer | Keep demos from becoming normative architecture evidence unless backed by contracts/tests. |

## Initial E0 disposition by responsibility family

### Clearly retained in AETHER

The following are not extraction candidates under the approved doctrine:

- canonical semantic types and DSL;
- schema admission;
- append order and accepted journal history;
- replay and merge semantics;
- recursive rule compilation/execution;
- provenance and proof traces;
- policy-scoped semantic visibility;
- AETHER evaluation receipts and trace identity;
- semantic coordination facts and lease/fence meaning;
- partition/cut/federation semantics;
- authority epochs, stale-epoch rejection, and divergent-prefix fencing;
- semantic sidecar identity/provenance/policy;
- semantic audit content.

### Clear FABRIC candidates

These are mechanical in the current estate, subject to the authorized-envelope
and influence-without-authority constraints:

- replica lag/health observations;
- replica endpoint/location configuration;
- bounded worker queues and backpressure;
- mechanical audit-event buffering/delivery;
- service/process health observations;
- capacity and host-resource observations.

This classification does **not** move these responsibilities in E0.

### Mixed seams requiring decomposition before any extraction

The highest-value E1 questions are concentrated in:

- durable storage adapter mechanics versus authoritative journal semantics;
- artifact/vector payload locality versus semantic sidecar contracts;
- replicated follower data movement versus AETHER prefix/epoch fencing;
- HTTP transport versus AETHER auth/policy API semantics;
- namespace semantic resolution versus worker/queue realization;
- semantic safety limits versus operational resource limits;
- the temporary `aether_api` facade;
- cross-plane performance/telemetry instrumentation.

## Historical lineage findings

1. PR #10, `Introduce AETHER-POL semantic layer`, is not present in live
   `main`. Its stated non-goals explicitly excluded scheduler, workflow engine,
   queue, graph runner, model-serving layer, and autonomous runtime. That is
   compatible with the approved separation and should be preserved as lineage,
   not resurrected by assumption.
2. No live AETHER-Learn routing implementation was found on current AETHER
   `main` under the searched router/allocation vocabulary. Future desired
   allocation therefore requires a separately admitted component/contract; it
   must not be conflated with current HTTP namespace routing or partition
   mechanics.
3. Existing documentation historically calls AETHER a semantic coordination
   fabric. Those records remain historically valid. After any effective split,
   uppercase `FABRIC` should refer only to the independently encapsulated
   mechanical component.

## E0 conclusion

The Council's proposed seam is present in the live codebase, but it does not
align one-to-one with existing crate boundaries.

The strongest extraction candidate is not `aether_partition` or `aether_http`
as a whole. It is a narrower set of mechanical responsibilities currently
embedded in those crates: endpoint/liveness state, replica movement, queueing,
backpressure, capacity observation, and physical realization.

Conversely, coordination leases, authority epochs, namespace semantic identity,
federated cuts, imported-fact provenance, admission, and policy visibility are
firmly AETHER responsibilities even when implemented near operational code.

E0 therefore recommends **decomposition-first E1 contracts**, not crate or
repository movement.