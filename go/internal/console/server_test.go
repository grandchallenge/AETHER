package console

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/fyremael/aether/go/internal/client"
)

func TestValidateLoopbackAddr(t *testing.T) {
	valid := []string{
		"127.0.0.1:3080",
		"127.0.0.1:0",
		"127.0.1.5:8080",
		"[::1]:3080",
		"[::1]:0",
		"localhost:3080",
		"LOCALHOST:3080",
	}
	for _, addr := range valid {
		if _, err := ValidateLoopbackAddr(addr); err != nil {
			t.Fatalf("expected valid loopback %q, got error: %v", addr, err)
		}
	}

	invalid := []string{
		"",
		":3080",
		"0.0.0.0:3080",
		"[::]:3080",
		"192.168.1.25:3080",
		"10.0.0.5:3080",
		"example.com:3080",
		"127.0.0.1",
		"127.0.0.1:99999",
		"127.0.0.1:-1",
	}
	for _, addr := range invalid {
		if _, err := ValidateLoopbackAddr(addr); err == nil {
			t.Fatalf("expected non-loopback/invalid address %q to fail closed", addr)
		}
	}
}

func TestSecurityHeadersMethodAndOriginGuards(t *testing.T) {
	api := client.New("http://127.0.0.1:3000", "secret-token-never-leak")
	handler := NewHandler(api, Config{BaseURL: "http://127.0.0.1:3000"})

	// 1. Static asset endpoints return strict security headers and never leak bearer token.
	for _, tc := range []struct {
		path string
		ct   string
	}{
		{"/", "text/html"},
		{"/assets/styles.css", "text/css"},
		{"/assets/app.js", "application/javascript"},
	} {
		req := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080"+tc.path, nil)
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, req)
		if rec.Code != http.StatusOK {
			t.Fatalf("%s: expected 200, got %d", tc.path, rec.Code)
		}
		if !strings.Contains(rec.Header().Get("Content-Type"), tc.ct) {
			t.Fatalf("%s: unexpected Content-Type %q", tc.path, rec.Header().Get("Content-Type"))
		}
		if rec.Header().Get("Content-Security-Policy") == "" {
			t.Fatalf("%s: missing CSP header", tc.path)
		}
		if rec.Header().Get("X-Content-Type-Options") != "nosniff" {
			t.Fatalf("%s: missing nosniff", tc.path)
		}
		if rec.Header().Get("X-Frame-Options") != "DENY" {
			t.Fatalf("%s: missing DENY frame option", tc.path)
		}
		if rec.Header().Get("Cache-Control") != "no-store" {
			t.Fatalf("%s: missing Cache-Control no-store", tc.path)
		}
		if strings.Contains(rec.Body.String(), "secret-token-never-leak") {
			t.Fatalf("%s: bearer token leaked in asset body", tc.path)
		}
	}

	// 2. Non-GET/HEAD methods are strictly rejected (read-only invariant).
	for _, method := range []string{http.MethodPost, http.MethodPut, http.MethodDelete, http.MethodPatch} {
		req := httptest.NewRequest(method, "http://127.0.0.1:3080/api/snapshot", nil)
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, req)
		if rec.Code != http.StatusMethodNotAllowed {
			t.Fatalf("%s: expected 405, got %d", method, rec.Code)
		}
		if rec.Header().Get("Allow") != "GET, HEAD" {
			t.Fatalf("%s: expected Allow: GET, HEAD, got %q", method, rec.Header().Get("Allow"))
		}
	}

	// 3. Anti-DNS-rebinding (non-loopback Host) and cross-site Origin/Sec-Fetch-Site denied.
	badHost := httptest.NewRequest(http.MethodGet, "http://evil.example:3080/api/snapshot", nil)
	badHost.Host = "evil.example:3080"
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, badHost)
	if rec.Code != http.StatusForbidden {
		t.Fatalf("non-loopback Host: expected 403, got %d", rec.Code)
	}

	badOrigin := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080/api/snapshot", nil)
	badOrigin.Header.Set("Origin", "https://attacker.example")
	rec = httptest.NewRecorder()
	handler.ServeHTTP(rec, badOrigin)
	if rec.Code != http.StatusForbidden {
		t.Fatalf("cross-origin Origin: expected 403, got %d", rec.Code)
	}

	crossSite := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080/api/snapshot", nil)
	crossSite.Header.Set("Sec-Fetch-Site", "cross-site")
	rec = httptest.NewRecorder()
	handler.ServeHTTP(rec, crossSite)
	if rec.Code != http.StatusForbidden {
		t.Fatalf("Sec-Fetch-Site cross-site: expected 403, got %d", rec.Code)
	}

	// 4. Invalid namespace and out-of-range limit fail closed with 400.
	badNS := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080/api/snapshot?namespace=../etc", nil)
	rec = httptest.NewRecorder()
	handler.ServeHTTP(rec, badNS)
	if rec.Code != http.StatusBadRequest {
		t.Fatalf("invalid namespace: expected 400, got %d", rec.Code)
	}

	badLimit := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080/api/snapshot?limit=999", nil)
	rec = httptest.NewRecorder()
	handler.ServeHTTP(rec, badLimit)
	if rec.Code != http.StatusBadRequest {
		t.Fatalf("out-of-range limit: expected 400, got %d", rec.Code)
	}
}

