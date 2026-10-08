use crate::{C2ShadowDisposition, C2ShadowEvidence};
use aether_control_bridge::OperationClass;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const C3_DIFFERENTIAL_REVISION: &str = "aether-http-c3-differential/1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum C3DifferentialVerdict {
    Equivalent,
    UnavailableEquivalent,
    Disagreement,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum C3DisagreementClass {
    ShadowFailure,
    ShadowRejection,
    OperationBindingMismatch,
    MissingIdentityBinding,
    PermittedSetMismatch,
    AuthorityEffect,
    ReferenceAuthorityLost,
    ProvenanceMismatch,
    ReferenceResultUnpaired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct C3DifferentialEvidence {
    pub revision: String,
    pub request_id: String,
    pub operation_class: Option<OperationClass>,
    pub method: String,
    pub path: String,
    pub namespace_ref: String,
    pub principal_ref: String,
    pub operation_admission_id: Option<String>,
    pub envelope_id: Option<String>,
    pub placement_decision_id: Option<String>,
    pub candidate_mechanical_attempt_id: Option<String>,
    pub decision_resource_snapshot_digest: Option<String>,
    pub fresh_control_witness_revision: Option<String>,
    pub fresh_resource_snapshot_digest: Option<String>,
    pub reference_permitted_resource_ids: Vec<String>,
    pub fabric_selected_resource_id: Option<String>,
    pub verdict: C3DifferentialVerdict,
    pub disagreement_class: Option<C3DisagreementClass>,
    pub authority_effect: String,
    pub reference_path_authoritative: bool,
    pub operation_profile_digest: Option<String>,
    pub issuer_build_digest: Option<String>,
    pub selector_build_digest: Option<String>,
    pub effective_policy_digest: Option<String>,
    pub request_payload_digest: Option<String>,
    pub reference_result_status: Option<u16>,
    pub reference_result_digest: Option<String>,
}

impl C3DifferentialEvidence {
    pub fn is_equivalent(&self) -> bool {
        matches!(
            self.verdict,
            C3DifferentialVerdict::Equivalent | C3DifferentialVerdict::UnavailableEquivalent
        )
    }
}

pub fn adjudicate_c3_observation(source: &C2ShadowEvidence) -> C3DifferentialEvidence {
    let mut out = C3DifferentialEvidence {
        revision: C3_DIFFERENTIAL_REVISION.into(),
        request_id: source.request_id.clone(),
        operation_class: source.operation_class,
        method: source.method.clone(),
        path: source.path.clone(),
        namespace_ref: source.namespace_ref.clone(),
        principal_ref: source.principal_ref.clone(),
        operation_admission_id: source.operation_admission_id.clone(),
        envelope_id: source.envelope_id.clone(),
        placement_decision_id: source.placement_decision_id.clone(),
        candidate_mechanical_attempt_id: source.candidate_mechanical_attempt_id.clone(),
        decision_resource_snapshot_digest: source.decision_resource_snapshot_digest.clone(),
        fresh_control_witness_revision: source.fresh_control_witness_revision.clone(),
        fresh_resource_snapshot_digest: source.fresh_resource_snapshot_digest.clone(),
        reference_permitted_resource_ids: source.reference_permitted_resource_ids.clone(),
        fabric_selected_resource_id: source.fabric_selected_resource_id.clone(),
        verdict: C3DifferentialVerdict::Disagreement,
        disagreement_class: None,
        authority_effect: source.authority_effect.clone(),
        reference_path_authoritative: source.reference_path_authoritative,
        operation_profile_digest: source.operation_profile_digest.clone(),
        issuer_build_digest: source.issuer_build_digest.clone(),
        selector_build_digest: source.selector_build_digest.clone(),
        effective_policy_digest: source.effective_policy_digest.clone(),
        request_payload_digest: source.request_payload_digest.clone(),
        reference_result_status: source.reference_result_status,
        reference_result_digest: source.reference_result_digest.clone(),
    };

    if source.authority_effect != "none" {
        out.disagreement_class = Some(C3DisagreementClass::AuthorityEffect);
        return out;
    }
    if !source.reference_path_authoritative {
        out.disagreement_class = Some(C3DisagreementClass::ReferenceAuthorityLost);
        return out;
    }

    let Some(operation) = source.operation_class else {
        out.disagreement_class = Some(C3DisagreementClass::OperationBindingMismatch);
        return out;
    };
    let profile = operation.profile();
    if source.method != profile.http_method || source.path != profile.http_path {
        out.disagreement_class = Some(C3DisagreementClass::OperationBindingMismatch);
        return out;
    }

    // Reject plausible-looking caller-produced records without exact profile,
    // digest and non-empty subject provenance. This is not a signature or a
    // substitute for capturing from the AETHER-owned controller.
    let valid_digest = |d: &Option<String>| {
        d.as_ref()
            .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
    };
    if source.request_id.is_empty()
        || source.namespace_ref.is_empty()
        || source.principal_ref.is_empty()
        || source.operation_profile_digest.as_ref() != profile.digest().ok().as_ref()
        || !valid_digest(&source.issuer_build_digest)
        || !valid_digest(&source.selector_build_digest)
        || !valid_digest(&source.effective_policy_digest)
        || !valid_digest(&source.request_payload_digest)
    {
        out.disagreement_class = Some(C3DisagreementClass::ProvenanceMismatch);
        return out;
    }

    match source.disposition {
        C2ShadowDisposition::ShadowFailed => {
            out.disagreement_class = Some(C3DisagreementClass::ShadowFailure);
            return out;
        }
        C2ShadowDisposition::ShadowRejected => {
            out.disagreement_class = Some(C3DisagreementClass::ShadowRejection);
            return out;
        }
        C2ShadowDisposition::CandidateUnavailable => {
            if source.operation_admission_id.is_none()
                || source.envelope_id.is_none()
                || source.decision_resource_snapshot_digest.is_none()
            {
                out.disagreement_class = Some(C3DisagreementClass::MissingIdentityBinding);
                return out;
            }
            if source.permitted_set_equal != Some(true)
                || source.differential_equivalent != Some(true)
                || !source.reference_permitted_resource_ids.is_empty()
                || source.fabric_selected_resource_id.is_some()
            {
                out.disagreement_class = Some(C3DisagreementClass::PermittedSetMismatch);
                return out;
            }
            out.verdict = C3DifferentialVerdict::UnavailableEquivalent;
            return out;
        }
        C2ShadowDisposition::CandidateValidated => {}
    }

    if source.operation_admission_id.is_none()
        || source.envelope_id.is_none()
        || source.placement_decision_id.is_none()
        || source.candidate_mechanical_attempt_id.is_none()
        || source.decision_resource_snapshot_digest.is_none()
        || source.fresh_control_witness_revision.is_none()
        || source.fresh_resource_snapshot_digest.is_none()
    {
        out.disagreement_class = Some(C3DisagreementClass::MissingIdentityBinding);
        return out;
    }

    if source.permitted_set_equal != Some(true) || source.differential_equivalent != Some(true) {
        out.disagreement_class = Some(C3DisagreementClass::PermittedSetMismatch);
        return out;
    }

    if source.reference_permitted_resource_ids.len() != 1
        || source.fabric_selected_resource_id.as_ref()
            != source.reference_permitted_resource_ids.first()
    {
        out.disagreement_class = Some(C3DisagreementClass::PermittedSetMismatch);
        return out;
    }

    out.verdict = C3DifferentialVerdict::Equivalent;
    out
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct C3ReplayBundle {
    pub revision: String,
    pub evicted_observations: u64,
    pub observations: Vec<C3DifferentialEvidence>,
}

pub fn adjudicate_c3_replay_bundle(
    bundle: &C3ReplayBundle,
) -> Result<C3CoverageSummary, C3CoverageError> {
    if bundle.revision != C3_DIFFERENTIAL_REVISION {
        return Err(C3CoverageError::InvalidCoverageEvidence);
    }
    if bundle.evicted_observations != 0 {
        return Err(C3CoverageError::EvidenceLoss);
    }
    adjudicate_c3_first_lane_coverage(&bundle.observations)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct C3CoverageSummary {
    pub revision: String,
    pub observed_operation_classes: usize,
    pub required_operation_classes: usize,
    pub exact_first_lane_coverage: bool,
    pub all_observations_equivalent: bool,
    pub authority_effect: String,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum C3CoverageError {
    #[error("C3 first-lane coverage is incomplete or contains an out-of-domain operation")]
    FirstLaneCoverageMismatch,
    #[error("C3 contains a differential disagreement")]
    DifferentialDisagreement,
    #[error("C3 evidence is missing a paired successful authoritative HTTP result")]
    ReferenceResultUnpaired,
    #[error("C3 coverage evidence has duplicate request identity or inconsistent provenance")]
    InvalidCoverageEvidence,
    #[error("C3 evidence retention dropped one or more observations")]
    EvidenceLoss,
}

pub fn adjudicate_c3_first_lane_coverage(
    observations: &[C3DifferentialEvidence],
) -> Result<C3CoverageSummary, C3CoverageError> {
    if observations.iter().any(|item| !item.is_equivalent()) {
        return Err(C3CoverageError::DifferentialDisagreement);
    }

    let observed = observations
        .iter()
        .filter_map(|item| item.operation_class)
        .collect::<BTreeSet<_>>();
    let required = OperationClass::ALL_FIRST_LANE
        .into_iter()
        .collect::<BTreeSet<_>>();

    if observed != required {
        return Err(C3CoverageError::FirstLaneCoverageMismatch);
    }

    // Caller-provided verdicts are not proof: independently recheck exact
    // profile, subject, build and digest shapes, resource-set projection and
    // reference-response pairings, including uniqueness.
    let mut request_ids = BTreeSet::new();
    for item in observations {
        let valid_digest = |d: &Option<String>| {
            d.as_ref()
                .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        };
        let Some(operation) = item.operation_class else {
            return Err(C3CoverageError::InvalidCoverageEvidence);
        };
        let profile = operation.profile();
        if !request_ids.insert(item.request_id.clone())
            || item.request_id.is_empty()
            || item.namespace_ref.is_empty()
            || item.principal_ref.is_empty()
            || item.method != profile.http_method
            || item.path != profile.http_path
            || item.operation_profile_digest.as_ref() != profile.digest().ok().as_ref()
            || !valid_digest(&item.issuer_build_digest)
            || !valid_digest(&item.selector_build_digest)
            || !valid_digest(&item.effective_policy_digest)
            || !valid_digest(&item.request_payload_digest)
            || !valid_digest(&item.decision_resource_snapshot_digest)
            || item.authority_effect != "none"
            || !item.reference_path_authoritative
            || item.verdict == C3DifferentialVerdict::Disagreement
            || item.disagreement_class.is_some()
        {
            return Err(C3CoverageError::InvalidCoverageEvidence);
        }
        if item.reference_result_status != Some(200) || !valid_digest(&item.reference_result_digest)
        {
            return Err(C3CoverageError::ReferenceResultUnpaired);
        }
        if item.verdict == C3DifferentialVerdict::Equivalent
            && (item.reference_permitted_resource_ids.len() != 1
                || item.fabric_selected_resource_id.as_ref()
                    != item.reference_permitted_resource_ids.first()
                || item.placement_decision_id.is_none()
                || item.candidate_mechanical_attempt_id.is_none()
                || item.fresh_control_witness_revision.is_none()
                || !valid_digest(&item.fresh_resource_snapshot_digest))
        {
            return Err(C3CoverageError::InvalidCoverageEvidence);
        }
        if item.verdict == C3DifferentialVerdict::UnavailableEquivalent
            && (!item.reference_permitted_resource_ids.is_empty()
                || item.fabric_selected_resource_id.is_some())
        {
            return Err(C3CoverageError::InvalidCoverageEvidence);
        }
    }

    Ok(C3CoverageSummary {
        revision: C3_DIFFERENTIAL_REVISION.into(),
        observed_operation_classes: observed.len(),
        required_operation_classes: required.len(),
        exact_first_lane_coverage: true,
        all_observations_equivalent: true,
        authority_effect: "none".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_fabric::AETHER_LOCAL_BLOCKING_POOL_ID;

    fn source(operation: OperationClass) -> C2ShadowEvidence {
        let profile = operation.profile();
        C2ShadowEvidence {
            request_id: format!("request:{}", operation.as_str()),
            method: profile.http_method,
            path: profile.http_path,
            namespace_ref: "default".into(),
            principal_ref: "test-principal".into(),
            operation_class: Some(operation),
            operation_admission_id: Some(format!("admission:{}", operation.as_str())),
            envelope_id: Some(format!("envelope:{}", operation.as_str())),
            placement_decision_id: Some(format!("placement:{}", operation.as_str())),
            candidate_mechanical_attempt_id: Some(format!("attempt:{}", operation.as_str())),
            permitted_set_equal: Some(true),
            decision_resource_snapshot_digest: Some("1".repeat(64)),
            fresh_control_witness_revision: Some("rev-2".into()),
            fresh_resource_snapshot_digest: Some("2".repeat(64)),
            reference_permitted_resource_ids: vec![AETHER_LOCAL_BLOCKING_POOL_ID.into()],
            fabric_selected_resource_id: Some(AETHER_LOCAL_BLOCKING_POOL_ID.into()),
            differential_equivalent: Some(true),
            disposition: C2ShadowDisposition::CandidateValidated,
            detail: None,
            authority_effect: "none".into(),
            reference_path_authoritative: true,
            operation_profile_digest: operation.profile().digest().ok(),
            issuer_build_digest: Some("a".repeat(64)),
            selector_build_digest: Some("b".repeat(64)),
            effective_policy_digest: Some("c".repeat(64)),
            request_payload_digest: Some("d".repeat(64)),
            reference_result_status: None,
            reference_result_digest: None,
        }
    }

    #[test]
    fn exact_23_operation_lane_closes_structurally() {
        let observations = OperationClass::ALL_FIRST_LANE
            .into_iter()
            .map(source)
            .map(|item| adjudicate_c3_observation(&item))
            .collect::<Vec<_>>();
        assert_eq!(
            adjudicate_c3_first_lane_coverage(&observations),
            Err(C3CoverageError::ReferenceResultUnpaired)
        );
    }

    #[test]
    fn deliberate_permitted_set_disagreement_fails_closed() {
        let mut item = source(OperationClass::History);
        item.reference_permitted_resource_ids.clear();
        item.permitted_set_equal = Some(false);
        item.differential_equivalent = Some(false);
        let evidence = adjudicate_c3_observation(&item);
        assert_eq!(evidence.verdict, C3DifferentialVerdict::Disagreement);
        assert_eq!(
            evidence.disagreement_class,
            Some(C3DisagreementClass::PermittedSetMismatch)
        );
    }

    #[test]
    fn principal_or_route_binding_tamper_is_not_reinterpreted_as_equivalence() {
        let mut item = source(OperationClass::History);
        item.path = "/v1/state/current".into();
        let evidence = adjudicate_c3_observation(&item);
        assert_eq!(
            evidence.disagreement_class,
            Some(C3DisagreementClass::OperationBindingMismatch)
        );
    }

    #[test]
    fn shadow_failure_is_observable_and_non_equivalent() {
        let mut item = source(OperationClass::History);
        item.disposition = C2ShadowDisposition::ShadowFailed;
        item.detail = Some("injected shadow failure".into());
        let evidence = adjudicate_c3_observation(&item);
        assert_eq!(
            evidence.disagreement_class,
            Some(C3DisagreementClass::ShadowFailure)
        );
        assert!(evidence.reference_path_authoritative);
        assert_eq!(evidence.authority_effect, "none");
    }

    #[test]
    fn exact_replay_is_identical() {
        let item = source(OperationClass::History);
        assert_eq!(
            adjudicate_c3_observation(&item),
            adjudicate_c3_observation(&item)
        );
    }

    #[test]
    fn coverage_rejects_a_missing_operation() {
        let observations = OperationClass::ALL_FIRST_LANE
            .into_iter()
            .take(22)
            .map(source)
            .map(|item| adjudicate_c3_observation(&item))
            .collect::<Vec<_>>();
        assert_eq!(
            adjudicate_c3_first_lane_coverage(&observations).unwrap_err(),
            C3CoverageError::FirstLaneCoverageMismatch
        );
    }

    #[test]
    fn self_labelled_23_profile_records_cannot_fake_completed_http_proof() {
        let records = OperationClass::ALL_FIRST_LANE
            .into_iter()
            .map(source)
            .map(|source| adjudicate_c3_observation(&source))
            .collect::<Vec<_>>();
        assert!(records.iter().all(|item| item.is_equivalent()));
        assert_eq!(
            adjudicate_c3_first_lane_coverage(&records),
            Err(C3CoverageError::ReferenceResultUnpaired)
        );
    }

    #[test]
    fn coverage_rechecks_provenance_after_verdict_and_rejects_duplicates() {
        let mut records = OperationClass::ALL_FIRST_LANE
            .into_iter()
            .map(source)
            .map(|item| adjudicate_c3_observation(&item))
            .collect::<Vec<_>>();
        for item in &mut records {
            item.reference_result_status = Some(200);
            item.reference_result_digest = Some("e".repeat(64));
        }
        assert!(adjudicate_c3_first_lane_coverage(&records).is_ok());
        let original = records[0].clone();
        records[0].issuer_build_digest = Some("forged".into());
        assert_eq!(
            adjudicate_c3_first_lane_coverage(&records),
            Err(C3CoverageError::InvalidCoverageEvidence)
        );
        records[0] = original;
        records[0].request_id = records[1].request_id.clone();
        assert_eq!(
            adjudicate_c3_first_lane_coverage(&records),
            Err(C3CoverageError::InvalidCoverageEvidence)
        );
    }

    #[test]
    fn coverage_rejects_mismatched_profile_even_if_verdict_marked_equivalent() {
        let mut records = OperationClass::ALL_FIRST_LANE
            .into_iter()
            .map(source)
            .map(|item| adjudicate_c3_observation(&item))
            .collect::<Vec<_>>();
        for item in &mut records {
            item.reference_result_status = Some(200);
            item.reference_result_digest = Some("e".repeat(64));
        }
        records[0].path = "/v1/state/current".into();
        assert_eq!(
            adjudicate_c3_first_lane_coverage(&records),
            Err(C3CoverageError::InvalidCoverageEvidence)
        );
    }

    #[test]
    fn replay_bundle_fails_on_eviction_or_wrong_revision() {
        let records = OperationClass::ALL_FIRST_LANE
            .into_iter()
            .map(source)
            .map(|item| adjudicate_c3_observation(&item))
            .collect::<Vec<_>>();
        let bundle = C3ReplayBundle {
            revision: C3_DIFFERENTIAL_REVISION.into(),
            evicted_observations: 1,
            observations: records,
        };
        assert_eq!(
            adjudicate_c3_replay_bundle(&bundle),
            Err(C3CoverageError::EvidenceLoss)
        );
        let mut bundle = bundle;
        bundle.evicted_observations = 0;
        bundle.revision = "untrusted-version".into();
        assert_eq!(
            adjudicate_c3_replay_bundle(&bundle),
            Err(C3CoverageError::InvalidCoverageEvidence)
        );
    }
}
