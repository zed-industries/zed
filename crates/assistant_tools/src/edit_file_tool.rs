use crate::{replace::replace_with_flexible_indent, schema::json_schema_for};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::IconName;

use crate::replace::replace_exact;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolInput {
    /// The full path of the file to modify in the project.
    ///
    /// WARNING: When specifying which file path need changing, you MUST
    /// start each path with one of the project's root directories.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - backend
    /// - frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with root-1. Without that, the path
    /// would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,

    /// A user-friendly markdown description of what's being replaced. This will be shown in the UI.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    pub display_description: String,

    /// The text to replace.
    pub old_string: String,

    /// The text to replace it with.
    pub new_string: String,
}

pub struct EditFileTool;

impl Tool for EditFileTool {
    fn name(&self) -> String {
        "edit_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("edit_file_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<EditFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<EditFileToolInput>(input.clone()) {
            Ok(input) => input.display_description,
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
    ) -> ToolResult {
        let input = match serde_json::from_value::<EditFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        cx.spawn(async move |cx: &mut AsyncApp| {
            let project_path = project.read_with(cx, |project, cx| {
                project
                    .find_project_path(&input.path, cx)
                    .context("Path not found in project")
            })??;

            let buffer = project
                .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                .await?;

            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            if input.old_string.is_empty() {
                return Err(anyhow!("`old_string` cannot be empty. Use a different tool if you want to create a file."));
            }

            if input.old_string == input.new_string {
                return Err(anyhow!("The `old_string` and `new_string` are identical, so no changes would be made."));
            }

            let result = cx
                .background_spawn(async move {
                    // Try to match exactly
                    let diff = replace_exact(&input.old_string, &input.new_string, &snapshot)
                    .await
                    // If that fails, try being flexible about indentation
                    .or_else(|| replace_with_flexible_indent(&input.old_string, &input.new_string, &snapshot))?;

                    if diff.edits.is_empty() {
                        return None;
                    }

                    let old_text = snapshot.text();

                    Some((old_text, diff))
                })
                .await;

            let Some((old_text, diff)) = result else {
                let err = buffer.read_with(cx, |buffer, _cx| {
                    let file_exists = buffer
                        .file()
                        .map_or(false, |file| file.disk_state().exists());

                    if !file_exists {
                        anyhow!("{} does not exist", input.path.display())
                    } else if buffer.is_empty() {
                        anyhow!(
                            "{} is empty, so the provided `old_string` wasn't found.",
                            input.path.display()
                        )
                    } else {
                        anyhow!("Failed to match the provided `old_string`")
                    }
                })?;

                return Err(err)
            };

            let snapshot = cx.update(|cx| {
                action_log.update(cx, |log, cx| {
                    log.buffer_read(buffer.clone(), cx)
                });
                let snapshot = buffer.update(cx, |buffer, cx| {
                    buffer.finalize_last_transaction();
                    buffer.apply_diff(diff, cx);
                    buffer.finalize_last_transaction();
                    buffer.snapshot()
                });
                action_log.update(cx, |log, cx| {
                    log.buffer_edited(buffer.clone(), cx)
                });
                snapshot
            })?;

            project.update( cx, |project, cx| {
                project.save_buffer(buffer, cx)
            })?.await?;

            let diff_str = cx.background_spawn(async move {
                let new_text = snapshot.text();
                language::unified_diff(&old_text, &new_text)
            }).await;


            Ok(format!("Edited {}:\n\n```diff\n{}\n```", input.path.display(), diff_str))

        }).into()
    }
}
