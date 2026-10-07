# AETHER/FABRIC E3/F1 — Current Protected State

Status: F1E protected; E3A adjudication pending
Issue: #94
As of: 2026-10-07

This file is the bounded exact-state reconciliation for the E3/F1 extraction lane after protected F1E. It supersedes earlier F1C/F1D/F1E phase labels in `docs/STATUS.md` and `docs/ROADMAP.md` where those labels have not yet been mechanically refreshed. Those stale labels are documentary only and are not authority to repeat or reopen completed work.

## Protected sequence

- E2 evidence basis: `fce5095cfd0db8766ae0c7d5e80c091b41284862`
- E3/F1 specification: `1ccce8756f7e79fe16753f2c8b3da182addc94c5`
- F1A contract/reference adapter: `24589252ca5584c1ac4385f8fd57b9c680eebbf3`
- F1B deterministic selector: `04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3`
- F1C hostile/replay evidence: `c21aebe269a83fe729c226189f6643de7f6fc1da`
- F1D non-operative shadow integration: `ebe535885ae73b7950a2f540a137f2836bdb2941`
- F1E differential-equivalence evidence: `47bec0af7823c0295f55a6b5f6ae3dac832bfeae`

F1D exact reviewed source head: `b19d3a736696dcfdd3737848d7b1654136410cca`.
F1E exact reviewed source head: `0ded819f4a46e50333e5c54c313c204f8006493a`.

## Current frontier

F1A-F1E are complete and protected. E3A is next: exact-head adjudication/readback
of the complete resource-selection extraction evidence against the protected E3/F1
acceptance criteria.

F1E proved exact permitted-set equality and no authorization widening over the
admitted one-resource live extraction domain. Synthetic multi-resource fixtures
remain contract/hostile evidence only and are not live-equivalence evidence.

## Authority boundary

Neither F1D nor F1E transfers routing authority. The AETHER reference path remains authoritative. F1D comparison evidence and F1E differential-equivalence evidence have `authority_effect = none` and do not acquire permits, mutate queues, reserve or dispatch work, create attempts, create `RouteRealized`, start or complete semantic attempts, or admit results.

E3B live routing/cutover remains separately governed and is not authorized by
F1A-F1E or by completion of E3A.

## Documentary supersession

Any older line describing F1C as active, F1D as pending, or F1E as pending is superseded by this ledger together with ADR 0025, `AETHER_FABRIC_F1D_SHADOW_INTEGRATION.md`, and `AETHER_FABRIC_F1E_DIFFERENTIAL_EQUIVALENCE.md`. A later routine whole-document refresh may remove those stale phrases without changing programme state.
