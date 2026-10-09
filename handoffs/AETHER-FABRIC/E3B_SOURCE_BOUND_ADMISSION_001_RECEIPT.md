# E3B Source-Bound Admission 001 — Protected Non-Operative Receipt

**Assignment:** AETHER-E3B-SOURCE-BOUND-ADMISSION-001; [issue #136](https://github.com/grandchallenge/AETHER/issues/136).  
**Disposition:** bounded observational implementation **PASS**; complete source-issued revocable mechanical authorization **NOT PROVED**, not activated.  
**Protected predecessor:** `efe54fbfaf604402a1232f7a85e3f1477264a39c`.  
**Candidate head:** `7e5b251dafcf527134787a56e5b8ac278ad29591`.  
**Protected merge/readback:** `aae5322ec53cc9dd27f3c4025a89ccf2265d29ea` ([PR #138](https://github.com/grandchallenge/AETHER/pull/138)).  
**Related:** practical experiment #133; C3 governance #130; parent E3B #110.

## Admitted factual source subset

Only explicit opt-in `POST /v1/documents/run` with Query scope produces an AETHER-owned bounded source observation, after successful actual `HttpKernelState::execute` token/scope/namespace authentication, rate and namespace-admission checks, `BoundedBlockingExecutor::run` worker execution, endpoint effective `apply_policy_binding` **inside the existing semantic closure**, and successful `run_document_with_limits`. The observation binds source request/profile/principal/namespace, typed request digest, actually bound policy digest and semantic response digest. The semantics of the default reference path are unchanged. No witness or route permit is minted; `FabricRoutingMode::ReferenceOnly` is default.

The observation is internal, non-serializable as an issuer token, explicitly `OBSERVED_NON_OPERATIVE`, has `authority_effect = none` and no freshness grant. Its bounded store drops nonblocking on contention/overflow and reports lost evidence; none of those events blocks execution. ADR [0025](../../docs/ADR/0025-e3b-source-bound-reference-observation-is-not-authorization.md) is the authority/stage map.

## Exact candidate verification

- [Rust CI and required gate](https://github.com/grandchallenge/AETHER/actions/runs/37917951366): **SUCCESS** — `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`; includes the aether_http, aether_control_bridge, aether_fabric crates.
- [GCL conformance](https://github.com/grandchallenge/AETHER/actions/runs/37917952027): **SUCCESS**.
- [Supply Chain](https://github.com/grandchallenge/AETHER/actions/runs/37917951470): **SUCCESS**.
- [GH-OS routing-enforcement](https://github.com/grandchallenge/AETHER/actions/runs/37918212516): **SUCCESS**.
- Exact-candidate read-only Formalist/Adversary/Referee logical assessment: [PR #138 review](https://github.com/grandchallenge/AETHER/pull/138); same author is **not** an independent GitHub approval.
- HTTP fixture: seeded substantive `execution_authorized` rule evaluation, nonempty matching semantic result; independent default-OFF and preview-enabled routers agree on derived tuples and query rows.
- Denials: missing or invalid token, insufficient scope, revoked token, wrong namespace, namespace saturation, rate limiting, malformed JSON and policy escalation produce no extra successful proof. Real worker queue admission before worker start also produces no successful proof. Bounded/contended store cases report evidence loss.
- Existing real HTTP queue test and full workspace regression are included in the exact candidate's workspace run. No separate performance benefit is claimed.

## Unproven and withheld

- There is **no per-request source-issued reliable auth/config decision generation or fresh revocable AETHER control-state witness** through this seam: `source_revision = None` intentionally. Historical observations may be replayed/copied; they have no consumable authorization capability.
- Late revocation/supersession/expiry, configuration reload during queueing, adversarial stale cross-principal/cross-namespace records as fresh grants, cancellation races, and actual *future* mechanical placement-to-execution must be studied only in a separately governed authorization lane. No positive proof of those properties is asserted by this receipt.
- No live FABRIC routing, second resource, `RouteRealized` permission, semantic mutation, C3 #130 disposition, #133 practical-workload completion, C4 promotion, or E3B-A reserved activation.
- GH CI covers all workspace crates on the source candidate; separate standalone targeted command transcripts were not captured. No live deployment or benchmark.

## Next permitted step

Close #136's **bounded safe non-operative code** portion, retain a named separate source-authority/control-witness successor for authenticated fresh producer state and hostile revoke/reload/cancel races, and keep #133 and #130 open until their own dispositions. Any operative dispatch must obtain its own exact source gate, Article IX/XI classification and E3B-A reserved activation before cutover.
