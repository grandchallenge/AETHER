# E3B-C0-D — Control Integrity, Revocation and Replay Contract

Status: C0 synthesis input
Issue: #113
Protected source basis: `0a1552adf719dbe57667a9b8705b986f0f381483`

## 1. Purpose

Define how C1/C2 can trust AETHER-issued control records without converting
structural schema validity, scheduler state, or serialized bytes into authority.

## 2. Integrity profiles

### 2.1 `aether-control-bridge-inproc/1`

This is the only profile authorized for C1 and C2.

Trust boundary:
- same AETHER process;
- protected Rust build;
- AETHER-owned issuer/control-registry modules;
- no network or untrusted deserialization boundary.

Required evidence:

1. record constructed through an AETHER issuer constructor that is not exposed
   as a FABRIC authority-minting API;
2. canonical record bytes under the protected canonicalization profile;
3. SHA-256 digest of those exact bytes;
4. exact issuer identity;
5. exact source commit/build identity;
6. exact issuer configuration/profile digest;
7. registry revision for lifecycle-bearing records;
8. process-local authority proof/capability held by the AETHER issuer/control
   registry.

A serialized JSON record by itself does not carry the process-local authority
proof. Re-deserializing a record cannot recreate issuer authority.

FABRIC receives validated record data and digests, not the ability to mint the
proof/capability.

### 2.2 Cross-process/network profile

Status: **NOT DEFINED BY C0 / NOT AUTHORIZED**.

If later deployment crosses a process/trust boundary, a separately protected
profile must add cryptographic issuer authentication, key identity/lifecycle,
rotation/revocation and replay protection.

C0 does not choose production signing keys or credentials.

Use of `aether-control-bridge-inproc/1` across a process or network boundary is
invalid.

## 3. Exact digest material

### Operation admission digest

Bind:
- operation class;
- request/correlation identity;
- namespace;
- principal reference;
- token reference when present;
- required scope;
- effective policy digest;
- operation-manifest ref/digest;
- operation-profile ref/digest;
- decision revision/time;
- AETHER source authority.

### Mechanical authorization digest

Bind:
- exact admitted operation record ID/digest;
- scope and payload manifest refs/digests;
- exact mechanical profile ref/digest;
- expiry;
- issuer identity;
- issuer revision/time.

### Envelope digest

SHA-256 of the exact E1 envelope bytes.

### Constraint digest

SHA-256 of canonical `PlacementConstraintSet` bytes.

### Witness digest

SHA-256 of canonical `ControlStateWitness` bytes.

### Placement decision input digest

Retain protected F1A/F1B framing:
- E1 exact bytes;
- constraint;
- resource snapshot;
- scheduler-policy artifact;
- selector implementation identity;
- control witness.

## 4. Envelope-control registry

C1 shall introduce an AETHER-owned in-process registry with one entry per
envelope:

```text
EnvelopeControlEntry {
  envelope_id
  envelope_digest
  authorization_ref
  issuer_ref
  state
  revision
  expires_at
  superseded_by?
  revoked_at?
  revocation_reason?
}
```

States:
`active | revoked | superseded`.

`expired` is a current-state projection when observation time is at or after
`expires_at`; it need not require a registry mutation.

## 5. Linearization

All lifecycle writes and witness observations for one envelope are linearized
by the AETHER registry.

The C1 implementation may use an in-memory mutex/RwLock or equivalent bounded
single-process synchronization. C0 does not authorize a distributed registry.

### Authorize

Atomic effect:
- verify authorization decision/profile;
- insert new envelope as `active`, revision 1.

Duplicate exact authorization:
- same envelope ID + same digest => idempotent readback;
- same envelope ID + different digest => integrity conflict, fail closed.

### Revoke

Atomic effect:
- transition `active -> revoked`;
- increment revision;
- record revocation time/reason.

Only AETHER issuer/control authority may invoke revoke.

Duplicate identical revocation is idempotent.

### Supersede

Not used by the first lane, but law is fixed:

- insert the replacement envelope only after validating its independent
  authorization;
- atomically transition the named prior active envelope to `superseded`;
- increment prior revision;
- bind replacement identity.

