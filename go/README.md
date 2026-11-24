# Go Boundary

This directory now contains the first real Go operator shell for AETHER.

The Go client negotiates `resource_limits_v1` and `pagination_v1` and exposes
`HistoryPage`, `RunDocumentPage`, and `ResolveTraceHandlePage`. Typed limit or
rate failures are terminal for that request; callers must not bypass them with
legacy endpoints.

Scope for Go in v1:

- CLI and admin commands
- deployment-oriented process wrappers
- service lifecycle integration
- narrow API boundary consumers

Implemented today:

- `cmd/aetherctl`, a real CLI over the stable HTTP boundary
- `cmd/aetherctl tui`, a pilot-focused read-mostly operator cockpit for live service health, coordination state, cut diffs, audit entries, history, and tuple proof traces
- `cmd/aetherctl console`, a loopback-only read-only Tufte web operator console (`net/http` + `embed.FS`, zero external CDNs, server-side bearer token inside `internal/client`) surfacing live health, service status, transport/storage TLS posture, resource controls, audit sparklines and policy-decision telemetry, namespace small multiples, schema catalog, append receipts, journal tail, principals, and replica status
- `internal/client`, a typed Go HTTP client for health, service status, history, audit, pilot coordination reports, pilot coordination delta reports, document runs, and execution-scoped proof resolution
- request-level policy-context support for document execution, with authenticated tokens able to impose the maximum semantic visibility that requests may narrow but not exceed
- explain, report, and history calls now follow the same token-bound effective policy as document execution on authenticated services
- Go unit coverage via `go test ./...`

Current commands:

```bash
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 health
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 history
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 --token-file ./pilot-operator.token status
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 --token-file ./pilot-operator.token reload-auth
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 --token-file ./pilot-operator.token coordination-report
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 --token-file ./pilot-operator.token coordination-diff --left asof:5 --right current
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 run --file ./document.aether
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 run --file ./document.aether --capabilities executor --visibilities ops
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 explain --trace-handle <64-hex-handle> --verify-replay --capabilities executor
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 --token-file ./pilot-operator.token tui --refresh 2s
go run ./cmd/aetherctl --base-url http://127.0.0.1:3000 --token-file ./pilot-operator.token console --listen 127.0.0.1:3080 --refresh 3s
```

Semantic CLI and TUI commands preflight the service capability set from
`/v1/status`. They fail closed when trace handles, namespace schema refs, append
receipts, or structured errors are unavailable, and never retry explanation by
tuple ID.

`aetherctl tui` is the live pilot operations entrance. It is intentionally narrow:

- read-only in v1
- backed only by the authenticated HTTP service
- focused on the current coordination pilot rather than arbitrary AETHER workflows
- aware of service status, replication summaries, and saved-cut diffs when the service exposes them

Auth handling for `aetherctl` now supports:

- `--token`
- `--token-file`
- `AETHER_TOKEN` as the fallback when neither flag is supplied

Current test command:

```bash
go test ./...
```

Out of scope:

- authoritative semantic execution
- resolver duplication
- rule-engine ownership
