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
pub struct FindReplaceFileToolInput {
    /// The path of the file to modify.
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

    /// The unique string to find in the file. This string cannot be empty;
    /// if the string is empty, the tool call will fail. Remember, do not use this tool
    /// to create new files from scratch, or to overwrite existing files! Use a different
    /// approach if you want to do that.
    ///
    /// If this string appears more than once in the file, this tool call will fail,
    /// so it is absolutely critical that you verify ahead of time that the string
    /// is unique. You can search within the file to verify this.
    ///
    /// To make the string more likely to be unique, include a minimum of 3 lines of context
    /// before the string you actually want to find, as well as a minimum of 3 lines of
    /// context after the string you want to find. (These lines of context should appear
    /// in the `replace` string as well.) If 3 lines of context is not enough to obtain
    /// a string that appears only once in the file, then double the number of context lines
    /// until the string becomes unique. (Start with 3 lines before and 3 lines after
    /// though, because too much context is needlessly costly.)
    ///
    /// Do not alter the context lines of code in any way, and make sure to preserve all
    /// whitespace and indentation for all lines of code. This string must be exactly as
    /// it appears in the file, because this tool will do a literal find/replace, and if
    /// even one character in this string is different in any way from how it appears
    /// in the file, then the tool call will fail.
    ///
    /// If you get an error that the `find` string was not found, this means that either
    /// you made a mistake, or that the file has changed since you last looked at it.
    /// Either way, when this happens, you should retry doing this tool call until it
    /// succeeds, up to 3 times. Each time you retry, you should take another look at
    /// the exact text of the file in question, to make sure that you are searching for
    /// exactly the right string. Regardless of whether it was because you made a mistake
    /// or because the file changed since you last looked at it, you should be extra
    /// careful when retrying in this way. It's a bad experience for the user if
    /// this `find` string isn't found, so be super careful to get it exactly right!
    ///
    /// <example>
    /// If a file contains this code:
    ///
    /// ```ignore
    /// fn check_user_permissions(user_id: &str) -> Result<bool> {
    ///     // Check if user exists first
    ///     let user = database.find_user(user_id)?;
    ///
    ///     // This is the part we want to modify
    ///     if user.role == "admin" {
    ///         return Ok(true);
    ///     }
    ///
    ///     // Check other permissions
    ///     check_custom_permissions(user_id)
    /// }
    /// ```
    ///
    /// Your find string should include at least 3 lines of context before and after the part
    /// you want to change:
    ///
    /// ```ignore
    /// fn check_user_permissions(user_id: &str) -> Result<bool> {
    ///     // Check if user exists first
    ///     let user = database.find_user(user_id)?;
    ///
    ///     // This is the part we want to modify
    ///     if user.role == "admin" {
    ///         return Ok(true);
    ///     }
    ///
    ///     // Check other permissions
    ///     check_custom_permissions(user_id)
    /// }
    /// ```
    ///
    /// And your replace string might look like:
    ///
    /// ```ignore
    /// fn check_user_permissions(user_id: &str) -> Result<bool> {
    ///     // Check if user exists first
    ///     let user = database.find_user(user_id)?;
    ///
    ///     // This is the part we want to modify
    ///     if user.role == "admin" || user.role == "superuser" {
    ///         return Ok(true);
    ///     }
    ///
    ///     // Check other permissions
    ///     check_custom_permissions(user_id)
    /// }
    /// ```
    /// </example>
    pub find: String,

    /// The string to replace the one unique occurrence of the find string with.
    pub replace: String,
}

pub struct FindReplaceFileTool;

impl Tool for FindReplaceFileTool {
    fn name(&self) -> String {
        "find_replace_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("find_replace_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<FindReplaceFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<FindReplaceFileToolInput>(input.clone()) {
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
        let input = match serde_json::from_value::<FindReplaceFileToolInput>(input) {
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

            if input.find.is_empty() {
                return Err(anyhow!("`find` string cannot be empty. Use a different tool if you want to create a file."));
            }

            if input.find == input.replace {
                return Err(anyhow!("The `find` and `replace` strings are identical, so no changes would be made."));
            }

            let result = cx
                .background_spawn(async move {
                    // Try to match exactly
                    let diff = replace_exact(&input.find, &input.replace, &snapshot)
                    .await
                    // If that fails, try being flexible about indentation
                    .or_else(|| replace_with_flexible_indent(&input.find, &input.replace, &snapshot))?;

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
                            "{} is empty, so the provided `find` string wasn't found.",
                            input.path.display()
                        )
                    } else {
                        anyhow!("Failed to match the provided `find` string")
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
