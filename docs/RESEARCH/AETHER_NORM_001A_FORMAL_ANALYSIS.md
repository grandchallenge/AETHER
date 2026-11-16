# AETHER-NORM-001A — Contextual Judgment, Delegation, and Obstructions to Normative Descent

**Version:** 1.0.0 (bounded mathematical research note)  
**Date:** 2026-11-16  
**Issue:** [AETHER #162](https://github.com/grandchallenge/AETHER/issues/162)  
**Protected starting head:** `17fdded4d2e3a202f0114812de3dfddd49cf9da2`  
**Classification:** `PROVED_FINITE_MATHEMATICS` for the elementary propositions below; `EXHAUSTIVE_MODEL_TESTED` for the recorded finite probes; `NOT_SOURCE_VERIFIED` for AETHER runtime authorization.  
**Authority effect:** `none`. No operative rule, policy, software service, new authority, or constitutional amendment is proposed.

## Abstract and decision

The motivating question is whether an institution's normative judgments can be coherently interpreted across restricted observational contexts, changing historical cuts, and composed delegation paths. We construct a finite observation-presheaf, prove an exact **factorization criterion** for lifting a full-information binary judgment down to a restricted view, construct the **maximally informative sound three-valued abstraction**, and prove a sufficient **dependency-closure condition** for commuting positive Horn closure with visibility restriction. We give countermodels for closed-world negation, conflicting repairs, and locally compatible probabilistic marginals lacking a global joint distribution. We also give a narrowly conditioned delegation attenuation result.

**Disposition: ADVANCE a small non-operative NORM-001B conformance study of the cross-context diagnostic only; SHELVE the larger claim that a new autonomous “moral topology” or categorical governance engine is warranted.** The mathematical constructions are not claimed novel in themselves: they are applications of quotient factorization, abstract interpretation, Horn-program slicing, constraints, and presheaf contextuality. The potential contribution is an *integrated, provenance/cut-bound cross-context diagnostic* that AETHER can test against its existing scoped semantics without adding a new authority path. If that diagnostic does not catch distinct policy/context failures over ordinary baseline fixtures, stop NORM-001B as well.

## 1. Definition of the research object

Fix an AETHER authority partition `p`, immutable historical cut `t`, effective schema/rule/policy identities `s,r,g`, and a finite set `A` of **ground observational atoms** already admitted for analysis. The research model does **not** equate authority partitions, authenticate issuers, or select a global `AsOf` cut.

An observation context has type

```text
Context = {
  authority_partition: PartitionId,
  exact_cut: ElementId,
  principal: PrincipalId,
  visible_atoms: subset(A),
  policy_digest: Digest,
  rule_digest: Digest,
  authorization_source: SourceId
}
```

For a **fixed** `(p,t,s,r,g)` and **one authorized observational-domain relation**, select a finite collection `C` of visible-atom sets closed under intersection and ordered by inclusion, `V ≤ W` iff `V ⊆ W`. An arrow `V → W` means that `V` contains no more observable atoms than `W`; it is **not** a delegation of rights or a new principal authorization. Do not draw an arrow between incompatible principals, policy digests, partitions, cuts, or schemas. Principals may be encoded as distinct observation objects, but morphisms are permitted only by an explicit policy-certified restriction relation.

Let `2^V` be the set of Boolean valuations on atoms of `V`. Define the contravariant observation functor

```text
Obs : C^op -> FinSet
Obs(V) = 2^V
Obs(W -> V) = π_(W,V),   π_(W,V)(w) = w restricted to V .
```

The maps satisfy the identity and composition laws because finite function restriction does:
`π_(V,V)=id` and `π_(U,V)=π_(W,V) ∘ π_(U,W)` for `V ⊆ W ⊆ U`. Thus **Obs is a presheaf for this strictly fixed-cut Boolean observational model**. This proves nothing about a presheaf of *actual normative conclusions*, which may fail to restrict naturally.

For a full valuation `w∈2^A`, its coherent observational family is `w|V`. This is the only sense in which the term *section* is used below unless a different presheaf is specified.

### Typed distinction of five layers

```text
ObservedFact      : source-backed, policy-projected observation
NormStatement     : issuer-bound policy assertion (duty, prohibition, privilege, claim, power, immunity)
DerivedJudgment   : result of a stated inference rule at an exact cut
InstitutionalAct  : reserved constitutional approval/rejection/amendment, separately recorded
OperativePermit   : source-owned, current authorization at an actual start/commit boundary
```

A `DerivedJudgment` is never an `OperativePermit` by cast, serialization, proof trace, or name. Hohfeldian claim/duty, privilege/no-right, power/liability and immunity/disability are typed *correlative normative relations among parties*, not platform API scopes. Contrary-to-duty semantics must state an explicit deontic logic; contradictory `O(a)` and `O(¬a)` do not yield a contradiction in every such logic.

A decision at a historical cut is an *explanation of what a model derived*, not a current execution capability. An action's source-verified validity must be re-established at the actual linearization point under AETHER's E3B/Article IX boundaries. This note cannot settle the E3B source-owner race.

## 2. Theorem: when a judgment descends to a narrower context

Let `Ω=2^A`, let `V⊆A`, and let `π_V:Ω→2^V` be restriction. Let `D={permit,deny}` and let `q:Ω→D` be a *fixed-cut, full-information model judgment* for one precisely defined action. The values of `q` do not themselves imply execution authorization.

**Theorem 1 (descent/factorization criterion).** A total restricted judgment `q_V:2^V→D` satisfying `q=q_V∘π_V` exists if and only if:

```text
for all w,w' in Ω:
  π_V(w)=π_V(w')  =>  q(w)=q(w').
```

If it exists, `q_V` is unique because `π_V` is surjective.

**Proof.** If `q=q_V∘π_V`, equal restricted observations have equal images under `q_V`. Conversely, for each restricted observation `v`, choose any `w` with `π_V(w)=v` and set `q_V(v)=q(w)`. Fiber constancy makes this definition independent of the choice; surjectivity makes it total and uniquely determined. □

**Operational interpretation.** For a specified policy decision, *view-completeness* requires constancy on observational-equivalence classes. If two source states look identical to a restricted principal but have different full-information decisions, no truthful, total binary decision procedure using only that principal's observation exists. This is a factorization obstruction, not an ethical truth claim and not automatically a vulnerability in the actual AETHER policy path.

### Countermodel 1: hidden prohibition and closed-world negation

Let `A={blocked}`, `V=∅`. Define `q(w)=deny` iff `blocked∈w`, and `permit` otherwise.

```text
full world w              visible observation π_V(w)      full judgment q(w)
{}                        {}                              permit
{blocked}                 {}                              deny
```

The two worlds share the same visible observation while requiring opposite full judgments. No total `q_V` exists.

If a restricted policy engine evaluates `permit :- not blocked` as negation-as-failure over its reduced input and elevates that judgment to action authority, the apparently narrower context may produce a grant absent in full context. The safe design is not “guess deny” when the requirement is to reproduce the full judgment; it is to **return unknown** for the epistemic judgment and route any executable decision to the separately authorized source. A platform may independently choose fail-closed denial as *an enforcement policy*; this is a different specification and should not claim faithful full-judgment replication.

## 3. Theorem: maximally sound three-valued abstraction

Let `D_?={permit,deny,unknown}`. For observation `v∈2^V` define `F_v={w∈Ω:π_V(w)=v}` and

```text
q̂_V(v) = permit  if q(F_v) = {permit}
         deny    if q(F_v) = {deny}
         unknown otherwise.
```

**Theorem 2 (soundness and maximal precision).** If `q̂_V(v)≠unknown`, then for every `w∈F_v`, `q̂_V(v)=q(w)`. Among all sound `D_?`-valued functions that return a definite answer only if it agrees with every full state compatible with the observation, `q̂_V` is pointwise maximally informative.

**Proof.** The definition supplies unanimity when the answer is definite. If both full decisions occur in a fiber, any sound definite answer would disagree with one, so every sound function must return `unknown`. If a fiber is unanimous, returning its unique value is sound and strictly more informative than `unknown`. □

**Corollary (refinement stability).** If `V⊆W` and `v'=w|W`, `v=v'|V`, then `q̂_V(v)` being definite implies `q̂_W(v')` has the same definite value. Additional genuine information can resolve unknown, but cannot overturn a sound definite conclusion. This holds at the **same** fixed history/policy/rule version, not across a revocation or changing world.

**Noninterference interpretation.** The abstraction is a function of the visible observation alone. Two full worlds with the same observation produce the same `q̂_V`. This is a simple mathematical form of observational noninterference *for the abstract evaluator*. It does not prove noninterference of AETHER HTTP error codes, timings, queries, audit artifacts or current scoped runtime.

**Efficiency.** Naïve enumeration of all compatible worlds is exponential in hidden atoms. This definition is a specification oracle, not a requirement to enumerate worlds in production. A static rule-dependency check or solver may implement the decision condition more economically.

## 4. Positive Horn closure and visibility restriction

Let `R` be a finite positive ground Horn rule system over `A`, and `Cl_R(S)` its least forward-chaining closure from `S⊆A`. Let `V⊆A`. Write `R_V` for rules whose head is in `V`.

**Theorem 3 (dependency-closed slicing; sufficient condition).** If *every rule of R with a head in V has all its body atoms in V*, then for every `S⊆A`,

```text
Cl_R(S) ∩ V  =  Cl_(R_V)(S ∩ V).
```

**Proof.** Define the inflationary immediate consequence operator
`T_R(X)=X∪{h:(B→h)∈R, B⊆X}`. Under the stated condition, `T_R(X)∩V=T_(R_V)(X∩V)`: all rules with a visible head depend only on visible premises. The equality holds at iteration zero and propagates by induction to every iteration. Finite least fixed points stabilize after finitely many iterations; take their visible parts. □

This is a conservative policy/rule dependency criterion, not a necessary condition: redundant rules may violate the syntactic condition without changing the outcome. Stratified negation, non-monotone exceptions, policy-dependent compilation and changing history need distinct proofs. The result gives a *cross-context commutation test* for a small positive fragment, not a new definition of authorization.

### Rule-level diagram

```text
      source state S  ---- Cl_R ----> full derived state
           |                              |
        project V                      project V
           |                              |
           v                              v
     visible source  --- Cl_(R_V) -> visible derived state
```

The diagram commutes only under the specified hypotheses (or some independently demonstrated semantic equivalence). The runtime must not assume the diagram commutes solely because policy scope is narrower.

## 5. Probabilistic local/global coherence

The observation presheaf `Obs(V)=2^V` above glues compatible local **assignments** by ordinary set-function gluing. A different construction, `Dist(V)`, associates each context with probability distributions over assignments and restricts by marginalization. It is a presheaf of sets; it does **not** in general satisfy a sheaf existence axiom.

For binary `x,y,z`, take contexts `{x,y}`, `{y,z}`, `{z,x}`. On each pair place the distribution with probability `1/2` on `(0,1)` and `1/2` on `(1,0)`, zero elsewhere.

Every overlap singleton has the same uniform marginal `(1/2,1/2)`: **the family is compatible on overlaps**. Nevertheless, no joint distribution on `{x,y,z}` can have all three pairwise anticorrelations with probability one, since `x≠y` and `y≠z` imply `x=z`, contrary to `z≠x`. This is a genuine failure of **global distribution extension**, an established contextuality phenomenon, not a novel moral-theory result.

Distinguish it sharply from the simpler statement that each local constraint set is nonempty: nonempty local sets need not give an actually compatible family of assignments. Do not describe either example as cohomology without an explicit complex/coefficient group and a justified comparison theorem. The probabilistic construction is justified by Abramsky and Brandenburger's framework; its direct application to institutional norms requires separate semantics.

## 6. Partial delegation and a law of attenuation

Define a versioned delegation record as

```text
Del = (issuer, subject, target, operation_set, object_set,
       scope, not_before, expires_at, policy_digest, source_id,
       source_epoch, revocation_reference, provenance_id).
```

A purported chain `d1:A→B`, `d2:B→C` is composable only when the intermediate principal matches, domain/target semantics and policy identities are compatible, scope intersection is well typed, and source-issued predicates certify every component as valid at the applicable point. Let `Cap(d)` be the effective capability set, not a bag of token claims.

For independently source-validated consecutive delegations, define the candidate effective set:

```text
Cap(d2∘d1) = Cap(d1) ∩ Scope(d2)
```

with empty effective set if any required validity predicate fails. Then `Cap(d2∘d1)⊆Cap(d1)` immediately. Composition of proof-carrying *descriptions* can concatenate lineage, but a path proof is not an issuer of live authority. Timestamp/epoch/revocation updates may break associative composition of live *validity predicates*; first prove stability for immutable snapshots, and treat source-current admission as an external condition.

This is standard capability attenuation, not an original categorical theorem. A full category would require identity arrows, typed associative composition, and principled treatment of failed/noncomposable arrows; until these are provided, **do not call these delegation records a category**.

## 7. Source-bound AETHER/INTELLECT mapping

| Research construct | Existing source-of-truth boundary | What is **not** established |
|---|---|---|
| Ground fact at a cut | AETHER `SPEC.md` §5–7 datoms, journal `History`, `AsOf` and `Current`; resolver | A fact is not a normative or constitutional act |
| Normative premises | Candidate typed rules and issuer-recorded policy facts; INTELLECT `CONSTITUTION.md` Articles I–V governs actual institutional roles | A policy premise is not moral truth |
| Derived judgment | AETHER rule compiler, stratified rule runtime, scoped execution and derivation provenance | No first-class admitted deontic runtime, and no operative grant from a derived tuple |
| Historical explanation | AETHER provenance trace and execution-scoped proof identity | An opaque trace handle is not permission or independent review |
| Observational context | AETHER effective policy scope and cut; `docs/SEMANTIC_COMPLIANCE_MATRIX.md` | No mathematical proof of platform-wide noninterference or contextual descent |
| Current action admission | AETHER E3B/C0 source authority and default-OFF E3B current-state/atomic-start programme | No activation, no one-shot source-owner permit, no concurrency proof |
| Reserved judgment | INTELLECT Human Steward and phase-gate obligations | NORM-001 cannot approve, amend or override the constitution |
| Federation | Partition-qualified cuts and imported fact provenance; `docs/COMMERCIALIZATION/DISTRIBUTED_TRUTH.md` | No fake global cut, cross-partition consensus or canonical global moral state |

The current single-node controlled-alpha boundary and external qualification limitations remain unchanged. The AETHER rule engine already scopes facts and compilation; *the newly proposed diagnostic must be evaluated against that existing behavior*, not asserted as a newly uncovered vulnerability.

## 8. Comparison with established formalisms and novelty test

| Framework | Established core | Increment proposed here | Novelty judgement |
|---|---|---|---|
| Hohfeld (1913) | Relational legal claims/duties, privileges/no-rights, powers/liabilities, immunities/disabilities | Connect counterparty relations to versioned issuer and evidence records | Conceptual application, not new theory |
| W3C ODRL 2.2 | Permissions, prohibitions, duties, constraints and conflict strategies | Audit whether inferred policy judgments are view-complete under exact visibility restrictions | Integration test, not alternative policy language |
| W3C PROV-O | Agents, activities, entities, derivations and delegations | Source/cut-exact normative proof dependencies | Provenance application |
| NIST ABAC | Subject/object/action/environment attributes evaluated under policy | Two-world indistinguishability and explicit unknown for incomplete context | Existing policy concerns; precise conformance diagnostic |
| Standard capability models | Scope attenuation, expiration/revocation | Separate immutable chain proof from source-current action admission | Standard security principle, context-specific test |
| Datalog/Horn + abstract interpretation | Least fixed-point closure and sound overapproximation | Check visibility/rule-closure commute for approved positive fragments | Known mathematics; practical regression property |
| Abramsky–Brandenburger contextuality | Compatible local distributions can lack global extensions | Examine analogous institutional compatibility only with defined semantics | Not novel, speculative cross-domain use |

**Assessment:** No new theorem of field-wide originality has been established. The precise cross-context **factorization check**, maximal three-valued sound abstraction, and closure/restriction commute condition provide one combined, independently testable AETHER **diagnostic** that may be useful even where per-view results seem correct. However, a well-implemented baseline ABAC/policy analysis system may already provide equivalent protection. Measure that before creating additional research machinery.

**References (published sources, not implementation evidence):**
- Hohfeld, *Some Fundamental Legal Conceptions as Applied in Judicial Reasoning*, Yale Law Journal (1913), DOI: 10.2307/785533; https://dsrpevidence.org/element/799
- W3C, *ODRL Information Model 2.2*, 2018: https://www.w3.org/TR/odrl-model/
- W3C, *PROV-O: The PROV Ontology*, 2013: https://www.w3.org/TR/prov-o/
- NIST, *Guide to Attribute Based Access Control*, SP 800-162 (updated 2019): https://csrc.nist.gov/pubs/sp/800/162/upd2/final
- Abramsky and Brandenburger, *The Sheaf-Theoretic Structure of Non-Locality and Contextuality*, arXiv:1102.0264: https://arxiv.org/abs/1102.0264
- W3C, *Workshop Report: Future of ODRL*, 2026: https://www.w3.org/news/2026/w3c-workshop-report-the-future-of-odrl/

## 9. Reproducibility and evidence grading

An ephemeral Python finite-model script was executed through the authenticated GCL CHAD OS tunnel (not an AETHER test runner or independent attestation). It enumerated:
- all `2^3=8` full Boolean worlds, `2^3=8` observation views, and `2^8=256` binary judgments;
- 6,912 observed-fiber cases for Theorem 1;
- 16,384 soundness evaluations for Theorem 2;
- 55,296 view-refinement comparisons;
- projection identity/composition across comparable visibility subsets;
- dependency-closed positive Horn-rule closure comparisons;
- hidden `blocked` forcing `unknown`; and
- incompatible pairwise anticorrelation with no global binary assignment.

**Result:** all finite assertions passed; exit code `0`.

**CHAD local execution capture:** `7a7b7e891d3145218eed6e5a3bb3c35c`. **Command SHA-256:** `f64894b58677f3336775a90ebd223afeb2faa09095545581edbc8a271cbe1c2b`. **Evidential limitation:** CHAD reports `LOCAL_OS_ONLY_NOT_INDEPENDENTLY_AUTHENTICATED`; code was one-shot and is *not* a checked-in test artifact. These enumeration counts are finite verification aids; the general mathematical theorems stand on their written proofs, not on the enumerator. A separate NORM-001B, if admitted, would need a versioned off-path conformance runner with exact-head tests.

## 10. Assumption ledger and falsification

**A1.** Worlds are Boolean valuations of finite *ground* observations; contested evidence, uncertainty, temporal updates, and complex agent beliefs are outside this model.  
**A2.** Observational context maps are defined only at an identical partition/cut/policy/rule identity with an authorized projection relation. A policy change or principal identity change is *not* an arrow by default.  
**A3.** The full judgment `q` is deterministic for its exact-version rule and historical state. Contextual moral justification is not entailed.  
**A4.** “Sound” means agreement with all complete worlds consistent with the observation; it is epistemic, not an operative authorization grant.  
**A5.** Positive Horn closure theorem excludes negation and changes of policy; all visible heads depend exclusively on visible body atoms.  
**A6.** Delegation composition assumes independently verified source current-validity; no new granting authority is smuggled into composition.  
**A7.** Probability distributions describe an analogy, not a measured ethical population.  
**A8.** External source release claims remain controlled single-node alpha and pending qualifications; documentation is not source evidence.

**Falsifiers and kill conditions:**
1. Find a counterexample to Theorem 1 or 2 within these exact definitions; if found, correct/reject the mathematical note.
2. Show a dependency-closed positive Horn rule set where the projection/closure square fails under the stated semantics; if found, correct/reject Theorem 3.
3. Show the proposed diagnostic yields no new detected class of error, no independently useful explanation, or unjustified overhead compared to baseline typed ABAC/Datalog cross-view comparison; **shelve NORM-001B and the context-topology abstraction**.
4. Show the AETHER/INTELLECT authority model cannot supply a legitimate distinct full-information `q` and fixed-view projection; then the proposed conformance target is ill-posed and must be retired rather than invented.
5. Any attempt to promote `DerivedJudgment` or `q̂_V` to an action permit without source-owned current authorization is an architectural violation, not a successful application.

## 11. Disposition and handoff

**NORM-001A scientific disposition:** `ADVANCE_NARROW_DIAGNOSTIC_ONLY`. The specified descent obstruction and maximally sound partial decision are rigorously well defined and potentially useful as a non-operative consistency test of AETHER scoped semantics. The results do **not** justify a new moral topology engine, global normative completion, automatic ethical adjudication, or a categorical production abstraction; those directions are **SHELVED** pending a genuine mathematical distinction.

**Next permissible item:** one small off-path NORM-001B conformance spike, not a general new platform. Before spending runtime integration effort, evaluate 13 finite fixtures against the ordinary typed baseline and the context-factorization diagnostic. Stop if no incremental diagnostic value appears. Any operative change requires a separate authority review. Exact-head protected admission and review are prerequisites.

**Reviewer posture:** Formalist, Adversary and Referee logical passes must examine this exact-head content as non-authoring/read-only reviews. Same-system passes are not independent human review or mathematical certification.
