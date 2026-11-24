'use strict';

(function () {
  const state = {
    namespace: '',
    intervalMs: 3000,
    intervalInitialized: false,
    timer: null,
    filter: '',
    lastSnapshot: null,
    inFlight: false
  };

  const els = {
    metaVersion: document.getElementById('meta-version'),
    metaUpstream: document.getElementById('meta-upstream'),
    metaTransport: document.getElementById('meta-transport'),
    metaStorage: document.getElementById('meta-storage'),
    selNamespace: document.getElementById('sel-namespace'),
    selInterval: document.getElementById('sel-interval'),
    btnRefresh: document.getElementById('btn-refresh'),
    btnTheme: document.getElementById('btn-theme'),
    kpiHealth: document.getElementById('kpi-health'),
    kpiMode: document.getElementById('kpi-mode'),
    kpiNs: document.getElementById('kpi-ns'),
    kpiPrincipals: document.getElementById('kpi-principals'),
    kpiJournal: document.getElementById('kpi-journal'),
    kpiAudit: document.getElementById('kpi-audit'),
    kpiFetch: document.getElementById('kpi-fetch'),
    errorBanner: document.getElementById('error-banner'),
    auditCompatMeta: document.getElementById('audit-compat-meta'),
    auditSparkline: document.getElementById('audit-sparkline'),
    policyTotal: document.getElementById('policy-total'),
    policyBreakdown: document.getElementById('policy-breakdown'),
    pathDotplot: document.getElementById('path-dotplot'),
    rcCancelSemantics: document.getElementById('rc-cancel-semantics'),
    rcGrid: document.getElementById('resource-controls-grid'),
    capCount: document.getElementById('cap-count'),
    capLine: document.getElementById('capabilities-line'),
    nsActiveLabel: document.getElementById('ns-active-label'),
    nsMultiples: document.getElementById('ns-multiples'),
    auditFilter: document.getElementById('audit-filter'),
    auditShowing: document.getElementById('audit-showing'),
    auditTbody: document.getElementById('audit-tbody'),
    historyMeta: document.getElementById('history-meta'),
    schemaSummary: document.getElementById('schema-summary'),
    receiptsMeta: document.getElementById('receipts-meta'),
    receiptsTbody: document.getElementById('receipts-tbody'),
    datomsMeta: document.getElementById('datoms-meta'),
    historyTbody: document.getElementById('history-tbody'),
    principalsTbody: document.getElementById('principals-tbody'),
    replicasMeta: document.getElementById('replicas-meta'),
    replicasTbody: document.getElementById('replicas-tbody')
  };

  function clearNode(node) {
    while (node.firstChild) node.removeChild(node.firstChild);
  }

  function el(tag, className, text) {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined && text !== null) node.textContent = String(text);
    return node;
  }

  function addKV(parent, key, value, valClass) {
    const row = el('div', 'kv-row');
    row.appendChild(el('span', 'muted', key));
    row.appendChild(el('span', valClass || '', value));
    parent.appendChild(row);
  }

  function shortDigest(d) {
    if (!d) return '—';
    return d.length > 14 ? d.slice(0, 14) + '…' : d;
  }

  function fmtTime(ms) {
    if (!ms) return '—';
    const d = new Date(ms);
    return d.toISOString().slice(11, 19) + 'Z';
  }

  function fmtBytes(n) {
    if (!n) return '0 B';
    if (n >= 1048576) return (n / 1048576).toFixed(1) + ' MiB';
    if (n >= 1024) return (n / 1024).toFixed(0) + ' KiB';
    return n + ' B';
  }

  function formatPolicy(pc) {
    if (!pc) return 'public';
    const caps = Array.isArray(pc.capabilities) && pc.capabilities.length ? pc.capabilities.join(',') : '-';
    const vis = Array.isArray(pc.visibilities) && pc.visibilities.length ? pc.visibilities.join(',') : '-';
    return 'cap=[' + caps + '] vis=[' + vis + ']';
  }

  function formatDatomValue(v) {
    if (v === undefined || v === null) return 'null';
    return String(v);
  }

  function renderSparkline(buckets) {
    clearNode(els.auditSparkline);
    const ns = 'http://www.w3.org/2000/svg';
    if (!buckets || buckets.length === 0) {
      els.auditSparkline.appendChild(el('span', 'mono muted', 'No audit entries captured yet.'));
      return;
    }
    const maxTotal = Math.max(1, ...buckets.map(b => (b.ok || 0) + (b.warn || 0) + (b.error || 0)));
    const w = 320;
    const h = 50;
    const svg = document.createElementNS(ns, 'svg');
    svg.setAttribute('viewBox', '0 0 ' + w + ' ' + h);
    svg.setAttribute('preserveAspectRatio', 'none');

    // Subtle baseline hairline
    const base = document.createElementNS(ns, 'line');
    base.setAttribute('x1', '0');
    base.setAttribute('x2', String(w));
    base.setAttribute('y1', String(h - 1));
    base.setAttribute('y2', String(h - 1));
    base.setAttribute('stroke', 'var(--rule)');
    base.setAttribute('stroke-width', '1');
    svg.appendChild(base);

    const step = w / buckets.length;
    const barW = Math.max(3, step - 3);
    buckets.forEach((b, i) => {
      const ok = b.ok || 0;
      const warn = b.warn || 0;
      const err = b.error || 0;
      let y = h - 2;
      const drawSeg = (count, color) => {
        if (!count) return;
        const segH = Math.max(2, (count / maxTotal) * (h - 6));
        y -= segH;
        const rect = document.createElementNS(ns, 'rect');
        rect.setAttribute('x', (i * step + 1).toFixed(1));
        rect.setAttribute('y', Math.max(0, y).toFixed(1));
        rect.setAttribute('width', barW.toFixed(1));
        rect.setAttribute('height', segH.toFixed(1));
        rect.setAttribute('fill', color);
        svg.appendChild(rect);
      };
      drawSeg(ok, 'var(--ok)');
      drawSeg(warn, 'var(--warn)');
      drawSeg(err, 'var(--err)');
    });
    els.auditSparkline.appendChild(svg);
  }

  function renderDotplot(paths) {
    clearNode(els.pathDotplot);
    if (!paths || paths.length === 0) {
      els.pathDotplot.appendChild(el('div', 'muted', 'No endpoint traffic recorded yet.'));
      return;
    }
    const maxCount = Math.max(1, ...paths.map(p => p.count));
    paths.forEach(p => {
      const row = el('div', 'dot-row');
      const lbl = el('span', 'dot-label', p.path);
      lbl.title = p.path;
      const track = el('div', 'dot-track');
      track.appendChild(el('div', 'dot-line'));
      const pct = Math.max(3, Math.min(97, (p.count / maxCount) * 100));
      const dot = el('div', p.errors > 0 ? 'dot-mark dot-mark-err' : 'dot-mark');
      dot.style.left = 'calc(' + pct.toFixed(1) + '% - 3.5px)';
      track.appendChild(dot);
      const num = el('span', 'dot-num', p.errors > 0 ? p.count + ' · ' + p.errors + 'e' : String(p.count));
      row.appendChild(lbl);
      row.appendChild(track);
      row.appendChild(num);
      els.pathDotplot.appendChild(row);
    });
  }

  function renderNamespaceOptions(snap) {
    const names = new Set();
    if (snap.namespace) names.add(snap.namespace);
    if (snap.status && Array.isArray(snap.status.namespaces)) {
      snap.status.namespaces.forEach(n => { if (n.namespace) names.add(n.namespace); });
    }
    if (names.size === 0) names.add('default');
    const sorted = Array.from(names).sort();
    const current = state.namespace || snap.namespace || 'default';
    clearNode(els.selNamespace);
    sorted.forEach(ns => {
      const opt = el('option', '', ns);
      opt.value = ns;
      if (ns === current) opt.selected = true;
      els.selNamespace.appendChild(opt);
    });
  }

  function renderAuditTable(snap) {
    clearNode(els.auditTbody);
    if (snap.audit_error) {
      const tr = el('tr');
      const td = el('td', 'warn', 'Audit unavailable: ' + snap.audit_error);
      td.colSpan = 7;
      tr.appendChild(td);
      els.auditTbody.appendChild(tr);
      els.auditShowing.textContent = 'unavailable';
      return;
    }
    const all = (snap.audit && Array.isArray(snap.audit.entries)) ? snap.audit.entries : [];
    const q = state.filter.trim().toLowerCase();
    const filtered = all.slice().reverse().filter(e => {
      if (!q) return true;
      const hay = [
        e.method, e.path, String(e.status), e.principal, e.outcome,
        e.context && e.context.namespace, e.context && e.context.policy_decision, e.detail
      ].filter(Boolean).join(' ').toLowerCase();
      return hay.includes(q);
    });
    const shown = filtered.slice(0, 60);
    els.auditShowing.textContent = shown.length + ' / ' + all.length + ' entries';
    if (shown.length === 0) {
      const tr = el('tr');
      const td = el('td', 'muted', all.length === 0 ? 'No audit entries yet.' : 'No audit entries match filter.');
      td.colSpan = 7;
      tr.appendChild(td);
      els.auditTbody.appendChild(tr);
      return;
    }
    shown.forEach(e => {
      const tr = el('tr');
      tr.appendChild(el('td', 'muted', fmtTime(e.timestamp_ms)));
      const stCls = e.status >= 500 ? 'err' : e.status >= 400 ? 'warn' : 'ok';
      tr.appendChild(el('td', stCls, String(e.status)));
      const mp = el('td', '', e.method + ' ' + e.path);
      mp.title = e.method + ' ' + e.path;
      tr.appendChild(mp);
      tr.appendChild(el('td', '', e.principal || '—'));
      tr.appendChild(el('td', 'muted', (e.context && e.context.namespace) || 'default'));
      tr.appendChild(el('td', '', (e.context && e.context.policy_decision) || '—'));
      const detText = e.detail ? e.outcome + ' · ' + e.detail : e.outcome;
      const det = el('td', 'muted', detText);
      det.title = detText;
      tr.appendChild(det);
      els.auditTbody.appendChild(tr);
    });
  }

  function renderSnapshot(snap) {
    state.lastSnapshot = snap;
    renderNamespaceOptions(snap);

    const st = snap.status;
    els.metaVersion.textContent = st
      ? 'v' + st.build_version + ' · cfg ' + st.config_version + ' · schema ' + st.schema_version
      : 'status unavailable';
    els.metaUpstream.textContent = 'upstream: ' + (snap.base_url || '—');

    if (st && st.transport) {
      const tlsTag = st.transport.listener_tls ? 'native-tls' : (st.transport.listener_loopback ? 'loopback' : 'ingress-https');
      els.metaTransport.textContent = 'transport: ' + (st.transport.http_mode || '—') + ' (' + tlsTag + ')';
    } else {
      els.metaTransport.textContent = 'transport: —';
    }
    if (st && st.storage) {
      const pgTls = st.storage.postgres_tls_mode ? ' · pg-tls=' + st.storage.postgres_tls_mode : '';
      els.metaStorage.textContent = 'storage: ' + (st.storage.backend || '—') + pgTls + ' · sidecars=' + (st.storage.sidecar_mode || '—');
    } else {
      els.metaStorage.textContent = 'storage: —';
    }

    // KPI strip
    if (snap.health && snap.health.status === 'ok') {
      els.kpiHealth.textContent = 'OK';
      els.kpiHealth.className = 'kpi-val ok';
    } else {
      els.kpiHealth.textContent = snap.health_error ? 'UNREACHABLE' : (snap.health ? snap.health.status : 'DEGRADED');
      els.kpiHealth.className = 'kpi-val err';
    }
    els.kpiMode.textContent = st ? st.service_mode : '—';
    els.kpiNs.textContent = st ? (st.active_namespace_count + ' active / ' + (st.namespaces ? st.namespaces.length : 0) + ' listed') : '—';
    if (st && Array.isArray(st.principals)) {
      const revoked = st.principals.filter(p => p.revoked).length;
      els.kpiPrincipals.textContent = (st.principals.length - revoked) + ' active · ' + revoked + ' revoked';
    } else {
      els.kpiPrincipals.textContent = '—';
    }
    if (snap.history && snap.history.page) {
      els.kpiJournal.textContent = snap.history.page.total + ' datoms (' + snap.namespace + ')';
    } else {
      els.kpiJournal.textContent = snap.history_error ? 'restricted' : '—';
    }
    const at = snap.audit_telemetry || {};
    els.kpiAudit.textContent = (at.status_2xx || 0) + ' / ' + (at.status_4xx || 0) + ' / ' + (at.status_5xx || 0);
    els.kpiFetch.textContent = snap.fetch_duration_ms + 'ms · ' + fmtTime(snap.collected_at_unix_ms);

    // Top-level degraded warnings
    const errs = [];
    if (snap.health_error) errs.push('health: ' + snap.health_error);
    if (snap.status_error) errs.push('status: ' + snap.status_error);
    if (errs.length > 0) {
      els.errorBanner.textContent = errs.join(' | ');
      els.errorBanner.classList.remove('hidden');
    } else {
      els.errorBanner.classList.add('hidden');
    }

    // Section 1: Audit Sparkline, Policy Breakdown, Top Paths
    els.auditCompatMeta.textContent =
      'total=' + (at.total_entries || 0) +
      ' · legacy_endpoints=' + (at.legacy_endpoint_count || 0) +
      ' · omitted_schema_ref=' + (at.schema_omitted_count || 0);
    renderSparkline(at.sparkline_buckets || []);

    clearNode(els.policyBreakdown);
    const pEntries = Object.entries(at.policy_decisions || {}).sort((a, b) => b[1] - a[1]);
    els.policyTotal.textContent = pEntries.length ? pEntries.reduce((acc, x) => acc + x[1], 0) + ' evaluated' : '0 evaluated';
    if (pEntries.length === 0) {
      els.policyBreakdown.appendChild(el('div', 'muted', 'No policy-scoped decisions recorded.'));
    } else {
      pEntries.forEach(([k, v]) => {
        const cls = k.includes('deny') || k.includes('reject') ? 'err' : k.includes('narrow') ? 'warn' : 'ok';
        addKV(els.policyBreakdown, k, String(v), cls);
      });
    }
    renderDotplot(at.top_paths || []);

    // Section 2: Resource Controls & Capabilities
    clearNode(els.rcGrid);
    if (st && st.resource_controls) {
      const rc = st.resource_controls;
      els.rcCancelSemantics.textContent = rc.cancellation_semantics || '—';
      const rcRows = [
        ['global_worker_limit', rc.global_worker_limit],
        ['per_ns_concurrency', rc.per_namespace_concurrency_limit],
        ['per_ns_queue_limit', rc.per_namespace_queue_limit],
        ['requests_per_minute', rc.requests_per_minute],
        ['operation_timeout', rc.operation_timeout_ms + ' ms'],
        ['max_page_size', rc.max_page_size],
        ['max_request_body', fmtBytes(rc.max_request_body_bytes)],
        ['max_document_bytes', fmtBytes(rc.max_document_bytes)],
        ['max_document_rules', rc.max_document_rules],
        ['max_runtime_iters', rc.max_runtime_iterations],
        ['max_derived_tuples', rc.max_derived_tuples],
        ['audit_queue_limit', rc.audit_queue_limit],
        ['execution_retention', rc.execution_retention]
      ];
      rcRows.forEach(([k, v]) => addKV(els.rcGrid, k, String(v)));
    } else {
      els.rcCancelSemantics.textContent = '—';
      els.rcGrid.appendChild(el('div', 'muted', 'Status endpoint unavailable.'));
    }
    const caps = (st && Array.isArray(st.capabilities)) ? st.capabilities : [];
    els.capCount.textContent = caps.length + ' active';
    els.capLine.textContent = caps.length ? caps.join(' · ') : '—';

    // Section 3: Namespace Small Multiples
    clearNode(els.nsMultiples);
    els.nsActiveLabel.textContent = 'viewing namespace: ' + snap.namespace + ' (click a namespace to switch)';
    const nsList = (st && Array.isArray(st.namespaces) && st.namespaces.length)
      ? st.namespaces
      : [{ namespace: snap.namespace, principals: [] }];
    const auditByNs = at.by_namespace || {};
    nsList.forEach(item => {
      const card = el('div', 'ns-card mono');
      const isSel = item.namespace === snap.namespace;
      card.setAttribute('data-selected', String(isSel));
      const head = el('div', 'ns-card-head');
      head.appendChild(el('span', 'ns-card-name', item.namespace));
      head.appendChild(el('span', isSel ? 'ok' : 'muted', isSel ? 'SELECTED' : 'switch'));
      card.appendChild(head);
      addKV(card, 'principals', String(item.principals ? item.principals.length : 0));
      addKV(card, 'audit events', String(auditByNs[item.namespace] || 0));
      if (isSel && snap.schema && snap.schema.active) {
        addKV(card, 'active schema', snap.schema.active.schema_ref.version + ' (' + shortDigest(snap.schema.active.schema_ref.digest) + ')');
      } else if (isSel) {
        addKV(card, 'active schema', snap.schema_error ? 'restricted' : 'none');
      }
      card.addEventListener('click', () => {
        state.namespace = item.namespace;
        fetchSnapshot();
      });
      els.nsMultiples.appendChild(card);
    });

    // Section 4: Audit Table
    renderAuditTable(snap);

    // Section 5: Schema, Append Receipts & Journal Tail
    clearNode(els.schemaSummary);
    if (snap.schema_error) {
      addKV(els.schemaSummary, 'schema catalog', 'unavailable (' + snap.schema_error + ')', 'warn');
    } else if (snap.schema) {
      const active = snap.schema.active;
      addKV(els.schemaSummary, 'active schema', active ? active.schema_ref.version + ' · ' + shortDigest(active.schema_ref.digest) : 'none');
      addKV(els.schemaSummary, 'revisions / baselines', (snap.schema.revisions ? snap.schema.revisions.length : 0) + ' / ' + (snap.schema.baselines ? snap.schema.baselines.length : 0));
      const quarantined = (snap.schema.baselines || []).filter(b => b.status === 'quarantined').length;
      if (quarantined > 0) addKV(els.schemaSummary, 'quarantined baselines', String(quarantined), 'err');
    } else {
      addKV(els.schemaSummary, 'schema catalog', '—');
    }

    clearNode(els.receiptsTbody);
    if (snap.receipts_error) {
      els.receiptsMeta.textContent = 'restricted';
      const tr = el('tr');
      const td = el('td', 'warn', snap.receipts_error);
      td.colSpan = 5;
      tr.appendChild(td);
      els.receiptsTbody.appendChild(tr);
    } else {
      const receipts = Array.isArray(snap.receipts) ? snap.receipts : [];
      els.receiptsMeta.textContent = receipts.length + ' total';
      if (receipts.length === 0) {
        const tr = el('tr');
        const td = el('td', 'muted', 'No append receipts recorded.');
        td.colSpan = 5;
        tr.appendChild(td);
        els.receiptsTbody.appendChild(tr);
      } else {
        receipts.slice(-10).reverse().forEach(r => {
          const tr = el('tr');
          tr.appendChild(el('td', '', shortDigest(r.batch_id)));
          tr.appendChild(el('td', '', r.schema_ref ? r.schema_ref.version : '—'));
          const cutEl = r.committed_cut && r.committed_cut.last_element != null ? '#' + r.committed_cut.last_element : '0';
          tr.appendChild(el('td', '', cutEl + ' (' + (r.committed_cut ? r.committed_cut.entry_count : 0) + ')'));
          tr.appendChild(el('td', 'num', String(r.appended)));
          const flags = [];
          if (r.idempotent_replay) flags.push('replay');
          if (r.schema_ref_was_implicit) flags.push('implicit-schema');
          tr.appendChild(el('td', flags.length ? 'warn' : 'muted', flags.length ? flags.join(',') : 'explicit'));
          els.receiptsTbody.appendChild(tr);
        });
      }
    }

    clearNode(els.historyTbody);
    if (snap.history_error) {
      els.historyMeta.textContent = 'restricted';
      els.datomsMeta.textContent = 'unavailable';
      const tr = el('tr');
      const td = el('td', 'warn', snap.history_error);
      td.colSpan = 6;
      tr.appendChild(td);
      els.historyTbody.appendChild(tr);
    } else if (snap.history) {
      const datoms = Array.isArray(snap.history.datoms) ? snap.history.datoms : [];
      els.historyMeta.textContent = 'ns=' + snap.namespace;
      els.datomsMeta.textContent = 'showing ' + datoms.length + ' of ' + (snap.history.page ? snap.history.page.total : datoms.length);
      if (datoms.length === 0) {
        const tr = el('tr');
        const td = el('td', 'muted', 'Journal is empty in this namespace.');
        td.colSpan = 6;
        tr.appendChild(td);
        els.historyTbody.appendChild(tr);
      } else {
        datoms.slice().reverse().forEach(d => {
          const tr = el('tr');
          tr.appendChild(el('td', 'num', String(d.element)));
          tr.appendChild(el('td', 'num', String(d.entity)));
          tr.appendChild(el('td', 'num', String(d.attribute)));
          tr.appendChild(el('td', '', d.op));
          const val = formatDatomValue(d.value);
          const vTd = el('td', '', val);
          vTd.title = val;
          tr.appendChild(vTd);
          tr.appendChild(el('td', 'muted', formatPolicy(d.policy)));
          els.historyTbody.appendChild(tr);
        });
      }
    }

    // Section 6: Principals
    clearNode(els.principalsTbody);
    const principals = (st && Array.isArray(st.principals)) ? st.principals : [];
    if (principals.length === 0) {
      const tr = el('tr');
      const td = el('td', 'muted', 'No principals reported.');
      td.colSpan = 6;
      tr.appendChild(td);
      els.principalsTbody.appendChild(tr);
    } else {
      principals.forEach(p => {
        const tr = el('tr');
        tr.appendChild(el('td', '', p.principal_id || p.principal));
        tr.appendChild(el('td', 'muted', p.token_id || '—'));
        tr.appendChild(el('td', '', (p.scopes || []).join(',')));
        tr.appendChild(el('td', '', (p.namespaces || []).join(',')));
        tr.appendChild(el('td', 'muted', formatPolicy(p.policy_context)));
        tr.appendChild(el('td', p.revoked ? 'err' : 'ok', p.revoked ? 'REVOKED' : 'active'));
        els.principalsTbody.appendChild(tr);
      });
    }

    // Section 7: Replicas
    clearNode(els.replicasTbody);
    const replicas = (st && Array.isArray(st.replicas)) ? st.replicas : [];
    els.replicasMeta.textContent = replicas.length ? replicas.length + ' replicas' : (st ? st.service_mode : 'single_node');
    if (replicas.length === 0) {
      const tr = el('tr');
      const td = el('td', 'muted', 'Single-node mode — no replicated authority partitions configured.');
      td.colSpan = 7;
      tr.appendChild(td);
      els.replicasTbody.appendChild(tr);
    } else {
      replicas.forEach(r => {
        const tr = el('tr');
        tr.appendChild(el('td', '', r.partition));
        tr.appendChild(el('td', 'num', String(r.replica_id)));
        tr.appendChild(el('td', '', r.role));
        tr.appendChild(el('td', 'num', String(r.leader_epoch)));
        tr.appendChild(el('td', 'num', r.applied_element != null ? String(r.applied_element) : '—'));
        tr.appendChild(el('td', 'num', String(r.replication_lag)));
        tr.appendChild(el('td', r.healthy ? 'ok' : 'err', r.healthy ? 'healthy' : 'degraded'));
        els.replicasTbody.appendChild(tr);
      });
    }
  }

  async function fetchSnapshot() {
    if (state.inFlight) return;
    state.inFlight = true;
    try {
      const qs = state.namespace ? '?namespace=' + encodeURIComponent(state.namespace) : '';
      const res = await fetch('/api/snapshot' + qs, { cache: 'no-store' });
      const data = await res.json();
      if (!res.ok) {
        els.errorBanner.textContent = data.error || ('HTTP ' + res.status);
        els.errorBanner.classList.remove('hidden');
        return;
      }
      if (!state.namespace && data.namespace) {
        state.namespace = data.namespace;
      }
      if (!state.intervalInitialized && data.refresh_interval_ms >= 1000) {
        state.intervalInitialized = true;
        const ms = Number(data.refresh_interval_ms);
        state.intervalMs = ms;
        const match = Array.from(els.selInterval.options).find(o => Number(o.value) === ms);
        if (!match) {
          const opt = el('option', '', Math.round(ms / 1000) + 's');
          opt.value = String(ms);
          els.selInterval.insertBefore(opt, els.selInterval.firstChild);
        }
        els.selInterval.value = String(ms);
        resetTimer();
      }
      renderSnapshot(data);
    } catch (err) {
      els.errorBanner.textContent = 'Console fetch failed: ' + err.message;
      els.errorBanner.classList.remove('hidden');
    } finally {
      state.inFlight = false;
    }
  }

  function resetTimer() {
    if (state.timer) {
      clearInterval(state.timer);
      state.timer = null;
    }
    if (state.intervalMs > 0) {
      state.timer = setInterval(fetchSnapshot, state.intervalMs);
    }
  }

  els.selNamespace.addEventListener('change', () => {
    state.namespace = els.selNamespace.value;
    fetchSnapshot();
  });
  els.selInterval.addEventListener('change', () => {
    state.intervalMs = Number(els.selInterval.value);
    resetTimer();
  });
  els.btnRefresh.addEventListener('click', fetchSnapshot);
  els.auditFilter.addEventListener('input', () => {
    state.filter = els.auditFilter.value;
    if (state.lastSnapshot) renderAuditTable(state.lastSnapshot);
  });
  els.btnTheme.addEventListener('click', () => {
    const root = document.documentElement;
    const cur = root.getAttribute('data-theme');
    if (cur === 'dark') root.setAttribute('data-theme', 'light');
    else if (cur === 'light') root.setAttribute('data-theme', 'dark');
    else {
      const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
      root.setAttribute('data-theme', prefersDark ? 'light' : 'dark');
    }
  });
  window.addEventListener('keydown', (e) => {
    if (e.target && (e.target.tagName === 'INPUT' || e.target.tagName === 'SELECT')) return;
    if (e.key === 'r') fetchSnapshot();
  });

  fetchSnapshot();
  resetTimer();
})();
