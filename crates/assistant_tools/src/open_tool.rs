use anyhow::{anyhow, Context as _, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, AppContext, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::markdown::MarkdownString;

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

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./open_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::ExternalLink
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(OpenToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<OpenToolInput>(input.clone()) {
            Ok(input) => format!("Open `{}`", MarkdownString::escape(&input.path_or_url)),
            Err(_) => "Open file or URL".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input: OpenToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        cx.background_spawn(async move {
            open::that(&input.path_or_url).context("Failed to open URL or file path")?;

            Ok(format!("Successfully opened {}", input.path_or_url))
        })
    }
}
