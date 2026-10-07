use crate::{FabricRoutingMode, HttpKernelState};
use aether_fabric::{
    realize_selected_placement, ControlStateWitness, FabricContractError, PlacementConstraintSet,
    PlacementSelected, RealizationError, RealizationRequest, RouteRealizedEvidence,
    AETHER_LOCAL_BLOCKING_POOL_ID,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FabricCutoverError {
    #[error("FABRIC candidate realization is disabled; reference routing remains authoritative")]
    RoutingModeDisabled,
    #[error("placement lies outside the admitted E3B one-resource live domain")]
    LiveDomainViolation,
    #[error(transparent)]
    Contract(#[from] FabricContractError),
    #[error(transparent)]
    Realization(#[from] RealizationError),
}

impl HttpKernelState {
    /// E3B-R readiness surface.
    ///
    /// This method is intentionally not part of the live HTTP execution path.
    /// It is callable only when the explicit CandidateReadiness mode is set,
    /// captures the current local-pool snapshot without acquiring permits, and
    /// returns RouteRealized evidence only after the pure realization bridge
    /// rechecks the supplied current governed control witness.
    ///
    /// Production routing remains unchanged until a separate E3B-A activation
    /// delta is authorized.
    pub fn fabric_candidate_realize_placement(
        &self,
        e1_envelope_bytes: &[u8],
        constraint: &PlacementConstraintSet,
        placement: &PlacementSelected,
        current_witness: &ControlStateWitness,
        request: &RealizationRequest,
    ) -> Result<RouteRealizedEvidence, FabricCutoverError> {
        if self.fabric_routing_mode() != FabricRoutingMode::CandidateReadiness {
            return Err(FabricCutoverError::RoutingModeDisabled);
        }
        if placement.resource_id != AETHER_LOCAL_BLOCKING_POOL_ID {
            return Err(FabricCutoverError::LiveDomainViolation);
        }

        let snapshot = self.fabric_reference_pool_snapshot(request.realization_time_unix_ms)?;
        Ok(realize_selected_placement(
            e1_envelope_bytes,
            constraint,
            placement,
            &snapshot,
            current_witness,
            request,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HttpKernelOptions, InMemoryKernelService};
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

    fn witness(e1: &[u8], id: &str, revision: &str, time: u64) -> ControlStateWitness {
        ControlStateWitness {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "ControlStateWitness".into(),
            witness_id: id.into(),
            envelope_id: "E1".into(),
            envelope_digest: sha256_hex(e1),
            e1_expires_at: "2026-10-07T00:00:00Z".into(),
            observed_state: ControlState::Active,
            observed_revision: revision.into(),
            observed_at_unix_ms: time,
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
            decision_time_unix_ms: witness.observed_at_unix_ms,
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

    fn placement(
        e1: &[u8],
        constraint: &PlacementConstraintSet,
        witness: &ControlStateWitness,
    ) -> PlacementSelected {
        let policy = aether_fabric::f1b_scheduler_policy_identity();
        let policy_digest =
            sha256_hex(&policy.canonical_bytes().expect("canonical policy identity"));
        let selector_digest = "3".repeat(64);
        let decision_input_digest = "4".repeat(64);
        let decision_id =
            aether_fabric::placement_decision_id(&decision_input_digest, &selector_digest);
        PlacementSelected {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "PlacementSelected".into(),
            placement_decision_id: decision_id.clone(),
            event_id: format!("placement-decision:{decision_id}"),
            correlation_id: "C1".into(),
            envelope_id: "E1".into(),
            envelope_digest: sha256_hex(e1),
            placement_constraint_id: constraint.constraint_set_id.clone(),
            placement_constraint_digest: constraint.digest().unwrap(),
            resource_snapshot_id: "S-placement".into(),
            resource_snapshot_digest: "1".repeat(64),
            scheduler_policy_id: policy.policy_id,
            scheduler_policy_digest: policy_digest,
            selector_implementation_digest: selector_digest,
            decision_time_unix_ms: constraint.decision_time_unix_ms,
            decision_input_digest,
            control_state_ref: witness.witness_id.clone(),
            control_state_digest: witness.digest().unwrap(),
            resource_id: AETHER_LOCAL_BLOCKING_POOL_ID.into(),
            eligibility_evidence: vec!["f1a_reference_admissible".into()],
            selection_rank: 1,
            authority_effect: "none".into(),
        }
    }

    fn request(time: u64) -> RealizationRequest {
        RealizationRequest {
            realization_request_id: "R1".into(),
            realization_time_unix_ms: time,
            occurred_at: "2026-10-07T10:30:00Z".into(),
        }
    }

    #[test]
    fn default_mode_keeps_candidate_realization_disabled() {
        let state = HttpKernelState::new(InMemoryKernelService::new());
        assert_eq!(
            state.fabric_routing_mode(),
            FabricRoutingMode::ReferenceOnly
        );

        let e1 = e1_envelope_bytes();
        let historical = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &historical);
        let selected = placement(&e1, &constraint, &historical);
        let current = witness(&e1, "W2", "rev-2", 1_100);

        let error = state
            .fabric_candidate_realize_placement(
                &e1,
                &constraint,
                &selected,
                &current,
                &request(1_100),
            )
            .unwrap_err();
        assert!(matches!(error, FabricCutoverError::RoutingModeDisabled));
    }

    #[test]
    fn candidate_readiness_realization_is_read_only_and_freshly_rechecked() {
        let state = HttpKernelState::with_options(
            InMemoryKernelService::new(),
            HttpKernelOptions::default()
                .with_fabric_routing_mode(FabricRoutingMode::CandidateReadiness),
        );
        let e1 = e1_envelope_bytes();
        let historical = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &historical);
        let selected = placement(&e1, &constraint, &historical);
        let current = witness(&e1, "W2", "rev-2", 1_100);

        let before = state.fabric_reference_pool_snapshot(1_100).unwrap();
        let route = state
            .fabric_candidate_realize_placement(
                &e1,
                &constraint,
                &selected,
                &current,
                &request(1_100),
            )
            .unwrap();
        let after = state.fabric_reference_pool_snapshot(1_100).unwrap();

        assert_eq!(before, after);
        assert_eq!(route.endpoint_id, AETHER_LOCAL_BLOCKING_POOL_ID);
        assert_eq!(route.authority_effect, "none");
    }

    #[test]
    fn candidate_readiness_rejects_late_revocation() {
        let state = HttpKernelState::with_options(
            InMemoryKernelService::new(),
            HttpKernelOptions::default()
                .with_fabric_routing_mode(FabricRoutingMode::CandidateReadiness),
        );
        let e1 = e1_envelope_bytes();
        let historical = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &historical);
        let selected = placement(&e1, &constraint, &historical);
        let mut current = witness(&e1, "W2", "rev-2", 1_100);
        current.observed_state = ControlState::Revoked;

        let error = state
            .fabric_candidate_realize_placement(
                &e1,
                &constraint,
                &selected,
                &current,
                &request(1_100),
            )
            .unwrap_err();

        assert!(matches!(
            error,
            FabricCutoverError::Realization(RealizationError::Contract(
                FabricContractError::ControlState(_)
            ))
        ));
    }

    #[test]
    fn candidate_readiness_rejects_synthetic_live_resource() {
        let state = HttpKernelState::with_options(
            InMemoryKernelService::new(),
            HttpKernelOptions::default()
                .with_fabric_routing_mode(FabricRoutingMode::CandidateReadiness),
        );
        let e1 = e1_envelope_bytes();
        let historical = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &historical);
        let mut selected = placement(&e1, &constraint, &historical);
        selected.resource_id = "synthetic-resource".into();
        let current = witness(&e1, "W2", "rev-2", 1_100);

        let error = state
            .fabric_candidate_realize_placement(
                &e1,
                &constraint,
                &selected,
                &current,
                &request(1_100),
            )
            .unwrap_err();

        assert!(matches!(error, FabricCutoverError::LiveDomainViolation));
    }
}
