# AETHER/FABRIC E1 Candidate Schemas

Protocol candidate: `aether-fabric/1.0`
Issue: #85
Status: specification/conformance only; not a production runtime protocol

## Purpose

These schemas make the E1 interface law mechanically inspectable before any
FABRIC runtime or code extraction exists.

They are deliberately split by authority/producer domain:

- `mechanical_envelope.schema.json`
  - closed mechanical realization envelope;
  - may be narrowed downstream, never widened;
  - carries opaque upstream authorization references;
  - `authority_effect` is structurally fixed to `none`.
- `mechanical_event.schema.json`
  - FABRIC-domain route, queue, transport, replica-movement and object-location
    observations;
  - no record type in this schema represents semantic admission or authority.
- `semantic_lifecycle_event.schema.json`
  - AETHER-owned semantic start/completion/submission/admission events;
  - intentionally separate from the mechanical event schema;
  - `authority_owner` is structurally fixed to `AETHER`.
- `telemetry_evidence.schema.json`
  - operational observations only;
  - requires ordinary provenance-bearing AETHER submission/admission before
    governed reasoning may consume the observation as semantic evidence.
- `identity_binding.schema.json`
  - explicit cross-stratum observation/reference records;
  - endpoint, semantic actor, institutional principal and GHOS controller
    identities remain non-collapsible.

## Non-authority

Schema validation means only that bytes conform to this candidate contract.
It does **not** mean:

- the envelope was institutionally authorized;
- the issuer possesses the referenced authority;
- an endpoint is an authorized semantic actor;
- a controller is GHOS-admitted;
- a delivered payload is semantically admitted;
- a replica is authoritative;
- Article IX has changed;
- FABRIC exists as an activated runtime.

A later integrity/security profile must bind the exact envelope and its upstream
authorization reference. E1 intentionally does not choose the final signature,
credential, key-management, or attestation mechanism.

## Version behavior

All schemas in this directory require:

```text
protocol_family = aether-fabric
protocol_major  = 1
protocol_minor  = 0
```

Unknown protocol major or unsupported required capability is a pre-start
failure. No implicit downgrade is permitted.

## Custody

These schemas are currently custody artifacts of AETHER because E1 defines the
boundary from the existing AETHER semantic authority outward.

Their presence under `grandchallenge/AETHER` is not a decision that a future
independently encapsulated FABRIC repository should permanently own the
protocol. Protocol custody can be reconsidered only after E2/E3 demonstrate a
stable independent boundary and the Council conditions for E4 are met.

## Verification

The normative prose is:

- `docs/ARCHITECTURE/AETHER_FABRIC_E1_INTERFACE_LAW.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_EVENT_ALGEBRA.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_IDENTITY_AND_AUTHORITY.md`
- `docs/ARCHITECTURE/AETHER_FABRIC_E1_CONFORMANCE_VECTORS.md`
- `docs/ADR/0021-aether-fabric-e1-interface-law.md`

If a schema and the approved normative law disagree, E1 is not complete; the
mismatch must be repaired rather than silently choosing whichever artifact is
more convenient to implement.
