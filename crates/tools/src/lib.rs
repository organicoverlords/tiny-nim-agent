pub mod git;
pub use git::{git_diff, git_status, GitOutput};

use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkspaceRoot {
    root: PathBuf,
}

impl WorkspaceRoot {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, ToolError> {
        let root = root.into();
        if root.as_os_str().is_empty() {
            return Err(ToolError::EmptyWorkspaceRoot);
        }
        Ok(Self { root })
    }

    pub fn root_path(&self) -> &Path {
        &self.root
    }

    pub fn resolve_relative(&self, path: impl AsRef<Path>) -> Result<PathBuf, ToolError> {
        let path = path.as_ref();
        if path.is_absolute() {
            return Err(ToolError::AbsolutePathBlocked);
        }
        for component in path.components() {
            if matches!(component, Component::ParentDir) {
                return Err(ToolError::ParentTraversalBlocked);
            }
        }
        if is_protected(path) {
            return Err(ToolError::ProtectedPathBlocked);
        }
        Ok(self.root.join(path))
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> Result<String, ToolError> {
        let resolved = self.resolve_relative(path)?;
        fs::read_to_string(resolved).map_err(ToolError::Io)
    }

    pub fn write_file(&self, path: impl AsRef<Path>, content: &str) -> Result<(), ToolError> {
        let resolved = self.resolve_relative(path)?;
        if let Some(parent) = resolved.parent() {
            fs::create_dir_all(parent).map_err(ToolError::Io)?;
        }
        fs::write(resolved, content).map_err(ToolError::Io)
    }

    pub fn delete_file(&self, path: impl AsRef<Path>) -> Result<(), ToolError> {
        let resolved = self.resolve_relative(path)?;
        fs::remove_file(resolved).map_err(ToolError::Io)
    }

    pub fn list_dir(&self, path: impl AsRef<Path>) -> Result<Vec<String>, ToolError> {
        let resolved = self.resolve_relative(path)?;
        let mut entries = Vec::new();
        for entry in fs::read_dir(resolved).map_err(ToolError::Io)? {
            let entry = entry.map_err(ToolError::Io)?;
            let name = entry.file_name().to_string_lossy().to_string();
            if !is_hidden_or_build_dir(&name) {
                entries.push(name);
            }
        }
        entries.sort();
        Ok(entries)
    }

    pub fn run_shell(&self, command: ShellCommand) -> Result<ShellResult, ToolError> {
        if is_blocked_program(&command.program) {
            return Err(ToolError::CommandBlocked);
        }
        let mut child = Command::new(&command.program)
            .args(&command.args)
            .current_dir(&self.root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(ToolError::Io)?;

        let start = Instant::now();
        let timeout = Duration::from_millis(command.timeout_ms.max(1));
        loop {
            if child.try_wait().map_err(ToolError::Io)?.is_some() {
                let output = child.wait_with_output().map_err(ToolError::Io)?;
                return Ok(ShellResult {
                    exit_code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    timed_out: false,
                });
            }
            if start.elapsed() >= timeout {
                child.kill().map_err(ToolError::Io)?;
                let output = child.wait_with_output().map_err(ToolError::Io)?;
                return Ok(ShellResult {
                    exit_code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    timed_out: true,
                });
            }
            thread::sleep(Duration::from_millis(5));
        }
    }
}

fn is_protected(path: &Path) -> bool {
    path.components().any(|component| match component {
        Component::Normal(value) => matches!(
            value.to_string_lossy().as_ref(),
            ".git" | ".env" | "target" | "node_modules"
        ),
        _ => false,
    })
}

fn is_hidden_or_build_dir(name: &str) -> bool {
    matches!(name, ".git" | "target" | "node_modules")
}

fn is_blocked_program(program: &str) -> bool {
    matches!(program, "rm" | "del" | "format" | "shutdown" | "reboot")
}

#[derive(Debug)]
pub enum ToolError {
    EmptyWorkspaceRoot,
    AbsolutePathBlocked,
    ParentTraversalBlocked,
    ProtectedPathBlocked,
    CommandBlocked,
    Io(std::io::Error),
}

impl PartialEq for ToolError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::EmptyWorkspaceRoot, Self::EmptyWorkspaceRoot)
                | (Self::AbsolutePathBlocked, Self::AbsolutePathBlocked)
                | (Self::ParentTraversalBlocked, Self::ParentTraversalBlocked)
                | (Self::ProtectedPathBlocked, Self::ProtectedPathBlocked)
                | (Self::CommandBlocked, Self::CommandBlocked)
                | (Self::Io(_), Self::Io(_))
        )
    }
}

