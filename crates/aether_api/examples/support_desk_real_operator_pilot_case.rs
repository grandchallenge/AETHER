use aether_api::{
    KernelService, ResolveTraceHandleRequest, RunDocumentRequest, SqliteKernelService,
};
use aether_ast::{QueryRow, Value};
use serde::Deserialize;
use std::{env, fs, path::PathBuf, process::Command};

const FIXTURE_PATH: &str = "fixtures/product/support-desk-real-operator-pilot-v1.json";
const PROTOCOL_ID: &str = "AETHER-SUPPORT-DESK-REAL-OPERATOR-PILOT-001";
const CASE_SET_ID: &str = "AETHER-SUPPORT-DESK-REAL-OPERATOR-CASES-001";
const CLAIM_BOUNDARY: &str = "controlled_single_node_alpha";
const SUPPORT_DEMO_SOURCE: &str = include_str!("demo_05_ai_support_resolution_desk.rs");
const SOURCE_CONTRACT_MARKERS: &[&str] = &[
    "case_action_ready(resolution) <- candidate_resolution(resolution), resolution_policy_approved(resolution), resolution_confident(resolution), resolution_has_retrieved_evidence(resolution), not resolution_blocked(resolution), not resolution_suppressed(resolution), resolution_case(resolution, case), case_status(case, \"open\"), not case_claimed(case)",
    "case_resolution_selected(resolution, case, owner, epoch) <- candidate_resolution(resolution), resolution_policy_approved(resolution), resolution_confident(resolution), resolution_has_retrieved_evidence(resolution), not resolution_blocked(resolution), not resolution_suppressed(resolution), resolution_case(resolution, case), case_status(case, \"open\"), active_assignment(case, owner, epoch)",
];

#[derive(Debug, Deserialize)]
struct CasePack {
    schema_version: String,
    case_set_id: String,
    protocol_id: String,
    data_classification: String,
    cases: Vec<PilotCase>,
}

#[derive(Debug, Deserialize)]
struct PilotCase {
    case_id: String,
    semantic_facts: Vec<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let case_id = args.next().ok_or(
        "usage: cargo run -p aether_api --example support_desk_real_operator_pilot_case --release -- <case-id> [sqlite-path]",
    )?;
    let db_path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/pilot/support-desk-real-operator.sqlite"));

    verify_source_contract()?;
    let revision = git_revision()?;
    let pack: CasePack = serde_json::from_str(&fs::read_to_string(FIXTURE_PATH)?)?;
    verify_pack(&pack)?;
    let case = pack
        .cases
        .iter()
        .find(|candidate| candidate.case_id == case_id)
        .ok_or_else(|| format!("unknown frozen pilot case: {case_id}"))?;
    verify_semantic_facts(case)?;

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut service = SqliteKernelService::open(&db_path)?;

    println!("AETHER Support Desk — bounded real-operator pilot");
    println!("=================================================");
    println!("Protocol: {PROTOCOL_ID}");
    println!("Case set: {CASE_SET_ID}");
    println!("Case: {}", case.case_id);
    println!("AETHER revision: {revision}");
    println!("Journal backend: sqlite");
    println!("Claim boundary: {CLAIM_BOUNDARY}");
    println!("Data classification: synthetic");
    println!("Ground truth is intentionally not loaded by this executable.");

    let active = run_query(&mut service, case, "goal active_case(case)\n  keep case")?;
    print_rows("1. Active support cases now", query_rows(&active));

    let evidence = run_query(
        &mut service,
        case,
        "goal retrieved_evidence(case, evidence)\n  keep case, evidence",
    )?;
    print_rows("2. Retrieved evidence", query_rows(&evidence));

    let ready = run_query(
        &mut service,
        case,
        "goal ready_resolution(case, resolution)\n  keep case, resolution",
    )?;
    print_rows("3. Resolutions actually ready", query_rows(&ready));

    let owner = run_query(
        &mut service,
        case,
        "goal active_assignment(case, owner, epoch)\n  keep case, owner, epoch",
    )?;
    print_rows("4a. Current assignment", query_rows(&owner));

    let stale = run_query(
        &mut service,
        case,
        "goal stale_assignment(case, owner, epoch)\n  keep case, owner, epoch",
    )?;
    print_rows("4b. Historical/stale assignment", query_rows(&stale));

    let selected = run_query(
        &mut service,
        case,
        "goal selected_resolution(case, resolution, owner, epoch)\n  keep case, resolution, owner, epoch",
    )?;
    print_rows("5. Current selected resolution", query_rows(&selected));

