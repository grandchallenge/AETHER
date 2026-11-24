//! #148 Tranche 2: non-operative authoritative namespace lifecycle,
//! request-specific policy binding, lock-order safe 3-step consumption,
//! issuer-bound `RouteRealized` attempt identity, dual-clock expiry/rollback,
//! and Mode A/B restart ledger contract tests (ADR 0034).
//!
//! Strictly `#[cfg(test)]` — `NO_ARTICLE_IX_AUTHORITY_CHANGE`.
//! Real AETHER objects exercised: `HttpKernelState`, `HttpAuth`, validated
//! `reload_auth_from_config`, `bound_policy_context`, `NamespaceServiceDirectory`,
//! `RateLimiter`, `admit_namespace`, `BoundedBlockingExecutor`,
//! `AetherMechanicalAuthorityIssuer`, `EnvelopeControlRegistry`,
//! `fabric_shadow_compare_placement`, `realize_selected_placement`.
//! Non-operative contract fixtures (NOT wired into live routing):
//! `AuthoritativeNamespaceLifecycle`, `DurableLedgerWriter`, `ClockSource`,
//! `NonOperativeSourceOwner`, `PreparedAttempt`, `NonOperativeStartToken`.

use super::*;
use aether_control_bridge::{
    unix_ms_to_rfc3339, AetherMechanicalAuthorityIssuer, IssuerBuildIdentity,
    OperationAdmissionInput, OperationClass, PolicyBindingEvidence,
};
use aether_fabric::{
    canonicalize_serializable, realize_selected_placement, sha256_hex, ControlState,
    PlacementDecision, RealizationRequest, RouteRealizedEvidence, SelectorImplementationIdentity,
    AETHER_LOCAL_BLOCKING_POOL_ID,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Weak;
use std::time::{Duration, Instant};

const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn new_incarnation_id() -> String {
    format!("inc-{:032x}", rand::random::<u128>())
}

fn canonical_digest<T: Serialize>(value: &T) -> Option<String> {
    canonicalize_serializable(value)
        .ok()
        .map(|b| sha256_hex(&b))
}

fn effective_policy_digest(policy: Option<&PolicyContext>) -> Option<String> {
    match policy {
        Some(p) => canonical_digest(p),
        None => Some(sha256_hex(b"aether-public-policy/1")),
    }
}

#[derive(Clone)]
struct ControlledClock {
    state: Arc<Mutex<(u64, u64)>>,
}

impl ControlledClock {
    fn new(start_wall_ms: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new((start_wall_ms, 0))),
        }
    }

    fn set_wall_ms(&self, wall_ms: u64) {
        self.state.lock().unwrap().0 = wall_ms;
    }

    fn advance_wall_ms(&self, delta_ms: u64) {
        let mut guard = self.state.lock().unwrap();
        guard.0 = guard.0.saturating_add(delta_ms);
    }

    fn advance_monotonic_ms(&self, delta_ms: u64) {
        let mut guard = self.state.lock().unwrap();
        guard.1 = guard.1.saturating_add(delta_ms);
    }

    fn read(&self) -> (u64, u64) {
        *self.state.lock().unwrap()
    }
}

#[derive(Clone)]
enum ClockSource {
    System { origin: Instant },
    Controlled(ControlledClock),
}

impl ClockSource {
    fn system() -> Self {
        Self::System {
            origin: Instant::now(),
        }
    }

    fn read(&self) -> (u64, u64) {
        match self {
            Self::System { origin } => (
                now_millis(),
                origin.elapsed().as_millis().min(u128::from(u64::MAX)) as u64,
            ),
            Self::Controlled(clock) => clock.read(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NamespaceLifecycleState {
    Uninitialized,
    Active { revision: u64 },
    Draining { revision: u64 },
    Quarantined { revision: u64 },
    Retired { revision: u64 },
}

#[derive(Default)]
struct AuthoritativeNamespaceLifecycle {
    states: HashMap<NamespaceId, NamespaceLifecycleState>,
}

impl AuthoritativeNamespaceLifecycle {
    fn register_uninitialized(&mut self, ns: &NamespaceId) {
        self.states
            .entry(ns.clone())
            .or_insert(NamespaceLifecycleState::Uninitialized);
    }

    fn activate(&mut self, ns: &NamespaceId) -> Result<u64, &'static str> {
        let next = match self.states.get(ns).copied() {
            None | Some(NamespaceLifecycleState::Uninitialized) => 1,
            Some(NamespaceLifecycleState::Active { revision })
            | Some(NamespaceLifecycleState::Quarantined { revision }) => revision.saturating_add(1),
            Some(NamespaceLifecycleState::Draining { .. }) => {
                return Err("draining namespace cannot return directly to active")
            }
            Some(NamespaceLifecycleState::Retired { .. }) => {
                return Err("retired namespace is terminal")
            }
        };
        self.states.insert(
            ns.clone(),
            NamespaceLifecycleState::Active { revision: next },
        );
        Ok(next)
    }

    fn drain(&mut self, ns: &NamespaceId) -> Result<u64, &'static str> {
        match self.states.get(ns).copied() {
            Some(NamespaceLifecycleState::Active { revision }) => {
                let next = revision.saturating_add(1);
                self.states.insert(
                    ns.clone(),
                    NamespaceLifecycleState::Draining { revision: next },
                );
                Ok(next)
            }
            _ => Err("only active namespace can transition to draining"),
        }
    }

    fn quarantine(&mut self, ns: &NamespaceId) -> Result<u64, &'static str> {
        match self.states.get(ns).copied() {
            Some(NamespaceLifecycleState::Active { revision })
            | Some(NamespaceLifecycleState::Draining { revision }) => {
                let next = revision.saturating_add(1);
                self.states.insert(
                    ns.clone(),
                    NamespaceLifecycleState::Quarantined { revision: next },
                );
                Ok(next)
            }
            _ => Err("only active or draining namespace can transition to quarantined"),
        }
    }

    fn retire(&mut self, ns: &NamespaceId) -> Result<u64, &'static str> {
        match self.states.get(ns).copied() {
            Some(NamespaceLifecycleState::Active { revision })
            | Some(NamespaceLifecycleState::Draining { revision })
            | Some(NamespaceLifecycleState::Quarantined { revision }) => {
                let next = revision.saturating_add(1);
                self.states.insert(
                    ns.clone(),
                    NamespaceLifecycleState::Retired { revision: next },
                );
                Ok(next)
            }
            _ => Err("namespace cannot transition to retired from current state"),
        }
    }