func TestSnapshotAggregatesLiveUpstreamAndRedactsSecrets(t *testing.T) {
	const secretToken = "super-secret-pilot-token-98765"
	var nsMu sync.Mutex
	var observedNamespaces []string

	upstream := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer "+secretToken {
			w.WriteHeader(http.StatusUnauthorized)
			_, _ = w.Write([]byte(`{"error":"missing bearer token"}`))
			return
		}
		if ns := r.Header.Get("X-Aether-Namespace"); ns != "" {
			nsMu.Lock()
			observedNamespaces = append(observedNamespaces, ns)
			nsMu.Unlock()
		}
		w.Header().Set("Content-Type", "application/json")
		switch r.URL.Path {
		case "/health":
			_, _ = w.Write([]byte(`{"status":"ok"}`))
		case "/v1/status":
			_, _ = w.Write([]byte(`{
				"status":"ok",
				"build_version":"0.1.0-test",
				"config_version":"pilot-v1",
				"schema_version":"v1",
				"effective_namespace":"ops",
				"service_mode":"single_node",
				"transport":{"http_mode":"loopback_plaintext","listener_loopback":true,"listener_tls":false},
				"storage":{"backend":"sqlite","sidecar_mode":"sqlite_local_per_namespace"},
				"active_namespace_count":2,
				"capabilities":["trace_handles_v1","namespace_schema_ref_v1","append_receipts_v1","structured_errors_v1","resource_limits_v1","pagination_v1"],
				"namespaces":[{"namespace":"default","principals":["operator"]},{"namespace":"ops","principals":["operator"]}],
				"principals":[{"principal":"operator","principal_id":"principal:operator","token_id":"token:operator","scopes":["query","ops"],"namespaces":["default","ops"],"source":"token_file","revoked":false}],
				"replicas":[],
				"resource_controls":{"max_request_body_bytes":1048576,"max_document_bytes":262144,"max_document_rules":512,"max_runtime_iterations":4096,"max_derived_tuples":1000000,"operation_timeout_ms":30000,"max_page_size":500,"requests_per_minute":600,"global_worker_limit":8,"per_namespace_concurrency_limit":1,"per_namespace_queue_limit":64,"audit_queue_limit":1024,"execution_retention":1024,"cancellation_semantics":"cancel_before_start_complete_after_start"}
			}`))
		case "/v1/audit":
			_, _ = w.Write([]byte(`{"entries":[
				{"timestamp_ms":1700000001000,"principal":"operator","method":"POST","path":"/v1/documents/run","status":200,"scope":"query","outcome":"allowed","context":{"namespace":"ops","policy_decision":"narrowed"}},
				{"timestamp_ms":1700000002000,"principal":"operator","method":"POST","path":"/v1/documents/run","status":403,"scope":"query","outcome":"denied","detail":"policy exceeded","context":{"namespace":"ops","policy_decision":"denied"}},
				{"timestamp_ms":1700000003000,"principal":"operator","method":"POST","path":"/v1/explain/tuple","status":409,"scope":"query","outcome":"denied","context":{"namespace":"default","legacy_endpoint":true,"schema_ref_omitted":true}}
			]}`))
		case "/v1/history/page":
			_, _ = w.Write([]byte(`{
				"page":{"offset":0,"limit":25,"total":1},
				"datoms":[{"entity":7,"attribute":2,"value":{"String":"task-ready"},"op":"Assert","element":1,"replica":1,"causal_context":{"frontier":[]},"provenance":{"author_principal":"operator","agent_id":"a","tool_id":"t","session_id":"s","source_ref":{"uri":"mem"},"parent_datom_ids":[],"confidence":1.0,"trust_domain":"local","schema_version":"v1"}}]
			}`))
		case "/v1/schema":
			// Simulate a hostile upstream error that echoes the secret token;
			// console must redact it before sending JSON to the browser.
			w.WriteHeader(http.StatusForbidden)
			_, _ = fmt.Fprintf(w, `{"error":"forbidden for token %s"}`, secretToken)
		case "/v1/append/receipts":
			_, _ = w.Write([]byte(`[]`))
		default:
			http.NotFound(w, r)
		}
	}))
	defer upstream.Close()

	api := client.New(upstream.URL, secretToken)
	handler := NewHandler(api, Config{
		BaseURL:          upstream.URL,
		DefaultNamespace: "ops",
		RefreshInterval:  5 * time.Second,
	})

	req := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080/api/snapshot?namespace=ops", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)
	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200 snapshot, got %d: %s", rec.Code, rec.Body.String())
	}

	raw := rec.Body.String()
	if strings.Contains(raw, secretToken) {
		t.Fatalf("bearer token leaked in snapshot payload: %s", raw)
	}

	var snap Snapshot
	if err := json.Unmarshal(rec.Body.Bytes(), &snap); err != nil {
		t.Fatalf("decode snapshot: %v", err)
	}
	if !snap.ReadOnly || snap.AuthorityEffect != "none" {
		t.Fatalf("expected read_only=true authority_effect=none, got %+v", snap)
	}
	if snap.Health == nil || snap.Health.Status != "ok" {
		t.Fatalf("expected healthy upstream, got %+v", snap.Health)
	}
	if snap.Status == nil || snap.Status.ResourceControls.GlobalWorkerLimit != 8 {
		t.Fatalf("expected resource_controls.global_worker_limit=8, got %+v", snap.Status)
	}
	if snap.AuditTelemetry.TotalEntries != 3 || snap.AuditTelemetry.Status2xx != 1 || snap.AuditTelemetry.Status4xx != 2 {
		t.Fatalf("unexpected audit telemetry: %+v", snap.AuditTelemetry)
	}
	if snap.AuditTelemetry.LegacyEndpointCount != 1 || snap.AuditTelemetry.SchemaOmittedCount != 1 {
		t.Fatalf("expected compatibility counters=1, got %+v", snap.AuditTelemetry)
	}
	if snap.AuditTelemetry.PolicyDecisions["narrowed"] != 1 || snap.AuditTelemetry.PolicyDecisions["denied"] != 1 {
		t.Fatalf("unexpected policy decisions: %+v", snap.AuditTelemetry.PolicyDecisions)
	}
	if snap.History == nil || len(snap.History.Datoms) != 1 || snap.History.Datoms[0].Value != `"task-ready"` {
		t.Fatalf("expected formatted datom value, got %+v", snap.History)
	}
	if !strings.Contains(snap.SchemaError, "[redacted]") {
		t.Fatalf("expected secret redaction in schema_error, got %q", snap.SchemaError)
	}
	nsMu.Lock()
	nsCount := len(observedNamespaces)
	nsMu.Unlock()
	if nsCount == 0 {
		t.Fatalf("expected X-Aether-Namespace header forwarded to upstream")
	}
}

