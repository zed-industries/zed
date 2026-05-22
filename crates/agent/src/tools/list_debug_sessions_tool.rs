use super::get_debug_session_state_tool::breakpoints_summary;
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Write, sync::Arc};

/// List all active debug sessions in the project.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListDebugSessionsInput {}

pub struct ListDebugSessionsTool {
    project: Entity<Project>,
}

impl ListDebugSessionsTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for ListDebugSessionsTool {
    type Input = ListDebugSessionsInput;
    type Output = String;

    const NAME: &'static str = "list_debug_sessions";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(&self, _input: Result<Self::Input, Value>, _cx: &mut App) -> SharedString {
        "List debug sessions".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            input
                .recv()
                .await
                .map_err(|error| format!("Failed to receive tool input: {error}"))?;

            let mut output = breakpoints_summary(&project, cx);

            project.read_with(cx, |project, cx| {
                let dap_store = project.dap_store().read(cx);
                let mut sessions = dap_store.sessions().peekable();

                if sessions.peek().is_none() {
                    if output.is_empty() {
                        output.push_str("No active debug sessions found.");
                    } else {
                        output.push_str("No active debug sessions found.\n");
                    }
                    return Ok(output);
                }

                output.push_str("Active debug sessions:\n");
                for session_entity in sessions {
                    let session = session_entity.read(cx);
                    let label = session
                        .label()
                        .map(|label| format!(" ({label})"))
                        .unwrap_or_default();
                    let status = if session.is_terminated() {
                        "terminated"
                    } else if session.any_stopped_thread() {
                        "stopped"
                    } else {
                        "running"
                    };
                    writeln!(
                        output,
                        "- ID: {}, Adapter: {}{}, Status: {}",
                        session.session_id().to_proto(),
                        session.adapter().0,
                        label,
                        status
                    )
                    .ok();
                }

                Ok(output)
            })
        })
    }
}
