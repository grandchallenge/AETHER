# AETHER/Jev Reflex Architecture

Status: WP00 candidate
Issue: #89

## Purpose

Define the narrow boundary at which a fast closed-world probabilistic model can
assist AETHER without becoming semantic or institutional authority.

Jev is the first intended provider. The kernel contract is provider-neutral.

## Semantic path

```text
              authoritative
              AETHER state
                   |
                   | exact cut
                   v
          +------------------+
          | State projection |
          +------------------+
                   |
                   | digest-bound input
                   v
          +------------------+
          | Reflex provider  |
          |   (Jev later)    |
          +------------------+
                   |
                   | closed distribution
                   v
          +------------------+
          | Deterministic    |
          | reflex gate      |
          +------------------+
             /      |       \
            /       |        \
          act   deliberate  escalate
            \       |        /
             \      |       /
              +-------------+
              | caller owns |
              | transition  |
              +-------------+
                   |
                   | ordinary AETHER admission
                   v
              new state/cut
```

The provider does not own the last arrow. Neither does `GateOutcome::Act`:
`act` means the reflex path is eligible under supplied evidence, not that the
crate has granted permission or executed a capability.

## Objects

### Decision schema

A versioned schema supplies the complete answer vocabulary for one question.
The schema is part of the semantic contract. An answer outside that vocabulary
is invalid rather than "hallucinated into" a new action.

### State projection reference

A projection binds:

- projection identity;
- one or more exact AETHER partition cuts;
- content digest;
- policy digest;
- a content-digested decision schema.

The reflex therefore judges a reproducible projection, not ambient mutable
state.

### Probabilistic decision

The provider returns its identity and one probability for every declared
choice. The vector must be finite, non-negative and normalized.

### Reflex policy

The deterministic policy is content-digested and currently carries two scalar
gates:

- minimum selected probability;
- minimum top-two margin.

These are routing parameters. The full policy is retained in the receipt so the
historical gate is inspectable without relying on a mutable name. They are not
claims that a given probability is well calibrated.

### Authority grant

Authority is separate input. It names a grant, its digest and exact AETHER cut,
principal, capability, and the choices that grant permits. The authority cut
must equal the projected decision cut. WP00 validates the supplied evidence
shape and binding but does not attest that the grant exists; the future
integration layer must resolve it through ordinary AETHER authority/admission.

The gate ordering is intentional:

1. invalid data fails closed;
2. insufficient confidence/margin deliberates;
3. stale/different-cut or out-of-schema authority fails closed;
4. sufficient confidence without matching authority escalates;
5. only sufficient confidence plus matching authority may return `act`.

### Decision receipt

Receipts are replay evidence and carry `authority_effect: none`.

A caller may later submit a receipt through ordinary governed AETHER admission.
WP00 does not perform that submission automatically.

## Why the distinction matters

AETHER answers "what is semantically admitted and why?"

A reflex provider answers "given this projected state and closed question, what
does the evidence look like?"

The gate answers "is this reflex sharp enough for the configured path?"

An authority record answers "may this actor perform this choice?"

Collapsing those four questions would make a probability score silently become
law.

## Deferred integration

WP00 intentionally excludes:

- Jev transport or credentials;
- HTTP/service endpoints;
- automatic receipt persistence;
- online calibration;
- learned threshold updates;
- memory retrieval integration;
- router integration;
- AETHER-POL/AETHER-Learn activation;
- autonomous action execution.

Those are later work packages after the current remediation programme permits
feature widening on the relevant surfaces.
