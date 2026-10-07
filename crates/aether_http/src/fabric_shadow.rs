use crate::http::HttpKernelState;
use aether_fabric::{
    f1b_scheduler_policy_identity, f1b_select_resource, reference_resource_admissible,
    ControlStateWitness, FabricContractError, PlacementConstraintSet, PlacementDecision,
    ResourceSnapshot, SelectorImplementationIdentity, AETHER_LOCAL_BLOCKING_POOL_ID,
};

/// Non-operative F1D comparison between the current AETHER reference surface
/// and the F1B FABRIC selector over one exact modeled local-pool snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FabricShadowPlacementComparison {
    pub resource_snapshot: ResourceSnapshot,
    pub reference_permitted_resource_ids: Vec<String>,
    pub fabric_selected_resource_id: Option<String>,
    pub fabric_decision: PlacementDecision,
    pub permitted_set_equal: bool,
    pub authority_effect: String,
}

impl HttpKernelState {
    /// Execute the F1D shadow comparison without acquiring permits, enqueueing
    /// work, creating attempts, dispatching, or changing semantic state.
    ///
    /// The local resource snapshot is captured exactly once and that exact
    /// snapshot is supplied to both the AETHER reference predicate and FABRIC
    /// selector. The reference path remains authoritative; the returned value
    /// is comparison evidence only.
    pub fn fabric_shadow_compare_placement(
        &self,
        e1_envelope_bytes: &[u8],
        constraint: &PlacementConstraintSet,
        witness: &ControlStateWitness,
        observed_at_unix_ms: u64,
        selector: &SelectorImplementationIdentity,
    ) -> Result<FabricShadowPlacementComparison, FabricContractError> {
        let snapshot = self.fabric_reference_pool_snapshot(observed_at_unix_ms)?;

        let reference_admissible = reference_resource_admissible(
            e1_envelope_bytes,
            constraint,
            &snapshot,
            witness,
            AETHER_LOCAL_BLOCKING_POOL_ID,
        )?;
        let reference_permitted_resource_ids = if reference_admissible {
            vec![AETHER_LOCAL_BLOCKING_POOL_ID.to_owned()]
        } else {
            Vec::new()
        };

        let fabric_decision = f1b_select_resource(
            e1_envelope_bytes,
            constraint,
            &snapshot,
            witness,
            &f1b_scheduler_policy_identity(),
            selector,
        )?;
        let fabric_selected_resource_id = match &fabric_decision {
            PlacementDecision::Selected(selected) => Some(selected.resource_id.clone()),
            PlacementDecision::Unavailable(_) => None,
        };
        let fabric_permitted_resource_ids = fabric_selected_resource_id
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        let permitted_set_equal = fabric_permitted_resource_ids == reference_permitted_resource_ids;

        Ok(FabricShadowPlacementComparison {
            resource_snapshot: snapshot,
            reference_permitted_resource_ids,
            fabric_selected_resource_id,
            fabric_decision,
            permitted_set_equal,
            authority_effect: "none".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_fabric::{
        sha256_hex, ControlState, ResourceRequirements, CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT,
        E1_PROTOCOL_FAMILY,
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
        }"#
            .to_vec()
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
            selector_id: "f1d-shadow-selector".into(),
            source_commit: "f1d-source-commit".into(),
            source_tree: "f1d-source-tree".into(),
            artifact_sha256: "1".repeat(64),
        }
    }

    #[test]
    fn shadow_eligible_path_matches_reference_and_is_read_only() {
        let state = HttpKernelState::new(crate::InMemoryKernelService::new());
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let before = state.fabric_reference_pool_snapshot(1_000).unwrap();

        let comparison = state
            .fabric_shadow_compare_placement(
                &e1,
                &constraint,
                &witness,
                1_000,
                &selector_identity(),
            )
            .unwrap();
        let after = state.fabric_reference_pool_snapshot(1_000).unwrap();

        assert_eq!(before, after);
        assert_eq!(comparison.resource_snapshot, before);
        assert_eq!(
            comparison.reference_permitted_resource_ids,
            vec![AETHER_LOCAL_BLOCKING_POOL_ID.to_owned()]
        );
        assert_eq!(
            comparison.fabric_selected_resource_id.as_deref(),
            Some(AETHER_LOCAL_BLOCKING_POOL_ID)
        );
        assert!(comparison.permitted_set_equal);
        assert_eq!(comparison.authority_effect, "none");
    }

    #[test]
    fn shadow_unavailable_path_matches_empty_reference_set() {
        let state = HttpKernelState::new(crate::InMemoryKernelService::new());
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let mut constraint = constraint(&e1, &witness);
        constraint.resource_requirements.minimum_capacity_units = u64::MAX;

        let comparison = state
            .fabric_shadow_compare_placement(
                &e1,
                &constraint,
                &witness,
                1_000,
                &selector_identity(),
            )
            .unwrap();

        assert!(comparison.reference_permitted_resource_ids.is_empty());
        assert!(comparison.fabric_selected_resource_id.is_none());
        assert!(matches!(
            comparison.fabric_decision,
            PlacementDecision::Unavailable(_)
        ));
        assert!(comparison.permitted_set_equal);
        assert_eq!(comparison.authority_effect, "none");
    }

    #[test]
    fn shadow_revoked_control_state_fails_closed_before_comparison() {
        let state = HttpKernelState::new(crate::InMemoryKernelService::new());
        let e1 = e1_envelope_bytes();
        let mut witness = witness(&e1);
        witness.observed_state = ControlState::Revoked;
        witness.observed_revision = "rev-2".into();
        let constraint = constraint(&e1, &witness);

        let error = state
            .fabric_shadow_compare_placement(
                &e1,
                &constraint,
                &witness,
                1_000,
                &selector_identity(),
            )
            .unwrap_err();

        assert!(matches!(error, FabricContractError::ControlState(_)));
    }
}
