package console

import (
	"context"
	"embed"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"net"
	"net/http"
	"net/url"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/fyremael/aether/go/internal/client"
)

//go:embed assets/*
var embeddedAssets embed.FS

var validNamespacePattern = regexp.MustCompile(`^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$`)

const (
	defaultListenAddr      = "127.0.0.1:3080"
	defaultRefreshInterval = 3 * time.Second
	minRefreshInterval     = 1 * time.Second
	defaultHistoryLimit    = 25
	maxHistoryLimit        = 100
	snapshotTimeout        = 8 * time.Second
)

// Config configures the read-only loopback Tufte operator console.
type Config struct {
	ListenAddr       string
	BaseURL          string
	DefaultNamespace string
	RefreshInterval  time.Duration
	HistoryLimit     int
	Output           io.Writer
}

func (c Config) normalized() Config {
	out := c
	if strings.TrimSpace(out.ListenAddr) == "" {
		out.ListenAddr = defaultListenAddr
	}
	if out.RefreshInterval < minRefreshInterval {
		out.RefreshInterval = defaultRefreshInterval
	}
	if out.HistoryLimit <= 0 {
		out.HistoryLimit = defaultHistoryLimit
	}
	if out.HistoryLimit > maxHistoryLimit {
		out.HistoryLimit = maxHistoryLimit
	}
	if out.Output == nil {
		out.Output = os.Stdout
	}
	return out
}

// ValidateLoopbackAddr verifies that addr is an explicit host:port bound solely
// to a loopback address (127.0.0.0/8, ::1, or localhost). Wildcard and LAN
// addresses are rejected fail-closed so the server-side bearer token cannot be
// exposed beyond loopback.
func ValidateLoopbackAddr(addr string) (string, error) {
	trimmed := strings.TrimSpace(addr)
	host, portStr, err := net.SplitHostPort(trimmed)
	if err != nil {
		return "", fmt.Errorf("invalid listen address %q: %w", addr, err)
	}
	if host == "" {
		return "", fmt.Errorf("listen address %q must specify an explicit loopback host (e.g. 127.0.0.1:3080)", addr)
	}
	port, err := strconv.Atoi(portStr)
	if err != nil || port < 0 || port > 65535 {
		return "", fmt.Errorf("invalid listen port %q in %q", portStr, addr)
	}
	if !isLoopbackHost(host) {
		return "", fmt.Errorf("refusing non-loopback listen host %q: operator console is loopback-only", host)
	}
	return trimmed, nil
}

func isLoopbackHost(host string) bool {
	cleaned := strings.TrimSpace(strings.Trim(host, "[]"))
	if cleaned == "" {
		return false
	}
	if strings.EqualFold(cleaned, "localhost") {
		return true
	}
	ip := net.ParseIP(cleaned)
	return ip != nil && ip.IsLoopback()
}

func sanitizeBaseURL(raw string) string {
	trimmed := strings.TrimSpace(raw)
	if trimmed == "" {
		return ""
	}
	parsed, err := url.Parse(trimmed)
	if err != nil {
		return trimmed
	}
	parsed.User = nil
	parsed.RawQuery = ""
	parsed.Fragment = ""
	return parsed.String()
}

// Snapshot is the read-only operational payload served to the Tufte UI.
type Snapshot struct {
	CollectedAtUnixMS int64                         `json:"collected_at_unix_ms"`
	FetchDurationMS   int64                         `json:"fetch_duration_ms"`
	BaseURL           string                        `json:"base_url"`
	Namespace         string                        `json:"namespace"`
	ReadOnly          bool                          `json:"read_only"`
	AuthorityEffect   string                        `json:"authority_effect"`
	RefreshIntervalMS int64                         `json:"refresh_interval_ms"`
	Health            *client.HealthResponse        `json:"health,omitempty"`
	HealthError       string                        `json:"health_error,omitempty"`
	Status            *client.ServiceStatusResponse `json:"status,omitempty"`
	StatusError       string                        `json:"status_error,omitempty"`
	Audit             *client.AuditLogResponse      `json:"audit,omitempty"`
	AuditError        string                        `json:"audit_error,omitempty"`
	AuditTelemetry    AuditTelemetry                `json:"audit_telemetry"`
	History           *ConsoleHistoryView           `json:"history,omitempty"`
	HistoryError      string                        `json:"history_error,omitempty"`
	Schema            *client.SchemaCatalogResponse `json:"schema,omitempty"`
	SchemaError       string                        `json:"schema_error,omitempty"`
	Receipts          []client.AppendReceipt        `json:"receipts,omitempty"`
	ReceiptsError     string                        `json:"receipts_error,omitempty"`
}

