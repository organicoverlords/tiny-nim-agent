use crate::smoke_report::smoke_ledger_json;
use crate::SmokeRunResponse;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SmokeProofArtifact {
    pub run_id: String,
    pub path: PathBuf,
    pub json: String,
}

pub fn write_smoke_proof_artifact(
    workspace: &Path,
    response: &SmokeRunResponse,
) -> Result<SmokeProofArtifact, std::io::Error> {
    let proof_dir = workspace.join(".tiny-nim-agent").join("proofs");
    fs::create_dir_all(&proof_dir)?;
    let path = proof_dir.join(format!("{}.json", safe_run_id(&response.run_id)));
    let json = smoke_ledger_json(response);
    fs::write(&path, &json)?;
    Ok(SmokeProofArtifact {
        run_id: response.run_id.clone(),
        path,
        json,
    })
}

pub fn artifact_response_json(artifact: &SmokeProofArtifact) -> String {
    format!(
        "{{\"run_id\":{},\"proof_path\":{},\"proof_json\":{}}}",
        quoted(&artifact.run_id),
        quoted(&artifact.path.display().to_string()),
        artifact.json
    )
}

fn safe_run_id(run_id: &str) -> String {
    run_id
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '_' })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SmokeRunResponse, ToolResultView};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn workspace() -> PathBuf {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("tiny-webui-proof-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn response() -> SmokeRunResponse {
        SmokeRunResponse {
            run_id: "run-1".to_string(),
            state: "final".to_string(),
            ok: true,
            evidence: vec![
                "dir_listed".to_string(),
                "git_status_checked".to_string(),
                "git_diff_checked".to_string(),
                "file_written".to_string(),
                "file_read".to_string(),
                "file_deleted".to_string(),
            ],
            tool_results: vec![ToolResultView { tool: "write_file".to_string(), ok: true }],
        }
    }

    #[test]
    fn writes_proof_file_under_workspace() {
        let root = workspace();
        let artifact = write_smoke_proof_artifact(&root, &response()).unwrap();
        let content = fs::read_to_string(&artifact.path).unwrap();

        assert!(artifact.path.ends_with(".tiny-nim-agent/proofs/run-1.json"));
        assert!(content.contains("\"verified\":true"));
        assert!(content.contains("\"run_id\":\"run-1\""));
    }

    #[test]
    fn artifact_response_contains_path_and_json() {
        let root = workspace();
        let artifact = write_smoke_proof_artifact(&root, &response()).unwrap();
        let json = artifact_response_json(&artifact);

        assert!(json.contains("\"proof_path\""));
        assert!(json.contains("\"proof_json\""));
        assert!(json.contains("\"verified\":true"));
    }
}
