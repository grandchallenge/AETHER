# ADR 0031: Portable frozen operator surface

- Date: 2026-10-20
- Status: Accepted implementation decision; protected admission pending

## Context

The real-operator executable previously required Cargo, Git and source-relative
fixture files. Operator feedback also identified insufficient context and
machine-oriented presentation. Reproducible alpha acceptance requires the
actual package to run its six-case workflow without development tools.

## Decision

Include the existing Rust operator example in the Windows pilot package.
Embed the frozen case fixture and compile its product SHA through the scoped
`AETHER_OPERATOR_BUILD_REVISION` build variable. Its identity command exposes
that SHA and the embedded fixture digest. Package assembly rejects stale or
unbound binaries; the facilitator records the executable digest and rejects
revision/fixture mismatches. The verified package digest remains the custody
anchor; self-reported executable metadata alone is not qualification.

Keep the cases, source facts, comparator, questions, counterbalanced schedule
and scoring unchanged. Add the same general orientation to both conditions,
plain labels for decision inputs, and persisted-handle proof inspection.
The runtime never deserializes ground truth into its operator case type;
automated tests alone compare all six result sets with the frozen answers.
The default workspace test suite includes those checks and original-handle
replay after SQLite restart. Canonical pilot files use LF on Windows so
byte-bound preflight has a reproducible checkout.

## Boundaries

This is presentation, packaging and acceptance work over the Rust kernel.
Frozen case snapshots are not journal-driven live support state. Journal
Current/AsOf, schema admission and stopped backup/restore retain their separate
package qualification requirements. No semantic, schema, authorization,
FABRIC, trial-data, scoring or external-action authority is changed. A real
participant and a protected exact candidate are still required for a new
operator session; automated rehearsal cannot replace human observations.
