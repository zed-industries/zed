use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
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

    fn name() -> &'static str {
        "diagnostics"
    }

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
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        match input.path {
            Some(path) if !path.is_empty() => {
                let Some(project_path) = self.project.read(cx).find_project_path(&path, cx) else {
                    return Task::ready(Err(anyhow!("Could not find path {path} in project",)));
                };

                let buffer = self
                    .project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx));

                cx.spawn(async move |cx| {
                    let mut output = String::new();
                    let buffer = buffer.await?;
                    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

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
                        )?;
                    }

                    if output.is_empty() {
                        Ok("File doesn't have errors or warnings!".to_string())
                    } else {
                        Ok(output)
                    }
                })
            }
            _ => {
                let project = self.project.read(cx);
                let mut output = String::new();
                let mut has_diagnostics = false;

                for (project_path, _, summary) in project.diagnostic_summaries(true, cx) {
                    if summary.error_count > 0 || summary.warning_count > 0 {
                        let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx)
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

                if has_diagnostics {
                    Task::ready(Ok(output))
                } else {
                    Task::ready(Ok("No errors or warnings found in the project.".into()))
                }
            }
        }
    }
}
