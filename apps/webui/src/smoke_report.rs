use crate::SmokeRunResponse;

const REQUIRED_EVIDENCE: [&str; 6] = [
    "dir_listed",
    "git_status_checked",
    "git_diff_checked",
    "file_written",
    "file_read",
    "file_deleted",
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SmokeReport {
    pub run_id: String,
    pub verified: bool,
    pub missing_evidence: Vec<String>,
    pub final_answer: String,
}

pub fn smoke_report(response: &SmokeRunResponse) -> SmokeReport {
    let missing = REQUIRED_EVIDENCE
        .iter()
        .filter(|key| !response.evidence.iter().any(|evidence| evidence == **key))
        .map(|key| key.to_string())
        .collect::<Vec<_>>();
    let tools_ok = response.tool_results.iter().all(|tool| tool.ok);
    let verified = response.ok && response.state == "final" && missing.is_empty() && tools_ok;
    SmokeReport {
        run_id: response.run_id.clone(),
        verified,
        missing_evidence: missing,
        final_answer: final_answer(response, verified),
    }
}

pub fn smoke_ledger_json(response: &SmokeRunResponse) -> String {
    let report = smoke_report(response);
    format!(
        "{{\"run_id\":{},\"verified\":{},\"missing_evidence\":[{}],\"final_answer\":{},\"evidence\":[{}],\"tool_results\":[{}]}}",
        quoted(&report.run_id),
        report.verified,
        quoted_list(&report.missing_evidence),
        quoted(&report.final_answer),
        quoted_list(&response.evidence),
        tool_results_json(response)
    )
}

pub fn smoke_report_html(response: &SmokeRunResponse) -> String {
    let report = smoke_report(response);
    let missing = if report.missing_evidence.is_empty() {
        "none".to_string()
    } else {
        report.missing_evidence.join(", ")
    };
    format!(
        "<section class=\"run-card\"><h2>Final report</h2><p><strong>Run:</strong> {}</p><p><strong>Verified:</strong> {}</p><p><strong>Missing evidence:</strong> {}</p><pre>{}</pre></section>",
        escape_html(&report.run_id),
        report.verified,
        escape_html(&missing),
        escape_html(&report.final_answer)
    )
}

fn final_answer(response: &SmokeRunResponse, verified: bool) -> String {
    if verified {
        format!(
            "I inspected the workspace, checked git status and git diff, created agent-smoke.txt, read it back, deleted it, and verified the required evidence in run {}.",
            response.run_id
        )
    } else {
        format!(
            "The smoke run {} is not verified. I will not claim completion without the required evidence.",
            response.run_id
        )
    }
}

fn tool_results_json(response: &SmokeRunResponse) -> String {
    response
        .tool_results
        .iter()
        .map(|tool| format!("{{\"tool\":{},\"ok\":{}}}", quoted(&tool.tool), tool.ok))
        .collect::<Vec<_>>()
        .join(",")
}

fn quoted_list(values: &[String]) -> String {
    values.iter().map(|value| quoted(value)).collect::<Vec<_>>().join(",")
}

fn quoted(value: &str) -> String {
    format!("\"{}\"", escape_json(value))
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SmokeRunResponse, ToolResultView};

    fn response(evidence: Vec<&str>) -> SmokeRunResponse {
        SmokeRunResponse {
            run_id: "run-1".to_string(),
            state: "final".to_string(),
            ok: true,
            evidence: evidence.into_iter().map(str::to_string).collect(),
            tool_results: vec![ToolResultView { tool: "write_file".to_string(), ok: true }],
        }
    }

    #[test]
    fn complete_evidence_verifies_final_answer() {
        let report = smoke_report(&response(REQUIRED_EVIDENCE.to_vec()));
        assert!(report.verified);
        assert!(report.missing_evidence.is_empty());
        assert!(report.final_answer.contains("created agent-smoke.txt"));
    }

    #[test]
    fn missing_evidence_blocks_completion_claim() {
        let report = smoke_report(&response(vec!["file_written"]));
        assert!(!report.verified);
        assert!(report.missing_evidence.contains(&"file_deleted".to_string()));
        assert!(report.final_answer.contains("not verified"));
    }

    #[test]
    fn ledger_json_contains_report_and_tools() {
        let json = smoke_ledger_json(&response(REQUIRED_EVIDENCE.to_vec()));
        assert!(json.contains("\"verified\":true"));
        assert!(json.contains("\"final_answer\""));
        assert!(json.contains("write_file"));
    }
}
