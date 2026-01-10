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

/// Saves files that have unsaved changes.
///
/// Use this tool when you need to edit files but they have unsaved changes that must be saved first.
/// Only use this tool after asking the user for permission to save their unsaved changes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveFileToolInput {
    /// The paths of the files to save.
    pub paths: Vec<PathBuf>,
}

pub struct SaveFileTool {
    project: Entity<Project>,
}

impl SaveFileTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for SaveFileTool {
    type Input = SaveFileToolInput;
    type Output = String;

    fn name() -> &'static str {
        "save_file"
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
            Ok(input) if input.paths.len() == 1 => "Save file".into(),
            Ok(input) => format!("Save {} files", input.paths.len()).into(),
            Err(_) => "Save files".into(),
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
            let mut buffers_to_save: FxHashSet<Entity<Buffer>> = FxHashSet::default();

            let mut saved_paths: Vec<PathBuf> = Vec::new();
            let mut clean_paths: Vec<PathBuf> = Vec::new();
            let mut not_found_paths: Vec<PathBuf> = Vec::new();
            let mut open_errors: Vec<(PathBuf, String)> = Vec::new();
            let dirty_check_errors: Vec<(PathBuf, String)> = Vec::new();
            let mut save_errors: Vec<(String, String)> = Vec::new();

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
                    buffers_to_save.insert(buffer);
                    saved_paths.push(path);
                } else {
                    clean_paths.push(path);
                }
            }

            // Save each buffer individually since there's no batch save API.
            for buffer in buffers_to_save {
                let path_for_buffer = buffer
                    .read_with(cx, |buffer, _| {
                        buffer
                            .file()
                            .map(|file| file.path().to_rel_path_buf())
                            .map(|path| path.as_rel_path().as_unix_str().to_owned())
                    })
                    .unwrap_or_else(|| "<unknown>".to_string());

                let save_task = project.update(cx, |project, cx| project.save_buffer(buffer, cx));

                if let Err(error) = save_task.await {
                    save_errors.push((path_for_buffer, error.to_string()));
                }
            }

            let mut lines: Vec<String> = Vec::new();

            if !saved_paths.is_empty() {
                lines.push(format!("Saved {} file(s).", saved_paths.len()));
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
            if !save_errors.is_empty() {
                lines.push(format!("Save failed ({}):", save_errors.len()));
                for (path, error) in &save_errors {
                    lines.push(format!("- {}: {}", path, error));
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
    async fn test_save_file_output_and_effects(cx: &mut TestAppContext) {
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
        let tool = Arc::new(SaveFileTool::new(project.clone()));

        // Make dirty.txt dirty in-memory.
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
            "dirty.txt buffer should be dirty before save"
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
                    SaveFileToolInput {
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

        // Output should mention saved + clean.
        assert!(
            output.contains("Saved 1 file(s)."),
            "expected saved count line, got:\n{output}"
        );
        assert!(
            output.contains("1 clean."),
            "expected clean count line, got:\n{output}"
        );

        // Effect: dirty buffer should now be clean and disk should have new content.
        assert!(
            !dirty_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "dirty.txt buffer should not be dirty after save"
        );

        let disk_dirty = fs.load(path!("/root/dirty.txt").as_ref()).await.unwrap();
        assert_eq!(
            disk_dirty, "in memory: dirty\n",
            "dirty.txt disk content should be updated"
        );

        // Sanity: clean buffer should remain clean and disk unchanged.
        let disk_clean = fs.load(path!("/root/clean.txt").as_ref()).await.unwrap();
        assert_eq!(disk_clean, "on disk: clean\n");

        // Test empty paths case.
        let output = cx
            .update(|cx| {
                tool.clone().run(
                    SaveFileToolInput { paths: vec![] },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(output, "No paths provided.");

        // Test not-found path case.
        let output = cx
            .update(|cx| {
                tool.clone().run(
                    SaveFileToolInput {
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
    }
}
