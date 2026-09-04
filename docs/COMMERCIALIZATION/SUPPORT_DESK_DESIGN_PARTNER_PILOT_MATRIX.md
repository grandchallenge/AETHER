# AETHER AI Support Resolution Desk — Design-Partner Pilot Matrix

**Matrix ID:** `AETHER-SUPPORT-DESK-PILOT-MATRIX-001`  
**Qualified technical substrate:** `f0f20532c40fcb389e55cdf10c44e5a3ac1423e9`  
**Canonical evidence custody:** protected `main` `6a1babb83789f693c3d36dbec51383bfb9fb86f3`  
**DPQ:** `AETHER-DPQ-SUPPORT-DESK-002` — `TECHNICAL_DPQ_PASS__PRODUCT_SUPERIORITY_UNESTABLISHED`  
**Active claim boundary:** `controlled_single_node_alpha`  
**Decision purpose:** identify what a bounded design-partner pilot may truthfully exercise, what still requires explicit authority or additional evidence, and what remains outside scope.

## Bound source records

- support-desk narrative: `docs/COMMERCIALIZATION/AI_SUPPORT_RESOLUTION_DESK.md`, blob `24733203e86bca963bda877119e094f7ab8c046c`;
- known limitations: `docs/KNOWN_LIMITATIONS.md`, blob `0ddc078eeb3ec09217a715442dff51bdc3ad0698`;
- repaired support-desk implementation: `crates/aether_api/examples/demo_05_ai_support_resolution_desk.rs`, blob `1a1b9747e87398d4539d7b2e47714ad1d7a3598d`;
- DPQ-002 machine result: `docs/evidence/AETHER_DPQ_SUPPORT_DESK_002.json`, blob `db6f38d6cd15e9b29e0108ac485cb7bbdba07b8c`;
- DPQ-002 human receipt: `docs/evidence/AETHER_DPQ_SUPPORT_DESK_002.md`, blob `475373a713e2740381acb71b776fd4e3dc5769ba`.

`AETHER-DPQ-SUPPORT-DESK-001` remains immutable historical evidence for its earlier exact subject. This matrix does not rewrite it or widen any source claim. If a source record and this matrix conflict, the narrower source claim governs.

## Pilot posture

The technically credible wedge is a **single-node governed support-resolution desk** in which support cases, retrieved evidence, candidate resolutions, approvals, assignments, stale recommendations, replay, and explanation are exercised inside one controlled AETHER deployment boundary.

The prior lifecycle blocker is closed. AETHER #70 was repaired through independently reviewed #71, and protected-main `AETHER-DPQ-SUPPORT-DESK-002` passed on exact subject `f0f20532c40fcb389e55cdf10c44e5a3ac1423e9`. The qualification now proves that readiness and selected-resolution derivation require an open case in the bound support-desk slice; the closed-case scenario fails closed while preserving historical ownership reconstruction.

This establishes bounded **technical acceptance** of the repaired support-desk wedge. It does not itself authorize external design-partner contact or operation. Any real external pilot remains subject to separate applicable GCT Founder authority, and any customer/private-data use requires separate data/privacy/security authorization and an exact handling plan.

The pilot is not a commercial-beta launch, autonomous support SaaS, managed multi-tenant service, stable new product API, production-readiness claim, or distributed-control-plane claim.

## Supported now — bounded pilot surface

