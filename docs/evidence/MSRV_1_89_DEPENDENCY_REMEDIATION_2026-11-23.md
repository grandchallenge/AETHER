# Rust MSRV 1.89 Alignment and Dependency Security Remediation (`#166`)

- **Tracking issue:** `grandchallenge/AETHER#166`
- **Baseline protected `main`:** `e637b6b3f56a93b8c8bdedde71c9d09cecc5ab97` (issue initially observed at `edd2acfd8533643afe5fe17d2392a72e5598cb50`, regressed by Dependabot PR `#152` `54997e37164293caddff1128801f332f6b4e8a00`)
- **Authority boundary:** Build-contract, MSRV, and supply-chain dependency remediation only. Does **not** alter C3 `#130` protected completion, C4, E3B-A authority, controlled single-node alpha posture, or commercial-beta qualification.

## 1. Exact Reproduction on Protected `main` (`e637b6b3`)

In a fresh isolated clone at `e637b6b3f56a93b8c8bdedde71c9d09cecc5ab97`:

- `cargo +1.86.0 check --workspace --all-targets --locked` → exit `101`:
  - `sha2@0.11.1 requires rustc 1.88`
  - `sysinfo@0.39.6 requires rustc 1.95`
- `cargo check --workspace --all-targets --locked` on `rustc 1.94.0` → exit `101`:
  - `sysinfo@0.39.6 requires rustc 1.95`
- Full `cargo metadata --format-version 1 --locked` audit across all 296 locked crates identified exactly three packages exceeding Rust `1.86`:
  1. `wasip2@1.0.4+wasi-0.2.12` (`rust-version = 1.87.0`, WASI target only via `whoami -> wasite -> wasi`)
  2. `sha2@0.11.1` (`rust-version = 1.88`, direct workspace dependency)
  3. `sysinfo@0.39.6` (`rust-version = 1.95`, direct dependency of `aether_perf` and `aether_api`)
- Live protected-`main` workflows failing at `e637b6b3` on `1.86.0`:
  - `CI` run `45295987730` (`Rust integration (ubuntu-latest, 1.86.0)`, `Rust integration (windows-latest, 1.86.0)`, `Container smoke`)
  - `Supply Chain` run `45295987717` (`package`)
  - `Pages` run `45295987741` (`build`)

## 2. Security Audit and Explicit MSRV Decision (`1.86` → `1.89`)

Running `cargo audit` (`0.22.2`, 1,563 RustSec advisories) against protected `e637b6b3` returned **7 vulnerabilities and 2 warnings**:

| Crate | Baseline | Patched | Advisory / Status | Minimum Rust |
| --- | --- | --- | --- | --- |
| `libsqlite3-sys` (via `rusqlite`) | `0.38.2` (`rusqlite 0.40.2`) | `0.40.0` (`rusqlite 0.43.0`) | `RUSTSEC-2026-0459` (CVE-2026-119375, CVSS 6.8) | `1.89.0` (`rusqlite`) / `1.88.0` (`libsqlite3-sys`) |
| `ctutils` | `0.4.2` | `0.4.8` | `RUSTSEC-2026-0385` (non-constant-time masks) | `1.87` |
| `quinn-proto` | `0.11.18` | `0.11.19` | `RUSTSEC-2026-0523`, `RUSTSEC-2026-0524` | `1.85` |
| `quinn-udp` | `0.5.15` | `0.5.16` | `RUSTSEC-2026-0538` | `1.85` |
| `url` | `2.5.8` | `2.5.9` | `RUSTSEC-2026-0592` | `1.63` |
| `webpki-roots` | `1.0.8` | `1.0.12` | `RUSTSEC-2026-0381` | `1.71` |
| `rustix` | `1.1.4` | `1.1.6` | `RUSTSEC-2026-0486` (unsound warning) | `1.65` |
| `chacha20` | `0.10.1` | `0.10.2` | yanked warning | `1.85` |

**Decision:** Upgrade the repository's supported Rust MSRV explicitly from **`1.86`** to **`1.89`** (rather than keeping `1.86` or jumping to `1.95`):

