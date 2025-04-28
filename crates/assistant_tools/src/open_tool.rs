use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::markdown::MarkdownEscaped;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OpenToolInput {
    /// The path or URL to open with the default application.
    path_or_url: String,
}

pub struct OpenTool;

impl Tool for OpenTool {
    fn name(&self) -> String {
        "open".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./open_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::ArrowUpRight
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<OpenToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<OpenToolInput>(input.clone()) {
            Ok(input) => format!("Open `{}`", MarkdownEscaped(&input.path_or_url)),
            Err(_) => "Open file or URL".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: OpenToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        cx.background_spawn(async move {
            open::that(&input.path_or_url).context("Failed to open URL or file path")?;

            Ok(format!("Successfully opened {}", input.path_or_url))
        })
        .into()
    }
}
