use agent_client_protocol::v2::schema as acp_v2;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, ThreadEnvironment, ToolCallEventStream, ToolInput};

/// Stops a command that you started with the terminal tool and `run_in_background`, while it is still running.
///
/// The command is killed and this call returns the output it had produced. No result message follows.
///
/// Only commands that you started in the background, and that have not finished, can be stopped.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StopCommandToolInput {
    /// The terminal_id that the terminal tool returned when it started the command in the background.
    pub terminal_id: String,
}

pub struct StopCommandTool {
    environment: Rc<dyn ThreadEnvironment>,
}

impl StopCommandTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
    }
}

impl AgentTool for StopCommandTool {
    type Input = StopCommandToolInput;
    type Output = String;

    const NAME: &'static str = "stop_command";

    fn kind() -> acp_v2::ToolKind {
        acp_v2::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Stop background command".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|error| error.to_string())?;
            let terminal_id = acp_v2::TerminalId::new(input.terminal_id);
            cx.update(|cx| self.environment.stop_background_command(terminal_id, cx))
                .await
                .map_err(|error| format!("{error:#}"))
        })
    }
}
