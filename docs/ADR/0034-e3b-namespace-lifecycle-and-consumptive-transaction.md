# ADR 0034 — Authoritative Namespace Lifecycle and Issuer-Bound Consumptive Start Transaction

**Status:** Candidate, bounded `#148` Tranche 2 preparation (NON-OPERATIVE TEST-ONLY). No live routing, semantic API, or authority change.
**Predecessor:** protected `main` `bd799745263d0f9ccde09b71be078640db800738` (following `#148` Tranche 1 / PR `#170` at `e637b6b3` and `#166` / PR `#171` at `bd799745`).
**Basis:** ADR 0029, ADR 0033, `#146`, `#148` (and `#148` PR `#170` completion note `NEXT:`); C3 classification `#130` closed through PRs `#165`/`#167`.

---

## 1. Purpose and strict non-operative boundary

ADR 0033 (`crates/aether_http/src/source_owner_actual.rs`) replaced simulated auth, token-default policy, namespace membership, and worker permits with genuine `HttpAuth`, validated file reload (`reload_auth_from_config`), `RateLimiter`, `admit_namespace`, and `BoundedBlockingExecutor` barriers combined with a fixture `AetherMechanicalAuthorityIssuer`. That tranche deliberately left five preparation items open before any separately governed operative delta may be proposed:

1. An explicit **authoritative namespace lifecycle contract** mapped against the real `NamespaceServiceDirectory`.
2. **Request-specific effective policy binding** (`bound_policy_context(&principal, requested)`) across validated auth reload narrowing/widening and payload/policy substitution.
3. A complete empirical **lock-order audit and deadlock-free three-phase reload migration proof** across all synchronization primitives in `HttpKernelState`.
4. A **non-replayable, issuer-bound `RouteRealized` attempt identity** with a non-serializable capability boundary, **two-mode process crash/restart policy** (ephemeral vs. tamper-evident hash-chained durable consumption ledger), **dual-clock expiry and rollback rejection**, and **cancellation vs. post-start completion linearization**.
5. An **exact-delta operative classification matrix** separating this `NO_ARTICLE_IX_AUTHORITY_CHANGE` preparation tranche from future Article IX / X / XI operative gates.

Everything implemented in `crates/aether_http/src/source_owner_transaction.rs` is compiled strictly under `#[cfg(test)]`. Production `HttpKernelState`, `HttpAuth`, `NamespaceServiceDirectory`, `BoundedBlockingExecutor`, `C2ShadowController`, `FabricRoutingMode::ReferenceOnly`, `Cargo.toml`, and `Cargo.lock` are unchanged.

---

## 2. Authoritative namespace lifecycle contract

### 2.1 Gap in current production `NamespaceServiceDirectory`

In current `crates/aether_http/src/http.rs` (lines 1984–2091):

- `NamespaceServiceDirectory` stores `services: Mutex<HashMap<NamespaceId, Arc<NamespaceServiceHandle>>>`.
- Each `NamespaceServiceHandle` holds `state: Mutex<NamespaceServiceState>` with only three variants:
  - `Uninitialized`
  - `Ready(Box<dyn KernelService + Send>)`
  - `Failed(String)`
- `NamespaceServiceDirectory::execute()` acquires `services`, clones the handle, drops `services`, acquires `handle.state`, lazily opens SQLite/Postgres inside `handle.state` if `Uninitialized`, and executes synchronous kernel work while holding `handle.state`.
- There is **no** namespace lifecycle revision counter, no `Draining`, `Quarantined`, or `Retired` state, and no journal/schema revocation epoch exposed independently of `HttpAuth`.

### 2.2 Prohibitions

- **Do not synthesize a namespace lifecycle epoch from `HttpAuth::generation`.** `HttpAuth::generation` tracks bearer-token/scope/namespace-grant configuration reloads only; a namespace can enter `Draining`, `Quarantined`, `Retired`, or `NamespaceServiceState::Failed` while `HttpAuth` is completely unchanged.
- **Do not treat `namespace_admission` `OwnedSemaphorePermit` as semantic or lifecycle permission.** It is a per-namespace concurrency capacity slot only.
- **Do not perform lazy `open_namespace_service()` (SQLite/Postgres I/O) inside any source-control owner lock.**

### 2.3 Contract state machine

Every namespace eligible for future consumptive start must expose an explicit, AETHER-owned `AuthoritativeNamespaceLifecycle` record with monotonic `revision: u64`:

