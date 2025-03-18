use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{DiagnosticSeverity, OffsetRangeExt};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

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
    pub path: Option<PathBuf>,
}

pub struct DiagnosticsTool;

impl Tool for DiagnosticsTool {
    fn name(&self) -> String {
        "diagnostics".into()
    }

    fn description(&self) -> String {
        include_str!("./diagnostics_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(DiagnosticsToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<DiagnosticsToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        if let Some(path) = input.path {
            let Some(project_path) = project.read(cx).find_project_path(&path, cx) else {
                return Task::ready(Err(anyhow!("Could not find path in project")));
            };
            let buffer = project.update(cx, |project, cx| project.open_buffer(project_path, cx));

            cx.spawn(|cx| async move {
                let mut output = String::new();
                let buffer = buffer.await?;
                let snapshot = buffer.read_with(&cx, |buffer, _cx| buffer.snapshot())?;

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
        } else {
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

            if has_diagnostics {
                Task::ready(Ok(output))
            } else {
                Task::ready(Ok("No errors or warnings found in the project.".to_string()))
            }
        }
    }
}
