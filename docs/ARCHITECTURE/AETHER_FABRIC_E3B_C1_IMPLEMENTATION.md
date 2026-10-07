# AETHER/FABRIC E3B-C1 â€” Off-Path Control-Bridge Implementation

Status: protected complete — C2 shadow integration eligible; E3B-A not authorized
Issue: #117
Parent: #110
Protected C0 basis: `5493da50aaeff9c944cee8c01891bad498e631ac`
Exact implementation code commit: `8df42c6cd3afa09df8f99c2b0a55a97d11aa5423`

## 1. C1 scope

C1 implements the AETHER-owned control bridge authorized by the protected C0
contract. It does not connect the issuer to ordinary HTTP execution.

Implemented crate:

`crates/aether_control_bridge`

The implementation is deliberately off-path. There is no public production
constructor for `AetherMechanicalAuthorityIssuer` in C1, no HTTP dependency,
no `FabricRoutingMode` mutation, and no live FABRIC routing switch.

A later C2 tranche must add any AETHER-owned integration factory and ordinary
request shadow hookup under fresh review.

## 2. Exact implementation identity

Implementation code commit:

`8df42c6cd3afa09df8f99c2b0a55a97d11aa5423`

Implementation tree:

`acb289de8947a87ebf09abd2ed4abcd101814665`

`crates/aether_control_bridge/src/lib.rs`:
- Git blob: `53ed752e3f2bd44e70d43abcddebff01683f2da5`
- SHA-256: `cd9c3555947455edbd6d8f310f81d9467b5b2ec0f40e69b9f026b6b651e06764`

`crates/aether_control_bridge/Cargo.toml`:
- Git blob: `ac814a047b7a09f47ad4b8d9048a597c4dca5fca`
- SHA-256: `4073b8232f0caa05c9f32d142fd01aa0fb509a0124a68eb507ab2d3e5db85c0b`

Implementation parent:

`5493da50aaeff9c944cee8c01891bad498e631ac`

## 3. Build and validation identity

Local validation environment:

- rustc `1.94.0 (4a4ef493e 2026-03-02)`
- rustc commit `4a4ef493e3a1488c6e321570238084b38948f6db`
- host `x86_64-pc-windows-msvc`
- LLVM `21.1.8`
- cargo `1.94.0 (85eff7c80 2026-01-15)`

Exact local C1 test executable after the full replay:

- artifact: `aether_control_bridge-11338835a4d070ea.exe`
- SHA-256: `43a81cd7cfdcf9a7329fb25a56f087297de2a551c829361e4557e83f36a22ec8`
- bytes: `2137088`

This local executable identity is diagnostic evidence, not a production issuer
artifact and not deployment authority. Protected CI on the final candidate head
must independently rebuild and validate C1.

The public `IssuerBuildIdentity` contract is implemented and binds
`source_commit`, `source_tree`, and `artifact_sha256`. C1 does not create
a production issuer instance, so no production build artifact is asserted here.
C2 must bind its actual protected integration build.

## 4. Public contract surface

C1 implements the C0-required public types:

- `OperationClass`
- `OperationProfile`
- `MechanicalProfile`
- `OperationAdmissionInput`
- `AetherOperationAdmissionDecision`
- `MechanicalAuthorizationDecision`
- `IssuedMechanicalControlBundle`
- `EnvelopeControlState`
- `EnvelopeControlRegistry`
- `AuthorityTransport`
- `AuthorityIssuanceError`
- `IssuerBuildIdentity`
- `AetherMechanicalAuthorityIssuer`

The issuer exposes observation/decision/issuance/lifecycle methods but has no
public constructor in C1. Its authority capability is private, non-serializable,
and cannot be recreated by deserializing records.

## 5. Closed first-lane operation registry

Exactly 23 operation classes are accepted:

