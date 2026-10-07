# E3B-C0-E1 — Adversarial Threat Matrix

Status: C0 synthesis input
Issue: #113
AETHER protected basis: `0a1552adf719dbe57667a9b8705b986f0f381483`

## Threat matrix

| Threat | Attack | Required defense | Disposition |
| --- | --- | --- | --- |
| Authentication laundering | Treat valid bearer token as E1 envelope authority | Separate Q1 auth from Q4/Q5 issuer decision; envelope `authorization_ref` points to AETHER decision, never token | CLOSED BY CONTRACT |
| Namespace laundering | Treat valid namespace header/permit as mechanical authority | Namespace is input identity/sequencing only; explicit issuer decision required | CLOSED |
| Capacity laundering | Free worker/queue slot creates authorization | Resource snapshots are mechanical observation only | CLOSED |
| Liveness laundering | Healthy endpoint/resource treated as permission | Liveness may filter eligible resources but cannot create envelope | CLOSED |
| FABRIC self-authorization | Selector constructs E1 or current witness | Issuer/registry AETHER-owned; FABRIC lacks process-local issuer capability | CLOSED |
| Schema-valid forgery | Structurally valid JSON accepted as authority | Schema validation separate from issuer/integrity proof | CLOSED |
| Digest-only forgery | Correct SHA-256 on attacker bytes treated as issuer proof | In-process profile requires issuer capability + exact source/build/config identity | CLOSED |
| Stale witness | Placement/realization uses old active witness after revocation | Fresh witness at placement and distinct fresh witness at realization | CLOSED |
| Placement replay after revoke | Replay old `PlacementSelected` to realize work | Placement has no authority; realization rechecks current AETHER state | CLOSED |
| Cross-request replay | Reuse authorization from another HTTP request | Request identity + operation manifest/profile are bound into admission/authorization identity | CLOSED |
| Cross-namespace reuse | Reuse envelope in another namespace | Namespace is inside immutable operation manifest/admission digest | CLOSED |
| Principal collapse | Token/principal/request IDs collapse into same authority type | Typed identity ledger; equality carries no implication | CLOSED |
| Resource/principal collapse | Resource ID equal to actor string gains semantic authority | Endpoint resource is a separate identity stratum | CLOSED |
| Controller collapse | Reachable host/resource treated as admitted GHOS controller | Controller identity/authority not issued by C0/FABRIC | CLOSED |
| Implicit protocol downgrade | Unsupported record/profile silently treated as old version | Unknown/unsupported protocol/profile fails closed | CLOSED |
| Constraint weakening | E3 removes E1 restriction | Protected subset/narrowing validator; widening fails closed | CLOSED |
| Forged authorization ref | Envelope points to token/request instead of mechanical decision | Field mapping requires exact mechanical authorization ID/digest | CLOSED |
| Forged scope/payload | Request changes after authorization | Exact immutable manifest bytes/digests bound into admission, authorization and envelope | CLOSED |
| Expiry reset | Retry/replay creates fresh expiry without new authority | Retry cannot extend expiry; replacement requires new AETHER authorization | CLOSED |
| Duplicate issuance split-brain | Same envelope ID maps to different bytes | Registry: same ID/same digest idempotent; same ID/different digest hard conflict | CLOSED |
| Supersession race | Old and replacement both appear active | Registry atomically transitions prior state and inserts replacement under one linearization regime | CLOSED BY SPEC; NOT USED FIRST LANE |
| Placement/realization TOCTOU | Resource/control state changes after selection | Q10 fresh witness + current resource snapshot immediately before route realization | CLOSED |
| Selector substitution | Different selector binary uses same policy version | Placement binds exact selector implementation digest | CLOSED |
| Policy substitution | Different mechanical policy uses same human label | Exact policy/profile bytes and digests are bound | CLOSED |
| Shadow-to-live promotion | C1/C2 evidence reused as production authority | Shadow records not grandfathered; later activation packet must bind exact live config and issuer | CLOSED |
| Silent fallback | Integrity failure on FABRIC candidate hidden by reference execution | Candidate failure recorded distinctly; future live mode fails closed; rollback is explicit mode transition | CLOSED |
| Mutation overgeneralization | Append/schema/leader/sidecar mutation gets blanket envelope from auth success | Seven authoritative mutations excluded from first lane | CLOSED |
| Anonymous-auth overreach | Current no-token configuration yields anonymous principal and is treated as universal issuer authority | Anonymous is auth configuration fact only; protected operation profile + issuer decision still required | CLOSED |
| Evidence persistence confusion | `run_document` execution receipt treated as FABRIC or semantic admission | Execution/proof persistence remains AETHER-owned at/after semantic start | CLOSED |
| Local-process trust escape | In-process record serialized and reused elsewhere | `aether-control-bridge-inproc/1` invalid outside same protected process | CLOSED |
| Revocation from liveness | FABRIC resource health revokes envelope | Only AETHER registry writes control state | CLOSED |

## Hostile invariants for C1

C1 tests SHALL include at least:

1. bearer token cannot construct envelope without issuer decision;
2. namespace permit cannot construct envelope;
3. resource snapshot cannot construct envelope;
4. FABRIC cannot mint issuer capability;
5. serialized/reparsed in-process record lacks issuer proof;
6. forged `authorization_ref` rejected;
7. changed operation manifest after admission rejected;
8. changed profile digest rejected;
9. cross-namespace replay rejected;
10. cross-request replay rejected;
11. stale witness after revoke rejected;
12. stale witness after supersession rejected;
13. expired envelope rejected;
14. duplicate envelope ID/different digest rejected;
15. identical duplicate issuance is idempotent;
16. placement replay after revoke cannot realize;
17. selector implementation substitution changes decision input;
18. constraint widening rejected;
19. retry cannot reset expiry;
20. shadow failure is visible and not reported as FABRIC success.

## Residual risks

These are not C0 defects but remain future boundaries:

- distributed/cross-process issuer authentication is not specified;
- live routing/rollback delta is not yet implemented;
- mutation endpoints are not in the first lane;
- production key lifecycle is not selected;
- reserved E3B-A activation disposition remains unset.

## C0-E1 disposition

No unresolved authority-laundering path remains inside the bounded C0 first-lane
specification.

Residual risks are explicitly outside C1/C2 authority and fail closed.
