# AETHER/FABRIC E0 Boundary Matrix

Status: E0 inventory candidate, revised after Adversary pass
Issue: `AETHER-FABRIC-ENCAP-E0-001` / #83
AETHER basis: protected `main` `159cf930ae9130060f16d9fbc2608ad8a4069fae`
Council basis: INTELLECT `18df1c3ef89712fe00af37a484cc03ce270e99bd`

## Purpose

Classify material live AETHER responsibility families before any code movement.
Classification is by responsibility, not crate. Existing crates may contain
responsibilities from several planes. `AMBIGUOUS` is therefore a valid result.

## Vocabulary

- `SEMANTIC`: determines admitted AETHER state, identity, policy visibility,
  replay, derivation, provenance, proof identity, semantic ordering/atomicity,
  semantic fencing, or semantic acceptance/rejection.
- `INSTITUTIONAL`: polity/application meaning above the AETHER kernel, including
  work objects, guild/office semantics, governed allocation, and decisions.
- `EXECUTION`: protected-controller/external-execution responsibility belonging
  to GHOS/operator governance rather than semantic admission or transport.
- `FABRIC`: non-authoritative physical realization: endpoint/resource
  observation, movement, placement, delivery, queue capacity, backpressure,
  replica movement, locality, or equivalent mechanics **inside an already
  authorized envelope**.
- `AMBIGUOUS`: mixed responsibility requiring a narrower contract and a
  discriminating test before permanent ownership can be assigned.

## Non-negotiable rules

1. Delivery does not imply semantic admission.
2. Liveness does not imply institutional authority.
3. Resource capability does not imply permission.
4. Physical replication does not imply semantic authority replication.
5. Missing transport is not negative evidence.
6. AETHER semantic correctness must remain possible without FABRIC.
7. FABRIC mechanics must remain useful without importing AETHER semantics.
8. Mechanical timeout/backpressure cannot declare a semantic operation safely
   cancelled after semantic execution has begun.
9. GHOS controller admission and INTELLECT constitutional authority remain
   outside AETHER/FABRIC implementation convenience.

## Matrix

