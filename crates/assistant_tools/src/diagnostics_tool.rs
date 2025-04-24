use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language::{DiagnosticSeverity, OffsetRangeExt};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;

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
    #[serde(deserialize_with = "deserialize_path")]
    pub path: Option<String>,
}

fn deserialize_path<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    // The model passes an empty string sometimes
    Ok(opt.filter(|s| !s.is_empty()))
}

pub struct DiagnosticsTool;

impl Tool for DiagnosticsTool {
    fn name(&self) -> String {
        "diagnostics".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./diagnostics_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::XCircle
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<DiagnosticsToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        if let Some(path) = serde_json::from_value::<DiagnosticsToolInput>(input.clone())
            .ok()
            .and_then(|input| match input.path {
                Some(path) if !path.is_empty() => Some(MarkdownString::inline_code(&path)),
                _ => None,
            })
        {
            format!("Check diagnostics for {path}")
        } else {
            "Check project diagnostics".to_string()
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        match serde_json::from_value::<DiagnosticsToolInput>(input)
            .ok()
            .and_then(|input| input.path)
        {
            Some(path) if !path.is_empty() => {
                let Some(project_path) = project.read(cx).find_project_path(&path, cx) else {
                    return Task::ready(Err(anyhow!("Could not find path {path} in project",)))
                        .into();
                };

                let buffer =
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx));

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
                .into()
            }
            _ => {
                let project = project.read(cx);
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
                            Path::new(worktree.read(cx).root_name())
                                .join(project_path.path)
                                .display(),
                            summary.error_count,
                            summary.warning_count
                        ));
                    }
                }

                action_log.update(cx, |action_log, _cx| {
                    action_log.checked_project_diagnostics();
                });

                if has_diagnostics {
                    Task::ready(Ok(output)).into()
                } else {
                    Task::ready(Ok("No errors or warnings found in the project.".to_string()))
                        .into()
                }
            }
        }
    }
}
