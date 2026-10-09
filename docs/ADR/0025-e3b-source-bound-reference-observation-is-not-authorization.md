# ADR 0025 — E3B source-bound completed-reference observation is not authorization

**Status:** implementation candidate, non-operative/default-OFF; subject to exact-head CI and protected review.
**Issue:** [#136](https://github.com/grandchallenge/AETHER/issues/136).
**Protected predecessor:** `efe54fbfaf604402a1232f7a85e3f1477264a39c`.
**Governance:** C0 authority-issuance contract; E3B #110; practical-workload #133; unresolved C3 #130.

## Decision and scope

The initial E3B source producer is a **post-execution, AETHER-owned observational record** for precisely one operation: genuine authenticated `POST /v1/documents/run` with Query scope. Instrument **inside** its existing semantic-operation closure, *after* its existing `apply_policy_binding` and *after* `run_document_with_limits` returns a successful response. No other handler is enrolled. Do not move a policy check or replace `HttpKernelState::execute`, `BoundedBlockingExecutor::run`, service journal writes, or tracing.

Instrumentation requires explicit `HttpKernelOptions::with_source_bound_preview_capacity`; the default is `None`. The opt-in store is memory-bounded, nonblocking and diagnostic only, with an explicit lost-observations counter and no permitted emission failure path into reference execution. Its records deliberately implement **no serde transport**, no issuer interface, no signature, no dispatch handle and no authoritative source-control witness. Existing `FabricRoutingMode::ReferenceOnly` stays default; `CandidateReadiness` remains read-only.

The observation is *stronger than authentication or C2 shadow input* because the callback is reached only after the actual AETHER `authorize`, `RateLimiter::admit`, `admit_namespace`, `BoundedBlockingExecutor` worker-permit acquisition and semantic closure entry; the record is emitted only after the genuine endpoint policy binding and a successful nonempty-capable service response. A denied policy after a worker starts produces **no successful observation**.

This is not even a pre-execution authorization certificate: it is a historical diagnostic of AETHER's completed reference work. A new FABRIC-controlled mechanical attempt still requires distinct, fresh, governed AETHER mechanical authorization and a source control-state witness.

## Source authority timeline

| Stage | Source of fact | What is established | What is *not* established |
| --- | --- | --- | --- |
| Router/body parsing | Axum handler | Method/path and syntactically typed payload | Admission or semantic success |
| `HttpKernelState::execute` | Namespace resolution, `HttpAuth::authorize` | Principal, token, scope, namespace at that point in time | Rate/namespace/worker admission; effective endpoint policy |
| Legacy optional C2 shadow | `C2ShadowController::observe_request` before rate and namespace | Speculative off-path comparison | Actual downstream source admission, governed control witness |
| Rate and namespace | `RateLimiter::admit`; `admit_namespace` owned permit | Rate success; namespace slot admitted | Worker start, policy, or effective control revision |
| Executor | `BoundedBlockingExecutor::run` | Global admission permit, worker permit; closure executes on blocking worker | Endpoint policy acceptance or semantic success |
| `run_document` callback | `apply_policy_binding` | **Effective** policy for this authenticated principal and request | Mechanical reauthorization for future work |
| AETHER semantic core | `run_document_with_limits` | On `Ok`, executed work, result identity/provenance and actual response for hashing | HTTP delivery, FABRIC selection, live benefit |
| Preview record | Private nonblocking bounded `SourceBindingPreview::record` | Post-hoc exact request/profile/policy/result digests; success observations | Revocation/supersession/expiry control witness, routing/attempt authorization |
| Future governed recheck | **Not implemented** | None | Fresh control-state witness, rollback, mechanical attempt, `RouteRealized` |

For `execute_partitioned` and `resolve_execution_trace`, the source timeline differs; this tranche does not authorize instrumentation there. The legacy `ExplainTuple` 409 remains governed by #130, not a positive exception.

## Identity, loss and freshness

Each observation binds AETHER's own request ID, validated principal/token identity and namespace, Query scope, fixed operation-class/profile digest, digest of the typed request as parsed by the endpoint, digest of **the actual effective** AETHER-bound policy (or the canonical public-policy marker), and hash of the successful semantic response. All successful-observation flags are facts inferred from a callback actually running in the source execution stack. Records are not user-supplied evidence and cannot be fed to C1's issuer as an authenticated grant. No `source_operation_admitted` is reused from shadow.

Local observation time is diagnostic. The service currently lacks a single trustworthy per-operation auth/config decision generation/revision through this seam, so `source_revision = None`; inventing one would falsely claim fresh, revocable control. A returned record may become stale immediately after an auth reload or token revocation; never use readback for downstream permission. A queue wait before callback is not a successful proof. A successfully completed worker may finish after HTTP cancellation; observation proves semantic completion, not delivery.

The preview's evidence can be dropped on retention overflow, lock contention or poison. The dropped counter distinguishes incomplete readback; poisoned/contended readback is unavailable. Neither loss nor a corrupted preview causes source work to fail or reroute. The store's diagnostics are **not** signed durable evidence or complete temporal coverage.

## Constitutional classification and successor

**Non-operative observation:** no Article IX authority change asserted; this ADR is not Article XI approval for a different execution path.

**Operative authorization:** must be handled in a separately governed lane with an AETHER-issued fresh source authority/control witness, revocation/reload/expiry identity, placement-to-execution check, cancellation and attempt lifecycle, exact reference-equivalence and rollback tests, and explicit Article IX/XI classification. No C2-to-C3 promotion; #130 must be resolved independently; preserve one `aether-local-blocking-pool` until authorized expansion. E3B-A production activation remains reserved and unset.

**Required verification before protected admission:** actual HTTP seeded `execution_authorized` query; authorization/policy/rate/namespace/worker distinctions; denial, exhaustion, tamper and freshness gaps; exact-head fmt/clippy/workspace and targeted tests; GCL, security, supply-chain and routing enforcement; non-authoring logical Formalist/Adversary/Referee passes and protected merge/readback. Where coverage cannot be produced, report PARTIAL rather than fabricated certification.
