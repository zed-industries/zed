use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, AppContext, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeletePathToolInput {
    /// The path of the file or directory to delete.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can delete the first file by providing a path of "directory1/a/something.txt"
    /// </example>
    pub path: String,
}

pub struct DeletePathTool;

impl Tool for DeletePathTool {
    fn name(&self) -> String {
        "delete-path".into()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./delete_path_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Trash
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(DeletePathToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<DeletePathToolInput>(input.clone()) {
            Ok(input) => format!("Delete “`{}`”", input.path),
            Err(_) => "Delete path".to_string(),
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
        let path_str = match serde_json::from_value::<DeletePathToolInput>(input) {
            Ok(input) => input.path,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        match project
            .read(cx)
            .find_project_path(&path_str, cx)
            .and_then(|path| project.update(cx, |project, cx| project.delete_file(path, false, cx)))
        {
            Some(deletion_task) => cx.background_spawn(async move {
                match deletion_task.await {
                    Ok(()) => Ok(format!("Deleted {path_str}")),
                    Err(err) => Err(anyhow!("Failed to delete {path_str}: {err}")),
                }
            }),
            None => Task::ready(Err(anyhow!(
                "Couldn't delete {path_str} because that path isn't in this project."
            ))),
        }
    }
}
