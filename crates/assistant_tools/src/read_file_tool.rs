use std::sync::Arc;

use crate::{code_symbols_tool::file_outline, schema::json_schema_for};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, Entity, Task};
use itertools::Itertools;
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

/// If the model requests to read a file whose size exceeds this, then
/// the tool will return an error along with the model's symbol outline,
/// and suggest trying again using line ranges from the outline.
const MAX_FILE_SIZE_TO_READ: usize = 16384;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileToolInput {
    /// The relative path of the file to read.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - directory1
    /// - directory2
    ///
    /// If you want to access `file.txt` in `directory1`, you should use the path `directory1/file.txt`.
    /// If you want to access `file.txt` in `directory2`, you should use the path `directory2/file.txt`.
    /// </example>
    pub path: String,

    /// Optional line number to start reading on (1-based index)
    #[serde(default)]
    pub start_line: Option<usize>,

    /// Optional line number to end reading on (1-based index)
    #[serde(default)]
    pub end_line: Option<usize>,
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> String {
        "read_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./read_file_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::FileSearch
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ReadFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<ReadFileToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path);
                match (input.start_line, input.end_line) {
                    (Some(start), None) => format!("Read file {path} (from line {start})"),
                    (Some(start), Some(end)) => format!("Read file {path} (lines {start}-{end})"),
                    _ => format!("Read file {path}"),
                }
            }
            Err(_) => "Read file".to_string(),
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
        let input = match serde_json::from_value::<ReadFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", &input.path,))).into();
        };

        let file_path = input.path.clone();
        cx.spawn(async move |cx| {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            // Check if specific line ranges are provided
            if input.start_line.is_some() || input.end_line.is_some() {
                let result = buffer.read_with(cx, |buffer, _cx| {
                    let text = buffer.text();
                    let start = input.start_line.unwrap_or(1);
                    let lines = text.split('\n').skip(start - 1);
                    if let Some(end) = input.end_line {
                        let count = end.saturating_sub(start).max(1); // Ensure at least 1 line
                        Itertools::intersperse(lines.take(count), "\n").collect()
                    } else {
                        Itertools::intersperse(lines, "\n").collect()
                    }
                })?;

                action_log.update(cx, |log, cx| {
                    log.buffer_read(buffer, cx);
                })?;

                Ok(result)
            } else {
                // No line ranges specified, so check file size to see if it's too big.
                let file_size = buffer.read_with(cx, |buffer, _cx| buffer.text().len())?;

                if file_size <= MAX_FILE_SIZE_TO_READ {
                    // File is small enough, so return its contents.
                    let result = buffer.read_with(cx, |buffer, _cx| buffer.text())?;

                    action_log.update(cx, |log, cx| {
                        log.buffer_read(buffer, cx);
                    })?;

                    Ok(result)
                } else {
                    // File is too big, so return an error with the outline
                    // and a suggestion to read again with line numbers.
                    let outline = file_outline(project, file_path, action_log, None, 0, cx).await?;

                    Ok(format!("This file was too big to read all at once. Here is an outline of its symbols:\n\n{outline}\n\nUsing the line numbers in this outline, you can call this tool again while specifying the start_line and end_line fields to see the implementations of symbols in the outline."))
                }
            }
        }).into()
    }
}
