# ADR 0028 — E3B one-attempt start needs atomic source authority at executor boundary

**Status:** #143 non-operative hostile model accepted under protected PR #144 at `566e7f4743a64d33e4a9ef378f4fa6f66466efc9`; operative authority withheld.  
**Protected predecessor:** `2cae6f3058d581d75fd0e7df92c9f3fc7e9b5433`.  
**Governing contract:** [E3B C0 authority-issuance contract](../ARCHITECTURE/AETHER_FABRIC_E3B_C0_AUTHORITY_ISSUANCE_CONTRACT.md), ADR 0027 mechanical issuer, ADR 0025 post-success observation, ADR 0026 source-auth epoch, C3 #130 and practical #133.  
**Classification:** off-path specification and tests only; `NO_ARTICLE_IX_AUTHORITY_CHANGE` under current C0 scope; NO operative Article XI/E3B-A authorization.

## Source map and the missing atomic transition

The real AETHER HTTP path is `HttpKernelState::execute`: authenticate under `HttpAuth` mutex (source auth generation), check rate and namespace, then await `BoundedBlockingExecutor::run` global admission and worker permits. Its `spawn_blocking` closure resolves the AETHER namespace and performs the genuine `run_document` effective policy binding and semantic work. The worker slot can be held while auth configuration changes. HTTP future cancellation while queued means no blocking worker is started; cancellation after `spawn_blocking` starts does not recall synchronous semantic execution.

C0/C1's `AetherMechanicalAuthorityIssuer` owns the in-process envelope-control registry and current `ControlStateWitness` for revoke/supersede/expiry. FABRIC's candidate realization only evaluates a supplied witness/read-only snapshot and returns non-authoritative evidence. It neither holds the AETHER auth mutex and C1 control registry through `spawn_blocking` start, nor obtains or consumes a source-issued worker permit. C0 defines `mechanical_attempt_id` only at `RouteRealized`, not at HTTP request arrival. The E3B #140 source-auth generation is local and independent of the C1 envelope revision, namespace and semantic policy.

Therefore **queue admission, a historical source observation, or read-only `RouteRealized` cannot authorize a subsequent mechanical start**.

## Test-only state-machine experiment

The candidate adds `crates/aether_http/src/source_attempt_model.rs`, compiled only with `#[cfg(test)]`; no production module/function/API is added. `SimulatedAtomicStart` is explicitly **idealized**: an exclusive mutable model state owns all simulated source revisions and logical attempt transitions, unlike the currently separate live locks. It makes no C1 issuer call, no FABRIC dispatch, no real token/policy read, and emits no E1 permit.

- One immutable binding covers request/envelope, principal/token, namespace, operation profile, typed payload digest, effective policy digest and the sole `aether-local-blocking-pool` resource.
- `Queued` stamps a simulated source snapshot but grants nothing. `WorkerAcquired` still grants nothing.
- The only hypothetical start linearization is `commit_start`, which under the model's exclusive authority simultaneously checks current envelope state, expiry, scope, namespace, source availability, auth/policy/control revision, identical binding and fresh attempt phase, and changes the attempt to `Started` once.
- `Cancelled` or `Rejected` is terminal without a start, consuming its attempt identity; `Started` may complete after a later revocation or HTTP task cancellation. C0 local profile `max_attempts = 1` prohibits a new attempt ID for the same request/envelope; no silent retry/backoff.
- Stale/tampered cross-request, principal, token, namespace, policy, payload, operation profile, envelope and resource identity cannot commit. Replayed attempt and duplicate logical request are rejected.

All these model transitions are **hypotheses under an idealized serializable lock**; their passing tests do not prove live AETHER issuance or a real atomic lock integration. The model cannot be used to acquire an executor permit. No changes are made to `HttpKernelState` reference semantic/routing path or E3B control bridge.

A separate test uses the **real** `BoundedBlockingExecutor` and Tokio cancellation: cancel while waiting behind a held worker permit and assert no closure start; then cancel after the actual blocking closure begins and assert synchronous work still finishes. This tests real executor cancellation semantics, **not** a real FABRIC or C1 source permit.

## Governing lock order and open requirements

Any operative start would require an explicit AETHER-owned *source authority transaction*: a stable same-request C1 envelope/control issuer identity and revocation/expiry, actual token-generation and current token validity, current namespace/scope, effective policy provenance/revision (not just a guessed digest), and a worker-start permit **consumed atomically** with actual start. A valid predecessor `RouteRealized` can only be matched to that exact attempt after a newly observed source control state; it cannot synthesize the permit.

The present code does **not** define a safe lock order across live `status -> auth` reload, C1 mutable registry, namespace service and worker permits; holding those locks across blocking user work is also unacceptable. An operative design must specify a consistent global lock order or an ownership/transaction handoff where a single AETHER source actor owns the start transition, and test revocation between each stage, cancellation, expiry, restart, and duplicate retry. It must decide whether revocation before start forbids the start and what audit evidence is emitted for cancellation; it must not retroactively cancel already-started semantic work.

Do not treat an in-memory auth counter (resets on restart) as a durable replay identity, or the C1 `OperationAdmissionInput.source_operation_admitted` field as an authentication primitive. The C2 shadow cannot issue a real source permit. Do not widen the one-pool closed-world profile or route excluded mutation operations.

## Authority and acceptance

The current Article IX boundary remains entirely with AETHER; Article X reserved production/control-plane disposition and Article XI constitutional amendment apply when their actual trigger is reached. This test-only work neither changes constitutional law nor authorizes E3B-A. Separate C3 #130 and #133 practical acceptance cannot be inferred.

Required for a source-model *test subset* claim: exact-head `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, targeted executor/model suites, GCL conformance, supply chain, GH-OS routing, logical Formalist/Adversary/Referee review, protected merge/readback. Overall #143 must remain **PARTIAL** until an actual source-issued consumptive permission and its revocation/replay evidence exist under a separately authorized operative tranche.
