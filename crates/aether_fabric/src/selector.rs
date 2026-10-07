use super::*;

pub const F1B_POLICY_ID: &str = "aether-fabric-f1b-lexicographic-v1";
const F1B_POLICY_BYTES: &[u8] =
    include_bytes!("../../../schemas/aether_fabric/e3/f1b_selector_policy.json");

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlacementDecision {
    Selected(PlacementSelected),
    Unavailable(PlacementUnavailable),
}

pub fn f1b_scheduler_policy_identity() -> SchedulerPolicyIdentity {
    SchedulerPolicyIdentity {
        policy_id: F1B_POLICY_ID.into(),
        artifact_sha256: sha256_hex(F1B_POLICY_BYTES),
    }
}

fn validate_f1b_policy(policy: &SchedulerPolicyIdentity) -> Result<(), FabricContractError> {
    policy.validate()?;
    let expected = f1b_scheduler_policy_identity();
    if policy != &expected {
        return Err(FabricContractError::SelectorIdentity(
            "F1B requires the exact protected lexicographic policy artifact".into(),
        ));
    }
    Ok(())
}

fn decision_event_id(placement_decision_id: &str) -> String {
    format!("placement-decision:{placement_decision_id}")
}

pub fn f1b_select_resource(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
    snapshot: &ResourceSnapshot,
    witness: &ControlStateWitness,
    policy: &SchedulerPolicyIdentity,
    selector: &SelectorImplementationIdentity,
) -> Result<PlacementDecision, FabricContractError> {
    validate_f1b_policy(policy)?;
    selector.validate()?;

    let digests = decision_input_digests(
        e1_envelope_bytes,
        constraint,
        snapshot,
        policy,
        selector,
        witness,
    )?;

    let mut eligible = Vec::new();
    for resource in &snapshot.resources {
        if reference_resource_admissible(
            e1_envelope_bytes,
            constraint,
            snapshot,
            witness,
            &resource.resource_id,
        )? {
            eligible.push(resource);
        }
    }

    eligible.sort_by(|left, right| {
        left.queue_depth
            .cmp(&right.queue_depth)
            .then_with(|| {
                right
                    .available_capacity
                    .available
                    .cmp(&left.available_capacity.available)
            })
            .then_with(|| left.resource_id.cmp(&right.resource_id))
    });

    let placement_id = placement_decision_id(
        &digests.decision_input_digest,
        &digests.selector_implementation_digest,
    );
    let event_id = decision_event_id(&placement_id);

    if let Some(resource) = eligible.first() {
        return Ok(PlacementDecision::Selected(PlacementSelected {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: E3_PROTOCOL_MAJOR,
            protocol_minor: E3_PROTOCOL_MINOR,
            record_type: "PlacementSelected".into(),
            placement_decision_id: placement_id,
            event_id,
            correlation_id: constraint.correlation_id.clone(),
            envelope_id: constraint.envelope_id.clone(),
            envelope_digest: digests.envelope_digest,
            placement_constraint_id: constraint.constraint_set_id.clone(),
            placement_constraint_digest: digests.placement_constraint_digest,
            resource_snapshot_id: snapshot.snapshot_id.clone(),
            resource_snapshot_digest: digests.resource_snapshot_digest,
            scheduler_policy_id: policy.policy_id.clone(),
            scheduler_policy_digest: digests.scheduler_policy_digest,
            selector_implementation_digest: digests.selector_implementation_digest,
            decision_time_unix_ms: constraint.decision_time_unix_ms,
            decision_input_digest: digests.decision_input_digest,
            control_state_ref: constraint.control_state_ref.clone(),
            control_state_digest: digests.control_state_digest,
            resource_id: resource.resource_id.clone(),
            eligibility_evidence: vec![
                "f1a_reference_admissible".into(),
                "f1b_lexicographic_rank_1".into(),
            ],
            selection_rank: 1,
            authority_effect: "none".into(),
        }));
    }

    Ok(PlacementDecision::Unavailable(PlacementUnavailable {
        schema_version: "1.1".into(),
        protocol_family: E1_PROTOCOL_FAMILY.into(),
        protocol_major: E3_PROTOCOL_MAJOR,
        protocol_minor: E3_PROTOCOL_MINOR,
        record_type: "PlacementUnavailable".into(),
        placement_decision_id: placement_id,
        event_id,
        correlation_id: constraint.correlation_id.clone(),
        envelope_id: constraint.envelope_id.clone(),
        envelope_digest: digests.envelope_digest,
        placement_constraint_id: constraint.constraint_set_id.clone(),
        placement_constraint_digest: digests.placement_constraint_digest,
        resource_snapshot_id: snapshot.snapshot_id.clone(),
        resource_snapshot_digest: digests.resource_snapshot_digest,
        scheduler_policy_id: policy.policy_id.clone(),
        scheduler_policy_digest: digests.scheduler_policy_digest,
        selector_implementation_digest: digests.selector_implementation_digest,
        decision_time_unix_ms: constraint.decision_time_unix_ms,
        decision_input_digest: digests.decision_input_digest,
        control_state_ref: constraint.control_state_ref.clone(),
        control_state_digest: digests.control_state_digest,
        reason: "no_eligible_resource".into(),
        authority_effect: "none".into(),
    }))
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
            selector_id: "f1b-selector".into(),
            source_commit: "commit".into(),
            source_tree: "tree".into(),
            artifact_sha256: "1".repeat(64),
        }
    }

    fn resource(id: &str, queue_depth: u64, available: u64) -> ResourceObservation {
        let mut item = local_blocking_pool_observation(16, available, queue_depth);
        item.resource_id = id.into();
        item
    }

    #[test]
    fn exact_input_replays_to_exact_decision() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![resource("r1", 3, 4), resource("r2", 1, 2)],
        )
        .unwrap();
        let policy = f1b_scheduler_policy_identity();
        let selector = selector_identity();

        let first =
            f1b_select_resource(&e1, &constraint, &snapshot, &witness, &policy, &selector).unwrap();
        let second =
            f1b_select_resource(&e1, &constraint, &snapshot, &witness, &policy, &selector).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn lower_queue_depth_wins_first() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![resource("r1", 4, 16), resource("r2", 1, 1)],
        )
        .unwrap();
        let decision = f1b_select_resource(
            &e1,
            &constraint,
            &snapshot,
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity(),
        )
        .unwrap();
        match decision {
            PlacementDecision::Selected(selected) => assert_eq!(selected.resource_id, "r2"),
            PlacementDecision::Unavailable(_) => panic!("expected selection"),
        }
    }

    #[test]
    fn greater_available_capacity_breaks_queue_tie() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![resource("r1", 1, 2), resource("r2", 1, 5)],
        )
        .unwrap();
        let decision = f1b_select_resource(
            &e1,
            &constraint,
            &snapshot,
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity(),
        )
        .unwrap();
        match decision {
            PlacementDecision::Selected(selected) => assert_eq!(selected.resource_id, "r2"),
            PlacementDecision::Unavailable(_) => panic!("expected selection"),
        }
    }

    #[test]
    fn resource_id_is_stable_final_tie_break() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let left = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![resource("z", 1, 5), resource("a", 1, 5)],
        )
        .unwrap();
        let right = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![resource("a", 1, 5), resource("z", 1, 5)],
        )
        .unwrap();
        assert_eq!(left.snapshot_digest, right.snapshot_digest);
        for snapshot in [&left, &right] {
            let decision = f1b_select_resource(
                &e1,
                &constraint,
                snapshot,
                &witness,
                &f1b_scheduler_policy_identity(),
                &selector_identity(),
            )
            .unwrap();
            match decision {
                PlacementDecision::Selected(selected) => assert_eq!(selected.resource_id, "a"),
                PlacementDecision::Unavailable(_) => panic!("expected selection"),
            }
        }
    }

    #[test]
    fn ineligible_resource_cannot_win_ranking() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let mut invalid = resource("invalid", 0, 16);
        invalid.trust_zone = "wrong-zone".into();
        let snapshot =
            ResourceSnapshot::new("S1", 1_000, "test", vec![invalid, resource("valid", 9, 1)])
                .unwrap();
        let decision = f1b_select_resource(
            &e1,
            &constraint,
            &snapshot,
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity(),
        )
        .unwrap();
        match decision {
            PlacementDecision::Selected(selected) => assert_eq!(selected.resource_id, "valid"),
            PlacementDecision::Unavailable(_) => panic!("expected selection"),
        }
    }

    #[test]
    fn no_eligible_resource_returns_non_authoritative_unavailable() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot =
            ResourceSnapshot::new("S1", 1_000, "test", vec![resource("saturated", 16, 0)]).unwrap();
        let decision = f1b_select_resource(
            &e1,
            &constraint,
            &snapshot,
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity(),
        )
        .unwrap();
        match decision {
            PlacementDecision::Selected(_) => panic!("expected unavailable"),
            PlacementDecision::Unavailable(unavailable) => {
                assert_eq!(unavailable.reason, "no_eligible_resource");
                assert_eq!(unavailable.authority_effect, "none");
            }
        }
    }

    #[test]
    fn policy_substitution_fails_closed() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot =
            ResourceSnapshot::new("S1", 1_000, "test", vec![resource("r1", 0, 1)]).unwrap();
        let mut policy = f1b_scheduler_policy_identity();
        policy.artifact_sha256 = "f".repeat(64);
        assert!(f1b_select_resource(
            &e1,
            &constraint,
            &snapshot,
            &witness,
            &policy,
            &selector_identity(),
        )
        .is_err());
    }

    #[test]
    fn selection_is_pre_attempt_and_non_authoritative() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot =
            ResourceSnapshot::new("S1", 1_000, "test", vec![resource("r1", 0, 1)]).unwrap();
        let decision = f1b_select_resource(
            &e1,
            &constraint,
            &snapshot,
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity(),
        )
        .unwrap();
        let value = match decision {
            PlacementDecision::Selected(selected) => serde_json::to_value(selected).unwrap(),
            PlacementDecision::Unavailable(_) => panic!("expected selection"),
        };
        assert!(value.get("mechanical_attempt_id").is_none());
        assert_eq!(
            value.get("authority_effect").and_then(Value::as_str),
            Some("none")
        );
    }
}
