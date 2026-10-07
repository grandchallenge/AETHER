use super::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const E3B_ROUTE_CLASS: &str = "fabric-resource-selection-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealizationRequest {
    pub realization_request_id: String,
    pub realization_time_unix_ms: u64,
    pub occurred_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteRealizedEvidence {
    pub schema_version: String,
    pub protocol_family: String,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub domain: String,
    pub authority_effect: String,
    pub record_type: String,
    pub event_id: String,
    pub correlation_id: String,
    pub causation_id: String,
    pub mechanical_attempt_id: String,
    pub occurred_at: String,
    pub envelope_id: String,
    pub endpoint_id: String,
    pub resource_class: String,
    pub route_class: String,
    pub mechanical_policy_ref: String,
}

#[derive(Debug, Error)]
pub enum RealizationError {
    #[error(transparent)]
    Contract(#[from] FabricContractError),
    #[error("invalid realization request: {0}")]
    InvalidRequest(String),
    #[error("placement binding mismatch: {0}")]
    PlacementBinding(String),
    #[error("selected resource is not admissible at realization time")]
    SelectedResourceUnavailable,
}

fn validate_placement_binding(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
    placement: &PlacementSelected,
) -> Result<Value, RealizationError> {
    validate_placement_constraint_narrowing(e1_envelope_bytes, constraint)?;
    let envelope = parse_json_no_duplicates(e1_envelope_bytes)?;

    if placement.schema_version != "1.1"
        || placement.protocol_family != E1_PROTOCOL_FAMILY
        || placement.protocol_major != E3_PROTOCOL_MAJOR
        || placement.protocol_minor != E3_PROTOCOL_MINOR
        || placement.record_type != "PlacementSelected"
    {
        return Err(RealizationError::PlacementBinding(
            "expected exact aether-fabric/1.1 PlacementSelected".into(),
        ));
    }
    if placement.authority_effect != "none" {
        return Err(RealizationError::PlacementBinding(
            "placement authority_effect must be none".into(),
        ));
    }
    if placement.envelope_id != constraint.envelope_id
        || placement.envelope_digest != sha256_hex(e1_envelope_bytes)
    {
        return Err(RealizationError::PlacementBinding(
            "placement does not bind the exact E1 envelope".into(),
        ));
    }
    if placement.correlation_id != constraint.correlation_id {
        return Err(RealizationError::PlacementBinding(
            "placement correlation_id mismatch".into(),
        ));
    }
    if placement.placement_constraint_id != constraint.constraint_set_id
        || placement.placement_constraint_digest != constraint.digest()?
    {
        return Err(RealizationError::PlacementBinding(
            "placement does not bind the exact placement constraint".into(),
        ));
    }
    if placement.control_state_ref != constraint.control_state_ref
        || placement.control_state_digest != constraint.control_state_digest
    {
        return Err(RealizationError::PlacementBinding(
            "placement historical control witness mismatch".into(),
        ));
    }

    let protected_policy = f1b_scheduler_policy_identity();
    if placement.scheduler_policy_id != protected_policy.policy_id
        || placement.scheduler_policy_digest != sha256_hex(&protected_policy.canonical_bytes()?)
    {
        return Err(RealizationError::PlacementBinding(
            "placement does not bind the protected F1B policy".into(),
        ));
    }

    let expected_decision_id = placement_decision_id(
        &placement.decision_input_digest,
        &placement.selector_implementation_digest,
    );
    if placement.placement_decision_id != expected_decision_id
        || placement.event_id != format!("placement-decision:{expected_decision_id}")
    {
        return Err(RealizationError::PlacementBinding(
            "placement decision/event identity is internally inconsistent".into(),
        ));
    }

    Ok(envelope)
}

fn current_constraint(
    constraint: &PlacementConstraintSet,
    witness: &ControlStateWitness,
    realization_time_unix_ms: u64,
) -> Result<PlacementConstraintSet, RealizationError> {
    let mut current = constraint.clone();
    current.decision_time_unix_ms = realization_time_unix_ms;
    current.control_state_ref = witness.witness_id.clone();
    current.control_state_digest = witness.digest()?;
    Ok(current)
}

pub fn realize_selected_placement(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
    placement: &PlacementSelected,
    current_snapshot: &ResourceSnapshot,
    current_witness: &ControlStateWitness,
    request: &RealizationRequest,
) -> Result<RouteRealizedEvidence, RealizationError> {
    if request.realization_request_id.is_empty() {
        return Err(RealizationError::InvalidRequest(
            "realization_request_id must be non-empty".into(),
        ));
    }
    if request.occurred_at.is_empty() {
        return Err(RealizationError::InvalidRequest(
            "occurred_at must be non-empty".into(),
        ));
    }
    if request.realization_time_unix_ms < placement.decision_time_unix_ms {
        return Err(RealizationError::InvalidRequest(
            "realization time cannot precede placement decision time".into(),
        ));
    }

    let envelope = validate_placement_binding(e1_envelope_bytes, constraint, placement)?;
    let current_constraint = current_constraint(
        constraint,
        current_witness,
        request.realization_time_unix_ms,
    )?;

    let admissible = reference_resource_admissible(
        e1_envelope_bytes,
        &current_constraint,
        current_snapshot,
        current_witness,
        &placement.resource_id,
    )?;
    if !admissible {
        return Err(RealizationError::SelectedResourceUnavailable);
    }

    let resource = current_snapshot
        .resources
        .iter()
        .find(|resource| resource.resource_id == placement.resource_id)
        .ok_or(RealizationError::SelectedResourceUnavailable)?;

    let mechanical_policy_ref = envelope
        .get("mechanical_policy_ref")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            RealizationError::PlacementBinding("missing E1 mechanical_policy_ref".into())
        })?
        .to_owned();

    let current_witness_digest = current_witness.digest()?;
    let request_bytes = canonicalize_serializable(request)?;
    let mechanical_attempt_id = framed_digest(&[
        ("realization-request", &request_bytes),
        (
            "placement-decision",
            placement.placement_decision_id.as_bytes(),
        ),
        ("current-control", current_witness_digest.as_bytes()),
        (
            "current-snapshot",
            current_snapshot.snapshot_digest.as_bytes(),
        ),
    ]);
    let event_id = format!("route-realized:{mechanical_attempt_id}");

    Ok(RouteRealizedEvidence {
        schema_version: "1.0".into(),
        protocol_family: E1_PROTOCOL_FAMILY.into(),
        protocol_major: E1_PROTOCOL_MAJOR,
        protocol_minor: E1_PROTOCOL_MINOR,
        domain: "mechanical".into(),
        authority_effect: "none".into(),
        record_type: "RouteRealized".into(),
        event_id,
        correlation_id: placement.correlation_id.clone(),
        causation_id: placement.event_id.clone(),
        mechanical_attempt_id,
        occurred_at: request.occurred_at.clone(),
        envelope_id: placement.envelope_id.clone(),
        endpoint_id: placement.resource_id.clone(),
        resource_class: resource.resource_class.clone(),
        route_class: E3B_ROUTE_CLASS.into(),
        mechanical_policy_ref,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
            eligible_resource_classes: Some(vec![AETHER_LOCAL_BLOCKING_POOL_CLASS.into()]),
            trust_zones: Some(vec![AETHER_LOCAL_TRUST_ZONE.into()]),
            required_capabilities: Some(vec![AETHER_BLOCKING_CAPABILITY.into()]),
            locality_constraints: Some(vec!["local-process".into()]),
        }
    }

    fn placement(
        e1: &[u8],
        constraint: &PlacementConstraintSet,
        witness: &ControlStateWitness,
    ) -> PlacementSelected {
        let policy = f1b_scheduler_policy_identity();
        let policy_digest = sha256_hex(&policy.canonical_bytes().unwrap());
        let selector_digest = "3".repeat(64);
        let decision_input_digest = "4".repeat(64);
        let decision_id = placement_decision_id(&decision_input_digest, &selector_digest);
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

    fn request(id: &str, time: u64) -> RealizationRequest {
        RealizationRequest {
            realization_request_id: id.into(),
            realization_time_unix_ms: time,
            occurred_at: "2026-10-07T10:30:00Z".into(),
        }
    }

    #[test]
    fn fresh_active_recheck_creates_route_realized_attempt() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let route = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_100),
        )
        .unwrap();

        assert_eq!(route.record_type, "RouteRealized");
        assert_eq!(route.endpoint_id, AETHER_LOCAL_BLOCKING_POOL_ID);
        assert_eq!(route.resource_class, AETHER_LOCAL_BLOCKING_POOL_CLASS);
        assert_eq!(route.causation_id, selected.event_id);
        assert_eq!(route.authority_effect, "none");
        assert!(!route.mechanical_attempt_id.is_empty());
    }

    #[test]
    fn identical_realization_request_is_idempotent() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();
        let req = request("R1", 1_100);

        let first = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &req,
        )
        .unwrap();
        let second = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &req,
        )
        .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn retry_request_gets_new_attempt_identity() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let first = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_100),
        )
        .unwrap();
        let second = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R2", 1_100),
        )
        .unwrap();

        assert_ne!(first.mechanical_attempt_id, second.mechanical_attempt_id);
    }

    #[test]
    fn revocation_after_placement_blocks_realization() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let mut current_witness = witness(&e1, "W2", "rev-2", 1_100);
        current_witness.observed_state = ControlState::Revoked;
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let error = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_100),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            RealizationError::Contract(FabricContractError::ControlState(_))
        ));
    }

    #[test]
    fn selected_resource_must_still_be_admissible() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 0, 8)],
        )
        .unwrap();

        let error = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_100),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            RealizationError::SelectedResourceUnavailable
        ));
    }

    #[test]
    fn forged_placement_constraint_binding_fails_closed() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let mut selected = placement(&e1, &constraint, &placement_witness);
        selected.placement_constraint_digest = "f".repeat(64);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let error = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_100),
        )
        .unwrap_err();

        assert!(matches!(error, RealizationError::PlacementBinding(_)));
    }

    #[test]
    fn supersession_or_expiry_after_placement_blocks_realization() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        for state in [ControlState::Superseded, ControlState::Expired] {
            let mut current = witness(&e1, "W2", "rev-2", 1_100);
            current.observed_state = state;
            let error = realize_selected_placement(
                &e1,
                &constraint,
                &selected,
                &snapshot,
                &current,
                &request("R1", 1_100),
            )
            .unwrap_err();
            assert!(matches!(
                error,
                RealizationError::Contract(FabricContractError::ControlState(_))
            ));
        }
    }

    #[test]
    fn stale_realization_snapshot_fails_closed() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W2", "rev-2", 1_200);
        let snapshot = ResourceSnapshot::new(
            "S-stale",
            1_000,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let error = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_200),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            RealizationError::Contract(FabricContractError::Snapshot(_))
        ));
    }

    #[test]
    fn realization_cannot_precede_placement_decision_time() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W0", "rev-0", 900);
        let snapshot = ResourceSnapshot::new(
            "S-earlier",
            900,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let error = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 900),
        )
        .unwrap_err();

        assert!(matches!(error, RealizationError::InvalidRequest(_)));
    }

    #[test]
    fn exact_request_record_is_attempt_identity_material() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let selected = placement(&e1, &constraint, &placement_witness);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();
        let first_request = request("R1", 1_100);
        let mut second_request = first_request.clone();
        second_request.occurred_at = "2026-10-07T10:30:01Z".into();

        let first = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &first_request,
        )
        .unwrap();
        let second = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &second_request,
        )
        .unwrap();

        assert_ne!(first.mechanical_attempt_id, second.mechanical_attempt_id);
    }

    #[test]
    fn placement_must_bind_protected_f1b_policy_and_decision_identity() {
        let e1 = e1_envelope_bytes();
        let placement_witness = witness(&e1, "W1", "rev-1", 1_000);
        let constraint = constraint(&e1, &placement_witness);
        let mut selected = placement(&e1, &constraint, &placement_witness);
        selected.scheduler_policy_digest = "f".repeat(64);
        let current_witness = witness(&e1, "W2", "rev-2", 1_100);
        let snapshot = ResourceSnapshot::new(
            "S-current",
            1_100,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();

        let error = realize_selected_placement(
            &e1,
            &constraint,
            &selected,
            &snapshot,
            &current_witness,
            &request("R1", 1_100),
        )
        .unwrap_err();

        assert!(matches!(error, RealizationError::PlacementBinding(_)));
    }
}
