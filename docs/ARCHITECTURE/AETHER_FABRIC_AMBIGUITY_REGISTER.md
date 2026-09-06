# AETHER/FABRIC E0 Ambiguity Register

Status: E0 candidate
Issue: #83

This register contains only seams that are not yet justified as clean AETHER or
FABRIC responsibilities. An ambiguity is not a defect. It is a prohibition on
premature extraction.

| ID | Ambiguous seam | Why unresolved | Discriminating evidence / test | Failure if misclassified | Earliest decision stage |
| --- | --- | --- | --- | --- | --- |
| AF-A01 | Durable journal backend mechanics versus authoritative journal semantics | SQLite/Postgres/filesystem work is mechanical, but transactional append, expected-cut identity, durability, and receipt semantics are part of AETHER truth. | Implement or model a backend adapter whose failure/retry/restart behavior is opaque to semantic replay; prove identical accepted cuts/receipts under backend substitution. | Moving the backend wholesale into FABRIC could let transport/storage success redefine accepted history. | E1/E2 |
| AF-A02 | Sidecar payload storage/locality versus semantic sidecar contract | Artifact/vector bytes are not inline semantic state, but current sidecar catalog, provenance, policy, and retrieval return path are intentionally journal-subordinated. | Separate physical object/shard locator from semantic reference identity; prove cache hit/miss, replica placement, and search execution cannot change visibility/provenance/admission. | Availability/rank becomes de facto semantic judgment or leaks policy. | E1/E2 |
| AF-A03 | Replica movement/follower catch-up versus authority-partition fencing | `aether_partition` contains both byte/prefix movement and semantic leader epoch, stale-epoch rejection, divergent-prefix fencing, explicit cuts, and imported-fact provenance. | Define an opaque `ReplicatePrefix` mechanical contract; hostile tests must show stale or divergent replicas can receive bytes but cannot gain append authority until AETHER validates epoch/prefix. | FABRIC liveness/replication becomes authority. | E1/E2 |
| AF-A04 | HTTP transport versus AETHER semantic service edge | HTTP framing is mechanical, while bearer auth, principal binding, policy narrowing, semantic namespace identity, append admission, and audit content are AETHER responsibilities. | Split request transport from a typed AETHER request contract; prove the same semantic service can run direct/in-process and over a transport adapter with equivalent semantic outputs. | Moving `aether_http` wholesale moves policy/admission into FABRIC or makes AETHER correctness network-dependent. | E1/E2 |
| AF-A05 | Namespace routing versus worker/queue realization | Selecting a semantic namespace and its policy context is meaning-bearing; selecting a worker/queue for an already resolved namespace is mechanical. | Produce `NamespaceResolved` then `MechanicalEnvelopeAuthorized`; run starvation/priority tests with worker selection varied while semantic namespace resolution remains fixed. | FABRIC scheduling can silently alter semantic tenant/policy selection. | E1/E2 |
| AF-A06 | Resource limits that are semantic safety versus operational backpressure | Document/rule/runtime/result limits may prevent undefined/unbounded semantic behavior, while queue/body/worker limits may simply protect resources. | For each limit, change it in a test: if canonical semantic meaning for accepted inputs changes, retain in AETHER; if only latency/admission-to-execution capacity changes under a fixed semantic contract, treat as mechanical. | A safety invariant could be relaxed by FABRIC, or an operational limit could unnecessarily ossify AETHER semantics. | E1 |
| AF-A07 | `aether_api` compatibility facade as a possible interface seam | The facade re-exports all public surfaces and is explicitly temporary after R6. It is not a principled AETHER/FABRIC contract. | E1 interface must compile/test without depending on `aether_api` as the normative owner. | Temporary compatibility becomes permanent cross-plane architecture and recreates coupling. | E1 |
| AF-A08 | Cross-plane performance and telemetry ownership | `aether_perf` measures semantic execution, HTTP, partition, sidecar, host facts, drift, and capacity. These are different semantic domains. | Partition metrics into semantic-correctness/performance, FABRIC transport/capacity, and deployment qualification; prove production crates remain independent of measurement tooling. | FABRIC may treat benchmark score as authority, or AETHER may absorb host scheduling policy. | E1 |
| AF-A09 | Coordination lease terminology | Current AETHER leases are semantic authority facts with epochs/fencing; future FABRIC likely needs operational resource leases. | Introduce distinct types/names in E1. Test that an operational lease cannot satisfy any semantic `lease_active` or fenced-action predicate. | Resource ownership is mistaken for institutional/action authority. | E1 |
| AF-A10 | Heartbeat terminology | Current coordination heartbeats contribute to semantic lease/fence state; endpoint liveness heartbeats are mechanical observations. | Define `SemanticLeaseHeartbeat` versus `EndpointLivenessObservation`; test missing endpoint heartbeat does not directly expire semantic authority unless an AETHER rule explicitly consumes admitted evidence. | Network failure directly rewrites institutional authority without semantic policy. | E1/E2 |
| AF-A11 | Manual replica promotion versus future mechanical failover | Current manual promotion increments an AETHER leader epoch and changes append authority; ordinary failover/placement belongs to operations. | Separate physical endpoint failover from authority promotion. Hostile test: a mechanically selected replacement remains read-only/unadmitted until AETHER authority transition succeeds. | FABRIC becomes an election/authority system by accident. | E2/E3 |
| AF-A12 | Federated import payload transport | Imported-fact semantics require explicit partition cuts and provenance, while moving the payload between hosts is mechanical. | Run identical federated evaluation with local direct import and transported import; compare canonical imported fact identity, cuts, provenance, policy visibility, and derived outputs. | A transport rewrite changes source identity or creates a fake global clock. | E2 |
| AF-A13 | Audit sink delivery versus required audit evidence | Event buffering/writing is mechanical, but some audit records are required for provenance/accountability. | Simulate blocked/full sink. System must fail or degrade according to declared audit policy without silently claiming a completed semantic action whose required audit evidence was lost. | Mechanical logging loss destroys accountability while semantic state claims success. | E2 |
| AF-A14 | AETHER-POL realization status | PR #10 defines a clean institutional layer but is not on live `main`; later repository history has diverged substantially. | Rebase conceptually, not mechanically: compare its object/fact model against current schema/policy/provenance and Council separation before any fresh candidate. | Stale design is treated as already admitted architecture. | separate POL admission |
| AF-A15 | AETHER-Learn location and objective authority | Desired allocation is institutionally meaningful, but no live implementation was found in current AETHER main under E0 search. | Establish exact owner/repository, objective contract, outcome ledger, and review route. Prove FABRIC telemetry can update evidence without directly changing the institutional objective. | Mechanical latency/cost becomes undeclared utility or routing policy. | separate AETHER-Learn admission |
| AF-A16 | GHOS payload transport integration | A future FABRIC could physically move GHOS work/results, but GHOS owns controller admission and protected execution authority. | Define a transport-only bridge; prove an unadmitted endpoint can receive bytes but cannot execute under GHOS authority or produce an accepted protected receipt. | FABRIC endpoint identity launders controller authority. | separate GHOS integration review |