| State | Eligible for new start? | Allowed transitions |
| --- | --- | --- |
| `Absent` / `Uninitialized` | No (fail closed: `NamespaceLifecycleMissing`) | `-> Active { revision: 1 }` |
| `Active { revision: r }` | Yes, **only** if `r == bound.namespace_lifecycle_revision` and real `NamespaceServiceDirectory` handle is not `Failed` / contended / poisoned | `-> Active { revision: r + 1 }` (rotation), `-> Draining { revision: r + 1 }`, `-> Quarantined { revision: r + 1 }`, `-> Retired { revision: r + 1 }` |
| `Draining { revision: r }` | No (fail closed: `NamespaceLifecycleDenied`); already-started work may complete | `-> Quarantined { revision: r + 1 }`, `-> Retired { revision: r + 1 }` |
| `Quarantined { revision: r }` | No (fail closed: `NamespaceLifecycleDenied`) | `-> Active { revision: r + 1 }` (explicit operator recovery), `-> Retired { revision: r + 1 }` |
| `Retired { revision: r }` | No (fail closed: `NamespaceLifecycleDenied`) | Terminal (all further transitions rejected) |

At the worker-acquired start barrier, the non-operative contract owner inspects **both**:

1. The real `NamespaceServiceDirectory` (`state.services.services.try_lock()` and, if a handle exists, `handle.state.try_lock()`): any `NamespaceServiceState::Failed(_)` denies with `NamespaceDirectoryFailed`, and any lock contention or poison denies fail-closed with `Unavailable` without ever invoking `open_namespace_service()`.
2. The authoritative lifecycle record (`AuthoritativeNamespaceLifecycle`): absent/uninitialized denies with `NamespaceLifecycleMissing`; non-`Active` or mismatched `revision` denies with `NamespaceLifecycleDenied`.

---

## 3. Request-specific effective policy binding

ADR 0033 bound only token-default policy (`bound_policy_context(&principal, None)`). Real operations such as `POST /v1/documents/run` accept an explicit `RunDocumentRequest.policy_context: Option<PolicyContext>`, which `crates/aether_http/src/http.rs` (`bound_policy_context`, lines 2563–2591) normalizes and verifies as `requested.subset_of(&granted)` when `principal.policy_bound` is true.

The contract binds **both** the canonical requested policy context (inside `typed_request_payload` and `payload_manifest_digest`) and the computed `effective_policy_digest`:

- **At prepare:** calls real `bound_policy_context(&principal, requested.clone())`. If `requested` exceeds the token grant, preparation fails closed (`HttpError::Forbidden`) and no bundle or attempt is created.
- **At the worker-acquired start barrier:** re-evaluates real `bound_policy_context(&current_principal, Some_or_None_requested)` under `HttpAuth`:
  - If a validated `reload_auth_from_config()` narrowed the token's policy grant so the original `requested` policy is no longer a subset, `bound_policy_context` returns `Err(HttpError::Forbidden)` and the transaction denies specifically with `PolicyEscalationDenied`.
  - If a reload widened or rotated the token's policy grant (advancing `HttpAuth::generation` even while `requested` remains a subset), generation mismatch denies with `SourceChanged`.
  - If the caller substitutes a different requested `PolicyContext` or payload bytes between prepare and start, the recomputed payload/effective-policy digest mismatches the issuer-bound attempt digest and denies with `IdentityMismatch`.

---

## 4. Empirical lock-order audit and deadlock-free migration proof

### 4.1 Complete current lock inventory in `crates/aether_http`

Audited directly against protected `main` (`bd799745`):