| Surface | Pilot status | Evidence / boundary |
| --- | --- | --- |
| Governed support-case desk | `SUPPORTED` | Current exemplar represents cases, evidence, candidate resolutions, approvals, escalations and assignment state. |
| Derived action readiness | `SUPPORTED_WITHIN_QUALIFIED_OPEN_CASE_SLICE` | DPQ-002 passed evidence/approval/confidence/suppression/dependency behavior and requires `case_status == open` for readiness. |
| Non-open case readiness/selection fencing | `SUPPORTED` | DPQ-002 `closed_case_block` passed: ready=`false`, selected=`false`, current owner=`lead-ana`, reason=`case_not_open`. |
| Missing-approval fail-closed behavior | `SUPPORTED` | DPQ-002 `missing_approval_block` passed with `approval_missing`. |
| Suppression fail-closed behavior | `SUPPORTED` | DPQ-002 `suppressed_resolution` passed with `suppressed`. |
| Lease-backed handoff / current owner | `SUPPORTED` | DPQ-002 handoff scenario selects `lead-ana` while preserving `triage-agent` as historical stale ownership. |
| Stale recommendation fencing | `SUPPORTED` | Historical stale assignment remains visible but non-current; DPQ-002 handoff/stale-fencing scenario passed. |
| Retrieved evidence subordinate to semantic control | `SUPPORTED` | Artifact/vector retrieval re-enters the rule path as evidence; retrieval is not the authority layer. |
| `Current` / prior-cut replay | `SUPPORTED` | Current single-node kernel supports replay at exact cuts within its declared semantic slice. |
| Provenance-bearing explanation | `SUPPORTED` | Current exemplar exposes why the selected path is true through explanation/proof surfaces. |
| Runnable demonstration / workflow pack | `SUPPORTED` | Existing customer-workflow acceptance passed in the protected DPQ-002 execution. |
| Single-node packaged operation | `SUPPORTED_WITH_OPERATOR_DISCIPLINE` | Current package has startup, auth, backup/restore and rotation helpers; it remains an operator-managed single-node bundle. |
| SQLite default journal | `SUPPORTED` | SQLite remains the default local/package journal backend inside the controlled-alpha boundary. |
| Trusted appenders / authenticated service boundary | `SUPPORTED_WITHIN_DECLARED_CONFIG` | Active claim is limited to trusted appenders and explicitly supported deployment boundaries. |

## Conditional — admissible only with explicit constraints or more evidence

| Surface | Pilot status | Required condition / missing evidence |
| --- | --- | --- |
| Real design-partner users | `CONDITIONAL__FOUNDER_AUTHORITY_REQUIRED` | Technical acceptance is now satisfied, but external pilot/partner engagement requires separate applicable GCT Founder authority. |
| Customer/private production data | `CONDITIONAL__SEPARATE_AUTHORIZATION_REQUIRED` | Not authorized by Mandate 002 technical qualification; requires separate data/privacy/security authorization and exact handling plan. |
| Real operator-effort measurement | `CONDITIONAL__NEXT_EVIDENCE` | Needed before any claim of time savings, cost savings, accuracy improvement, preference or product superiority. |
| Richer case-lifecycle semantics | `CONDITIONAL__OUTSIDE_DPQ_002` | DPQ-002 qualifies explicit open versus non-open fencing only. Any richer lifecycle states or transition semantics require deliberate definition and separate fail-capable tests before reliance. |
| Optional Postgres journal | `CONDITIONAL` | Journal-first optional backend only; not a SQL rule engine, global `AsOf`, consensus layer, or sidecar catalog. Remote/TLS deployment claims remain bounded by exact hosted evidence. |
| Non-loopback HTTP deployment | `CONDITIONAL` | Requires explicitly trusted HTTPS ingress and current ingress-isolation/TLS evidence; native HTTP TLS is not claimed. |
| Performance / sizing discussion | `CONDITIONAL__DIAGNOSTIC_ONLY` | Capacity/performance artifacts are planning diagnostics, not SLA, cloud-SKU, throughput or commercial sizing guarantees. |
| Go/Python client use | `CONDITIONAL__EARLY_SURFACE` | Capability-negotiated real boundary clients, but still early and not a stable broad administration/product SDK commitment. |
| Backup/restore during pilot | `CONDITIONAL__QUIESCED` | File-copy procedure requires stopped-service confirmation; no online snapshot/coordination lock is claimed. |

## Explicitly unsupported / out of pilot scope

