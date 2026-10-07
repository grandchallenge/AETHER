# AETHER/FABRIC E3B-A0 — Activation Preflight Packet

Status: BLOCKED_PRE_ACTIVATION
Issue: #110
Protected E3B-R source head: `209ec0cbfa49af8bb3827650f2c1f2b7640253d1`
Protected E3B-R merge: `04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`
Current constitutional baseline: INTELLECT Constitution 1.2.0 / Article IX

## Purpose

This packet is the exact preflight boundary between protected E3B runtime
readiness and any later live FABRIC routing activation.

It does not authorize activation.

## Protected readiness result

E3B-R proves that, given authentic upstream-governed E1/E3 records, the
placement-to-`RouteRealized` bridge can:

- re-check current control state and current resource admissibility;
- reject late revocation, supersession, expiry and stale resource evidence;
- create `mechanical_attempt_id` only at realization;
- replay an identical realization request idempotently;
- distinguish a retry request;
- emit E1-shaped mechanical evidence with `authority_effect = none`;
- leave the current HTTP execution path unchanged by default.

The protected runtime exposes no `live_fabric` routing mode.

## Activation blockers

### A0-B1 — upstream authority-record issuance

The ordinary protected HTTP path currently does not receive runtime instances
of:

- `MechanicalEnvelopeAuthorized`;
- `PlacementConstraintSet`;
- a fresh governed `ControlStateWitness`;
- `PlacementSelected`.

FABRIC is not authorized to synthesize these records from successful
authentication, namespace admission, local capacity, liveness or scheduler
state.

Before an activation candidate can exist, a separately governed AETHER/upstream
producer must define who issues those records, from which admitted authority
facts, with what integrity binding, revocation source, lifetime and replay
identity.

Status: **MISSING / BLOCKING**.

### A0-B2 — exact live-routing and rollback delta

No exact code/configuration delta currently routes ordinary production HTTP work
through the protected FABRIC realization bridge.

A future activation candidate must identify:

- the exact reference-path call site replaced or wrapped;
- the exact source of all E1/E3 inputs;
- the exact queue-admission boundary after `RouteRealized`;
- the exact rollback/kill-switch delta;
- the exact canary scope and failure behavior;
- proof that rollback requires no semantic-state migration.

Status: **MISSING / BLOCKING**.

### A0-B3 — reserved activation disposition

Production/control-plane activation is a reserved authority event under
AETHER's declared governance. Repository merge success, CI, shadow evidence or
this packet cannot create that authority.

Required disposition: **UNSET**.

No automation may populate it on behalf of the Human Steward or any other
reserved authority.

### A0-B4 — constitutional conditional

The current E3B design preserves AETHER semantic authority and therefore does
not, by readiness evidence alone, require an Article XI amendment.

If the exact future live-routing candidate changes the effective Article IX
authority boundary, AETHER semantic authority, constitutional wording or
effective authority schedule, a completed effective Article XI amendment and
Human Steward approval become prerequisites before activation.

Status: **CONDITIONAL / NOT TRIGGERED BY E3B-R**.

## Required activation packet

A future E3B-A activation packet is admissible only when it binds all of:

1. exact protected E3B-R merge;
2. protected upstream authority-record producer;
3. exact live-routing delta;
4. exact rollback delta;
5. exact canary/deployment target;
6. exact control-record integrity/authentication profile;
7. fresh Formalist review;
8. fresh Adversary review;
9. fresh Referee completion;
10. required protected checks;
11. zero unresolved review threads;
12. applicable constitutional amendment evidence if A0-B4 is triggered;
13. authentic reserved production/control-plane disposition.

Any head, config, deployment target or control-record producer change
invalidates earlier activation review evidence.

## Current disposition

```text
E3B-R: PROTECTED COMPLETE
E3B-A: BLOCKED
reserved_activation_disposition: UNSET
live_fabric_routing: NOT ACTIVE
```

## Named boundary

**E3B-A RESERVED LIVE ACTIVATION BOUNDARY**

The next substantive governance problem is A0-B1: define and protect the
upstream AETHER authority-record issuance contract. That contract is not
authorized to be invented inside FABRIC.
