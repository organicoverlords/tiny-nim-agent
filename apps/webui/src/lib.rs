pub mod http;

use std::path::Path;
use tiny_agent_core::{run_first_smoke_dry_run, AgentError, SessionState};
use tiny_proof::{ProofEvent, RunLedger};
use tiny_tools::WorkspaceRoot;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebUiTarget {
    pub ui_style_reference: &'static str,
    pub agent_reference: &'static str,
}

impl WebUiTarget {
    pub fn chatgpt_with_opencode_loop() -> Self {
        Self {
            ui_style_reference: "ChatGPT-like chat surface",
            agent_reference: "OpenCode-like coding loop",
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} with {}",
            self.ui_style_reference, self.agent_reference
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SmokeRunResponse {
    pub run_id: String,
    pub state: String,
    pub ok: bool,
    pub evidence: Vec<String>,
    pub tool_results: Vec<ToolResultView>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToolResultView {
    pub tool: String,
    pub ok: bool,
}

impl SmokeRunResponse {
    pub fn from_ledger(state: &SessionState, ledger: &RunLedger) -> Self {
        Self {
            run_id: ledger.run_id().as_str().to_string(),
            state: session_state_name(state).to_string(),
            ok: matches!(state, SessionState::Final),
            evidence: evidence_keys(ledger),
            tool_results: tool_results(ledger),
        }
    }
}

pub fn run_smoke_for_workspace(root: impl AsRef<Path>) -> Result<SmokeRunResponse, WebUiError> {
    let workspace = WorkspaceRoot::new(root.as_ref().to_path_buf()).map_err(WebUiError::Tool)?;
    let session = run_first_smoke_dry_run(&workspace).map_err(WebUiError::Agent)?;
    Ok(SmokeRunResponse::from_ledger(&session.state, &session.ledger))
}

fn session_state_name(state: &SessionState) -> &'static str {
    match state {
        SessionState::Queued => "queued",
        SessionState::Planning => "planning",
        SessionState::ModelTurn => "model_turn",
        SessionState::ToolTurn => "tool_turn",
        SessionState::Verifying => "verifying",
        SessionState::Final => "final",
        SessionState::Failed => "failed",
    }
}

fn evidence_keys(ledger: &RunLedger) -> Vec<String> {
    let mut keys = ledger
        .events()
        .iter()
        .filter_map(|event| match event {
            ProofEvent::Evidence { key, .. } => Some(key.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

fn tool_results(ledger: &RunLedger) -> Vec<ToolResultView> {
    ledger
        .events()
        .iter()
        .filter_map(|event| match event {
            ProofEvent::ToolResult { tool, ok, .. } => Some(ToolResultView {
                tool: tool.clone(),
                ok: *ok,
            }),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
pub enum WebUiError {
    Agent(AgentError),
    Tool(tiny_tools::ToolError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn git_workspace_path() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("tiny-webui-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        let status = Command::new("git")
            .args(["init"])
            .current_dir(&root)
            .status()
            .unwrap();
        assert!(status.success());
        root
    }

    #[test]
    fn target_names_reference_behavior() {
        let target = WebUiTarget::chatgpt_with_opencode_loop();
        assert!(target.summary().contains("ChatGPT-like"));
        assert!(target.summary().contains("OpenCode-like"));
    }

    #[test]
    fn smoke_response_exposes_final_state_and_proof() {
        let root = git_workspace_path();
        let response = run_smoke_for_workspace(&root).unwrap();

        assert!(response.ok);
        assert_eq!(response.state, "final");
        assert_eq!(response.run_id, "dry-run-first-smoke");
        assert!(response.evidence.contains(&"dir_listed".to_string()));
        assert!(response.evidence.contains(&"git_status_checked".to_string()));
        assert!(response.evidence.contains(&"git_diff_checked".to_string()));
        assert!(response.evidence.contains(&"file_written".to_string()));
        assert!(response.evidence.contains(&"file_read".to_string()));
        assert!(response.evidence.contains(&"file_deleted".to_string()));
        assert_eq!(response.tool_results.len(), 6);
        assert!(response.tool_results.iter().all(|result| result.ok));
        assert!(!root.join("agent-smoke.txt").exists());
    }
}