## Highest-priority discriminating experiments

### D1 — Direct versus transported semantic equivalence

Run the same AETHER workload through:

1. direct/in-process semantic service;
2. a deliberately simple reference transport adapter.

Compare after removing schema-declared operational metadata:

- accepted journal cuts;
- append receipts;
- resolved state;
- derived tuples;
- policy visibility;
- provenance;
- proof traces;
- semantic admission outcomes.

Any unexplained divergence blocks extraction.

### D2 — Influence-without-authority hostile scheduler

Keep `AllocationDesired` and semantic inputs fixed. Vary only mechanical
realization using a hostile scheduler that attempts:

- starvation;
- priority inversion;
- selective delay;
- selective result suppression;
- retry asymmetry;
- locality bias.

Required result:

- operational completion/latency may change;
- AETHER must distinguish unknown/undelivered from false/rejected;
- no new institutional permission or semantic acceptance may appear.

### D3 — Replica fencing under mechanically successful replication

Allow a stale follower to receive every byte successfully while retaining an
old epoch.

Required result:

- physical synchronization can succeed;
- semantic append authority remains rejected until the AETHER authority
  transition is valid;
- divergent-prefix conditions remain fail-closed.

### D4 — Sidecar locality permutation

Move/cache artifact or vector payloads among locations while holding AETHER
semantic references fixed.

Required result:

- semantic identity/provenance/visibility are unchanged;
- availability/rank is reported as operational/retrieval evidence only;
- semantic admission still occurs through AETHER.

## E0 closure rule

This register may shrink only when a discriminating contract/test is recorded.
Entries must not be closed merely because a target architecture would be
cleaner if they belonged to FABRIC.
