use std::sync::Arc;

use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ThinkingToolInput {
    /// Content to think about. This should be a description of what to think about or
    /// a problem to solve.
    content: String,
}

pub struct ThinkingTool;

impl Tool for ThinkingTool {
    fn name(&self) -> String {
        "thinking".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./thinking_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::LightBulb
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ThinkingToolInput>(format)
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Thinking".to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _window: Option<AnyWindowHandle>,
        _cx: &mut App,
    ) -> ToolResult {
        // This tool just "thinks out loud" and doesn't perform any actions.
        Task::ready(match serde_json::from_value::<ThinkingToolInput>(input) {
            Ok(_input) => Ok("Finished thinking.".to_string()),
            Err(err) => Err(anyhow!(err)),
        })
        .into()
    }
}