    print_decision_inputs(case);
    if !print_trace(&mut service, &selected)? {
        let _ = print_trace(&mut service, &ready)?;
    }

    println!();
    println!("Operator task: answer the five frozen pilot questions from the surfaces above.");
    println!("Do not enter customer/private production data or execute any real external action.");
    Ok(())
}

fn verify_source_contract() -> Result<(), Box<dyn std::error::Error>> {
    for marker in SOURCE_CONTRACT_MARKERS {
        if !SUPPORT_DEMO_SOURCE.contains(marker) {
            return Err(
                format!("qualified support-desk source contract marker missing: {marker}").into(),
            );
        }
    }
    Ok(())
}

fn git_revision() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !output.status.success() {
        return Err("cannot establish exact git revision".into());
    }
    let revision = String::from_utf8(output.stdout)?.trim().to_owned();
    if revision.len() != 40 || !revision.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("invalid git revision: {revision}").into());
    }
    Ok(revision)
}

fn verify_pack(pack: &CasePack) -> Result<(), Box<dyn std::error::Error>> {
    if pack.schema_version != "aether.support-desk-real-operator-case-pack.v1"
        || pack.case_set_id != CASE_SET_ID
        || pack.protocol_id != PROTOCOL_ID
        || pack.data_classification != "synthetic"
        || pack.cases.len() != 6
    {
        return Err("pilot case pack identity/data boundary mismatch".into());
    }
    Ok(())
}

fn verify_semantic_facts(case: &PilotCase) -> Result<(), Box<dyn std::error::Error>> {
    let arities = [
        ("case_status", 3usize),
        ("retrieved_evidence", 3),
        ("candidate_resolution", 3),
        ("resolution_policy_approval", 4),
        ("resolution_confidence", 4),
        ("resolution_suppression", 4),
        ("resolution_dependency", 4),
        ("dependency_status", 4),
        ("case_assignment", 4),
        ("active_assignment", 4),
    ];
    for fact in &case.semantic_facts {
        let predicate = fact.first().ok_or("empty semantic fact")?;
        let expected = arities
            .iter()
            .find(|(name, _)| name == predicate)
            .map(|(_, arity)| *arity)
            .ok_or_else(|| format!("unsupported pilot predicate: {predicate}"))?;
        if fact.len() != expected {
            return Err(format!("bad arity for {predicate}: {} != {expected}", fact.len()).into());
        }
    }
    Ok(())
}

fn run_query(
    service: &mut SqliteKernelService,
    case: &PilotCase,
    query_body: &str,
) -> Result<aether_api::RunDocumentResponse, Box<dyn std::error::Error>> {
    service
        .run_document(RunDocumentRequest {
            dsl: pilot_dsl(case, query_body),
            policy_context: None,
        })
        .map_err(Into::into)
}

