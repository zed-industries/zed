use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use chrono::{Local, Utc};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Timezone {
    /// Use UTC for the datetime.
    Utc,
    /// Use local time for the datetime.
    Local,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NowToolInput {
    /// The timezone to use for the datetime.
    timezone: Timezone,
}

pub struct NowTool;

impl Tool for NowTool {
    fn name(&self) -> String {
        "now".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        "Returns the current datetime in RFC 3339 format. Only use this tool when the user specifically asks for it or the current task would benefit from knowing the current datetime.".into()
    }

    fn icon(&self) -> IconName {
        IconName::Info
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(NowToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Get current time".to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        let input: NowToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let now = match input.timezone {
            Timezone::Utc => Utc::now().to_rfc3339(),
            Timezone::Local => Local::now().to_rfc3339(),
        };
        let text = format!("The current datetime is {now}.");

        Task::ready(Ok(text))
    }
}
