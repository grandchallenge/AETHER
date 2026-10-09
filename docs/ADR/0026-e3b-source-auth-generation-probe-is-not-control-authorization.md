# ADR 0026 — AETHER HTTP-auth generation is a diagnostic epoch, not a control permit

**Status:** E3B #140 non-operative implementation candidate, exact-head CI and protected review required.  
**Protected predecessor:** `3b66cfc9125ee84cca1c2c998f1849299b5b04e0`.  
**Dependencies:** #136 protected result, ADR 0025, C0 authority issuance, #130 C3 acceptance, #133 practical acceptance, parent #110.  
**Authority classification:** no Article IX production-semantic authority change; Article XI/E3B-A **not** granted.

## Decision: source-owned epoch at the existing mutex

The preexisting `HttpKernelState::auth: Arc<Mutex<HttpAuth>>` serializes real token authorization and configuration reload. Retain that lock and introduce a process-local `HttpAuth::generation: u64`, initialized at zero on process construction. In the *same lock*:

- successful `HttpAuth::authorize` returns `AuthenticatedPrincipal::auth_generation` stamped with the actual token decision;
- `HttpAuth::replace_config` obtains a checked successor generation, constructs a replacement token map, and atomically replaces it after the reload handler has validated the deployment configuration;
- a failed configuration validation or a poisoned lock does not report a newly admitted generation.

This generation is *genuine for HTTP-auth configuration replacement in one process*. It is not an authoritative revision of the C1 mechanical authorization registry, semantic policy store, namespace service, source provenance, or cross-process state. Restarts reset it. The old endpoint policy-binding location, after the worker starts, remains unchanged.

The explicitly opt-in post-success `run_document` observation now carries `auth_generation_at_admission: Option<u64>`. Its existing `source_revision` remains **None**, precisely because there is still no complete AETHER-issued current control-state witness. No bearer secret is copied into the observation, and the source record remains a bounded, unsigned, non-serializable diagnostic rather than a C1 issuer input.

## Read-only source check

`HttpKernelState::source_control_preview_check` acquires the real auth mutex with `try_lock`, compares the current generation against the one captured at authentication, and requires exact equality with one retained AETHER observation (including request identity, principal/token reference, namespace, operation profile, typed payload, effective policy, and semantic response digests). A missing preview, lock contention, retention loss or unusable claim fails closed on **the diagnostic claim**. The result enum has no route ID, worker capability, E1 permit, or `RouteRealized`.

`SnapshotMatchNonOperative` states only that a historical observation matched *during this particular comparison*. A caller may copy the complete record and repeat this read-only comparison: an exact duplicate can match again but **cannot gain an execution permit**. That is not an anti-replay guarantee. The code is deliberately unsuitable for mechanical authorization and cannot be substituted for a one-time, consumed, source-issued authorization identity.

The lock acquisition gives atomic comparison with respect to local auth replacement, but it is released before a caller could dispatch. Such a hypothetical later dispatch would suffer a time-of-check/time-of-use race. The current implementation performs no dispatch. Future live admission would need AETHER-owned atomic *permit acquisition and consumption at the executor boundary*, with stable identity, expiry, revocation, namespace and policy proof and an audited cancellation lifecycle. It would need a distinctly governed Article IX/XI disposition and E3B-A activation.

## Hostile evidence and distinguishable outcomes

Tests use the real authenticated Axum router and existing `BoundedBlockingExecutor`: an admitted `run_document` waits behind a test-held worker permit while the exact source replacement routine swaps its token configuration under the auth mutex. The already-admitted operation may subsequently complete on the reference path, while a new request using the old token is rejected. The earlier historical observation reports the earlier generation and fails the live diagnostic recheck; the new authenticated request binds the current generation. These tests do **not** claim a deployed config-file reload or genuine externally sourced revocation event, only the same in-process replacement primitive used after normal validation.

Pure hostile checks distinguish changed request identity, principal, token reference, namespace, policy, typed payload, profile and semantic result; absent token, incomplete/lost evidence, missing readback, and mismatched auth generation. All fail closed on *claims*. Same-record replay remains a non-authorizing diagnostic match.

Worker cancellation before start versus completion after start, live service restart, expiration, independent token revocation not expressed through auth-config replacement, C1 supersession/expiry, and cross-process control-state consistency have **not** been proved. No no-duplicate-attempt semantics or true mechanical attempt creation exists in this tranche.

## Source ownership, lock order, constitutional gates

The live `auth` mutex remains the only lock added to the diagnostic check path; record emission takes only the bounded preview mutex, never `auth`. The checker takes `auth` before read-only preview access; the semantic worker does not take preview then auth. The deployment reload path holds `status` then `auth` as before and never takes the preview lock. Under any contention, the probe declines a claim rather than block reference execution. All existing service semantics, audit, namespace admission, journal and reference routing remain under AETHER.

This ADR approves no new authority: no C2 shadow promotion, no `source_operation_admitted: true` laundering, no second resource, no `live_fabric`, no change to the C0 first-lane eligibility registry, no closure of #130/#133, and no E3B-A reserved production activation. A future source-issued, consumptive permit/witness crosses a separately governed authorization boundary. `ReferenceOnly` remains the default.

## Acceptance boundary

Candidate quality requires exact-head Rust formatting, Clippy, full workspace tests, GCL conformance, supply-chain, GH-OS routing enforcement, separate logical Formalist/Adversary/Referee passes, protected merge and main readback. The valid terminal claim is **auth-generation probe complete**; the full current-authority witness remains unavailable.
