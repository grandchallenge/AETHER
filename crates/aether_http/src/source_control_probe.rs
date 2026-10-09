//! In-process, non-operative recheck of AETHER-owned diagnostic observations.
//!
//! The source authentication generation is sampled under the same mutex as
//! authorization. This is *not* a lease, permit, stable revocation witness,
//! signature, serialization format, or assurance that a later check is fresh.
//! Matching this probe can never cause a FABRIC route or worker start.

use crate::source_binding::{SourceBoundObservation, SourceBoundReadback};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceControlProbeVerdict {
    DisabledOrUnavailable,
    EvidenceIncomplete,
    MissingAuthenticatedSource,
    SourceGenerationChanged,
    RecordMismatch,
    /// A historical record matched during the check. NOT an authorization,
    /// and not a freshness guarantee after the check releases the mutex.
    SnapshotMatchNonOperative,
}

pub(crate) fn verify_snapshot(
    current_auth_generation: u64,
    observations: Option<&SourceBoundReadback>,
    claim: &SourceBoundObservation,
) -> SourceControlProbeVerdict {
    let Some(readback) = observations else {
        return SourceControlProbeVerdict::DisabledOrUnavailable;
    };
    if readback.dropped_observations != 0 {
        return SourceControlProbeVerdict::EvidenceIncomplete;
    }
    if claim.token_ref.is_none()
        || claim.auth_generation_at_admission.is_none()
        || !claim.source_rate_and_namespace_admitted
        || !claim.reference_worker_started
        || !claim.semantic_result_succeeded
        || claim.authority_effect != "none"
        || claim.disposition != crate::source_binding::SOURCE_BOUND_DISPOSITION
        || claim.operation_class != aether_control_bridge::OperationClass::RunDocument
        || claim.http_method != "POST"
        || claim.http_path != "/v1/documents/run"
        || claim.required_scope != "query"
    {
        return SourceControlProbeVerdict::MissingAuthenticatedSource;
    }
    if claim.auth_generation_at_admission != Some(current_auth_generation) {
        return SourceControlProbeVerdict::SourceGenerationChanged;
    }
    if !readback.observations.iter().any(|record| record == claim) {
        return SourceControlProbeVerdict::RecordMismatch;
    }
    SourceControlProbeVerdict::SnapshotMatchNonOperative
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_control_bridge::OperationClass;

    fn record() -> SourceBoundObservation {
        SourceBoundObservation {
            request_id: "real-request".into(),
            operation_class: OperationClass::RunDocument,
            operation_profile_ref: "aether-http-op/run_document/1".into(),
            operation_profile_digest: "1".repeat(64),
            http_method: "POST",
            http_path: "/v1/documents/run",
            namespace_ref: "default".into(),
            principal_ref: "principal:alice".into(),
            token_ref: Some("token:alice".into()),
            required_scope: "query",
            request_payload_digest: "2".repeat(64),
            effective_policy_digest: "3".repeat(64),
            semantic_result_digest: "4".repeat(64),
            observed_at_unix_ms: 123,
            source_revision: None,
            auth_generation_at_admission: Some(7),
            disposition: crate::source_binding::SOURCE_BOUND_DISPOSITION,
            authority_effect: "none",
            source_rate_and_namespace_admitted: true,
            reference_worker_started: true,
            semantic_result_succeeded: true,
        }
    }

    fn evidence(record: SourceBoundObservation) -> SourceBoundReadback {
        SourceBoundReadback {
            observations: vec![record],
            dropped_observations: 0,
        }
    }

    #[test]
    fn adversarial_cross_identity_payload_policy_and_profile_are_not_source_matches() {
        let baseline = record();
        let readback = evidence(baseline.clone());
        assert_eq!(
            verify_snapshot(7, Some(&readback), &baseline),
            SourceControlProbeVerdict::SnapshotMatchNonOperative
        );
        let mutations: [fn(&mut SourceBoundObservation); 8] = [
            |x| x.request_id = "different".into(),
            |x| x.principal_ref = "principal:bob".into(),
            |x| x.token_ref = Some("token:bob".into()),
            |x| x.namespace_ref = "other".into(),
            |x| x.request_payload_digest = "5".repeat(64),
            |x| x.effective_policy_digest = "6".repeat(64),
            |x| x.operation_profile_digest = "7".repeat(64),
            |x| x.semantic_result_digest = "8".repeat(64),
        ];
        for change in mutations {
            let mut mutated = baseline.clone();
            change(&mut mutated);
            assert_eq!(
                verify_snapshot(7, Some(&readback), &mutated),
                SourceControlProbeVerdict::RecordMismatch
            );
        }
        // Identical replay merely yields the same historical diagnostic
        // membership; it NEVER mints or consumes an execution permit.
        assert_eq!(
            verify_snapshot(7, Some(&readback), &baseline),
            SourceControlProbeVerdict::SnapshotMatchNonOperative
        );
    }

    #[test]
    fn reload_missing_identity_and_evidence_loss_fail_closed_on_claims() {
        let baseline = record();
        let readback = evidence(baseline.clone());
        assert_eq!(
            verify_snapshot(8, Some(&readback), &baseline),
            SourceControlProbeVerdict::SourceGenerationChanged
        );
        assert_eq!(
            verify_snapshot(7, None, &baseline),
            SourceControlProbeVerdict::DisabledOrUnavailable
        );
        let mut incomplete = readback.clone();
        incomplete.dropped_observations = 1;
        assert_eq!(
            verify_snapshot(7, Some(&incomplete), &baseline),
            SourceControlProbeVerdict::EvidenceIncomplete
        );
        let mut anonymous = baseline.clone();
        anonymous.token_ref = None;
        assert_eq!(
            verify_snapshot(7, Some(&readback), &anonymous),
            SourceControlProbeVerdict::MissingAuthenticatedSource
        );
        let mut fake_grant = baseline;
        fake_grant.authority_effect = "mechanical";
        assert_eq!(
            verify_snapshot(7, Some(&readback), &fake_grant),
            SourceControlProbeVerdict::MissingAuthenticatedSource
        );
    }
}
