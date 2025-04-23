use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::AnyWindowHandle;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use language_model::LanguageModelToolSchemaFormat;
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct PartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    contents: String,
}

pub struct CreateFileTool;

const DEFAULT_UI_TEXT: &str = "Create file";

impl Tool for CreateFileTool {
    fn name(&self) -> String {
        "create_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./create_file_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::FileCreate
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<CreateFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<CreateFileToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path);
                format!("Create file {path}")
            }
            Err(_) => DEFAULT_UI_TEXT.to_string(),
        }
    }

    fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<PartialInput>(input.clone()).ok() {
            Some(input) if !input.path.is_empty() => input.path,
            _ => DEFAULT_UI_TEXT.to_string(),
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
        let input = match serde_json::from_value::<CreateFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };
        let project_path = match project.read(cx).find_project_path(&input.path, cx) {
            Some(project_path) => project_path,
            None => {
                return Task::ready(Err(anyhow!("Path to create was outside the project"))).into();
            }
        };
        let contents: Arc<str> = input.contents.as_str().into();
        let destination_path: Arc<str> = input.path.as_str().into();

        cx.spawn(async move |cx| {
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await
                .map_err(|err| anyhow!("Unable to open buffer for {destination_path}: {err}"))?;
            cx.update(|cx| {
                action_log.update(cx, |action_log, cx| {
                    action_log.track_buffer(buffer.clone(), cx)
                });
                buffer.update(cx, |buffer, cx| buffer.set_text(contents, cx));
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_edited(buffer.clone(), cx)
                });
            })?;

            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                .await
                .map_err(|err| anyhow!("Unable to save buffer for {destination_path}: {err}"))?;

            Ok(format!("Created file {destination_path}"))
        })
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn still_streaming_ui_text_with_path() {
        let tool = CreateFileTool;
        let input = json!({
            "path": "src/main.rs",
            "contents": "fn main() {\n    println!(\"Hello, world!\");\n}"
        });

        assert_eq!(tool.still_streaming_ui_text(&input), "src/main.rs");
    }

    #[test]
    fn still_streaming_ui_text_without_path() {
        let tool = CreateFileTool;
        let input = json!({
            "path": "",
            "contents": "fn main() {\n    println!(\"Hello, world!\");\n}"
        });

        assert_eq!(tool.still_streaming_ui_text(&input), DEFAULT_UI_TEXT);
    }

    #[test]
    fn still_streaming_ui_text_with_null() {
        let tool = CreateFileTool;
        let input = serde_json::Value::Null;

        assert_eq!(tool.still_streaming_ui_text(&input), DEFAULT_UI_TEXT);
    }

    #[test]
    fn ui_text_with_valid_input() {
        let tool = CreateFileTool;
        let input = json!({
            "path": "src/main.rs",
            "contents": "fn main() {\n    println!(\"Hello, world!\");\n}"
        });

        assert_eq!(tool.ui_text(&input), "Create file `src/main.rs`");
    }

    #[test]
    fn ui_text_with_invalid_input() {
        let tool = CreateFileTool;
        let input = json!({
            "invalid": "field"
        });

        assert_eq!(tool.ui_text(&input), DEFAULT_UI_TEXT);
    }
}
