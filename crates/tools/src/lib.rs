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
        let resolved = self.root.join(path);
        if is_protected(path) {
            return Err(ToolError::ProtectedPathBlocked);
        }
        Ok(resolved)
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ToolError {
    EmptyWorkspaceRoot,
    AbsolutePathBlocked,
    ParentTraversalBlocked,
    ProtectedPathBlocked,
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

    #[test]
    fn resolves_relative_file_inside_workspace() {
        let root = WorkspaceRoot::new("/repo").unwrap();
        let path = root.resolve_relative("README.md").unwrap();
        assert_eq!(path, PathBuf::from("/repo/README.md"));
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