A child/derived envelope is not supersession.

### Witness

Atomic read:
- exact registry entry;
- exact envelope digest;
- current revision;
- observation time.

Projection:
- `revoked` if registry state revoked;
- `superseded` if registry state superseded;
- `expired` if observation time >= E1 expiry;
- otherwise `active`.

A witness for unknown/mismatched envelope fails closed.

## 6. Freshness

### Placement-time witness

Required immediately before FABRIC selection.

### Realization-time witness

A distinct fresh observation is required after `PlacementSelected` and before
`RouteRealized`.

Historical placement evidence remains historical after revocation; it cannot
authorize realization.

### Resource snapshot

First-lane C1/C2 default:
`max_snapshot_age_ms = 1000`.

The bound is carried in `PlacementConstraintSet` and may be narrowed by a
later protected operation profile.

A future activation packet may choose a tighter value. It may not silently
widen the protected bound without review.

## 7. Expiry

First-lane envelope expiry is bounded by the exact operation profile and HTTP
operation timeout.

C1 default rule:

```text
expires_at <= issued_at + operation_timeout_ms
```

No retry, replay, placement or witness can extend expiry.

A replacement envelope requires a new upstream authorization decision.

## 8. Replay and idempotency

### Request/admission

The exact request identity is part of operation-admission identity.

Equal payloads in distinct HTTP requests do not collapse into one authorization
unless the endpoint's existing semantic idempotency contract explicitly
provides such identity.

### Authorization/envelope

Re-observation of exactly identical canonical input yields the same logical
record identity.

Identity collision with different bytes is a hard integrity failure.

### Placement

Protected F1 idempotency remains controlling.

Canonical-equivalent resource snapshots and identical exact decision inputs
produce identical logical placement evidence.

### Realization

Protected E3B-R behavior remains controlling:

- exact realization-request replay => same logical `RouteRealized` evidence;
- explicit retry request => new `mechanical_attempt_id`;
- placement replay alone => no attempt.

The first lane has no automatic retry.

## 9. TOCTOU rule

The authoritative realization predicate is not:

```text
placement was valid when selected
```

It is:

```text
placement evidence is valid
AND fresh current AETHER control witness is active
AND current resource evidence is admissible
AND exact realization request is valid
```

This predicate is evaluated immediately before `RouteRealized`.

A revocation/supersession/expiry that linearizes before the fresh witness
prevents realization.

If it linearizes after `RouteRealized` but before semantic start, the queued
attempt may be pre-start rejected according to E1 law.

Once AETHER records semantic start, later mechanical revocation does not erase
the semantic attempt.

## 10. Authority-laundering prohibitions

The following can never replace an issuer/control proof:

- valid JSON schema;
- correct digest without issuer proof;
- bearer-token success;
- namespace permit;
- rate-limit permit;
- available worker slot;
- healthy resource;
- placement selection;
- local process co-location;
- successful prior request.

FABRIC cannot reconstruct a missing AETHER proof from these facts.

## 11. Candidate/shadow failure behavior

During C1/C2:

- reference execution remains authoritative;
- candidate control-path integrity failure is recorded as candidate evidence
  failure;
- the failure must be visible in comparison/adjudication evidence;
- the reference execution is not described as a successful FABRIC fallback.

## 12. Future live failure behavior

A future `live_fabric` mode, if separately authorized, must fail closed on
control-record integrity/authentication failure.

Per-request silent fallback after an invalid FABRIC authorization attempt is
forbidden because it would hide the failure.

Operational rollback is a separate configuration transition back to
`ReferenceOnly`, bound to the exact protected rollback delta.

## 13. Migration and rollback

C0/C1/C2 own no AETHER semantic state.

Removing the in-process issuer/control registry or disabling shadow issuance:
- requires no journal rewrite;
- requires no semantic-state migration;
- leaves the reference execution path intact.

Control records generated in shadow mode are evidence only and are not
grandfathered into a later live activation.

## 14. C0-D disposition

The first-lane integrity, lifecycle, revocation, replay, freshness and TOCTOU
rules are sufficiently explicit for an off-path C1 implementation.

Cross-process authority transport remains deliberately out of scope and fails
closed.
