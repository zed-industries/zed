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
            let dirty_check_errors: Vec<(PathBuf, String)> = Vec::new();
            let mut reload_errors: Vec<String> = Vec::new();

            for path in input_paths {
                let Some(project_path) =
                    project.read_with(cx, |project, cx| project.find_project_path(&path, cx))
                else {
                    not_found_paths.push(path);
                    continue;
                };

                let open_buffer_task =
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx));

                let buffer = match open_buffer_task.await {
                    Ok(buffer) => buffer,
                    Err(error) => {
                        open_errors.push((path, error.to_string()));
                        continue;
                    }
                };

                let is_dirty = buffer.read_with(cx, |buffer, _| buffer.is_dirty());

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

                if let Err(error) = reload_task.await {
                    reload_errors.push(error.to_string());
                }
            }

            let mut lines: Vec<String> = Vec::new();

            if !restored_paths.is_empty() {
                lines.push(format!("Restored {} file(s).", restored_paths.len()));
            }
            if !clean_paths.is_empty() {
                lines.push(format!("{} clean.", clean_paths.len()));
            }

            if !not_found_paths.is_empty() {
                lines.push(format!("Not found ({}):", not_found_paths.len()));
                for path in &not_found_paths {
                    lines.push(format!("- {}", path.display()));
                }
            }
            if !open_errors.is_empty() {
                lines.push(format!("Open failed ({}):", open_errors.len()));
                for (path, error) in &open_errors {
                    lines.push(format!("- {}: {}", path.display(), error));
                }
            }
            if !dirty_check_errors.is_empty() {
                lines.push(format!(
                    "Dirty check failed ({}):",
                    dirty_check_errors.len()
                ));
                for (path, error) in &dirty_check_errors {
                    lines.push(format!("- {}: {}", path.display(), error));
                }
            }
            if !reload_errors.is_empty() {
                lines.push(format!("Reload failed ({}):", reload_errors.len()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::Fs;
    use gpui::TestAppContext;
    use language::LineEnding;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[gpui::test]
    async fn test_restore_file_from_disk_output_and_effects(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "dirty.txt": "on disk: dirty\n",
                "clean.txt": "on disk: clean\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(RestoreFileFromDiskTool::new(project.clone()));

        // Make dirty.txt dirty in-memory by saving different content into the buffer without saving to disk.
        let dirty_project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path("root/dirty.txt", cx)
                .expect("dirty.txt should exist in project")
        });

        let dirty_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(dirty_project_path, cx)
            })
            .await
            .unwrap();
        dirty_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), "in memory: dirty\n")], None, cx);
        });
        assert!(
            dirty_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "dirty.txt buffer should be dirty before restore"
        );

        // Ensure clean.txt is opened but remains clean.
        let clean_project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path("root/clean.txt", cx)
                .expect("clean.txt should exist in project")
        });

        let clean_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(clean_project_path, cx)
            })
            .await
            .unwrap();
        assert!(
            !clean_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "clean.txt buffer should start clean"
        );

        let output = cx
            .update(|cx| {
                tool.clone().run(
                    RestoreFileFromDiskToolInput {
                        paths: vec![
                            PathBuf::from("root/dirty.txt"),
                            PathBuf::from("root/clean.txt"),
                        ],
                    },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();

        // Output should mention restored + clean.
        assert!(
            output.contains("Restored 1 file(s)."),
            "expected restored count line, got:\n{output}"
        );
        assert!(
            output.contains("1 clean."),
            "expected clean count line, got:\n{output}"
        );

        // Effect: dirty buffer should be restored back to disk content and become clean.
        let dirty_text = dirty_buffer.read_with(cx, |buffer, _| buffer.text());
        assert_eq!(
            dirty_text, "on disk: dirty\n",
            "dirty.txt buffer should be restored to disk contents"
        );
        assert!(
            !dirty_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "dirty.txt buffer should not be dirty after restore"
        );

        // Disk contents should be unchanged (restore-from-disk should not write).
        let disk_dirty = fs.load(path!("/root/dirty.txt").as_ref()).await.unwrap();
        assert_eq!(disk_dirty, "on disk: dirty\n");

        // Sanity: clean buffer should remain clean and unchanged.
        let clean_text = clean_buffer.read_with(cx, |buffer, _| buffer.text());
        assert_eq!(clean_text, "on disk: clean\n");
        assert!(
            !clean_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "clean.txt buffer should remain clean"
        );

        // Test empty paths case.
        let output = cx
            .update(|cx| {
                tool.clone().run(
                    RestoreFileFromDiskToolInput { paths: vec![] },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(output, "No paths provided.");

        // Test not-found path case (path outside the project root).
        let output = cx
            .update(|cx| {
                tool.clone().run(
                    RestoreFileFromDiskToolInput {
                        paths: vec![PathBuf::from("nonexistent/path.txt")],
                    },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();
        assert!(
            output.contains("Not found (1):"),
            "expected not-found header line, got:\n{output}"
        );
        assert!(
            output.contains("- nonexistent/path.txt"),
            "expected not-found path bullet, got:\n{output}"
        );

        let _ = LineEnding::Unix; // keep import used if the buffer edit API changes
    }
}
