# AETHER/FABRIC F1E — Differential Equivalence Evidence

Status: protected implementation — documentary readback candidate
Issue: #94
Protected basis: `7f871359c3513231af111e8949f5d5c55ee4781f`
Protected F1D implementation merge: `ebe535885ae73b7950a2f540a137f2836bdb2941`
Protected F1E source head: `0ded819f4a46e50333e5c54c313c204f8006493a`
Protected F1E merge: `47bec0af7823c0295f55a6b5f6ae3dac832bfeae`

## Objective

F1E closes E3A's differential-equivalence requirement for the admitted live
extraction domain only:

`aether-local-blocking-pool-single-resource-v1`.

The current AETHER reference path remains authoritative. F1E adjudicates
non-operative F1D shadow evidence and either produces evidence that the two
permitted-placement sets are equal with no authorization widening, or fails
closed.

F1E is not a routing decision.

## Admitted live domain

The live claim is intentionally limited to the current single local blocking
resource pool:

`aether-local-blocking-pool`.

Synthetic multi-resource snapshots remain useful contract tests, but they are
rejected by the F1E live-equivalence adjudicator and cannot be reported as live
equivalence evidence.

## Required properties

For each admitted comparison:

1. the resource snapshot contains exactly the protected current local pool;
2. AETHER's reference permitted set is either the singleton current pool or the
   empty set;
3. the FABRIC decision is bound to the exact compared resource-snapshot digest;
4. the FABRIC selected-resource projection agrees with its underlying
   `PlacementDecision`;
5. every FABRIC-selected resource belongs to the AETHER-permitted set;
6. the FABRIC and AETHER permitted sets are exactly equal;
7. the F1D comparison record and underlying FABRIC decision both carry
   `authority_effect = none`.

Any violation is an evidence failure. It is never transformed into a usable
placement result.

## Hostile evidence

The F1E tests include:

- eligible live pool => equal singleton sets;
- unavailable live pool => equal empty sets;
- injected FABRIC authorization widening => rejected;
- injected reference-only mismatch => rejected;
- synthetic multi-resource evidence => rejected as outside the live domain;
- authority-effect laundering => rejected;
- selected-resource projection tamper => rejected;
- snapshot-binding tamper => rejected;
- identical evidence replay => identical differential evidence.

The existing F1A-F1D hostile/replay tests remain part of the E3A evidence base.

## Non-interference

F1E introduces no queue, semaphore, reservation, dispatch, retry, transport,
attempt, `RouteRealized`, semantic, policy, provenance, or institutional-state
mutation.

The adjudicator consumes an already-produced F1D comparison value. It has no
reference to the live blocking executor and cannot alter its state.

## Rollback

F1E owns no semantic or mechanical execution state. Removing the adjudicator
requires no state migration and leaves the AETHER reference execution path
unchanged.

## Successor boundary

Successful F1E completes the bounded F1A-F1E implementation/evidence sequence.
The next step is E3A exact-head adjudication/readback of the complete extraction
evidence.

E3B live FABRIC routing/cutover remains a separate governed decision and is not
authorized by F1E or E3A.


## Protected completion

F1E was reviewed on exact source head
`0ded819f4a46e50333e5c54c313c204f8006493a` and protected by squash merge
`47bec0af7823c0295f55a6b5f6ae3dac832bfeae` on 7 October 2026.

Exact-head audit passes:

- Formalist PASS — review `5439837567`;
- Adversary PASS — review `5439837952`;
- Referee COMPLETE — review `5439838267`.

Exact-head protected checks:

- Required CI gate `112709457489` — success;
- Required Supply Chain gate `112708830868` — success;
- policy / policy `112708740627` — success;
- security / action-policy `112708740206` — success;
- routing-enforcement `112708738952` — success.

No inline review threads remained at merge. Protected `main` read back to the
F1E merge immediately after protection.

## Current boundary

The bounded F1A–F1E implementation/evidence sequence is complete. The next
governed action is E3A exact-head adjudication/readback of the complete
extraction evidence against the acceptance criteria in
`AETHER_FABRIC_E3_F1_RESOURCE_SELECTION.md`.

This F1E completion does not authorize E3B live routing/cutover.
