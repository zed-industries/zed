use std::sync::Arc;

use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use web_search::WebSearchRegistry;

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
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<WebSearchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };
        let provider = WebSearchRegistry::read_global(cx)
            .providers()
            .next()
            .unwrap()
            .clone();

        let search_task = provider.search(input.query, cx);
        cx.background_spawn(async move {
            let results = search_task.await?;
            dbg!(&results);
            Ok("Got results".to_string())
        })
    }
}
