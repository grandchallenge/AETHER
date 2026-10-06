from __future__ import annotations

import json
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator
from jsonschema.exceptions import ValidationError

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_ROOT = REPO_ROOT / "schemas" / "aether_fabric" / "e3"

SCHEMAS = {
    path.name: json.loads(path.read_text(encoding="utf-8"))
    for path in SCHEMA_ROOT.glob("*.schema.json")
}


def test_f1a_e3_schemas_are_draft_2020_12_meta_valid() -> None:
    assert set(SCHEMAS) == {
        "placement_constraint.schema.json",
        "control_state_witness.schema.json",
        "resource_snapshot.schema.json",
        "placement_decision.schema.json",
    }
    for schema in SCHEMAS.values():
        Draft202012Validator.check_schema(schema)


def selected_fixture() -> dict[str, object]:
    digest = "a" * 64
    return {
        "schema_version": "1.1",
        "protocol_family": "aether-fabric",
        "protocol_major": 1,
        "protocol_minor": 1,
        "record_type": "PlacementSelected",
        "placement_decision_id": "P1",
        "event_id": "EV1",
        "correlation_id": "C1",
        "envelope_id": "E1",
        "envelope_digest": digest,
        "placement_constraint_id": "PC1",
        "placement_constraint_digest": digest,
        "resource_snapshot_id": "S1",
        "resource_snapshot_digest": digest,
        "scheduler_policy_id": "POL1",
        "scheduler_policy_digest": digest,
        "selector_implementation_digest": digest,
        "decision_time_unix_ms": 1,
        "decision_input_digest": digest,
        "control_state_ref": "W1",
        "control_state_digest": digest,
        "resource_id": "aether-local-blocking-pool",
        "eligibility_evidence": ["eligible"],
        "selection_rank": 1,
        "authority_effect": "none",
    }


def test_placement_selected_is_valid_pre_attempt_evidence() -> None:
    validator = Draft202012Validator(SCHEMAS["placement_decision.schema.json"])
    validator.validate(selected_fixture())


def test_placement_decision_rejects_mechanical_attempt_id() -> None:
    fixture = selected_fixture()
    fixture["mechanical_attempt_id"] = "M1"
    validator = Draft202012Validator(SCHEMAS["placement_decision.schema.json"])
    with pytest.raises(ValidationError):
        validator.validate(fixture)


def test_e3_constraint_rejects_implicit_protocol_downgrade() -> None:
    fixture = {
        "schema_version": "1.1",
        "protocol_family": "aether-fabric",
        "protocol_major": 1,
        "protocol_minor": 0,
        "record_type": "PlacementConstraintSet",
        "constraint_set_id": "PC1",
        "envelope_id": "E1",
        "envelope_digest": "a" * 64,
        "correlation_id": "C1",
        "resource_requirements": {
            "capacity_unit": "blocking_admission_slot",
            "minimum_capacity_units": 1,
        },
        "max_snapshot_age_ms": 100,
        "decision_time_unix_ms": 1,
        "control_state_ref": "W1",
        "control_state_digest": "b" * 64,
        "source_authority_ref": "AUTH",
        "authority_effect": "mechanical_narrowing_only",
    }
    validator = Draft202012Validator(SCHEMAS["placement_constraint.schema.json"])
    with pytest.raises(ValidationError):
        validator.validate(fixture)
