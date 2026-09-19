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

Closed `control_bridge` record rooted in an upstream governed scope.

- `authority_owner = upstream_governed_record`;
- `authority_effect = mechanical_only`;
- the envelope authorizes only its bounded physical actions;
- `scope_ref` identifies immutable bytes and `scope_digest` is SHA-256 of those
  exact bytes;
- `derived_from_envelope_id` denotes child narrowing;
- `supersedes_envelope_id` denotes replacement and is mutually exclusive with
  child derivation in one record;
- `integrity_profile_ref` is required;
- schema validity alone does not prove the upstream authorization is genuine.

### `envelope_control.schema.json`

Upstream-owned `MechanicalEnvelopeRevoked` control record.

- domain is `control_bridge`;
- authority owner is `upstream_governed_record`;
- `integrity_profile_ref` is required;
- a separately verified revocation stops only new/pre-start use;
- FABRIC may consume a verified revocation but may not self-issue one as
  authority;
- revocation cannot erase an already-started AETHER semantic attempt.

### `mechanical_event.schema.json`

FABRIC-domain route, queue, transport, replica-movement and object-location
events.

- every event requires `mechanical_attempt_id`;
- every event has `authority_effect = none`;
- `PreStartRejected` is keyed to the exact attempt;
- no record type represents semantic admission or authority.

### `semantic_lifecycle_event.schema.json`

AETHER-owned semantic start/completion/submission/admission events.

- `authority_owner = AETHER`;
- a mechanically realized `SemanticExecutionStarted` binds both
  `mechanical_envelope_id` and `mechanical_attempt_id`;
- the two mechanical binding fields are either both present or both absent;
- both absent is the local/no-FABRIC AETHER path.

### `telemetry_evidence.schema.json`

Operational observations only.

- JSON numbers include integer queue depth/replication lag and floating
  latency/cost without overlapping `oneOf` branches;
- an observation remains non-authoritative until ordinary AETHER
  submission/admission.

### `identity_binding.schema.json`

Explicit cross-stratum observation/reference records.

- endpoint, semantic actor, institutional principal and GHOS controller remain
  non-collapsible;
- authorization scope uses `scope_ref` plus SHA-256 of the exact referenced
  bytes;
- a typed reference does not itself prove upstream authority.

## Fixture map

Valid candidate fixtures:

- `examples/valid/telemetry_queue_depth.json` — integer telemetry;
- `examples/valid/telemetry_latency.json` — floating telemetry;
- `examples/valid/semantic_started_mechanical.json` — semantic start bound to
  exact envelope and mechanical attempt;
- `examples/valid/envelope_revoked.json` — structurally valid upstream
  revocation record.

Invalid candidate fixture:

- `examples/invalid/semantic_started_half_bound.json` — carries envelope binding
  without mechanical-attempt binding and must fail the semantic-start schema.

The valid/invalid directory names are declarative until E2 adds an executable
schema/conformance harness.

## Non-authority

A structurally valid instance does **not** mean:

- the upstream authorization is genuine;
- the issuer possesses the referenced authority;
- an endpoint is an authorized semantic actor;
- a controller is GHOS-admitted;
- a delivered payload is semantically admitted;
- a replica is authoritative;
- a revocation is trustworthy without the selected integrity/authority profile;
- Article IX has changed;
- FABRIC exists as an activated runtime.

A later integrity/security profile must bind the exact record and upstream
authorization. E1 does not select the final signature, credential, key-
management or attestation mechanism.

## Version behavior

All schemas require `aether-fabric/1.0`. Unknown major, required record type or
unsupported required capability is a pre-start failure. No implicit downgrade
is permitted.

## Custody

These schemas are currently AETHER custody artifacts because E1 defines the
boundary from existing AETHER semantic authority outward. Their repository
location does not decide permanent protocol custody or authorize an independent
FABRIC repository/runtime.

## Normative prose

- `docs/ARCHITECTURE/AETHER_FABRIC_E1_INTERFACE_LAW.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_EVENT_ALGEBRA.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_IDENTITY_AND_AUTHORITY.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_CONFORMANCE_VECTORS.md`
- `docs/ADR/0021-aether-fabric-e1-interface-law.md`

If schema and normative law disagree, E1 is not complete. The mismatch must be
repaired rather than choosing whichever artifact is easier to implement.
