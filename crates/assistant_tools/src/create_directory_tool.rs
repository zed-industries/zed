use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateDirectoryToolInput {
    /// The path of the new directory.
    ///
    /// <example>
    /// If the project has the following structure:
    ///
    /// - directory1/
    /// - directory2/
    ///
    /// You can create a new directory by providing a path of "directory1/new_directory"
    /// </example>
    pub path: String,
}

pub struct CreateDirectoryTool;

impl Tool for CreateDirectoryTool {
    fn name(&self) -> String {
        "create-directory".into()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./create_directory_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Folder
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(CreateDirectoryToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<CreateDirectoryToolInput>(input.clone()) {
            Ok(input) => {
                format!(
                    "Create directory {}",
                    MarkdownString::inline_code(&input.path)
                )
            }
            Err(_) => "Create directory".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<CreateDirectoryToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };
        let project_path = match project.read(cx).find_project_path(&input.path, cx) {
            Some(project_path) => project_path,
            None => return Task::ready(Err(anyhow!("Path to create was outside the project"))),
        };
        let destination_path: Arc<str> = input.path.as_str().into();

        cx.spawn(async move |cx| {
            project
                .update(cx, |project, cx| {
                    project.create_entry(project_path.clone(), true, cx)
                })?
                .await
                .map_err(|err| anyhow!("Unable to create directory {destination_path}: {err}"))?;

            Ok(format!("Created directory {destination_path}"))
        })
    }
}
