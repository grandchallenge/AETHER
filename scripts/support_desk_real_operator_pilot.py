#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import statistics
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/product/support-desk-real-operator-pilot-v1.json"
SCHEMA = ROOT / "schemas/pilot/support-desk-real-operator-evidence.schema.json"
PROTOCOL = ROOT / "docs/COMMERCIALIZATION/SUPPORT_DESK_REAL_OPERATOR_PILOT_PROTOCOL.md"
RUST_SURFACE = ROOT / "crates/aether_api/examples/support_desk_real_operator_pilot_case.rs"
PROTOCOL_ID = "AETHER-SUPPORT-DESK-REAL-OPERATOR-PILOT-001"
CASE_SET_ID = "AETHER-SUPPORT-DESK-REAL-OPERATOR-CASES-001"
ALLOWED_OPERATORS = ("operator-01", "operator-02", "operator-03")
CLAIM_BOUNDARY = {
    "controlled_single_node_alpha": True,
    "product_superiority_claim_allowed": False,
    "operator_savings_claim_allowed": False,
    "commercial_beta_claim_allowed": False,
    "production_readiness_claim_allowed": False,
}
FORBIDDEN_TEXT = (
    "@gmail.com",
    "@outlook.com",
    "bearer ",
    "private_key",
    "api_key",
    "password=",
)


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def git(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args], cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False
    )
    if completed.returncode != 0:
        raise RuntimeError(completed.stderr.strip() or f"git {' '.join(args)} failed")
    return completed.stdout.strip()


def current_revision() -> str:
    revision = git("rev-parse", "HEAD")
    if len(revision) != 40 or any(ch not in "0123456789abcdef" for ch in revision):
        raise RuntimeError(f"cannot establish exact AETHER revision: {revision!r}")
    return revision


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pack() -> dict[str, Any]:
    payload = read_json(FIXTURE)
    if payload.get("schema_version") != "aether.support-desk-real-operator-case-pack.v1":
        raise RuntimeError("unexpected pilot fixture schema")
    if payload.get("case_set_id") != CASE_SET_ID or payload.get("protocol_id") != PROTOCOL_ID:
        raise RuntimeError("pilot fixture identity mismatch")
    if payload.get("data_classification") != "synthetic":
        raise RuntimeError("first-tranche fixture must remain synthetic")
    if len(payload.get("cases", [])) != 6:
        raise RuntimeError("first-tranche fixture must contain exactly six cases")
    return payload


def case_ids() -> list[str]:
    return [case["case_id"] for case in pack()["cases"]]


def schedule(operator_id: str) -> list[dict[str, Any]]:
    if operator_id not in ALLOWED_OPERATORS:
        raise ValueError(f"operator_id must be one of {', '.join(ALLOWED_OPERATORS)}")
    ids = case_ids()
    if operator_id == "operator-03":
        ids = [ids[3], ids[4], ids[5], ids[0], ids[1], ids[2]]
    rows: list[dict[str, Any]] = []
    for pair_index, case_id in enumerate(ids, start=1):
        if operator_id == "operator-01":
            first = "aether" if pair_index % 2 else "conventional_event_log"
        elif operator_id == "operator-02":
            first = "conventional_event_log" if pair_index % 2 else "aether"
        else:
            first = "aether" if pair_index % 2 else "conventional_event_log"
        second = "conventional_event_log" if first == "aether" else "aether"
        for condition in (first, second):
            rows.append(
                {
                    "order_index": len(rows) + 1,
                    "pair_index": pair_index,
                    "case_id": case_id,
                    "condition": condition,
                }
            )
    return rows


