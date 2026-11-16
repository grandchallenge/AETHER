# AETHER-NORM-001: Context-Indexed Normative Semantics and Compositional Authority

**Status:** RESEARCH CHARTER CANDIDATE — NON-OPERATIVE  
**Date:** 2026-11-16  
**Base:** protected `grandchallenge/AETHER` main at `310b26ac6b8e0b449a90dd1e76cba1ad1327f1e6`  
**Scope:** specification and falsifiable off-path research, not operative policy, deployment, runtime authorization, constitutional amendment, or release qualification.

## Thesis

Develop a formal account of norms as context-indexed, evidence-bearing relationships among agents, roles, actions, claims and institutions. The goal is not to automate moral judgment, but to make the structure, exercise, conflict and revision of judgment explicit, inspectable and accountable. Logical validity relative to admitted rules must never be mistaken for moral justification or institutional authority.

The programme combines (i) deontic semantics and Hohfeldian relations, (ii) AETHER's exact historical cuts, policy projection, recursive derivation and provenance, and (iii) typed compositional delegation, revocation and protected admission. Its contribution must be tested against simpler typed policy models; abstract mathematical machinery is optional unless it yields stronger theorems or discriminating counterexamples.

## Existing ownership and non-authority

AETHER owns the journal, resolver, rule execution, policy visibility, exact source proofs, and governed action admission. INTELLECT owns constitutional roles and judgments within its separate protected repository. FABRIC does not mint semantic authority; AETHER E3B routing remains default-OFF and separately governed.

This charter creates **no new permission**, policy authority, approval role, service, database, production integration, proof of ethical correctness, or release claim. NORM-001 may use synthetic fixtures and research documents only. Any operative or cross-repository integration requires separate exact-delta governance and evidence.

## Mathematical specification candidate

Define a finite category/poset `Ctx` of *observational contexts*, each bound to a partition, exact historical cut, principal, effective visibility, policy digest, rule digest and explicit authority source. Treat time advancement and policy revision as distinct state transitions; neither is silently an information-restriction map.

For each context `c`, a tentative normative interpretation is
`N(c)=(F_c,O_c,P_c,H_c,E_c,D_c)`:

- `F_c`: admitted factual observations and history;
- `O_c`: duties and contrary-to-duty expressions;
- `P_c`: stated permissions and prohibitions, **not** operative action grants;
- `H_c`: rights, duties, powers and immunities with explicit counterparties;
- `E_c`: evidential qualification and source provenance;
- `D_c`: typed delegation scope, principal, target, operation, validity and revocation references.

Investigate restriction maps `ρ_{c,c'}:N(c)→N(c')` for a *specified class* of compatible context morphisms. A presheaf, indexed-category or fibration interpretation must be established by composition/identity laws, not presumed. Privacy projection and revoked authority can defeat naive restriction claims.

For positive monotone rule fragments, fixed-point closure may be modeled as
`Cl_c(S)=μX.(S∪T_c(X))` on a suitable lattice. This is rule consequence, **not** moral completion, consistency, justification or permission. Exceptions, stratified negation, supersession and revocation require separate version-aware treatment. There is no generally unique repair of `O(a)` and `O(¬a)` absent extra normative criteria.

Define a *historical justification* judgment `Γ;c⊢MayExecute(a,α,x)[e]`. It cannot issue live permission. Any real start would require a fresh source-owned admission and atomic start boundary under AETHER's existing E3B governance. An old receipt is not an executable capability.

## Research questions

**RQ1 — Contextual restriction:** Under precisely which assumptions do narrower views preserve relevant conclusions and prevent authority amplification? Supply negative counterexamples, including hidden-fact inference and epoch changes.

**RQ2 — Compositional authority:** Develop typed partial composition of delegations; test `Cap(g∘f) ⊆ Cap(f)` for admitted delegation paths. Consider rights of principals, attenuation, expiries, revoked intermediates, cross-domain interpretation, and proof identity. Check whether categorical composition is an appropriate formalism rather than a rhetorical analogy.

**RQ3 — Local/global coherence:** For finite covers of contexts, decide when local normative assignments agree on overlap and admit a global section. Produce a locally satisfiable but globally ungluable example. Do not claim cohomological obstructions without a constructed complex and a proven correspondence.

