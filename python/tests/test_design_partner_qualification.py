from __future__ import annotations

import importlib.util
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT_PATH = REPO_ROOT / "scripts" / "design_partner_qualification.py"
FIXTURE_PATH = REPO_ROOT / "fixtures" / "product" / "dpq-support-desk-v1.json"
DEMO_SOURCE = REPO_ROOT / "crates" / "aether_api" / "examples" / "demo_05_ai_support_resolution_desk.rs"


def load_module():
    spec = importlib.util.spec_from_file_location("design_partner_qualification", SCRIPT_PATH)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class DesignPartnerQualificationTests(unittest.TestCase):
    def test_fixture_preserves_claim_boundary(self) -> None:
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))

        self.assertEqual(fixture["claim_boundary"], "controlled_single_node_alpha")
        self.assertFalse(fixture["metric_boundary"]["human_time_claim_allowed"])
        self.assertFalse(fixture["metric_boundary"]["monetary_cost_claim_allowed"])
        self.assertFalse(fixture["metric_boundary"]["product_superiority_claim_allowed"])
        self.assertFalse(fixture["synthetic_evaluator_boundary"]["semantic_authority"])

    def test_runtime_markers_are_enforced_by_existing_acceptance(self) -> None:
        module = load_module()
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))

        self.assertEqual(
            module.missing_required_items(
                module.enforced_customer_runtime_markers(),
                fixture["runtime_markers"],
            ),
            [],
        )

    def test_source_contract_markers_bind_real_support_rules(self) -> None:
        module = load_module()
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))
        source = DEMO_SOURCE.read_text(encoding="utf-8")

        self.assertEqual(module.missing_markers(source, fixture["source_contract_markers"]), [])

    def test_all_synthetic_scenarios_reconstruct_expected_state(self) -> None:
        module = load_module()
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))

        results = [module.evaluate_scenario(item) for item in fixture["scenarios"]]
        self.assertTrue(results)
        self.assertTrue(all(item["passed"] for item in results))

    def test_missing_approval_fails_closed(self) -> None:
        module = load_module()
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))
        scenario = next(item for item in fixture["scenarios"] if item["id"] == "missing_approval_block")

        state = module.scenario_state(scenario["events"])
        self.assertFalse(state["ready"])
        self.assertEqual(state["why"], "approval_missing")

    def test_handoff_marks_prior_owner_stale(self) -> None:
        module = load_module()
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))
        scenario = next(item for item in fixture["scenarios"] if item["id"] == "handoff_and_stale_fencing")

        state = module.scenario_state(scenario["events"])
        self.assertEqual(state["current_owner"], "lead-ana")
        self.assertEqual(state["stale_owners"], ["triage-agent"])
        self.assertFalse(state["ready"])
        self.assertEqual(state["why"], "already_claimed")

    def test_case_status_does_not_silently_extend_current_readiness_rule(self) -> None:
        module = load_module()
        events = [
            {"kind": "case", "status": "closed"},
            {"kind": "dependency", "status": "done"},
            {"kind": "resolution", "approval": "approved", "suppression": "clear", "confidence": "high"},
            {"kind": "evidence", "present": True},
        ]

        state = module.scenario_state(events)
        self.assertTrue(state["ready"])
        self.assertEqual(state["why"], "evidence+approval+dependency+confidence")

    def test_absent_dependency_does_not_silently_create_a_block(self) -> None:
        module = load_module()
        events = [
            {"kind": "resolution", "approval": "approved", "suppression": "clear", "confidence": "high"},
            {"kind": "evidence", "present": True},
        ]

        state = module.scenario_state(events)
        self.assertTrue(state["ready"])
        self.assertEqual(state["why"], "evidence+approval+dependency+confidence")

    def test_comparator_units_are_structural_proxy_only(self) -> None:
        module = load_module()
        fixture = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))

        results = [module.evaluate_scenario(item) for item in fixture["scenarios"]]
        baseline = sum(item["proxy"]["baseline_event_log_inspection_units"] for item in results)
        aether = sum(item["proxy"]["aether_semantic_query_units"] for item in results)

        self.assertGreater(baseline, aether)
        for item in results:
            self.assertIn("not human time or monetary cost", item["proxy"]["unit_definition"])


if __name__ == "__main__":
    unittest.main()
