# E3B #140 — Source Auth Generation Diagnostic, Protected Readback

**Assignment:** E3B source-current-control witness — revocation, reload and replay contract (non-operative), [issue #140](https://github.com/grandchallenge/AETHER/issues/140).  
**Disposition:** **PARTIAL** for the full current-control-witness assignment; genuine HTTP-auth generation and read-only source replay probe **PROTECTED**.  
**Protected predecessor:** `3b66cfc9125ee84cca1c2c998f1849299b5b04e0`.  
**Source candidate:** `1e6f3085f2855ce56694ed5d66a2eaff16af2145`.  
**Source protected merge/readback:** `3bbe48d7c361dd7a740def1da4ce56497a2d24ae`; [PR #141](https://github.com/grandchallenge/AETHER/pull/141).  
**Governance:** ADR [0026](../../docs/ADR/0026-e3b-source-auth-generation-probe-is-not-control-authorization.md), prior #136 protected source observation, #110 parent, C3 #130, practical #133.

## What is actually proved

The real `HttpAuth` source mutex now owns a process-local checked monotone auth generation. The generation attached to `AuthenticatedPrincipal` is captured under the actual bearer-token authorization lock; a successful validated auth-config replacement increments it while holding the same lock. A changed generation makes the earlier opt-in `SourceBoundObservation` stale for the new **diagnostic comparison**. The record remains non-serializable as an authority token and its `source_revision` stays unset, since HTTP auth generation is **not** a general-purpose source/control-state revision.

`HttpKernelState::source_control_preview_check` is read-only, non-operative and uses `try_lock` to compare the source generation and exact retained observation: request, principal/token identity, namespace, typed payload, policy, operation profile and semantic result. Missing/lost evidence and source changes decline the claim. An exact copy can still match repeatedly; this is historical **snapshot membership only**, never a consumptive authorization, route, dispatch permit, or replay immunity.

The real HTTP test seeds substantive `execution_authorized` rule evaluation, forces an already-authorized request to queue behind the actual `BoundedBlockingExecutor` worker permit, executes the same source auth replacement primitive used by the validated reload path, then releases the worker. That already-admitted source work may finish, whereas a subsequent new request with the retired token is denied. Its old source observation is classified stale by current diagnostic recheck. A new accepted token gets the next epoch and matches in the read-only diagnostic. Additional exact-record tampering (request, principal/token, namespace, policy, payload, profile, semantic result) and missing evidence fail closed. The default ReferenceOnly path remains unchanged.

## Exact candidate checks and audit

- [Rust CI](https://github.com/grandchallenge/AETHER/actions/runs/37923248373) **PASS**: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (including all aether_http/aether_control_bridge/aether_fabric test targets). Required CI aggregate PASS.
- [GCL conformance](https://github.com/grandchallenge/AETHER/actions/runs/37923249189) **PASS**.
- [Supply Chain](https://github.com/grandchallenge/AETHER/actions/runs/37923248275) **PASS**.
- [GH-OS routing enforcement](https://github.com/grandchallenge/AETHER/actions/runs/37923495350) **PASS** after ready-for-review; first PR trigger also passed.
- Formalist, Adversary and Referee: read-only *logical* [PR #141 review](https://github.com/grandchallenge/AETHER/pull/141); authoring-system comments are not independent GitHub approval.
- Exact source candidate, PR, protected merge and `main` readback verified.

## Definitive nonclaims

**No source-current *mechanical control-state witness* has been issued.** The new epoch detects local auth-config replacement, but is not durable across restart and says nothing about separately governed C1 revocation/supersession/expiry, semantic policy revision, or cross-process control. A snapshot check ends when its mutex is released; a future dispatch would face a TOCTOU race. It issues no one-time permit, no operation-attempt identity, no revocation-safe mechanical start, no cancellation rollback, no performance/benefit claim. An identical diagnostic replay still matches and cannot be treated as a consumptive authorization.

The test uses the real source auth replacement primitive *directly under the source mutex*, not a deployed configuration-file reload. External reload/revocation/expiry and cancellation races have not all been reproduced. No `source_operation_admitted: true` shadow promotion, no new FABRIC execution path, no `live_fabric`, no second eligible resource, no Article IX/XI operative authorization, no #130/#133 closure, no C4 or E3B-A activation.

## Next legitimate authority transaction

Separate a bounded *specification and adversarial* tranche for a genuine source-issued, one-time/consumptive mechanical start permit under the actual executor lock/order, including fresh current control-state registry, token/policy/namespace generation, deterministic cancel/revoke/retry identities, expiry and reload races, before any operative code. Any such step must be independently classified under Article IX/XI and E3B-A; a completed generation probe does not authorize it.
