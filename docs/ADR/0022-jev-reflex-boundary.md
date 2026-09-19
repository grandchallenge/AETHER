# ADR 0022 — Bounded probabilistic reflex boundary

Status: Proposed for WP00 review
Date: 2026-09-19
Issue: #89

## Context

AETHER already owns exact semantic state, provenance, replay, policy scope, and
authority-bearing coordination. A System-One/Jev-class model can cheaply answer
closed questions over projected state, but its probability distribution is not
an authority source.

The architectural risk is authority laundering: treating a high-confidence
model result as if it were permission, truth, or an admitted semantic
transition.

## Decision

Add a provider-neutral Rust crate, `aether_reflex`, with this path:

```text
exact AETHER cut
  -> governed projection
  -> versioned closed decision schema
  -> probabilistic provider result
  -> deterministic reflex gate
  -> act | deliberate | escalate
```

The first intended provider adapter is Jev. WP00 does not implement that adapter.

### Invariant 1: confidence is not authority

```text
confidence != authority
```

The gate may return `act` only when all of the following hold:

1. the provider distribution is structurally valid and closed-world;
2. the selected probability meets the policy threshold;
3. the top-two margin meets the policy threshold;
4. an explicit, separately supplied authority record authorizes the selected
   choice;
5. that authority record is digest-bound to the same exact semantic cut.

A high-confidence result with no grant, or with a grant for a different choice,
returns `escalate`.

### Invariant 2: every reflex is replay-bound

A decision request names an exact federated cut. Any partition cut with
unqualified `Current` is rejected. The projection also binds content and projection-policy digests, while the
versioned decision schema and reflex gate policy each carry their own content
digests.

This lets a receipt answer "what state was judged?" without re-running against a
later world.

### Invariant 3: closed-world means schema-closed, not semantically infallible

The provider must return exactly one finite, non-negative probability for every
choice in the declared question schema and no unknown choices. Probability mass
must normalize to one within a declared tolerance.

A valid distribution can still be wrong. Schema closure prevents malformed
outputs; it does not prove semantic correctness.

### Invariant 4: receipts are evidence

A decision receipt records:

- exact projection identity;
- question/schema identity;
- provider/model/revision;
- complete distribution;
- complete digest-bound deterministic gate policy;
- selected branch and confidence/margin evidence;
- optional authority reference, digest, and exact cut;
- `authority_effect: none`.

The receipt cannot, by its type, claim that it granted authority.

## Consequences

Benefits:

- routine semantic discrimination becomes separable from deliberation;
- AETHER can later support cheap routing, relevance, contradiction, novelty,
  evidence and escalation reflexes;
- replay can distinguish historical decisions from present-day re-evaluation;
- confidence thresholds can later be calibrated against recorded outcomes;
- provider replacement does not change kernel authority law.

Costs:

- every reflex requires a maintained decision ontology;
- exact projection construction must be governed;
- later service integration needs explicit admission and receipt persistence;
- empirical calibration remains a separate evidence programme.

## Rejected alternatives

- let Jev append authoritative facts directly;
- treat confidence above a threshold as permission;
- allow `Current` as a replay identity;
- accept open-text provider outputs and parse them after the fact;
- put Jev transport/client code into the semantic kernel;
- activate AETHER-POL or AETHER-Learn as part of WP00.

## Non-authority

This ADR does not change Article IX authority, service execution, HTTP,
authentication, deployment, claim promotion, AETHER-POL, AETHER-Learn, or the
controlled single-node alpha claim.

A Jev API adapter and any automatic journal admission require a separate
governed tranche.