def preflight(operator_id: str) -> dict[str, Any]:
    revision = current_revision()
    tracked_dirty = git("status", "--porcelain", "--untracked-files=no")
    if tracked_dirty:
        raise RuntimeError("tracked working tree is dirty; refuse pilot execution")
    for path in (FIXTURE, SCHEMA, PROTOCOL, RUST_SURFACE):
        if not path.exists():
            raise RuntimeError(f"required pilot surface missing: {path.relative_to(ROOT)}")
    fixture = pack()
    return {
        "record_type": "AETHER_SUPPORT_DESK_OPERATOR_SESSION_MANIFEST",
        "protocol_id": PROTOCOL_ID,
        "case_set_id": CASE_SET_ID,
        "operator_id": operator_id,
        "aether_revision": revision,
        "journal_backend": "sqlite",
        "data_classification": "synthetic",
        "customer_private_production_data_allowed": False,
        "fixture_sha256": sha256(FIXTURE),
        "schema_sha256": sha256(SCHEMA),
        "protocol_sha256": sha256(PROTOCOL),
        "sanitization_provenance": fixture["sanitization_provenance"],
        "schedule": schedule(operator_id),
        "claim_boundary": CLAIM_BOUNDARY,
    }


def case_by_id(case_id: str) -> dict[str, Any]:
    for case in pack()["cases"]:
        if case["case_id"] == case_id:
            return case
    raise ValueError(f"unknown frozen case: {case_id}")


def task_for(operator_id: str, order_index: int) -> dict[str, Any]:
    rows = schedule(operator_id)
    if order_index < 1 or order_index > len(rows):
        raise ValueError("order_index must be between 1 and 12")
    return rows[order_index - 1]


def show_task(operator_id: str, order_index: int, execute_aether: bool, sqlite_path: str | None) -> int:
    task = task_for(operator_id, order_index)
    case = case_by_id(task["case_id"])
    print(f"operator={operator_id}")
    print(f"order_index={order_index}")
    print(f"case_id={task['case_id']}")
    print(f"condition={task['condition']}")
    print()
    if task["condition"] == "conventional_event_log":
        print("Conventional ticket/event-log condition")
        print("=======================================")
        for event in case["conventional_event_log"]:
            print(f"{event['event_index']:02d}  {event['at']}  {event['type']}: {event['detail']}")
        print("\nAnswer the five frozen operator questions. Ground truth is not displayed.")
        return 0

    command = [
        "cargo",
        "run",
        "-p",
        "aether_api",
        "--example",
        "support_desk_real_operator_pilot_case",
        "--release",
        "--",
        task["case_id"],
    ]
    if sqlite_path:
        command.append(sqlite_path)
    if not execute_aether:
        print("AETHER condition")
        print("================")
        print("Run exactly:")
        print(" ".join(command))
        return 0
    return subprocess.run(command, cwd=ROOT, check=False).returncode


def validate_evidence(path: Path) -> dict[str, Any]:
    try:
        from jsonschema import Draft202012Validator
    except ImportError as exc:
        raise RuntimeError("jsonschema is required; install requirements-release.txt") from exc

    evidence = read_json(path)
    schema = read_json(SCHEMA)
    Draft202012Validator.check_schema(schema)
    errors = sorted(Draft202012Validator(schema).iter_errors(evidence), key=lambda err: list(err.path))
    if errors:
        detail = "\n".join(f"- {'/'.join(map(str, err.path)) or '<root>'}: {err.message}" for err in errors)
        raise RuntimeError(f"evidence schema validation failed:\n{detail}")

    operator_id = evidence["operator"]["operator_id"]
    expected_schedule = schedule(operator_id)
    tasks = evidence["tasks"]
    if len(tasks) != 12:
        raise RuntimeError("each first-tranche operator must have exactly 12 task observations")
    observed_schedule = [
        {"order_index": task["order_index"], "case_id": task["case_id"], "condition": task["condition"]}
        for task in tasks
    ]
    expected_compact = [
        {"order_index": row["order_index"], "case_id": row["case_id"], "condition": row["condition"]}
        for row in expected_schedule
    ]
    if observed_schedule != expected_compact:
        raise RuntimeError("evidence task order does not match the frozen counterbalancing schedule")

    revision = current_revision()
    if evidence["execution"]["aether_revision"] != revision:
        raise RuntimeError(
            f"evidence revision {evidence['execution']['aether_revision']} != running revision {revision}"
        )
    if evidence["execution"]["journal_backend"] != "sqlite":
        raise RuntimeError("pilot journal backend must remain sqlite")
    expected_fixture_revision = f"sha256:{sha256(FIXTURE)}"
    if evidence["execution"]["case_fixture_revision"] != expected_fixture_revision:
        raise RuntimeError("evidence is not bound to the exact frozen fixture bytes")
    if evidence["execution"]["comparator_revision"] != expected_fixture_revision:
        raise RuntimeError("comparator must be bound to the same frozen fixture bytes")
    if evidence["data_boundary"]["classification"] != "synthetic":
        raise RuntimeError("first tranche is synthetic-only")
    if evidence["data_boundary"]["customer_private_production_data_present"] is not False:
        raise RuntimeError("customer/private production data is outside the authorized pilot")
    if evidence["claim_boundary"] != CLAIM_BOUNDARY:
        raise RuntimeError("claim boundary was widened")

    raw = path.read_text(encoding="utf-8").lower()
    for marker in FORBIDDEN_TEXT:
        if marker in raw:
            raise RuntimeError(f"evidence contains forbidden direct/private marker: {marker}")
    return evidence


