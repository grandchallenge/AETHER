//! #143: a test-only, explicitly *non-operative* model of one-attempt start.
//!
//! The source, authorization and worker-start transitions below are an
//! idealized single serialized state machine for hostile interleavings.
//! It is NOT connected to HttpKernelState, the C1 issuer, FABRIC, or the
//! BoundedBlockingExecutor. No public runtime type, issued permit, dispatch,
//! signature, serialized flag, or control-state claim is created here.
//!
//! Source-control inputs in this model are *simulated*; actual production
//! control registry, token, namespace and policy freshness are NOT proven.

use aether_fabric::AETHER_LOCAL_BLOCKING_POOL_ID;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceRevision {
    auth: u64,
    policy: u64,
    control: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EnvelopeState {
    Active,
    Revoked,
    Superseded,
}

#[derive(Clone, Debug)]
struct SimulatedSource {
    revision: SourceRevision,
    envelope: EnvelopeState,
    expiry_ms: u64,
    now_ms: u64,
    scope_query: bool,
    namespace_enabled: bool,
    available: bool,
}

impl Default for SimulatedSource {
    fn default() -> Self {
        Self {
            revision: SourceRevision {
                auth: 1,
                policy: 2,
                control: 3,
            },
            envelope: EnvelopeState::Active,
            expiry_ms: 500,
            now_ms: 100,
            scope_query: true,
            namespace_enabled: true,
            available: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct AttemptBinding {
    request: String,
    envelope: String,
    principal: String,
    token: String,
    namespace: String,
    operation_profile_digest: String,
    typed_payload_digest: String,
    effective_policy_digest: String,
    resource: String,
}

impl AttemptBinding {
    fn fixture() -> Self {
        Self {
            request: "request-1".into(),
            envelope: "E1".into(),
            principal: "principal:alice".into(),
            token: "token:alice".into(),
            namespace: "default".into(),
            operation_profile_digest: "1".repeat(64),
            typed_payload_digest: "2".repeat(64),
            effective_policy_digest: "3".repeat(64),
            resource: AETHER_LOCAL_BLOCKING_POOL_ID.into(),
        }
    }

    fn complete(&self) -> bool {
        [
            &self.request,
            &self.envelope,
            &self.principal,
            &self.token,
            &self.namespace,
            &self.operation_profile_digest,
            &self.typed_payload_digest,
            &self.effective_policy_digest,
        ]
        .iter()
        .all(|value| !value.is_empty())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Queued,
    WorkerAcquired,
    Started,
    Cancelled,
    Rejected,
    Completed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Refusal {
    MissingSource,
    InvalidBinding,
    WrongResource,
    DuplicateAttempt,
    DuplicateRequest,
    UnknownAttempt,
    BindingMismatch,
    InvalidPhase,
    Revoked,
    Superseded,
    Expired,
    ScopeDenied,
    NamespaceDenied,
    AuthReloaded,
    PolicyChanged,
    ControlRevisionChanged,
    StartedCannotBeCancelled,
}

#[derive(Clone, Debug)]
struct Entry {
    binding: AttemptBinding,
    admission_revision: SourceRevision,
    phase: Phase,
}

#[derive(Default)]
struct SimulatedAtomicStart {
    source: SimulatedSource,
    attempts: BTreeMap<String, Entry>,
    /// Protected C1 local profile max_attempts = 1: same logical
    /// request/envelope cannot be retried with a fresh attempt ID.
    request_keys: BTreeSet<(String, String)>,
    total_starts: u64,
}

impl SimulatedAtomicStart {
    fn source_check(&self) -> Result<(), Refusal> {
        let source = &self.source;
        if !source.available {
            return Err(Refusal::MissingSource);
        }
        match source.envelope {
            EnvelopeState::Active => {}
            EnvelopeState::Revoked => return Err(Refusal::Revoked),
            EnvelopeState::Superseded => return Err(Refusal::Superseded),
        }
        if source.now_ms >= source.expiry_ms {
            return Err(Refusal::Expired);
        }
        if !source.scope_query {
            return Err(Refusal::ScopeDenied);
        }
        if !source.namespace_enabled {
            return Err(Refusal::NamespaceDenied);
        }
        Ok(())
    }

    /// Queue admission is *not* mechanical permission. Stamping the source
    /// snapshot here creates no right to start after a source change.
    fn enqueue(&mut self, id: &str, binding: AttemptBinding) -> Result<(), Refusal> {
        self.source_check()?;
        if id.is_empty() || !binding.complete() {
            return Err(Refusal::InvalidBinding);
        }
        if binding.resource != AETHER_LOCAL_BLOCKING_POOL_ID {
            return Err(Refusal::WrongResource);
        }
        if self.attempts.contains_key(id) {
            return Err(Refusal::DuplicateAttempt);
        }
        let request_key = (binding.request.clone(), binding.envelope.clone());
        if !self.request_keys.insert(request_key) {
            return Err(Refusal::DuplicateRequest);
        }
        self.attempts.insert(
            id.into(),
            Entry {
                binding,
                admission_revision: self.source.revision.clone(),
                phase: Phase::Queued,
            },
        );
        Ok(())
    }

    /// Represents acquisition of the genuine executor worker slot, but only
    /// in the model. Work has not started: it can still be cancelled.
    fn worker_acquired(&mut self, id: &str) -> Result<(), Refusal> {
        let entry = self.attempts.get_mut(id).ok_or(Refusal::UnknownAttempt)?;
        if entry.phase != Phase::Queued {
            return Err(Refusal::InvalidPhase);
        }
        entry.phase = Phase::WorkerAcquired;
        Ok(())
    }

    /// Atomicity here follows solely from exclusive mutable access to this
    /// MODEL. A real start would need a unified AETHER-owned source+control
    /// transaction at the executor boundary; the actual locks are separate.
    fn commit_start(&mut self, id: &str, binding: &AttemptBinding) -> Result<(), Refusal> {
        let entry = self.attempts.get(id).ok_or(Refusal::UnknownAttempt)?;
        if &entry.binding != binding {
            return Err(Refusal::BindingMismatch);
        }
        if entry.phase != Phase::WorkerAcquired {
            return Err(Refusal::InvalidPhase);
        }
        let result = self.source_check().and_then(|()| {
            if entry.admission_revision.auth != self.source.revision.auth {
                return Err(Refusal::AuthReloaded);
            }
            if entry.admission_revision.policy != self.source.revision.policy {
                return Err(Refusal::PolicyChanged);
            }
            if entry.admission_revision.control != self.source.revision.control {
                return Err(Refusal::ControlRevisionChanged);
            }
            Ok(())
        });
        let entry = self.attempts.get_mut(id).expect("already checked");
        match result {
            Ok(()) => {
                entry.phase = Phase::Started;
                self.total_starts += 1;
                Ok(())
            }
            Err(cause) => {
                entry.phase = Phase::Rejected;
                Err(cause)
            }
        }
    }

    /// Cancellation before start consumes the attempt with no worker start.
    /// After start, the model preserves reference-style finish semantics.
    fn cancel(&mut self, id: &str) -> Result<(), Refusal> {
        let entry = self.attempts.get_mut(id).ok_or(Refusal::UnknownAttempt)?;
        match entry.phase {
            Phase::Queued | Phase::WorkerAcquired => {
                entry.phase = Phase::Cancelled;
                Ok(())
            }
            Phase::Started | Phase::Completed => Err(Refusal::StartedCannotBeCancelled),
            Phase::Cancelled | Phase::Rejected => Err(Refusal::InvalidPhase),
        }
    }

    fn complete(&mut self, id: &str) -> Result<(), Refusal> {
        let entry = self.attempts.get_mut(id).ok_or(Refusal::UnknownAttempt)?;
        if entry.phase != Phase::Started {
            return Err(Refusal::InvalidPhase);
        }
        entry.phase = Phase::Completed;
        Ok(())
    }

    fn phase(&self, id: &str) -> Phase {
        self.attempts[id].phase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ready() -> (SimulatedAtomicStart, AttemptBinding) {
        let mut m = SimulatedAtomicStart::default();
        let binding = AttemptBinding::fixture();
        m.enqueue("attempt-1", binding.clone()).unwrap();
        m.worker_acquired("attempt-1").unwrap();
        (m, binding)
    }

    #[test]
    fn successful_start_is_single_use_and_reference_work_may_finish() {
        let (mut m, binding) = ready();
        m.commit_start("attempt-1", &binding).unwrap();
        assert_eq!(m.total_starts, 1);
        assert_eq!(
            m.commit_start("attempt-1", &binding),
            Err(Refusal::InvalidPhase)
        );
        m.source.envelope = EnvelopeState::Revoked;
        m.source.revision.control += 1;
        assert_eq!(
            m.cancel("attempt-1"),
            Err(Refusal::StartedCannotBeCancelled)
        );
        m.complete("attempt-1").unwrap();
        assert_eq!(m.phase("attempt-1"), Phase::Completed);
        assert_eq!(m.total_starts, 1);
    }

    #[test]
    fn cancellation_queued_or_worker_acquired_consumes_attempt_without_start() {
        for acquired in [false, true] {
            let mut m = SimulatedAtomicStart::default();
            let binding = AttemptBinding::fixture();
            m.enqueue("attempt-1", binding.clone()).unwrap();
            if acquired {
                m.worker_acquired("attempt-1").unwrap();
            }
            m.cancel("attempt-1").unwrap();
            assert_eq!(m.phase("attempt-1"), Phase::Cancelled);
            assert_eq!(
                m.commit_start("attempt-1", &binding),
                Err(Refusal::InvalidPhase)
            );
            assert_eq!(
                m.enqueue("attempt-2", binding),
                Err(Refusal::DuplicateRequest)
            );
            assert_eq!(m.total_starts, 0);
        }
    }

    #[test]
    fn authority_revocation_supersession_and_expiry_before_start_all_deny() {
        for state in [EnvelopeState::Revoked, EnvelopeState::Superseded] {
            let (mut m, binding) = ready();
            m.source.envelope = state;
            m.source.revision.control += 1;
            let expected = match state {
                EnvelopeState::Revoked => Refusal::Revoked,
                EnvelopeState::Superseded => Refusal::Superseded,
                EnvelopeState::Active => unreachable!(),
            };
            assert_eq!(m.commit_start("attempt-1", &binding), Err(expected));
            assert_eq!(m.phase("attempt-1"), Phase::Rejected);
            assert_eq!(m.total_starts, 0);
        }
        let (mut m, binding) = ready();
        m.source.now_ms = m.source.expiry_ms;
        assert_eq!(m.commit_start("attempt-1", &binding), Err(Refusal::Expired));
        assert_eq!(m.total_starts, 0);
    }

    #[test]
    fn auth_policy_control_and_source_loss_at_worker_barrier_deny() {
        let changes: [fn(&mut SimulatedSource); 6] = [
            |s| s.revision.auth += 1,
            |s| s.revision.policy += 1,
            |s| s.revision.control += 1,
            |s| s.scope_query = false,
            |s| s.namespace_enabled = false,
            |s| s.available = false,
        ];
        let expected = [
            Refusal::AuthReloaded,
            Refusal::PolicyChanged,
            Refusal::ControlRevisionChanged,
            Refusal::ScopeDenied,
            Refusal::NamespaceDenied,
            Refusal::MissingSource,
        ];
        for (change, refusal) in changes.into_iter().zip(expected) {
            let (mut m, binding) = ready();
            change(&mut m.source);
            assert_eq!(m.commit_start("attempt-1", &binding), Err(refusal));
            assert_eq!(m.total_starts, 0);
        }
    }

    #[test]
    fn tampered_or_cross_identity_claims_fail_without_consuming_lease() {
        let changes: [fn(&mut AttemptBinding); 9] = [
            |b| b.request = "other".into(),
            |b| b.envelope = "other".into(),
            |b| b.principal = "principal:bob".into(),
            |b| b.token = "token:bob".into(),
            |b| b.namespace = "other".into(),
            |b| b.operation_profile_digest = "4".repeat(64),
            |b| b.typed_payload_digest = "5".repeat(64),
            |b| b.effective_policy_digest = "6".repeat(64),
            |b| b.resource = "second-worker-resource".into(),
        ];
        for change in changes {
            let (mut m, binding) = ready();
            let mut tampered = binding.clone();
            change(&mut tampered);
            assert_eq!(
                m.commit_start("attempt-1", &tampered),
                Err(Refusal::BindingMismatch)
            );
            assert_eq!(m.total_starts, 0);
            // The authentic same-attempt binding can still pass the model;
            // no tampered payload is permitted to mint an independent start.
            m.commit_start("attempt-1", &binding).unwrap();
            assert_eq!(m.total_starts, 1);
        }
    }

    #[test]
    fn attempts_are_nonreplayable_even_after_cancellation_or_denial() {
        let (mut m, binding) = ready();
        assert_eq!(
            m.enqueue("attempt-1", binding.clone()),
            Err(Refusal::DuplicateAttempt)
        );
        assert_eq!(
            m.enqueue("attempt-2", binding.clone()),
            Err(Refusal::DuplicateRequest)
        );
        m.source.revision.auth += 1;
        assert_eq!(
            m.commit_start("attempt-1", &binding),
            Err(Refusal::AuthReloaded)
        );
        assert_eq!(
            m.commit_start("attempt-1", &binding),
            Err(Refusal::InvalidPhase)
        );
        assert_eq!(
            m.enqueue("attempt-3", binding),
            Err(Refusal::DuplicateRequest)
        );
        assert_eq!(m.total_starts, 0);
    }

    #[test]
    fn no_one_can_start_from_a_queue_only_or_wrong_resource() {
        let mut m = SimulatedAtomicStart::default();
        let mut binding = AttemptBinding::fixture();
        binding.resource = "another-resource".into();
        assert_eq!(m.enqueue("wrong", binding), Err(Refusal::WrongResource));
        let binding = AttemptBinding::fixture();
        m.enqueue("attempt-1", binding.clone()).unwrap();
        assert_eq!(
            m.commit_start("attempt-1", &binding),
            Err(Refusal::InvalidPhase)
        );
        assert_eq!(m.total_starts, 0);
    }

    #[test]
    fn simulated_linearization_orders_revocation_relative_to_start() {
        // Model transitions are atomic under &mut self. Two explicit
        // schedules demonstrate which side of the start boundary matters.
        let (mut revoked_first, binding) = ready();
        revoked_first.source.envelope = EnvelopeState::Revoked;
        revoked_first.source.revision.control += 1;
        assert_eq!(
            revoked_first.commit_start("attempt-1", &binding),
            Err(Refusal::Revoked)
        );
        assert_eq!(revoked_first.total_starts, 0);

        let (mut started_first, binding) = ready();
        started_first.commit_start("attempt-1", &binding).unwrap();
        started_first.source.envelope = EnvelopeState::Revoked;
        started_first.source.revision.control += 1;
        started_first.complete("attempt-1").unwrap();
        assert_eq!(started_first.total_starts, 1);
    }

    #[test]
    fn incomplete_and_missing_attempts_never_start() {
        let mut m = SimulatedAtomicStart::default();
        let mut binding = AttemptBinding::fixture();
        binding.principal.clear();
        assert_eq!(m.enqueue("a1", binding), Err(Refusal::InvalidBinding));
        assert_eq!(m.worker_acquired("missing"), Err(Refusal::UnknownAttempt));
        assert_eq!(
            m.commit_start("missing", &AttemptBinding::fixture()),
            Err(Refusal::UnknownAttempt)
        );
    }
}