type ConsoleDatom struct {
	Element   uint64                 `json:"element"`
	Entity    uint64                 `json:"entity"`
	Attribute uint64                 `json:"attribute"`
	Op        string                 `json:"op"`
	Value     string                 `json:"value"`
	Policy    *client.PolicyEnvelope `json:"policy,omitempty"`
}

type ConsoleHistoryView struct {
	Page   client.PageInfo `json:"page"`
	Datoms []ConsoleDatom  `json:"datoms"`
}

func toConsoleHistory(page *client.PagedHistoryResponse) *ConsoleHistoryView {
	if page == nil {
		return nil
	}
	datoms := make([]ConsoleDatom, 0, len(page.Datoms))
	for _, d := range page.Datoms {
		datoms = append(datoms, ConsoleDatom{
			Element:   d.Element,
			Entity:    d.Entity,
			Attribute: d.Attribute,
			Op:        d.Op,
			Value:     d.Value.Display(),
			Policy:    d.Policy,
		})
	}
	return &ConsoleHistoryView{
		Page:   page.Page,
		Datoms: datoms,
	}
}

type PathCount struct {
	Path   string `json:"path"`
	Count  int    `json:"count"`
	Errors int    `json:"errors"`
}

type SparkBucket struct {
	OK    int `json:"ok"`
	Warn  int `json:"warn"`
	Error int `json:"error"`
}

type AuditTelemetry struct {
	TotalEntries        int            `json:"total_entries"`
	Status2xx           int            `json:"status_2xx"`
	Status4xx           int            `json:"status_4xx"`
	Status5xx           int            `json:"status_5xx"`
	LegacyEndpointCount int            `json:"legacy_endpoint_count"`
	SchemaOmittedCount  int            `json:"schema_omitted_count"`
	PolicyDecisions     map[string]int `json:"policy_decisions"`
	ByNamespace         map[string]int `json:"by_namespace"`
	TopPaths            []PathCount    `json:"top_paths"`
	SparklineBuckets    []SparkBucket  `json:"sparkline_buckets"`
}

func summarizeAudit(audit *client.AuditLogResponse) AuditTelemetry {
	out := AuditTelemetry{
		PolicyDecisions:  map[string]int{},
		ByNamespace:      map[string]int{},
		TopPaths:         []PathCount{},
		SparklineBuckets: []SparkBucket{},
	}
	if audit == nil || len(audit.Entries) == 0 {
		return out
	}

	entries := audit.Entries
	out.TotalEntries = len(entries)
	pathStats := map[string]*PathCount{}

	for _, entry := range entries {
		switch {
		case entry.Status >= 200 && entry.Status < 300:
			out.Status2xx++
		case entry.Status >= 400 && entry.Status < 500:
			out.Status4xx++
		case entry.Status >= 500:
			out.Status5xx++
		}
		if entry.Context.LegacyEndpoint {
			out.LegacyEndpointCount++
		}
		if entry.Context.SchemaRefOmitted {
			out.SchemaOmittedCount++
		}
		if entry.Context.PolicyDecision != nil && *entry.Context.PolicyDecision != "" {
			out.PolicyDecisions[*entry.Context.PolicyDecision]++
		}
		ns := "default"
		if entry.Context.Namespace != nil && *entry.Context.Namespace != "" {
			ns = *entry.Context.Namespace
		}
		out.ByNamespace[ns]++

		pc, ok := pathStats[entry.Path]
		if !ok {
			pc = &PathCount{Path: entry.Path}
			pathStats[entry.Path] = pc
		}
		pc.Count++
		if entry.Status >= 400 {
			pc.Errors++
		}
	}

	for _, pc := range pathStats {
		out.TopPaths = append(out.TopPaths, *pc)
	}
	sort.Slice(out.TopPaths, func(i, j int) bool {
		if out.TopPaths[i].Count != out.TopPaths[j].Count {
			return out.TopPaths[i].Count > out.TopPaths[j].Count
		}
		return out.TopPaths[i].Path < out.TopPaths[j].Path
	})
	if len(out.TopPaths) > 10 {
		out.TopPaths = out.TopPaths[:10]
	}

	const maxBuckets = 16
	bucketCount := maxBuckets
	if len(entries) < bucketCount {
		bucketCount = len(entries)
	}
	buckets := make([]SparkBucket, bucketCount)
	for i, entry := range entries {
		idx := (i * bucketCount) / len(entries)
		if idx >= bucketCount {
			idx = bucketCount - 1
		}
		switch {
		case entry.Status >= 500:
			buckets[idx].Error++
		case entry.Status >= 400:
			buckets[idx].Warn++
		default:
			buckets[idx].OK++
		}
	}
	out.SparklineBuckets = buckets
	return out
}

