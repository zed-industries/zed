use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeletePathToolInput {
    /// The path to the file or directory to delete.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can delete the first file by providing the path "directory1/a/something.txt"
    /// </example>
    pub path: String,
}

pub struct DeletePathTool;

impl Tool for DeletePathTool {
    fn name(&self) -> String {
        "delete-path".into()
    }

    fn description(&self) -> String {
        include_str!("./delete_path_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(DeletePathToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let target_path = match serde_json::from_value::<DeletePathToolInput>(input) {
            Ok(input) => input.path,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let mut deleted_paths = Vec::new();
        let mut errors = Vec::new();

        // Find the path in any of the worktrees
        for worktree_handle in project.read(cx).worktrees(cx) {
            let worktree = worktree_handle.read(cx);
            let path = worktree.abs_path().join(&target_path);
            let display_path = target_path.clone();
            match fs::remove_file(&path) {
                Ok(()) => {
                    deleted_paths.push(display_path);
                }
                Err(file_err) => {
                    // Try to remove directory if it's not a file. Retrying as a directory
                    // on error saves a syscall compared to checking whether it's
                    // a directory up front for every single file.
                    if let Err(dir_err) = fs::remove_dir_all(&path) {
                        let error = if path.is_dir() {
                            format!("Failed to delete directory {}: {dir_err}", display_path)
                        } else {
                            format!("Failed to delete file {}: {file_err}", display_path)
                        };

                        errors.push(error);
                    } else {
                        deleted_paths.push(display_path);
                    }
                }
            }
        }

        if errors.is_empty() {
            if deleted_paths.is_empty() {
                Task::ready(Ok(format!(
                    "No file or directory found at path: {}",
                    target_path
                )))
            } else {
                Task::ready(Ok(format!("Deleted {}", target_path)))
            }
        } else {
            Task::ready(Err(anyhow!(errors.join("\n"))))
        }
    }
}
