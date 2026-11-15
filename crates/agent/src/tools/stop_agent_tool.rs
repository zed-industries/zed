use agent_client_protocol::v2::schema as acp_v2;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, ThreadEnvironment, ToolCallEventStream, ToolInput};

/// Stops an agent that you started with `spawn_agent` and `run_in_background`, while it is still working.
///
/// - By default the agent stops at once and this call returns what it had produced so far. No result message follows.
/// - Set `after_current_step` to true to let the agent finish the step it is on first, the way a steering message does. This call then returns right away, and the agent's last message is delivered to you later as a new message, like any background agent's result.
/// - A stopped agent keeps its context. Call `spawn_agent` with its session_id to give it new instructions.
/// - Only agents that you started in the background, and that have not finished, can be stopped.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StopAgentToolInput {
    /// The session_id that `spawn_agent` returned for the agent.
    pub session_id: String,
    /// Set to true to let the agent finish the step it is on before it stops. Defaults to false, which stops it at once.
    #[serde(default, skip_serializing_if = "is_false")]
    pub after_current_step: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
}

pub struct StopAgentTool {
    environment: Rc<dyn ThreadEnvironment>,
}

impl StopAgentTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
    }
}

impl AgentTool for StopAgentTool {
    type Input = StopAgentToolInput;
    type Output = String;

    const NAME: &'static str = "stop_agent";

    fn kind() -> acp_v2::ToolKind {
        acp_v2::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Stop agent".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|error| error.to_string())?;
            let session_id = acp_v2::SessionId::new(input.session_id);
            let immediately = !input.after_current_step;
            cx.update(|cx| {
                self.environment
                    .stop_background_subagent(session_id, immediately, cx)
            })
            .await
            .map_err(|error| format!("{error:#}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stops_at_once_unless_asked_to_wait_for_the_current_step() -> Result<(), serde_json::Error>
    {
        let at_once: StopAgentToolInput = serde_json::from_value(json!({
            "session_id": "session",
        }))?;
        assert!(!at_once.after_current_step);

        let after_step: StopAgentToolInput = serde_json::from_value(json!({
            "session_id": "session",
            "after_current_step": true,
        }))?;
        assert!(after_step.after_current_step);
        Ok(())
    }
}
