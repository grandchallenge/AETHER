use crate::FabricShadowPlacementComparison;
use aether_fabric::{FabricContractError, PlacementDecision, AETHER_LOCAL_BLOCKING_POOL_ID};
use thiserror::Error;

/// F1E differential-equivalence evidence over the admitted live extraction
/// domain: exactly one current AETHER local blocking resource pool.
///
/// This record is evidence only. It cannot authorize routing, queueing,
/// dispatch, attempt creation, or any semantic state transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FabricDifferentialEquivalenceEvidence {
    pub live_domain: String,
    pub resource_snapshot_digest: String,
    pub reference_permitted_resource_ids: Vec<String>,
    pub fabric_permitted_resource_ids: Vec<String>,
    pub permitted_set_equal: bool,
    pub no_authorization_widening: bool,
    pub authority_effect: String,
}

#[derive(Debug, Error)]
pub enum FabricDifferentialEquivalenceError {
    #[error("F1E evidence is outside the admitted one-resource live extraction domain")]
    LiveDomainViolation,
    #[error("shadow evidence carries an authority effect")]
    AuthorityEffect,
    #[error("FABRIC placement evidence is not bound to the compared resource snapshot")]
    SnapshotBindingMismatch,
    #[error("FABRIC selected-resource projection is internally inconsistent")]
    SelectedResourceProjectionMismatch,
    #[error("FABRIC selection lies outside the AETHER-permitted live set")]
    AuthorizationSetWidening,
    #[error("FABRIC and AETHER permitted-placement sets differ")]
    PermittedSetMismatch,
    #[error(transparent)]
    Contract(#[from] FabricContractError),
}

fn decision_snapshot_digest(decision: &PlacementDecision) -> &str {
    match decision {
        PlacementDecision::Selected(selected) => &selected.resource_snapshot_digest,
        PlacementDecision::Unavailable(unavailable) => &unavailable.resource_snapshot_digest,
    }
}

fn decision_authority_effect(decision: &PlacementDecision) -> &str {
    match decision {
        PlacementDecision::Selected(selected) => &selected.authority_effect,
        PlacementDecision::Unavailable(unavailable) => &unavailable.authority_effect,
    }
}

fn decision_selected_resource(decision: &PlacementDecision) -> Option<&str> {
    match decision {
        PlacementDecision::Selected(selected) => Some(selected.resource_id.as_str()),
        PlacementDecision::Unavailable(_) => None,
    }
}

/// Adjudicate one non-operative F1D comparison as F1E live-equivalence
/// evidence.
///
/// The function fails closed on any mismatch. In particular, disagreement is
/// never returned as a usable placement result.
pub fn adjudicate_f1e_live_equivalence(
    comparison: &FabricShadowPlacementComparison,
) -> Result<FabricDifferentialEquivalenceEvidence, FabricDifferentialEquivalenceError> {
    if comparison.authority_effect != "none"
        || decision_authority_effect(&comparison.fabric_decision) != "none"
    {
        return Err(FabricDifferentialEquivalenceError::AuthorityEffect);
    }

    if comparison.resource_snapshot.resources.len() != 1
        || comparison.resource_snapshot.resources[0].resource_id != AETHER_LOCAL_BLOCKING_POOL_ID
    {
        return Err(FabricDifferentialEquivalenceError::LiveDomainViolation);
    }

    if comparison
        .reference_permitted_resource_ids
        .iter()
        .any(|resource_id| resource_id != AETHER_LOCAL_BLOCKING_POOL_ID)
        || comparison.reference_permitted_resource_ids.len() > 1
    {
        return Err(FabricDifferentialEquivalenceError::LiveDomainViolation);
    }

    comparison.resource_snapshot.verify_digest()?;
    if decision_snapshot_digest(&comparison.fabric_decision)
        != comparison.resource_snapshot.snapshot_digest
    {
        return Err(FabricDifferentialEquivalenceError::SnapshotBindingMismatch);
    }

    if decision_selected_resource(&comparison.fabric_decision)
        != comparison.fabric_selected_resource_id.as_deref()
    {
        return Err(FabricDifferentialEquivalenceError::SelectedResourceProjectionMismatch);
    }

    let fabric_permitted_resource_ids = comparison
        .fabric_selected_resource_id
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    let no_authorization_widening = fabric_permitted_resource_ids.iter().all(|resource_id| {
        comparison
            .reference_permitted_resource_ids
            .contains(resource_id)
    });
    if !no_authorization_widening {
        return Err(FabricDifferentialEquivalenceError::AuthorizationSetWidening);
    }

    let permitted_set_equal =
        fabric_permitted_resource_ids == comparison.reference_permitted_resource_ids;
    if !permitted_set_equal || !comparison.permitted_set_equal {
        return Err(FabricDifferentialEquivalenceError::PermittedSetMismatch);
    }

    Ok(FabricDifferentialEquivalenceEvidence {
        live_domain: "aether-local-blocking-pool-single-resource-v1".into(),
        resource_snapshot_digest: comparison.resource_snapshot.snapshot_digest.clone(),
        reference_permitted_resource_ids: comparison.reference_permitted_resource_ids.clone(),
        fabric_permitted_resource_ids,
        permitted_set_equal,
        no_authorization_widening,
        authority_effect: "none".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HttpKernelState;
    use aether_fabric::{
        sha256_hex, ControlState, ControlStateWitness, PlacementConstraintSet,
        ResourceRequirements, SelectorImplementationIdentity,
        CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT, E1_PROTOCOL_FAMILY,
    };

    fn e1_envelope_bytes() -> Vec<u8> {
        br#"{
          "schema_version":"1.0",
          "protocol_family":"aether-fabric",
          "protocol_major":1,
          "protocol_minor":0,
          "record_type":"MechanicalEnvelopeAuthorized",
          "domain":"control_bridge",
          "authority_owner":"upstream_governed_record",
          "authority_effect":"mechanical_only",
          "envelope_id":"E1",
          "correlation_id":"C1",
          "authorization_ref":"A1",
          "issuer_ref":"I1",
          "scope_ref":"scope://one",
          "scope_digest":{"algorithm":"sha256","value":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"},
          "payload_ref":"payload://one",
          "permitted_actions":["dispatch_payload"],
          "eligible_resource_classes":["local-blocking-pool"],
          "trust_zones":["aether-process"],
          "locality_constraints":["local-process"],
          "mechanical_policy_ref":"MP1",
          "priority_class":"normal",
          "fairness_policy_ref":"FP1",
          "retry_policy":{"max_attempts":1,"backoff_class":"none"},
          "redundancy_policy":{"max_parallel_copies":1},
          "expires_at":"2026-10-07T00:00:00Z",
          "required_capabilities":["blocking_execution"],
          "integrity_profile_ref":"IP1"
        }"#.to_vec()
    }

    fn witness(e1: &[u8]) -> ControlStateWitness {
        ControlStateWitness {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "ControlStateWitness".into(),
            witness_id: "W1".into(),
            envelope_id: "E1".into(),
            envelope_digest: sha256_hex(e1),
            e1_expires_at: "2026-10-07T00:00:00Z".into(),
            observed_state: ControlState::Active,
            observed_revision: "rev-1".into(),
            observed_at_unix_ms: 1_050,
            source_authority_ref: "AUTH".into(),
            authority_effect: "none".into(),
        }
    }

    fn constraint(e1: &[u8], witness: &ControlStateWitness) -> PlacementConstraintSet {
        PlacementConstraintSet {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "PlacementConstraintSet".into(),
            constraint_set_id: "PC1".into(),
            envelope_id: "E1".into(),
            envelope_digest: sha256_hex(e1),
            correlation_id: "C1".into(),
            resource_requirements: ResourceRequirements {
                capacity_unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
                minimum_capacity_units: 1,
            },
            max_snapshot_age_ms: 100,
            decision_time_unix_ms: 1_050,
            control_state_ref: witness.witness_id.clone(),
            control_state_digest: witness.digest().unwrap(),
            source_authority_ref: "AUTH".into(),
            authority_effect: "mechanical_narrowing_only".into(),
            eligible_resource_classes: Some(vec!["local-blocking-pool".into()]),
            trust_zones: Some(vec!["aether-process".into()]),
            required_capabilities: Some(vec!["blocking_execution".into()]),
            locality_constraints: Some(vec!["local-process".into()]),
        }
    }

    fn selector_identity() -> SelectorImplementationIdentity {
        SelectorImplementationIdentity {
            selector_id: "f1e-equivalence-selector".into(),
            source_commit: "f1e-source-commit".into(),
            source_tree: "f1e-source-tree".into(),
            artifact_sha256: "1".repeat(64),
        }
    }

    fn comparison(minimum_capacity_units: u64) -> FabricShadowPlacementComparison {
        let state = HttpKernelState::new(crate::InMemoryKernelService::new());
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let mut constraint = constraint(&e1, &witness);
        constraint.resource_requirements.minimum_capacity_units = minimum_capacity_units;
        state
            .fabric_shadow_compare_placement(
                &e1,
                &constraint,
                &witness,
                1_000,
                &selector_identity(),
            )
            .unwrap()
    }

    #[test]
    fn eligible_live_domain_proves_set_equality_and_no_widening() {
        let comparison = comparison(1);
        let evidence = adjudicate_f1e_live_equivalence(&comparison).unwrap();
        assert_eq!(
            evidence.reference_permitted_resource_ids,
            vec![AETHER_LOCAL_BLOCKING_POOL_ID.to_owned()]
        );
        assert_eq!(
            evidence.fabric_permitted_resource_ids,
            vec![AETHER_LOCAL_BLOCKING_POOL_ID.to_owned()]
        );
        assert!(evidence.permitted_set_equal);
        assert!(evidence.no_authorization_widening);
        assert_eq!(evidence.authority_effect, "none");
    }

    #[test]
    fn unavailable_live_domain_proves_equal_empty_sets() {
        let comparison = comparison(u64::MAX);
        let evidence = adjudicate_f1e_live_equivalence(&comparison).unwrap();
        assert!(evidence.reference_permitted_resource_ids.is_empty());
        assert!(evidence.fabric_permitted_resource_ids.is_empty());
        assert!(evidence.permitted_set_equal);
        assert!(evidence.no_authorization_widening);
    }

    #[test]
    fn injected_fabric_widening_is_rejected_not_returned_as_placement() {
        let mut comparison = comparison(1);
        comparison.reference_permitted_resource_ids.clear();
        comparison.permitted_set_equal = false;
        assert!(matches!(
            adjudicate_f1e_live_equivalence(&comparison).unwrap_err(),
            FabricDifferentialEquivalenceError::AuthorizationSetWidening
        ));
    }

    #[test]
    fn injected_reference_only_mismatch_is_rejected() {
        let mut comparison = comparison(u64::MAX);
        comparison
            .reference_permitted_resource_ids
            .push(AETHER_LOCAL_BLOCKING_POOL_ID.to_owned());
        comparison.permitted_set_equal = false;
        assert!(matches!(
            adjudicate_f1e_live_equivalence(&comparison).unwrap_err(),
            FabricDifferentialEquivalenceError::PermittedSetMismatch
        ));
    }

    #[test]
    fn synthetic_multi_resource_snapshot_cannot_be_claimed_as_live_equivalence() {
        let mut comparison = comparison(1);
        let mut second = comparison.resource_snapshot.resources[0].clone();
        second.resource_id = "synthetic-second-resource".into();
        comparison.resource_snapshot = aether_fabric::ResourceSnapshot::new(
            "synthetic-multi",
            comparison.resource_snapshot.observed_at_unix_ms,
            "synthetic-test",
            vec![comparison.resource_snapshot.resources[0].clone(), second],
        )
        .unwrap();
        assert!(matches!(
            adjudicate_f1e_live_equivalence(&comparison).unwrap_err(),
            FabricDifferentialEquivalenceError::LiveDomainViolation
        ));
    }

    #[test]
    fn authority_effect_laundering_is_rejected() {
        let mut comparison = comparison(1);
        comparison.authority_effect = "mechanical_only".into();
        assert!(matches!(
            adjudicate_f1e_live_equivalence(&comparison).unwrap_err(),
            FabricDifferentialEquivalenceError::AuthorityEffect
        ));
    }

    #[test]
    fn selected_resource_projection_tamper_is_rejected() {
        let mut comparison = comparison(1);
        comparison.fabric_selected_resource_id = None;
        assert!(matches!(
            adjudicate_f1e_live_equivalence(&comparison).unwrap_err(),
            FabricDifferentialEquivalenceError::SelectedResourceProjectionMismatch
        ));
    }

    #[test]
    fn snapshot_binding_tamper_is_rejected() {
        let mut comparison = comparison(1);
        match &mut comparison.fabric_decision {
            PlacementDecision::Selected(selected) => {
                selected.resource_snapshot_digest = "2".repeat(64);
            }
            PlacementDecision::Unavailable(_) => panic!("expected selected decision"),
        }
        assert!(matches!(
            adjudicate_f1e_live_equivalence(&comparison).unwrap_err(),
            FabricDifferentialEquivalenceError::SnapshotBindingMismatch
        ));
    }

    #[test]
    fn replay_of_identical_shadow_evidence_is_identical() {
        let comparison = comparison(1);
        let first = adjudicate_f1e_live_equivalence(&comparison).unwrap();
        let second = adjudicate_f1e_live_equivalence(&comparison).unwrap();
        assert_eq!(first, second);
    }
}