fn pilot_dsl(case: &PilotCase, query_body: &str) -> String {
    let facts = case
        .semantic_facts
        .iter()
        .map(|fact| {
            let predicate = &fact[0];
            let args = fact[1..]
                .iter()
                .map(|value| format!("\"{}\"", escape(value)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("  {predicate}({args})")
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"
schema v1 {{}}

predicates {{
  case_status(String, String)
  retrieved_evidence(String, String)
  candidate_resolution(String, String)
  resolution_policy_approval(String, String, String)
  resolution_confidence(String, String, String)
  resolution_suppression(String, String, String)
  resolution_dependency(String, String, String)
  dependency_status(String, String, String)
  case_assignment(String, String, String)
  active_assignment(String, String, String)
  active_case(String)
  resolution_dependency_closure(String, String, String)
  dependency_complete(String, String)
  resolution_blocked(String, String)
  resolution_policy_approved(String, String)
  resolution_suppressed(String, String)
  resolution_confident(String, String)
  resolution_has_retrieved_evidence(String, String)
  case_claimed(String)
  ready_resolution(String, String)
  selected_resolution(String, String, String, String)
  stale_assignment(String, String, String)
}}

facts {{
{facts}
}}

rules {{
  active_case(case) <- case_status(case, "open")
  resolution_dependency_closure(case, resolution, dep) <- resolution_dependency(case, resolution, dep)
  resolution_dependency_closure(case, resolution, dep) <- resolution_dependency(case, resolution, mid), resolution_dependency_closure(case, mid, dep)
  dependency_complete(case, dep) <- dependency_status(case, dep, "complete")
  resolution_blocked(case, resolution) <- resolution_dependency_closure(case, resolution, dep), not dependency_complete(case, dep)
  resolution_policy_approved(case, resolution) <- resolution_policy_approval(case, resolution, "approved")
  resolution_suppressed(case, resolution) <- resolution_suppression(case, resolution, "suppressed")
  resolution_confident(case, resolution) <- resolution_confidence(case, resolution, "high")
  resolution_has_retrieved_evidence(case, resolution) <- candidate_resolution(case, resolution), retrieved_evidence(case, evidence)
  case_claimed(case) <- active_assignment(case, owner, epoch)
  ready_resolution(case, resolution) <- candidate_resolution(case, resolution), resolution_policy_approved(case, resolution), resolution_confident(case, resolution), resolution_has_retrieved_evidence(case, resolution), not resolution_blocked(case, resolution), not resolution_suppressed(case, resolution), case_status(case, "open"), not case_claimed(case)
  selected_resolution(case, resolution, owner, epoch) <- candidate_resolution(case, resolution), resolution_policy_approved(case, resolution), resolution_confident(case, resolution), resolution_has_retrieved_evidence(case, resolution), not resolution_blocked(case, resolution), not resolution_suppressed(case, resolution), case_status(case, "open"), active_assignment(case, owner, epoch)
  stale_assignment(case, owner, epoch) <- case_assignment(case, owner, epoch), not active_assignment(case, owner, epoch)
}}

materialize {{
  active_case
  resolution_dependency_closure
  dependency_complete
  resolution_blocked
  resolution_policy_approved
  resolution_suppressed
  resolution_confident
  resolution_has_retrieved_evidence
  case_claimed
  ready_resolution
  selected_resolution
  stale_assignment
}}

query {{
  current
  {query_body}
}}
"#
    )
}

fn query_rows(response: &aether_api::RunDocumentResponse) -> &[QueryRow] {
    response
        .query
        .as_ref()
        .expect("pilot query should exist")
        .rows
        .as_slice()
}

fn print_rows(title: &str, rows: &[QueryRow]) {
    println!();
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
    if rows.is_empty() {
        println!("  - none");
    } else {
        for row in rows {
            println!("  - {}", format_values(&row.values));
        }
    }
}

fn print_decision_inputs(case: &PilotCase) {
    println!();
    println!("Decision inputs (non-derived)");
    println!("-----------------------------");
    for fact in &case.semantic_facts {
        if matches!(
            fact[0].as_str(),
            "case_status"
                | "resolution_policy_approval"
                | "resolution_confidence"
                | "resolution_suppression"
                | "resolution_dependency"
                | "dependency_status"
        ) {
            println!("  - {}({})", fact[0], fact[1..].join(", "));
        }
    }
}

fn print_trace(
    service: &mut SqliteKernelService,
    response: &aether_api::RunDocumentResponse,
) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(tuple_id) = query_rows(response).first().and_then(|row| row.tuple_id) else {
        return Ok(false);
    };
    let Some(receipt) = response.execution.as_ref() else {
        return Ok(false);
    };
    let Some(binding) = receipt
        .trace_handles
        .iter()
        .find(|candidate| candidate.local_tuple_id == tuple_id)
    else {
        return Ok(false);
    };
    let trace = service
        .resolve_trace_handle(ResolveTraceHandleRequest {
            handle: binding.handle.clone(),
            policy_context: None,
            verify_replay: true,
        })?
        .record
        .trace;
    println!();
    println!("Proof/provenance trace");
    println!("----------------------");
    println!("  - root tuple: t{}", trace.root.0);
    println!("  - tuples in trace: {}", trace.tuples.len());
    for tuple in trace.tuples.iter().take(12) {
        println!(
            "  - t{} via r{} -> {}",
            tuple.tuple.id.0,
            tuple.metadata.rule_id.0,
            format_values(&tuple.tuple.values)
        );
    }
    Ok(true)
}

fn format_values(values: &[Value]) -> String {
    values
        .iter()
        .map(format_value)
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(value) => value.to_string(),
        Value::I64(value) => value.to_string(),
        Value::U64(value) => value.to_string(),
        Value::F64(value) => format!("{value:.4}"),
        Value::String(value) => value.clone(),
        Value::Bytes(value) => format!("<{} bytes>", value.len()),
        Value::Entity(id) => format!("entity({})", id.0),
        Value::List(values) => format!("[{}]", format_values(values)),
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
