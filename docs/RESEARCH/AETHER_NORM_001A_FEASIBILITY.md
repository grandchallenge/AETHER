# AETHER-NORM-001A — Feasibility review, first lemmas and countermodels

**Status:** RESEARCH EVIDENCE; finite model only; **not** AETHER source verification or an operative authorization result.  
**Date:** 2026-11-16.  
**Parent:** [AETHER-NORM-001 charter](AETHER_NORMATIVE_SEMANTICS_PROGRAMME.md).  
**Source boundary:** AETHER `main` at `310b26ac6b8e0b449a90dd1e76cba1ad1327f1e6`; original charter candidate `21f973a8795115b5ff275cdcf61bff594baf21ca`.

## Decision

**Advance a bounded NORM-001A formalization, not a general moral-topology product.**

The current proposal has a tractable formal core (capability attenuation, interpretation of missing information, finite constraint satisfiability and norm-conflict repair) and an existing implementation target (AETHER's already-established event/time-cut and provenance semantics). Its broader categorical novelty is **unproved**. Should NORM-001A fail to distinguish its framework from conventional typed authorization plus a rule engine, abandon the categorical layer and keep only any valuable finite semantics.

A review of AETHER's source-owned E3B contracts reveals that all execution-start and revocation/epoch atomicity remain separate governed work. No research result here may be interpreted as closing, implementing, or activating E3B-A.

## Model 1 — Scope-intersection attenuation

Let `U` be a finite capability universe. Let an original source-owned authority grant be `A₀ ⊆ U`. For delegation step `i`, let the valid scope be `Sᵢ ⊆ U` and let `validᵢ` be the *independently established* conjunction of relevant issuer, recipient, time, revocation and policy conditions.

Define

```text
A_i = A_(i-1) ∩ S_i    if valid_i
A_i = ∅                otherwise.
```

**Lemma 1 (attenuation).** `A_i ⊆ A_(i-1) ⊆ A₀` for every `i ≥ 1`.

**Proof.** In the valid case an intersection is contained in each of its factors. In the invalid case the empty set is contained in every set. Induction gives the result. □

**Scope of lemma.** This is elementary, established mathematics and is **not a novelty claim**. It assumes no stage can mint a fresh, separately authorized grant and that validity is trustworthy. If independent authority escalation exists, a new source-owned grant must be modeled rather than hidden inside the same delegation chain. A fabricated `validᵢ=true`, stale validity read or point-of-start race can still permit an unauthorized real-world action; the set theorem does not prevent this.

**Falsification target.** Find a legal model extension that permits a narrower proof chain to increase effective privileges without an independent source-owned grant, or show that current AETHER executor integration cannot preserve the source-owned validity predicate at the action linearization point.

## Model 2 — Visibility is not negation

Given source facts `F={blocked}` and a rule `permit :- not blocked`, a broad view derives no permission. A narrowed view that hides `blocked` and treats an absent fact as false may derive `permit`.

Thus restricting visible facts can increase derived permissions if negation-as-failure is used naively. This is a **counterexample to unqualified restriction/naturality or privilege non-amplification**, not evidence of an actual defect in AETHER's implementation.

**Consequence:** Normative conclusions must distinguish missing, hidden, false, and source-verified negative facts, or prohibit an observational projection from authorizing an action. An authorized execution predicate must be checked at its source-owned authority boundary; visible policy inference can remain an explanation, never a minted grant.

## Model 3 — Local satisfiability does not imply global consistency

Let the agents/contexts use variables `x,y,z ∈ {0,1}` with three locally valid constraints:

```text
C_xy: x != y
C_yz: y != z
C_zx: z != x
```

Each edge-context admits two local satisfying assignments, and each vertex's local feasible-value projection is `{0,1}`. However, no global `(x,y,z)` satisfies all three constraints (an odd binary inequality cycle).

**Proposition 2.** There exist collections of individually satisfiable, pairwise domain-compatible local normative constraints whose conjunction is unsatisfiable.

**Proof.** If `x ≠ y` and `y ≠ z` for binary values then `x=z`, contradicting `z ≠ x`. □

**Important limitation.** This example is *not* a counterexample to the sheaf gluing axiom. It shows that compatible **projected domains or local solution sets** need not supply a matching family of individual local sections. The distinction between projections and matching sections must be explicit before invoking sheaf cohomology.

## Model 4 — No canonical norm-conflict repair

For obligations `{O(a), O(¬a)}`, restoring satisfiability by subset-minimal deletion yields two equally minimal repairs:

```text
{O(a)}
{O(¬a)}
```

No unique repair follows from the logical language alone. An institutional priority, reasoned adjudication, or revision authority must choose between them. Formal inconsistency detection does not imply moral judgment or permission to rewrite the rules.

## Empirical record — finite exhaustive probes

On the authenticated GCL CHAD operating environment, a local one-shot Python 3 finite-model probe completed with exit code `0`.

- **Attenuation:** 65,536 exhaustively enumerated three-hop combinations of four-capability subsets, with intermediate anti-amplification assertions; PASS.
- **Local/global:** three individually satisfiable binary inequality edges with no global solution; PASS.
- **Repair:** two distinct subset-minimal single-removal repairs; PASS.
- **Visibility:** naive closed-world projection converts a denial into a derived permission; PASS.

**Execution capture:** `03be907446fa4871b5ee2131a184792e`; `command_sha256=a2d6f003457a097c9b50e70db2581abd8d29070c58b47bc18525ed8505355cde`. The execution capture states `LOCAL_OS_ONLY_NOT_INDEPENDENTLY_AUTHENTICATED`; this is **exploratory model evidence**, not a release gate, AETHER kernel integration, independent mathematical referee report, or proof of temporal source-owner admission.

## Distinct review questions

**Formalist:** Are the context morphisms, codomains, restriction naturality laws, and authorization proof syntax precisely typed? Can any claimed theorem be proved without smuggling in the conclusion as an assumption?

**Adversary:** Does closed-world negation after visibility filtering produce a false permission? Can revocation between proof and execution start invalidate a historical authorization? Can apparently compatible local permissions lack any global assignment? Do alternative repairs compete?

**Referee:** Retain the modest finite lemmas and demonstrated countermodels; reject any claim of canonical ethics, global sheaf coherence, genuine runtime authority, new cryptographic guarantee, or novelty from set intersection.

## NORM-001A delivery contract

**Next authorized work:** versioned syntax; exact context-indexed semantics; status/evidence types; an explicit mapping to existing AETHER `Current/AsOf`, provenance and policy mechanisms; comparisons with ODRL, PROV-O, Hohfeld and standard delegation/access-control logic; at least two nontrivial countermodels. Publish an assumption ledger and a novelty/complexity comparison. No runtime code.

**Decision gate:** If no new compositional law, proof technique or meaningful diagnostic can be demonstrated beyond existing typed policy checks, **shelve the moral-topology abstraction** and preserve any independently useful conformance tests under existing AETHER governance. An ordinary trivial attenuation lemma does not pass this novelty gate.

**Do not advance to NORM-001B:** until NORM-001A is source-bound, mathematically reviewed, and admitted under protected repository policy. No governance authority is granted by this document.