| Lock / Primitive | Location | Current nested edges | Held across `.await` or I/O? |
| --- | --- | --- | --- |
| `status: Mutex<ServiceStatusResponse>` | `http.rs:635, 679` | **`status -> auth`** in `reload_auth_from_config` (`http.rs:679 -> 696`) | No `.await`; file load/resolve happens before lock (`http.rs:675-677`) |
| `auth: Mutex<HttpAuth>` | `http.rs:402, 605, 647, 696` | `auth (try_lock) -> source_binding_preview.evidence (try_lock)` (`http.rs:402 -> 408`) | No `.await` or I/O |
| `rate_limiter.buckets: Mutex<...>` | `http.rs:271` | None (leaf) | No |
| `namespace_admission: Mutex<HashMap<..., Arc<Semaphore>>>` | `http.rs:616` | None (leaf; clones `Arc<Semaphore>` before `try_acquire_owned`) | No |
| `services.services: Mutex<HashMap<..., Arc<NamespaceServiceHandle>>>` | `http.rs:2053, 2087` | None (drops before `handle.state.lock()`) | No |
| `handle.state: Mutex<NamespaceServiceState>` | `http.rs:2066` | `handle.state -> source_binding_preview.evidence (try_lock)` in `run_document` (`http.rs:2066 -> 2078 -> 3466 -> source_binding.rs:75`) | **Yes**: held across lazy SQLite/Postgres open and synchronous `KernelService` work |
| `source_binding_preview.evidence: Mutex<VecDeque<SourceBoundObservation>>` | `source_binding.rs:59, 75, 88` | None (leaf; `try_lock` only) | No |
| `audit.entries: Mutex<...>` | `http.rs:2263, 2274, 2304, 2317, 2330` | None (leaf) | No (file I/O runs on dedicated writer thread outside lock) |
| `c2_shadow.issuer` / `c2_shadow.evidence` | `fabric_control_shadow.rs:156, 178, 192, 323` | **`c2_shadow.issuer -> c2_shadow.evidence`** (blocking nested edge: `observe_request` holds `issuer` from line `323` across `self.retain(base)` -> `evidence.lock()` at line `156`; acyclic because `evidence` is a leaf; Gate `O2` must preserve this order or drop `issuer` before calling `retain()`) | No `.await` or I/O (reads only atomic semaphore counts) |
| `blocking.admitted` / `blocking.workers` (`tokio::sync::Semaphore`) | `BoundedBlockingExecutor::run` | Async acquire `admitted -> workers`, then `spawn_blocking` | Held across `spawn_blocking` execution (not sync mutexes) |

### 4.2 Why naive insertion of a source-control owner deadlocks

Because `reload_auth_from_config()` currently holds `status` while acquiring `auth` (`http.rs:679 -> 696`), any naive design that introduces a unified `SourceControlOwner` enclosing `auth` and reads or updates `status` while holding `SourceControlOwner` creates a classic AB/BA cycle:

- Thread 1 (`reload_auth_from_config`): locks `status` -> waits for `SourceControlOwner` (`auth`)
- Thread 2 (owner path): locks `SourceControlOwner` -> waits for `status`

Similarly, acquiring `handle.state.lock()` (blocking) inside `SourceControlOwner` while a running worker holds `handle.state` and attempts to complete/report to `SourceControlOwner` would deadlock AND stall auth/C1 behind SQLite/Postgres I/O.

### 4.3 Deadlock-free prospective migration (`O1` / `O2` target)

1. **Structural immutability check without locking `status`:** In `http.rs:680-688`, `bind_addr`, `storage`, `transport`, and `configured_work_limits` never change after `HttpKernelState` construction. Store those four structural invariants in an immutable struct on `HttpKernelState` (just like `configured_work_limits` already is). Only `config_version`, `schema_version`, `principals`, and a monotone `published_auth_generation: u64` live inside `status`.
2. **Three-phase lock-free-I/O reload (zero nested mutexes):**
   - *Phase 1 (unlocked):* Load and resolve `PilotServiceConfig` from disk; verify structural invariants against the immutable startup descriptor.
   - *Phase 2 (single lock: `SourceControlOwner` only):* Commit `auth.replace_config(resolved.auth)`, capturing the new `generation`, and immediately drop `SourceControlOwner`.
   - *Phase 3 (single lock: `status` only):* Acquire `status`; update `config_version`, `schema_version`, `principals`, and `published_auth_generation` **only if** `new_generation > status.published_auth_generation` (monotone snapshot guard preventing out-of-order status regression under concurrent reloads); drop `status`.
3. **Three-step consumptive start with zero disk I/O under `SourceControlOwner`:**
   - Acquire `rate_limiter`, `namespace_admission`, `blocking.admitted`, and `blocking.workers` permits **outside** `SourceControlOwner`.
   - *Step 1 (`SourceControlOwner` RAM-only):* Revalidate `HttpAuth` grant, request-specific `bound_policy_context`, `AuthoritativeNamespaceLifecycle`, non-blocking `NamespaceServiceDirectory` health (`try_lock` only), C1 `EnvelopeControlRegistry` state/revision, dual-clock bounds, exact issuer-bound attempt digest, and ledger status (`Prepared`). Atomically transition attempt state `Prepared -> Committing` (the irrevocable start linearization point) and **drop `SourceControlOwner`**.
   - *Step 2 (`DurableLedgerWriter` only, Mode B):* Acquire only the dedicated append-only ledger writer mutex (never acquires `SourceControlOwner`, `auth`, `status`, `services`, or `C1`), assign `(seq, prev_hash, entry_hash)`, write the JSONL record, call `sync_all()`, and drop `DurableLedgerWriter`.
   - *Step 3 (`SourceControlOwner` RAM-only):* Promote `Committing -> ConsumedStarted` (or `Committing -> Denied` fail-closed if Step 2 returned an I/O error, issuing no token) and drop `SourceControlOwner`.
