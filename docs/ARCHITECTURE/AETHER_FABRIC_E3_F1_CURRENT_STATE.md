# AETHER/FABRIC E3/F1 — Current Protected State

Status: E3A protected complete; E3B separately governed
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
- E3A exact-head adjudication: `23353cf01e5c8cefef18eedc8b878c2290fb50f0`
- E3B-R live-control readiness: `04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`

F1D exact reviewed source head: `b19d3a736696dcfdd3737848d7b1654136410cca`.
F1E exact reviewed source head: `0ded819f4a46e50333e5c54c313c204f8006493a`.
E3A exact reviewed source head: `23200374e34053f686b9da0af848a7a56a3fde5a`.
E3A completion receipt: issue #94 comment `6035347356`.

## Current frontier

F1A-F1E and E3A are complete and protected. `AETHER_FABRIC_E3A_ADJUDICATION.md` is the authoritative completion record. The bounded E3/F1 implementation-equivalence lane is closed.

F1E proved exact permitted-set equality and no authorization widening over the
admitted one-resource live extraction domain. Synthetic multi-resource fixtures
remain contract/hostile evidence only and are not live-equivalence evidence.

## Authority boundary

Neither F1D nor F1E transfers routing authority. The AETHER reference path remains authoritative. F1D comparison evidence and F1E differential-equivalence evidence have `authority_effect = none` and do not acquire permits, mutate queues, reserve or dispatch work, create attempts, create `RouteRealized`, start or complete semantic attempts, or admit results.

E3B-R readiness is protected at `04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`. C0-A through C0-F are protected at `4eb776748b3c70829bd8dca0c1bc48cb6210776f` from exact reviewed source head `3a7558311e92616c1200fbcc501e188ed495d192`. C1 is protected complete at merge `2e959418f8a27dfb72beb8502e2b3ecdf82c6f65` from exact reviewed head `97c0878d067fcd7cdde978c2d3967c08ead5d006`; the implementation code commit is `8df42c6cd3afa09df8f99c2b0a55a97d11aa5423`. C1 has no ordinary HTTP integration or live-routing authority. C2 real-HTTP shadow integration is the next eligible tranche but has not started. Live routing/cutover remains separately governed and is not authorized by F1A-F1E, E3A, E3B-R, C0, or C1. The activation preflight is `AETHER_FABRIC_E3B_A0_ACTIVATION_PREFLIGHT.md`.

## Documentary supersession

Any older line describing F1C as active, F1D/F1E as pending, or E3A as pending is superseded by this ledger together with `AETHER_FABRIC_E3A_ADJUDICATION.md`. This includes stale phase/status lines in `docs/STATUS.md`, `docs/ROADMAP.md`, ADR 0025, and the original E3/F1 specification header; their normative design clauses remain unchanged. A later routine whole-document refresh may remove those stale phrases without changing programme state.
