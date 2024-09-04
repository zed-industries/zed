use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use chrono::{Local, Utc};
use gpui::{Task, WeakView, WindowContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Timezone {
    /// Use UTC for the datetime.
    Utc,
    /// Use local time for the datetime.
    Local,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileToolInput {
    /// The timezone to use for the datetime.
    timezone: Timezone,
}

pub struct NowTool;

impl Tool for NowTool {
    fn name(&self) -> String {
        "now".into()
    }

    fn description(&self) -> String {
        "Returns the current datetime in RFC 3339 format.".into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(FileToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _workspace: WeakView<workspace::Workspace>,
        _cx: &mut WindowContext,
    ) -> Task<Result<String>> {
        let input: FileToolInput = match serde_json::from_value(input) {
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
