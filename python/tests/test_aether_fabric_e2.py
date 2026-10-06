from __future__ import annotations

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "python"))

from aether_fabric_e2 import load_vector_registry, validate_e1_schemas, validate_trace


def test_e1_schemas_and_declared_fixtures_execute() -> None:
    result = validate_e1_schemas(REPO_ROOT)
    assert result["schema_count"] == 6
    assert set(result["valid_fixtures"]) == {
        "telemetry_queue_depth.json",
        "telemetry_latency.json",
        "semantic_started_mechanical.json",
        "envelope_revoked.json",
    }
    assert result["invalid_fixture"] == "semantic_started_half_bound.json"
    assert result["invalid_error_count"] >= 1
    assert result["invalid_reason"] == "semantic_start_half_bound"


def test_v01_v51_registry_is_gap_free_and_d1_d5_present() -> None:
    registry = load_vector_registry(REPO_ROOT)
    assert set(registry["vectors"]) == {f"V{i:02d}" for i in range(1, 52)}
    assert all(entry["families"] for entry in registry["vectors"].values())
    required = {
        "D1_SEMANTIC_EQUIVALENCE",
        "D2_INFLUENCE_WITHOUT_AUTHORITY",
        "D3_REPLICA_FENCING",
        "D4_SIDECAR_LOCALITY",
        "D5_RESOURCE_LIFECYCLE",
    }
    observed = {family for entry in registry["vectors"].values() for family in entry["families"]}
    assert required <= observed


def test_d1_positive_exact_attempt_lifecycle() -> None:
    trace = [
        {"record_type": "QueueAdmitted", "mechanical_attempt_id": "M1", "envelope_id": "E1"},
        {"record_type": "SemanticExecutionStarted", "semantic_attempt_id": "S1",
         "mechanical_envelope_id": "E1", "mechanical_attempt_id": "M1"},
        {"record_type": "SemanticExecutionCompleted", "semantic_attempt_id": "S1"},
    ]
    assert validate_trace(trace) == []


def test_d1_hostile_hidden_commit_and_half_binding_fail_closed() -> None:
    hidden = [
        {"record_type": "PreStartRejected", "mechanical_attempt_id": "M1", "envelope_id": "E1"},
        {"record_type": "SemanticExecutionStarted", "semantic_attempt_id": "S1",
         "mechanical_envelope_id": "E1", "mechanical_attempt_id": "M1"},
    ]
    assert "1:hidden_commit_after_pre_start_rejection" in validate_trace(hidden)
    half = [{"record_type": "SemanticExecutionStarted", "semantic_attempt_id": "S2",
             "mechanical_envelope_id": "E2"}]
    assert validate_trace(half) == ["0:semantic_start_half_bound"]


def test_d2_positive_telemetry_requires_submission_and_admission() -> None:
    trace = [
        {"record_type": "TelemetryEvidenceObserved", "authority_effect": "none"},
        {"record_type": "SemanticSubmissionProposed"},
        {"record_type": "SemanticAdmissionAccepted"},
        {"record_type": "AllocationDesired"},
    ]
    assert validate_trace(trace) == []


def test_d2_hostile_authority_laundering_and_telemetry_bypass_fail() -> None:
    laundering = [{"record_type": "PayloadDelivered", "authority_effect": "semantic"}]
    assert validate_trace(laundering) == ["0:mechanical_authority_laundering"]
    bypass = [
        {"record_type": "TelemetryEvidenceObserved", "authority_effect": "none"},
        {"record_type": "AllocationDesired"},
    ]
    assert validate_trace(bypass) == ["1:unadmitted_telemetry_policy_bypass"]


def test_d3_replica_copy_cannot_promote_authority() -> None:
    positive = [
        {"record_type": "ReplicaMovementObserved", "authority_effect": "none"},
        {"record_type": "ReplicaPromoted", "authority_owner": "AETHER"},
    ]
    assert validate_trace(positive) == []
    hostile = [
        {"record_type": "ReplicaMovementObserved", "authority_effect": "none"},
        {"record_type": "LeaderEpochChanged", "authority_owner": "FABRIC"},
    ]
    assert validate_trace(hostile) == ["1:replica_movement_authority_promotion"]


def test_d4_physical_locality_is_observation_only() -> None:
    assert validate_trace([
        {"record_type": "PhysicalObjectLocationObserved", "authority_effect": "none", "admitted": False}
    ]) == []
    assert validate_trace([
        {"record_type": "PhysicalObjectLocationObserved", "authority_effect": "none", "semantic_relevant": True}
    ]) == ["0:physical_locality_semantic_collapse"]


def test_d5_positive_started_attempt_remains_semantic_after_revocation() -> None:
    trace = [
        {"record_type": "SemanticExecutionStarted", "semantic_attempt_id": "S5",
         "mechanical_envelope_id": "E5", "mechanical_attempt_id": "M5"},
        {"record_type": "MechanicalEnvelopeRevoked", "envelope_id": "E5",
         "authority_owner": "upstream_governed_record"},
        {"record_type": "SemanticExecutionCompleted", "semantic_attempt_id": "S5"},
    ]
    assert validate_trace(trace) == []


def test_d5_revocation_blocks_retry_and_new_start() -> None:
    trace = [
        {"record_type": "MechanicalEnvelopeRevoked", "envelope_id": "E5",
         "authority_owner": "upstream_governed_record"},
        {"record_type": "RetryRealized", "envelope_id": "E5",
         "retry_of_mechanical_attempt_id": "M5a", "mechanical_attempt_id": "M5b"},
        {"record_type": "QueueAdmitted", "envelope_id": "E5", "mechanical_attempt_id": "M5b"},
    ]
    assert validate_trace(trace) == ["1:retry_after_revocation", "2:new_mechanical_work_after_revocation"]


def test_protocol_downgrade_identity_collapse_and_ghos_reachability_fail() -> None:
    trace = [
        {"record_type": "ProtocolNegotiated", "protocol_family": "aether-fabric", "protocol_major": 2},
        {"record_type": "IdentityObservation",
         "identity_strata": {
             "endpoint_id": "agent-17",
             "semantic_actor_ref": "agent-17",
             "institutional_principal_ref": "agent-17",
             "controller_ref": "agent-17",
         },
         "collapsed_authority": True},
        {"record_type": "ExternalExecutionStarted", "controller_admitted": False},
    ]
    assert validate_trace(trace) == [
        "0:implicit_or_incompatible_protocol_downgrade",
        "1:identity_equality_authority_collapse",
        "2:ghos_reachability_without_controller_admission",
    ]