    /// Test-only fault injection removing a non-retired lifecycle record to exercise
    /// `NamespaceLifecycleMissing`; never allowed to erase terminal `Retired`.
    fn fault_inject_clear_non_retired(&mut self, ns: &NamespaceId) -> Result<(), &'static str> {
        if matches!(
            self.states.get(ns),
            Some(NamespaceLifecycleState::Retired { .. })
        ) {
            return Err("retired namespace is terminal and cannot be cleared");
        }
        self.states.remove(ns);
        Ok(())
    }

    fn active_revision(&self, ns: &NamespaceId) -> Result<u64, ConsumeVerdict> {
        match self.states.get(ns).copied() {
            None | Some(NamespaceLifecycleState::Uninitialized) => {
                Err(ConsumeVerdict::NamespaceLifecycleMissing)
            }
            Some(NamespaceLifecycleState::Active { revision }) => Ok(revision),
            Some(
                NamespaceLifecycleState::Draining { .. }
                | NamespaceLifecycleState::Quarantined { .. }
                | NamespaceLifecycleState::Retired { .. },
            ) => Err(ConsumeVerdict::NamespaceLifecycleDenied),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirectoryHealth {
    HealthyOrUninitialized,
    Failed,
    Unavailable,
}

/// Non-blocking observation of the REAL `NamespaceServiceDirectory`.
/// Never calls `open_namespace_service()` or holds any lock across I/O.
fn inspect_namespace_directory(state: &HttpKernelState, ns: &NamespaceId) -> DirectoryHealth {
    let handle = {
        let Ok(services) = state.services.services.try_lock() else {
            return DirectoryHealth::Unavailable;
        };
        services.get(ns).cloned()
    };
    let Some(handle) = handle else {
        return DirectoryHealth::HealthyOrUninitialized;
    };
    let Ok(guard) = handle.state.try_lock() else {
        return DirectoryHealth::Unavailable;
    };
    match &*guard {
        NamespaceServiceState::Failed(_) => DirectoryHealth::Failed,
        NamespaceServiceState::Ready(_) | NamespaceServiceState::Uninitialized => {
            DirectoryHealth::HealthyOrUninitialized
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceGrant {
    generation: u64,
    principal: String,
    token: String,
    namespace: String,
    effective_policy_digest: String,
}

enum SourceGrantRead {
    Granted(SourceGrant),
    SourceDenied,
    PolicyEscalationDenied,
}

impl SourceGrant {
    /// Evaluates real `HttpAuth::authorize` and real `bound_policy_context(&principal, requested)`.
    fn read(
        auth: &HttpAuth,
        headers: &HeaderMap,
        requested_policy: Option<&PolicyContext>,
    ) -> SourceGrantRead {
        let Ok(ns) = namespace_from_headers(headers) else {
            return SourceGrantRead::SourceDenied;
        };
        let Ok(principal) = auth.authorize(headers, AuthScope::Query, &ns) else {
            return SourceGrantRead::SourceDenied;
        };
        let effective = match bound_policy_context(&principal, requested_policy.cloned()) {
            Ok(effective) => effective,
            Err(HttpError::Forbidden { .. }) => return SourceGrantRead::PolicyEscalationDenied,
            Err(_) => return SourceGrantRead::SourceDenied,
        };
        let (Some(principal_id), Some(token_id), Some(policy_digest)) = (
            principal.principal_id,
            principal.token_id,
            effective_policy_digest(effective.as_ref()),
        ) else {
            return SourceGrantRead::SourceDenied;
        };
        SourceGrantRead::Granted(Self {
            generation: principal.auth_generation,
            principal: principal_id,
            token: token_id,
            namespace: ns.to_string(),
            effective_policy_digest: policy_digest,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct AttemptBinding {
    process_incarnation_id: String,
    issuer_ref: String,
    issuer_revision: String,
    issuer_build_digest: String,
    operation_admission_id: String,
    mechanical_authorization_id: String,
    envelope_id: String,
    envelope_digest: String,
    control_revision: String,
    operation_profile_digest: String,
    payload_manifest_digest: String,
    effective_policy_digest: String,
    namespace_ref: String,
    principal_ref: String,
    token_ref: String,
    auth_generation: u64,
    namespace_lifecycle_revision: u64,
    endpoint_id: String,
    mechanical_attempt_id: String,
    correlation_id: String,
    issued_wall_ms: u64,
    expires_at_unix_ms: u64,
}

impl AttemptBinding {
    fn binding_digest(&self) -> String {
        canonical_digest(self).expect("attempt binding canonical digest")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum LedgerEvent {
    ConsumedStarted,
    Completed,
    Cancelled,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttemptStatus {
    Prepared,
    Committing,
    ConsumedStarted,
    Completed,
    Cancelled,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct LedgerRecordPayload {
    seq: u64,
    prev_hash: String,
    incarnation_id: String,
    attempt_digest: String,
    event: LedgerEvent,
    wall_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct LedgerRecord {
    #[serde(flatten)]
    payload: LedgerRecordPayload,
    entry_hash: String,
}

#[derive(Debug, Eq, PartialEq)]
enum LedgerError {
    IntegrityFailed(&'static str),
    IoFailed,
}

/// Mode B durable append-only SHA-256 hash-chained JSONL consumption ledger.
/// Held strictly OUTSIDE `OwnerCore` so `sync_all()` never blocks `HttpAuth`,
/// C1, lifecycle, or reload paths.
struct DurableLedgerWriter {
    path: PathBuf,
    next_seq: u64,
    tail_hash: String,
    high_water_wall_ms: u64,
    expected_file_sha256: String,
}

impl DurableLedgerWriter {
    fn open(path: PathBuf) -> Result<(Self, HashMap<String, AttemptStatus>, u64), LedgerError> {
        let mut attempts = HashMap::new();
        let mut next_seq = 1u64;
        let mut tail_hash = GENESIS_HASH.to_string();
        let mut high_water_wall_ms = 0u64;

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|_| LedgerError::IoFailed)?;
            }
            return Ok((
                Self {
                    path,
                    next_seq,
                    tail_hash,
                    high_water_wall_ms,
                    expected_file_sha256: sha256_hex(b""),
                },
                attempts,
                high_water_wall_ms,
            ));
        }

        let raw_bytes = fs::read(&path).map_err(|_| LedgerError::IoFailed)?;
        let expected_file_sha256 = sha256_hex(&raw_bytes);
        if !raw_bytes.is_empty() && !raw_bytes.ends_with(b"\n") {
            return Err(LedgerError::IntegrityFailed("unterminated trailing record"));
        }
        let text = std::str::from_utf8(&raw_bytes)
            .map_err(|_| LedgerError::IntegrityFailed("non-utf8 ledger bytes"))?;
        let trimmed = text.strip_suffix('\n').unwrap_or(text);
        if !raw_bytes.is_empty() {
            for line in trimmed.split('\n') {
                if line.is_empty() {
                    return Err(LedgerError::IntegrityFailed("empty ledger line"));
                }
                let record: LedgerRecord = serde_json::from_str(line)
                    .map_err(|_| LedgerError::IntegrityFailed("malformed json record"))?;
                if record.payload.seq != next_seq {
                    return Err(LedgerError::IntegrityFailed("non-monotonic sequence"));
                }
                if record.payload.prev_hash != tail_hash {
                    return Err(LedgerError::IntegrityFailed("broken prev_hash link"));
                }
                let recomputed = canonical_digest(&record.payload)
                    .ok_or(LedgerError::IntegrityFailed("canonicalization failed"))?;
                if recomputed != record.entry_hash {
                    return Err(LedgerError::IntegrityFailed("entry_hash mismatch"));
                }
                if record.payload.wall_ms < high_water_wall_ms {
                    return Err(LedgerError::IntegrityFailed("persisted wall_ms regressed"));
                }

                let next_status = match (
                    attempts.get(&record.payload.attempt_digest),
                    record.payload.event,
                ) {
                    (None, LedgerEvent::ConsumedStarted) => AttemptStatus::ConsumedStarted,
                    (None, LedgerEvent::Cancelled) => AttemptStatus::Cancelled,
                    (None, LedgerEvent::Denied) => AttemptStatus::Denied,
                    (Some(AttemptStatus::ConsumedStarted), LedgerEvent::Completed) => {
                        AttemptStatus::Completed
                    }
                    _ => {
                        return Err(LedgerError::IntegrityFailed(
                            "illegal attempt state transition in ledger",
                        ))
                    }
                };
                attempts.insert(record.payload.attempt_digest, next_status);
                high_water_wall_ms = record.payload.wall_ms;
                tail_hash = record.entry_hash;
                next_seq = next_seq.saturating_add(1);
            }
        }

        Ok((
            Self {
                path,
                next_seq,
                tail_hash,
                high_water_wall_ms,
                expected_file_sha256,
            },
            attempts,
            high_water_wall_ms,
        ))
    }

    fn append(
        &mut self,
        incarnation_id: &str,
        attempt_digest: &str,
        event: LedgerEvent,
        wall_ms: u64,
    ) -> Result<(), LedgerError> {
        let mut current_bytes = if self.path.exists() {
            fs::read(&self.path).map_err(|_| LedgerError::IoFailed)?
        } else {
            Vec::new()
        };
        if sha256_hex(&current_bytes) != self.expected_file_sha256 {
            return Err(LedgerError::IntegrityFailed(
                "external ledger modification detected",
            ));
        }
        let persisted_wall_ms = wall_ms.max(self.high_water_wall_ms);
        let payload = LedgerRecordPayload {
            seq: self.next_seq,
            prev_hash: self.tail_hash.clone(),
            incarnation_id: incarnation_id.to_string(),
            attempt_digest: attempt_digest.to_string(),
            event,
            wall_ms: persisted_wall_ms,
        };
        let entry_hash = canonical_digest(&payload)
            .ok_or(LedgerError::IntegrityFailed("canonicalization failed"))?;
        let record = LedgerRecord {
            payload,
            entry_hash: entry_hash.clone(),
        };
        let mut encoded = serde_json::to_string(&record)
            .map_err(|_| LedgerError::IntegrityFailed("json encode failed"))?;
        encoded.push('\n');

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|_| LedgerError::IoFailed)?;
        file.write_all(encoded.as_bytes())
            .map_err(|_| LedgerError::IoFailed)?;
        file.sync_all().map_err(|_| LedgerError::IoFailed)?;

        current_bytes.extend_from_slice(encoded.as_bytes());
        self.expected_file_sha256 = sha256_hex(&current_bytes);
        self.tail_hash = entry_hash;
        self.high_water_wall_ms = persisted_wall_ms;
        self.next_seq = self.next_seq.saturating_add(1);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrepareError {
    OwnerMismatch,
    SourceDenied,
    PolicyEscalation,
    RateLimited,
    NamespaceBusy,
    NamespaceLifecycleMissing,
    NamespaceLifecycleDenied,
    ClockRollback,
    OwnerUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CancelError {
    CannotCancelStarted,
    AlreadyCancelled,
    AlreadyDenied,
    UnknownAttempt,
    Unavailable,
    LedgerPersistenceFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConsumeVerdict {
    SourceDenied,
    SourceChanged,
    PolicyEscalationDenied,
    NamespaceLifecycleMissing,
    NamespaceLifecycleDenied,
    NamespaceDirectoryFailed,
    ControlInvalid,
    ClockRollbackDenied,
    ClockExpired,
    IdentityMismatch,
    IncarnationMismatch,
    AttemptAlreadyConsumed,
    AttemptCancelled,
    AttemptAlreadyTerminal,
    Unavailable,
    LedgerIntegrityFailed,
}

/// Private, non-Clone, non-Serialize/Deserialize attempt handle bound to one
/// exact issuer-verified `RouteRealizedEvidence` and `RunDocumentRequest`.
struct PreparedAttempt {
    binding: AttemptBinding,
    attempt_digest: String,
    input: OperationAdmissionInput,
    payload: RunDocumentRequest,
    payload_digest: String,
    route: RouteRealizedEvidence,
    issued_monotonic_ms: u64,
    ttl_ms: u64,
}

/// Private, non-Clone, non-Serialize/Deserialize non-operative token returned
/// only after Step 1-3 consume succeeds. Cannot call `KernelService` or mutate
/// `FabricRoutingMode::ReferenceOnly`.
struct NonOperativeStartToken {
    attempt_digest: String,
    incarnation_id: String,
    _non_operative_only: (),
}

struct OwnerCore {
    incarnation_id: String,
    auth_owner: Weak<Mutex<HttpAuth>>,
    workers: Weak<Semaphore>,
    issuer: AetherMechanicalAuthorityIssuer,
    lifecycle: AuthoritativeNamespaceLifecycle,
    attempts: HashMap<String, AttemptStatus>,
    high_water_wall_ms: u64,
    clock: ClockSource,
}

/// Non-operative 3-step contract source owner:
/// - `core` mutex performs RAM-only validation and state transitions (ZERO I/O).
/// - `ledger_writer` mutex (Mode B only) performs durable append + `sync_all()`
///   strictly outside `core`, `HttpAuth`, `status`, and `NamespaceServiceDirectory`.
struct NonOperativeSourceOwner {
    core: Arc<Mutex<OwnerCore>>,
    ledger_writer: Option<Arc<Mutex<DurableLedgerWriter>>>,
}

enum Step1Outcome {
    ReservedCommitting {
        wall_ms: u64,
    },
    DeniedRecorded {
        verdict: ConsumeVerdict,
        wall_ms: u64,
    },
    RejectedNoWrite(ConsumeVerdict),
}

impl NonOperativeSourceOwner {
    fn new_ephemeral(state: &HttpKernelState, ttl_ms: u64, clock: ClockSource) -> Self {
        Self::build(state, ttl_ms, clock, None).expect("ephemeral owner construction")
    }

    fn open_durable(
        state: &HttpKernelState,
        ttl_ms: u64,
        clock: ClockSource,
        ledger_path: PathBuf,
    ) -> Result<Self, LedgerError> {
        Self::build(state, ttl_ms, clock, Some(ledger_path))
    }

    fn build(
        state: &HttpKernelState,
        ttl_ms: u64,
        clock: ClockSource,
        ledger_path: Option<PathBuf>,
    ) -> Result<Self, LedgerError> {
        let (ledger_writer, attempts, high_water_wall_ms) = match ledger_path {
            Some(path) => {
                let (writer, recovered, persisted_high_water) = DurableLedgerWriter::open(path)?;
                (
                    Some(Arc::new(Mutex::new(writer))),
                    recovered,
                    persisted_high_water,
                )
            }
            None => (None, HashMap::new(), 0),
        };
        let issuer = AetherMechanicalAuthorityIssuer::for_c2_shadow(
            "source-148-tranche2-test",
            "test-only-rev1",
            IssuerBuildIdentity {
                source_commit: "test-only".into(),
                source_tree: "test-only".into(),
                artifact_sha256: "9".repeat(64),
            },
            ttl_ms,
            128,
        )
        .expect("fixture issuer construction");
        Ok(Self {
            core: Arc::new(Mutex::new(OwnerCore {
                incarnation_id: new_incarnation_id(),
                auth_owner: Arc::downgrade(&state.auth),
                workers: Arc::downgrade(&state.blocking.workers),
                issuer,
                lifecycle: AuthoritativeNamespaceLifecycle::default(),
                attempts,
                high_water_wall_ms,
                clock,
            })),
            ledger_writer,
        })
    }

    fn incarnation_id(&self) -> String {
        self.core.lock().unwrap().incarnation_id.clone()
    }

    fn register_uninitialized_namespace(&self, ns: &NamespaceId) {
        self.core
            .lock()
            .unwrap()
            .lifecycle
            .register_uninitialized(ns);
    }

    fn activate_namespace(&self, ns: &NamespaceId) -> Result<u64, &'static str> {
        self.core.lock().unwrap().lifecycle.activate(ns)
    }

    fn drain_namespace(&self, ns: &NamespaceId) -> Result<u64, &'static str> {
        self.core.lock().unwrap().lifecycle.drain(ns)
    }

    fn quarantine_namespace(&self, ns: &NamespaceId) -> Result<u64, &'static str> {
        self.core.lock().unwrap().lifecycle.quarantine(ns)
    }

    fn retire_namespace(&self, ns: &NamespaceId) -> Result<u64, &'static str> {
        self.core.lock().unwrap().lifecycle.retire(ns)
    }

    fn fault_inject_clear_non_retired_namespace(
        &self,
        ns: &NamespaceId,
    ) -> Result<(), &'static str> {
        self.core
            .lock()
            .unwrap()
            .lifecycle
            .fault_inject_clear_non_retired(ns)
    }

    fn observe_envelope_state(&self, envelope_id: &str) -> ControlState {
        let core = self.core.lock().unwrap();
        let (wall_ms, _) = core.clock.read();
        core.issuer
            .registry()
            .observe(envelope_id, wall_ms)
            .unwrap()
            .observed_state
    }

    fn revoke_envelope(&self, envelope_id: &str, reason: &str) {
        let mut core = self.core.lock().unwrap();
        let (wall_ms, _) = core.clock.read();
        let ts = wall_ms.max(core.high_water_wall_ms);
        core.issuer
            .revoke_envelope(envelope_id, ts, reason)
            .unwrap();
    }

    fn supersede_envelope(&self, prepared: &PreparedAttempt) {
        let mut core = self.core.lock().unwrap();
        let (wall_ms, _) = core.clock.read();
        let mut replacement = prepared.input.clone();
        replacement.request_id.push_str("-superseding");
        replacement.correlation_id.push_str("-superseding");
        replacement.decided_at_unix_ms = wall_ms.max(core.high_water_wall_ms);
        core.issuer
            .issue_superseding_control_bundle(&prepared.binding.envelope_id, &replacement)
            .unwrap();
    }

    fn prepare(
        &self,
        state: &HttpKernelState,
        headers: &HeaderMap,
        payload: RunDocumentRequest,
    ) -> Result<(PreparedAttempt, tokio::sync::OwnedSemaphorePermit), PrepareError> {
        // Verify exact owner identity BEFORE touching external state or consuming
        // rate-limit / namespace / C1 capacity.
        {
            let core = self
                .core
                .lock()
                .map_err(|_| PrepareError::OwnerUnavailable)?;
            if !Weak::ptr_eq(&core.auth_owner, &Arc::downgrade(&state.auth))
                || !Weak::ptr_eq(&core.workers, &Arc::downgrade(&state.blocking.workers))
            {
                return Err(PrepareError::OwnerMismatch);
            }
        }

        // Acquire external `state.auth` strictly OUTSIDE `OwnerCore` and drop before re-entering.
        let grant = {
            let auth = state
                .auth
                .lock()
                .map_err(|_| PrepareError::OwnerUnavailable)?;
            match SourceGrant::read(&auth, headers, payload.policy_context.as_ref()) {
                SourceGrantRead::Granted(grant) => grant,
                SourceGrantRead::SourceDenied => return Err(PrepareError::SourceDenied),
                SourceGrantRead::PolicyEscalationDenied => {
                    return Err(PrepareError::PolicyEscalation)
                }
            }
        };
        let ns = namespace_from_headers(headers).map_err(|_| PrepareError::SourceDenied)?;
        state
            .rate_limiter
            .admit(&ns, &grant.principal)
            .map_err(|_| PrepareError::RateLimited)?;
        let ns_permit = state
            .admit_namespace(&ns)
            .map_err(|_| PrepareError::NamespaceBusy)?;

        let mut core = self
            .core
            .lock()
            .map_err(|_| PrepareError::OwnerUnavailable)?;
        let ns_revision = match core.lifecycle.active_revision(&ns) {
            Ok(rev) => rev,
            Err(ConsumeVerdict::NamespaceLifecycleMissing) => {
                return Err(PrepareError::NamespaceLifecycleMissing)
            }
            Err(_) => return Err(PrepareError::NamespaceLifecycleDenied),
        };
        let (wall_ms, monotonic_ms) = core.clock.read();
        if wall_ms < core.high_water_wall_ms {
            return Err(PrepareError::ClockRollback);
        }
        let issued_wall_ms = wall_ms;
        core.high_water_wall_ms = issued_wall_ms;

        let profile = OperationClass::RunDocument.profile();
        let payload_value =
            serde_json::to_value(&payload).map_err(|_| PrepareError::OwnerUnavailable)?;
        let payload_digest =
            canonical_digest(&payload_value).ok_or(PrepareError::OwnerUnavailable)?;
        let input = OperationAdmissionInput {
            request_id: format!("tx-148-{:032x}", rand::random::<u128>()),
            correlation_id: format!("tx-148-corr-{:032x}", rand::random::<u128>()),
            operation_class: OperationClass::RunDocument,
            http_method: profile.http_method,
            http_path: profile.http_path,
            namespace_ref: grant.namespace.clone(),
            principal_ref: grant.principal.clone(),
            token_ref: Some(grant.token.clone()),
            required_scope: profile.required_scope,
            policy_binding: PolicyBindingEvidence::Bound {
                effective_policy_digest: grant.effective_policy_digest.clone(),
            },
            typed_request_payload: payload_value,
            // Real auth/policy/rate/namespace checks ran above, but this test
            // fixture does NOT execute semantic work or authorize live dispatch.
            source_operation_admitted: true,
            decision_revision: format!("auth:{}:ns:{}", grant.generation, ns_revision),
            decided_at_unix_ms: issued_wall_ms,
            source_authority_ref: "test-only-no-dispatch-148-tranche2".into(),
        };
        let bundle = core
            .issuer
            .issue_control_bundle(&input)
            .map_err(|_| PrepareError::OwnerUnavailable)?;
        core.issuer
            .verify_bundle(&bundle)
            .map_err(|_| PrepareError::OwnerUnavailable)?;
        if bundle.admission.effective_policy_digest != grant.effective_policy_digest {
            return Err(PrepareError::OwnerUnavailable);
        }
        // Derive monotonic TTL directly from the C1 bundle expiry so monotonic
        // and wall expiry can never diverge.
        let bound_ttl_ms = bundle
            .authorization
            .expires_at_unix_ms
            .saturating_sub(issued_wall_ms);
        if bound_ttl_ms == 0 {
            return Err(PrepareError::OwnerUnavailable);
        }
        let selector = SelectorImplementationIdentity {
            selector_id: "source-148-tranche2-test".into(),
            source_commit: "test-only".into(),
            source_tree: "test-only".into(),
            artifact_sha256: "8".repeat(64),
        };
        let comparison = state
            .fabric_shadow_compare_placement(
                &bundle.envelope_bytes,
                &bundle.placement_constraint,
                &bundle.control_state_witness,
                issued_wall_ms,
                &selector,
            )
            .map_err(|_| PrepareError::OwnerUnavailable)?;
        if !comparison.permitted_set_equal {
            return Err(PrepareError::OwnerUnavailable);
        }
        let PlacementDecision::Selected(selected) = comparison.fabric_decision else {
            return Err(PrepareError::OwnerUnavailable);
        };
        let occurred_at =
            unix_ms_to_rfc3339(issued_wall_ms).map_err(|_| PrepareError::OwnerUnavailable)?;
        let route = realize_selected_placement(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &selected,
            &comparison.resource_snapshot,
            &bundle.control_state_witness,
            &RealizationRequest {
                realization_request_id: format!("tx-realize:{}", input.request_id),
                realization_time_unix_ms: issued_wall_ms,
                occurred_at,
            },
        )
        .map_err(|_| PrepareError::OwnerUnavailable)?;
        if route.authority_effect != "none" || route.endpoint_id != AETHER_LOCAL_BLOCKING_POOL_ID {
            return Err(PrepareError::OwnerUnavailable);
        }

        let binding = AttemptBinding {
            process_incarnation_id: core.incarnation_id.clone(),
            issuer_ref: bundle.authorization.issuer_ref.clone(),
            issuer_revision: bundle.authorization.issuer_revision.clone(),
            issuer_build_digest: bundle.authorization.issuer_build_digest.clone(),
            operation_admission_id: bundle.admission.operation_admission_id.clone(),
            mechanical_authorization_id: bundle.authorization.mechanical_authorization_id.clone(),
            envelope_id: bundle.control_state_witness.envelope_id.clone(),
            envelope_digest: sha256_hex(&bundle.envelope_bytes),
            control_revision: bundle.control_state_witness.observed_revision.clone(),
            operation_profile_digest: bundle.admission.operation_profile_digest.clone(),
            payload_manifest_digest: bundle.authorization.payload_manifest_digest.clone(),
            effective_policy_digest: grant.effective_policy_digest,
            namespace_ref: grant.namespace,
            principal_ref: grant.principal,
            token_ref: grant.token,
            auth_generation: grant.generation,
            namespace_lifecycle_revision: ns_revision,
            endpoint_id: route.endpoint_id.clone(),
            mechanical_attempt_id: route.mechanical_attempt_id.clone(),
            correlation_id: route.correlation_id.clone(),
            issued_wall_ms,
            expires_at_unix_ms: bundle.authorization.expires_at_unix_ms,
        };
        let attempt_digest = binding.binding_digest();
        core.attempts
            .insert(attempt_digest.clone(), AttemptStatus::Prepared);

        Ok((
            PreparedAttempt {
                binding,
                attempt_digest,
                input,
                payload,
                payload_digest,
                route,
                issued_monotonic_ms: monotonic_ms,
                ttl_ms: bound_ttl_ms,
            },
            ns_permit,
        ))
    }

    fn cancel_before_start(&self, prepared: &PreparedAttempt) -> Result<(), CancelError> {
        let (incarnation_id, wall_ms) = {
            let mut core = self.core.lock().map_err(|_| CancelError::Unavailable)?;
            match core.attempts.get(&prepared.attempt_digest).copied() {
                Some(AttemptStatus::Prepared) => {
                    core.attempts
                        .insert(prepared.attempt_digest.clone(), AttemptStatus::Cancelled);
                    let (wall_ms, _) = core.clock.read();
                    let ts = wall_ms.max(core.high_water_wall_ms);
                    core.high_water_wall_ms = ts;
                    (core.incarnation_id.clone(), ts)
                }
                Some(
                    AttemptStatus::Committing
                    | AttemptStatus::ConsumedStarted
                    | AttemptStatus::Completed,
                ) => return Err(CancelError::CannotCancelStarted),
                Some(AttemptStatus::Cancelled) => return Err(CancelError::AlreadyCancelled),
                Some(AttemptStatus::Denied) => return Err(CancelError::AlreadyDenied),
                None => return Err(CancelError::UnknownAttempt),
            }
        };
        if let Some(writer) = &self.ledger_writer {
            let mut ledger = writer.lock().map_err(|_| CancelError::Unavailable)?;
            ledger
                .append(
                    &incarnation_id,
                    &prepared.attempt_digest,
                    LedgerEvent::Cancelled,
                    wall_ms,
                )
                .map_err(|_| CancelError::LedgerPersistenceFailed)?;
        }
        Ok(())
    }

    fn consume_start(
        &self,
        state: &HttpKernelState,
        headers: &HeaderMap,
        prepared: &PreparedAttempt,
        presented_route: &RouteRealizedEvidence,
        presented_payload: &RunDocumentRequest,
    ) -> Result<NonOperativeStartToken, ConsumeVerdict> {
        // Step 1: RAM-only validation and reservation under `core` (NO disk I/O).
        let (incarnation_id, step1) = {
            let mut core = self.core.lock().map_err(|_| ConsumeVerdict::Unavailable)?;
            let incarnation_id = core.incarnation_id.clone();
            let outcome = Self::step1_validate_and_reserve(
                &mut core,
                state,
                headers,
                prepared,
                presented_route,
                presented_payload,
            );
            (incarnation_id, outcome)
        };

        // Step 2: Optional durable append + sync_all() strictly OUTSIDE `core`.
        match step1 {
            Step1Outcome::RejectedNoWrite(verdict) => Err(verdict),
            Step1Outcome::DeniedRecorded { verdict, wall_ms } => {
                if let Some(writer) = &self.ledger_writer {
                    let mut ledger = writer.lock().map_err(|_| ConsumeVerdict::Unavailable)?;
                    ledger
                        .append(
                            &incarnation_id,
                            &prepared.attempt_digest,
                            LedgerEvent::Denied,
                            wall_ms,
                        )
                        .map_err(|_| ConsumeVerdict::LedgerIntegrityFailed)?;
                }
                Err(verdict)
            }
            Step1Outcome::ReservedCommitting { wall_ms } => {
                let persist_ok = if let Some(writer) = &self.ledger_writer {
                    match writer.lock() {
                        Ok(mut ledger) => ledger
                            .append(
                                &incarnation_id,
                                &prepared.attempt_digest,
                                LedgerEvent::ConsumedStarted,
                                wall_ms,
                            )
                            .is_ok(),
                        Err(_) => false,
                    }
                } else {
                    true
                };

                // Step 3: RAM-only promotion `Committing -> ConsumedStarted` or fail-closed `Denied`.
                let mut core = self.core.lock().map_err(|_| ConsumeVerdict::Unavailable)?;
                if persist_ok {
                    core.attempts.insert(
                        prepared.attempt_digest.clone(),
                        AttemptStatus::ConsumedStarted,
                    );
                    Ok(NonOperativeStartToken {
                        attempt_digest: prepared.attempt_digest.clone(),
                        incarnation_id,
                        _non_operative_only: (),
                    })
                } else {
                    core.attempts
                        .insert(prepared.attempt_digest.clone(), AttemptStatus::Denied);
                    Err(ConsumeVerdict::LedgerIntegrityFailed)
                }
            }
        }
    }

    fn step1_validate_and_reserve(
        core: &mut OwnerCore,
        state: &HttpKernelState,
        headers: &HeaderMap,
        prepared: &PreparedAttempt,
        presented_route: &RouteRealizedEvidence,
        presented_payload: &RunDocumentRequest,
    ) -> Step1Outcome {
        // 1. Check persisted / in-memory attempt status BEFORE incarnation so
        //    replays of pre-restart consumed/cancelled/denied attempts report
        //    exact ledger status, while unstarted pre-restart attempts fail
        //    closed with `IncarnationMismatch`.
        match core.attempts.get(&prepared.attempt_digest).copied() {
            Some(
                AttemptStatus::Committing
                | AttemptStatus::ConsumedStarted
                | AttemptStatus::Completed,
            ) => return Step1Outcome::RejectedNoWrite(ConsumeVerdict::AttemptAlreadyConsumed),
            Some(AttemptStatus::Cancelled) => {
                return Step1Outcome::RejectedNoWrite(ConsumeVerdict::AttemptCancelled)
            }
            Some(AttemptStatus::Denied) => {
                return Step1Outcome::RejectedNoWrite(ConsumeVerdict::AttemptAlreadyTerminal)
            }
            None => {
                if prepared.binding.process_incarnation_id != core.incarnation_id {
                    return Step1Outcome::RejectedNoWrite(ConsumeVerdict::IncarnationMismatch);
                }
                return Step1Outcome::RejectedNoWrite(ConsumeVerdict::IdentityMismatch);
            }
            Some(AttemptStatus::Prepared) => {}
        }

        let (wall_ms, monotonic_ms) = core.clock.read();
        let ledger_wall_ms = wall_ms.max(core.high_water_wall_ms);

        let deny = |core: &mut OwnerCore, verdict: ConsumeVerdict| -> Step1Outcome {
            core.high_water_wall_ms = ledger_wall_ms;
            core.attempts
                .insert(prepared.attempt_digest.clone(), AttemptStatus::Denied);
            Step1Outcome::DeniedRecorded {
                verdict,
                wall_ms: ledger_wall_ms,
            }
        };

        // 2. Incarnation & exact issuer-bound RouteRealized / payload / owner identity.
        if prepared.binding.process_incarnation_id != core.incarnation_id {
            return deny(core, ConsumeVerdict::IncarnationMismatch);
        }
        let presented_payload_digest = serde_json::to_value(presented_payload)
            .ok()
            .and_then(|v| canonical_digest(&v));
        if !Weak::ptr_eq(&core.auth_owner, &Arc::downgrade(&state.auth))
            || !Weak::ptr_eq(&core.workers, &Arc::downgrade(&state.blocking.workers))
            || presented_route != &prepared.route
            || presented_route.envelope_id != prepared.binding.envelope_id
            || presented_route.correlation_id != prepared.binding.correlation_id
            || presented_route.endpoint_id != AETHER_LOCAL_BLOCKING_POOL_ID
            || presented_route.authority_effect != "none"
            || presented_payload_digest.as_deref() != Some(prepared.payload_digest.as_str())
            || prepared.binding.binding_digest() != prepared.attempt_digest
        {
            return deny(core, ConsumeVerdict::IdentityMismatch);
        }

        // 3. Revalidate REAL `HttpAuth` + request-specific `bound_policy_context` via try_lock.
        let current_grant = {
            let Ok(auth) = state.auth.try_lock() else {
                return deny(core, ConsumeVerdict::Unavailable);
            };
            SourceGrant::read(&auth, headers, presented_payload.policy_context.as_ref())
        };
        let current = match current_grant {
            SourceGrantRead::Granted(grant) => grant,
            SourceGrantRead::SourceDenied => return deny(core, ConsumeVerdict::SourceDenied),
            SourceGrantRead::PolicyEscalationDenied => {
                return deny(core, ConsumeVerdict::PolicyEscalationDenied)
            }
        };
        if current.principal != prepared.binding.principal_ref
            || current.token != prepared.binding.token_ref
            || current.namespace != prepared.binding.namespace_ref
            || current.effective_policy_digest != prepared.binding.effective_policy_digest
            || current.generation != prepared.binding.auth_generation
        {
            return deny(core, ConsumeVerdict::SourceChanged);
        }

        // 4. Inspect REAL `NamespaceServiceDirectory` (try_lock only, no lazy init).
        let Ok(ns) = NamespaceId::new(prepared.binding.namespace_ref.clone()) else {
            return deny(core, ConsumeVerdict::IdentityMismatch);
        };
        match inspect_namespace_directory(state, &ns) {
            DirectoryHealth::HealthyOrUninitialized => {}
            DirectoryHealth::Failed => return deny(core, ConsumeVerdict::NamespaceDirectoryFailed),
            DirectoryHealth::Unavailable => return deny(core, ConsumeVerdict::Unavailable),
        }

        // 5. Revalidate authoritative namespace lifecycle contract state & exact revision.
        match core.lifecycle.active_revision(&ns) {
            Ok(rev) if rev == prepared.binding.namespace_lifecycle_revision => {}
            Ok(_) => return deny(core, ConsumeVerdict::NamespaceLifecycleDenied),
            Err(verdict) => return deny(core, verdict),
        }

        // 6. Dual-clock rollback and monotonic expiry validation.
        if wall_ms < prepared.binding.issued_wall_ms
            || wall_ms < core.high_water_wall_ms
            || monotonic_ms < prepared.issued_monotonic_ms
        {
            return deny(core, ConsumeVerdict::ClockRollbackDenied);
        }
        core.high_water_wall_ms = wall_ms;
        if monotonic_ms.saturating_sub(prepared.issued_monotonic_ms) >= prepared.ttl_ms {
            return deny(core, ConsumeVerdict::ClockExpired);
        }

        // 7. Revalidate REAL C1 `EnvelopeControlRegistry` state and exact revision
        //    (runs BEFORE local wall-expiry defense-in-depth so real C1 `ControlState::Expired`
        //    is exercised directly on wall expiry).
        let Ok(witness) = core
            .issuer
            .registry()
            .observe(&prepared.binding.envelope_id, wall_ms)
        else {
            return deny(core, ConsumeVerdict::ControlInvalid);
        };
        if witness.observed_state == ControlState::Expired
            || wall_ms >= prepared.binding.expires_at_unix_ms
        {
            return deny(core, ConsumeVerdict::ClockExpired);
        }
        if witness.observed_state != ControlState::Active
            || witness.observed_revision != prepared.binding.control_revision
        {
            return deny(core, ConsumeVerdict::ControlInvalid);
        }

        // 8. Irrevocable start linearization point (RAM reservation before durable flush).
        core.attempts
            .insert(prepared.attempt_digest.clone(), AttemptStatus::Committing);
        Step1Outcome::ReservedCommitting { wall_ms }
    }

    fn complete_started(&self, token: NonOperativeStartToken) -> Result<(), ConsumeVerdict> {
        let wall_ms = {
            let mut core = self.core.lock().map_err(|_| ConsumeVerdict::Unavailable)?;
            if token.incarnation_id != core.incarnation_id {
                return Err(ConsumeVerdict::IncarnationMismatch);
            }
            if core.attempts.get(&token.attempt_digest) != Some(&AttemptStatus::ConsumedStarted) {
                return Err(ConsumeVerdict::IdentityMismatch);
            }
            core.attempts
                .insert(token.attempt_digest.clone(), AttemptStatus::Completed);
            let (wall_ms, _) = core.clock.read();
            let ts = wall_ms.max(core.high_water_wall_ms);
            core.high_water_wall_ms = ts;
            ts
        };
        if let Some(writer) = &self.ledger_writer {
            let mut ledger = writer.lock().map_err(|_| ConsumeVerdict::Unavailable)?;
            ledger
                .append(
                    &token.incarnation_id,
                    &token.attempt_digest,
                    LedgerEvent::Completed,
                    wall_ms,
                )
                .map_err(|_| ConsumeVerdict::LedgerIntegrityFailed)?;
        }
        Ok(())
    }

    fn status_of(&self, attempt_digest: &str) -> Option<AttemptStatus> {
        self.core
            .lock()
            .unwrap()
            .attempts
            .get(attempt_digest)
            .copied()
    }
}

struct TransactionFixture {
    state: HttpKernelState,
    config: PilotServiceConfig,
    root: PathBuf,
    data_root: PathBuf,
    config_path: PathBuf,
    ledger_path: PathBuf,
    headers: HeaderMap,
}

impl TransactionFixture {
    fn new(workers: usize, queue: usize) -> Self {
        use crate::{
            PilotAuthConfig, PilotConcurrencyConfig, PilotHttpTransportConfig, PilotStorageConfig,
            PilotTokenConfig,
        };
        let root =
            std::env::temp_dir().join(format!("aether-148-t2-{:032x}", rand::random::<u128>()));
        fs::create_dir_all(&root).unwrap();
        let config_path = root.join("pilot-service.json");
        let data_root = root.join("data");
        let ledger_path = root.join("attempt-ledger.jsonl");
        let config = PilotServiceConfig {
            config_version: "source-148-t2".into(),
            schema_version: "v1".into(),
            service_mode: ServiceMode::SingleNode,
            bind_addr: "127.0.0.1:3000".into(),
            http_transport: PilotHttpTransportConfig::default(),
            concurrency: PilotConcurrencyConfig {
                namespace_workers: workers,
                namespace_queue: queue,
                audit_queue: 1024,
            },
            database_path: None,
            storage: Some(PilotStorageConfig::Sqlite {
                data_root: data_root.clone(),
            }),
            audit_log_path: Some(root.join("audit.jsonl")),
            auth: PilotAuthConfig {
                tokens: vec![PilotTokenConfig {
                    principal: "operator".into(),
                    principal_id: Some("principal:operator".into()),
                    token_id: Some("token:operator".into()),
                    scopes: vec![AuthScope::Query],
                    policy_context: Some(PolicyContext {
                        capabilities: vec!["read".into(), "explain".into()],
                        visibilities: vec!["staff".into(), "public".into()],
                    }),
                    token: Some("fixture-token".into()),
                    token_env: None,
                    token_file: None,
                    token_command: None,
                    namespaces: vec![NamespaceId::default()],
                    revoked: false,
                }],
                revoked_token_ids: Vec::new(),
                revoked_principal_ids: Vec::new(),
            },
        };
        fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        let resolved = config.clone().resolve(&config_path).unwrap();
        let state = HttpKernelState::with_sqlite_namespaces(
            data_root.clone(),
            HttpKernelOptions::new()
                .with_auth(resolved.auth.clone())
                .with_service_status(resolved.service_status())
                .with_auth_reload_config_path(config_path.clone())
                .with_namespace_work_limits(workers, queue),
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static("Bearer fixture-token"),
        );
        Self {
            state,
            config,
            root,
            data_root,
            config_path,
            ledger_path,
            headers,
        }
    }

    fn reload(&mut self, change: impl FnOnce(&mut PilotServiceConfig)) {
        change(&mut self.config);
        fs::write(&self.config_path, serde_json::to_vec(&self.config).unwrap()).unwrap();
        self.state.reload_auth_from_config().unwrap();
    }
}

impl Drop for TransactionFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn sample_request(policy_context: Option<PolicyContext>) -> RunDocumentRequest {
    RunDocumentRequest {
        dsl: crate::coordination_pilot_dsl("current", "goal task_ready(t)\n  keep t"),
        policy_context,
    }
}

fn narrowed_policy() -> PolicyContext {
    PolicyContext {
        capabilities: vec!["read".into()],
        visibilities: vec!["public".into()],
    }
}

async fn run_at_real_worker_barrier<M>(
    fixture: &mut TransactionFixture,
    owner: &Arc<NonOperativeSourceOwner>,
    payload: RunDocumentRequest,
    queued: bool,
    mutate: M,
) -> (Result<(), ConsumeVerdict>, usize)
where
    M: FnOnce(&mut TransactionFixture, &Arc<NonOperativeSourceOwner>, &PreparedAttempt),
{
    let state = fixture.state.clone();
    let (prepared, ns_permit) = owner
        .prepare(&state, &fixture.headers, payload.clone())
        .unwrap();
    let prepared = Arc::new(prepared);
    let held_worker = Arc::clone(&state.blocking.workers)
        .acquire_owned()
        .await
        .unwrap();

    let (acquired_tx, acquired_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let starts = Arc::new(AtomicUsize::new(0));

    let worker_owner = Arc::clone(owner);
    let worker_state = state.clone();
    let worker_headers = fixture.headers.clone();
    let worker_prepared = Arc::clone(&prepared);
    let worker_starts = Arc::clone(&starts);
    let route = prepared.route.clone();
    let task = tokio::spawn(async move {
        let executor = worker_state.blocking.clone();
        executor
            .run(move || {
                let _ns = ns_permit;
                acquired_tx.send(()).unwrap();
                release_rx.recv_timeout(Duration::from_secs(15)).unwrap();
                match worker_owner.consume_start(
                    &worker_state,
                    &worker_headers,
                    &worker_prepared,
                    &route,
                    &payload,
                ) {
                    Ok(token) => {
                        worker_starts.fetch_add(1, Ordering::AcqRel);
                        Ok(worker_owner.complete_started(token))
                    }
                    Err(verdict) => Ok(Err(verdict)),
                }
            })
            .await
    });

    tokio::time::timeout(Duration::from_secs(15), async {
        while state.blocking.admitted.available_permits() != 1 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("queued admission while worker held");
    assert_eq!(state.blocking.workers.available_permits(), 0);

    let mut mutate = Some(mutate);
    if queued {
        mutate.take().unwrap()(fixture, owner, &prepared);
    }
    drop(held_worker);
    tokio::time::timeout(Duration::from_secs(15), acquired_rx)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.blocking.workers.available_permits(), 0);
    if !queued {
        mutate.take().unwrap()(fixture, owner, &prepared);
    }
    release_tx.send(()).unwrap();

    let outcome = tokio::time::timeout(Duration::from_secs(15), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(state.blocking.workers.available_permits(), 1);
    assert_eq!(state.blocking.admitted.available_permits(), 2);
    assert_eq!(
        state.fabric_routing_mode(),
        FabricRoutingMode::ReferenceOnly
    );
    (outcome, starts.load(Ordering::Acquire))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn request_specific_policy_narrowing_widening_and_payload_substitution() {
    // 1. Escalated request fails closed at prepare before any bundle or attempt exists.
    {
        let fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            30_000,
            ClockSource::system(),
        ));
        owner.activate_namespace(&NamespaceId::default()).unwrap();
        let escalated = sample_request(Some(PolicyContext {
            capabilities: vec!["read".into(), "admin".into()],
            visibilities: vec!["public".into()],
        }));
        assert!(matches!(
            owner.prepare(&fixture.state, &fixture.headers, escalated),
            Err(PrepareError::PolicyEscalation)
        ));
    }

    // 2. Unchanged narrowed requested policy completes once inside real worker.
    {
        let mut fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(
            NonOperativeSourceOwner::open_durable(
                &fixture.state,
                30_000,
                ClockSource::system(),
                fixture.ledger_path.clone(),
            )
            .unwrap(),
        );
        owner.activate_namespace(&NamespaceId::default()).unwrap();
        let (result, starts) = run_at_real_worker_barrier(
            &mut fixture,
            &owner,
            sample_request(Some(narrowed_policy())),
            false,
            |_, _, _| {},
        )
        .await;
        assert_eq!(result, Ok(()));
        assert_eq!(starts, 1);
    }

    // 3. Reload narrowing token grant below requested policy -> PolicyEscalationDenied at both barriers.
    for queued in [true, false] {
        let mut fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            30_000,
            ClockSource::system(),
        ));
        owner.activate_namespace(&NamespaceId::default()).unwrap();
        let (result, starts) = run_at_real_worker_barrier(
            &mut fixture,
            &owner,
            sample_request(Some(narrowed_policy())),
            queued,
            |f, _, _| {
                f.reload(|c| {
                    c.auth.tokens[0].policy_context = Some(PolicyContext {
                        capabilities: vec!["read".into()],
                        visibilities: vec!["staff".into()],
                    });
                });
            },
        )
        .await;
        assert_eq!(result, Err(ConsumeVerdict::PolicyEscalationDenied));
        assert_eq!(starts, 0);
    }

    // 4. Reload widening token grant (generation change) -> SourceChanged.
    {
        let mut fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            30_000,
            ClockSource::system(),
        ));
        owner.activate_namespace(&NamespaceId::default()).unwrap();
        let (result, starts) = run_at_real_worker_barrier(
            &mut fixture,
            &owner,
            sample_request(Some(narrowed_policy())),
            false,
            |f, _, _| {
                f.reload(|c| {
                    c.auth.tokens[0].policy_context = Some(PolicyContext {
                        capabilities: vec!["read".into(), "explain".into(), "audit".into()],
                        visibilities: vec!["staff".into(), "public".into()],
                    });
                });
            },
        )
        .await;
        assert_eq!(result, Err(ConsumeVerdict::SourceChanged));
        assert_eq!(starts, 0);
    }

    // 5. Request policy/payload substitution between prepare and start -> IdentityMismatch.
    {
        let fixture = TransactionFixture::new(1, 1);
        let owner =
            NonOperativeSourceOwner::new_ephemeral(&fixture.state, 30_000, ClockSource::system());
        owner.activate_namespace(&NamespaceId::default()).unwrap();
        let (prepared, _permit) = owner
            .prepare(
                &fixture.state,
                &fixture.headers,
                sample_request(Some(narrowed_policy())),
            )
            .unwrap();
        let substituted = sample_request(Some(PolicyContext {
            capabilities: vec!["read".into(), "explain".into()],
            visibilities: vec!["staff".into()],
        }));
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &substituted,
                )
                .err(),
            Some(ConsumeVerdict::IdentityMismatch)
        );
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::AttemptAlreadyTerminal)
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn authoritative_namespace_lifecycle_and_real_directory_failure_at_both_barriers() {
    #[derive(Clone, Copy)]
    enum NsTransition {
        Drain,
        Quarantine,
        Retire,
        RotateRevision,
        ClearLifecycle,
        FailRealDirectory,
    }

    for queued in [true, false] {
        for (transition, expected) in [
            (
                NsTransition::Drain,
                ConsumeVerdict::NamespaceLifecycleDenied,
            ),
            (
                NsTransition::Quarantine,
                ConsumeVerdict::NamespaceLifecycleDenied,
            ),
            (
                NsTransition::Retire,
                ConsumeVerdict::NamespaceLifecycleDenied,
            ),
            (
                NsTransition::RotateRevision,
                ConsumeVerdict::NamespaceLifecycleDenied,
            ),
            (
                NsTransition::ClearLifecycle,
                ConsumeVerdict::NamespaceLifecycleMissing,
            ),
            (
                NsTransition::FailRealDirectory,
                ConsumeVerdict::NamespaceDirectoryFailed,
            ),
        ] {
            let mut fixture = TransactionFixture::new(1, 1);
            let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
                &fixture.state,
                30_000,
                ClockSource::system(),
            ));
            let ns = NamespaceId::default();
            owner.activate_namespace(&ns).unwrap();
            let (result, starts) = run_at_real_worker_barrier(
                &mut fixture,
                &owner,
                sample_request(Some(narrowed_policy())),
                queued,
                |f, o, _| match transition {
                    NsTransition::Drain => {
                        o.drain_namespace(&ns).unwrap();
                    }
                    NsTransition::Quarantine => {
                        o.quarantine_namespace(&ns).unwrap();
                    }
                    NsTransition::Retire => {
                        o.retire_namespace(&ns).unwrap();
                        assert!(o.activate_namespace(&ns).is_err());
                        assert!(o.fault_inject_clear_non_retired_namespace(&ns).is_err());
                    }
                    NsTransition::RotateRevision => {
                        o.activate_namespace(&ns).unwrap();
                    }
                    NsTransition::ClearLifecycle => {
                        o.fault_inject_clear_non_retired_namespace(&ns).unwrap();
                    }
                    NsTransition::FailRealDirectory => {
                        // Force a genuine SQLite initialization failure in the REAL
                        // `NamespaceServiceDirectory` by placing a directory at the
                        // default SQLite file path while HttpAuth & semaphore stay valid.
                        fs::create_dir_all(f.data_root.join("default.sqlite")).unwrap();
                        assert!(f.state.services.execute(&ns, |_| Ok(())).is_err());
                    }
                },
            )
            .await;
            assert_eq!(result, Err(expected), "queued={queued}");
            assert_eq!(starts, 0);
        }
    }

    // Preparing against an absent or explicitly Uninitialized lifecycle fails closed.
    let fixture = TransactionFixture::new(1, 1);
    let ns = NamespaceId::default();
    let owner =
        NonOperativeSourceOwner::new_ephemeral(&fixture.state, 30_000, ClockSource::system());
    assert!(matches!(
        owner.prepare(&fixture.state, &fixture.headers, sample_request(None)),
        Err(PrepareError::NamespaceLifecycleMissing)
    ));
    owner.register_uninitialized_namespace(&ns);
    assert!(matches!(
        owner.prepare(&fixture.state, &fixture.headers, sample_request(None)),
        Err(PrepareError::NamespaceLifecycleMissing)
    ));
    assert_eq!(owner.activate_namespace(&ns), Ok(1));
}

#[test]
fn issuer_bound_attempt_identity_tamper_replay_and_concurrent_start_race() {
    let fixture = TransactionFixture::new(1, 1);
    let other_fixture = TransactionFixture::new(1, 1);
    let owner = Arc::new(
        NonOperativeSourceOwner::open_durable(
            &fixture.state,
            30_000,
            ClockSource::system(),
            fixture.ledger_path.clone(),
        )
        .unwrap(),
    );
    owner.activate_namespace(&NamespaceId::default()).unwrap();

    // 0. Passing a foreign HttpKernelState to `prepare()` fails closed before touching
    //    rate/namespace/C1 resources (cannot bypass real_state admission).
    assert!(matches!(
        owner.prepare(
            &other_fixture.state,
            &other_fixture.headers,
            sample_request(Some(narrowed_policy())),
        ),
        Err(PrepareError::OwnerMismatch)
    ));

    // 1. Tampering any RouteRealized field or using a foreign HttpKernelState denies.
    for tamper in [
        "mechanical_attempt_id",
        "envelope_id",
        "correlation_id",
        "endpoint_id",
        "authority_effect",
        "foreign_state",
    ] {
        let (prepared, _permit) = owner
            .prepare(
                &fixture.state,
                &fixture.headers,
                sample_request(Some(narrowed_policy())),
            )
            .unwrap();
        let mut route = prepared.route.clone();
        let target_state = if tamper == "foreign_state" {
            &other_fixture.state
        } else {
            match tamper {
                "mechanical_attempt_id" => route.mechanical_attempt_id = "forged-attempt".into(),
                "envelope_id" => route.envelope_id = "forged-envelope".into(),
                "correlation_id" => route.correlation_id = "forged-correlation".into(),
                "endpoint_id" => route.endpoint_id = "foreign-pool".into(),
                "authority_effect" => route.authority_effect = "dispatch".into(),
                _ => unreachable!(),
            }
            &fixture.state
        };
        assert_eq!(
            owner
                .consume_start(
                    target_state,
                    &fixture.headers,
                    &prepared,
                    &route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::IdentityMismatch),
            "tamper={tamper}"
        );
        // Subsequent replay with the untampered route is permanently rejected.
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::AttemptAlreadyTerminal)
        );
    }

    // 2. Concurrent two-thread race on the exact same PreparedAttempt: strictly one wins.
    let (prepared, _permit) = owner
        .prepare(
            &fixture.state,
            &fixture.headers,
            sample_request(Some(narrowed_policy())),
        )
        .unwrap();
    let prepared = Arc::new(prepared);
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let wins = Arc::new(AtomicUsize::new(0));
    let duplicates = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let owner = Arc::clone(&owner);
            let state = fixture.state.clone();
            let headers = fixture.headers.clone();
            let prepared = Arc::clone(&prepared);
            let barrier = Arc::clone(&barrier);
            let wins = Arc::clone(&wins);
            let duplicates = Arc::clone(&duplicates);
            std::thread::spawn(move || {
                // Round-trip RouteRealizedEvidence through JSON to verify deserialized
                // copies still obey single-use consumption and cannot bypass the ledger.
                let json = serde_json::to_string(&prepared.route).unwrap();
                let deserialized_route: RouteRealizedEvidence =
                    serde_json::from_str(&json).unwrap();
                barrier.wait();
                match owner.consume_start(
                    &state,
                    &headers,
                    &prepared,
                    &deserialized_route,
                    &prepared.payload,
                ) {
                    Ok(token) => {
                        wins.fetch_add(1, Ordering::AcqRel);
                        owner.complete_started(token).unwrap();
                    }
                    Err(ConsumeVerdict::AttemptAlreadyConsumed) => {
                        duplicates.fetch_add(1, Ordering::AcqRel);
                    }
                    Err(other) => panic!("unexpected concurrent race verdict: {other:?}"),
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(wins.load(Ordering::Acquire), 1);
    assert_eq!(duplicates.load(Ordering::Acquire), 1);
    assert_eq!(
        owner.status_of(&prepared.attempt_digest),
        Some(AttemptStatus::Completed)
    );
}

#[test]
fn process_crash_restart_mode_a_ephemeral_and_mode_b_durable_hash_chained_ledger() {
    let fixture = TransactionFixture::new(1, 4);
    let ns = NamespaceId::default();

    // --- Mode A: Ephemeral restart denies ALL pre-restart attempts fail-closed ---
    {
        let owner_before =
            NonOperativeSourceOwner::new_ephemeral(&fixture.state, 30_000, ClockSource::system());
        owner_before.activate_namespace(&ns).unwrap();
        let (pre_restart, _p) = owner_before
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();

        let owner_after =
            NonOperativeSourceOwner::new_ephemeral(&fixture.state, 30_000, ClockSource::system());
        owner_after.activate_namespace(&ns).unwrap();
        assert_ne!(owner_before.incarnation_id(), owner_after.incarnation_id());
        assert_eq!(
            owner_after
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &pre_restart,
                    &pre_restart.route,
                    &pre_restart.payload,
                )
                .err(),
            Some(ConsumeVerdict::IncarnationMismatch)
        );
        let (fresh, _p2) = owner_after
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        let token = owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &fresh,
                &fresh.route,
                &fresh.payload,
            )
            .unwrap();
        owner_after.complete_started(token).unwrap();
    }

    // --- Mode B: Durable hash-chained JSONL ledger across restart ---
    let base_wall = now_millis();
    let clock_before = ControlledClock::new(base_wall);
    let owner_before = NonOperativeSourceOwner::open_durable(
        &fixture.state,
        30_000,
        ClockSource::Controlled(clock_before.clone()),
        fixture.ledger_path.clone(),
    )
    .unwrap();
    owner_before.activate_namespace(&ns).unwrap();

    // 1. Consumed & completed before restart.
    let (consumed_attempt, p1) = owner_before
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    clock_before.advance_wall_ms(50);
    clock_before.advance_monotonic_ms(50);
    let token = owner_before
        .consume_start(
            &fixture.state,
            &fixture.headers,
            &consumed_attempt,
            &consumed_attempt.route,
            &consumed_attempt.payload,
        )
        .unwrap();
    owner_before.complete_started(token).unwrap();
    drop(p1);

    // 2. Cancelled before restart.
    let (cancelled_attempt, p2) = owner_before
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    clock_before.advance_wall_ms(50);
    owner_before
        .cancel_before_start(&cancelled_attempt)
        .unwrap();
    drop(p2);

    // 3. Denied before restart (expired).
    let (denied_attempt, p3) = owner_before
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    clock_before.advance_wall_ms(40_000);
    clock_before.advance_monotonic_ms(40_000);
    assert_eq!(
        owner_before
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &denied_attempt,
                &denied_attempt.route,
                &denied_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::ClockExpired)
    );
    drop(p3);

    // 4. ConsumedStarted in-flight at crash (durable flush succeeded, never reached Completed).
    let (in_flight_attempt, p_inflight) = owner_before
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    let in_flight_token = owner_before
        .consume_start(
            &fixture.state,
            &fixture.headers,
            &in_flight_attempt,
            &in_flight_attempt.route,
            &in_flight_attempt.payload,
        )
        .unwrap();
    // Simulate process crash before `complete_started` runs.
    drop(in_flight_token);
    drop(p_inflight);

    // 5. Prepared but unstarted before crash.
    let (unstarted_attempt, p4) = owner_before
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    drop(p4);
    let prior_incarnation = owner_before.incarnation_id();
    drop(owner_before);

    // Restart with a rolled-back wall clock to verify persisted high-water rollback check.
    let rolled_back_clock = ControlledClock::new(base_wall + 10);
    let owner_after = NonOperativeSourceOwner::open_durable(
        &fixture.state,
        30_000,
        ClockSource::Controlled(rolled_back_clock.clone()),
        fixture.ledger_path.clone(),
    )
    .unwrap();
    owner_after.activate_namespace(&ns).unwrap();
    assert_ne!(owner_after.incarnation_id(), prior_incarnation);
    assert_eq!(
        owner_after.status_of(&in_flight_attempt.attempt_digest),
        Some(AttemptStatus::ConsumedStarted)
    );

    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &consumed_attempt,
                &consumed_attempt.route,
                &consumed_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::AttemptAlreadyConsumed)
    );
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &in_flight_attempt,
                &in_flight_attempt.route,
                &in_flight_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::AttemptAlreadyConsumed)
    );
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &cancelled_attempt,
                &cancelled_attempt.route,
                &cancelled_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::AttemptCancelled)
    );
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &denied_attempt,
                &denied_attempt.route,
                &denied_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::AttemptAlreadyTerminal)
    );
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &unstarted_attempt,
                &unstarted_attempt.route,
                &unstarted_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::IncarnationMismatch)
    );

    // Restarted owner refuses even to prepare a new attempt while wall clock is behind
    // the persisted durable ledger high-water mark.
    assert!(matches!(
        owner_after.prepare(&fixture.state, &fixture.headers, sample_request(None)),
        Err(PrepareError::ClockRollback)
    ));

    // Once wall clock catches up past persisted high-water mark, mid-flight rollback
    // between prepare and start still fails closed at consume_start.
    rolled_back_clock.set_wall_ms(base_wall + 50_000);
    let (rollback_attempt, p5) = owner_after
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    rolled_back_clock.set_wall_ms(base_wall + 49_999);
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &rollback_attempt,
                &rollback_attempt.route,
                &rollback_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::ClockRollbackDenied)
    );
    drop(p5);

    // Restoring wall clock past high-water mark allows a fresh post-restart attempt to succeed.
    rolled_back_clock.set_wall_ms(base_wall + 50_001);
    let (fresh_post_restart, p6) = owner_after
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    let token = owner_after
        .consume_start(
            &fixture.state,
            &fixture.headers,
            &fresh_post_restart,
            &fresh_post_restart.route,
            &fresh_post_restart.payload,
        )
        .unwrap();
    owner_after.complete_started(token).unwrap();
    drop(p6);

    // Mid-run same-length 1-byte tamper in the ledger fails Step 2 closed without issuing a token.
    let (same_len_tamper_attempt, p_same_len) = owner_after
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    let original_bytes = fs::read(&fixture.ledger_path).unwrap();
    let mut flipped_bytes = original_bytes.clone();
    assert!(!flipped_bytes.is_empty());
    let flip_idx = flipped_bytes.len() / 2;
    flipped_bytes[flip_idx] = if flipped_bytes[flip_idx] == b'a' {
        b'b'
    } else {
        b'a'
    };
    assert_eq!(flipped_bytes.len(), original_bytes.len());
    fs::write(&fixture.ledger_path, &flipped_bytes).unwrap();
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &same_len_tamper_attempt,
                &same_len_tamper_attempt.route,
                &same_len_tamper_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::LedgerIntegrityFailed)
    );
    assert_eq!(
        owner_after.status_of(&same_len_tamper_attempt.attempt_digest),
        Some(AttemptStatus::Denied)
    );
    drop(p_same_len);

    // Mid-run external ledger truncation also fails Step 2 closed without issuing a token.
    let (mid_run_tamper_attempt, p7) = owner_after
        .prepare(&fixture.state, &fixture.headers, sample_request(None))
        .unwrap();
    fs::write(
        &fixture.ledger_path,
        &original_bytes[..original_bytes.len().saturating_sub(12)],
    )
    .unwrap();
    assert_eq!(
        owner_after
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &mid_run_tamper_attempt,
                &mid_run_tamper_attempt.route,
                &mid_run_tamper_attempt.payload,
            )
            .err(),
        Some(ConsumeVerdict::LedgerIntegrityFailed)
    );
    assert_eq!(
        owner_after.status_of(&mid_run_tamper_attempt.attempt_digest),
        Some(AttemptStatus::Denied)
    );
    drop(p7);

    // Reopening a truncated ledger fails closed.
    assert!(matches!(
        NonOperativeSourceOwner::open_durable(
            &fixture.state,
            30_000,
            ClockSource::system(),
            fixture.ledger_path.clone(),
        ),
        Err(LedgerError::IntegrityFailed(_))
    ));

    // Reopening a 1-byte torn tail (missing trailing '\n') fails closed rather than
    // allowing a subsequent append to concatenate onto the last record.
    fs::write(
        &fixture.ledger_path,
        &original_bytes[..original_bytes.len().saturating_sub(1)],
    )
    .unwrap();
    assert_eq!(
        NonOperativeSourceOwner::open_durable(
            &fixture.state,
            30_000,
            ClockSource::system(),
            fixture.ledger_path.clone(),
        )
        .err(),
        Some(LedgerError::IntegrityFailed("unterminated trailing record"))
    );

    // Reopening a byte-tampered ledger fails closed.
    let mut tampered = String::from_utf8(original_bytes).unwrap();
    assert!(tampered.contains("consumed_started"));
    tampered = tampered.replacen("consumed_started", "cancelled", 1);
    fs::write(&fixture.ledger_path, tampered).unwrap();
    assert!(matches!(
        NonOperativeSourceOwner::open_durable(
            &fixture.state,
            30_000,
            ClockSource::system(),
            fixture.ledger_path.clone(),
        ),
        Err(LedgerError::IntegrityFailed(_))
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dual_clock_rollback_monotonic_expiry_and_real_ttl_expiry() {
    let fixture = TransactionFixture::new(1, 2);
    let ns = NamespaceId::default();
    let start_wall = now_millis();

    // 1. Wall-clock rollback denies fail-closed.
    {
        let clock = ControlledClock::new(start_wall);
        let owner = NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            30_000,
            ClockSource::Controlled(clock.clone()),
        );
        owner.activate_namespace(&ns).unwrap();
        let (prepared, _p) = owner
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        clock.set_wall_ms(start_wall - 1);
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::ClockRollbackDenied)
        );
    }

    // 2. Frozen wall clock + monotonic elapsed >= ttl denies fail-closed.
    {
        let clock = ControlledClock::new(start_wall);
        let owner = NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            500,
            ClockSource::Controlled(clock.clone()),
        );
        owner.activate_namespace(&ns).unwrap();
        let (prepared, _p) = owner
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        assert_eq!(prepared.ttl_ms, 500);
        // Wall clock stays completely frozen while monotonic elapsed hits TTL.
        clock.advance_monotonic_ms(500);
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::ClockExpired)
        );
    }

    // 3. Wall clock advances past expires_at_unix_ms while monotonic stays < TTL:
    //    real C1 `EnvelopeControlRegistry` observes `ControlState::Expired`.
    {
        let clock = ControlledClock::new(start_wall);
        let owner = NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            500,
            ClockSource::Controlled(clock.clone()),
        );
        owner.activate_namespace(&ns).unwrap();
        let (prepared, _p) = owner
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        clock.advance_wall_ms(500);
        assert_eq!(
            owner.observe_envelope_state(&prepared.binding.envelope_id),
            ControlState::Expired
        );
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::ClockExpired)
        );
    }

    // 4. Real system clock short-TTL expiry inside real BoundedBlockingExecutor.
    {
        let mut live_fixture = TransactionFixture::new(1, 1);
        let ttl_ms = 180;
        let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
            &live_fixture.state,
            ttl_ms,
            ClockSource::system(),
        ));
        owner.activate_namespace(&ns).unwrap();
        let (result, starts) = run_at_real_worker_barrier(
            &mut live_fixture,
            &owner,
            sample_request(Some(narrowed_policy())),
            false,
            |_, o, prepared| {
                std::thread::sleep(Duration::from_millis(ttl_ms + 30));
                assert_eq!(
                    o.observe_envelope_state(&prepared.binding.envelope_id),
                    ControlState::Expired
                );
            },
        )
        .await;
        assert_eq!(result, Err(ConsumeVerdict::ClockExpired));
        assert_eq!(starts, 0);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_before_start_and_post_start_completion_linearization() {
    let ns = NamespaceId::default();

    // 1. Pre-start cancel at queue barrier and at acquired-worker barrier blocks start.
    for queued in [true, false] {
        let mut fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(
            NonOperativeSourceOwner::open_durable(
                &fixture.state,
                30_000,
                ClockSource::system(),
                fixture.ledger_path.clone(),
            )
            .unwrap(),
        );
        owner.activate_namespace(&ns).unwrap();
        let (result, starts) = run_at_real_worker_barrier(
            &mut fixture,
            &owner,
            sample_request(Some(narrowed_policy())),
            queued,
            |_, o, prepared| {
                o.cancel_before_start(prepared).unwrap();
                assert_eq!(
                    o.cancel_before_start(prepared),
                    Err(CancelError::AlreadyCancelled)
                );
            },
        )
        .await;
        assert_eq!(result, Err(ConsumeVerdict::AttemptCancelled));
        assert_eq!(starts, 0);
    }

    // 2. Pre-start C1 revoke and supersede both deny at worker barrier.
    for is_supersede in [false, true] {
        let mut fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            30_000,
            ClockSource::system(),
        ));
        owner.activate_namespace(&ns).unwrap();
        let (result, starts) = run_at_real_worker_barrier(
            &mut fixture,
            &owner,
            sample_request(Some(narrowed_policy())),
            false,
            |_, o, prepared| {
                if is_supersede {
                    o.supersede_envelope(prepared);
                } else {
                    o.revoke_envelope(&prepared.binding.envelope_id, "pre-start revoke");
                }
            },
        )
        .await;
        assert_eq!(result, Err(ConsumeVerdict::ControlInvalid));
        assert_eq!(starts, 0);
    }

    // 3. Post-start revoke, supersede, real token-revoking reload, namespace quarantine,
    //    and late cancel do NOT roll back already-started synchronous work.
    for use_supersede in [false, true] {
        let mut fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(
            NonOperativeSourceOwner::open_durable(
                &fixture.state,
                30_000,
                ClockSource::system(),
                fixture.ledger_path.clone(),
            )
            .unwrap(),
        );
        owner.activate_namespace(&ns).unwrap();
        let payload = sample_request(Some(narrowed_policy()));
        let (prepared, ns_permit) = owner
            .prepare(&fixture.state, &fixture.headers, payload.clone())
            .unwrap();
        let prepared = Arc::new(prepared);

        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let (finish_tx, finish_rx) = mpsc::sync_channel(1);
        let starts = Arc::new(AtomicUsize::new(0));

        let worker_owner = Arc::clone(&owner);
        let worker_state = fixture.state.clone();
        let worker_headers = fixture.headers.clone();
        let worker_prepared = Arc::clone(&prepared);
        let worker_starts = Arc::clone(&starts);
        let task = tokio::spawn(async move {
            let executor = worker_state.blocking.clone();
            executor
                .run(move || {
                    let _ns = ns_permit;
                    let token = worker_owner
                        .consume_start(
                            &worker_state,
                            &worker_headers,
                            &worker_prepared,
                            &worker_prepared.route,
                            &payload,
                        )
                        .expect("start commits before post-start mutations");
                    worker_starts.fetch_add(1, Ordering::AcqRel);
                    started_tx.send(()).unwrap();
                    finish_rx.recv_timeout(Duration::from_secs(15)).unwrap();
                    worker_owner.complete_started(token).unwrap();
                    Ok(())
                })
                .await
        });

        tokio::time::timeout(Duration::from_secs(15), started_rx)
            .await
            .unwrap()
            .unwrap();

        // All hostile post-start mutations happen AFTER consume_start linearized:
        assert_eq!(
            owner.cancel_before_start(&prepared),
            Err(CancelError::CannotCancelStarted)
        );
        if use_supersede {
            owner.supersede_envelope(&prepared);
        } else {
            owner.revoke_envelope(&prepared.binding.envelope_id, "post-start revoke");
        }
        owner.quarantine_namespace(&ns).unwrap();
        fixture.reload(|c| c.auth.tokens[0].revoked = true);
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::AttemptAlreadyConsumed)
        );

        finish_tx.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(15), task)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(starts.load(Ordering::Acquire), 1);
        assert_eq!(
            owner.status_of(&prepared.attempt_digest),
            Some(AttemptStatus::Completed)
        );
        assert_eq!(
            fixture.state.fabric_routing_mode(),
            FabricRoutingMode::ReferenceOnly
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn lock_contention_poison_and_multithreaded_deadlock_freedom_stress() {
    let ns = NamespaceId::default();

    // 1. Real HttpAuth lock contention -> Unavailable (non-blocking try_lock).
    {
        let fixture = TransactionFixture::new(1, 1);
        let owner =
            NonOperativeSourceOwner::new_ephemeral(&fixture.state, 30_000, ClockSource::system());
        owner.activate_namespace(&ns).unwrap();
        let (prepared, _p) = owner
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        let guard = fixture.state.auth.lock().unwrap();
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::Unavailable)
        );
        drop(guard);
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::AttemptAlreadyTerminal)
        );
    }

    // 2. Real NamespaceServiceDirectory lock contention -> Unavailable (non-blocking try_lock).
    {
        let fixture = TransactionFixture::new(1, 1);
        let owner =
            NonOperativeSourceOwner::new_ephemeral(&fixture.state, 30_000, ClockSource::system());
        owner.activate_namespace(&ns).unwrap();
        let (prepared, _p) = owner
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        let guard = fixture.state.services.services.lock().unwrap();
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::Unavailable)
        );
        drop(guard);
    }

    // 3. Poisoned owner lock fails closed without affecting ReferenceOnly mode.
    {
        let fixture = TransactionFixture::new(1, 1);
        let owner = Arc::new(NonOperativeSourceOwner::new_ephemeral(
            &fixture.state,
            30_000,
            ClockSource::system(),
        ));
        owner.activate_namespace(&ns).unwrap();
        let (prepared, _p) = owner
            .prepare(&fixture.state, &fixture.headers, sample_request(None))
            .unwrap();
        let poison_core = Arc::clone(&owner.core);
        assert!(std::thread::spawn(move || {
            let _lock = poison_core.lock().unwrap();
            panic!("deliberate owner poison");
        })
        .join()
        .is_err());
        assert_eq!(
            owner
                .consume_start(
                    &fixture.state,
                    &fixture.headers,
                    &prepared,
                    &prepared.route,
                    &prepared.payload,
                )
                .err(),
            Some(ConsumeVerdict::Unavailable)
        );
        assert_eq!(
            fixture.state.fabric_routing_mode(),
            FabricRoutingMode::ReferenceOnly
        );
    }

    // 4. Multi-threaded deadlock-freedom stress test:
    //    Synchronized startup + concurrent real `reload_auth_from_config` (`status -> auth`),
    //    authoritative namespace lifecycle rotation, and real worker-barrier
    //    Mode B durable transactions complete inside a bounded timeout and leave
    //    a hash-chain-verifiable ledger on disk (zero scheduling-flake assumptions).
    let fixture = TransactionFixture::new(2, 8);
    let owner = Arc::new(
        NonOperativeSourceOwner::open_durable(
            &fixture.state,
            30_000,
            ClockSource::system(),
            fixture.ledger_path.clone(),
        )
        .unwrap(),
    );
    owner.activate_namespace(&ns).unwrap();

    let start_barrier = Arc::new(std::sync::Barrier::new(3));
    let stop = Arc::new(AtomicBool::new(false));
    let reload_state = fixture.state.clone();
    let reload_stop = Arc::clone(&stop);
    let reload_barrier = Arc::clone(&start_barrier);
    let reload_thread = std::thread::spawn(move || {
        reload_barrier.wait();
        let mut reloads = 0usize;
        while (!reload_stop.load(Ordering::Acquire) && reloads < 40) || reloads == 0 {
            reload_state.reload_auth_from_config().unwrap();
            reloads += 1;
            std::thread::yield_now();
        }
        reloads
    });

    let lifecycle_owner = Arc::clone(&owner);
    let lifecycle_stop = Arc::clone(&stop);
    let lifecycle_barrier = Arc::clone(&start_barrier);
    let lifecycle_thread = std::thread::spawn(move || {
        lifecycle_barrier.wait();
        let ns = NamespaceId::default();
        let mut transitions = 0usize;
        while (!lifecycle_stop.load(Ordering::Acquire) && transitions < 40) || transitions == 0 {
            let _ = lifecycle_owner.quarantine_namespace(&ns);
            let _ = lifecycle_owner.activate_namespace(&ns);
            transitions += 1;
            std::thread::yield_now();
        }
        transitions
    });

    // Ensure both mutator OS threads are active before launching worker tasks.
    start_barrier.wait();

    let completed_attempts = Arc::new(AtomicUsize::new(0));
    let denied_or_cancelled = Arc::new(AtomicUsize::new(0));
    let ledger_tx_attempts = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();
    for worker_idx in 0..2usize {
        let owner = Arc::clone(&owner);
        let state = fixture.state.clone();
        let headers = fixture.headers.clone();
        let completed_attempts = Arc::clone(&completed_attempts);
        let denied_or_cancelled = Arc::clone(&denied_or_cancelled);
        let ledger_tx_attempts = Arc::clone(&ledger_tx_attempts);
        workers.push(tokio::spawn(async move {
            for iter in 0..16usize {
                let payload = sample_request(Some(narrowed_policy()));
                let Ok((prepared, ns_permit)) = owner.prepare(&state, &headers, payload.clone())
                else {
                    denied_or_cancelled.fetch_add(1, Ordering::AcqRel);
                    tokio::task::yield_now().await;
                    continue;
                };
                if (iter + worker_idx).is_multiple_of(5) {
                    let _ = owner.cancel_before_start(&prepared);
                }
                let exec_owner = Arc::clone(&owner);
                let exec_state = state.clone();
                let exec_headers = headers.clone();
                let exec_completed = Arc::clone(&completed_attempts);
                let exec_denied = Arc::clone(&denied_or_cancelled);
                let exec_ledger = Arc::clone(&ledger_tx_attempts);
                state
                    .blocking
                    .run(move || {
                        let _ns = ns_permit;
                        exec_ledger.fetch_add(1, Ordering::AcqRel);
                        match exec_owner.consume_start(
                            &exec_state,
                            &exec_headers,
                            &prepared,
                            &prepared.route,
                            &payload,
                        ) {
                            Ok(token) => {
                                exec_owner.complete_started(token).unwrap();
                                exec_completed.fetch_add(1, Ordering::AcqRel);
                            }
                            Err(_) => {
                                exec_denied.fetch_add(1, Ordering::AcqRel);
                            }
                        }
                        Ok(())
                    })
                    .await
                    .unwrap();
            }
        }));
    }

    tokio::time::timeout(Duration::from_secs(15), async {
        for worker in workers {
            worker.await.unwrap();
        }
    })
    .await
    .expect("concurrent reload + lifecycle + worker stress must never deadlock");

    stop.store(true, Ordering::Release);
    let reload_count = reload_thread.join().unwrap();
    let lifecycle_count = lifecycle_thread.join().unwrap();
    assert!(reload_count > 0);
    assert!(lifecycle_count > 0);
    assert!(
        completed_attempts.load(Ordering::Acquire) + denied_or_cancelled.load(Ordering::Acquire)
            >= 32
    );

    // Guarantee at least one post-quiescence Mode B durable transaction even if every
    // chaotic iteration happened to collide with `Quarantined` during `prepare`.
    if ledger_tx_attempts.load(Ordering::Acquire) == 0 {
        let _ = owner.activate_namespace(&ns);
        let payload = sample_request(Some(narrowed_policy()));
        let (prepared, _ns_permit) = owner
            .prepare(&fixture.state, &fixture.headers, payload.clone())
            .unwrap();
        let token = owner
            .consume_start(
                &fixture.state,
                &fixture.headers,
                &prepared,
                &prepared.route,
                &payload,
            )
            .unwrap();
        owner.complete_started(token).unwrap();
    }

    // Reopen durable ledger from disk and confirm the entire hash chain verifies cleanly.
    let (_reopened, recovered_attempts, _high_water) =
        DurableLedgerWriter::open(fixture.ledger_path.clone()).unwrap();
    assert!(!recovered_attempts.is_empty());
    assert_eq!(
        fixture.state.fabric_routing_mode(),
        FabricRoutingMode::ReferenceOnly
    );
}