| # | Responsibility | Current owner / evidence | Class | Why / authority consequence | Candidate future owner | E0 unresolved question |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | Canonical entity/value/query/rule/provenance types | `aether_ast` | `SEMANTIC` | Defines objects whose meaning is replayed/derived. | AETHER | — |
| 2 | Canonical DSL semantics | `SPEC.md`, `aether_ast`, `aether_rules` | `SEMANTIC` | Canonical expression surface for facts, rules, time, policy. | AETHER | POL may project facts but not redefine kernel semantics. |
| 3 | Schema registry/type contracts | `aether_schema` | `SEMANTIC` | Defines valid semantic inputs. | AETHER | — |
| 4 | Namespace schema activation | ADR 0012, service admission | `SEMANTIC` | Certifies journal prefix and active semantic schema. | AETHER | — |
| 5 | Append admission validation | ADR 0012, `aether_service_core::admission` | `SEMANTIC` | Validation precedes authoritative commit. | AETHER | — |
| 6 | Append order / element identity | `aether_storage`, AST element IDs | `SEMANTIC` | Accepted journal history is AETHER authority. | AETHER | Backend I/O split separately below. |
| 7 | Expected-cut/idempotent atomic append | ADR 0012, storage/service | `SEMANTIC` | Atomic mutation and receipt identity determine accepted history. | AETHER | — |
| 8 | Local durable backend I/O | SQLite/Postgres storage implementations | `AMBIGUOUS` | I/O is mechanical; exact transaction/cut/receipt semantics are semantic. | AETHER storage adapter or future narrow substrate | Can backend substitution preserve identical accepted cuts/receipts without policy interpretation? |
| 9 | Current/`AsOf` replay | `aether_resolver` | `SEMANTIC` | Deterministic temporal meaning. | AETHER | — |
| 10 | CRDT/cardinality merge semantics | resolver/schema | `SEMANTIC` | Merge rules determine resolved truth. | AETHER | — |
| 11 | Policy-closed dependency certification | resolver, ADR 0010 | `SEMANTIC` | Hidden dependencies cannot alter visible truth. | AETHER | — |
| 12 | Rule parse/safety/type/stratification | `aether_rules` | `SEMANTIC` | Defines admissible recursive semantics. | AETHER | — |
| 13 | SCC/stratum executable semantic plan | `aether_plan`, ADR 0019 | `SEMANTIC` | Schedule is part of deterministic fixed-point meaning. | AETHER | — |
| 14 | Semi-naive closure / derived sets | `aether_runtime` | `SEMANTIC` | Produces derived semantic state. | AETHER | — |
| 15 | Proof/derivation explanation | `aether_explain` | `SEMANTIC` | Explains why a fact/tuple exists. | AETHER | Renderer transport may remain outside. |
| 16 | Policy scope before replay/compile | ADR 0010 | `SEMANTIC` | Policy is semantic input, not presentation metadata. | AETHER | — |
| 17 | AETHER execution IDs / trace handles | ADR 0011 | `SEMANTIC` | Bind proof to namespace/cut/schema/program/policy/imports. | AETHER | Must remain distinct from GHOS process-execution receipts. |
| 18 | Semantic evaluation orchestration | `aether_service_core` | `SEMANTIC` | Orders semantic admission/evaluation; not general job scheduling. | AETHER | Must not absorb endpoint placement. |
| 19 | Tasks/claims/lease/heartbeat/fence/outcome facts | `SPEC.md` §10, pilot | `SEMANTIC` | Current coordination meaning is represented as facts and rules. | AETHER | Physical dispatch remains separable. |
| 20 | Lease epoch / stale-holder fenced commit | `SPEC.md` §10.1 | `SEMANTIC` | Authority rule, not resource reservation. | AETHER | Future FABRIC lease must use distinct type/name. |
| 21 | Coordination application DSL/reports/deltas | `aether_pilot` | `INSTITUTIONAL` | Application/proof layer above service semantics. | POL/application layer | Live main has no admitted POL crate. |
| 22 | AETHER-POL typed institutional model | preserved PR #10 only | `INSTITUTIONAL` | Historical proposal for polity/guild/work/claim/evidence/decision facts. | AETHER-POL above AETHER | Requires fresh admission; stale PR is lineage only. |
| 23 | AETHER-Learn desired allocation | no live implementation found on current main | `INSTITUTIONAL` | Choosing who ought to do work is upstream institutional allocation. | AETHER-Learn/POL | Exact live owner/contract remains open. |
| 24 | Artifact/vector semantic identity/provenance/policy | sidecar, ADR 0007 | `SEMANTIC` | Sidecars may not become independent truth sources. | AETHER | — |
| 25 | Physical artifact/vector payload locality/catalog mechanics | `aether_sidecar` | `AMBIGUOUS` | Locality/movement is mechanical but current catalog is journal-subordinated. | possible FABRIC/storage sidecar | Can physical movement vary without changing identity/visibility/provenance/replay? |
| 26 | Sidecar/vector result semantic re-entry | ADR 0007 | `SEMANTIC` | Results gain meaning only with provenance/admission. | AETHER | — |
| 27 | Partition identity / explicit cuts | AST + `aether_partition` | `SEMANTIC` | Defines which authority prefixes participate in meaning. | AETHER | — |
| 28 | Federated import identity/provenance | `aether_partition` | `SEMANTIC` | Source partition/cut remains part of fact identity. | AETHER | Payload transfer may later be mechanical. |
| 29 | Federated semantic execution/explain | `aether_partition` | `SEMANTIC` | Derived meaning over explicit cuts; no fake global clock. | AETHER | — |
| 30 | Leader epoch / semantic authority promotion | `aether_partition` | `SEMANTIC` | Promotion changes append authority. | AETHER | Automatic authority election would need separate governance. |
| 31 | Stale-epoch/divergent-prefix fencing | partition prototype | `SEMANTIC` | Prevents live/copy-complete replica from becoming semantic authority. | AETHER | — |
| 32 | Follower byte/prefix movement | partition prototype | `AMBIGUOUS` | Movement is FABRIC-shaped but intertwined with prefix/epoch validation. | likely future FABRIC adapter | Can copying remain opaque until AETHER validates prefix/epoch? |
| 33 | Replica lag/health observation | `ReplicaStatus` | `FABRIC` | Operational observation only. | FABRIC | Must re-enter AETHER as evidence before governed allocation. |
| 34 | Replica endpoint/database-path realization | `ReplicaConfig` | `FABRIC` | Concrete location/topology is physical realization. | FABRIC/deployment config | Need stable non-semantic endpoint identity. |
| 35 | HTTP/network framing and byte transport | `aether_http` | `AMBIGUOUS` | Transport is mechanical; crate also owns auth/policy/audit/namespace semantics. | AETHER gateway + optional transport adapter | Which functions are generic transport versus AETHER protocol? |
| 36 | Principal authentication binding | HTTP/service auth | `SEMANTIC` | Principal establishes maximum policy visibility. | AETHER service edge | Credential issuance remains separately governed. |
| 37 | Request policy narrowing | ADR 0010 / service path | `SEMANTIC` | Determines effective semantic snapshot. | AETHER | — |
| 38 | Semantic namespace resolution | HTTP/service path | `SEMANTIC` | Chooses the namespace/policy domain in which meaning is evaluated. | AETHER | Mechanical endpoint selection must occur after this step. |
| 39 | Concrete worker selection / global worker-pool capacity | resource-control service | `FABRIC` | Mechanical realization under fixed semantic namespace/envelope. | FABRIC candidate | Must not alter actor eligibility or institutional priority. |
| 40 | Queue occupancy, queue wait, pre-start timeout, backpressure observations | resource-control service | `FABRIC` | Mechanical capacity state **only before semantic work starts**. | FABRIC candidate | D2 must cover starvation, fairness, head-of-line blocking, timeout. |
| 41 | Same-namespace semantic serialization/order | `RESOURCE_CONTROL_CONTRACT.md` | `SEMANTIC` | One active semantic operation preserves deterministic same-namespace order. | AETHER | Future scheduler may realize work but cannot weaken this contract. |
| 42 | `cancel_before_start_complete_after_start` semantic lifecycle | `RESOURCE_CONTROL_CONTRACT.md` | `SEMANTIC` | Prevents reported timeout/failure while a started authority mutation may still commit. | AETHER | FABRIC timeout cannot override started semantic execution. |
| 43 | No-partial-authority/no-receipt-on-resource rejection | resource-control contract | `SEMANTIC` | Resource failure must not partially append authority or publish semantic receipts. | AETHER | Mechanical admission must preserve this postcondition. |
| 44 | Request/document/rule/runtime/result limits | resource-control contract | `AMBIGUOUS` | Some limits protect semantic boundedness/atomicity; others only resource use. | split AETHER/FABRIC | Classify each limit by semantic effect, not implementation location. |
| 45 | Audit record semantic content | HTTP audit | `SEMANTIC` | Preserves principal/namespace/cut/query/provenance lineage. | AETHER | Physical sink may be external. |
| 46 | Audit sink queue/buffering/delivery | resource-control/audit path | `FABRIC` | Mechanical event movement after audit event is formed. | FABRIC/logging substrate | Backpressure must fail visibly; required evidence cannot vanish silently. |
| 47 | Service/process health observation | HTTP/status and partition status | `FABRIC` | Liveness is operational evidence only. | FABRIC/service ops | AETHER may admit observations with provenance. |
| 48 | Capacity curves/node classes/host facts | `aether_perf`, capacity planning | `FABRIC` | Resource/capacity observations are mechanical evidence, not permission. | FABRIC/ops telemetry | Upstream allocator may consume only through admitted evidence. |
| 49 | Cross-plane benchmarks/drift | `aether_perf` | `AMBIGUOUS` | Measures semantic runtime and mechanical service behavior. | leaf measurement tooling | Split metric ownership once E1 interface exists. |
| 50 | Compatibility re-export facade | `aether_api` | `AMBIGUOUS` | Temporary R6 migration surface, not architectural owner. | shrink/deprecate per ADR 0019 | Must not become normative AETHER/FABRIC protocol. |
| 51 | Release-readiness/supply-chain/promotion execution | workflows/evidence ledger | `EXECUTION` | Protected execution/evidence governance, not semantic transport. | GHOS/release governance | FABRIC may later move bytes but cannot execute reserved gate authority. |
| 52 | `.ghos-routing` workflow/controller registry | `.ghos-routing/workflows.json` | `EXECUTION` | Binds final workflows to `GITHUB_ACTIONS` persistent controller. | GHOS | AETHER/FABRIC cannot infer controller admission. |
| 53 | Go admin/deployment shell | `go/` | `EXECUTION` | Non-authoritative client/operator tooling. | operator/GHOS tooling | Generic discovery may later consume FABRIC. |
| 54 | Python SDK/research harness | `python/` | `EXECUTION` | Non-authoritative client/harness surface. | client/tooling | Not an extraction target merely because external to kernel. |
| 55 | Commercial/demo/reference-app packaging | docs/examples/app packs | `INSTITUTIONAL` | Workflow/product semantics above kernel. | application layer | Demos are not normative architecture evidence without contracts/tests. |