4. **Strict global lock partial order (provably acyclic):**
   - No mutex is ever acquired while holding `DurableLedgerWriter`.
   - Neither `SourceControlOwner`, `status`, `rate_limiter`, `namespace_admission`, `services.services`, `handle.state`, nor `audit.entries` is ever held across `.await`.
   - `SourceControlOwner` never performs file/network/DB I/O, `sync_all()`, lazy namespace initialization, selector/audit I/O, or semantic work.
   - In the non-operative `#[cfg(test)]` harness (where production `http.rs` is untouched and `auth` remains external to the test owner), `prepare()` checks owner identity before calling `state.auth.lock()` strictly **outside** `OwnerCore` (dropping it before locking `OwnerCore`), and while holding `OwnerCore`, the harness accesses external `state.auth`, `state.services.services`, and `handle.state` strictly via non-blocking `try_lock()`, so it cannot participate in any blocking cycle with unmodified `reload_auth_from_config` (`status -> auth`).

---

## 5. Attempt identity, capability boundary, restart, clock, and cancellation semantics

### 5.1 Canonical issuer-bound `RouteRealized` attempt digest

Every prepared attempt derives a canonical SHA-256 `AttemptBindingDigest` (`gcl-cjson-set-v1` canonical JSON) binding all 22 exact identity fields:

- Process & issuer provenance: `process_incarnation_id`, `issuer_ref`, `issuer_revision`, `issuer_build_digest`
- Admission & authorization identities: `operation_admission_id`, `mechanical_authorization_id`, `envelope_id`, `envelope_digest`, `control_revision`
- Operation & semantic binding: `operation_profile_digest`, `payload_manifest_digest`, `effective_policy_digest`
- Source subject & revisions: `namespace_ref`, `principal_ref`, `token_ref`, `auth_generation`, `namespace_lifecycle_revision`
- Realized placement & timing: `endpoint_id` (must equal `aether-local-blocking-pool`), `mechanical_attempt_id`, `correlation_id`, `issued_wall_ms`, `expires_at_unix_ms`

`PreparedAttempt` and `NonOperativeStartToken` are private, non-`Clone`, non-`Serialize`, non-`Deserialize` Rust types inside the test module. Deserializing a `RouteRealizedEvidence` JSON payload or a persisted ledger line cannot reconstruct a `PreparedAttempt` or mint a `NonOperativeStartToken`. `NonOperativeStartToken` is consumed by move on `complete_started()` and cannot invoke `KernelService` or change `FabricRoutingMode::ReferenceOnly`.

### 5.2 Two-mode process crash and restart policy

| Mode | Ledger storage | Behavior on restart (`new ProcessIncarnation`) |
| --- | --- | --- |
| **Mode A (Ephemeral)** | In-memory map only | Fresh random 128-bit `process_incarnation_id` is generated at startup; every pre-restart attempt is rejected fail-closed (`IncarnationMismatch`). Zero-resetting epochs to replay pre-crash attempts is forbidden. |
| **Mode B (Durable hash-chained JSONL)** | Append-only file on disk with `sync_all()` per transition; each entry commits `seq`, `prev_hash`, `incarnation_id`, `attempt_digest`, `event`, `wall_ms`, and `entry_hash = sha256(canonical_json(payload))` | Startup verifies trailing-`'\n'` termination (rejecting even 1-byte torn tails), the complete hash chain from genesis (`"0".repeat(64)`), strict sequential `seq`, and wall-clock high-water mark (`max_wall_ms`); any byte tamper, broken chain, or torn/truncated record fails closed at startup (`LedgerError::IntegrityFailed`). Mid-run appends verify the full on-disk file SHA-256 digest before appending so truncation or same-length byte tampering fails closed (`ConsumeVerdict::LedgerIntegrityFailed`) without issuing a token. Verified restart permanently rejects replays of prior `ConsumedStarted`/`Completed` (`AttemptAlreadyConsumed`), `Cancelled` (`AttemptCancelled`), and `Denied` (`AttemptAlreadyTerminal`) attempts, rejects unstarted pre-restart attempts (`IncarnationMismatch`), enforces persisted `high_water_wall_ms` against post-restart clock rollback, and permits fresh post-restart attempts under the new `ProcessIncarnation`. |

