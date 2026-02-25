use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, Entity, Task};
use language::{DiagnosticSeverity, OffsetRangeExt};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, sync::Arc};
use ui::SharedString;
use util::markdown::MarkdownInlineCode;

/// Get errors and warnings for the project or a specific file.
///
/// This tool can be invoked after a series of edits to determine if further edits are necessary, or if the user asks to fix errors or warnings in their codebase.
///
/// When a path is provided, shows all diagnostics for that specific file.
/// When no path is provided, shows a summary of error and warning counts for all files in the project.
///
/// <example>
/// To get diagnostics for a specific file:
/// {
///     "path": "src/main.rs"
/// }
///
/// To get a project-wide diagnostic summary:
/// {}
/// </example>
///
/// <guidelines>
/// - If you think you can fix a diagnostic, make 1-2 attempts and then give up.
/// - Don't remove code you've generated just because you can't fix an error. The user can help you fix it.
/// </guidelines>
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DiagnosticsToolInput {
    /// The path to get diagnostics for. If not provided, returns a project-wide summary.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - lorem
    /// - ipsum
    ///
    /// If you wanna access diagnostics for `dolor.txt` in `ipsum`, you should use the path `ipsum/dolor.txt`.
    /// </example>
    pub path: Option<String>,
}

pub struct DiagnosticsTool {
    project: Entity<Project>,
}

impl DiagnosticsTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for DiagnosticsTool {
    type Input = DiagnosticsToolInput;
    type Output = String;

    const NAME: &'static str = "diagnostics";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Some(path) = input.ok().and_then(|input| match input.path {
            Some(path) if !path.is_empty() => Some(path),
            _ => None,
        }) {
            format!("Check diagnostics for {}", MarkdownInlineCode(&path)).into()
        } else {
            "Check project diagnostics".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            match input.path {
                Some(path) if !path.is_empty() => {
                    let (_project_path, open_buffer_task) = project.update(cx, |project, cx| {
                        let Some(project_path) = project.find_project_path(&path, cx) else {
                            return Err(format!("Could not find path {path} in project"));
                        };
                        let task = project.open_buffer(project_path.clone(), cx);
                        Ok((project_path, task))
                    })?;

                    let buffer = futures::select! {
                        result = open_buffer_task.fuse() => result.map_err(|e| e.to_string())?,
                        _ = event_stream.cancelled_by_user().fuse() => {
                            return Err("Diagnostics cancelled by user".to_string());
                        }
                    };
                    let mut output = String::new();
                    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

                    for (_, group) in snapshot.diagnostic_groups(None) {
                        let entry = &group.entries[group.primary_ix];
                        let range = entry.range.to_point(&snapshot);
                        let severity = match entry.diagnostic.severity {
                            DiagnosticSeverity::ERROR => "error",
                            DiagnosticSeverity::WARNING => "warning",
                            _ => continue,
                        };

                        writeln!(
                            output,
                            "{} at line {}: {}",
                            severity,
                            range.start.row + 1,
                            entry.diagnostic.message
                        )
                        .ok();
                    }

                    if output.is_empty() {
                        Ok("File doesn't have errors or warnings!".to_string())
                    } else {
                        Ok(output)
                    }
                }
                _ => {
                    let (output, has_diagnostics) = project.read_with(cx, |project, cx| {
                        let mut output = String::new();
                        let mut has_diagnostics = false;

                        for (project_path, _, summary) in project.diagnostic_summaries(true, cx) {
                            if summary.error_count > 0 || summary.warning_count > 0 {
                                let Some(worktree) =
                                    project.worktree_for_id(project_path.worktree_id, cx)
                                else {
                                    continue;
                                };

                                has_diagnostics = true;
                                output.push_str(&format!(
                                    "{}: {} error(s), {} warning(s)\n",
                                    worktree.read(cx).absolutize(&project_path.path).display(),
                                    summary.error_count,
                                    summary.warning_count
                                ));
                            }
                        }

                        (output, has_diagnostics)
                    });

                    if has_diagnostics {
                        Ok(output)
                    } else {
                        Ok("No errors or warnings found in the project.".into())
                    }
                }
            }
        })
    }
}
