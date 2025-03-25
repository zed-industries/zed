use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
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

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./thinking_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Brain
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(ThinkingToolInput);
        serde_json::to_value(&schema).unwrap()
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
        _cx: &mut App,
    ) -> Task<Result<String>> {
        // This tool just "thinks out loud" and doesn't perform any actions.
        Task::ready(match serde_json::from_value::<ThinkingToolInput>(input) {
            Ok(_input) => Ok("Finished thinking.".to_string()),
            Err(err) => Err(anyhow!(err)),
        })
    }
}
