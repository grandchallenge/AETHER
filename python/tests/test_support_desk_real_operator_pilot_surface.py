from __future__ import annotations

import importlib.util
import json
import subprocess
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "support_desk_real_operator_pilot.py"
RUST_SURFACE = ROOT / "crates" / "aether_api" / "examples" / "support_desk_real_operator_pilot_case.rs"


def load_surface():
    spec = importlib.util.spec_from_file_location("support_desk_real_operator_pilot", SCRIPT)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_frozen_schedule_has_twelve_tasks_per_operator() -> None:
    surface = load_surface()
    for operator_id in surface.ALLOWED_OPERATORS:
        rows = surface.schedule(operator_id)
        assert len(rows) == 12
        assert [row["order_index"] for row in rows] == list(range(1, 13))
        for case_id in surface.case_ids():
            conditions = [row["condition"] for row in rows if row["case_id"] == case_id]
            assert sorted(conditions) == ["aether", "conventional_event_log"]


def test_operator_two_reverses_operator_one_condition_order() -> None:
    surface = load_surface()
    one = surface.schedule("operator-01")
    two = surface.schedule("operator-02")
    assert [row["case_id"] for row in one] == [row["case_id"] for row in two]
    for left, right in zip(one, two, strict=True):
        assert left["condition"] != right["condition"]


def test_operator_three_uses_frozen_rotated_case_order() -> None:
    surface = load_surface()
    rows = surface.schedule("operator-03")
    first_of_each_pair = [rows[index]["case_id"] for index in range(0, 12, 2)]
    ids = surface.case_ids()
    assert first_of_each_pair == [ids[3], ids[4], ids[5], ids[0], ids[1], ids[2]]


def test_preflight_preserves_data_and_claim_boundaries() -> None:
    surface = load_surface()
    manifest = surface.preflight("operator-01")
    assert manifest["protocol_id"] == surface.PROTOCOL_ID
    assert manifest["case_set_id"] == surface.CASE_SET_ID
    assert manifest["journal_backend"] == "sqlite"
    assert manifest["data_classification"] == "synthetic"
    assert manifest["customer_private_production_data_allowed"] is False
    assert manifest["claim_boundary"] == surface.CLAIM_BOUNDARY
    assert len(manifest["fixture_sha256"]) == 64


def test_preflight_rejects_checkout_bytes_that_differ_from_commit(monkeypatch) -> None:
    surface = load_surface()
    revision = surface.current_revision()
    monkeypatch.setattr(surface, "git", lambda *args: "" if args[0] == "status" else revision)
    monkeypatch.setattr(surface, "committed_sha256", lambda path: "0" * 64)
    with pytest.raises(RuntimeError, match="pilot bytes differ from HEAD"):
        surface.preflight("operator-01")


@pytest.mark.parametrize("field", ["product_sha", "fixture_sha256", "case_set_id"])
def test_packaged_operator_rejects_wrong_identity(tmp_path, monkeypatch, field) -> None:
    surface = load_surface()
    binary = tmp_path / "operator.exe"
    binary.write_bytes(b"qualified candidate bytes")
    identity = {
        "record_type": "AETHER_SUPPORT_DESK_PACKAGED_SURFACE_V1",
        "product_sha": surface.current_revision(),
        "fixture_sha256": surface.sha256(surface.FIXTURE),
        "case_set_id": surface.CASE_SET_ID,
    }
    identity[field] = "wrong"
    monkeypatch.setattr(
        surface.subprocess, "run",
        lambda *args, **kwargs: subprocess.CompletedProcess(args, 0, json.dumps(identity)),
    )
    # current_revision must not use the mocked process runner.
    monkeypatch.setattr(surface, "current_revision", lambda: "a" * 40)
    if field != "product_sha":
        identity["product_sha"] = "a" * 40
    with pytest.raises(RuntimeError, match="does not match"):
        surface.operator_binary_identity(binary)


def test_packaged_task_launches_binary_without_cargo(tmp_path, monkeypatch) -> None:
    surface = load_surface()
    binary = tmp_path / "operator.exe"
    binary.write_bytes(b"qualified candidate bytes")
    monkeypatch.setattr(surface, "operator_binary_identity", lambda path: {})
    commands = []
    def run(command, **kwargs):
        commands.append(command)
        return subprocess.CompletedProcess(command, 0)
    monkeypatch.setattr(surface.subprocess, "run", run)
    assert surface.show_task("operator-01", 1, True, "case.sqlite", binary) == 0
    assert commands == [[str(binary.resolve()), surface.case_ids()[0], "case.sqlite"]]


def test_rust_surface_does_not_deserialize_or_render_frozen_ground_truth() -> None:
    source = RUST_SURFACE.read_text(encoding="utf-8")
    struct_body = source.split("struct PilotCase {", 1)[1].split("}", 1)[0]
    assert "ground_truth" not in struct_body
    assert "semantic_facts" in struct_body
    assert "SUPPORT_DEMO_SOURCE" in source
    assert "SOURCE_CONTRACT_MARKERS" in source


def test_surface_has_no_external_action_or_claim_widening_commands() -> None:
    raw = (SCRIPT.read_text(encoding="utf-8") + RUST_SURFACE.read_text(encoding="utf-8")).lower()
    assert "product_superiority_claim_allowed\": true" not in raw
    assert "operator_savings_claim_allowed\": true" not in raw
    assert "commercial_beta_claim_allowed\": true" not in raw
    assert "production_readiness_claim_allowed\": true" not in raw
    assert "customer_private_production_data_allowed\": true" not in raw
