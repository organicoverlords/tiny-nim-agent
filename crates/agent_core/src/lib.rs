use tiny_proof::{verify_required_evidence, ProofError, RunId, RunLedger};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AgentError {
    NoObjectives,
    ZeroTurnBudget,
    TurnBudgetExceeded,
    UnknownObjective { id: String },
    OpenObjectivesRemain,
    Proof(ProofError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_proof::ProofEvent;

    fn session() -> Session {
        let run_id = RunId::new("run-1").unwrap();
        let plan = SessionPlan::new(vec![Objective::new(
            "smoke",
            vec!["file_written".to_string()],
        )])
        .unwrap();
        Session::new(run_id, plan, 3).unwrap()
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
    fn session_fails_when_turn_budget_is_exceeded() {
        let mut session = session();
        session.advance(SessionState::Planning).unwrap();
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
