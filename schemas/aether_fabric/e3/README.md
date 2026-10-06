# AETHER/FABRIC E3/F1 candidate schemas

Status: F1A candidate, non-runtime
Protocol extension: `aether-fabric/1.1`
Protected basis: AETHER `adc19231a5e98112e712d370f913e87d2299af0b`

These schemas add placement evidence without changing the accepted E1
`aether-fabric/1.0` envelope bytes or schemas.

Records:

- `PlacementConstraintSet` — upstream-governed mechanical narrowing of an exact
  E1 envelope. It supplies resource requirements, decision time, snapshot
  freshness and the exact control-state witness binding.
- `ControlStateWitness` — upstream evidence of the exact envelope control state
  used for the decision. The witness is non-authoritative and must be observed at
  the exact `decision_time_unix_ms`, preventing a stale pre-revocation witness
  from being replayed as current state.
- `ResourceSnapshot` — canonicalized mechanical resource observation.
- `PlacementSelected | PlacementUnavailable` — non-operative shadow evidence;
  neither record creates a mechanical attempt or execution authority.

## Canonicalization

Rust implementation: `crates/aether_fabric`.

Canonicalization profile `gcl-cjson-set-v1`:

1. UTF-8 JSON only.
2. Duplicate object keys are rejected before typed decoding.
3. Object keys are emitted deterministically by the typed JSON representation.
4. `resources[]` is sorted by stable `resource_id`.
5. Semantically set-valued arrays are sorted before hashing.
6. Non-finite numbers are forbidden.
7. Selection timestamps and capacity units use integer units.
8. Snapshot digest is SHA-256 over the canonical snapshot with
   `snapshot_digest=""`; the populated digest is then carried in the record.
9. Decision-input digest is a length-prefixed, label-separated SHA-256 over the
   exact E1 envelope, E3 constraint, canonical snapshot, scheduler policy,
   selector implementation identity and control-state witness.

## Live baseline

F1A models the current AETHER `BoundedBlockingExecutor` as one stable resource
pool: `aether-local-blocking-pool`. Individual semaphore permits are not
assigned fabricated worker identities.

The `aether_http::HttpKernelState::fabric_reference_pool_snapshot` adapter is
read-only: it observes configured limits and available permits and does not
acquire a permit, enqueue work, create a `mechanical_attempt_id`, or mutate
semantic state. `fabric_reference_pool_admissible` composes that observation
with the pure F1A narrowing/freshness/control-state predicate to answer whether
the modeled current pool is admissible for the exact governed input. It also
has no scheduling side effect.

## Non-authority

These are candidate contract/evidence records. They do not activate FABRIC,
route live work, admit queue entries, dispatch payloads, create semantic state,
or authorize E3B cutover.


## F1B policy artifact

`f1b_selector_policy.json` is the exact non-operative F1B ranking artifact. The
Rust selector binds its SHA-256 digest through `f1b_scheduler_policy_identity()`.
After F1A admissibility filtering, ranking is lower queue depth, then greater
available admission capacity, then stable resource ID. No policy fallback or
implicit substitution is allowed.