1. **Why `1.86` is not security-feasible without guard suppression:** Resolving `RUSTSEC-2026-0459` (SQLite WAL uninitialized page-cache disclosure) requires `rusqlite >= 0.41.0` (`rust-version = 1.89.0`), and resolving `RUSTSEC-2026-0385` (`ctutils`) requires `>= 0.4.4` (`rust-version = 1.87`). Staying on `1.86` cannot pass the non-suppressed `Supply Chain` `cargo-audit` gate.
2. **Why `1.89` rather than `1.95`:** `1.89` is the minimal toolchain that resolves every active RustSec vulnerability and warning, retains `sha2 0.11.1` (AArch64 Stacked Borrows fix, MSRV `1.88`), accommodates `wasip2 1.0.4+wasi-0.2.12` (`1.87.0`), and compiles cleanly on both `1.89.0` and `1.94.0` (CHAD default toolchain) without forcing a jump to `1.95` solely for `sysinfo`.
3. **`sysinfo` selection and API diff:** Select `sysinfo = { version = "=0.38.4", default-features = false, features = ["system"] }` (`rust-version = 1.88`, zero RustSec advisories). This retains the `0.38.0` Windows `Motherboard`/`Product` soundness fix, disables unused `user`/`component`/`disk`/`network` features (removing the Unix `users`/`groups` paths updated in `0.39.0`/`0.39.4` and pruning 6 unused transitive crates from `296` to `290`), and requires **zero** `sysinfo` API source changes in `crates/aether_perf`.
4. **MSRV-gated Clippy alignment:** Raising `rust-version` from `1.86` to `1.89` enables `clippy::manual_is_multiple_of` (stabilized in Rust `1.87`); four existing modulo checks in `crates/aether_perf/src/lib.rs` and `crates/aether_api/examples/performance_dashboard.rs` were updated to `.is_multiple_of(...)`.

## 3. Lockstep Surface Alignment

Every declared Rust MSRV, build container, workflow, test, and documentation claim was updated together:

- `Cargo.toml`: `rust-version = "1.89"`
- `crates/{aether_api,aether_perf,aether_service_core,aether_sidecar,aether_storage}/Cargo.toml`: `rusqlite = "=0.43.0"`, `sysinfo = { version = "=0.38.4", default-features = false, features = ["system"] }`
- `Cargo.lock`: updated to 290 crates; `0` crates with `rust-version > 1.89.0`; `0` disallowed/unknown licenses; `cargo audit` exits `0` (`0` vulnerabilities, `0` warnings)
- `Dockerfile` and `fixtures/release/allowed-actions.json`: `rust:1.89-bookworm@sha256:948f9b08a66e7fe01b03a98ef1c7568292e07ec2e4fe90d88c07bb14563c84ff`
- `.github/workflows/ci.yml`: `rust-integration` matrix `["1.89.0", stable]` plus PR-time `cargo +1.89.0 check --workspace --all-targets --locked` regression guard in `rust-pr`
- `.github/workflows/{pages,pilot-extended-operability,supply-chain}.yml`: `rustup toolchain install 1.89.0` / `rustup default 1.89.0`
- `python/tests/test_pages_identity.py`: expects `1.89.0`
- `python/tests/test_supply_chain.py`: added `test_declared_rust_msrv_is_consistent_across_build_and_release_surfaces` to fail closed on future cross-surface MSRV drift
- `README.md` and `docs/SUPPLY_CHAIN_SECURITY.md`: updated badge and security baseline to `Rust 1.89+`

## 4. Verification Summary

- `cargo audit` (`0.22.2`): **0 vulnerabilities, 0 warnings** (290 crate dependencies)
- `cargo metadata --format-version 1 --locked` audit: **0 packages > 1.89.0**, **0 license policy violations**
- Rust `1.89.0` (declared MSRV): `cargo +1.89.0 fmt --all --check`, `cargo +1.89.0 clippy --workspace --all-targets --locked -- -D warnings`, `cargo +1.89.0 test --workspace --locked` (**53 suites, 367 passed, 0 failed, 11 ignored**)
- Rust `1.94.0` (local default stable): `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` (**53 suites, 367 passed, 0 failed, 11 ignored**)
- Go (`go test ./...`): **passed** (`aetherctl`, `client`, `tui`)
- Python (`python -m pytest python/tests -q`, including `test_supply_chain`, `test_pages_identity`, `test_release_readiness_contract`): verified on clean committed tree
- External pinned GH-OS routing gate (`ghos_execution_routing_gate.py`, SHA-256 `fc0a9a4d20de72e9fbc04c8cd54cffc3a6e4657fb09e7978b360616bd5e94a17`): **passed**
- Hosted PR and post-merge `main` outcomes (`CI`, `Supply Chain`, `Pages`) remain governed by their exact GitHub Actions run records and are not asserted by local checks alone.
