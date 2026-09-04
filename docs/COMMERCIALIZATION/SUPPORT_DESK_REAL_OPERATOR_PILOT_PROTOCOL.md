# AETHER AI Support Resolution Desk — Bounded Real-Operator Pilot Protocol

**Protocol ID:** `AETHER-SUPPORT-DESK-REAL-OPERATOR-PILOT-001`  
**Authority:** Founder-approved `APPROVE_A_SYNTHETIC_OR_SANITIZED_REAL_OPERATOR_PILOT` via `grandchallenge/GCT-EXECUTIVE#34`  
**Pilot intake:** `grandchallenge/AETHER#75`  
**Protected technical basis:** `b6cba2a595735874e42cbd5789dfca6d29c3ab3b`  
**DPQ-002-qualified semantic subject:** `f0f20532c40fcb389e55cdf10c44e5a3ac1423e9`  
**Canonical pilot matrix:** `docs/COMMERCIALIZATION/SUPPORT_DESK_DESIGN_PARTNER_PILOT_MATRIX.md` blob `18ea976bfbffd8ce6ffbad15095a1a941915e287`  
**Claim boundary:** `controlled_single_node_alpha`

## 1. Purpose

Measure whether real support operators can use the bounded AETHER Support Resolution Desk to complete and reconstruct representative support-resolution tasks with useful replay/provenance behavior, without exceeding the already-qualified controlled-single-node-alpha boundary.

This pilot is an evidence-acquisition exercise. It is not a commercial beta, production deployment, partnership commitment, product-superiority study, or operator-savings claim.

## 2. First-tranche size

The first tranche is deliberately small:

- **3 real operators**;
- **6 paired task cases per operator**;
- each operator completes each case under both:
  1. the AETHER support-desk condition; and
  2. a conventional ticket/event-log reconstruction condition;
- condition order is counterbalanced across operators where practical;
- total planned paired observations: **18 paired case comparisons**.

This sample is only for directional product evidence and defect discovery. It is not sized or powered to support population-level superiority, productivity, accuracy, preference, or savings claims.

Any increase beyond 3 operators or material expansion of task scope requires a recorded tranche extension against this same authority boundary.

## 3. Participant boundary

Participants may be internal operators or non-binding external design-partner/operator evaluators acting in a professional capacity.

Before participation, each operator must receive a concise briefing that states:

- this is an alpha evaluation, not production support software;
- all cases are synthetic, demonstrator, or deliberately sanitized;
- no real customer/private production data may be entered;
- no action suggested by the desk should be executed against a real external system as part of this pilot;
- observed timing and workflow data are evaluation evidence, not employee-performance assessment;
- the participant may stop a task or the session at any time;
- free-text feedback must not include real customer/private information.

Pilot evidence stores only a pseudonymous operator identifier such as `operator-01`. Names, email addresses, scheduling records, and other direct contact information must remain outside the AETHER pilot evidence packet.

Compensation, contracts, paid services, or other external obligations are not authorized by this protocol and require separate authority if proposed.

## 4. Data boundary

Allowed case material:

- synthetic cases created specifically for the pilot;
- demonstrator cases already present in the repository;
- deliberately sanitized examples whose sanitization provenance is recorded and whose content cannot reasonably identify a real customer, person, confidential transaction, or production secret.

Forbidden case material:

- customer/private production data;
- credentials, secrets, tokens, private keys, or production identifiers;
- unredacted support tickets or real support transcripts;
- personal information not necessary for the pilot;
- confidential third-party material not separately authorized for this use.

Each case must carry `data_classification` and `sanitization_provenance` fields in the evidence record. Any uncertainty about whether material is sufficiently sanitized is a stop condition.

## 5. Exact execution configuration

Every operator session must record:

- exact AETHER commit SHA;
- exact support-desk implementation/configuration identifier;
- journal backend;
- declared visibility/authority domain;
- authenticated operator identity as a pseudonymous pilot identifier;
- trusted-appender configuration;
- fixture/case-set revision;
- comparator revision;
- session start/end timestamps.

The pilot configuration must remain:

1. one AETHER node;
2. one declared visibility-and-authority domain;
3. SQLite default journal unless a separately reviewed alternative is admitted;
4. explicitly named trusted appenders;
5. support workflow limited to case, retrieved evidence, candidate resolution/escalation, approval, assignment/handoff, stale fencing, replay, and explanation;
6. no autonomous external action merely because a resolution becomes ready.

Material semantic/runtime drift from the DPQ-002-qualified slice requires fresh exact-revision DPQ qualification before further operator use.

## 6. Case set

The six cases must collectively include:

1. normal evidenced resolution;
2. missing-approval fail-closed path;
3. dependency-incomplete path;
4. ownership handoff with stale prior assignment;
5. suppressed recommendation;
6. closed/non-open case that must not produce a ready or selected resolution.

Each case has one frozen ground-truth packet specifying the expected answers to the five operator questions:

1. What support cases are active now?
2. What evidence is available for this case?
3. Which resolution, if any, is actually ready?
4. Who owns the case now, and what assignment is stale?
5. Why is the current selected resolution true or not true?

Ground truth must be frozen before the first participant session and may not be edited after observing pilot results except through an explicitly versioned correction that invalidates affected observations.

## 7. Conventional comparator

The comparator is intentionally ordinary: the same case facts are presented as a chronological ticket/event-log record with no AETHER semantic derivation, proof trace, replay query, or current-state semantic projection.

The comparator must:

- contain the same decision-relevant facts as the AETHER condition;
- preserve the same timestamps/event ordering;
- not intentionally handicap the conventional workflow through missing information, misleading formatting, or artificial delays;
- record the exact comparator fixture revision;
- permit the operator to inspect and reconstruct state using ordinary ticket/event-log operations only.

The purpose is to measure integration/workflow burden and reconstruction effort, not to create a straw-man baseline.

## 8. Per-task measurements

For each operator × case × condition, record:

- completion status: `complete`, `partial`, or `failed`;
- correctness against the five-question ground truth;
- observed task duration in seconds;
- operator action/inspection step count;
- number of backtracks or repeated inspections;
- stale-state error count;
- unsupported-workflow request count;
- recovery action count;
- whether replay/current-prior-cut reconstruction was used;
- whether proof/provenance explanation was used;
- operator-rated replay/provenance usefulness on a 1–5 ordinal scale when applicable;
- concise defect/blocker codes;
- optional sanitized free-text observation.

Timing begins when the task packet is presented and ends when the operator submits the five answers or abandons the task. Pauses caused by facilitator/tooling setup are recorded separately and excluded only when explicitly marked with reason.

## 9. Session-level measurements

Record per session:

- setup/startup defects;
- authentication/configuration friction;
- backup/recovery actions, if exercised;
- observed requests that fall outside the supported pilot matrix;
- operator confusion attributable to product semantics versus interface/packaging;
- any incident or stop-condition trigger;
- facilitator interventions.

Facilitator intervention that changes the participant's substantive answer invalidates that task observation for comparative timing/correctness purposes, while the intervention remains preserved as defect evidence.

## 10. Stop and narrow conditions

Pause the affected task immediately, and pause the full pilot when material, if any of the following occurs:

- non-synthetic/non-sanitized customer or private production data enters the pilot path;
- the running AETHER revision/configuration cannot be exactly identified;
- the running candidate materially differs from DPQ-002-qualified semantics without fresh qualification;
- a correctness defect causes a ready/selected result to violate the qualified support-desk contract;
- a case requires unsupported richer lifecycle semantics, managed multi-tenancy, multi-host consensus/failover, or another explicitly unsupported surface;
- a participant would need to execute an external real-world action to complete the task;
- an authorization, privacy, security, or contractual boundary would be exceeded;
- ordinary use is dominated by a blocking product defect rather than the semantic workflow under evaluation.

A stop does not count as pilot failure to be hidden or retried away. Preserve it as evidence and route remediation prospectively.

## 11. Analysis boundary

Permitted analysis:

- paired descriptive summaries for task completion, correctness, time, steps, reconstruction effort, recovery, and defects;
- case-level and operator-level paired differences;
- qualitative categorization of unsupported requests and blocking gaps;
- uncertainty ranges and raw-observation tables;
- identification of conditions under which the AETHER workflow is easier, harder, or simply different.

Not permitted from this tranche alone:

- product-superiority claims;
- generalized productivity or cost-savings claims;
- statistical population claims;
- commercial ROI claims;
- production-readiness, SLA, beta, or GA claims;
- statements that operator preference has been established beyond the observed participants/tasks.

## 12. Return packet

The governed pilot return must contain:

- protocol ID and exact protocol revision;
- Founder authority reference;
- exact AETHER subject/configuration and case/comparator revisions;
- participant count and pseudonymous IDs;
- all valid per-task evidence records;
- invalidated observations with reasons;
- descriptive paired summaries;
- defect/recovery ledger;
- unsupported-request ledger;
- stop/narrow events;
- data-boundary/sanitization attestations;
- residual uncertainty and missing evidence;
- a proposed enterprise disposition among `PRODUCTIZE`, `ACTIVE`, `NARROW`, `HOLD`, or `RETIRE`, with rationale and reversal conditions.

A proposed `PRODUCTIZE` disposition remains a recommendation only. It does not create product, commercial, partnership, pricing, data, hosting, production, or public-claim authority.

## 13. Pre-run gate

No real-operator session may begin until:

1. this protocol and its evidence schema are protected on AETHER `main` under existing provider controls;
2. exact-head CI/supply-chain/policy/routing checks are green;
3. an independent review has accepted the protocol subject;
4. the exact case/comparator fixtures intended for the first session are frozen and recorded;
5. the running AETHER subject is either the DPQ-002-qualified subject or a later exact subject shown not to materially change the qualified semantics.