| Operation | Profile | SHA-256 |
| --- | --- | --- |
| history | `aether-http-op/history/1` | `85e4525ee426025ff37148d9bd048f99be449f39b6cff6019f2bc5cd532ed34c` |
| history_page | `aether-http-op/history_page/1` | `6e2d76d8a742ab68e1f3387f10c2170f781039d91fdf4e85a3009a7f6fcec0e7` |
| append_dry_run | `aether-http-op/append_dry_run/1` | `379f3d0fce710c90c1ed95583f3d0e2fc009385586d4c9f6415d8dc211b1e58e` |
| append_receipts | `aether-http-op/append_receipts/1` | `8085c6a2fc66bf66729498ea480c3b8f21f5553aca3218359bcd944ab2c18b59` |
| schema_catalog | `aether-http-op/schema_catalog/1` | `0f37e3f4ea9a0fb561b375cddcc90dfa7827cff1139a60858e9f335095b5d988` |
| current_state | `aether-http-op/current_state/1` | `abb9dc378a058687eec9a40f4ba97d01c7610e08c492bb5eb722788a1cb6795d` |
| as_of | `aether-http-op/as_of/1` | `66eae0d80b2f37c5035275f29a43698fd4fa024b34a21e0c49733691dd6b6c47` |
| parse_document | `aether-http-op/parse_document/1` | `71625f71889debe2a41f89e86a11a71468963881dcceba55416f06762289b186` |
| run_document | `aether-http-op/run_document/1` | `a1eabd3df789c89fa1d0fcbcc492f204a8db192cffc83fa3e3f2c50766c04676` |
| run_document_page | `aether-http-op/run_document_page/1` | `a47005f10718064b0c59336dc727782a9d0646fd0e7230d430bcdd744ccfa4ef` |
| coordination_pilot_report | `aether-http-op/coordination_pilot_report/1` | `db054c227bbf2d73ace9ca04c4de0696180111a2ced84097c43390164c4af15b` |
| coordination_delta_report | `aether-http-op/coordination_delta_report/1` | `6f2b9eb420e3cf22b6cba4115a2ee8cfbd5f163d6b3ea02413a94c961d63f783` |
| partition_status | `aether-http-op/partition_status/1` | `d55a76e729bf48953b70bb9fd0a786ece86daeaf6679728b012eabd5bb6c9cbb` |
| partition_history | `aether-http-op/partition_history/1` | `10e099eb8f0b02614b0390397ec1c894072c1564b41f84cd5eafae4746fad022` |
| partition_state | `aether-http-op/partition_state/1` | `774dc105e3dc04d81f9f72b1806e3e2d7dda4f58963979c2d175b51808e7b612` |
| federated_history | `aether-http-op/federated_history/1` | `f4e916a6f29449929d2ce856892cbbc5da976489c44e393520799f422614f50e` |
| federated_run_document | `aether-http-op/federated_run_document/1` | `4d77e0663ee900fcce8b04c5e40d902ad9f0fc46f956b2ec54d0255e5735cf37` |
| federated_report | `aether-http-op/federated_report/1` | `59038f8fbdbb1e41cc5a576e277ec69a56ad6997fd94c786fe387bda6f571efc` |
| explain_tuple | `aether-http-op/explain_tuple/1` | `f1e96ab398e42a8de0c6d88e709ab45fa3497382b54bdee816cdab6d9436a793` |
| resolve_trace_handle | `aether-http-op/resolve_trace_handle/1` | `d3c8b4aa80aafbd36a96c90262ad50f921a9f17b27c93bef9e563feba7f0bbf4` |
| resolve_trace_handle_page | `aether-http-op/resolve_trace_handle_page/1` | `73293797da44cc80412b9b7eb3635a024b47810d8601aee5b5efb94c12ce6dbe` |
| get_artifact_reference | `aether-http-op/get_artifact_reference/1` | `2d43e76ec28d48c13913ec784f5db87327cf4af2df41075eb67ece4a228803ac` |
| search_vectors | `aether-http-op/search_vectors/1` | `cb7624370097d8667dc66cd5846d334d025343532e54096e46ac842085e13c72` |

All seven C0-excluded authoritative mutation endpoints are hard rejections.

## 6. Protected local mechanical profile

Logical profile:

`aether-http-local-blocking/1`

Canonical SHA-256:

`de380e9da47ac1ce7fbe9c585d0b4f6e21e30810ad94b2d8fb0b3a21eec48fd2`

Canonical content:

```json
{"backoff_class":"none","capacity_unit":"blocking_admission_slot","eligible_resource_classes":["local-blocking-pool"],"fairness_policy_ref":"aether-bounded-admission/1","integrity_profile_ref":"aether-control-bridge-inproc/1","locality_constraints":["local-process"],"max_attempts":1,"max_parallel_copies":1,"max_snapshot_age_ms":1000,"minimum_capacity_units":1,"permitted_actions":["queue_pre_start"],"priority_class":"normal","profile_ref":"aether-http-local-blocking/1","required_capabilities":["blocking_execution"],"trust_zones":["aether-process"]}
```

The mechanical profile is a distinct artifact from each per-operation profile.
`MechanicalAuthorizationDecision.mechanical_profile_digest` binds this exact
artifact; `AetherOperationAdmissionDecision.operation_profile_digest` binds the
operation-specific profile.

## 7. Operation admission and mechanical authorization

The implementation separates:

1. `AetherOperationAdmissionDecision`: AETHER permission to attempt one exact
   operation under an exact namespace/principal/policy/request/profile binding.
2. `MechanicalAuthorizationDecision`: AETHER mechanical-only projection of
   an admitted operation.
3. E1 `MechanicalEnvelopeAuthorized`: exact closed mechanical envelope.
4. E3 `PlacementConstraintSet` and AETHER current `ControlStateWitness`.

Bearer-token validity, namespace permits, capacity, liveness, resource health,
or serialized record shape cannot substitute for the operation-admission
decision.

## 8. In-memory control registry

`EnvelopeControlRegistry` is bounded and process-local.

It implements:

- exact active-envelope registration;
- exact duplicate registration as idempotent readback;
- same envelope ID with different bytes/authority as hard integrity conflict;
- current control-state observation;
- `active -> revoked`;
- `active -> superseded`;
- expiry as a point-in-time projection;
- monotonic revision changes;
- authorization-time and expiry-time transition bounds.