type syncBuffer struct {
	mu  sync.Mutex
	buf bytes.Buffer
}

func (b *syncBuffer) Write(p []byte) (int, error) {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.buf.Write(p)
}

func (b *syncBuffer) String() string {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.buf.String()
}

func TestRunStartsLoopbackAndShutsDownCleanly(t *testing.T) {
	upstream := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		switch r.URL.Path {
		case "/health":
			_, _ = w.Write([]byte(`{"status":"ok"}`))
		case "/v1/status":
			_, _ = w.Write([]byte(`{"status":"ok","build_version":"0.1.0","config_version":"v1","schema_version":"v1","service_mode":"single_node","storage":{"backend":"sqlite","sidecar_mode":"sqlite_local_per_namespace"},"principals":[],"replicas":[]}`))
		default:
			w.WriteHeader(http.StatusForbidden)
			_, _ = w.Write([]byte(`{"error":"restricted"}`))
		}
	}))
	defer upstream.Close()

	var out syncBuffer
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	errCh := make(chan error, 1)
	go func() {
		errCh <- Run(ctx, client.New(upstream.URL, "loopback-test-token"), Config{
			ListenAddr: "127.0.0.1:0",
			BaseURL:    upstream.URL,
			Output:     &out,
		})
	}()

	var listenURL string
	deadline := time.Now().Add(3 * time.Second)
	for time.Now().Before(deadline) {
		text := out.String()
		if idx := strings.Index(text, "http://127.0.0.1:"); idx >= 0 {
			rest := text[idx:]
			if end := strings.Index(rest, " "); end > 0 {
				listenURL = rest[:end]
				break
			}
		}
		time.Sleep(15 * time.Millisecond)
	}
	if listenURL == "" {
		t.Fatalf("console did not report bound loopback URL; output=%q", out.String())
	}

	resp, err := http.Get(listenURL + "api/snapshot")
	if err != nil {
		t.Fatalf("GET live console snapshot: %v", err)
	}
	body, _ := io.ReadAll(resp.Body)
	_ = resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("expected 200 from live loopback console, got %d: %s", resp.StatusCode, string(body))
	}
	if strings.Contains(string(body), "loopback-test-token") {
		t.Fatalf("token leaked from live console server")
	}

	// Verify HEAD and 404 preserve strict security headers.
	headResp, err := http.Head(listenURL)
	if err != nil {
		t.Fatalf("HEAD /: %v", err)
	}
	_ = headResp.Body.Close()
	if headResp.StatusCode != http.StatusOK || headResp.Header.Get("X-Frame-Options") != "DENY" {
		t.Fatalf("HEAD / unexpected status=%d frame=%q", headResp.StatusCode, headResp.Header.Get("X-Frame-Options"))
	}

	nfResp, err := http.Get(listenURL + "nonexistent")
	if err != nil {
		t.Fatalf("GET /nonexistent: %v", err)
	}
	_ = nfResp.Body.Close()
	if nfResp.StatusCode != http.StatusNotFound || nfResp.Header.Get("X-Content-Type-Options") != "nosniff" {
		t.Fatalf("GET /nonexistent unexpected status=%d nosniff=%q", nfResp.StatusCode, nfResp.Header.Get("X-Content-Type-Options"))
	}

	cancel()
	select {
	case err := <-errCh:
		if err != nil {
			t.Fatalf("expected clean shutdown, got: %v", err)
		}
	case <-time.After(3 * time.Second):
		t.Fatalf("console server did not shut down within timeout")
	}
}

