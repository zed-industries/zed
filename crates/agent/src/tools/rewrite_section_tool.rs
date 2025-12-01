use std::sync::Arc;

use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AgentTool, ToolCallEventStream};

/// Replaces text in <rewrite_this></rewrite_this> tags with your replacement_text.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RewriteSectionInput {
    /// A brief description of the edit you have made.
    ///
    /// This is optional - if the edit is simple or obvious, you should leave it empty.
    pub description: String,

    /// The text to replace the section with.
    pub replacement_text: String,
}

pub struct RewriteSectionTool;

impl AgentTool for RewriteSectionTool {
    type Input = RewriteSectionInput;
    type Output = String;

    fn name() -> &'static str {
        "rewrite_section"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "".into()
    }

    fn run(
        self: Arc<Self>,
        _input: Self::Input,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        todo!()
        // let now = match input.timezone {
        //     Timezone::Utc => Utc::now().to_rfc3339(),
        //     Timezone::Local => Local::now().to_rfc3339(),
        // };
        // Task::ready(Ok(format!("The current datetime is {now}.")))
    }
}
