use tiny_model_contract::ToolCall;
use tiny_proof::{verify_required_evidence, ProofError, ProofEvent, RunId, RunLedger};
use tiny_tools::{ToolError, WorkspaceRoot};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SessionState {
    Queued,
    Planning,
    ModelTurn,
    ToolTurn,
    Verifying,
    Final,
    Failed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Objective {
    pub id: String,
    pub required_evidence: Vec<String>,
    completed: bool,
}

impl Objective {
    pub fn new(id: impl Into<String>, required_evidence: Vec<String>) -> Self {
        Self {
            id: id.into(),
            required_evidence,
            completed: false,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SessionPlan {
    objectives: Vec<Objective>,
}

impl SessionPlan {
    pub fn new(objectives: Vec<Objective>) -> Result<Self, AgentError> {
        if objectives.is_empty() {
            return Err(AgentError::NoObjectives);
        }
        Ok(Self { objectives })
    }

    pub fn next_open(&self) -> Option<&Objective> {
        self.objectives.iter().find(|objective| !objective.completed)
    }

    pub fn mark_completed(&mut self, id: &str) -> Result<(), AgentError> {
        let objective = self
            .objectives
            .iter_mut()
            .find(|objective| objective.id == id)
            .ok_or_else(|| AgentError::UnknownObjective { id: id.to_string() })?;
        objective.completed = true;
        Ok(())
    }

    pub fn all_completed(&self) -> bool {
        self.objectives.iter().all(Objective::is_completed)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Session {
    pub state: SessionState,
    pub plan: SessionPlan,
    pub ledger: RunLedger,
    turn_count: u32,
    max_turns: u32,
}

impl Session {
    pub fn new(run_id: RunId, plan: SessionPlan, max_turns: u32) -> Result<Self, AgentError> {
        if max_turns == 0 {
            return Err(AgentError::ZeroTurnBudget);
        }
        Ok(Self {
            state: SessionState::Queued,
            plan,
            ledger: RunLedger::new(run_id),
            turn_count: 0,
            max_turns,
        })
    }

    pub fn advance(&mut self, next: SessionState) -> Result<(), AgentError> {
        self.turn_count = self.turn_count.saturating_add(1);
        if self.turn_count > self.max_turns {
            self.state = SessionState::Failed;
            return Err(AgentError::TurnBudgetExceeded);
        }
        self.state = next;
        Ok(())
    }

    pub fn execute_tool_call(
        &mut self,
        workspace: &WorkspaceRoot,
        call: &ToolCall,
    ) -> Result<String, AgentError> {
        self.advance(SessionState::ToolTurn)?;
        let run_id = self.ledger.run_id().clone();
        let outcome = execute_workspace_tool(workspace, call);
        self.ledger.push(ProofEvent::ToolResult {
            run_id: run_id.clone(),
            tool: call.name.clone(),
            ok: outcome.is_ok(),
        });
        if outcome.is_ok() {
            self.ledger.push(ProofEvent::Evidence {
                run_id,
                key: evidence_key(&call.name).to_string(),
            });
        }
        outcome
    }

    pub fn verify_objective(&mut self, id: &str) -> Result<(), AgentError> {
        let required = self
            .plan
            .objectives
            .iter()
            .find(|objective| objective.id == id)
            .ok_or_else(|| AgentError::UnknownObjective { id: id.to_string() })?
            .required_evidence
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();

        verify_required_evidence(&self.ledger, &required).map_err(AgentError::Proof)?;
        self.plan.mark_completed(id)
    }

    pub fn finalize_if_complete(&mut self) -> Result<(), AgentError> {
        if self.plan.all_completed() {
            self.state = SessionState::Final;
            Ok(())
        } else {
            Err(AgentError::OpenObjectivesRemain)
        }
    }
}

fn execute_workspace_tool(workspace: &WorkspaceRoot, call: &ToolCall) -> Result<String, AgentError> {
    match call.name.as_str() {
        "read_file" => {
            let path = json_arg(&call.args_json, "path")?;
            workspace.read_file(path).map_err(AgentError::Tool)
        }
        "write_file" => {
            let path = json_arg(&call.args_json, "path")?;
            let content = json_arg(&call.args_json, "content")?;
            workspace.write_file(path, &content).map_err(AgentError::Tool)?;
            Ok(String::new())
        }
        "delete_file" => {
            let path = json_arg(&call.args_json, "path")?;
            workspace.delete_file(path).map_err(AgentError::Tool)?;
            Ok(String::new())
        }
        "list_dir" => {
            let path = json_arg(&call.args_json, "path")?;
            workspace
                .list_dir(path)
                .map(|entries| entries.join("\n"))
                .map_err(AgentError::Tool)
        }
        other => Err(AgentError::UnknownTool { name: other.to_string() }),
    }
}

fn evidence_key(tool_name: &str) -> &str {
    match tool_name {
        "read_file" => "file_read",
        "write_file" => "file_written",
        "delete_file" => "file_deleted",
        "list_dir" => "dir_listed",
        _ => "tool_completed",
    }
}

fn json_arg(json: &str, key: &str) -> Result<String, AgentError> {
    let needle = format!("\"{}\"", key);
    let start = json.find(&needle).ok_or_else(|| AgentError::MissingArg { key: key.into() })?;
    let after_key = &json[start + needle.len()..];
    let colon = after_key.find(':').ok_or_else(|| AgentError::MissingArg { key: key.into() })?;
    let after_colon = after_key[colon + 1..].trim_start();
    let value = after_colon
        .strip_prefix('"')
        .ok_or_else(|| AgentError::MissingArg { key: key.into() })?;
    let end = value.find('"').ok_or_else(|| AgentError::MissingArg { key: key.into() })?;
    Ok(value[..end].to_string())
}

#[derive(Debug, Eq, PartialEq)]
pub enum AgentError {
    NoObjectives,
    ZeroTurnBudget,
    TurnBudgetExceeded,
    UnknownObjective { id: String },
    UnknownTool { name: String },
    MissingArg { key: String },
    OpenObjectivesRemain,
    Proof(ProofError),
    Tool(ToolError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tiny_proof::ProofEvent;

    fn session() -> Session {
        let run_id = RunId::new("run-1").unwrap();
        let plan = SessionPlan::new(vec![Objective::new(
            "smoke",
            vec!["file_written".to_string()],
        )])
        .unwrap();
        Session::new(run_id, plan, 6).unwrap()
    }

    fn workspace() -> WorkspaceRoot {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("tiny-agent-core-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        WorkspaceRoot::new(root).unwrap()
    }

    #[test]
    fn plan_requires_objectives() {
        assert_eq!(SessionPlan::new(Vec::new()).unwrap_err(), AgentError::NoObjectives);
    }

    #[test]
    fn session_advances_state_until_budget() {
        let mut session = session();
        session.advance(SessionState::Planning).unwrap();
        assert_eq!(session.state, SessionState::Planning);
    }

    #[test]
    fn executes_write_read_delete_tool_calls_with_evidence() {
        let mut session = session();
        let workspace = workspace();
        session
            .execute_tool_call(
                &workspace,
                &ToolCall {
                    name: "write_file".to_string(),
                    args_json: "{\"path\":\"agent-smoke.txt\",\"content\":\"ok\"}".to_string(),
                },
            )
            .unwrap();
        let read_back = session
            .execute_tool_call(
                &workspace,
                &ToolCall {
                    name: "read_file".to_string(),
                    args_json: "{\"path\":\"agent-smoke.txt\"}".to_string(),
                },
            )
            .unwrap();
        session
            .execute_tool_call(
                &workspace,
                &ToolCall {
                    name: "delete_file".to_string(),
                    args_json: "{\"path\":\"agent-smoke.txt\"}".to_string(),
                },
            )
            .unwrap();

        assert_eq!(read_back, "ok");
        assert!(session.ledger.has_evidence("file_written"));
        assert!(session.ledger.has_evidence("file_read"));
        assert!(session.ledger.has_evidence("file_deleted"));
    }

    #[test]
    fn session_fails_when_turn_budget_is_exceeded() {
        let mut session = session();
        session.advance(SessionState::Planning).unwrap();
        session.advance(SessionState::ModelTurn).unwrap();
        session.advance(SessionState::ToolTurn).unwrap();
        session.advance(SessionState::Verifying).unwrap();
        session.advance(SessionState::ModelTurn).unwrap();
        session.advance(SessionState::ToolTurn).unwrap();
        let err = session.advance(SessionState::Verifying).unwrap_err();
        assert_eq!(err, AgentError::TurnBudgetExceeded);
        assert_eq!(session.state, SessionState::Failed);
    }

    #[test]
    fn objective_requires_ledger_evidence() {
        let mut session = session();
        let err = session.verify_objective("smoke").unwrap_err();
        assert_eq!(
            err,
            AgentError::Proof(ProofError::MissingEvidence {
                key: "file_written".to_string()
            })
        );
    }

    #[test]
    fn objective_completes_when_evidence_exists() {
        let mut session = session();
        let run_id = RunId::new("run-1").unwrap();
        session.ledger.push(ProofEvent::Evidence {
            run_id,
            key: "file_written".to_string(),
        });
        session.verify_objective("smoke").unwrap();
        session.finalize_if_complete().unwrap();
        assert_eq!(session.state, SessionState::Final);
    }
}
