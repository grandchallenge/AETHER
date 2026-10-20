# ADR 0030: Licensed console dependency and bounded alpha acceptance

- Date: 2026-10-20
- Status: Accepted implementation decision; protected admission pending

## Context

Supply Chain run `38213923644` on `c7ade937` failed because the temporary
unknown-license exception for `go-localereader` v0.0.1 expired. That release
archive has no license file. Its upstream README declares MIT, and upstream
subsequently merged the MIT LICENSE without changing implementation files.

## Decision

Pin `github.com/mattn/go-localereader` to the exact upstream merge
`2491eb6c1c75720122ef321ed7acc3a8d9de95b1`, resolved by Go as
`v0.0.2-0.20220822084749-2491eb6c1c75`. Remove the unknown-component exception.
Retain `allow_unknown: false`, the SPDX allowlist, strict SBOM generation,
vulnerability gates, and all required checks. Check the actual downloaded
module license through the existing SBOM producer.

This dependency is confined to the Go console. No Rust semantics, policy,
execution authority, journal format or FABRIC routing changes are admitted.
The personal-console changes in the original checkout remain separate.
The resulting package targets the acceptance contract in
`docs/ALPHA_MILESTONE.md`; qualification does not promote the alpha claim.

## Evidence

- Upstream v0.0.1 is `6338b4c69507fb1f9edba3db33882a8e9ab9bfa8`.
- [Upstream comparison](https://github.com/mattn/go-localereader/compare/v0.0.1...2491eb6c1c75720122ef321ed7acc3a8d9de95b1)
  adds only LICENSE; implementation files are unchanged.
- [Pinned LICENSE](https://github.com/mattn/go-localereader/blob/2491eb6c1c75720122ef321ed7acc3a8d9de95b1/LICENSE)
  contains the MIT permission grant.

## Consequences

The build uses a Go pseudo-version because upstream has not tagged its license
addition. Go module checksums preserve the selected archive identity. No
exception expiry renewal is needed; unknown licenses continue to fail closed.
Hosted checks and independent exact-candidate qualification remain required.

## Post-merge advisory follow-through

PR #153 protected this license repair at `d0e6f4a`. A fresh scan then found nine inherited Rust dependency advisories. The separate compatible-lockfile repair is recorded in `docs/evidence/ALPHA_ADVISORY_REPAIR_2026-10-20.md`; it does not renew a license exception or change kernel architecture. Qualification remains required for the successor.
