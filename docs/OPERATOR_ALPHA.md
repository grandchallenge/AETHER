# Support desk operator alpha

This Windows package contains the Rust service, Go console and a standalone
six-case support-desk runner. The runner embeds the frozen synthetic case pack;
it needs no Cargo, Git, Python, source checkout or network connection.

## Before a task

You are reviewing a synthetic support case before deciding the next action.
The desk shows current cases, evidence, eligible resolutions, ownership and
the facts behind a decision. "Ready" means eligible before assignment;
"selected" means assigned to a current owner. A stale assignment belongs to
an earlier ownership epoch. No suggested action is executed externally.

From the extracted package directory, run a case with a dedicated database:

```powershell
.\bin\support_desk_real_operator_pilot_case.exe pilot-case-01-normal-resolution .\data\operator-case-01.sqlite
```

The other case IDs are `pilot-case-02-missing-approval`,
`pilot-case-03-dependency-incomplete`, `pilot-case-04-handoff-stale-fencing`,
`pilot-case-05-suppressed-resolution`, and `pilot-case-06-closed-case-fencing`.
The facilitator follows the existing counterbalanced schedule; operators
should not inspect later cases before their turn.

Each run shows the five question surfaces, readable decision inputs and a
replay-verified execution-scoped proof when a positive derived tuple exists.
An empty result does not itself carry a proof of absence. The source inputs
remain visible for reconstructing why a resolution is unavailable.

To inspect a previously printed handle after the process exits, run:

```powershell
.\bin\support_desk_real_operator_pilot_case.exe --explain <handle> .\data\operator-case-01.sqlite
```

This checks the stored digests and replays the original execution before
returning its full proof. Keep the handle with its original SQLite state.

The cases are frozen DSL snapshots, not a live support inbox or a demonstration
of journal-driven case updates. The SQLite execution store retains their
replay metadata. Service journal Current/AsOf and stopped backup/restore are
separately required in package qualification.

## Package identity and pilot evidence

`bin\support_desk_real_operator_pilot_case.exe --identity` prints its compiled
product revision and embedded fixture SHA-256. They must match the qualified
package and facilitator manifest. `operator-surface.json` records the same
identity; the independently verified package digest is the custody anchor.

The facilitator can use the existing source-side pilot script's
`--operator-binary <extracted-package>\bin\support_desk_real_operator_pilot_case.exe`
option on `prepare` and `show-task`. It checks revision, fixture and binary
identity before launching, avoiding compilation during timed tasks.
The operator machine needs only the extracted package.

This is controlled single-node alpha using synthetic data, one declared
visibility/authority domain and trusted appenders. A human session needs the
protected protocol briefing and its actual observations. Engineering rehearsal
does not provide human task times, answers, usefulness ratings or comparisons.
