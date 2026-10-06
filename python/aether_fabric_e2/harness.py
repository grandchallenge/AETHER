from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any, Iterable

from jsonschema import Draft202012Validator

E1_SCHEMAS = (
    "mechanical_envelope.schema.json",
    "envelope_control.schema.json",
    "mechanical_event.schema.json",
    "semantic_lifecycle_event.schema.json",
    "telemetry_evidence.schema.json",
    "identity_binding.schema.json",
)

MECHANICAL_EVENTS = {
    "RouteRealized",
    "QueueAdmitted",
    "PreStartRejected",
    "PayloadDispatched",
    "PayloadDelivered",
    "DeliveryReceiptObserved",
    "ReplicaMovementObserved",
    "PhysicalObjectLocationObserved",
    "TelemetryEvidenceObserved",
}


def _load(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def load_vector_registry(root: Path) -> dict[str, Any]:
    return _load(root / "schemas" / "aether_fabric" / "e2" / "vector_registry.json")


def validate_e1_schemas(root: Path) -> dict[str, Any]:
    schema_root = root / "schemas" / "aether_fabric" / "e1"
    schemas: dict[str, dict[str, Any]] = {}
    for name in E1_SCHEMAS:
        schema = _load(schema_root / name)
        Draft202012Validator.check_schema(schema)
        schemas[name] = schema

    fixture_schema = {
        "telemetry_queue_depth.json": "telemetry_evidence.schema.json",
        "telemetry_latency.json": "telemetry_evidence.schema.json",
        "semantic_started_mechanical.json": "semantic_lifecycle_event.schema.json",
        "envelope_revoked.json": "envelope_control.schema.json",
    }
    valid_results: dict[str, bool] = {}
    for fixture_name, schema_name in fixture_schema.items():
        fixture = _load(schema_root / "examples" / "valid" / fixture_name)
        Draft202012Validator(schemas[schema_name]).validate(fixture)
        valid_results[fixture_name] = True

    invalid_path = schema_root / "examples" / "invalid" / "semantic_started_half_bound.json"
    invalid_fixture = _load(invalid_path)
    errors = list(Draft202012Validator(schemas["semantic_lifecycle_event.schema.json"]).iter_errors(invalid_fixture))
    if not errors:
        raise AssertionError("semantic_started_half_bound.json unexpectedly validated")

    def error_messages(error: Any) -> list[str]:
        messages = [error.message]
        for child in error.context:
            messages.extend(error_messages(child))
        return messages

    messages = [message for error in errors for message in error_messages(error)]
    if not any("mechanical_attempt_id" in message for message in messages):
        raise AssertionError("half-bound fixture failed for an unexpected reason")

    return {
        "schema_count": len(schemas),
        "valid_fixtures": valid_results,
        "invalid_fixture": invalid_path.name,
        "invalid_error_count": len(errors),
        "invalid_reason": "semantic_start_half_bound",
    }


def validate_trace(events: Iterable[dict[str, Any]]) -> list[str]:
    """Return deterministic E2 violations for an abstract cross-plane trace.

    The trace vocabulary deliberately mirrors E1 records but is not a runtime
    implementation. Unknown records are retained as test stimuli and only
    acquire meaning through the explicit checks below.
    """

    violations: list[str] = []
    started_attempts: set[str] = set()
    rejected_attempts: set[str] = set()
    revoked_envelopes: set[str] = set()
    pending_telemetry_event: str | None = None
    telemetry_submission_ref: str | None = None
    last_replica_event: str | None = None

    for index, event in enumerate(events):
        record = event.get("record_type")
        attempt = event.get("mechanical_attempt_id")
        envelope = event.get("envelope_id") or event.get("mechanical_envelope_id")

        if record == "SemanticExecutionStarted":
            has_envelope = bool(event.get("mechanical_envelope_id"))
            has_attempt = bool(event.get("mechanical_attempt_id"))
            if has_envelope != has_attempt:
                violations.append(f"{index}:semantic_start_half_bound")
            if has_attempt:
                if attempt in rejected_attempts:
                    violations.append(f"{index}:hidden_commit_after_pre_start_rejection")
                started_attempts.add(str(attempt))

        if record == "PreStartRejected" and attempt:
            if attempt in started_attempts:
                violations.append(f"{index}:pre_start_rejection_after_semantic_start")
            rejected_attempts.add(str(attempt))

        if record in MECHANICAL_EVENTS and event.get("authority_effect", "none") != "none":
            violations.append(f"{index}:mechanical_authority_laundering")

        if record in {"MechanicalEnvelopeAuthorized", "MechanicalEnvelopeRevoked"}:
            if event.get("authority_owner") != "upstream_governed_record":
                violations.append(f"{index}:fabric_control_authority_forgery")

        if record == "MechanicalEnvelopeRevoked" and envelope:
            revoked_envelopes.add(str(envelope))

        if record in {"RouteRealized", "QueueAdmitted"} and envelope in revoked_envelopes:
            violations.append(f"{index}:new_mechanical_work_after_revocation")

        if record == "RetryRealized":
            prior = event.get("retry_of_mechanical_attempt_id")
            if not attempt or attempt == prior:
                violations.append(f"{index}:retry_reuses_mechanical_attempt_id")
            if envelope in revoked_envelopes:
                violations.append(f"{index}:retry_after_revocation")

        if record == "ProtocolNegotiated":
            if event.get("protocol_family") != "aether-fabric" or event.get("protocol_major") != 1:
                violations.append(f"{index}:implicit_or_incompatible_protocol_downgrade")
            required = set(event.get("required_capabilities", []))
            supported = set(event.get("supported_capabilities", []))
            if not required.issubset(supported):
                violations.append(f"{index}:required_capability_missing")

        if record == "ScopeBindingChecked":
            raw = event.get("scope_bytes_utf8")
            digest = event.get("scope_digest")
            if not isinstance(raw, str) or not isinstance(digest, str):
                violations.append(f"{index}:scope_binding_missing_exact_bytes")
            elif hashlib.sha256(raw.encode("utf-8")).hexdigest() != digest:
                violations.append(f"{index}:scope_digest_mismatch")

        if record == "EnvelopeDerivationChecked":
            parent = event.get("parent", {})
            child = event.get("child", {})
            subset_fields = ("permitted_actions", "eligible_resource_classes", "trust_zones")
            widened = any(
                not set(child.get(field, [])).issubset(set(parent.get(field, [])))
                for field in subset_fields
            )
            if child.get("max_attempts", 0) > parent.get("max_attempts", 0):
                widened = True
            if child.get("max_parallel_copies", 0) > parent.get("max_parallel_copies", 0):
                widened = True
            priority = {"low": 0, "normal": 1, "high": 2, "critical": 3}
            if priority.get(child.get("priority_class"), 99) > priority.get(parent.get("priority_class"), -1):
                widened = True
            if widened:
                violations.append(f"{index}:envelope_widening")

        if record == "TelemetryEvidenceObserved":
            pending_telemetry_event = str(event.get("event_id") or "")
            telemetry_submission_ref = None
            if not pending_telemetry_event:
                violations.append(f"{index}:telemetry_missing_event_identity")

        if (
            record == "SemanticSubmissionProposed"
            and pending_telemetry_event
            and event.get("source_event_id") == pending_telemetry_event
        ):
            telemetry_submission_ref = str(event.get("submission_ref") or "")
            if not telemetry_submission_ref:
                violations.append(f"{index}:telemetry_submission_missing_identity")

        if (
            record == "SemanticAdmissionAccepted"
            and pending_telemetry_event
            and telemetry_submission_ref
            and event.get("submission_ref") == telemetry_submission_ref
        ):
            pending_telemetry_event = None
            telemetry_submission_ref = None

        if record == "AllocationDesired" and pending_telemetry_event:
            violations.append(f"{index}:unadmitted_telemetry_policy_bypass")

        if record == "ReplicaMovementObserved":
            last_replica_event = str(event.get("event_id") or "")
            if not last_replica_event:
                violations.append(f"{index}:replica_movement_missing_event_identity")

        if record in {"LeaderEpochChanged", "ReplicaPromoted"} and last_replica_event:
            if event.get("authority_owner") != "AETHER":
                violations.append(f"{index}:replica_movement_authority_promotion")
            elif (
                event.get("validated_movement_event_id") != last_replica_event
                or not event.get("aether_validation_ref")
            ):
                violations.append(f"{index}:replica_promotion_without_aether_validation")

        if record == "PhysicalObjectLocationObserved":
            if any(event.get(key) for key in ("semantic_relevant", "policy_visible", "admitted")):
                violations.append(f"{index}:physical_locality_semantic_collapse")

        if record == "ExternalExecutionStarted" and not event.get("controller_admitted", False):
            violations.append(f"{index}:ghos_reachability_without_controller_admission")

        if record == "PayloadDelivered" and event.get("semantic_admitted", False):
            violations.append(f"{index}:delivery_collapsed_into_semantic_admission")

        identities = event.get("identity_strata")
        if isinstance(identities, dict) and identities:
            values = [identities.get(k) for k in (
                "endpoint_id",
                "semantic_actor_ref",
                "institutional_principal_ref",
                "controller_ref",
            )]
            if len({v for v in values if v is not None}) == 1 and event.get("collapsed_authority", False):
                violations.append(f"{index}:identity_equality_authority_collapse")

    return violations
