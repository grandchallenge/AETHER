//! AETHER-owned observations of *completed* source work, never authorization.
//!
//! The opt-in producer is called only from the authenticated real
//! `POST /v1/documents/run` semantic closure, after rate/namespace/worker
//! admission, successful effective policy binding and a real semantic result.
//! Readback is diagnostic only: no serializable issuer token, permission,
//! dispatch hook, current control witness, or FABRIC resource acquisition.

use aether_control_bridge::OperationClass;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

pub const SOURCE_BOUND_DISPOSITION: &str = "OBSERVED_NON_OPERATIVE";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceBoundObservation {
    pub request_id: String,
    pub operation_class: OperationClass,
    pub operation_profile_ref: String,
    pub operation_profile_digest: String,
    pub http_method: &'static str,
    pub http_path: &'static str,
    pub namespace_ref: String,
    pub principal_ref: String,
    pub token_ref: Option<String>,
    pub required_scope: &'static str,
    pub request_payload_digest: String,
    pub effective_policy_digest: String,
    pub semantic_result_digest: String,
    pub observed_at_unix_ms: u64,
    /// No revision is invented: the HTTP auth source exposes no reliable
    /// generation tied to this individual operation.
    pub source_revision: Option<String>,
    pub disposition: &'static str,
    pub authority_effect: &'static str,
    /// True only because the callback ran inside the actual executor after
    /// AETHER's rate and namespace admission. Not a reusable admission grant.
    pub source_rate_and_namespace_admitted: bool,
    pub reference_worker_started: bool,
    pub semantic_result_succeeded: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceBoundReadback {
    pub observations: Vec<SourceBoundObservation>,
    /// Lost evidence prevents any completeness claim.
    pub dropped_observations: u64,
}

pub(crate) struct SourceBindingPreview {
    evidence: Mutex<VecDeque<SourceBoundObservation>>,
    capacity: usize,
    dropped: AtomicU64,
}

impl SourceBindingPreview {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            evidence: Mutex::new(VecDeque::new()),
            capacity: capacity.max(1),
            dropped: AtomicU64::new(0),
        }
    }

    /// Nonblocking: preview errors cannot interrupt or redirect reference work.
    pub(crate) fn record(&self, observation: SourceBoundObservation) {
        let Ok(mut evidence) = self.evidence.try_lock() else {
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        };
        if evidence.len() == self.capacity {
            evidence.pop_front();
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
        evidence.push_back(observation);
    }

    /// A poisoned or contended store produces no readback, never a false proof.
    pub(crate) fn readback(&self) -> Option<SourceBoundReadback> {
        let evidence = self.evidence.try_lock().ok()?;
        Some(SourceBoundReadback {
            observations: evidence.iter().cloned().collect(),
            dropped_observations: self.dropped.load(Ordering::Relaxed),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observation(id: &str) -> SourceBoundObservation {
        SourceBoundObservation {
            request_id: id.into(),
            operation_class: OperationClass::RunDocument,
            operation_profile_ref: "aether-http-op/run_document/1".into(),
            operation_profile_digest: "1".repeat(64),
            http_method: "POST",
            http_path: "/v1/documents/run",
            namespace_ref: "default".into(),
            principal_ref: "source-principal".into(),
            token_ref: None,
            required_scope: "query",
            request_payload_digest: "2".repeat(64),
            effective_policy_digest: "3".repeat(64),
            semantic_result_digest: "4".repeat(64),
            observed_at_unix_ms: 1,
            source_revision: None,
            disposition: SOURCE_BOUND_DISPOSITION,
            authority_effect: "none",
            source_rate_and_namespace_admitted: true,
            reference_worker_started: true,
            semantic_result_succeeded: true,
        }
    }

    #[test]
    fn preview_is_bounded_and_lost_evidence_is_reported() {
        let preview = SourceBindingPreview::new(1);
        preview.record(observation("first"));
        preview.record(observation("second"));
        let readback = preview.readback().unwrap();
        assert_eq!(readback.observations.len(), 1);
        assert_eq!(readback.observations[0].request_id, "second");
        assert_eq!(readback.dropped_observations, 1);
        assert_eq!(readback.observations[0].authority_effect, "none");
    }

    #[test]
    fn contended_store_drops_without_blocking_or_minting_authority() {
        let preview = SourceBindingPreview::new(1);
        let _guard = preview.evidence.lock().unwrap();
        preview.record(observation("lost"));
        assert_eq!(preview.dropped.load(Ordering::Relaxed), 1);
        // Readback remains unavailable rather than lying about zero observations.
        assert!(preview.readback().is_none());
    }
}
