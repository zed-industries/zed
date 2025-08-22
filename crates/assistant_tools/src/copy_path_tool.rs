use crate::schema::json_schema_for;
use action_log::ActionLog;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{Tool, ToolResult};
use gpui::AnyWindowHandle;
use gpui::{App, AppContext, Entity, Task};
use language_model::LanguageModel;
use language_model::{LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::markdown::MarkdownInlineCode;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CopyPathToolInput {
    /// The source path of the file or directory to copy.
    /// If a directory is specified, its contents will be copied recursively (like `cp -r`).
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can copy the first file by providing a source_path of "directory1/a/something.txt"
    /// </example>
    pub source_path: String,

    /// The destination path where the file or directory should be copied to.
    ///
    /// <example>
    /// To copy "directory1/a/something.txt" to "directory2/b/copy.txt",
    /// provide a destination_path of "directory2/b/copy.txt"
    /// </example>
    pub destination_path: String,
}

pub struct CopyPathTool;

impl Tool for CopyPathTool {
    fn name(&self) -> String {
        "copy_path".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &Entity<Project>, _: &App) -> bool {
        false
    }

    fn may_perform_edits(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./copy_path_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::ToolCopy
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<CopyPathToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<CopyPathToolInput>(input.clone()) {
            Ok(input) => {
                let src = MarkdownInlineCode(&input.source_path);
                let dest = MarkdownInlineCode(&input.destination_path);
                format!("Copy {src} to {dest}")
            }
            Err(_) => "Copy path".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<CopyPathToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };
        let copy_task = project.update(cx, |project, cx| {
            match project
                .find_project_path(&input.source_path, cx)
                .and_then(|project_path| project.entry_for_path(&project_path, cx))
            {
                Some(entity) => match project.find_project_path(&input.destination_path, cx) {
                    Some(project_path) => {
                        project.copy_entry(entity.id, None, project_path.path, cx)
                    }
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
            let _ = copy_task.await.with_context(|| {
                format!(
                    "Copying {} to {}",
                    input.source_path, input.destination_path
                )
            })?;
            Ok(format!("Copied {} to {}", input.source_path, input.destination_path).into())
        })
        .into()
    }
}