impl Eq for ToolError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShellCommand {
    pub program: String,
    pub args: Vec<String>,
    pub timeout_ms: u64,
}

impl ShellCommand {
    pub fn new(program: impl Into<String>, args: Vec<String>, timeout_ms: u64) -> Self {
        Self {
            program: program.into(),
            args,
            timeout_ms,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShellResult {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ToolKind {
    ReadOnly,
    Write,
    Shell,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToolSpec {
    pub name: String,
    pub kind: ToolKind,
}

impl ToolSpec {
    pub fn new(name: impl Into<String>, kind: ToolKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_workspace() -> WorkspaceRoot {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("tiny-tools-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        WorkspaceRoot::new(root).unwrap()
    }

    #[test]
    fn exposes_root_path_for_read_only_subtools() {
        let root = WorkspaceRoot::new("/repo").unwrap();
        assert_eq!(root.root_path(), Path::new("/repo"));
    }

    #[test]
    fn resolves_relative_file_inside_workspace() {
        let root = WorkspaceRoot::new("/repo").unwrap();
        let path = root.resolve_relative("README.md").unwrap();
        assert_eq!(path, PathBuf::from("/repo/README.md"));
    }

    #[test]
    fn read_write_delete_file_round_trip() {
        let root = temp_workspace();
        root.write_file("notes/agent-smoke.txt", "hello from tool")
            .unwrap();
        let content = root.read_file("notes/agent-smoke.txt").unwrap();
        assert_eq!(content, "hello from tool");
        root.delete_file("notes/agent-smoke.txt").unwrap();
        assert!(matches!(root.read_file("notes/agent-smoke.txt"), Err(ToolError::Io(_))));
    }

    #[test]
    fn list_dir_hides_build_and_git_dirs() {
        let root = temp_workspace();
        root.write_file("README.md", "readme").unwrap();
        fs::create_dir_all(root.root.join("target")).unwrap();
        fs::create_dir_all(root.root.join("node_modules")).unwrap();
        fs::create_dir_all(root.root.join(".git")).unwrap();
        let entries = root.list_dir(".").unwrap();
        assert_eq!(entries, vec!["README.md".to_string()]);
    }

    #[test]
    fn shell_command_returns_output() {
        let root = temp_workspace();
        let result = root
            .run_shell(ShellCommand::new(
                "sh",
                vec!["-c".to_string(), "printf hello".to_string()],
                1_000,
            ))
            .unwrap();
        assert_eq!(result.stdout, "hello");
        assert_eq!(result.exit_code, Some(0));
        assert!(!result.timed_out);
    }

    #[test]
    fn shell_command_times_out() {
        let root = temp_workspace();
        let result = root
            .run_shell(ShellCommand::new(
                "sh",
                vec!["-c".to_string(), "sleep 2".to_string()],
                10,
            ))
            .unwrap();
        assert!(result.timed_out);
    }

    #[test]
    fn shell_blocks_obvious_destructive_programs() {
        let root = temp_workspace();
        let err = root.run_shell(ShellCommand::new("rm", vec![], 100)).unwrap_err();
        assert_eq!(err, ToolError::CommandBlocked);
    }

    #[test]
    fn blocks_absolute_path() {
        let root = WorkspaceRoot::new("/repo").unwrap();
        let err = root.resolve_relative("/etc/passwd").unwrap_err();
        assert_eq!(err, ToolError::AbsolutePathBlocked);
    }

    #[test]
    fn blocks_parent_traversal() {
        let root = WorkspaceRoot::new("/repo").unwrap();
        let err = root.resolve_relative("../secret").unwrap_err();
        assert_eq!(err, ToolError::ParentTraversalBlocked);
    }

    #[test]
    fn blocks_protected_segments() {
        let root = WorkspaceRoot::new("/repo").unwrap();
        let err = root.resolve_relative(".git/config").unwrap_err();
        assert_eq!(err, ToolError::ProtectedPathBlocked);
    }

    #[test]
    fn records_tool_kind() {
        let spec = ToolSpec::new("read_file", ToolKind::ReadOnly);
        assert_eq!(spec.kind, ToolKind::ReadOnly);
    }
}
