use agent_client_protocol as acp;
use anyhow::Result;
use collections::FxHashSet;
use gpui::{App, Entity, SharedString, Task};
use language::Buffer;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream};

/// Discards unsaved changes in open buffers by reloading file contents from disk.
///
/// Use this tool when:
/// - You attempted to edit files but they have unsaved changes the user does not want to keep.
/// - You want to reset files to the on-disk state before retrying an edit.
///
/// Only use this tool after asking the user for permission, because it will discard unsaved changes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RestoreFileFromDiskToolInput {
    /// The paths of the files to restore from disk.
    pub paths: Vec<PathBuf>,
}

pub struct RestoreFileFromDiskTool {
    project: Entity<Project>,
}

impl RestoreFileFromDiskTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for RestoreFileFromDiskTool {
    type Input = RestoreFileFromDiskToolInput;
    type Output = String;

    fn name() -> &'static str {
        "restore_file_from_disk"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) if input.paths.len() == 1 => "Restore file from disk".into(),
            Ok(input) => format!("Restore {} files from disk", input.paths.len()).into(),
            Err(_) => "Restore files from disk".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let project = self.project.clone();
        let input_paths = input.paths;

        cx.spawn(async move |cx| {
            let mut buffers_to_reload: FxHashSet<Entity<Buffer>> = FxHashSet::default();

            let mut restored_paths: Vec<PathBuf> = Vec::new();
            let mut clean_paths: Vec<PathBuf> = Vec::new();
            let mut not_found_paths: Vec<PathBuf> = Vec::new();
            let mut open_errors: Vec<(PathBuf, String)> = Vec::new();
            let mut dirty_check_errors: Vec<(PathBuf, String)> = Vec::new();
            let mut reload_errors: Vec<String> = Vec::new();

            for path in input_paths {
                let project_path =
                    project.read_with(cx, |project, cx| project.find_project_path(&path, cx));

                let project_path = match project_path {
                    Ok(Some(project_path)) => project_path,
                    Ok(None) => {
                        not_found_paths.push(path);
                        continue;
                    }
                    Err(error) => {
                        open_errors.push((path, error.to_string()));
                        continue;
                    }
                };

                let open_buffer_task =
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx));

                let buffer = match open_buffer_task {
                    Ok(task) => match task.await {
                        Ok(buffer) => buffer,
                        Err(error) => {
                            open_errors.push((path, error.to_string()));
                            continue;
                        }
                    },
                    Err(error) => {
                        open_errors.push((path, error.to_string()));
                        continue;
                    }
                };

                let is_dirty = match buffer.read_with(cx, |buffer, _| buffer.is_dirty()) {
                    Ok(is_dirty) => is_dirty,
                    Err(error) => {
                        dirty_check_errors.push((path, error.to_string()));
                        continue;
                    }
                };

                if is_dirty {
                    buffers_to_reload.insert(buffer);
                    restored_paths.push(path);
                } else {
                    clean_paths.push(path);
                }
            }

            if !buffers_to_reload.is_empty() {
                let reload_task = project.update(cx, |project, cx| {
                    project.reload_buffers(buffers_to_reload, true, cx)
                });

                match reload_task {
                    Ok(task) => {
                        if let Err(error) = task.await {
                            reload_errors.push(error.to_string());
                        }
                    }
                    Err(error) => {
                        reload_errors.push(error.to_string());
                    }
                }
            }

            let mut lines: Vec<String> = Vec::new();
            if !restored_paths.is_empty() {
                lines.push(format!(
                    "Restored {} file(s) from disk (discarded unsaved changes).",
                    restored_paths.len()
                ));
            }
            if !clean_paths.is_empty() {
                lines.push(format!(
                    "{} file(s) had no unsaved changes.",
                    clean_paths.len()
                ));
            }
            if !not_found_paths.is_empty() {
                lines.push(format!(
                    "{} path(s) were not found in the project:",
                    not_found_paths.len()
                ));
                for path in &not_found_paths {
                    lines.push(format!("- {}", path.display()));
                }
            }
            if !open_errors.is_empty() {
                lines.push(format!(
                    "{} error(s) occurred while opening buffers:",
                    open_errors.len()
                ));
                for (path, error) in &open_errors {
                    lines.push(format!("- {}: {}", path.display(), error));
                }
            }
            if !dirty_check_errors.is_empty() {
                lines.push(format!(
                    "{} error(s) occurred while checking for unsaved changes:",
                    dirty_check_errors.len()
                ));
                for (path, error) in &dirty_check_errors {
                    lines.push(format!("- {}: {}", path.display(), error));
                }
            }
            if !reload_errors.is_empty() {
                lines.push(format!(
                    "{} error(s) occurred while reloading buffers:",
                    reload_errors.len()
                ));
                for error in &reload_errors {
                    lines.push(format!("- {}", error));
                }
            }

            if lines.is_empty() {
                Ok("No paths provided.".to_string())
            } else {
                Ok(lines.join("\n"))
            }
        })
    }
}
