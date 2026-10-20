# Alpha dependency advisory repair

- Baseline: protected `d0e6f4a30147d48d9b266a48f6843238e9a20720` (PR #153).
- Disposition: superseded before qualification; no release verdict or operator
  completion asserted. The expired Go license exception was repaired there.
- Trigger: fresh local cargo-audit advisory-database scan on 2026-10-20 found
  nine Rust vulnerabilities that the earlier license failure had masked.

## Bounded repair

| Package | Previous | Selected | Advisory coverage |
| --- | --- | --- | --- |
| quinn-proto | 0.11.16 | 0.11.18 | RUSTSEC-2026-0338, 0339, 0340, 0341, 0342 |
| rustls | 0.23.41 | 0.23.46 | RUSTSEC-2026-0285, 0346, 0348, 0349 |
| rustls-webpki | 0.103.13 | 0.103.16 | Dependency selected by patched rustls |

These are compatible lockfile updates within existing dependency ranges. No
workspace API major versions, Rust semantic implementation, policy, schema or
FABRIC authority are changed. This does not adopt the unrelated major-version
updates proposed by Dependabot #152, whose rand update fails compilation.

The repaired local audit returns zero vulnerabilities. Its yanked-version
warning for locked chacha20 0.10.1 remains visible; no audit suppression or
exception was added. Rust 1.86 compatibility, tests and hosted exact-candidate
Supply Chain outcomes must be established before package qualification.

Supply Chain `39417942453` and Capacity `39417947630` for the superseded
candidate were explicitly canceled to avoid spending qualification resources
on a known vulnerable package. This cancellation is not a product pass.

## Acceptance evidence already obtained

The pre-repair source `807d539270a549092a24227f2b09a3deab80e2eb` passed the
local final workspace gate (361 tests, 11 ignored), 177 Python tests, Go tests,
and its 38-component Go license BOM. An extracted Windows package passed all
six frozen cases with an empty PATH and an unrelated working directory,
reproduced results after process restart and stopped restore, and verified
both printed original proof handles after restore. A separate extracted-service
rehearsal passed twelve schema-bound journal, idempotency, Current/AsOf and
proof-replay/recovery checks using the packaged backup/restore helpers.

Those observations are local engineering evidence for the earlier exact
source/package. They are not transferred as qualification of the repaired
candidate and contain zero real-human task observations. Prior operator stops
and partial sessions remain intact. The milestone in `docs/ALPHA_MILESTONE.md`
remains open and retains controlled single-node alpha.
