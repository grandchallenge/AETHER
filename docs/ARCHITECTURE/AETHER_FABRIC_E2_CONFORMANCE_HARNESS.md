# AETHER/FABRIC E2 Executable Conformance Harness

Status: Protected E2 evidence — merge `fce5095cfd0db8766ae0c7d5e80c091b41284862`
Issue: #92
E1 protected basis: `a5198dc1acb9b31508773ff1617127338677e0a5`
Protocol: `aether-fabric/1.0`

## Purpose

E2 turns the E1 interface law into executable evidence without moving any
runtime responsibility. The reference implementation is Python because
`AGENTS.md` assigns Python the SDK/harness layer while Rust remains the
authoritative semantic kernel.

## Evidence surfaces

The harness executes three independent evidence classes:

1. JSON-Schema meta-validation for all six E1 schemas using Draft 2020-12.
2. E1 fixture replay: all four declared valid fixtures must validate and the
   half-bound semantic-start fixture must fail.
3. Deterministic abstract trace validation for the D1-D5 hostile boundaries.

`schemas/aether_fabric/e2/vector_registry.json` registers every E1 vector
V01-V51 and maps it to one or more executable evidence families. Registry
completeness is itself tested.

## D1-D5 meaning

- D1 checks exact mechanical-attempt identity and the
  `PreStartRejected xor SemanticExecutionStarted` boundary.
- D2 checks influence without authority: telemetry, delivery, GHOS reachability,
  and mechanical observations cannot directly create governed semantic or
  institutional effects.
- D3 checks that replica movement cannot change promotion/leader authority.
- D4 checks that physical locality/cache observations cannot become semantic
  relevance, visibility, or admission.
- D5 checks lifecycle fail-closed behavior around revocation, retries, and
  started-versus-pre-start state.

Additional executable families cover envelope lineage, typed identity and
version/schema/integrity conditions.

## Deliberate abstraction

The trace harness consumes small dictionaries shaped like E1 events. It is not
a transport, scheduler, storage layer, controller, or FABRIC runtime. This is
intentional: E2 tests the contract before any implementation is extracted.

Passing E2 establishes that the candidate contract and its hostile examples are
machine-checkable and internally consistent at this reference boundary. It does
not prove performance, distributed liveness, final cryptographic attestation,
or equivalence of a future extracted implementation.

## Non-authority

E2 does not activate FABRIC, move Rust/Go/Python runtime responsibilities,
modify Article IX or AETHER semantic authority, activate POL/AETHER-Learn,
change GHOS controller admission, deploy production state, or promote a claim.

A later extraction tranche must bind itself to exact protected E2 evidence and
satisfy its own governance gate.