## Initial E0 disposition

### Firmly retained in AETHER

- semantic types/DSL/schema;
- append admission/order/receipts;
- replay/merge/closure/proof;
- policy-scoped semantics;
- semantic coordination lease/fence meaning;
- semantic resource-order/atomicity guarantees;
- partition/cut/import/federated semantics;
- leader epoch, semantic promotion, stale/divergent fencing;
- sidecar identity/provenance/policy;
- semantic audit content.

### Clear mechanical FABRIC candidates

Subject to an authorized mechanical envelope and hostile influence tests:

- endpoint/resource health and location;
- replica lag observation;
- worker-pool capacity and concrete worker realization;
- queue occupancy/wait/backpressure **before semantic execution begins**;
- audit-event transport;
- host/capacity observations.

No item moves during E0.

### Mixed seams requiring E1 decomposition

- durable storage I/O versus authoritative journal semantics;
- sidecar payload locality versus semantic references;
- replica movement versus AETHER epoch/prefix fencing;
- HTTP transport versus semantic auth/policy/protocol;
- mechanical queue realization versus semantic serialization/cancellation;
- semantic safety limits versus operational limits;
- compatibility facade ownership;
- cross-plane telemetry/performance.

## Historical findings

1. PR #10 (`Introduce AETHER-POL semantic layer`) is not on live `main`. Its
   non-goals excluded scheduler, workflow engine, queue, graph runner,
   model-serving, and autonomous runtime. Preserve it as lineage only.
2. No live AETHER-Learn implementation was found under the E0 searched
   router/allocation vocabulary. Desired allocation therefore remains a future
   governed component, not existing namespace or partition routing.
3. Historical lowercase “AETHER fabric” language remains valid for the
   pre-separation architecture. After an effective split, uppercase `FABRIC`
   should identify only the independently encapsulated mechanical component.

## E0 conclusion

The separation seam is real but cross-cuts current crate boundaries.

In particular, `aether_partition`, `aether_http`, storage, sidecar, and resource
control each contain responsibilities that must be decomposed before extraction.
The Adversary review further establishes that queueing is not uniformly
mechanical: capacity/backpressure may be FABRIC-shaped, while same-namespace
semantic ordering, started-operation completion, and no-partial-authority
postconditions remain AETHER contracts.

E0 therefore recommends **decomposition-first E1 contracts**, not crate or
repository movement.