# ADR 0023: Explicit empty inputs in the frozen pilot adapter

Status: candidate correction for AETHER issue #81.

The operator tool generated an invalid inline schema block. After fixing that syntax, executing the frozen cases exposed missing extensional bindings for relations with no fixture rows, such as assignments in the normal case. The kernel deliberately rejects undeclared missing inputs; that control must remain intact.

The bounded adapter now emits a multiline schema and declares each absent fixture input as an empty derived relation in the DSL. The declaration conjoins the same positive and negative `case_status(a, b)` atom. Their conjunction has no solution, regardless of the case-status rows. All variables are bound by positive case-status atoms. The case-status anchor is required, so a missing anchor remains an error.

This adds no dummy facts, does not read ground truth, and does not compute readiness or recursive closure in host callbacks. Existing readiness, selection, suppression, dependency and stale-fencing rules remain unchanged. Nonempty fixture inputs retain their original rows. The change is confined to the six-case adapter; it neither changes the kernel nor creates a general missing-input fallback.

An explicit Cargo example test target makes ordinary workspace tests execute every frozen case through the SQLite runtime and all operator query surfaces. The test covers cases with and without assignment and dependency rows. Python preflight separately requires exact equality between working-file and committed bytes, rejecting silent line-ending conversion.

The stopped operator attempt remains immutable. Tests and protection do not constitute operator evidence; a separately identified session is needed after provider checks and the applicable review are satisfied.
