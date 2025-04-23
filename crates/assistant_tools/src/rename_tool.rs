use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, Entity, Task};
use language::{self, Buffer, ToPointUtf16};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;

use crate::schema::json_schema_for;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RenameToolInput {
    /// The relative path to the file containing the symbol to rename.
    ///
    /// WARNING: you MUST start this path with one of the project's root directories.
    pub path: String,

    /// The new name to give to the symbol.
    pub new_name: String,

    /// The text that comes immediately before the symbol in the file.
    pub context_before_symbol: String,

    /// The symbol to rename. This text must appear in the file right between
    /// `context_before_symbol` and `context_after_symbol`.
    ///
    /// The file must contain exactly one occurrence of `context_before_symbol` followed by
    /// `symbol` followed by `context_after_symbol`. If the file contains zero occurrences,
    /// or if it contains more than one occurrence, the tool will fail, so it is absolutely
    /// critical that you verify ahead of time that the string is unique. You can search
    /// the file's contents to verify this ahead of time.
    ///
    /// To make the string more likely to be unique, include a minimum of 1 line of context
    /// before the symbol, as well as a minimum of 1 line of context after the symbol.
    /// If these lines of context are not enough to obtain a string that appears only once
    /// in the file, then double the number of context lines until the string becomes unique.
    /// (Start with 1 line before and 1 line after though, because too much context is
    /// needlessly costly.)
    ///
    /// Do not alter the context lines of code in any way, and make sure to preserve all
    /// whitespace and indentation for all lines of code. The combined string must be exactly
    /// as it appears in the file, or else this tool call will fail.
    pub symbol: String,

    /// The text that comes immediately after the symbol in the file.
    pub context_after_symbol: String,
}

pub struct RenameTool;

impl Tool for RenameTool {
    fn name(&self) -> String {
        "rename".into()
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./rename_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<RenameToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<RenameToolInput>(input.clone()) {
            Ok(input) => {
                format!("Rename '{}' to '{}'", input.symbol, input.new_name)
            }
            Err(_) => "Rename symbol".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<RenameToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
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
                action_log.track_buffer(buffer.clone(), cx);
            })?;

            let position = {
                let Some(position) = buffer.read_with(cx, |buffer, _cx| {
                    find_symbol_position(&buffer, &input.context_before_symbol, &input.symbol, &input.context_after_symbol)
                })? else {
                    return Err(anyhow!(
                        "Failed to locate the symbol specified by context_before_symbol, symbol, and context_after_symbol. Make sure context_before_symbol and context_after_symbol each match exactly once in the file."
                    ));
                };

                buffer.read_with(cx, |buffer, _| {
                    position.to_point_utf16(&buffer.snapshot())
                })?
            };

            project
                .update(cx, |project, cx| {
                    project.perform_rename(buffer.clone(), position, input.new_name.clone(), cx)
                })?
                .await?;

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                .await?;

            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx)
            })?;

            Ok(format!("Renamed '{}' to '{}'", input.symbol, input.new_name))
        }).into()
    }
}

/// Finds the position of the symbol in the buffer, if it appears between context_before_symbol
/// and context_after_symbol, and if that combined string has one unique result in the buffer.
///
/// If an exact match fails, it tries adding a newline to the end of context_before_symbol and
/// to the beginning of context_after_symbol to accommodate line-based context matching.
fn find_symbol_position(
    buffer: &Buffer,
    context_before_symbol: &str,
    symbol: &str,
    context_after_symbol: &str,
) -> Option<language::Anchor> {
    let snapshot = buffer.snapshot();
    let text = snapshot.text();

    // First try with exact match
    let search_string = format!("{context_before_symbol}{symbol}{context_after_symbol}");
    let mut positions = text.match_indices(&search_string);
    let position_result = positions.next();

    if let Some(position) = position_result {
        // Check if the matched string is unique
        if positions.next().is_none() {
            let symbol_start = position.0 + context_before_symbol.len();
            let symbol_start_anchor =
                snapshot.anchor_before(snapshot.offset_to_point(symbol_start));

            return Some(symbol_start_anchor);
        }
    }

    // If exact match fails or is not unique, try with line-based context
    // Add a newline to the end of before context and beginning of after context
    let line_based_before = if context_before_symbol.ends_with('\n') {
        context_before_symbol.to_string()
    } else {
        format!("{context_before_symbol}\n")
    };

    let line_based_after = if context_after_symbol.starts_with('\n') {
        context_after_symbol.to_string()
    } else {
        format!("\n{context_after_symbol}")
    };

    let line_search_string = format!("{line_based_before}{symbol}{line_based_after}");
    let mut line_positions = text.match_indices(&line_search_string);
    let line_position = line_positions.next()?;

    // The line-based search string must also appear exactly once
    if line_positions.next().is_some() {
        return None;
    }

    let line_symbol_start = line_position.0 + line_based_before.len();
    let line_symbol_start_anchor =
        snapshot.anchor_before(snapshot.offset_to_point(line_symbol_start));

    Some(line_symbol_start_anchor)
}
