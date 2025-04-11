use std::sync::Arc;

use crate::schema::json_schema_for;
use anyhow::Result;
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchToolInput {
    /// The query to search for.
    query: String,
}

pub struct WebSearchTool;

impl Tool for WebSearchTool {
    fn name(&self) -> String {
        "web_search".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        true
    }

    fn description(&self) -> String {
        "Do a web search.".into()
    }

    fn icon(&self) -> IconName {
        IconName::Globe
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<WebSearchToolInput>(format)
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Web Search".to_string()
    }

    fn run(
        self: Arc<Self>,
        _input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        let text = "Did some googling".to_string();
        Task::ready(Ok(text))
    }
}
