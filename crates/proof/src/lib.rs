#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RunId(String);

impl RunId {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ProofError::EmptyRunId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProofEvent {
    RunStarted { run_id: RunId },
    ModelRoute { run_id: RunId, model: String },
    ToolResult { run_id: RunId, tool: String, ok: bool },
    Evidence { run_id: RunId, key: String },
    FinalClaimChecked { run_id: RunId, ok: bool },
}

impl ProofEvent {
    pub fn run_id(&self) -> &RunId {
        match self {
            Self::RunStarted { run_id }
            | Self::ModelRoute { run_id, .. }
            | Self::ToolResult { run_id, .. }
            | Self::Evidence { run_id, .. }
            | Self::FinalClaimChecked { run_id, .. } => run_id,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RunLedger {
    run_id: RunId,
    events: Vec<ProofEvent>,
}

impl RunLedger {
    pub fn new(run_id: RunId) -> Self {
        let mut ledger = Self {
            run_id: run_id.clone(),
            events: Vec::new(),
        };
        ledger.push(ProofEvent::RunStarted { run_id });
        ledger
    }

    pub fn run_id(&self) -> &RunId {
        &self.run_id
    }

    pub fn push(&mut self, event: ProofEvent) {
        if event.run_id() == &self.run_id {
            self.events.push(event);
        }
    }

    pub fn has_evidence(&self, key: &str) -> bool {
        self.events.iter().any(|event| {
            matches!(event, ProofEvent::Evidence { key: event_key, .. } if event_key == key)
        })
    }

    pub fn events(&self) -> &[ProofEvent] {
        &self.events
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProofError {
    EmptyRunId,
    MissingEvidence { key: String },
}

pub fn verify_required_evidence(
    ledger: &RunLedger,
    required: &[&str],
) -> Result<(), ProofError> {
    for key in required {
        if !ledger.has_evidence(key) {
            return Err(ProofError::MissingEvidence {
                key: (*key).to_string(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_id_cannot_be_empty() {
        assert_eq!(RunId::new(" ").unwrap_err(), ProofError::EmptyRunId);
    }

    #[test]
    fn ledger_exposes_run_id() {
        let run_id = RunId::new("run-1").unwrap();
        let ledger = RunLedger::new(run_id.clone());
        assert_eq!(ledger.run_id(), &run_id);
    }

    #[test]
    fn ledger_keeps_only_same_run_events() {
        let run_id = RunId::new("run-1").unwrap();
        let other = RunId::new("run-2").unwrap();
        let mut ledger = RunLedger::new(run_id.clone());

        ledger.push(ProofEvent::Evidence {
            run_id: run_id.clone(),
            key: "file_read".to_string(),
        });
        ledger.push(ProofEvent::Evidence {
            run_id: other,
            key: "wrong_run".to_string(),
        });

        assert_eq!(ledger.events().len(), 2);
        assert!(ledger.has_evidence("file_read"));
        assert!(!ledger.has_evidence("wrong_run"));
    }

    #[test]
    fn verifier_rejects_missing_evidence() {
        let run_id = RunId::new("run-1").unwrap();
        let ledger = RunLedger::new(run_id);
        let err = verify_required_evidence(&ledger, &["file_deleted"]).unwrap_err();
        assert_eq!(err, ProofError::MissingEvidence { key: "file_deleted".into() });
    }

    #[test]
    fn verifier_accepts_present_evidence() {
        let run_id = RunId::new("run-1").unwrap();
        let mut ledger = RunLedger::new(run_id.clone());
        ledger.push(ProofEvent::Evidence {
            run_id,
            key: "file_deleted".to_string(),
        });
        assert!(verify_required_evidence(&ledger, &["file_deleted"]).is_ok());
    }
}
