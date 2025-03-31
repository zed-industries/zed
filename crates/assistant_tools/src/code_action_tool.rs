use anyhow::{anyhow, Context as _, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{self, Anchor, Buffer, ToPointUtf16};
use language_model::LanguageModelRequestMessage;
use project::{self, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{ops::Range, sync::Arc};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CodeActionToolInput {
    /// The relative path to the file containing the text range.
    ///
    /// WARNING: you MUST start this path with one of the project's root directories.
    pub path: String,

    /// The code action to perform.
    pub action: CodeActionType,

    /// The text that comes immediately before the text range in the file.
    pub context_before_range: String,

    /// The text range. This text must appear in the file right between `context_before_range`
    /// and `context_after_range`.
    ///
    /// The file must contain exactly one occurrence of `context_before_range` followed by
    /// `text_range` followed by `context_after_range`. If the file contains zero occurrences,
    /// or if it contains more than one occurrence, the tool will fail, so it is absolutely
    /// critical that you verify ahead of time that the string is unique. You can search
    /// the file's contents to verify this ahead of time.
    ///
    /// To make the string more likely to be unique, include a minimum of 1 line of context
    /// before the text range, as well as a minimum of 1 line of context after the text range.
    /// If these lines of context are not enough to obtain a string that appears only once
    /// in the file, then double the number of context lines until the string becomes unique.
    /// (Start with 1 line before and 1 line after though, because too much context is
    /// needlessly costly.)
    ///
    /// Do not alter the context lines of code in any way, and make sure to preserve all
    /// whitespace and indentation for all lines of code. The combined string must be exactly
    /// as it appears in the file, or else this tool call will fail.
    pub text_range: String,

    /// The text that comes immediately after the text range in the file.
    pub context_after_range: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionType {
    /// Rename the text range to a new name
    Rename(String),
    
    /// List available code actions for the text range
    ListAvailable,
}

pub struct CodeActionTool;

impl Tool for CodeActionTool {
    fn name(&self) -> String {
        "code-action".into()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./code_action_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Wand
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(CodeActionToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<CodeActionToolInput>(input.clone()) {
            Ok(input) => {
                match input.action {
                    CodeActionType::Rename(ref new_name) => {
                        format!("Rename '{}' to '{}'", input.text_range, new_name)
                    }
                    CodeActionType::ListAvailable => {
                        format!("List available code actions for '{}'", input.text_range)
                    }
                }
            }
            Err(_) => "Perform code action".to_string(),
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
        let input = match serde_json::from_value::<CodeActionToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        cx.spawn(async move |cx| {
            let buffer = {
                let project_path = project.read_with(cx, |project, cx| {
                    project
                        .find_project_path(&input.path, cx)
                        .context("Path not found in project")
                })??;

                project.update(cx, |project, cx| project.open_buffer(project_path, cx))?.await?
            };

            action_log.update(cx, |action_log, cx| {
                action_log.buffer_read(buffer.clone(), cx);
            })?;

            let range = {
                let Some(range) = buffer.read_with(cx, |buffer, _cx| {
                    find_text_range(&buffer, &input.context_before_range, &input.text_range, &input.context_after_range)
                })? else {
                    return Err(anyhow!(
                        "Failed to locate the text specified by context_before_range, text_range, and context_after_range. Make sure context_before_range and context_after_range each match exactly once in the file."
                    ));
                };

                range
            };

            match input.action {
                CodeActionType::Rename(ref new_name) => {
                    let position = buffer.read_with(cx, |buffer, _| {
                        range.start.to_point_utf16(&buffer.snapshot())
                    })?;

                    // Execute the rename operation
                    let _ = project
                        .update(cx, |project, cx| {
                            project.perform_rename(buffer.clone(), position, new_name.clone(), cx)
                        })?
                        .await?;

                    action_log.update(cx, |log, cx| {
                        log.buffer_edited(buffer.clone(), Vec::new(), cx)
                    })?;

                    Ok(format!("Renamed '{}' to '{}'", input.text_range, new_name))
                }
                CodeActionType::ListAvailable => {
                    let (position_start, position_end) = buffer.read_with(cx, |buffer, _| {
                        let snapshot = buffer.snapshot();
                        (
                            range.start.to_point_utf16(&snapshot),
                            range.end.to_point_utf16(&snapshot)
                        )
                    })?;

                    // Convert position to display coordinates (1-based)
                    let position_start_display = language::Point {
                        row: position_start.row + 1,
                        column: position_start.column + 1,
                    };
                    
                    let position_end_display = language::Point {
                        row: position_end.row + 1,
                        column: position_end.column + 1,
                    };

                    // We already have the UTF-16 positions from our earlier calculation

                    // Get code actions for the range
                    let actions = project
                        .update(cx, |project, cx| {
                            project.code_actions(&buffer, range.clone(), None, cx)
                        })?
                        .await?;

                    // Format the results
                    let mut result = format!(
                        "Available code actions for text range '{}' at position {}:{} to {}:{} (UTF-16 coordinates):\n\n", 
                        input.text_range,
                        position_start_display.row, position_start_display.column,
                        position_end_display.row, position_end_display.column
                    );

                    if actions.is_empty() {
                        result.push_str("No code actions available for this range.");
                    } else {
                        for (i, action) in actions.iter().enumerate() {
                            let title = match &action.lsp_action {
                                project::LspAction::Action(code_action) => code_action.title.as_str(),
                                project::LspAction::Command(command) => command.title.as_str(),
                                project::LspAction::CodeLens(code_lens) => {
                                    if let Some(cmd) = &code_lens.command {
                                        cmd.title.as_str()
                                    } else {
                                        "Unknown code lens"
                                    }
                                },
                            };
                            
                            let kind = match &action.lsp_action {
                                project::LspAction::Action(code_action) => {
                                    if let Some(kind) = &code_action.kind {
                                        kind.as_str()
                                    } else {
                                        "unknown"
                                    }
                                },
                                project::LspAction::Command(_) => "command",
                                project::LspAction::CodeLens(_) => "code_lens",
                            };
                            
                            result.push_str(&format!("{}. {} ({})\n", i + 1, title, kind));
                        }
                    }

                    Ok(result)
                }
            }
        })
    }
}

/// Finds the range of the text in the buffer, if it appears between context_before_range
/// and context_after_range, and if that combined string has one unique result in the buffer.
fn find_text_range(
    buffer: &Buffer,
    context_before_range: &str,
    text_range: &str,
    context_after_range: &str,
) -> Option<Range<Anchor>> {
    let snapshot = buffer.snapshot();
    let text = snapshot.text();
    let search_string = format!("{context_before_range}{text_range}{context_after_range}");
    let mut positions = text.match_indices(&search_string);
    let position = positions.next()?.0;

    // The combined string must appear exactly once.
    if positions.next().is_some() {
        return None;
    }

    let range_start = position + context_before_range.len();
    let range_end = range_start + text_range.len();
    let range_start_anchor = snapshot.anchor_before(snapshot.offset_to_point(range_start));
    let range_end_anchor = snapshot.anchor_before(snapshot.offset_to_point(range_end));

    Some(range_start_anchor..range_end_anchor)
}