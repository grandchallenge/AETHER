# AETHER/FABRIC E0 Evidence Index

Status: E0 evidence map
Issue: #83
AETHER protected-main basis: `159cf930ae9130060f16d9fbc2608ad8a4069fae`
INTELLECT Council basis: `18df1c3ef89712fe00af37a484cc03ce270e99bd`

## Purpose

Bind the E0 responsibility classifications to inspectable live repository
surfaces. This index does not assert that every file in AETHER has been read
line-by-line. E0 claims coverage of **material responsibility families**, with
uncertain seams retained as `AMBIGUOUS` rather than inferred from naming.

## Governing evidence

| Evidence | Relevance to E0 |
| --- | --- |
| INTELLECT `governance/council_matters/GI-COUNCIL-AETHER-FABRIC-ENCAP-001/` at merge `18df1c3e...` | Governing architecture recommendation, conditions, influence-without-authority doctrine, E0-before-extraction requirement. |
| AETHER `AGENTS.md` | Rust semantic kernel is authoritative; service/API concerns must not dictate rule-engine semantics; production-semantic activation and permission escalation remain reserved. |
| AETHER `SPEC.md` | Defines authoritative semantic substrate, recursive closure, provenance, temporal replay, coordination facts, lease fencing, and sidecar subordination. |
| AETHER `Cargo.toml` | Enumerates the current Rust workspace responsibility crates. |
| `docs/ARCHITECTURE_BOUNDARIES.md` | Current R6 crate ownership and dependency contract. |
| `docs/STATUS.md` | Current controlled-single-node claim, distributed-truth prototype, resource controls, release/governance boundaries, and implemented responsibility extraction. |

## Semantic-kernel evidence

| Responsibility family | Evidence | E0 implication |
| --- | --- | --- |
| canonical semantic types / DSL | `AGENTS.md`, `SPEC.md`, `crates/aether_ast`, `crates/aether_rules` | `SEMANTIC`; cannot be a FABRIC concern. |
| schema/type contracts | `crates/aether_schema`, `SPEC.md` | `SEMANTIC`. |
| authoritative journal / append order | `SPEC.md`, `crates/aether_storage`, ADR 0012 | Journal history is semantic authority; storage implementation mechanics remain separately ambiguous. |
| replay / merge / dependency certification | `crates/aether_resolver`, ADR 0010 | `SEMANTIC`; policy projection precedes replay. |
| rule planning and closure | `crates/aether_rules`, `aether_plan`, `aether_runtime` | `SEMANTIC`; deterministic derived meaning. |
| proof / explanation | `crates/aether_explain`, ADR 0011 | `SEMANTIC`; proof identity binds exact semantic inputs. |
| policy visibility | ADR 0010 | Policy is semantic input, not presentation or transport metadata. |
| append admission | ADR 0012, `aether_service_core::admission` | `SEMANTIC`; validation occurs before authoritative commit. |
| AETHER evaluation receipt / trace identity | ADR 0011 | `SEMANTIC`; distinct from external process/controller execution receipt. |

## Coordination and authority evidence

| Responsibility family | Evidence | E0 implication |
| --- | --- | --- |
| tasks / claims / leases / heartbeats / expiries / fences / outcomes | `SPEC.md` §10, `aether_pilot`, demos and test plan | Their **meaning** is `SEMANTIC`, even if later physical dispatch is mechanical. |
| stale lease holder fencing | `SPEC.md` §10.1 | `SEMANTIC`; future operational resource leases require distinct types. |
| authority partitions / federated cuts | `aether_partition`, `docs/ARCHITECTURE.md`, `docs/KNOWN_LIMITATIONS.md` | Partition/cut/authority semantics remain in AETHER. |
| leader epoch / authority promotion | `aether_partition::{LeaderEpoch, PromoteReplica*}` | `SEMANTIC`; promotion changes append authority. |
| stale-epoch / divergent-prefix rejection | `aether_partition`, distributed-truth docs/status | `SEMANTIC`; physical replica health cannot override fencing. |
| replica lag / health | `aether_partition::ReplicaStatus` | `FABRIC` candidate as operational evidence. |
| replica endpoint/database path | `aether_partition::ReplicaConfig` | `FABRIC` candidate as physical realization. |
| follower data movement | `aether_partition` | `AMBIGUOUS`; mechanics are extraction-shaped but entangled with semantic prefix/epoch verification. |

## Sidecar / memory evidence

| Responsibility family | Evidence | E0 implication |
| --- | --- | --- |
| semantic artifact/vector identity, provenance, policy, orchestration reference | ADR 0007, `SPEC.md` §11, `aether_sidecar` | `SEMANTIC`; sidecars are explicitly not a second authority plane. |
| physical payload/catalog locality and movement | `aether_sidecar`, ADR 0007 consequences | `AMBIGUOUS`; likely mechanical only after identity/policy/replay contract is isolated. |
| projection of sidecar results into rules | ADR 0007 | `SEMANTIC`; must carry provenance and remain replay-compatible. |

