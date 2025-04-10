use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{DiagnosticSeverity, OffsetRangeExt};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::LspAction;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QuickfixToolInput {
    /// The path to get diagnostics for and apply quickfixes. If not provided, checks the entire project.
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
    /// If you want to apply quickfixes for `dolor.txt` in `ipsum`, you should use the path `ipsum/dolor.txt`.
    /// </example>
    pub path: Option<String>,
}

pub struct QuickfixTool;

impl Tool for QuickfixTool {
    fn name(&self) -> String {
        "quickfix".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./quickfix_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Check
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<QuickfixToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        if let Some(path) = serde_json::from_value::<QuickfixToolInput>(input.clone())
            .ok()
            .and_then(|input| match input.path {
                Some(path) if !path.is_empty() => Some(MarkdownString::inline_code(&path)),
                _ => None,
            })
        {
            format!("Apply quickfixes for {path}")
        } else {
            "Apply project-wide quickfixes".to_string()
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        match serde_json::from_value::<QuickfixToolInput>(input)
            .ok()
            .and_then(|input| input.path)
        {
            Some(path) if !path.is_empty() => {
                let Some(project_path) = project.read(cx).find_project_path(&path, cx) else {
                    return Task::ready(Err(anyhow!("Could not find path {path} in project")));
                };

                let buffer =
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx));

                cx.spawn(async move |cx| {
                    let mut output = String::new();
                    let mut fixes_applied = 0;
                    let mut errors_unfixed = 0;
                    let mut warnings_unfixed = 0;

                    let buffer = buffer.await?;
                    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                    // Collect diagnostics to apply quickfixes for
                    for entry in snapshot.diagnostic_groups(None).into_iter().flat_map(|(_, group)| {
                        group.entries.into_iter()/*.filter(|entry| {
                          let severity = entry.diagnostic.severity;

                          // Skip informational and hint diagnostics
                          severity == DiagnosticSeverity::ERROR ||
                             severity == DiagnosticSeverity::WARNING
                        })*/
                    }) {
                        let severity = entry.diagnostic.severity;
                        let range = entry.range.to_point(&snapshot);

                        // Get code actions (quickfixes) for this diagnostic
                        let actions = project
                            .update(cx, |project, cx| {
                                project.code_actions(
                                    &buffer,
                                    range.start..range.end,
                                    None,
                                    cx
                                )
                            })?.await;

                        match actions {
                            Ok(actions) => {
                                // Find quickfix actions
                                let quickfixes = actions.into_iter().filter(|action| {
                                    if let LspAction::Action(code_action) = &action.lsp_action {
                                        if let Some(kind) = &code_action.kind {
                                            kind.as_str().starts_with("quickfix")
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                }).collect::<Vec<_>>();

                                if !quickfixes.is_empty() {
                                    // Find the preferred quickfix (marked as is_preferred or the first one)
                                    let preferred_action = quickfixes.iter().find(|action| {
                                        if let LspAction::Action(code_action) = &action.lsp_action {
                                            code_action.is_preferred.unwrap_or(false)
                                        } else {
                                            false
                                        }
                                    }).unwrap_or(&quickfixes[0]);

                                    // Apply the quickfix
                                    let title = preferred_action.lsp_action.title().to_string();
                                    project
                                        .update(cx, |project, cx| {
                                            project.apply_code_action(buffer.clone(), preferred_action.clone(), true, cx)
                                        })?
                                        .await?;

                                    writeln!(output, "Applied quickfix: {title}")?;
                                    fixes_applied += 1;
                                } else {
                                    // Track unfixed diagnostics
                                    match severity {
                                        DiagnosticSeverity::ERROR => errors_unfixed += 1,
                                        DiagnosticSeverity::WARNING => warnings_unfixed += 1,
                                        _ => {}
                                    }

                                    writeln!(
                                        output,
                                        "No quickfix available for {} at line {}: {}",
                                        if severity == DiagnosticSeverity::ERROR { "error" } else { "warning" },
                                        range.start.row + 1,
                                        entry.diagnostic.message
                                    )?;
                                }
                            },
                            Err(err) => {
                                // Track unfixed diagnostics
                                match severity {
                                    DiagnosticSeverity::ERROR => errors_unfixed += 1,
                                    DiagnosticSeverity::WARNING => warnings_unfixed += 1,
                                    _ => {}
                                }

                                writeln!(
                                    output,
                                    "Failed to get quickfixes for {} at line {}: {}",
                                    if severity == DiagnosticSeverity::ERROR { "error" } else { "warning" },
                                    range.start.row + 1,
                                    err
                                )?;
                            }
                        }
                    }

                    // Save the buffer after applying fixes
                    if fixes_applied > 0 {
                        project
                            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                            .await?;

                        action_log.update(cx, |log, cx| {
                            log.buffer_edited(buffer.clone(), cx)
                        })?;
                    }

                    // Generate summary
                    if output.is_empty() {
                        Ok("No issues found in the file!".to_string())
                    } else {
                        writeln!(output, "\nSummary: Applied {} quickfixes. Remaining issues: {} errors, {} warnings.",
                            fixes_applied, errors_unfixed, warnings_unfixed)?;
                        Ok(output)
                    }
                })
            }
            _ => {
                let mut output = String::new();
                let mut files_with_diagnostics = Vec::new();

                // Collect all files with diagnostics for processing
                {
                    let project_ref = project.read(cx);
                    for (project_path, _, summary) in project_ref.diagnostic_summaries(true, cx) {
                        if summary.error_count > 0 || summary.warning_count > 0 {
                            if let Some(worktree) =
                                project_ref.worktree_for_id(project_path.worktree_id, cx)
                            {
                                let path_str = Path::new(worktree.read(cx).root_name())
                                    .join(project_path.path)
                                    .display()
                                    .to_string();

                                files_with_diagnostics.push(path_str);
                            }
                        }
                    }
                }

                // Create a task to process all files with diagnostics
                let project = project.clone();
                let action_log = action_log.clone();
                let files_to_process = files_with_diagnostics.clone();

                cx.spawn(async move |cx| {
                    let mut total_fixes_applied = 0;
                    let mut total_errors_unfixed = 0;
                    let mut total_warnings_unfixed = 0;

                    // Process each file with diagnostics
                    for file_path in files_to_process {
                        writeln!(output, "Processing {}...", file_path)?;

                        let Ok(Some(project_path)) = project.read_with(cx, |project, cx| project.find_project_path(&file_path, cx)) else {
                            writeln!(output, "  Could not resolve project path for {}", file_path)?;
                            continue;
                        };

                        let buffer_open_result = project.update(cx, |project, cx|
                            project.open_buffer(project_path, cx));

                        let Ok(buffer_handle) = buffer_open_result else {
                            writeln!(output, "  Failed to open buffer for {}", file_path)?;
                            continue;
                        };

                        let buffer = match buffer_handle.await {
                            Ok(buffer) => buffer,
                            Err(err) => {
                                writeln!(output, "  Failed to load buffer for {}: {}", file_path, err)?;
                                continue;
                            }
                        };

                        let mut file_fixes_applied = 0;
                        let mut file_errors_unfixed = 0;
                        let mut file_warnings_unfixed = 0;
                        let mut needs_save = false;

                        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                        // Process diagnostics for this file
                        for (_, group) in snapshot.diagnostic_groups(None) {
                            let entry = &group.entries[group.primary_ix];
                            let range = entry.range.to_point(&snapshot);
                            let severity = entry.diagnostic.severity;

                            // Skip informational and hint diagnostics
                            if severity != DiagnosticSeverity::ERROR &&
                               severity != DiagnosticSeverity::WARNING {
                                continue;
                            }

                            // Get code actions (quickfixes) for this diagnostic
                            let actions = project
                                .update(cx, |project, cx| {
                                    project.code_actions(
                                        &buffer,
                                        range.start..range.end,
                                        None,
                                        cx
                                    )
                                })?;

                            let actions_result = actions.await;

                            match actions_result {
                                Ok(actions) => {
                                    // Find quickfix actions
                                    let quickfixes = actions.into_iter().filter(|action| {
                                        if let LspAction::Action(code_action) = &action.lsp_action {
                                            if let Some(kind) = &code_action.kind {
                                                kind.as_str().starts_with("quickfix")
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    }).collect::<Vec<_>>();

                                    if !quickfixes.is_empty() {
                                        // Find the preferred quickfix (marked as isPreferred or the first one)
                                        let preferred_action = quickfixes.iter().find(|action| {
                                            if let LspAction::Action(code_action) = &action.lsp_action {
                                                code_action.is_preferred.unwrap_or(false)
                                            } else {
                                                false
                                            }
                                        }).unwrap_or(&quickfixes[0]);

                                        // Apply the quickfix
                                        let title = preferred_action.lsp_action.title().to_string();
                                        project
                                            .update(cx, |project, cx| {
                                                project.apply_code_action(buffer.clone(), preferred_action.clone(), true, cx)
                                            })?
                                            .await?;

                                        writeln!(output, "  Applied quickfix: {title}")?;
                                        file_fixes_applied += 1;
                                        needs_save = true;
                                    } else {
                                        // Track unfixed diagnostics
                                        match severity {
                                            DiagnosticSeverity::ERROR => file_errors_unfixed += 1,
                                            DiagnosticSeverity::WARNING => file_warnings_unfixed += 1,
                                            _ => {}
                                        }
                                    }
                                },
                                Err(_) => {
                                    // Track unfixed diagnostics
                                    match severity {
                                        DiagnosticSeverity::ERROR => file_errors_unfixed += 1,
                                        DiagnosticSeverity::WARNING => file_warnings_unfixed += 1,
                                        _ => {}
                                    }
                                }
                            }
                        }

                        // Save the buffer after applying fixes
                        if needs_save {
                            project
                                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                                .await?;

                            action_log.update(cx, |log, cx| {
                                log.buffer_edited(buffer.clone(), cx)
                            })?;
                        }

                        // Update totals
                        total_fixes_applied += file_fixes_applied;
                        total_errors_unfixed += file_errors_unfixed;
                        total_warnings_unfixed += file_warnings_unfixed;

                        // File summary
                        if file_fixes_applied > 0 || file_errors_unfixed > 0 || file_warnings_unfixed > 0 {
                            writeln!(output, "  {} quickfixes applied. Remaining: {} errors, {} warnings\n",
                                file_fixes_applied, file_errors_unfixed, file_warnings_unfixed)?;
                        } else {
                            writeln!(output, "  No issues fixed or found\n")?;
                        }
                    }

                    // Mark that we've checked diagnostics
                    action_log.update(cx, |action_log, _cx| {
                        action_log.checked_project_diagnostics();
                    })?;

                    // Generate overall summary
                    if files_with_diagnostics.is_empty() {
                        Ok("No issues found in the project!".to_string())
                    } else {
                        writeln!(output, "\nProject-wide summary: Applied {} quickfixes. Remaining issues: {} errors, {} warnings.",
                            total_fixes_applied, total_errors_unfixed, total_warnings_unfixed)?;
                        Ok(output)
                    }
                })
            }
        }
    }
}