*Scope note:* The Mode B SHA-256 hash chain detects accidental truncation, torn tails, and unauthenticated file tampering without adding external crates. A production deployment facing an adversary with write access to the ledger filesystem additionally requires an external MAC/signature key or authenticated journal boundary (deferred to future operative gate `O2`).

### 5.3 Dual-clock expiry and rollback rejection

Start validation checks **both** clocks:

1. **Wall clock (`wall_ms`):**
   - Rejects `wall_ms < high_water_wall_ms` fail-closed already at `prepare` (`PrepareError::ClockRollback`, including after a Mode B restart with a regressed host clock) and rejects `wall_ms < issued_wall_ms` or `wall_ms < high_water_wall_ms` at `consume_start` (`ConsumeVerdict::ClockRollbackDenied`).
   - Passes `wall_ms` to real `EnvelopeControlRegistry::observe(&envelope_id, wall_ms)` before the local wall-expiry defense-in-depth check, requiring `ControlState::Active` at the exact issued `control_revision` and mapping `ControlState::Expired` (or `wall_ms >= expires_at_unix_ms`) to `ClockExpired`.
2. **Monotonic clock (`elapsed_ms`):**
   - Derives `ttl_ms` directly from the issued C1 bundle (`expires_at_unix_ms - issued_wall_ms`) and rejects `elapsed_ms >= ttl_ms` with `ClockExpired` even if the wall clock is frozen or held below `expires_at_unix_ms`.

*Scope note:* Local monotonic elapsed bounds plus persisted wall high-water marks detect local wall-clock freezes and regressions, but do not by themselves establish a cross-node cryptographic time authority.

### 5.4 Cancellation vs. post-start completion linearization

- **Before Step 1 (`Prepared`):** `cancel_before_start` (whether while queued behind a held worker or after worker acquisition before `consume_start`) atomically transitions `Prepared -> Cancelled` (and durably records `Cancelled` in Mode B). Any subsequent `consume_start` returns `AttemptCancelled` with `0` starts.
- **At/after Step 1 (`Committing` / `ConsumedStarted`):** `consume_start` is the irrevocable start linearization point. Subsequent `cancel_before_start` returns `Err(CancelError::CannotCancelStarted)`. Subsequent `C1Revoke`, `C1Supersede`, real `reload_auth_from_config` (even token revocation), or namespace `Quarantined`/`Draining` does **not** retroactively roll back already-started synchronous work; the running work finishes and transitions `ConsumedStarted -> Completed` (`starts == 1`), while any duplicate start attempt returns `AttemptAlreadyConsumed`.

---

## 6. Exact-delta operative classification matrix (Article IX / X / XI)

| Delta | Scope | Classification | Status in this PR |
| --- | --- | --- | --- |
| **This PR (`#148` Tranche 2)** | ADR 0034 + `#[cfg(test)]` `source_owner_transaction.rs` + docs/receipt only; zero production code, routing, API, journal, or dependency changes | **`NO_ARTICLE_IX_AUTHORITY_CHANGE`** (no Article X permission expansion, no Article XI amendment) | Implemented & verified |
| **Future Gate O1** | Production `AuthoritativeNamespaceLifecycle` API + three-phase lock-free-I/O `reload_auth_from_config` refactor (default-OFF, `ReferenceOnly` unchanged) | Requires fresh exact-source Article IX/X check (touches production `HttpKernelState` lock/lifecycle code, still no FABRIC dispatch) | **NOT implemented / NOT approved** |
| **Future Gate O2** | Production default-OFF unified `SourceControlOwner` + authenticated durable attempt ledger + production key/clock trust profile | Requires fresh exact-source Article IX/X/XI classification and security/clock-trust sign-off | **NOT implemented / NOT approved** |
| **Future Gate O3 (`E3B-A`)** | Live FABRIC-directed routing cutover + `#133` practical workload acceptance | Reserved control-plane / Human Steward disposition + Article XI where authority boundary shifts | **Explicitly withheld / unapproved** |

`FabricRoutingMode::ReferenceOnly`, the single `aether-local-blocking-pool` resource domain, the seven excluded authoritative mutation endpoints, closed C3 `#130` (22 positive + 1 negative-control `explain_tuple`), open `#133`, and withheld `E3B-A` remain unchanged.