## Service-edge and resource evidence

| Responsibility family | Evidence | E0 implication |
| --- | --- | --- |
| HTTP framing / network delivery | `aether_http`, R6 boundary docs | `AMBIGUOUS`; transport is mechanical but crate also owns AETHER-specific auth/audit/namespace semantics. |
| authentication and policy ceiling | `aether_http`, ADR 0010, status/remediation docs | `SEMANTIC` at AETHER service edge; credential issuance remains external governance. |
| semantic namespace resolution | `aether_http`, service-core contracts | `SEMANTIC` side of a mixed route. |
| concrete worker/queue realization | resource-control paths, status/remediation docs | `FABRIC` candidate when bounded by an already authorized namespace/mechanical envelope. |
| queueing/backpressure | resource-control contract/status | `FABRIC` candidate subject to starvation/priority hostile tests. |
| size/runtime/result limits | resource-control contract/status | `AMBIGUOUS`; must be decomposed into semantic-safety versus operational-resource limits. |
| audit event meaning/content | HTTP audit path | `SEMANTIC`; preserves principal/cut/query/provenance lineage. |
| audit sink buffering/delivery | resource-control/audit implementation | `FABRIC` candidate, but required evidence may not be silently dropped. |
| health/status | HTTP/partition status surfaces | `FABRIC` candidate as operational evidence only. |

## Execution/GHOS evidence

| Responsibility family | Evidence | E0 implication |
| --- | --- | --- |
| persistent workflow execution | `.ghos-routing/workflows.json` | `EXECUTION`; all listed final workflows are bound to the admitted `GITHUB_ACTIONS` persistent controller. |
| controller admission | `.ghos-routing/workflows.json`, AETHER `AGENTS.md` | Neither AETHER nor future FABRIC gains controller authority by implementation or transport. |
| release qualification / supply-chain / readiness execution | `.github/workflows/*`, `.ghos-routing/workflows.json`, status | `EXECUTION`; may produce evidence but does not become semantic or FABRIC authority. |
| Go/Python clients/tooling | `go/`, `python/`, `AGENTS.md` | non-authoritative client/execution/tooling surfaces, not extraction targets merely because they are outside the Rust kernel. |

The routing registry specifically records `GITHUB_ACTIONS` as the
`PERSISTENT_CONTROLLER` for the governed workflows and explicitly sets all
listed claim-boundary flags, including `constitutional`, `merge`,
`certification`, `production`, and `claim_promotion`, to `false`.

## POL lineage evidence

### Preserved PR #10

PR #10, `Introduce AETHER-POL semantic layer`, is not present on live protected
`main`. Its proposal described typed `Polity`, `Guild`, `AgentContract`,
`WorkObject`, `Claim`, `EvidenceBundle`, `Critique`, `Verification`, `Decision`,
`RouteProposal`, `RouteDecision`, and `RouterUpdate` objects projected into
AETHER facts.

Its explicit non-goals included:

- autonomous agent runtime;
- routing logic;
- scheduler;
- workflow engine;
- queue;
- graph runner;
- model-serving layer.

E0 therefore treats this PR as **institutional lineage**, not current code or
current authority.

## AETHER-Learn evidence status

E0 repository search on current protected `main` found no live implementation
under the searched `AETHER-Learn`, regret-router, route-proposal, or allocation
vocabulary. This is a negative search finding, not proof that no related
experimental artifact exists anywhere in GCL history.

E0 therefore records desired allocation as a future `INSTITUTIONAL`/learning
contract whose exact owner, implementation, and admission route remain open.
It is not inferred from `aether_http` namespace routing, partition replication,
or capacity tooling.

## Material-coverage statement

The boundary matrix covers the following material live responsibility families:

1. canonical semantic representation;
2. schema and admission;
3. journal/persistence;
4. replay and closure;
5. policy and provenance;
6. proof identity;
7. coordination facts and lease authority;
8. sidecars/artifact-vector memory;
9. partitions/federation/replication;
10. HTTP/auth/audit/namespace routing;
11. resource controls/backpressure;
12. observability/performance/capacity;
13. client/tooling layers;
14. GHOS/workflow execution integration;
15. POL/AETHER-Learn lineage and absent-live-main boundaries.

The matrix does **not** claim every implementation function has already been
assigned a permanent plane. Mixed families remain `AMBIGUOUS` precisely to
prevent that overclaim.

## Evidence standard for E0 closure

A classification is sufficient for E0 when:

- the material responsibility is represented;
- the current owning surface is identified;
- its authority implications are explicit;
- mixed responsibility is not hidden by a crate-level label;
- unresolved extraction questions have a discriminating E1/E2 test.

E0 does not require executing the future interface or proving semantic
equivalence; those belong to E1/E2. E0 requires enough evidence to make
premature extraction demonstrably impermissible.
