use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MovePathToolInput {
    /// The source path of the file or directory to move/rename.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can move the first file by providing a source_path of "directory1/a/something.txt"
    /// </example>
    pub source_path: String,

    /// The destination path where the file or directory should be moved/renamed to.
    /// If the paths are the same except for the filename, then this will be a rename.
    ///
    /// <example>
    /// To move "directory1/a/something.txt" to "directory2/b/renamed.txt",
    /// provide a destination_path of "directory2/b/renamed.txt"
    /// </example>
    pub destination_path: String,
}

pub struct MovePathTool;

impl Tool for MovePathTool {
    fn name(&self) -> String {
        "move_path".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./move_path_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::ArrowRightLeft
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<MovePathToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<MovePathToolInput>(input.clone()) {
            Ok(input) => {
                let src = MarkdownString::inline_code(&input.source_path);
                let dest = MarkdownString::inline_code(&input.destination_path);
                let src_path = Path::new(&input.source_path);
                let dest_path = Path::new(&input.destination_path);

                match dest_path
                    .file_name()
                    .and_then(|os_str| os_str.to_os_string().into_string().ok())
                {
                    Some(filename) if src_path.parent() == dest_path.parent() => {
                        let filename = MarkdownString::inline_code(&filename);
                        format!("Rename {src} to {filename}")
                    }
                    _ => {
                        format!("Move {src} to {dest}")
                    }
                }
            }
            Err(_) => "Move path".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<MovePathToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };
        let rename_task = project.update(cx, |project, cx| {
            match project
                .find_project_path(&input.source_path, cx)
                .and_then(|project_path| project.entry_for_path(&project_path, cx))
            {
                Some(entity) => match project.find_project_path(&input.destination_path, cx) {
                    Some(project_path) => project.rename_entry(entity.id, project_path.path, cx),
                    None => Task::ready(Err(anyhow!(
                        "Destination path {} was outside the project.",
                        input.destination_path
                    ))),
                },
                None => Task::ready(Err(anyhow!(
                    "Source path {} was not found in the project.",
                    input.source_path
                ))),
            }
        });

        cx.background_spawn(async move {
            match rename_task.await {
                Ok(_) => Ok(format!(
                    "Moved {} to {}",
                    input.source_path, input.destination_path
                )),
                Err(err) => Err(anyhow!(
                    "Failed to move {} to {}: {}",
                    input.source_path,
                    input.destination_path,
                    err
                )),
            }
        })
        .into()
    }
}
