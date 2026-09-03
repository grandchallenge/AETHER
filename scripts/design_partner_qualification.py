#!/usr/bin/env python3
"""Bounded design-partner qualification for the AETHER support-desk exemplar.

This is a product-decision harness, not a semantic authority or release-promotion
mechanism. It deliberately reuses the existing customer-workflow acceptance
runner, binds the actual support DSL contract, and adds a synthetic conventional
workflow comparator. Comparator units are structural proxies only; they are not
human-time, monetary-cost, or product-superiority evidence.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures" / "product" / "dpq-support-desk-v1.json"
CUSTOMER_ACCEPTANCE = ROOT / "scripts" / "customer_workflow_acceptance.py"
DEMO_SOURCE = ROOT / "crates" / "aether_api" / "examples" / "demo_05_ai_support_resolution_desk.rs"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8", newline="\n")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8", newline="\n")


def current_revision() -> str:
    completed = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    return completed.stdout.strip() if completed.returncode == 0 else "UNKNOWN"


def run_existing_customer_acceptance(timeout_seconds: int) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="aether-dpq-") as tmp:
        out_json = Path(tmp) / "customer-workflow.json"
        out_md = Path(tmp) / "customer-workflow.md"
        command = [
            sys.executable,
            str(CUSTOMER_ACCEPTANCE),
            "run",
            "--out-json",
            str(out_json),
            "--out-md",
            str(out_md),
            "--timeout-seconds",
            str(timeout_seconds),
            "--enforce",
        ]
        completed = subprocess.run(
            command,
            cwd=ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=timeout_seconds + 60,
            check=False,
        )
        payload = load_json(out_json) if out_json.exists() else {
            "workflow_ready": False,
            "gates": [],
        }
        payload["runner_exit_code"] = completed.returncode
        payload["runner_output_tail"] = "\n".join(completed.stdout.splitlines()[-80:])
        return payload


def runtime_output(customer_payload: dict[str, Any]) -> str:
    parts: list[str] = []
    for gate in customer_payload.get("gates", []):
        tail = gate.get("output_tail")
        if isinstance(tail, str):
            parts.append(tail)
    return "\n".join(parts)


def missing_markers(text: str, markers: list[str]) -> list[str]:
    return [marker for marker in markers if marker not in text]


def scenario_state(events: list[dict[str, Any]]) -> dict[str, Any]:
    resolution: dict[str, Any] = {}
    dependency_states: list[str] = []
    evidence_present = False
    assignments: list[dict[str, Any]] = []
    case_open = False

    for event in events:
        kind = event.get("kind")
        if kind == "case":
            case_open = event.get("status") == "open"
        elif kind == "resolution":
            resolution.update(event)
        elif kind == "dependency":
            dependency_states.append(str(event.get("status")))
        elif kind == "evidence":
            evidence_present = evidence_present or event.get("present") is True
        elif kind == "assignment":
            assignments.append(event)

    active_assignments = [item for item in assignments if item.get("state") == "active"]
    current_assignment = max(active_assignments, key=lambda item: int(item.get("epoch", -1)), default=None)
    current_owner = current_assignment.get("owner") if current_assignment else None
    current_epoch = int(current_assignment.get("epoch", -1)) if current_assignment else None
    stale_owners = [
        str(item.get("owner"))
        for item in assignments
        if item.get("owner") is not None
        and (current_owner is None or item.get("owner") != current_owner or int(item.get("epoch", -1)) != current_epoch)
    ]

    dependencies_complete = bool(dependency_states) and all(state == "done" for state in dependency_states)
    approval = resolution.get("approval")
    suppression = resolution.get("suppression")
    confidence = resolution.get("confidence")

    if approval != "approved":
        why = "approval_missing"
    elif not dependencies_complete:
        why = "dependency_incomplete"
    elif not evidence_present:
        why = "evidence_missing"
    elif suppression == "suppressed":
        why = "suppressed"
    elif confidence != "high":
        why = "confidence_low"
    elif current_owner is not None:
        why = "already_claimed"
    elif not case_open:
        why = "case_not_open"
    else:
        why = "evidence+approval+dependency+confidence"

    ready = why == "evidence+approval+dependency+confidence"
    return {
        "ready": ready,
        "current_owner": current_owner,
        "stale_owners": stale_owners,
        "why": why,
    }


def evaluate_scenario(scenario: dict[str, Any]) -> dict[str, Any]:
    events = list(scenario.get("events", []))
    questions = list(scenario.get("questions", []))
    expected = dict(scenario.get("expected", {}))
    state = scenario_state(events)
    observed = {question: state.get(question) for question in questions}
    expected_for_questions = {question: expected.get(question) for question in questions}
    passed = observed == expected_for_questions

    # The comparator intentionally models a naive event-log workflow. Each
    # question requires a complete event scan to reconstruct current state.
    # AETHER query units model one semantic query per question. These units are
    # structural proxies only and cannot be translated into human time/cost.
    baseline_inspection_units = len(events) * len(questions)
    aether_query_proxy_units = len(questions)
    return {
        "id": scenario.get("id"),
        "purpose": scenario.get("purpose"),
        "passed": passed,
        "expected": expected_for_questions,
        "observed": observed,
        "proxy": {
            "baseline_event_log_inspection_units": baseline_inspection_units,
            "aether_semantic_query_units": aether_query_proxy_units,
            "unit_definition": "synthetic structural reconstruction proxy; not human time or monetary cost",
        },
    }


def build_report(args: argparse.Namespace) -> dict[str, Any]:
    fixture = load_json(FIXTURE)
    customer = run_existing_customer_acceptance(args.timeout_seconds)
    output = runtime_output(customer)
    source = DEMO_SOURCE.read_text(encoding="utf-8")

    missing_runtime = missing_markers(output, list(fixture["runtime_markers"]))
    missing_contract = missing_markers(source, list(fixture["source_contract_markers"]))
    scenarios = [evaluate_scenario(item) for item in fixture["scenarios"]]

    workflow_passed = customer.get("workflow_ready") is True and customer.get("runner_exit_code") == 0
    scenario_passed = all(item["passed"] for item in scenarios)
    technical_acceptance = workflow_passed and not missing_runtime and not missing_contract and scenario_passed

    baseline_units = sum(item["proxy"]["baseline_event_log_inspection_units"] for item in scenarios)
    aether_units = sum(item["proxy"]["aether_semantic_query_units"] for item in scenarios)
    ratio = round(aether_units / baseline_units, 4) if baseline_units else None

    generated_at = args.generated_at or datetime.now(timezone.utc).replace(microsecond=0).isoformat()
    return {
        "record_type": "AETHER_DESIGN_PARTNER_QUALIFICATION",
        "schema_version": fixture["schema_version"],
        "qualification_id": fixture["qualification_id"],
        "generated_at": generated_at,
        "subject_revision": current_revision(),
        "claim_boundary": fixture["claim_boundary"],
        "technical_acceptance": "PASS" if technical_acceptance else "FAIL",
        "productization_signal": (
            "TECHNICAL_DPQ_PASS__PRODUCT_SUPERIORITY_UNESTABLISHED"
            if technical_acceptance
            else "DPQ_BLOCKED_WITH_EXACT_DEFECTS"
        ),
        "existing_customer_workflow_acceptance": {
            "passed": workflow_passed,
            "runner_exit_code": customer.get("runner_exit_code"),
            "workflow_ready": customer.get("workflow_ready"),
        },
        "runtime_contract": {
            "missing_runtime_markers": missing_runtime,
            "missing_source_contract_markers": missing_contract,
            "passed": not missing_runtime and not missing_contract,
        },
        "scenarios": scenarios,
        "synthetic_comparator": {
            "baseline_event_log_inspection_units": baseline_units,
            "aether_semantic_query_units": aether_units,
            "query_to_inspection_ratio": ratio,
            "interpretation": "STRUCTURAL_PROXY_ONLY__NO_HUMAN_TIME_COST_OR_PRODUCT_SUPERIORITY_CLAIM",
        },
        "metric_boundary": fixture["metric_boundary"],
        "decision_boundary": {
            "may_support_bounded_design_partner_technical_acceptance": technical_acceptance,
            "may_establish_product_superiority": False,
            "may_establish_human_operator_cost_savings": False,
            "may_promote_commercial_beta": False,
            "may_promote_production_readiness": False,
            "next_evidence_if_technical_passes": [
                "bind an exact supported/unsupported pilot matrix",
                "measure real operator effort with human/design-partner participants before making cost or superiority claims",
                "prioritize only product-wedge remediation that blocks the bounded pilot",
            ],
        },
    }


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# AETHER Design-Partner Qualification — AI Support Resolution Desk",
        "",
        f"- Qualification: `{payload['qualification_id']}`",
        f"- Subject: `{payload['subject_revision']}`",
        f"- Claim boundary: `{payload['claim_boundary']}`",
        f"- Technical acceptance: `{payload['technical_acceptance']}`",
        f"- Productization signal: `{payload['productization_signal']}`",
        "",
        "> Comparator units are synthetic reconstruction proxies only. They are not human-time, monetary-cost, or product-superiority evidence.",
        "",
        "## Runtime and contract",
        "",
        f"- Existing customer-workflow acceptance passed: `{payload['existing_customer_workflow_acceptance']['passed']}`",
        f"- Missing runtime markers: `{payload['runtime_contract']['missing_runtime_markers']}`",
        f"- Missing source-contract markers: `{payload['runtime_contract']['missing_source_contract_markers']}`",
        "",
        "## Fail-capable scenarios",
        "",
        "| Scenario | Result | Expected | Observed | Baseline inspection units | AETHER query units |",
        "| --- | --- | --- | --- | ---: | ---: |",
    ]
    for scenario in payload["scenarios"]:
        lines.append(
            "| `{id}` | `{result}` | `{expected}` | `{observed}` | {baseline} | {aether} |".format(
                id=scenario["id"],
                result="PASS" if scenario["passed"] else "FAIL",
                expected=json.dumps(scenario["expected"], sort_keys=True),
                observed=json.dumps(scenario["observed"], sort_keys=True),
                baseline=scenario["proxy"]["baseline_event_log_inspection_units"],
                aether=scenario["proxy"]["aether_semantic_query_units"],
            )
        )

    comparator = payload["synthetic_comparator"]
    lines.extend(
        [
            "",
            "## Synthetic conventional-workflow comparator",
            "",
            f"- Event-log inspection units: `{comparator['baseline_event_log_inspection_units']}`",
            f"- AETHER semantic-query units: `{comparator['aether_semantic_query_units']}`",
            f"- Query/inspection ratio: `{comparator['query_to_inspection_ratio']}`",
            f"- Interpretation: `{comparator['interpretation']}`",
            "",
            "This comparison only shows that a semantic query surface can compress a deliberately naive event-log reconstruction procedure. It does not show that real support operators are faster, cheaper, more accurate, or more satisfied with AETHER.",
            "",
            "## Decision boundary",
            "",
            f"- Bounded design-partner technical acceptance supported: `{payload['decision_boundary']['may_support_bounded_design_partner_technical_acceptance']}`",
            "- Product superiority established: `false`",
            "- Human operator-cost savings established: `false`",
            "- Commercial beta promoted: `false`",
            "- Production readiness promoted: `false`",
            "",
            "### Next evidence after a technical pass",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["decision_boundary"]["next_evidence_if_technical_passes"])
    lines.append("")
    return "\n".join(lines)


def cmd_run(args: argparse.Namespace) -> int:
    payload = build_report(args)
    write_json(Path(args.out_json), payload)
    write_text(Path(args.out_md), render_markdown(payload))
    if args.enforce and payload["technical_acceptance"] != "PASS":
        print("design-partner qualification failed", file=sys.stderr)
        return 3
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run bounded AETHER support-desk design-partner qualification")
    subparsers = parser.add_subparsers(dest="command", required=True)
    run = subparsers.add_parser("run")
    run.add_argument("--out-json", required=True)
    run.add_argument("--out-md", required=True)
    run.add_argument("--timeout-seconds", type=int, default=180)
    run.add_argument("--generated-at")
    run.add_argument("--enforce", action="store_true")
    run.set_defaults(func=cmd_run)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
