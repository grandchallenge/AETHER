# AETHER/FABRIC E3/F1 — Current Protected State

Status: protected-state ledger candidate
Issue: #94
As of: 2026-10-06

This file is the bounded exact-state reconciliation for the E3/F1 extraction lane after F1D. It supersedes earlier F1C/F1D phase labels in `docs/STATUS.md` and `docs/ROADMAP.md` where those labels have not yet been mechanically refreshed. Those stale labels are documentary only and are not authority to repeat or reopen completed work.

## Protected sequence

- E2 evidence basis: `fce5095cfd0db8766ae0c7d5e80c091b41284862`
- E3/F1 specification: `1ccce8756f7e79fe16753f2c8b3da182addc94c5`
- F1A contract/reference adapter: `24589252ca5584c1ac4385f8fd57b9c680eebbf3`
- F1B deterministic selector: `04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3`
- F1C hostile/replay evidence: `c21aebe269a83fe729c226189f6643de7f6fc1da`
- F1D non-operative shadow integration: `ebe535885ae73b7950a2f540a137f2836bdb2941`

F1D exact reviewed source head: `b19d3a736696dcfdd3737848d7b1654136410cca`.

## Current frontier

F1E is next: differential-equivalence evidence over the admitted one-resource live extraction domain, including hostile mismatch/no-widening evidence.

F1E must not treat synthetic multi-resource fixtures as live equivalence evidence. It must compare the current AETHER reference outcome and FABRIC shadow outcome under the same exact modeled input and prove that no permitted-set widening is introduced.

## Authority boundary

F1D does not transfer routing authority. The AETHER reference path remains authoritative. F1D comparison evidence has `authority_effect = none` and does not acquire permits, mutate queues, reserve or dispatch work, create attempts, create `RouteRealized`, start or complete semantic attempts, or admit results.

E3B live routing/cutover remains separately governed and is not authorized by #94/F1D.

## Documentary supersession

Any older line describing F1C as an active candidate or F1D as pending is superseded by this ledger together with ADR 0025 and `AETHER_FABRIC_F1D_SHADOW_INTEGRATION.md`. A later routine whole-document refresh may remove those stale phrases without changing programme state.
