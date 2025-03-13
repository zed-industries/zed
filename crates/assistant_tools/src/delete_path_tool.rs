use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeletePathToolInput {
    /// The path of the file or directory to delete.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can delete the first file by providing a path of "directory1/a/something.txt"
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
        let path_str = match serde_json::from_value::<DeletePathToolInput>(input) {
            Ok(input) => input.path,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let mut path = PathBuf::from(&path_str);

        // Change a path of "foo/bar.txt" to "/Users/someone/project/foo/bar.txt"
        if !path.is_absolute() {
            let mut path_components = path.components();

            // Find the worktree whose last abs_path component equals this path's first component,
            // e.g. if this path starts with "foo/", find the worktree whose abs_path ends in "foo"
            if let Some(target_root_dir) = path_components.next() {
                for worktree in project.read(cx).worktrees(cx) {
                    let abs_path = worktree.read(cx).abs_path();

                    if abs_path.components().last() == Some(target_root_dir) {
                        // Use that abs_path as our path's prefix. Join it with the other components
                        // so we don't repeat the first component (the one that matched abs_path).
                        path = abs_path.join(path_components);
                        break;
                    }
                }
            }

            if !path.is_absolute() {
                return Task::ready(Err(anyhow!(
                    "Couldn't delete {} because it wasn't in any of this project's worktrees.",
                    path.display()
                )));
            }
        };

        cx.spawn(|_cx| async move {
            // Try to delete the file first
            match fs::remove_file(&path) {
                Ok(()) => Ok(format!("Deleted file {}", path_str)),
                Err(file_err) => {
                    // If it's not a file, try to delete it as a directory
                    match fs::remove_dir_all(&path) {
                        Ok(()) => Ok(format!("Deleted directory {}", path_str)),
                        Err(dir_err) => {
                            // Return an error with appropriate message
                            if path.is_dir() {
                                Err(anyhow!(
                                    "Failed to delete directory {}: {}",
                                    path_str,
                                    dir_err
                                ))
                            } else {
                                Err(anyhow!("Failed to delete file {}: {}", path_str, file_err))
                            }
                        }
                    }
                }
            }
        })
    }
}
