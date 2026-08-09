#!/usr/bin/env python3
"""Capture and verify AETHER's hosted repository control boundary."""

from __future__ import annotations

import argparse
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_POLICY = ROOT / ".github" / "repository-controls.json"


def gh_json(endpoint: str) -> Any:
    process = subprocess.run(
        ["gh", "api", endpoint],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if process.returncode != 0:
        return {
            "_error": process.stderr.strip() or process.stdout.strip(),
            "_returncode": process.returncode,
        }
    return json.loads(process.stdout)


def git_head() -> str:
    return subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def capture_snapshots(policy: dict[str, Any]) -> dict[str, Any]:
    repository = policy["repository"]
    branch = policy["protected_branch"]["name"]
    ruleset_list = gh_json(f"repos/{repository}/rulesets?includes_parents=true")
    ruleset_details: list[dict[str, Any]] = []
    if isinstance(ruleset_list, list):
        for item in ruleset_list:
            ruleset_id = item.get("id") if isinstance(item, dict) else None
            if ruleset_id is not None:
                detail = gh_json(f"repos/{repository}/rulesets/{ruleset_id}")
                if isinstance(detail, dict):
                    detail.setdefault("_requested_ruleset_id", ruleset_id)
                ruleset_details.append(detail)
    snapshots: dict[str, Any] = {
        "branch_protection": gh_json(f"repos/{repository}/branches/{branch}/protection"),
        "rulesets": {"list": ruleset_list, "details": ruleset_details},
        "actions": gh_json(f"repos/{repository}/actions/permissions"),
        "selected_actions": gh_json(
            f"repos/{repository}/actions/permissions/selected-actions"
        ),
        "repository": gh_json(f"repos/{repository}"),
        "custom_properties": gh_json(f"repos/{repository}/properties/values"),
        "private_vulnerability_reporting": gh_json(
            f"repos/{repository}/private-vulnerability-reporting"
        ),
        "environments": {},
    }
    for name in sorted(policy["environments"]):
        snapshots["environments"][name] = {
            "environment": gh_json(f"repos/{repository}/environments/{name}"),
            "branch_policies": gh_json(
                f"repos/{repository}/environments/{name}/deployment-branch-policies"
            ),
        }
    return snapshots


def audit(policy: dict[str, Any], snapshots: dict[str, Any]) -> list[str]:
    blockers: list[str] = []
    branch_policy = policy["protected_branch"]
    protection = snapshots["branch_protection"]
    if "_error" in protection:
        if (
            policy["protection_model"]["classic_protection"]
            != "transition_only_optional_after_verified_ruleset"
        ):
            blockers.append(f"branch protection unavailable: {protection['_error']}")
    else:
        observed_checks = {
            item.get("context")
            for item in protection.get("required_status_checks", {}).get("checks", [])
        }
        missing_checks = set(branch_policy["required_status_checks"]) - observed_checks
        if missing_checks:
            blockers.append(f"required status checks missing: {sorted(missing_checks)}")
        if protection.get("required_status_checks", {}).get("strict") is not branch_policy["strict"]:
            blockers.append("required status checks strictness differs")
        reviews = protection.get("required_pull_request_reviews", {})
        if reviews.get("required_approving_review_count", 0) < branch_policy["minimum_approvals"]:
            blockers.append("pull-request approval count is below policy")
        if reviews.get("dismiss_stale_reviews") is not branch_policy["dismiss_stale_reviews"]:
            blockers.append("stale-review dismissal differs")
        for key in ("require_code_owner_reviews", "require_last_push_approval"):
            if reviews.get(key, False) is not branch_policy[key]:
                blockers.append(f"pull-request review setting differs: {key}")
        if (
            protection.get("required_conversation_resolution", {}).get("enabled")
            is not branch_policy["required_conversation_resolution"]
        ):
            blockers.append("required conversation resolution differs")
        if protection.get("enforce_admins", {}).get("enabled") is not branch_policy["enforce_admins"]:
            blockers.append("administrator enforcement differs")
        for key in ("lock_branch", "allow_force_pushes", "allow_deletions"):
            if protection.get(key, {}).get("enabled") is not branch_policy[key]:
                blockers.append(f"{key} differs")

    ruleset_snapshot = snapshots.get("rulesets", {})
    ruleset_list = ruleset_snapshot.get("list", {})
    ruleset_details = ruleset_snapshot.get("details", [])
    if isinstance(ruleset_list, dict) and "_error" in ruleset_list:
        blockers.append(f"ruleset list unavailable: {ruleset_list['_error']}")
    if not isinstance(ruleset_details, list):
        blockers.append("ruleset details unavailable")
        ruleset_details = []
    details_by_name = {
        item.get("name"): item
        for item in ruleset_details
        if isinstance(item, dict) and "_error" not in item
    }
    for policy_key in ("default_branch_ruleset", "release_tag_ruleset"):
        expected = policy["protection_model"][policy_key]
        observed = details_by_name.get(expected["name"])
        if observed is None:
            blockers.append(f"required ruleset missing: {expected['name']}")
            continue
        for key in ("target", "enforcement"):
            if observed.get(key) != expected[key]:
                blockers.append(f"ruleset setting differs for {expected['name']}: {key}")
        if observed.get("bypass_actors") != expected["bypass_actors"]:
            blockers.append(f"ruleset bypass differs: {expected['name']}")
        conditions = observed.get("conditions", {}).get("ref_name", {})
        if conditions.get("include") != expected["include"] or conditions.get("exclude") != []:
            blockers.append(f"ruleset condition differs: {expected['name']}")
        rules = {
            item.get("type"): item
            for item in observed.get("rules", [])
            if isinstance(item, dict)
        }
        if set(rules) != set(expected["required_rules"]):
            blockers.append(f"ruleset rule set differs: {expected['name']}")
            continue
        if policy_key == "default_branch_ruleset":
            status = rules["required_status_checks"].get("parameters", {})
            observed_contexts = {
                item.get("context")
                for item in status.get("required_status_checks", [])
            }
            if observed_contexts != set(branch_policy["required_status_checks"]):
                blockers.append("ruleset required status checks differ")
            if status.get("strict_required_status_checks_policy") is not branch_policy["strict"]:
                blockers.append("ruleset required status strictness differs")
            if status.get("do_not_enforce_on_create") is not False:
                blockers.append("ruleset status checks allow creation bypass")
            pull_request = rules["pull_request"].get("parameters", {})
            expected_pr = {
                "dismiss_stale_reviews_on_push": branch_policy["dismiss_stale_reviews"],
                "require_code_owner_review": branch_policy["require_code_owner_reviews"],
                "require_last_push_approval": branch_policy["require_last_push_approval"],
                "required_approving_review_count": branch_policy["minimum_approvals"],
                "required_review_thread_resolution": branch_policy["required_conversation_resolution"],
            }
            for key, expected_value in expected_pr.items():
                if pull_request.get(key) != expected_value:
                    blockers.append(f"ruleset pull-request setting differs: {key}")
            if set(pull_request.get("allowed_merge_methods", [])) != set(
                expected["allowed_merge_methods"]
            ):
                blockers.append("ruleset allowed merge methods differ")

    actions_policy = policy["actions"]
    actions = snapshots["actions"]
    selected = snapshots["selected_actions"]
    for key in ("allowed_actions", "sha_pinning_required"):
        if actions.get(key) != actions_policy[key]:
            blockers.append(f"Actions setting differs: {key}")
    for key in ("github_owned_allowed", "verified_allowed"):
        if selected.get(key) != actions_policy[key]:
            blockers.append(f"selected Actions setting differs: {key}")
    if set(selected.get("patterns_allowed", [])) != set(actions_policy["patterns_allowed"]):
        blockers.append("selected Actions allowlist differs")

    repository = snapshots["repository"]
    for key, expected in policy["repository_settings"].items():
        if repository.get(key) is not expected:
            blockers.append(f"repository setting differs: {key}")

    custom_properties = snapshots["custom_properties"]
    if isinstance(custom_properties, dict) and "_error" in custom_properties:
        blockers.append(
            f"custom properties unavailable: {custom_properties['_error']}"
        )
    else:
        observed_properties = {
            item.get("property_name"): item.get("value")
            for item in custom_properties
            if isinstance(item, dict)
        }
        if observed_properties != policy["custom_properties"]:
            blockers.append("custom property projection differs")

    observed_security = repository.get("security_and_analysis", {})
    for key, expected in policy["security"].items():
        if key == "private_vulnerability_reporting":
            observed = snapshots["private_vulnerability_reporting"]
            actual = "enabled" if observed.get("enabled") is True else "disabled"
        else:
            actual = observed_security.get(key, {}).get("status")
        if actual != expected:
            blockers.append(f"security setting differs: {key}")

    for name, expected in sorted(policy["environments"].items()):
        snapshot = snapshots["environments"].get(name, {})
        environment = snapshot.get("environment", {})
        if "_error" in environment:
            blockers.append(f"environment unavailable: {name}")
            continue
        if environment.get("can_admins_bypass") is not expected["can_admins_bypass"]:
            blockers.append(f"environment administrator bypass differs: {name}")
        reviewers = [
            item
            for item in environment.get("protection_rules", [])
            if item.get("type") == "required_reviewers"
        ]
        reviewer_count = len(reviewers[0].get("reviewers", [])) if reviewers else 0
        if reviewer_count < expected["minimum_reviewers"]:
            blockers.append(f"environment reviewer count is below policy: {name}")
        policies = snapshot.get("branch_policies", {}).get("branch_policies", [])
        observed_branches = {item.get("name") for item in policies}
        missing_branches = set(expected["allowed_branches"]) - observed_branches
        if missing_branches:
            blockers.append(
                f"environment branches missing for {name}: {sorted(missing_branches)}"
            )
    return sorted(blockers)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--policy", default=str(DEFAULT_POLICY))
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    policy = json.loads(Path(args.policy).read_text(encoding="utf-8"))
    snapshots = capture_snapshots(policy)
    blockers = audit(policy, snapshots)
    evidence = {
        "schema_version": "aether.repository-controls-evidence.v1",
        "candidate_commit_sha": git_head(),
        "captured_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "repository": policy["repository"],
        "status": "passed" if not blockers else "blocked",
        "blockers": blockers,
        "snapshots": snapshots,
    }
    output = Path(args.out)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": evidence["status"], "blockers": blockers}, sort_keys=True))
    return 0 if not blockers else 1


if __name__ == "__main__":
    raise SystemExit(main())
