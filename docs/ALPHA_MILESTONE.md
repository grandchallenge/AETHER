# Reproducible operator alpha

## Acceptance contract

Deliver one exactly identified single-node SQLite package that a real operator
can use to complete the existing Support Resolution Desk workflow, and whose
decisions and execution-scoped proofs reproduce after restart and quiesced
backup/restore. Rust remains the semantic authority and the DSL remains the
rule surface. The release boundary remains controlled single-node alpha, one
visibility/authority domain and trusted appenders. Live FABRIC activation,
commercial beta, GA and external actions are outside this milestone.

## Baseline and local work

The implementation baseline is protected main
`c7ade9370da97a7aade4e4fdd893bcc7991c43b7`, reverified on 2026-10-20.
The original `F:\_codex\AETHER` checkout and its two divergent commits and
uncommitted personal-console work are preserved. This tranche uses a separate
worktree and `codex/alpha-acceptance`; it does not import the personal profile.

## Delivery sequence

| Packet | Required result | Current disposition |
| --- | --- | --- |
| A: baseline and release health | Preserve local work; remove expired license exception through an evidenced dependency repair; required exact-head checks pass | Baseline established; licensed upstream revision selected; hosted checks pending |
| B: usable operator workflow | Context and readable answers to the five frozen questions; all six cases execute correctly; proof remains inspectable | Portable runner, readable inputs and six-case correctness/restart test implemented; extracted-package rehearsal pending |
| C: package qualification | Clean candidate/package identity; clean install; schema-bound writes, idempotency, recursion, Current/AsOf, proof, restart and stopped backup/restore; mandatory hosted gates and independent bundle verification | Pending A/B and protected candidate |
| D: real operator | Separately identified session, briefing and existing counterbalanced 12-observation schedule; valid evidence and six correct AETHER tasks | Pending C and a participating human; prior stopped/partial records remain intact |
| E: closeout | Acceptance record linking package, qualification, recovery, operator evidence and residual defects; protected review/readback | Pending C/D |

Record engineering rehearsals separately from real observations. Never fill in
human answers, timings or usefulness ratings from an automated run. A stopped
or materially defective session leaves the milestone incomplete and records
the defect rather than replacing its observations.

## Current evidence and historical qualification

At the baseline, CI run `38213923599` passed; Supply Chain run `38213923644`
failed strict SBOM/license generation because the unknown-license exception
for `github.com/mattn/go-localereader@v0.0.1` expired on 2026-10-01.
The exception is removed by selecting upstream's licensed revision, with the
evidence and unchanged-code scope in ADR 0030. Local checks do not substitute
for a successful hosted run on the new candidate.

Historical Release Readiness runs `30176613797` and `31285674407` succeeded
for their recorded subjects. The latter qualified product
`dd92467d78abb64d176cf8eec7d963c5c6efd342` in `grandchallenge/AETHER`.
Neither qualifies the current baseline or a future package. Historical
"official run outstanding" remediation passages describe their original
candidate, not an assertion that qualification has never succeeded.
The GCL profile still retains controlled alpha and no claim-promotion role.

Issue #79 records a stopped setup and a later partial four-response session;
no completed twelve-observation operator packet is asserted. Issue #148's
non-operative FABRIC source-owner work is a separate programme and cannot
activate routing through this milestone.

## Closeout record

Record the protected product SHA/tree, tooling SHA/tree, package SHA-256,
configuration and fixture digests, prerequisite run/attempt IDs, independently
verified evidence bundle/verdict, deployment and recovery artifacts, operator
session ID and schema-valid evidence custody, defect dispositions, and final
claim boundary. Any product change requires a new candidate and affected
qualification. Missing human participation or protected approval is an open
dependency, not an engineering pass.
