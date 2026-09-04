from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "fixtures" / "product" / "support-desk-real-operator-pilot-v1.json"


def load_fixture() -> dict:
    return json.loads(FIXTURE.read_text(encoding="utf-8"))


def test_case_pack_identity_and_data_boundary() -> None:
    fixture = load_fixture()
    assert fixture["schema_version"] == "aether.support-desk-real-operator-case-pack.v1"
    assert fixture["case_set_id"] == "AETHER-SUPPORT-DESK-REAL-OPERATOR-CASES-001"
    assert fixture["protocol_id"] == "AETHER-SUPPORT-DESK-REAL-OPERATOR-PILOT-001"
    assert fixture["authority"].endswith("APPROVE_A_SYNTHETIC_OR_SANITIZED_REAL_OPERATOR_PILOT")
    assert fixture["protected_protocol_revision"] == "7f1a169d9c078792f057c0a72d60338acbb600c9"
    assert fixture["dpq_002_qualified_subject"] == "f0f20532c40fcb389e55cdf10c44e5a3ac1423e9"
    assert fixture["data_classification"] == "synthetic"
    assert "No customer/private production source was used" in fixture["sanitization_provenance"]


def test_five_operator_questions_are_frozen() -> None:
    fixture = load_fixture()
    assert fixture["operator_questions"] == [
        "What support cases are active now?",
        "What evidence is available for this case?",
        "Which resolution, if any, is actually ready?",
        "Who owns the case now, and what assignment is stale?",
        "Why is the current selected resolution true or not true?",
    ]


def test_comparator_is_not_intentionally_handicapped() -> None:
    comparator = load_fixture()["comparator_contract"]
    assert comparator["condition_id"] == "conventional_event_log"
    assert comparator["no_intentional_handicap"] is True
    assert comparator["facts_must_match_aether_condition"] is True
    assert comparator["operator_may_filter_or_sort"] is True
    assert comparator["operator_may_not_receive_derived_current_state"] is True


def test_six_unique_predeclared_cases_with_complete_ground_truth() -> None:
    cases = load_fixture()["cases"]
    assert len(cases) == 6
    ids = [case["case_id"] for case in cases]
    assert len(set(ids)) == 6

    required_suffixes = {
        "normal-resolution",
        "missing-approval",
        "dependency-incomplete",
        "handoff-stale-fencing",
        "suppressed-resolution",
        "closed-case-fencing",
    }
    assert {case_id.removeprefix("pilot-case-01-") if case_id.startswith("pilot-case-01-") else
            case_id.removeprefix("pilot-case-02-") if case_id.startswith("pilot-case-02-") else
            case_id.removeprefix("pilot-case-03-") if case_id.startswith("pilot-case-03-") else
            case_id.removeprefix("pilot-case-04-") if case_id.startswith("pilot-case-04-") else
            case_id.removeprefix("pilot-case-05-") if case_id.startswith("pilot-case-05-") else
            case_id.removeprefix("pilot-case-06-")
            for case_id in ids} == required_suffixes

    ground_truth_keys = {
        "active_cases",
        "evidence",
        "ready_resolution",
        "current_owner",
        "stale_owners",
        "selected_resolution",
        "why",
    }
    for case in cases:
        assert set(case["ground_truth"]) == ground_truth_keys
        assert case["semantic_facts"]
        assert case["conventional_event_log"]


def test_conventional_logs_are_strictly_ordered_and_unique() -> None:
    for case in load_fixture()["cases"]:
        events = case["conventional_event_log"]
        indexes = [event["event_index"] for event in events]
        assert indexes == list(range(1, len(events) + 1))
        assert len({event["at"] for event in events}) == len(events)
        assert all(event["type"] and event["detail"] for event in events)


def test_closed_case_fails_ready_and_selected_but_preserves_owner_history() -> None:
    case = next(
        case
        for case in load_fixture()["cases"]
        if case["case_id"] == "pilot-case-06-closed-case-fencing"
    )
    truth = case["ground_truth"]
    assert truth["active_cases"] == []
    assert truth["ready_resolution"] is None
    assert truth["selected_resolution"] is None
    assert truth["current_owner"] == "lead-ana"
    assert "closed" in truth["why"].lower()


def test_no_obvious_private_data_or_secret_markers_in_case_pack() -> None:
    raw = FIXTURE.read_text(encoding="utf-8").lower()
    forbidden = [
        "@gmail.com",
        "@outlook.com",
        "api_key",
        "private_key",
        "password=",
        "bearer ",
        "customer production",
    ]
    for marker in forbidden:
        assert marker not in raw
