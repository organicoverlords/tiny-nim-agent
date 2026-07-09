use crate::{ToolError, WorkspaceRoot};
use std::process::Command;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GitOutput {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn git_status(workspace: &WorkspaceRoot) -> Result<GitOutput, ToolError> {
    run_git(workspace, &["status", "--short"])
}

pub fn git_diff(workspace: &WorkspaceRoot) -> Result<GitOutput, ToolError> {
    run_git(workspace, &["diff", "--"])
}

fn run_git(workspace: &WorkspaceRoot, args: &[&str]) -> Result<GitOutput, ToolError> {
    let output = Command::new("git")
        .args(args)
        .current_dir(workspace.root_path())
        .output()
        .map_err(ToolError::Io)?;
    Ok(GitOutput {
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_git_workspace() -> WorkspaceRoot {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("tiny-git-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        let workspace = WorkspaceRoot::new(root).unwrap();
        run_git(&workspace, &["init"]).unwrap();
        workspace
    }

    #[test]
    fn git_status_reports_untracked_file() {
        let workspace = temp_git_workspace();
        fs::write(workspace.root_path().join("README.md"), "hello").unwrap();
        let output = git_status(&workspace).unwrap();
        assert_eq!(output.exit_code, Some(0));
        assert!(output.stdout.contains("README.md"));
    }

    #[test]
    fn git_diff_returns_success_for_repo() {
        let workspace = temp_git_workspace();
        let output = git_diff(&workspace).unwrap();
        assert_eq!(output.exit_code, Some(0));
    }
}