A witness cannot predate envelope authorization. Revocation or supersession at
or after expiry is rejected because the envelope is no longer active.

## 9. Integrity boundary

C1 supports only:

`aether-control-bridge-inproc/1`

The issuer owns a private non-serializable `AuthorityCapability`.
Serialization/deserialization of an admission, authorization, envelope,
constraint, witness, or other record does not recreate that capability.

`AuthorityTransport::CrossProcess` fails closed.

C1 selects no production credential, signing key, network trust root, or
cross-process authority protocol.

## 10. Idempotency and replay

The implementation binds identity to exact canonical material.

Covered behavior:

- equal exact operation inputs replay to equal logical admission/authorization;
- changed HTTP request identity changes admission identity;
- changed namespace changes admission identity;
- changed operation-manifest bytes fail verification;
- changed authorization reference fails verification;
- duplicate exact envelope issuance is idempotent;
- envelope ID collision with different bytes is a hard conflict;
- retry does not extend envelope expiry;
- selector implementation identity remains placement-decision input material;
- stale/revoked/superseded/expired control state blocks selection/realization.

## 11. Test and hostile-evidence closure

Focused C1 validation on exact implementation commit:

```text
cargo clippy -p aether_control_bridge --all-targets -- -D warnings
PASS

cargo test -p aether_control_bridge
29 unit tests PASS
1 compile-fail doctest PASS
0 failures
```

The 29 tests cover all protected C0 minimum obligations plus:

- exact method/path/scope profile mismatch;
- bounded-registry capacity;
- revocation idempotency;
- exact RFC3339 timeout/expiry binding;
- pre-authorization witness rejection;
- expired-envelope revoke/supersede rejection;
- exact local-envelope-profile assertions.

Full regression replay on the exact implementation commit:

```text
cargo test --workspace
PASS

cargo clippy --workspace --all-targets -- -D warnings
PASS

python -m pytest python/tests/test_aether_fabric_e2.py python/tests/test_aether_fabric_f1a.py
16/16 PASS
```

Existing E2/F1/E3B-R regression suites remain green, including:

- E2/F1A Python replay: 16/16;
- `aether_fabric`: 38/38;
- `aether_http`: 39/39;
- C1: 29/29 + compile-fail doctest.

Existing explicitly ignored stress/soak benchmarks remain ignored under their
pre-existing test annotations; C1 did not change those policies.

## 12. Off-path proof

C1 changes:

- root workspace membership/lockfile;
- new `aether_control_bridge` crate.

C1 does not change:

- `crates/aether_http/src/http.rs`;
- FABRIC cutover/routing configuration;
- production service configuration;
- semantic journal/storage;
- protocol schemas;
- credential/key material.

There is no public production issuer constructor in C1 and no ordinary HTTP
caller of the crate.

## 13. C1 acceptance state

Substantive implementation status:

```text
C0 contract conformance: PASS
closed-world first lane: PASS
seven mutation exclusions: PASS
operation/mechanical identity separation: PASS
mechanical profile exact digest: PASS
in-memory lifecycle registry: PASS
revocation/supersession/expiry timing: PASS
same-process authority capability: PASS
cross-process rejection: PASS
golden/hostile/replay suite: PASS
full Rust workspace tests: PASS
full Rust workspace clippy: PASS
HTTP integration: ABSENT BY DESIGN
live FABRIC routing: ABSENT
```

Protected governance completion:

- exact reviewed head: `97c0878d067fcd7cdde978c2d3967c08ead5d006`;
- protected merge: `2e959418f8a27dfb72beb8502e2b3ecdf82c6f65`;
- Formalist PASS: review `5449558320`;
- Adversary PASS: review `5449559745`;
- Referee COMPLETE: review `5449561091`;
- Required CI: `113065721575` success;
- Required Supply Chain: `113065207060` success;
- Rust PR fast: `113065172719` success;
- policy: `113065135368` success;
- security: `113065135595` success;
- routing-enforcement: `113065133898` success;
- GCL conformance run `37701309738`: success;
- unresolved review threads: none;
- protected-main readback: `2e959418f8a27dfb72beb8502e2b3ecdf82c6f65`.

Issue #117 completion receipt is emitted after the final documentary readback.

## 14. Successor boundary

With C1 protected, the only newly eligible successor is C2:
real-HTTP **shadow** integration.

C2 may add an AETHER-owned issuer factory and run the C1 issuance path beside
the reference path for admitted real requests, but it must remain non-operative
with respect to production routing.

C1 does not authorize:

- C2 effects beyond shadow-only integration;
- `live_fabric`;
- production FABRIC dispatch;
- E3B-A;
- cross-process authority transport;
- the seven excluded mutation endpoints.

Current boundary:

```text
E3B-R: PROTECTED COMPLETE
C0-A-F: PROTECTED COMPLETE
C1: PROTECTED COMPLETE
C2: AUTHORIZED SHADOW-ONLY / NOT STARTED
E3B-A: BLOCKED
reserved_activation_disposition: UNSET
live_fabric_routing: NOT ACTIVE
```