func TestSnapshotFetchesJournalTailWhenTotalExceedsLimit(t *testing.T) {
	var seenOffsets []string
	var mu sync.Mutex
	upstream := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		switch r.URL.Path {
		case "/health":
			_, _ = w.Write([]byte(`{"status":"ok"}`))
		case "/v1/status":
			_, _ = w.Write([]byte(`{"status":"ok","build_version":"0.1.0","config_version":"v1","schema_version":"v1","service_mode":"single_node","storage":{"backend":"sqlite","sidecar_mode":"sqlite_local_per_namespace"},"principals":[],"replicas":[]}`))
		case "/v1/history/page":
			offset := r.URL.Query().Get("offset")
			mu.Lock()
			seenOffsets = append(seenOffsets, offset)
			mu.Unlock()
			if offset == "7" {
				_, _ = w.Write([]byte(`{"page":{"offset":7,"limit":3,"total":10},"datoms":[{"entity":9,"attribute":1,"value":{"U64":42},"op":"Assert","element":10,"replica":1,"causal_context":{"frontier":[]},"provenance":{"author_principal":"op","agent_id":"a","tool_id":"t","session_id":"s","source_ref":{"uri":"mem"},"parent_datom_ids":[],"confidence":1.0,"trust_domain":"local","schema_version":"v1"}}]}`))
				return
			}
			_, _ = w.Write([]byte(`{"page":{"offset":0,"limit":3,"total":10},"datoms":[]}`))
		default:
			_, _ = w.Write([]byte(`{}`))
		}
	}))
	defer upstream.Close()

	handler := NewHandler(client.New(upstream.URL, "tail-token"), Config{
		BaseURL:      upstream.URL,
		HistoryLimit: 3,
	})
	req := httptest.NewRequest(http.MethodGet, "http://127.0.0.1:3080/api/snapshot?limit=3", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)
	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rec.Code)
	}
	var snap Snapshot
	if err := json.Unmarshal(rec.Body.Bytes(), &snap); err != nil {
		t.Fatalf("decode snapshot: %v", err)
	}
	if snap.History == nil || snap.History.Page.Offset != 7 || len(snap.History.Datoms) != 1 || snap.History.Datoms[0].Value != "42" {
		t.Fatalf("expected tail page at offset=7 element=10 value=42, got %+v (offsets=%v)", snap.History, seenOffsets)
	}
}
