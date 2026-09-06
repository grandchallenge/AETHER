# AETHER/FABRIC E0 Ambiguity Register

Status: E0 candidate, revised after Adversary pass
Issue: #83

An ambiguity is not a defect. It is a prohibition on premature extraction.
Entries close only through a narrower contract plus discriminating evidence.

| ID | Ambiguous seam | Why unresolved | Discriminating evidence / test | Misclassification failure | Earliest decision |
| --- | --- | --- | --- | --- | --- |
| AF-A01 | Durable journal backend mechanics vs authoritative journal semantics | SQLite/Postgres/filesystem I/O is mechanical; exact append/cut/receipt semantics are AETHER authority. | Backend substitution must preserve identical accepted cuts, receipts, replay, restart and failure postconditions. | Storage success/failure silently changes accepted history. | E1/E2 |
| AF-A02 | Sidecar payload locality vs semantic sidecar contract | Physical bytes/vectors are external, but identity/provenance/policy/replay are journal-subordinated. | Permute/cache/move payloads while holding semantic references fixed; identity, visibility, provenance and admission must remain unchanged. | Availability/rank becomes semantic judgment. | E1/E2 |
| AF-A03 | Replica movement vs authority-partition fencing | Follower movement is mechanical; epoch, prefix, cut and authority checks are semantic. | Allow stale follower to receive all bytes; it must remain unable to gain append authority until AETHER validates epoch/prefix. | Replication/liveness becomes authority. | E1/E2 |
| AF-A04 | HTTP transport vs AETHER service semantics | Framing is mechanical; auth, policy, namespace, admission and audit are semantic. | Direct/in-process and transported service calls must yield identical canonical semantic outcomes. | Moving HTTP wholesale makes AETHER network-dependent or moves policy/admission into FABRIC. | E1/E2 |
| AF-A05 | Namespace identity vs worker realization | Namespace/policy selection is semantic; concrete worker/queue selection is mechanical. | Produce a resolved semantic namespace first, then vary worker choice while semantic results remain invariant. | Scheduler silently changes tenant/policy domain. | E1/E2 |
| AF-A06 | Semantic safety limits vs operational limits | Rule/runtime/tuple/document limits can protect semantic boundedness; body/queue capacity may be mechanical. | Change one limit at a time. If accepted-input meaning/atomicity changes, retain in AETHER; if only resource realization changes, mechanical ownership is plausible. | FABRIC can relax semantic safety or AETHER unnecessarily ossifies capacity policy. | E1 |
| AF-A07 | Worker queues/backpressure vs semantic serialization/cancellation | The live resource contract combines mechanical worker/queue capacity with one-active-operation same-namespace order and `cancel_before_start_complete_after_start`. | Vary worker count, queue capacity, wait, timeout and scheduling. Operations that start must preserve accepted cuts/receipts; pre-start timeout must never produce a later hidden commit; same-namespace order must remain deterministic where required. | Mechanical timeout reports failure while semantic mutation later commits, or queue policy changes semantic order. | E1/E2 |
| AF-A08 | `aether_api` compatibility facade | It re-exports many surfaces and is explicitly temporary under ADR 0019. | E1 contract must compile/test without making the facade the normative owner. | Temporary compatibility becomes permanent cross-plane coupling. | E1 |
| AF-A09 | Cross-plane performance/telemetry ownership | `aether_perf` measures semantic runtime plus HTTP/partition/host/capacity behavior. | Split metric vocabulary into semantic, mechanical and qualification observations; production crates remain independent of measurement tooling. | Metrics become policy/authority or obscure causality. | E1 |
| AF-A10 | Coordination lease terminology | Current AETHER lease is semantic authority with epoch/fencing; FABRIC may need operational resource leases. | Distinct types/names; prove operational lease cannot satisfy semantic `lease_active`/fenced predicates. | Resource ownership is mistaken for action authority. | E1 |
| AF-A11 | Heartbeat terminology | Current coordination heartbeat can participate in semantic lease state; endpoint liveness is mechanical observation. | Separate semantic lease heartbeat from endpoint liveness evidence; missing endpoint heartbeat cannot directly expire authority unless an admitted AETHER rule says so. | Network loss directly rewrites institutional authority. | E1/E2 |
| AF-A12 | Manual authority promotion vs mechanical failover | Current promotion increments leader epoch and changes append authority. | Physically select a replacement endpoint while withholding semantic promotion; it must remain non-authoritative. | FABRIC becomes election/authority system. | E2/E3 |
| AF-A13 | Federated import payload transport | Import semantics bind explicit cuts/provenance; moving bytes is mechanical. | Local-direct vs transported import must preserve source cut, provenance, policy and derived results. | Transport changes source identity or invents global order. | E2 |
| AF-A14 | Audit sink delivery vs required audit evidence | Buffering is mechanical; some audit records are accountability requirements. | Block/fill sink; system must follow declared fail-visible policy and never claim a completed accountable operation whose required evidence silently vanished. | Mechanical loss destroys accountability. | E2 |
| AF-A15 | AETHER-POL realization status | PR #10 is semantically aligned but stale and absent from live main. | Fresh candidate must be reviewed against current schema/policy/provenance and Council architecture. | Stale design is treated as already admitted. | separate POL admission |
| AF-A16 | AETHER-Learn location/objective authority | Desired allocation is institutional, but no live implementation was found in current main search. | Establish exact owner/objective/outcome-ledger contract; FABRIC telemetry may enter only as evidence. | Cost/latency becomes undeclared utility/policy. | separate Learn admission |
| AF-A17 | GHOS payload transport integration | FABRIC may later move GHOS payloads; GHOS owns controller admission. | Transport-only bridge must prove unadmitted endpoint can receive bytes but cannot execute with GHOS authority. | Endpoint identity launders controller authority. | separate GHOS review |
| AF-A18 | Audit/log backpressure vs semantic completion | Event queue is mechanical but AETHER may require audit evidence for semantic accountability. | Exercise full/blocked audit queue during semantic operations; completion/failure must follow declared contract and remain reconstructible. | Backpressure silently erases evidence or changes truth. | E2 |

