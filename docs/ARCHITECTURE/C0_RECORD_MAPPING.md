# E3B-C0-C — Record Mapping and Identity Ledger

Status: C0 synthesis input
Issue: #113
Protected source basis: `0a1552adf719dbe57667a9b8705b986f0f381483`

## 1. Canonical record chain

The first-lane control chain is:

```text
AetherOperationAdmissionDecision
  -> MechanicalAuthorizationDecision
  -> MechanicalEnvelopeAuthorized
  -> PlacementConstraintSet
  -> ControlStateWitness
  -> PlacementSelected | PlacementUnavailable
  -> fresh ControlStateWitness
  -> RouteRealized | rejection
```

The first two records are AETHER-owned C0 types. E1/E3 records retain their
protected schemas.

## 2. AetherOperationAdmissionDecision

Purpose: typed projection of AETHER's decision that one exact request may
attempt one exact operation class.

Proposed fields:

| Field | Source |
| --- | --- |
| `schema_version` | C0 contract, initially `1.0` |
| `record_type` | constant `AetherOperationAdmissionDecision` |
| `operation_admission_id` | deterministic digest identity |
| `request_id` | AETHER request/correlation generator; provenance only |
| `operation_class` | protected C0 operation-profile registry |
| `namespace_ref` | resolved `NamespaceId` |
| `principal_ref` | authenticated principal ID; anonymous only where current AETHER configuration legitimately permits it |
| `token_ref` | token ID when present; provenance only |
| `required_scope` | protected endpoint profile |
| `effective_policy_digest` | canonical effective policy context or explicit public-policy marker |
| `operation_manifest_ref` | immutable manifest reference |
| `operation_manifest_digest` | SHA-256 exact manifest bytes |
| `operation_profile_ref` | protected profile identifier |
| `operation_profile_digest` | exact profile bytes |
| `decision` | `admitted | rejected` |
| `decision_revision` | AETHER issuer/admission revision |
| `decided_at_unix_ms` | AETHER decision clock |
| `source_authority_ref` | stable AETHER semantic-edge authority identity |
| `authority_effect` | `operation_attempt_only` |

### Identity

`operation_admission_id` is the SHA-256 framed digest of:

- canonical record fields excluding the ID itself;
- exact operation-manifest bytes;
- exact operation-profile bytes;
- source-authority revision.

HTTP request ID alone is not the identity.

## 3. MechanicalAuthorizationDecision

Purpose: mechanical-only projection of an admitted operation into the exact
mechanical profile that may be realized.

Proposed fields:

| Field | Source |
| --- | --- |
| `mechanical_authorization_id` | deterministic digest identity |
| `operation_admission_ref` | exact admitted AETHER operation-admission ID |
| `operation_admission_digest` | canonical admitted record digest |
| `correlation_id` | request/correlation ID, non-authoritative |
| `scope_manifest_ref` | immutable C0 scope manifest |
| `scope_manifest_digest` | SHA-256 exact bytes |
| `payload_manifest_ref` | immutable C0 payload manifest |
| `payload_manifest_digest` | SHA-256 exact bytes |
| `mechanical_profile_ref` | protected first-lane profile |
| `mechanical_profile_digest` | exact profile bytes |
| `expires_at` | bounded by current request/operation timeout profile |
| `issuer_ref` | `AetherMechanicalAuthorityIssuer` identity/revision |
| `decision_revision` | issuer registry revision |
| `decided_at_unix_ms` | issuer decision time |
| `authority_effect` | `mechanical_only` |

The decision has no resource-selection result.

## 4. Immutable operation manifest

C1 shall construct one deterministic manifest from typed AETHER request data
before issuer decision.

Profile:
`aether-operation-manifest/1`.

Required logical contents:

- operation class;
- namespace;
- semantic/request input reference;
- effective policy digest;
- exact typed request payload digest;
- protected operation-profile digest.

The manifest is encoded as deterministic UTF-8 JSON under the existing
`gcl-cjson-set-v1` canonicalization rules where applicable.

The exact emitted bytes are retained or reproducible and are what
`scope_digest` binds.

This does not claim that the original HTTP wire JSON bytes are canonical
semantic input. The typed AETHER request manifest is the admitted scope object.

## 5. MechanicalEnvelopeAuthorized field mapping

The protected E1 schema remains unchanged.

| E1 field | C0 source |
| --- | --- |
| `schema_version` | constant `1.0` |
| `protocol_family` | `aether-fabric` |
| `protocol_major/minor` | `1/0` |
| `record_type` | `MechanicalEnvelopeAuthorized` |
| `domain` | `control_bridge` |
| `authority_owner` | `upstream_governed_record` |
| `authority_effect` | `mechanical_only` |
| `envelope_id` | deterministic digest of mechanical authorization + envelope projection |
| `correlation_id` | mechanical decision correlation ID |
| `authorization_ref` | exact `mechanical_authorization_id` |
| `issuer_ref` | exact AETHER issuer identity/revision |
| `scope_ref` | operation scope-manifest reference |
| `scope_digest` | SHA-256 exact scope-manifest bytes |
| `payload_ref` | payload-manifest digest/reference |
| `permitted_actions` | operation profile; first lane = `queue_pre_start` |
| `eligible_resource_classes` | first lane = `local-blocking-pool` |
| `trust_zones` | first lane = `aether-process` |
| `locality_constraints` | first lane = `local-process` |
| `mechanical_policy_ref` | exact protected mechanical-profile ref |
| `priority_class` | first lane = `normal` unless narrower protected profile |
| `fairness_policy_ref` | exact AETHER bounded-admission policy ref |
| `retry_policy.max_attempts` | first lane = 1 |
| `retry_policy.backoff_class` | first lane = `none` |
| `redundancy_policy.max_parallel_copies` | first lane = 1 |
| `expires_at` | mechanical authorization expiry |
| `required_capabilities` | first lane = `blocking_execution` |
| `integrity_profile_ref` | exact C0-D integrity profile |

