use agent_client_protocol as acp;
use anyhow::Result;
use diffy::create_patch;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write as _, sync::Arc};

use crate::{AgentTool, ToolCallEventStream, ToolInput};

/// Lists files with unsaved edits and shows what changed since the last save.
///
/// Use this tool when you want to understand what the user has been working on
/// before making edits, or to avoid clobbering unsaved changes. Returns a unified
/// diff for each dirty (unsaved) buffer in the project. Files that have never been
/// saved are shown in full.
///
/// <guidelines>
/// - Call this at the start of an agentic session to discover pending user changes.
/// - If no dirty files exist, the output will say so clearly.
/// - Use `max_files` to limit output when you only need a quick overview.
/// </guidelines>
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RecentChangesToolInput {
    /// Maximum number of files to include. Defaults to 20.
    #[serde(default)]
    pub max_files: Option<usize>,
}

pub struct RecentChangesTool {
    project: Entity<Project>,
}

impl RecentChangesTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for RecentChangesTool {
    type Input = RecentChangesToolInput;
    type Output = String;

    const NAME: &'static str = "recent_changes";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Checking recent changes".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        let project = self.project.clone();

        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let max_files = input.max_files.unwrap_or(20).min(50);

            // Snapshot dirty buffer info synchronously before any async I/O.
            // Returns: (display_path, Option<abs_path>, is_new, current_text)
            let dirty_entries: Vec<(String, Option<std::path::PathBuf>, bool, String)> = cx
                .update(|cx| {
                    let buffer_store = project.read(cx).buffer_store().read(cx);

                    buffer_store
                        .buffers()
                        .filter(|buf| buf.read(cx).is_dirty())
                        .take(max_files)
                        .map(|buf| {
                            let buffer = buf.read(cx);
                            let file = buffer.file();

                            let display_path = file
                                .map(|f| f.full_path(cx).to_string_lossy().into_owned())
                                .unwrap_or_else(|| "<untitled>".to_string());

                            let abs_path = file.map(|f| f.abs_path(cx));

                            // A buffer that has never been saved has an empty saved_version.
                            let is_new = buffer.saved_version() == &clock::Global::new();

                            let current_text = buffer.text();

                            (display_path, abs_path, is_new, current_text)
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|e| format!("Failed to read buffers: {e}"))?;

            if dirty_entries.is_empty() {
                return Ok("No unsaved changes found.".to_string());
            }

            let fs = project
                .read_with(cx, |p, _| p.fs().clone())
                .map_err(|e| format!("{e}"))?;

            let mut output = String::new();
            writeln!(output, "{} file(s) with unsaved changes:\n", dirty_entries.len()).ok();

            for (display_path, abs_path, is_new, current_text) in dirty_entries {
                writeln!(output, "### {}", display_path).ok();

                if is_new || abs_path.is_none() {
                    // Never saved — show full content as a creation diff
                    let patch = create_patch("", &current_text);
                    writeln!(output, "```diff\n{}```", patch).ok();
                } else {
                    let abs_path = abs_path.unwrap();
                    match fs.load(&abs_path).await {
                        Ok(saved_text) => {
                            let patch = create_patch(&saved_text, &current_text);
                            let patch_str = patch.to_string();
                            if patch_str.lines().filter(|l| l.starts_with("@@")).count() == 0 {
                                writeln!(output, "_No textual differences._").ok();
                            } else {
                                writeln!(output, "```diff\n{}```", patch_str).ok();
                            }
                        }
                        Err(err) => {
                            writeln!(output, "_Could not load saved version: {}_", err).ok();
                            // Show current text truncated so the agent still has context
                            let preview = &current_text[..current_text.len().min(1000)];
                            let truncated = current_text.len() > 1000;
                            writeln!(output, "```\n{}{}```", preview, if truncated { "\n... (truncated)" } else { "" }).ok();
                        }
                    }
                }
                writeln!(output).ok();
            }

            Ok(output)
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        event_stream
            .update_fields(acp::ToolCallUpdateFields::new().content(vec![output.into()]));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AgentTool, ToolInput};
    use gpui::TestAppContext;
    use project::FakeFs;
    use std::sync::Arc;

    #[gpui::test]
    async fn test_no_dirty_buffers(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", serde_json::json!({"clean.rs": "fn main() {}"}))
            .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let tool = RecentChangesTool::new(project.clone());

        let (stream, _) = ToolCallEventStream::test();
        let output = cx
            .update(|cx| {
                Arc::new(tool).run(
                    ToolInput::resolved(RecentChangesToolInput { max_files: None }),
                    stream,
                    cx,
                )
            })
            .await
            .unwrap();

        assert_eq!(output, "No unsaved changes found.");
    }

    #[gpui::test]
    async fn test_dirty_buffer_shows_diff(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            serde_json::json!({"main.rs": "fn main() {\n    println!(\"hello\");\n}\n"}),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

        // Open buffer and make it dirty
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(
                    project::ProjectPath {
                        worktree_id: project
                            .worktrees(cx)
                            .next()
                            .unwrap()
                            .read(cx)
                            .id(),
                        path: "main.rs".into(),
                    },
                    cx,
                )
            })
            .await
            .unwrap();

        buffer.update(cx, |buf, cx| {
            buf.edit([(0..0, "// edited\n")], None, cx);
        });

        let tool = RecentChangesTool::new(project.clone());
        let (stream, _) = ToolCallEventStream::test();
        let output = cx
            .update(|cx| {
                Arc::new(tool).run(
                    ToolInput::resolved(RecentChangesToolInput { max_files: None }),
                    stream,
                    cx,
                )
            })
            .await
            .unwrap();

        assert!(
            output.contains("main.rs"),
            "output should mention the dirty file"
        );
        assert!(
            output.contains("```diff"),
            "output should contain a diff block"
        );
        assert!(
            output.contains("// edited"),
            "output should show the added line"
        );
    }
}
