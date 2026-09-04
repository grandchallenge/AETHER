# AETHER Support Desk Real-Operator Pilot — Facilitator Plan

**Protocol:** `AETHER-SUPPORT-DESK-REAL-OPERATOR-PILOT-001`  
**Case set:** `AETHER-SUPPORT-DESK-REAL-OPERATOR-CASES-001`  
**Protected protocol revision:** `7f1a169d9c078792f057c0a72d60338acbb600c9`

## Purpose

Run the first three-operator tranche without changing cases, ground truth, task wording, comparator facts, or scoring after participant observations begin.

## Frozen operator questions

For every case and condition, ask exactly:

1. What support cases are active now?
2. What evidence is available for this case?
3. Which resolution, if any, is actually ready?
4. Who owns the case now, and what assignment is stale?
5. Why is the current selected resolution true or not true?

The facilitator may explain interface mechanics before timing begins but must not explain the semantic answer during a timed task.

## Counterbalanced first-tranche order

Each operator completes all six cases in both conditions. To reduce simple condition-order bias, use these frozen schedules:

### operator-01

| Pair | Case | First condition | Second condition |
| --- | --- | --- | --- |
| 1 | `pilot-case-01-normal-resolution` | AETHER | conventional event log |
| 2 | `pilot-case-02-missing-approval` | conventional event log | AETHER |
| 3 | `pilot-case-03-dependency-incomplete` | AETHER | conventional event log |
| 4 | `pilot-case-04-handoff-stale-fencing` | conventional event log | AETHER |
| 5 | `pilot-case-05-suppressed-resolution` | AETHER | conventional event log |
| 6 | `pilot-case-06-closed-case-fencing` | conventional event log | AETHER |

### operator-02

Reverse operator-01's condition order for every case.

### operator-03

| Pair | Case | First condition | Second condition |
| --- | --- | --- | --- |
| 1 | `pilot-case-04-handoff-stale-fencing` | AETHER | conventional event log |
| 2 | `pilot-case-05-suppressed-resolution` | conventional event log | AETHER |
| 3 | `pilot-case-06-closed-case-fencing` | AETHER | conventional event log |
| 4 | `pilot-case-01-normal-resolution` | conventional event log | AETHER |
| 5 | `pilot-case-02-missing-approval` | AETHER | conventional event log |
| 6 | `pilot-case-03-dependency-incomplete` | conventional event log | AETHER |

The case order difference for operator-03 is deliberate but does not remove learning/carryover effects. Preserve condition order as evidence rather than attempting an inferential correction unsupported by a three-person tranche.

## Briefing script content requirements

Before the first timed task, the facilitator must communicate these facts in ordinary language:

- AETHER is an alpha evaluation, not production support software.
- All pilot cases are synthetic or deliberately sanitized; participants must not enter real customer/private production information.
- No pilot recommendation should be executed against a real external system.
- Timing and workflow observations are product-evaluation evidence, not employee-performance assessment.
- The participant may stop any task or the session.
- Free-text feedback must not include real customer/private information.

Record `briefing_acknowledged=true` only after the participant confirms understanding.

## Timing rule

Start the timer when the case condition is presented and the operator is free to work. Stop when the operator submits all five answers or abandons the task.

Exclude facilitator/tooling pauses only if:

1. the timer interruption is recorded;
2. the reason is recorded; and
3. the interruption did not provide substantive semantic guidance.

Do not silently remove difficult time from the observation.

## Intervention rule

A facilitator may help with mechanical navigation when the operator cannot access a control because of pilot packaging. Record every such intervention.

If an intervention changes or strongly cues the substantive answer, mark the task observation invalid for comparative timing/correctness while preserving the observation as defect evidence.

## Ground-truth scoring

Score the five operator answers against the frozen `ground_truth` object for the case-set revision used in the session. Do not edit ground truth in response to an operator answer.

If a genuine ground-truth defect is discovered:

1. stop affected scoring;
2. open a versioned correction;
3. identify every affected prior observation;
4. invalidate those observations prospectively;
5. do not overwrite the original case-set bytes.

## Data hygiene

Use only pseudonymous IDs such as `operator-01`. Direct participant names, emails, scheduling metadata, compensation details, or other contact records remain outside the AETHER evidence packet.

Before accepting sanitized free text, inspect it for customer names, personal data, credentials, private identifiers, or confidential third-party content. Redact or reject it before repository evidence custody.

## Stop conditions

The facilitator must stop or pause under the protocol if:

- customer/private production data appears;
- exact running revision/configuration cannot be established;
- the running semantics materially diverge from the DPQ-002-qualified slice without fresh qualification;
- a correctness defect violates the qualified readiness/selection contract;
- an unsupported distributed/multi-tenant/richer-lifecycle surface is required;
- task completion would require a real external action;
- an authorization/privacy/security/contractual boundary would be exceeded;
- blocking packaging/product defects dominate the semantic workflow being evaluated.

Preserve stops as evidence. Do not repeat a stopped task merely to obtain a cleaner result.

## End-of-session closeout

Before the participant leaves:

- confirm all twelve condition observations are accounted for or explicitly invalidated/not-run;
- capture unsupported workflow requests;
- capture semantic versus interface/packaging confusion separately;
- record facilitator interventions;
- verify no customer/private data entered the evidence packet;
- preserve raw observations before any summary is produced.
