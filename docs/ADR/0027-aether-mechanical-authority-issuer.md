# ADR 0027 — AETHER mechanical-authority issuer boundary

Status: Accepted — protected C0 merge `4eb776748b3c70829bd8dca0c1bc48cb6210776f`
Issue: #113
Protected predecessor: `0a1552adf719dbe57667a9b8705b986f0f381483`

## Context

Protected E3B-R can consume exact upstream AETHER/FABRIC control records and
safely realize `PlacementSelected -> RouteRealized`, but the ordinary HTTP path
does not currently produce those records.

Authentication, namespace resolution, policy binding, rate limiting, namespace
semaphore admission, worker capacity and liveness are all existing facts. None
is equivalent to upstream mechanical authorization.

## Decision

Introduce an AETHER-owned control-bridge issuer boundary.

The first implementation shall live in an AETHER-owned Rust crate,
`aether_control_bridge`, and shall remain off the ordinary HTTP path in C1.

The issuer shall create:

1. `AetherOperationAdmissionDecision` — permission to attempt one exact AETHER
   operation class;
2. `MechanicalAuthorizationDecision` — bounded mechanical projection of that
   admitted operation;
3. protected E1 `MechanicalEnvelopeAuthorized`;
4. protected E3 `PlacementConstraintSet`;
5. current AETHER-owned `ControlStateWitness`.

FABRIC remains the producer only of placement/mechanical evidence such as
`PlacementSelected` and `RouteRealized`.

## First lane

The first lane contains the 23 read/evaluate/explain operation classes frozen in
`AETHER_FABRIC_E3B_C0_AUTHORITY_ISSUANCE_CONTRACT.md`.

Seven authoritative mutations remain excluded:

- append;
- schema registration;
- schema activation;
- replica promotion;
- partition append;
- artifact-reference registration;
- vector-record registration.

## Integrity

C1/C2 use `aether-control-bridge-inproc/1` only.

Authority requires the protected AETHER issuer path plus a process-local,
non-serializable authority capability. JSON shape or digest alone cannot mint
authority.

Cross-process authority transport remains undefined and unauthorized.

## Lifecycle

AETHER owns an in-process envelope-control registry.

Only AETHER may authorize, revoke or supersede envelopes.

FABRIC placement is historical non-authoritative evidence. A fresh AETHER
control-state witness is required immediately before route realization.

`mechanical_attempt_id` is created only by the E3B realization bridge at
`RouteRealized`.

## Constitutional disposition

For this exact design:

`NO_ARTICLE_IX_AUTHORITY_CHANGE`.

The issuer remains inside AETHER and does not transfer append order, semantic
cuts, replay, policy visibility, provenance, derivation, proof traces or
semantic lifecycle authority to FABRIC.

This does not authorize E3B-A production activation.

## Consequences

Positive:
- removes the current activation ambiguity without authority laundering;
- provides explicit provenance for every authority-bearing field;
- preserves a local/no-FABRIC path;
- supports off-path C1 and real-request shadow C2 before any live cutover.

Costs:
- adds an explicit AETHER control-bridge type/registry;
- requires operation profiles and canonical manifests;
- leaves mutation endpoints for later semantic-boundary work;
- requires a different integrity profile if the control bridge ever crosses a
  process/trust boundary.

## Rollback

Before live activation, disable/delete the off-path issuer and shadow registry.
No AETHER semantic-state migration is required.


## Protected decision

ADR 0027 was accepted through the C0 exact-head transaction on source
`3a7558311e92616c1200fbcc501e188ed495d192` and protected merge
`4eb776748b3c70829bd8dca0c1bc48cb6210776f`.

This acceptance authorizes C1 off-path implementation of the AETHER-owned
control bridge only. C2 ordinary-request shadow integration, C3/C4 readiness,
and E3B-A live activation remain later gates.
