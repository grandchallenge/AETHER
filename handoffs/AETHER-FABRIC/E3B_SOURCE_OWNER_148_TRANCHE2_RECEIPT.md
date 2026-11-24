# E3B #148 Tranche 2 — Non-Operative Namespace Lifecycle & Consumptive Transaction Receipt

**State:** NON-OPERATIVE PREPARATION PASS / OPERATIVE SOURCE-OWNER & LIVE FABRIC DISPATCH WITHHELD (`PROTECTED_PARTIAL`).
**Issue:** [#148](https://github.com/grandchallenge/AETHER/issues/148).
**Protected predecessor:** `main` `bd799745263d0f9ccde09b71be078640db800738` (following `#148` Tranche 1 / PR `#170` at `e637b6b3` and `#166` / PR `#171` at `bd799745`).
**ADR:** [0034](../../docs/ADR/0034-e3b-namespace-lifecycle-and-consumptive-transaction.md) (extending [0029](../../docs/ADR/0029-e3b-source-owner-lock-order.md) and [0033](../../docs/ADR/0033-e3b-actual-source-worker-boundary.md)).
**Constitutional classification:** `NO_ARTICLE_IX_AUTHORITY_CHANGE` (strictly `#[cfg(test)]` + ADR/documentation only; zero production code, routing, API, journal, or dependency changes).

---

## 1. Real AETHER surfaces exercised vs. non-operative contract fixtures

### Real AETHER surfaces exercised (`ACTUAL_SOURCE_RESULTS`)
- `HttpKernelState` with SQLite-backed `NamespaceServiceDirectory` (`crates/aether_http/src/http.rs`).
- Real `HttpAuth` bearer-token / scope / namespace authorization and file-backed validated `reload_auth_from_config()` (`status -> auth`).
- Real request-specific effective policy binding via `bound_policy_context(&principal, requested)` (`http.rs:2563-2591`), covering initial escalation denial, narrowed request execution, reload narrowing below requested policy (`PolicyEscalationDenied`), reload widening generation change (`SourceChanged`), and payload/policy substitution (`IdentityMismatch`).
- Non-blocking (`try_lock`) inspection of real `NamespaceServiceDirectory` handles, including real `NamespaceServiceState::Failed` triggered by a genuine SQLite initialization failure while `HttpAuth` and namespace semaphore remain valid.
- Real `RateLimiter::admit`, real `admit_namespace` (`OwnedSemaphorePermit`), and real `BoundedBlockingExecutor` (`admitted` + `workers` semaphores and `spawn_blocking` execution) at both queue and worker-acquired barriers.
- Real `AetherMechanicalAuthorityIssuer` / `EnvelopeControlRegistry` (fixture-instantiated), `fabric_shadow_compare_placement`, and pure `realize_selected_placement` (`authority_effect == "none"`, `endpoint_id == "aether-local-blocking-pool"`).
- Continuous assertion that `FabricRoutingMode::ReferenceOnly` is preserved across every test and failure mode.

### Non-operative contract fixtures (`CONTRACT_MODELED_RESULTS`)
- `AuthoritativeNamespaceLifecycle` (`Uninitialized` / `Active(r)` / `Draining(r)` / `Quarantined(r)` / `Retired(r)`): proves fail-closed denial on drain, quarantine, retirement, terminal re-activation refusal, revision rotation, and missing lifecycle entry independently of `HttpAuth::generation`.
- Canonical SHA-256 `AttemptBinding` digest (22 bound fields including `process_incarnation_id`, issuer provenance, admission/authorization/envelope/profile/payload/policy digests, source revisions, realized attempt/endpoint, and wall bounds) with private non-`Clone`, non-`Serialize`, non-`Deserialize` `PreparedAttempt` and `NonOperativeStartToken`.
- Lock-order safe 3-step consumption (`OwnerCore` RAM-only -> `DurableLedgerWriter` out-of-lock `sync_all()` -> `OwnerCore` RAM-only): zero `.await`, zero disk/DB I/O, zero lazy namespace open, and zero semantic work under `OwnerCore`; non-blocking `try_lock` on external `HttpAuth` and `NamespaceServiceDirectory`.
- Two-mode process crash/restart policy:
  - **Mode A (Ephemeral):** fresh 128-bit `process_incarnation_id` denies all pre-restart attempts (`IncarnationMismatch`).
  - **Mode B (Durable SHA-256 hash-chained JSONL ledger):** survives restart to reject replays of consumed/completed AND in-flight-at-crash (`ConsumedStarted` without `Completed`) attempts (`AttemptAlreadyConsumed`), cancelled (`AttemptCancelled`), and denied (`AttemptAlreadyTerminal`) attempts; rejects unstarted pre-restart attempts (`IncarnationMismatch`); persists wall-clock high-water mark to block post-restart rolled-back clocks at `prepare` (`PrepareError::ClockRollback`) and mid-flight rollback at `consume_start` (`ClockRollbackDenied`); fails closed at startup (`LedgerError::IntegrityFailed`) on byte tamper, truncation, or 1-byte torn tails lacking `'\n'`, and fails closed mid-run (`ConsumeVerdict::LedgerIntegrityFailed`) via full on-disk file SHA-256 verification on truncation or same-length byte flips.
- Dual-clock expiry & rollback checks (wall rollback, frozen wall clock + monotonic `elapsed >= ttl`, wall expiry + real C1 `ControlState::Expired`, plus real system-clock short-TTL worker expiry).
- Cancellation vs. post-start completion linearization inside real `BoundedBlockingExecutor` (pre-start cancel blocks start; post-start C1 revoke, C1 supersede, real token-revoking reload, namespace quarantine, and late cancel fail to roll back already-started synchronous work, which completes once and rejects duplicate start).
- Multi-threaded stress test running concurrent real `reload_auth_from_config()`, lifecycle transitions, and real worker-barrier Mode B durable transactions within a bounded timeout with full on-disk hash-chain verification at exit.

---

## 2. Explicitly withheld authorities and unproven claims

- **No production runtime change:** `HttpKernelState`, `reload_auth_from_config` (`status -> auth`), `NamespaceServiceDirectory`, `BoundedBlockingExecutor`, `C2ShadowController`, `FabricRoutingMode::ReferenceOnly`, `Cargo.toml`, and `Cargo.lock` are completely unmodified.
- **No dispatch or semantic authority:** `NonOperativeStartToken` cannot invoke `KernelService` or direct FABRIC execution. The C1 fixture continues to set `source_operation_admitted: true` solely as a test input.
- **Not a production cryptographic or time-authority proof:** Mode B's SHA-256 hash chain detects truncation and unauthenticated tampering without adding dependencies, but does not replace a keyed MAC/signature or external trusted time authority in production.
- **Future operative gates remain unapproved:**
  - Gate `O1`: production `AuthoritativeNamespaceLifecycle` API + lock-free-I/O three-phase `reload_auth_from_config` migration (default-OFF).
  - Gate `O2`: production default-OFF unified `SourceControlOwner` + authenticated durable ledger + production key/clock trust profile.
  - Gate `O3` (`E3B-A`): live FABRIC-directed routing cutover, reserved control-plane / Human Steward disposition, and separate practical workload acceptance `#133`.
- Single resource domain (`aether-local-blocking-pool`), seven excluded authoritative mutations, and closed C3 `#130` (22 positive + 1 negative-control `explain_tuple`) remain untouched.

---

## 3. Verification and protected readback tracking

Local verification covers Rust MSRV `1.89.0` and default Rust `1.94.0` (`cargo fmt`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, full `cargo test --workspace --locked`), `cargo audit`, Go tests, Python `pytest`, and pinned GH-OS routing preflight. Hosted PR gates (`Required CI gate`, `Required Supply Chain gate`, `policy / policy`, `security / action-policy`, `routing-enforcement`), three distinct read-only non-authoring logical review passes (Formalist, Adversary, Referee under `GCL-AGENT-STAFFING-001`), and exact protected `main` merge/blob readback are recorded on the implementing PR and in the `GCL-CONTRIBUTION-RESULT/1` report on `#148`.
