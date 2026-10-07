# E3B-C0-E2 — Constitutional Disposition

Status: C0 synthesis input
Issue: #113

## Controlling law

INTELLECT exact source:
`grandchallenge/INTELLECT@f36495d75244cc9004739995d7ac48a19d42c21b`.

Effective authority schedule:
- schema version: `1.6.0`;
- Constitution effective version: `1.2.0`;
- Constitution content SHA-256:
  `85ea47cb67ebe15210b6743e186d36db2336bd0a610929392806406be5a49c1a`.

Article IX currently states that AETHER is the authoritative coordination
substrate for production deployments and owns append order, semantic cuts,
replay, policy visibility, provenance-bearing facts, recursive derivation and
proof traces.

Article X reserves Human Steward authorization for changes to the AETHER
authority boundary and other named reserved actions.

Article XI defines the amendment procedure when the Constitution is changed.

The separate AETHER/FABRIC Article XI packet in INTELLECT is
`PREPARATION_ONLY`; proposed version 1.3.0 is not effective and is not relied
upon by C0.

## Exact C0 question

Does the C0 issuer create/change effective AETHER authority, or merely project
existing AETHER authority into a bounded mechanical control record?

## C0 design tested

The C0 contract:

1. leaves HTTP authentication, namespace resolution and AETHER policy binding
   under AETHER;
2. introduces an AETHER-owned operation-admission record that means only
   permission to attempt an operation;
3. introduces an AETHER-owned mechanical authorization projection subordinate
   to that admitted operation;
4. keeps append order, semantic cuts, replay, policy visibility,
   provenance-bearing facts, recursive derivation, proof traces and semantic
   lifecycle under AETHER;
5. gives FABRIC only non-authoritative placement/realization mechanics inside a
   closed envelope;
6. preserves a legal local/no-FABRIC AETHER path;
7. excludes authoritative mutation endpoints from the first lane;
8. grants FABRIC no ability to authorize/revoke envelopes, interpret policy,
   admit results, change leader epochs, admit controllers, or make
   institutional decisions.

## Disposition

```text
NO_ARTICLE_IX_AUTHORITY_CHANGE
```

Reason:

The proposed issuer is itself AETHER-owned and does not delegate an existing
Article IX power to FABRIC. It projects an already-governed AETHER request into
a typed mechanical authorization whose effects are expressly non-semantic.

C0 changes representation and evidence structure, not the holder of production
semantic authority.

Therefore an Article XI constitutional amendment is **not required for C1
off-path implementation or C2 shadow issuance** under this exact contract.

## Conditions on this disposition

The disposition is invalidated and must be reopened if a later candidate:

- lets FABRIC decide whether an operation is semantically permitted;
- lets FABRIC mint/revoke upstream envelopes from scheduler/liveness facts;
- moves append order, semantic cuts, replay, policy visibility, provenance,
  derivation or proof-trace authority out of AETHER;
- makes resource availability or delivery sufficient for semantic admission;
- gives FABRIC leader/promotion authority;
- removes the independent local/no-FABRIC AETHER semantic path as an
  architectural invariant;
- changes effective Article IX wording or the authority schedule;
- otherwise changes the AETHER authority boundary.

If any such change occurs, Article X reserved Human Steward authority applies
and an effective Article XI amendment is required before activation.

## Production activation remains separate

This constitutional disposition does not approve E3B-A.

The protected A0 packet still requires:

- exact issuer implementation;
- exact live-routing delta;
- exact rollback delta;
- exact canary/deployment target;
- exact integrity profile;
- fresh reviews/checks;
- authentic reserved production/control-plane disposition.

```text
C0 constitutional status: NO_ARTICLE_IX_AUTHORITY_CHANGE
C1/C2 constitutional prerequisite: satisfied by current law
E3B-A activation authority: NOT GRANTED
reserved_activation_disposition: UNSET
```

## Migration/rollback

C0 is specification-only.

C1/C2 are constrained to off-path/in-process shadow state and own no AETHER
semantic journal state.

Rollback before live activation is deletion/disablement of the shadow issuer
and control registry; no semantic-state migration or constitutional rollback is
required.

## C0-E2 disposition

The C0 contract may proceed to implementation handoff without invoking Article
XI, provided the exact protected C0 synthesis preserves the boundaries above.