def valid_tasks(evidence: dict[str, Any]) -> list[dict[str, Any]]:
    return [task for task in evidence["tasks"] if not task["invalidated"]]


def paired_summary(evidence: dict[str, Any]) -> dict[str, Any]:
    pairs: dict[str, dict[str, dict[str, Any]]] = {}
    for task in valid_tasks(evidence):
        pairs.setdefault(task["case_id"], {})[task["condition"]] = task
    complete_pairs = [conditions for conditions in pairs.values() if set(conditions) == {"aether", "conventional_event_log"}]
    duration_deltas = [
        pair["aether"]["duration_seconds"] - pair["conventional_event_log"]["duration_seconds"]
        for pair in complete_pairs
    ]
    step_deltas = [
        pair["aether"]["operator_steps"] - pair["conventional_event_log"]["operator_steps"]
        for pair in complete_pairs
    ]
    return {
        "operator_id": evidence["operator"]["operator_id"],
        "valid_tasks": len(valid_tasks(evidence)),
        "complete_pairs": len(complete_pairs),
        "median_aether_minus_conventional_duration_seconds": statistics.median(duration_deltas)
        if duration_deltas
        else None,
        "median_aether_minus_conventional_steps": statistics.median(step_deltas) if step_deltas else None,
        "interpretation": "DESCRIPTIVE_DIRECTIONAL_EVIDENCE_ONLY__NO_SUPERIORITY_OR_SAVINGS_CLAIM",
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run/validate the bounded AETHER support-desk real-operator pilot")
    sub = parser.add_subparsers(dest="command", required=True)

    prepare = sub.add_parser("prepare")
    prepare.add_argument("--operator-id", required=True, choices=ALLOWED_OPERATORS)
    prepare.add_argument("--out", required=True)

    show = sub.add_parser("show-task")
    show.add_argument("--operator-id", required=True, choices=ALLOWED_OPERATORS)
    show.add_argument("--order-index", required=True, type=int)
    show.add_argument("--execute-aether", action="store_true")
    show.add_argument("--sqlite-path")

    validate = sub.add_parser("validate")
    validate.add_argument("--evidence", required=True)

    summary = sub.add_parser("summary")
    summary.add_argument("--evidence", required=True)
    summary.add_argument("--out")
    return parser


def main() -> int:
    args = build_parser().parse_args()
    try:
        if args.command == "prepare":
            manifest = preflight(args.operator_id)
            write_json(Path(args.out), manifest)
            print(json.dumps(manifest, indent=2))
            return 0
        if args.command == "show-task":
            preflight(args.operator_id)
            return show_task(args.operator_id, args.order_index, args.execute_aether, args.sqlite_path)
        if args.command == "validate":
            validate_evidence(Path(args.evidence))
            print("AETHER_SUPPORT_DESK_OPERATOR_EVIDENCE_VALID")
            return 0
        if args.command == "summary":
            evidence = validate_evidence(Path(args.evidence))
            payload = paired_summary(evidence)
            if args.out:
                write_json(Path(args.out), payload)
            print(json.dumps(payload, indent=2))
            return 0
    except (RuntimeError, ValueError, OSError, json.JSONDecodeError) as exc:
        print(f"pilot surface error: {exc}", file=sys.stderr)
        return 2
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