No first-lane envelope contains `derived_from_envelope_id` or
`supersedes_envelope_id` unless a later protected contract explicitly uses
those lifecycle operations.

## 6. PlacementConstraintSet mapping

| E3 field | C0 source |
| --- | --- |
| `constraint_set_id` | deterministic digest of exact projection |
| `envelope_id` | exact E1 envelope |
| `envelope_digest` | SHA-256 exact E1 bytes |
| `correlation_id` | inherited non-authoritative correlation |
| `resource_requirements.capacity_unit` | `blocking_admission_slot` |
| `minimum_capacity_units` | first lane = 1 |
| `max_snapshot_age_ms` | first-lane profile, C1 default 1000 ms |
| `decision_time_unix_ms` | AETHER projector time |
| `control_state_ref` | exact current witness ID |
| `control_state_digest` | exact current witness digest |
| `source_authority_ref` | AETHER control-bridge projector identity |
| `authority_effect` | `mechanical_narrowing_only` |
| optional resource/trust/capability/locality sets | exact subsets/strengthenings of E1 |

A constraint projection that widens any E1 dimension is invalid.

## 7. ControlStateWitness mapping

| E3 field | C0 source |
| --- | --- |
| `witness_id` | digest of envelope identity + registry revision + observation time/state |
| `envelope_id` | registry key |
| `envelope_digest` | exact registered E1 digest |
| `e1_expires_at` | exact E1 expiry |
| `observed_state` | authoritative registry state |
| `observed_revision` | monotonic registry revision token |
| `observed_at_unix_ms` | AETHER registry observation time |
| `source_authority_ref` | AETHER envelope-control registry identity |
| `authority_effect` | `none` |

The witness is an observation of AETHER authority state, not FABRIC
self-attestation.

## 8. PlacementSelected mapping

The protected F1B selector remains the sole producer.

C0 adds no fields to the E3 schema.

Required bindings already present:

- exact envelope ID/digest;
- exact constraint ID/digest;
- exact resource snapshot ID/digest;
- exact scheduler policy ID/digest;
- exact selector implementation digest;
- exact decision time/input digest;
- exact control-state ref/digest;
- selected resource;
- selection rank;
- `authority_effect = none`.

C1/C2 must supply the protected selector implementation identity from the exact
build artifact used for the decision. A human-readable version string is
insufficient.

## 9. RouteRealized identity handoff

`mechanical_attempt_id` does not exist in any C0 admission, envelope,
constraint, witness or placement record.

It is created only by the protected E3B realization bridge after:

1. exact placement validation;
2. fresh current witness;
3. current resource admissibility;
4. realization-request identity validation.

Exact replay of one realization request is idempotent. A distinct retry request
creates a new attempt ID.

## 10. Identity non-collapse ledger

These identities are distinct types even if strings happen to match:

| Identity | Authority/meaning |
| --- | --- |
| HTTP request ID | correlation only |
| token ID | credential provenance |
| institutional principal | human/institutional actor reference |
| AETHER principal/semantic actor | operation actor |
| namespace ID | semantic partition |
| operation admission ID | AETHER permission to attempt operation |
| mechanical authorization ID | AETHER mechanical projection |
| envelope ID | closed mechanical authorization |
| constraint-set ID | narrowing projection |
| witness ID | point-in-time control-state observation |
| placement decision ID | non-authoritative FABRIC decision |
| endpoint resource ID | mechanical resource |
| realization request ID | idempotency key for realization boundary |
| mechanical attempt ID | one realized mechanical attempt |
| semantic attempt ID | one AETHER semantic lifecycle attempt |
| controller ID | GHOS/controller authority, if any |

Forbidden implications include:

```text
token_id == principal_id          !=> same authority type
request_id == correlation_id     !=> authorization
resource_id == principal_id      !=> actor authority
placement_decision_id            !=> mechanical_attempt_id
mechanical_attempt_id            !=> semantic_attempt_id
endpoint reachability            !=> controller admission
```

## 11. First-lane protected profile identities

C0 freezes these logical profile names for C1 implementation:

- operation manifest: `aether-operation-manifest/1`;
- operation admission: `aether-operation-admission/1`;
- mechanical authorization: `aether-mechanical-authorization/1`;
- local mechanical profile: `aether-http-local-blocking/1`;
- fairness/admission profile: `aether-bounded-admission/1`;
- in-process integrity profile: `aether-control-bridge-inproc/1`.

Profile names are identifiers for exact protected bytes. C1 must ship the bytes
and their digests; strings alone are not integrity evidence.

## 12. C0-C disposition

Every authority-bearing E1/E3 field used by the first lane now has a named
AETHER/FABRIC source.

No field derives authority from capacity, liveness, queue availability,
successful transport or string-equal identities.