## Priority experiments

### D1 — Direct versus transported semantic equivalence

For identical admitted semantic inputs, compare direct/local AETHER with an
AETHER-over-reference-transport path after removing schema-declared operational
metadata. Compare:

- accepted journal cuts and append receipts;
- resolved state and derived tuples;
- policy visibility;
- provenance and proof traces;
- semantic admission outcomes.

Unexplained divergence blocks extraction.

### D2 — Influence-without-authority hostile scheduler

Hold institutional allocation and semantic inputs fixed. Vary only mechanical
realization using:

- starvation and selective delay;
- priority inversion;
- namespace/principal unfairness;
- head-of-line blocking;
- queue-capacity pressure;
- pre-start timeout;
- retry asymmetry;
- locality bias;
- result suppression.

Required:

- latency/completion may change;
- unknown/undelivered remains distinct from false/rejected;
- no permission or semantic acceptance is created;
- no operation reported timed out before start may later commit;
- once semantic execution begins, mechanical timeout cannot convert it into a
  background unobserved mutation;
- same-namespace semantic ordering remains deterministic where the AETHER
  contract requires serialization.

### D3 — Replica fencing despite mechanically successful replication

A stale follower may receive all bytes successfully while retaining an old
epoch. It must remain unable to append authoritatively; divergent prefix remains
fail-closed.

### D4 — Sidecar locality permutation

Move/cache artifact/vector payloads among locations while holding AETHER
semantic references fixed. Semantic identity, provenance and visibility remain
unchanged; availability/rank remains operational evidence until admitted.

### D5 — Resource-control semantic/mechanical split

Sweep global worker count, namespace queue length and queue wait independently
from semantic operation limits. Demonstrate:

1. mechanical capacity changes throughput/latency, not meaning;
2. one-active-operation same-namespace order is preserved;
3. `cancel_before_start_complete_after_start` holds;
4. rejected resource admission cannot partially append authority or publish an
   AETHER execution receipt/trace handle.

## Closure rule

An entry may be removed from this register only when a versioned contract and
discriminating evidence resolve it. Architectural elegance is not evidence.