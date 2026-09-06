# AETHER/FABRIC E1 Candidate Schemas

Protocol candidate: `aether-fabric/1.0`
Issue: #85
Status: specification/conformance only; not a production runtime protocol

## Purpose

These schemas make the E1 interface law mechanically inspectable before any
FABRIC runtime or code extraction exists.

Schema validation proves only structural conformance. It does not prove
upstream authority, cryptographic integrity, semantic admission, institutional
permission, constitutional authority, or GHOS controller admission.

## Schema split

### `mechanical_envelope.schema.json`

Closed mechanical realization envelope.

Key properties:

- `authority_effect` is fixed to `none`;
- `scope_ref` identifies immutable scope bytes;
- `scope_digest` is SHA-256 of the exact bytes identified by `scope_ref`;
- `derived_from_envelope_id` denotes child narrowing;
- `supersedes_envelope_id` denotes replacement and is mutually exclusive with
  child derivation in one envelope record;
- policy/fairness/retry/resource constraints are explicit;
- downstream components may narrow but never widen the parent contract.

### `envelope_control.schema.json`

Upstream-owned `MechanicalEnvelopeRevoked` control record.

- domain is `control_bridge`;
- authority owner is `upstream_governed_record`;
- `integrity_profile_ref` is required;
- FABRIC may consume a separately verified control record but may not self-issue
  it as authority;
- revocation prevents new/pre-start work and cannot erase an already-started
  AETHER semantic attempt.

### `mechanical_event.schema.json`

FABRIC-domain route, queue, transport, replica-movement and object-location
events.

- every event requires `mechanical_attempt_id`;
- every event has `authority_effect: none`;
- `PreStartRejected` is keyed to the exact attempt and includes
  `envelope_revoked` as an explicit reason;
- no record type represents semantic admission or authority.

### `semantic_lifecycle_event.schema.json`

AETHER-owned semantic start/completion/submission/admission events.

- `authority_owner` is fixed to `AETHER`;
- a mechanically realized `SemanticExecutionStarted` binds both
  `mechanical_envelope_id` and `mechanical_attempt_id`;
- the two mechanical binding fields are either both present or both absent;
- the absent case is the local/no-FABRIC AETHER path.

### `telemetry_evidence.schema.json`

Operational observation only.

- JSON numeric values include integer queue depth/replication lag and floating
  latency/cost without overlapping `oneOf` branches;
- observation remains non-authoritative until ordinary AETHER
  submission/admission.

### `identity_binding.schema.json`

Explicit cross-stratum observation/reference records.

- endpoint, semantic actor, institutional principal and GHOS controller remain
  non-collapsible;
- authorization scope references use `scope_ref` plus SHA-256 of the exact
  referenced bytes;
- a typed reference does not itself prove the upstream authority is valid.

## Fixture map

Valid candidate fixtures:

- `examples/valid/telemetry_queue_depth.json`
  - integer-valued telemetry;
- `examples/valid/telemetry_latency.json`
  - floating-valued telemetry;
- `examples/valid/semantic_started_mechanical.json`
  - AETHER semantic start bound to both envelope and mechanical attempt;
- `examples/valid/envelope_revoked.json`
  - structurally valid upstream envelope revocation control.

Invalid candidate fixture:

- `examples/invalid/semantic_started_half_bound.json`
  - carries `mechanical_envelope_id` without `mechanical_attempt_id` and must
    fail the semantic-start schema.

The presence of `valid/` and `invalid/` is declarative until E2 adds an
executable schema/conformance harness.

## Non-authority

A valid schema instance does **not** mean:

- the envelope was institutionally authorized;
- the issuer possesses the referenced authority;
- an endpoint is an authorized semantic actor;
- a controller is GHOS-admitted;
- a delivered payload is semantically admitted;
- a replica is authoritative;
- a revocation is trustworthy without the selected integrity/authority profile;
- Article IX has changed;
- FABRIC exists as an activated runtime.

A later integrity/security profile must bind the exact record and upstream
authorization. E1 intentionally does not select the final signature, credential,
key-management or attestation mechanism.

## Version behavior

All schemas in this directory require:

```text
protocol_family = aether-fabric
protocol_major  = 1
protocol_minor  = 0
```

Unknown protocol major, required record type or unsupported required capability
is a pre-start failure. No implicit downgrade is permitted.

## Custody

These schemas are currently AETHER custody artifacts because E1 defines the
boundary from existing AETHER semantic authority outward.

Their location does not decide permanent protocol custody. A future independent
FABRIC repository is not justified until E2/E3 evidence demonstrates a stable
boundary and the Council E4 conditions are met.

## Normative prose

- `docs/ARCHITECTURE/AETHER_FABRIC_E1_INTERFACE_LAW.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_EVENT_ALGEBRA.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_IDENTITY_AND_AUTHORITY.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_CONFORMANCE_VECTORS.md`
- `docs/ADR/0021-aether-fabric-e1-interface-law.md`

If schema and normative law disagree, E1 is not complete. The mismatch must be
repaired rather than choosing whichever artifact is easier to implement.
