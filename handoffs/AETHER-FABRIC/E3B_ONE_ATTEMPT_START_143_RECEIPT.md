# E3B #143 — One-Attempt Source Start Model: Protected Non-Operative Receipt

**Issue:** [#143](https://github.com/grandchallenge/AETHER/issues/143).  
**Disposition:** **PASS** for protected test-only state-machine and genuine executor cancellation boundaries; **PARTIAL** against full source-issued, consumptive, revocation-safe mechanical start authority.  
**Protected starting main:** `2cae6f3058d581d75fd0e7df92c9f3fc7e9b5433`.  
**Exact source head:** `a395edfaeb60f199bd9f4dde46d82bfe99a67f86`.  
**Protected implementation PR:** [#144](https://github.com/grandchallenge/AETHER/pull/144); merged/read back `566e7f4743a64d33e4a9ef378f4fa6f66466efc9`.  
**Authority sources:** protected [C0 contract](../../docs/ARCHITECTURE/AETHER_FABRIC_E3B_C0_AUTHORITY_ISSUANCE_CONTRACT.md), [ADR 0027](../../docs/ADR/0027-aether-mechanical-authority-issuer.md), [ADR 0028](../../docs/ADR/0028-e3b-source-attempt-start-linearization-is-model-only.md), prior #140 receipt, parent #110, C3 #130 and practical #133.

## Actually implemented

- `crates/aether_http/src/source_attempt_model.rs` is compiled **only** with `#[cfg(test)]`, with no runtime issuer/capability or route wiring. A deliberately idealized single mutable authority model gives precise linearization of simulated auth/policy/control revisions and source availability against an externally supplied hypothetical attempt identity. `Queued` and `WorkerAcquired` are non-authorizing; `commit_start` can consume once only when current modeled source evidence and immutable request/envelope/principal/token/namespace/payload/policy/profile/resource bindings all match. Cancelled/rejected attempts are terminal and C0's `max_attempts=1` prevents another attempt ID for the same logical request/envelope. It models both revocation-before-start denial and start-before-revocation allowed completion. It is **not** a proof of the current production lock structure.
- Tests adversarially cover revocation, supersession, expiry, auth reload, policy/control revision, missing source, rejected scope/namespace, cross-principal/token/request/namespace/profile/payload/policy/envelope/resource, replay/duplicates, cancellation before start and after worker slot acquisition, completion after start, and wrong-resource refusal.
- `crates/aether_http/src/http.rs` gains a separate **real** `BoundedBlockingExecutor` test: cancellation of an async task queued behind a held real worker semaphore prevents its blocking closure from starting and frees its permits; cancellation of the async caller after blocking work starts does not interrupt the synchronous closure, which completes and releases capacity.
- Source `HttpKernelState::execute`, `run_document` semantic closure, `HttpAuth`, C1 authority-issuer implementation, `fabric_candidate_realize_placement` and default `ReferenceOnly` routing **unchanged**.

## Exact-head evidence

- [CI run 38200769689](https://github.com/grandchallenge/AETHER/actions/runs/38200769689): `Rust PR fast` **SUCCESS**, `Required CI gate` **SUCCESS**, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` all exercised.
- [GCL conformance run 38200770128](https://github.com/grandchallenge/AETHER/actions/runs/38200770128): **SUCCESS**.
- [Supply Chain run 38200769626](https://github.com/grandchallenge/AETHER/actions/runs/38200769626): **SUCCESS**.
- [GH-OS routing enforcement run 38200948821](https://github.com/grandchallenge/AETHER/actions/runs/38200948821): **SUCCESS** following ready-for-review (earlier PR trigger also succeeded).
- Exact-head read-only logical Formalist, Adversary and Referee comment on [PR #144](https://github.com/grandchallenge/AETHER/pull/144), review ID `5486345959`. This is **not** a separate-user GitHub approval.
- PR #144 exact candidate and protected merge commit read back via authenticated GitHub repository API.

## Claims expressly withheld

No actual AETHER source-issued one-shot mechanical authorization is constructed or consumed. The model assumes an ideal atomic authority state which is *not* a live synchronization primitive across `HttpAuth`, C1's `EnvelopeControlRegistry`, service policy/namespace and the worker semaphore. Its simulated revocation races cannot certify a real source-control witness or genuine mechanical dispatch. Existing C1 `mechanical_attempt_id` originates at `RouteRealized`; any hypothetical model ID is not a new HTTP-issued identity or permit. No durable cross-process replay barrier, restarted-process generation, deployed auth reload, active FABRIC resource acquisition, actual attempt rollback, semantic-result benefit, or complete cancellation integration has been demonstrated.

C3 #130, practical #133, C4 and E3B-A remain open/unactivated, and Article IX/XI authority has not been widened. No production route, second resource class, excluded mutation, public serialized `source_operation_admitted`, or C2-shadow issuer elevation was introduced.

## Separately governed next transaction

Use the [operative permit authority packet](E3B_SOURCE_CONSUMPTIVE_PERMIT_AUTHORITY_PACKET_001.md) as a **preparation-only** exact-source contract for a source-owned atomic start transaction, identity lifetime/consumption, lock ordering, genuine source revisions and hostile deployed runtime races. Do not attach this test-only model to live execution or start an activation PR without the reserved governance disposition.
