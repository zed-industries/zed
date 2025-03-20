use std::path::Path;
use std::sync::Arc;

use crate::edit_files_tool::replace::replace_exact;
use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool, ToolSource};
use collections::HashSet;
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language::Point;
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum TextEditorToolInput {
    /// View file contents or directory listing
    View {
        /// The path to the file or directory to view
        path: Arc<Path>,
        /// Optional line range to view (1-based, inclusive)
        #[serde(default)]
        view_range: Option<(usize, usize)>,
    },

    /// Replace specific text in a file
    StrReplace {
        /// The path to the file to modify
        path: Arc<Path>,
        /// The text to replace (must match exactly)
        old_str: String,
        /// The new text to insert
        new_str: String,
    },

    /// Create a new file with content
    Create {
        /// The path where the new file should be created
        path: Arc<Path>,
        /// The content to write to the new file
        file_text: String,
    },

    /// Insert text at a specific line
    Insert {
        /// The path to the file to modify
        path: Arc<Path>,
        /// The line number after which to insert (0 for beginning)
        insert_line: usize,
        /// The text to insert
        new_str: String,
    },

    /// Undo the last edit made to a file
    UndoEdit {
        /// The path to the file whose last edit should be undone
        path: Arc<Path>,
    },
}

pub struct TextEditorTool;

impl Tool for TextEditorTool {
    fn name(&self) -> String {
        "str_replace_editor".into()
    }

    fn description(&self) -> String {
        String::new()
    }

    fn source(&self) -> ToolSource {
        ToolSource::RefactorMeProviderDefined {
            provider_name: "Anthropic",
            // todo! Depends on model!
            tool_type: "text_editor_20250124",
        }
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(TextEditorToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<TextEditorToolInput>(input.clone()) {
            Ok(input) => match input {
                TextEditorToolInput::View { path, .. } => format!("View `{}`", path.display()),
                TextEditorToolInput::StrReplace { path, .. } => {
                    format!("Edit file `{}`", path.display())
                }
                TextEditorToolInput::Create { path, .. } => {
                    format!("Create file `{}`", path.display())
                }
                TextEditorToolInput::Insert { path, .. } => {
                    format!("Edit file `{}`", path.display())
                }
                TextEditorToolInput::UndoEdit { path } => {
                    format!("Undo edit in `{}`", path.display())
                }
            },
            Err(_) => "Edit file".to_string(),
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
        // Basic input validation
        let input = match serde_json::from_value::<TextEditorToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        cx.spawn(async move |cx| {
            // Handle each command type
            let result = match input {
                TextEditorToolInput::View { view_range, path } => {
                    let buffer = open_buffer(&project, &path, cx).await?;

                    buffer.read_with(cx, |buffer, _cx| match view_range {
                        Some((start_row, end_row)) => {
                            let start = Point::new(start_row.saturating_sub(1) as u32, 0);
                            let end = Point::new(end_row as u32, 0);

                            buffer.text_for_range(start..end).collect::<String>()
                        }
                        None => buffer.text(),
                    })?
                }
                TextEditorToolInput::StrReplace {
                    old_str,
                    new_str,
                    path,
                } => {
                    let buffer = open_buffer(&project, &path, cx).await?;
                    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                    // todo! anthropic requires that we fail if >1 match is found
                    let diff_result = cx
                        .background_spawn(async move {
                            replace_exact(&old_str, &new_str, &snapshot).await
                        })
                        .await;

                    // todo! look at reference implementation to see what's the best response
                    match diff_result {
                        Some(diff) => {
                            buffer.update(cx, |buffer, cx| buffer.apply_diff(diff, cx))?;
                            save_changed_buffer(project, action_log, buffer, cx).await?;

                            "Replaced!".to_string()
                        }
                        None => return Err(anyhow!("Failed to match `old_str`")),
                    }
                }
                TextEditorToolInput::Create { path, file_text } => {
                    let buffer = open_buffer(&project, &path, cx).await?;
                    buffer.update(cx, |buffer, cx| buffer.set_text(file_text, cx))?;
                    save_changed_buffer(project, action_log, buffer, cx).await?;

                    format!("Created `{}`", path.display())
                }
                TextEditorToolInput::Insert {
                    insert_line: _,
                    new_str: _,
                    ..
                } => {
                    // todo!
                    return Err(anyhow!(format!(
                        "Insert command not available. Use str_replace to insert."
                    )));
                }
                TextEditorToolInput::UndoEdit { .. } => {
                    // todo!
                    return Err(anyhow!(format!(
                        "Undo command not available. Use str_replace to undo."
                    )));
                }
            };

            Ok(result)
        })
    }
}

async fn open_buffer(
    project: &Entity<Project>,
    path: &Path,
    cx: &mut AsyncApp,
) -> Result<Entity<language::Buffer>> {
    let project_path =
        // todo! how can we tell the model to not use abs paths
        project.read_with(cx, |project, cx| {
            if path.is_absolute() {
                project.project_path_for_absolute_path(&path, cx)
            } else {
                project.find_project_path(path, cx)
            }
        })?;

    let Some(project_path) = project_path else {
        return Err(anyhow!("Path {} not found in project", path.display()));
    };

    project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
        .await
}

async fn save_changed_buffer(
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    buffer: Entity<language::Buffer>,
    cx: &mut AsyncApp,
) -> Result<()> {
    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
        .await?;

    action_log.update(cx, |log, cx| {
        let mut changed_buffers = HashSet::default();
        changed_buffers.insert(buffer);

        log.buffer_edited(changed_buffers, cx);
    })?;

    Ok(())
}
