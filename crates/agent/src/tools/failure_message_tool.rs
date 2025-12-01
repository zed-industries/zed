use std::sync::Arc;

use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AgentTool, ToolCallEventStream};

/// Use this tool to provide a message to the user when you're unable to complete a task.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FailureMessageInput {
    /// A brief message to the user explaining why you're unable to fulfill the request.
    pub message: String,
}

pub struct FailureMessageTool;

impl AgentTool for FailureMessageTool {
    type Input = FailureMessageInput;
    type Output = String;

    fn name() -> &'static str {
        "failure_message"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Think
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
    }
}
