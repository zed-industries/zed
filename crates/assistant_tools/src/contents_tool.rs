use std::sync::Arc;

use crate::{code_symbols_tool::file_outline, schema::json_schema_for};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};
use itertools::Itertools;
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path};
use ui::IconName;
use util::markdown::MarkdownString;

/// If the model requests to read a file whose size exceeds this, then
/// the tool will return the file's symbol outline instead of its contents,
/// and suggest trying again using line ranges from the outline.
const MAX_FILE_SIZE_TO_READ: usize = 16384;

/// If the model requests to list the entries in a directory with more
/// entries than this, then the tool will return a subset of the entries
/// and suggest trying again.
const MAX_DIR_ENTRIES: usize = 1024;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContentsToolInput {
    /// The relative path of the file or directory to access.
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
    /// If you want to list contents in the directory `directory2/subfolder`, you should use the path `directory2/subfolder`.
    /// </example>
    pub path: String,

    /// Optional position (1-based index) to start reading on, if you want to read a subset of the contents.
    /// When reading a file, this refers to a line number in the file (e.g. 1 is the first line).
    /// When reading a directory, this refers to the number of the directory entry (e.g. 1 is the first entry).
    ///
    /// Defaults to 1.
    pub start: Option<u32>,

    /// Optional position (1-based index) to end reading on, if you want to read a subset of the contents.
    /// When reading a file, this refers to a line number in the file (e.g. 1 is the first line).
    /// When reading a directory, this refers to the number of the directory entry (e.g. 1 is the first entry).
    ///
    /// Defaults to reading until the end of the file or directory.
    pub end: Option<u32>,
}

pub struct ContentsTool;

impl Tool for ContentsTool {
    fn name(&self) -> String {
        "contents".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./contents_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::FileSearch
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ContentsToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<ContentsToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path);

                match (input.start, input.end) {
                    (Some(start), None) => format!("Read {path} (from line {start})"),
                    (Some(start), Some(end)) => {
                        format!("Read {path} (lines {start}-{end})")
                    }
                    _ => format!("Read {path}"),
                }
            }
            Err(_) => "Read file or directory".to_string(),
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
        let input = match serde_json::from_value::<ContentsToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        // Sometimes models will return these even though we tell it to give a path and not a glob.
        // When this happens, just list the root worktree directories.
        if matches!(input.path.as_str(), "." | "" | "./" | "*") {
            let output = project
                .read(cx)
                .worktrees(cx)
                .filter_map(|worktree| {
                    worktree.read(cx).root_entry().and_then(|entry| {
                        if entry.is_dir() {
                            entry.path.to_str()
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
                .join("\n");

            return Task::ready(Ok(output)).into();
        }

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", &input.path))).into();
        };

        let Some(worktree) = project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("Worktree not found"))).into();
        };
        let worktree = worktree.read(cx);

        let Some(entry) = worktree.entry_for_path(&project_path.path) else {
            return Task::ready(Err(anyhow!("Path not found: {}", input.path))).into();
        };

        // If it's a directory, list its contents
        if entry.is_dir() {
            let mut output = String::new();
            let start_index = input
                .start
                .map(|line| (line as usize).saturating_sub(1))
                .unwrap_or(0);
            let end_index = input
                .end
                .map(|line| (line as usize).saturating_sub(1))
                .unwrap_or(MAX_DIR_ENTRIES);
            let mut skipped = 0;

            for (index, entry) in worktree.child_entries(&project_path.path).enumerate() {
                if index >= start_index && index <= end_index {
                    writeln!(
                        output,
                        "{}",
                        Path::new(worktree.root_name()).join(&entry.path).display(),
                    )
                    .unwrap();
                } else {
                    skipped += 1;
                }
            }

            if output.is_empty() {
                output.push_str(&input.path);
                output.push_str(" is empty.");
            }

            if skipped > 0 {
                write!(
                    output,
                    "\n\nNote: Skipped {skipped} entries. Adjust start and end to see other entries.",
                ).ok();
            }

            Task::ready(Ok(output)).into()
        } else {
            // It's a file, so read its contents
            let file_path = input.path.clone();
            cx.spawn(async move |cx| {
                let buffer = cx
                    .update(|cx| {
                        project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                    })?
                    .await?;

                if input.start.is_some() || input.end.is_some() {
                    let result = buffer.read_with(cx, |buffer, _cx| {
                        let text = buffer.text();
                        let start = input.start.unwrap_or(1);
                        let lines = text.split('\n').skip(start as usize - 1);
                        if let Some(end) = input.end {
                            let count = end.saturating_sub(start).max(1); // Ensure at least 1 line
                            Itertools::intersperse(lines.take(count as usize), "\n").collect()
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
                        let result = buffer.read_with(cx, |buffer, _cx| buffer.text())?;

                        action_log.update(cx, |log, cx| {
                            log.buffer_read(buffer, cx);
                        })?;

                        Ok(result)
                    } else {
                        // File is too big, so return its outline and a suggestion to
                        // read again with a line number range specified.
                        let outline = file_outline(project, file_path, action_log, None, cx).await?;

                        Ok(format!("This file was too big to read all at once. Here is an outline of its symbols:\n\n{outline}\n\nUsing the line numbers in this outline, you can call this tool again while specifying the start and end fields to see the implementations of symbols in the outline."))
                    }
                }
            }).into()
        }
    }
}