**RQ4 — Norm conflict:** Distinguish deontic inconsistency, incompatible authority, missing information and irreconcilable purposes. Record multiple plausible repairs and why no canonical resolution follows without priorities or institutional judgment.

**RQ5 — Accountability:** Distinguish deterministic fixed-history policy replay from causal counterfactual claims about changed agent behavior. The latter need a separate causal model and uncertainty statement.

**RQ6 — Invariance:** Identify exact transformations that preserve judgments and provenance; do not infer moral correctness from invariance or universal properties.

## Finite corpus and hostile tests

First domain: synthetic INTELLECT-like research claim promotion, including proposal, evidence, verification, adversarial review, referee decision and protected promotion. No live records or permissions are modified.

Each fixture must carry an exact context/cut, rule/policy identity, principal, assumptions, event history, expected judgment, provenance trace, denial explanation and `authority_effect: none`.

Minimum conformance set:

1. Valid duty and evidenced discharge.
2. Duty without discharge; mere agent invocation is not review.
3. Prohibition plus explicit sourced exception.
4. Contradictory duties `O(a)` and `O(¬a)`.
5. Two equally minimal repairs; no canonical selection.
6. Two-hop scope-attenuated delegation.
7. Third-hop permission amplification denied.
8. Valid historical receipt followed by revocation before the proposed start.
9. Cross-principal or cross-partition replay denied.
10. Hidden fact projection with noninterference/opacity where policy requires.
11. Compatible local interpretations with a global gluing witness.
12. Individually satisfiable but globally incompatible local interpretations.
13. Fixed-history counterfactual replay distinguished from uncertain behavioral prediction.

Include tests where the proposed categorical construction fails or offers no advantage over straightforward policy logic.

## Bounded phases

### NORM-001A — Definitions and countermodels
Deliver typed syntax, mathematical semantics, source comparison to deontic logic, Hohfeld, ODRL, PROV-O, capability security, and (pre)sheaf methods; two formal negative results; an explicit ledger of assumptions. Gate: reviewed definitions and independently inspectable counterexamples. No kernel code.

### NORM-001B — Non-operative conformance
Build a deterministic finite research runner using **existing** test machinery, with no new service/database. Exact-head conformance corpus and reproducible PASS/FAIL/UNIMPLEMENTED receipts. Gate: all 13 fixture cases admitted as off-path research evidence; no authority effect.

### NORM-001C — Delegation and temporal proof
Formalize and where possible mechanically check attenuation, no privilege widening, cut/epoch validity, revocation and time-of-check/time-of-use counterexamples. Distinguish source-verified facts from model-only results. Gate: narrow theorem statements, hostile tests, exact-head reviews; no E3B-A authorization.

### NORM-001D — Coherence and mathematical value
Evaluate whether presheaf, sheaf or fibrational structures establish a useful new invariant or obstruction beyond standard policy logic. Gate: a substantive theorem, counterexample or documented **negative** research result; retire decorative machinery if it contributes no discriminatory value.

## Acceptance and honesty conditions

- For fixed inputs and rules, research decisions are deterministic, replayable, and provenance-addressable.
- No unknown, stale, revoked, out-of-scope, cross-principal or unauthorized chain can be interpreted as a live grant.
- The model explicitly separates factual observation, probability/confidence, normative conclusions, institutional approval, and actual execution permission.
- No scalar moral reputation score or inference of moral worth from policy compliance.
- No claim that fixed-point closure yields unique moral truth, globally consistent ethical gluing, or a universally canonical repair.
- No source-verified or production qualification label without actual source tests and protected controls.
- Compare runtime and conceptual complexity with a simple typed policy baseline; abandon unnecessary layers.

## First successor and governance

If this charter is admitted through protected review, create a bounded NORM-001A work item referencing the admitted SHA. Do not automatically activate further phases, alter `docs/ROADMAP.md`, `docs/STATUS.md`, `SPEC.md`, `INTELLECT`, or operational code. Preserve the current controlled single-node alpha label, operator acceptance work and E3B source-owner boundaries. Formalist, Adversary and Referee assessments are logical review passes under existing staffing policy; they must not be portrayed as independent human review.
