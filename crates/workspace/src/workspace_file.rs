use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Represents a workspace definition file (`.code-workspace`, future `.zed-workspace`).
/// This abstraction is format-agnostic to support multiple workspace file formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkspaceFileSource {
    /// Absolute path to the workspace file
    pub path: PathBuf,
    /// The format/type of workspace file
    pub kind: WorkspaceFileKind,
}

/// The type of workspace file format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkspaceFileKind {
    /// VS Code `.code-workspace` format
    CodeWorkspace,
    // Future: ZedWorkspace for native `.zed-workspace` format
}

/// Parsed workspace file content (format-agnostic)
#[derive(Debug, Clone)]
pub struct WorkspaceFileContent {
    /// Folders to open, resolved to absolute paths
    pub folders: Vec<PathBuf>,
    // TODO: settings translation; VS Code workspace settings could be mapped to Zed equivalents
}

impl WorkspaceFileSource {
    /// Detect workspace file type from path extension.
    /// Returns `Some(WorkspaceFileSource)` if the path is a recognized workspace file format.
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let path = path.as_ref();
        let extension = path.extension()?.to_str()?;

        let kind = match extension {
            "code-workspace" => WorkspaceFileKind::CodeWorkspace,
            // Future: "zed-workspace" => WorkspaceFileKind::ZedWorkspace,
            _ => return None,
        };

        Some(Self {
            path: path.to_path_buf(),
            kind,
        })
    }

    /// Returns the display name for this workspace file (the filename).
    pub fn display_name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Workspace")
    }

    /// Parse the workspace file and resolve folder paths to absolute paths.
    pub fn parse(&self, content: &str) -> Result<WorkspaceFileContent> {
        match self.kind {
            WorkspaceFileKind::CodeWorkspace => self.parse_code_workspace(content),
        }
    }

    fn parse_code_workspace(&self, content: &str) -> Result<WorkspaceFileContent> {
        let parsed: CodeWorkspaceFile = serde_json::from_str(content)
            .with_context(|| "Failed to parse `.code-workspace` file as JSON")?;

        let base_dir = self
            .path
            .parent()
            .context("Workspace file has no parent directory")?;

        let folders = parsed
            .folders
            .into_iter()
            .map(|f| {
                let folder_path = PathBuf::from(&f.path);
                if folder_path.is_absolute() {
                    folder_path
                } else {
                    base_dir.join(folder_path)
                }
            })
            .collect();

        Ok(WorkspaceFileContent { folders })
    }
}

impl WorkspaceFileKind {
    /// Convert the kind to a string for serialization/storage
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceFileKind::CodeWorkspace => "code-workspace",
        }
    }
}

impl FromStr for WorkspaceFileKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "code-workspace" => Ok(Self::CodeWorkspace),
            _ => Err(anyhow::anyhow!("Unknown workspace file kind: {}", s)),
        }
    }
}

/// VS Code `.code-workspace` JSON structure
/// Only includes fields we need for v1 - settings and other fields are ignored
#[derive(Deserialize)]
struct CodeWorkspaceFile {
    folders: Vec<CodeWorkspaceFolder>,
    // TODO:
    // - settings: VS Code workspace settings
    // - extensions: Recommended extensions
    // - launch: Debug configurations
    // - tasks: Task configurations
}

/// A folder entry in the `.code-workspace` file
#[derive(Deserialize)]
struct CodeWorkspaceFolder {
    path: String,
    // TODO:
    // - name: Display name for the folder in the explorer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path_code_workspace() {
        let source = WorkspaceFileSource::from_path("/path/to/project.code-workspace");
        assert!(source.is_some());
        let source = source.unwrap();
        assert_eq!(source.kind, WorkspaceFileKind::CodeWorkspace);
        assert_eq!(
            source.path,
            PathBuf::from("/path/to/project.code-workspace")
        );
    }

    #[test]
    fn test_from_path_non_workspace() {
        assert!(WorkspaceFileSource::from_path("/path/to/file.json").is_none());
        assert!(WorkspaceFileSource::from_path("/path/to/file.txt").is_none());
        assert!(WorkspaceFileSource::from_path("/path/to/folder").is_none());
    }

    #[test]
    fn test_display_name() {
        let source = WorkspaceFileSource::from_path("/path/to/my-project.code-workspace").unwrap();
        assert_eq!(source.display_name(), "my-project.code-workspace");
    }

    #[test]
    fn test_parse_code_workspace_relative_paths() {
        let source = WorkspaceFileSource::from_path("/projects/my-project.code-workspace").unwrap();
        let content = r#"{
            "folders": [
                { "path": "." },
                { "path": "packages/frontend" },
                { "path": "../shared-lib" }
            ]
        }"#;

        let parsed = source.parse(content).unwrap();
        assert_eq!(parsed.folders.len(), 3);
        assert_eq!(parsed.folders[0], PathBuf::from("/projects"));
        assert_eq!(
            parsed.folders[1],
            PathBuf::from("/projects/packages/frontend")
        );
        assert_eq!(parsed.folders[2], PathBuf::from("/projects/../shared-lib"));
    }

    #[test]
    fn test_parse_code_workspace_absolute_paths() {
        let source = WorkspaceFileSource::from_path("/projects/my-project.code-workspace").unwrap();
        let content = r#"{
            "folders": [
                { "path": "/absolute/path/to/folder" }
            ]
        }"#;

        let parsed = source.parse(content).unwrap();
        assert_eq!(parsed.folders.len(), 1);
        assert_eq!(parsed.folders[0], PathBuf::from("/absolute/path/to/folder"));
    }

    #[test]
    fn test_parse_code_workspace_with_names_ignored() {
        let source = WorkspaceFileSource::from_path("/projects/my-project.code-workspace").unwrap();
        let content = r#"{
            "folders": [
                { "name": "Custom Name", "path": "src" }
            ]
        }"#;

        let parsed = source.parse(content).unwrap();
        assert_eq!(parsed.folders.len(), 1);
        assert_eq!(parsed.folders[0], PathBuf::from("/projects/src"));
    }

    #[test]
    fn test_parse_code_workspace_with_settings_ignored() {
        let source = WorkspaceFileSource::from_path("/projects/my-project.code-workspace").unwrap();
        let content = r#"{
            "folders": [
                { "path": "." }
            ],
            "settings": {
                "editor.tabSize": 2,
                "files.exclude": {
                    "**/node_modules": true
                }
            }
        }"#;

        let parsed = source.parse(content).unwrap();
        assert_eq!(parsed.folders.len(), 1);
    }

    #[test]
    fn test_parse_code_workspace_invalid_json() {
        let source = WorkspaceFileSource::from_path("/projects/my-project.code-workspace").unwrap();
        let content = "not valid json";

        let result = source.parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_code_workspace_missing_folders() {
        let source = WorkspaceFileSource::from_path("/projects/my-project.code-workspace").unwrap();
        let content = r#"{ "settings": {} }"#;

        let result = source.parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_file_kind_roundtrip() {
        assert_eq!(
            WorkspaceFileKind::CodeWorkspace
                .as_str()
                .parse::<WorkspaceFileKind>()
                .unwrap(),
            WorkspaceFileKind::CodeWorkspace
        );
        assert!("unknown".parse::<WorkspaceFileKind>().is_err());
    }
}