// NewHandler constructs the hardened, strictly read-only HTTP handler for the
// Tufte operator console.
func NewHandler(api *client.Client, cfg Config) http.Handler {
	cfg = cfg.normalized()
	assetsFS, err := fs.Sub(embeddedAssets, "assets")
	if err != nil {
		panic("embedded console assets missing: " + err.Error())
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.NotFound(w, r)
			return
		}
		serveEmbeddedAsset(w, assetsFS, "index.html", "text/html; charset=utf-8")
	})
	mux.HandleFunc("/assets/styles.css", func(w http.ResponseWriter, r *http.Request) {
		serveEmbeddedAsset(w, assetsFS, "styles.css", "text/css; charset=utf-8")
	})
	mux.HandleFunc("/assets/app.js", func(w http.ResponseWriter, r *http.Request) {
		serveEmbeddedAsset(w, assetsFS, "app.js", "application/javascript; charset=utf-8")
	})
	mux.HandleFunc("/api/snapshot", func(w http.ResponseWriter, r *http.Request) {
		handleSnapshot(w, r, api, cfg)
	})

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		setSecurityHeaders(w)
		if r.Method != http.MethodGet && r.Method != http.MethodHead {
			w.Header().Set("Allow", "GET, HEAD")
			writeJSONError(w, http.StatusMethodNotAllowed, "read-only operator console accepts GET and HEAD only")
			return
		}
		if !validateLoopbackRequest(r) {
			writeJSONError(w, http.StatusForbidden, "rejected non-loopback host or cross-site origin")
			return
		}
		mux.ServeHTTP(w, r)
	})
}

func setSecurityHeaders(w http.ResponseWriter) {
	h := w.Header()
	h.Set("Content-Security-Policy", "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'")
	h.Set("X-Content-Type-Options", "nosniff")
	h.Set("X-Frame-Options", "DENY")
	h.Set("Referrer-Policy", "no-referrer")
	h.Set("Cache-Control", "no-store")
}

func validateLoopbackRequest(r *http.Request) bool {
	host := r.Host
	if h, _, err := net.SplitHostPort(host); err == nil {
		host = h
	}
	if !isLoopbackHost(host) {
		return false
	}
	if fetchSite := strings.TrimSpace(r.Header.Get("Sec-Fetch-Site")); strings.EqualFold(fetchSite, "cross-site") {
		return false
	}
	if origin := strings.TrimSpace(r.Header.Get("Origin")); origin != "" {
		parsed, err := url.Parse(origin)
		if err != nil {
			return false
		}
		if parsed.Scheme != "http" && parsed.Scheme != "https" {
			return false
		}
		if !isLoopbackHost(parsed.Hostname()) {
			return false
		}
	}
	return true
}

