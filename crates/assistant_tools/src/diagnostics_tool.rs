use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{DiagnosticSeverity, OffsetRangeExt};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]
pub struct DiagnosticsToolInput {
    /// The specific paths to get detailed diagnostics for (including individual line numbers).
    ///
    /// Regardless of whether any paths are specified here, a count of the total number of warnings
    /// and errors in the project will be reported, so providing paths here gets you strictly
    /// more information.
    ///
    /// These paths should never be absolute, and the first component
    /// of each path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - lorem
    /// - ipsum
    /// - amet
    ///
    /// If you want detailed diagnostics with line numbers for `dolor.txt` in `ipsum` and `consectetur.txt` in `amet`, you should use:
    ///
    ///     "paths": ["ipsum/dolor.txt", "amet/consectetur.txt"]
    /// </example>
    #[serde(deserialize_with = "deserialize_path")]
    #[serde(default)]
    pub paths: Vec<String>,

    /// Which severity levels to show. Default is all.
    /// To show only errors and warnings, you should use:
    ///
    ///     "severity": ["error", "warning"]
    #[serde(default)]
    pub severity: Vec<Severity>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    Copy,
    Clone,
    strum::Display,
    Hash,
)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    Error,
    Warning,
    Information,
    Hint,
}

fn deserialize_path<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let paths = Vec::<String>::deserialize(deserializer)?;
    // The model passes an empty string for some paths
    Ok(paths.into_iter().filter(|s| !s.is_empty()).collect())
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

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<DiagnosticsToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        serde_json::from_value::<DiagnosticsToolInput>(input.clone())
            .ok()
            .and_then(|input| {
                input.paths.first().map(|first_path| {
                    if input.paths.len() > 1 {
                        format!("Check diagnostics for {} paths", input.paths.len())
                    } else {
                        format!(
                            "Check diagnostics for {}",
                            MarkdownString::inline_code(first_path)
                        )
                    }
                })
            })
            .unwrap_or_else(|| "Check project diagnostics".to_string())
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = serde_json::from_value::<DiagnosticsToolInput>(input).unwrap_or_default();
        let severity_filter = input.severity;

        if input.paths.is_empty() {
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
                Task::ready(Ok(output))
            } else {
                Task::ready(Ok("No errors or warnings found in the project.".to_string()))
            }
        } else {
            let mut output = String::new();
            let mut buffer_tasks = Vec::with_capacity(input.paths.len());

            for path in input.paths {
                let Some(project_path) = project.read(cx).find_project_path(&path, cx) else {
                    return Task::ready(Err(anyhow!("Could not find path {path} in project",)));
                };

                buffer_tasks.push((
                    path,
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx)),
                ));
            }

            cx.spawn(async move |cx| {
                for (path, buffer_task) in buffer_tasks {
                    let mut path_printed = false;
                    let buffer = buffer_task.await?;
                    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                    for (_, group) in snapshot.diagnostic_groups(None) {
                        let entry = &group.entries[group.primary_ix];
                        let range = entry.range.to_point(&snapshot);

                        if let Ok(severity) = Severity::try_from(&entry.diagnostic.severity) {
                            if severity_filter.is_empty() || severity_filter.contains(&severity) {
                                if !path_printed {
                                    writeln!(output, "## {path}",)?;
                                    path_printed = true;
                                }

                                writeln!(
                                    output,
                                    "\n### {severity} at line {}\n{}",
                                    range.start.row + 1,
                                    entry.diagnostic.message
                                )?;
                            }
                        }
                    }
                }

                Ok(if output.is_empty() {
                    "No diagnostics found!".to_string()
                } else {
                    output
                })
            })
        }
    }
}

impl TryFrom<&DiagnosticSeverity> for Severity {
    type Error = ();

    fn try_from(
        value: &DiagnosticSeverity,
    ) -> Result<Self, <Severity as TryFrom<&DiagnosticSeverity>>::Error> {
        if *value == DiagnosticSeverity::ERROR {
            Ok(Self::Error)
        } else if *value == DiagnosticSeverity::WARNING {
            Ok(Self::Warning)
        } else if *value == DiagnosticSeverity::INFORMATION {
            Ok(Self::Information)
        } else if *value == DiagnosticSeverity::HINT {
            Ok(Self::Hint)
        } else {
            Err(())
        }
    }
}
