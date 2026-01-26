use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream};

/// A tool for thinking through problems, brainstorming ideas, or planning without executing any actions.
/// Use this tool when you need to work through complex problems, develop strategies, or outline approaches before taking action.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ThinkingToolInput {
    /// Content to think about. This should be a description of what to think about or a problem to solve.
    content: String,
}

pub struct ThinkingTool;

impl AgentTool for ThinkingTool {
    type Input = ThinkingToolInput;
    type Output = String;

    fn name() -> &'static str {
        "thinking"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Think
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Thinking".into()
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        event_stream
            .update_fields(acp::ToolCallUpdateFields::new().content(vec![input.content.into()]));
        Task::ready(Ok("Finished thinking.".to_string()))
    }
}
