use std::fs;
use std::path::{Component, Path, PathBuf};

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

#[derive(Debug)]
pub enum ToolError {
    EmptyWorkspaceRoot,
    AbsolutePathBlocked,
    ParentTraversalBlocked,
    ProtectedPathBlocked,
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
                | (Self::Io(_), Self::Io(_))
        )
    }
}

impl Eq for ToolError {}

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
        fs::create_dir_all(root.resolve_relative("target").unwrap_err_path()).ok();
        fs::create_dir_all(root.resolve_relative("node_modules").unwrap_err_path()).ok();
        let entries = root.list_dir(".").unwrap();
        assert_eq!(entries, vec!["README.md".to_string()]);
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

    trait ErrPath {
        fn unwrap_err_path(self) -> PathBuf;
    }

    impl ErrPath for Result<PathBuf, ToolError> {
        fn unwrap_err_path(self) -> PathBuf {
            match self {
                Ok(path) => path,
                Err(_) => PathBuf::new(),
            }
        }
    }
}