func serveEmbeddedAsset(w http.ResponseWriter, fsys fs.FS, name string, contentType string) {
	data, err := fs.ReadFile(fsys, name)
	if err != nil {
		http.Error(w, "asset not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", contentType)
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write(data)
}

func handleSnapshot(w http.ResponseWriter, r *http.Request, api *client.Client, cfg Config) {
	ns := strings.TrimSpace(r.URL.Query().Get("namespace"))
	if ns == "" {
		ns = strings.TrimSpace(cfg.DefaultNamespace)
	}
	if ns != "" && !validNamespacePattern.MatchString(ns) {
		writeJSONError(w, http.StatusBadRequest, "invalid namespace parameter")
		return
	}

	limit := cfg.HistoryLimit
	if rawLimit := strings.TrimSpace(r.URL.Query().Get("limit")); rawLimit != "" {
		parsed, err := strconv.Atoi(rawLimit)
		if err != nil || parsed < 1 || parsed > maxHistoryLimit {
			writeJSONError(w, http.StatusBadRequest, fmt.Sprintf("limit must be between 1 and %d", maxHistoryLimit))
			return
		}
		limit = parsed
	}

	scoped := api
	if ns != "" {
		scoped = api.WithNamespace(ns)
	}

	ctx, cancel := context.WithTimeout(r.Context(), snapshotTimeout)
	defer cancel()

	started := time.Now()
	snap := Snapshot{
		CollectedAtUnixMS: started.UnixMilli(),
		BaseURL:           sanitizeBaseURL(cfg.BaseURL),
		Namespace:         ns,
		ReadOnly:          true,
		AuthorityEffect:   "none",
		RefreshIntervalMS: cfg.RefreshInterval.Milliseconds(),
	}

	redact := func(err error) string {
		if err == nil {
			return ""
		}
		return api.RedactSecret(err.Error())
	}

	var wg sync.WaitGroup
	wg.Add(6)

	go func() {
		defer wg.Done()
		res, err := scoped.Health(ctx)
		if err != nil {
			snap.HealthError = redact(err)
			return
		}
		snap.Health = res
	}()

	go func() {
		defer wg.Done()
		res, err := scoped.Status(ctx)
		if err != nil {
			snap.StatusError = redact(err)
			return
		}
		snap.Status = res
	}()

	go func() {
		defer wg.Done()
		res, err := scoped.AuditLog(ctx)
		if err != nil {
			snap.AuditError = redact(err)
			return
		}
		snap.Audit = res
	}()

	go func() {
		defer wg.Done()
		res, err := scoped.HistoryPage(ctx, 0, limit)
		if err != nil {
			snap.HistoryError = redact(err)
			return
		}
		if res.Page.Total > limit {
			tailOffset := res.Page.Total - limit
			if tailRes, tailErr := scoped.HistoryPage(ctx, tailOffset, limit); tailErr == nil {
				res = tailRes
			}
		}
		snap.History = toConsoleHistory(res)
	}()

	go func() {
		defer wg.Done()
		res, err := scoped.SchemaCatalog(ctx)
		if err != nil {
			snap.SchemaError = redact(err)
			return
		}
		snap.Schema = res
	}()

	go func() {
		defer wg.Done()
		res, err := scoped.AppendReceipts(ctx)
		if err != nil {
			snap.ReceiptsError = redact(err)
			return
		}
		snap.Receipts = res
	}()

	wg.Wait()
	snap.FetchDurationMS = time.Since(started).Milliseconds()
	snap.AuditTelemetry = summarizeAudit(snap.Audit)

	if snap.Namespace == "" && snap.Status != nil && snap.Status.EffectiveNamespace != nil {
		snap.Namespace = *snap.Status.EffectiveNamespace
	}
	if snap.Namespace == "" {
		snap.Namespace = "default"
	}

	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(http.StatusOK)
	_ = json.NewEncoder(w).Encode(snap)
}

func writeJSONError(w http.ResponseWriter, statusCode int, message string) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(statusCode)
	_ = json.NewEncoder(w).Encode(map[string]any{
		"error":            message,
		"read_only":        true,
		"authority_effect": "none",
	})
}

// Run validates the loopback listen address, starts the read-only Tufte
// operator console, and shuts down cleanly when ctx is cancelled.
func Run(ctx context.Context, api *client.Client, cfg Config) error {
	cfg = cfg.normalized()
	addr, err := ValidateLoopbackAddr(cfg.ListenAddr)
	if err != nil {
		return err
	}

	listener, err := net.Listen("tcp", addr)
	if err != nil {
		return fmt.Errorf("listen on %s: %w", addr, err)
	}
	defer listener.Close()

	server := &http.Server{
		Handler:           NewHandler(api, cfg),
		ReadHeaderTimeout: 5 * time.Second,
		ReadTimeout:       10 * time.Second,
		WriteTimeout:      15 * time.Second,
		IdleTimeout:       60 * time.Second,
	}

	fmt.Fprintf(
		cfg.Output,
		"AETHER read-only Tufte operator console listening on http://%s/ (upstream: %s, authority_effect=none)\n",
		listener.Addr().String(),
		sanitizeBaseURL(cfg.BaseURL),
	)

	errCh := make(chan error, 1)
	go func() {
		errCh <- server.Serve(listener)
	}()

	select {
	case <-ctx.Done():
		shutdownCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_ = server.Shutdown(shutdownCtx)
		err := <-errCh
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			return err
		}
		return nil
	case err := <-errCh:
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			return err
		}
		return nil
	}
}
