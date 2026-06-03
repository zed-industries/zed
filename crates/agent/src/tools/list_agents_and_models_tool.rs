use agent_client_protocol::schema as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, AvailableAgents, ThreadEnvironment, ToolCallEventStream, ToolInput};

/// List the agents and models available for use with the `create_thread` tool.
///
/// Call this before `create_thread` if you need to pick a specific agent or a
/// non-default model (for example, to use a cheaper model for bulk work). If
/// you're happy with the user's current defaults, you don't need to call this.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListAgentsAndModelsToolInput {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAgentsAndModelsToolOutput {
    Success(AvailableAgents),
    Error { error: String },
}

impl From<ListAgentsAndModelsToolOutput> for LanguageModelToolResultContent {
    fn from(output: ListAgentsAndModelsToolOutput) -> Self {
        serde_json::to_string(&output)
            .unwrap_or_else(|e| format!("Failed to serialize list_agents_and_models output: {e}"))
            .into()
    }
}

pub struct ListAgentsAndModelsTool {
    environment: Rc<dyn ThreadEnvironment>,
}

impl ListAgentsAndModelsTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
    }
}

impl AgentTool for ListAgentsAndModelsTool {
    type Input = ListAgentsAndModelsToolInput;
    type Output = ListAgentsAndModelsToolOutput;

    const NAME: &'static str = "list_agents_and_models";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "List agents and models".into()
    }

    fn run(
        self: Arc<Self>,
        _input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let result = self.environment.list_available_agents(cx);
        Task::ready(match result {
            Ok(agents) => Ok(ListAgentsAndModelsToolOutput::Success(agents)),
            Err(error) => Err(ListAgentsAndModelsToolOutput::Error {
                error: error.to_string(),
            }),
        })
    }
}
