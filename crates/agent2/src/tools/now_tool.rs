use std::sync::Arc;

use agent_client_protocol as acp;
use anyhow::Result;
use chrono::{Local, Utc};
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AgentTool, ToolCallEventStream};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Timezone {
    /// Use UTC for the datetime.
    Utc,
    /// Use local time for the datetime.
    Local,
}

/// Returns the current datetime in RFC 3339 format.
/// Only use this tool when the user specifically asks for it or the current task would benefit from knowing the current datetime.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NowToolInput {
    /// The timezone to use for the datetime.
    timezone: Timezone,
}

pub struct NowTool;

impl AgentTool for NowTool {
    type Input = NowToolInput;
    type Output = String;

    fn name() -> &'static str {
        "now"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>) -> SharedString {
        "Get current time".into()
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        let now = match input.timezone {
            Timezone::Utc => Utc::now().to_rfc3339(),
            Timezone::Local => Local::now().to_rfc3339(),
        };
        Task::ready(Ok(format!("The current datetime is {now}.")))
    }
}
