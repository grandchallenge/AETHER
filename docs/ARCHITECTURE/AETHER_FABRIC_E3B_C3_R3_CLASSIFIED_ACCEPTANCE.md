# E3B-C3-R3: Classified 22-positive + one negative-control HTTP evidence

**State:** PROTECTED bounded C3 classification, not a live-routing
certificate. Implementation PR #165 merged and read back at
`edd2acfd8533643afe5fe17d2392a72e5598cb50`; exact-head evidence is
protected in [C3-R3 receipt](../../handoffs/AETHER-FABRIC/E3B_C3_R3_CLASSIFICATION_130_RECEIPT.md).
Parent #110; classification #130 closed; #127 residual evidence remains open.
Does not activate C4 or E3B-A.

## Source discrepancy and repair

Protected R2 seeded 23 authenticated HTTP requests against independently
initialized reference and shadow-enabled AETHER Axum routers. Twenty-two
succeeded (200), and `ExplainTuple` returned an intentionally identical
409/`ambiguous_tuple_reference` on both. The original strict 23-positive
`adjudicate_c3_replay_bundle` correctly fails; its semantics remain intact.

This change:

1. Narrows `aether_control_bridge::OperationClass::mechanically_eligible` to
   exactly 22 classes; the remaining `ExplainTuple` is recognized by its
   unchanged method/path/profile but `AetherMechanicalAuthorityIssuer` rejects
   it before admitting or minting a bundle even for source-admitted=true.
2. Independently checks every positive source pairing, profile, subject,
   provenance, build, exact status+digest, permitted resource set, unique
   request and no-widening, via the existing C3 verdict recomputation.
3. Introduces `adjudicate_c3_classified_replay_bundle`, a separate typed
   result requiring the single negative control to have real paired 409 HTTP
   errors (same code and canonical error-projection digest), a matching C2
   request identity, C1 `ShadowRejected` without envelope/placement/attempt,
   zero evidence loss and no authorization effect.
4. Preserves `ReferenceOnly` and the original legacy 23-positive C3
   verifier; it still cannot certify 23 positive operations.

## Scope and limits

The classified HTTP pair is collected by the authenticated real-handler
fixture, not inferred from the C2 shadow alone. Error-body hashes are evidence
of the observed pair, not signatures or fresh operative permits. Neither the
producer nor the classified checker is attached to production dispatch.
A signed, continuous production evidence sink and independent live-source
current-state witness are **not** proved. #148 owns future atomic-start work.

## Required replay

Run `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and the dedicated real seeded
`c3_positive_case_authenticated_http_matrix_evidence` fixture.
Adversarial checks must reject wrong control codes/digests, changed
identity, duplicate/missing rows, missing source records, evidence loss and
synthesized successful `ExplainTuple`. Bind receipts to the actual candidate
source head; do not transfer historical CI qualifications.
