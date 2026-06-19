use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Call this tool when you have finished all your work. You must use this tool
/// to complete your turn. Do not output raw text after using tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AttemptCompletionInput {
    /// A brief summary of what was accomplished in this turn.
    pub summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptCompletionOutput {
    pub summary: Option<String>,
}

impl From<AttemptCompletionOutput> for LanguageModelToolResultContent {
    fn from(value: AttemptCompletionOutput) -> Self {
        match value.summary {
            Some(summary) => summary.into(),
            None => "Task completed.".into(),
        }
    }
}

pub struct AttemptCompletionTool;

impl AgentTool for AttemptCompletionTool {
    type Input = AttemptCompletionInput;
    type Output = AttemptCompletionOutput;

    const NAME: &'static str = "attempt_completion";

    fn description() -> SharedString {
        "Call this tool when you have finished all your work. You must use this tool to complete your turn. Do not output raw text after using tools.".into()
    }

    fn supports_input_streaming() -> bool {
        false
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Completing task".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |_cx| {
            let input = input.recv().await.map_err(|e| AttemptCompletionOutput {
                summary: Some(e.to_string()),
            })?;

            Ok(AttemptCompletionOutput {
                summary: input.summary,
            })
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        _output: Self::Output,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