| Surface | Pilot status | Reason |
| --- | --- | --- |
| Turnkey autonomous support SaaS | `UNSUPPORTED` | Explicit non-claim in the support-desk narrative. |
| Finished ML operations platform | `UNSUPPORTED` | Explicit non-claim. |
| Stable new top-level product API | `UNSUPPORTED` | Current API/facade surfaces remain compatibility/early product boundaries. |
| Managed multi-tenant platform | `UNSUPPORTED` | Namespace isolation is not a full managed multi-tenant authority/deployment platform. |
| Multi-host failover / automatic election | `UNSUPPORTED` | Current replicated prototype lacks automatic election and managed failover. |
| Quorum consensus / generalized distributed truth | `UNSUPPORTED` | Outside controlled single-node alpha and current partition prototype. |
| Sidecar-independent replication/failover | `UNSUPPORTED` | Sidecars remain partition-local and journal-subordinated. |
| Native HTTP TLS product claim | `UNSUPPORTED` | Current supported pattern is loopback HTTP or trusted external TLS ingress; native HTTP TLS is not claimed. |
| Commercial beta | `UNSUPPORTED_PENDING_SEPARATE_RELEASE_QUALIFICATION` | Governed by non-waivable exact-candidate release policy; DPQ cannot promote it. |
| Production readiness / GA | `UNSUPPORTED` | DPQ explicitly cannot establish production readiness; GA blockers remain separate. |
| Human-time or monetary savings claim | `UNSUPPORTED_WITH_CURRENT_EVIDENCE` | Synthetic comparator units are structural proxies only. |
| Product superiority claim | `UNSUPPORTED_WITH_CURRENT_EVIDENCE` | DPQ-002 signal explicitly states superiority unestablished. |
| SLA / availability / commercial sizing promise | `UNSUPPORTED_WITH_CURRENT_EVIDENCE` | Current performance/capacity outputs are diagnostic planning artifacts, not guarantees. |

## Minimum bounded pilot configuration

Any future authorized design-partner pilot should start no broader than:

1. a candidate that preserves the DPQ-002-qualified support-desk semantics of exact subject `f0f20532c40fcb389e55cdf10c44e5a3ac1423e9`, including open-case gating for readiness and selection; any material semantic/runtime drift requires fresh exact-revision qualification before this matrix is relied upon;
2. one AETHER node / one declared visibility and authority domain;
3. explicitly named trusted appenders and authenticated operator identities;
4. SQLite default journal unless a separately justified Postgres path is admitted;
5. support-desk workflow limited to case, retrieved evidence, candidate resolution/escalation, approval, assignment, stale fencing, replay and explanation;
6. explicit capture of current/prior-cut state and why-selected traces;
7. no autonomous external action outside the desk merely because a resolution becomes ready;
8. no customer/private data until separately authorized;
9. no claim of operator savings, superiority, commercial beta or production readiness;
10. exact incident/rollback/backup procedure declared before pilot contact;
11. pilot evidence retained with exact software revision, configuration, scenario, operator role and outcome.

## Pilot acceptance measurements

A bounded authorized pilot may legitimately measure:

- whether the five support-team questions can be answered reliably;
- correctness of case-lifecycle/approval/suppression/dependency/readiness state;
- open versus non-open readiness/selection fencing;
- handoff and stale-state fencing behavior;
- replay/provenance usefulness for reconstructing decisions;
- defect/recovery count;
- operator steps and observed task time, provided those are measured from real participants rather than inferred from the synthetic comparator;
- unsupported workflow requests encountered;
- which product gaps actually block continued use.

Any human-performance measurement must preserve raw observations and uncertainty and must not be converted automatically into a superiority claim.

## Stop conditions

Stop or narrow the pilot if any of these occur:

- the candidate materially diverges from the DPQ-002-qualified semantics without fresh exact-revision qualification, or canonical DPQ-002 evidence becomes absent, stale, or contradicted;
- requested workflow requires richer lifecycle semantics that have not been explicitly defined and fail-capably tested;
- requested workflow requires multi-host consensus/failover or managed multi-tenancy;
- authorization or privacy handling exceeds the admitted pilot envelope;
- an unsupported API/deployment surface becomes necessary for ordinary use;
- operator burden is dominated by product gaps rather than the semantic workflow being evaluated;
- a commercial/product claim would outrun the controlled-alpha evidence boundary.

## Current disposition

`BOUNDED_TECHNICAL_ACCEPTANCE_SUPPORTED__EXTERNAL_PILOT_NOT_AUTHORIZED`

DPQ-002 closes the identified open/non-open case-lifecycle defect and supports bounded technical acceptance of the current support-desk wedge. It does **not** itself constitute a GCT `PRODUCTIZE` or `PARTNER` disposition and does not authorize external pilot contact, customer/private data, commercial beta, production readiness, pricing, contracts, paid hosting, product-superiority language, or operator-savings claims.

The next decision-relevant sequence is: protect this refreshed matrix against canonical DPQ-002 evidence; route the resulting bounded technical packet to the GCT Mandate 002 enterprise-disposition lane; and, only if separately authorized by the applicable Founder authority, consider a narrowly scoped real operator/design-partner pilot under explicit data/privacy/security controls. Real operator evidence must precede any claim of human-time savings, monetary savings, preference, accuracy improvement, or product superiority.
