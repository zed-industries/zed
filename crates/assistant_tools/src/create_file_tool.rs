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
pub struct CreateFileToolInput {
    /// The path where the file should be created.
    ///
    /// <example>
    /// If the project has the following structure:
    ///
    /// - directory1/
    /// - directory2/
    ///
    /// You can create a new file by providing a path of "directory1/new_file.txt"
    /// </example>
    pub path: String,

    /// The text contents of the file to create.
    ///
    /// <example>
    /// To create a file with the text "Hello, World!", provide contents of "Hello, World!"
    /// </example>
    pub contents: String,
}

pub struct CreateFileTool;

impl Tool for CreateFileTool {
    fn name(&self) -> String {
        "create-file".into()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./create_file_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::FileCreate
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(CreateFileToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<CreateFileToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path);
                format!("Create file {path}")
            }
            Err(_) => "Create file".to_string(),
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
        let input = match serde_json::from_value::<CreateFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };
        let project_path = match project.read(cx).find_project_path(&input.path, cx) {
            Some(project_path) => project_path,
            None => return Task::ready(Err(anyhow!("Path to create was outside the project"))),
        };
        let contents: Arc<str> = input.contents.as_str().into();
        let destination_path: Arc<str> = input.path.as_str().into();

        cx.spawn(async move |cx| {
            project
                .update(cx, |project, cx| {
                    project.create_entry(project_path.clone(), false, cx)
                })?
                .await
                .map_err(|err| anyhow!("Unable to create {destination_path}: {err}"))?;
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await
                .map_err(|err| anyhow!("Unable to open buffer for {destination_path}: {err}"))?;
            buffer.update(cx, |buffer, cx| {
                buffer.set_text(contents, cx);
            })?;

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                .await
                .map_err(|err| anyhow!("Unable to save buffer for {destination_path}: {err}"))?;

            Ok(format!("Created file {destination_path}"))
        })
    }
}
