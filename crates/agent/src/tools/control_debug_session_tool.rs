use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use dap::{SteppingGranularity, client::SessionId};
use gpui::{App, Entity, SharedString, Task};
use project::{Project, debugger::session::ThreadId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DebugControlAction {
    /// Resumes execution of a stopped thread.
    Continue { thread_id: i64 },
    /// Steps over the current statement.
    StepOver { thread_id: i64 },
    /// Steps into the current statement.
    StepIn { thread_id: i64 },
    /// Steps out of the current function.
    StepOut { thread_id: i64 },
    /// Pauses execution of a running thread.
    Pause { thread_id: i64 },
    /// Restarts the debug session.
    Restart { args: Option<Value> },
    /// Shuts down (terminates or disconnects) the debug session.
    Shutdown,
    /// Evaluates an expression in the context of a stack frame.
    Evaluate {
        expression: String,
        stack_frame_id: Option<u64>,
    },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ControlDebugSessionInput {
    /// The ID of the debug session to control (obtained from list_debug_sessions).
    pub session_id: u32,
    /// The control action to perform.
    pub action: DebugControlAction,
}

pub struct ControlDebugSessionTool {
    project: Entity<Project>,
}

impl ControlDebugSessionTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for ControlDebugSessionTool {
    type Input = ControlDebugSessionInput;
    type Output = String;

    const NAME: &'static str = "control_debug_session";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(&self, input: Result<Self::Input, Value>, _cx: &mut App) -> SharedString {
        if let Ok(input) = input {
            let action_name = match input.action {
                DebugControlAction::Continue { .. } => "Continue",
                DebugControlAction::StepOver { .. } => "Step Over",
                DebugControlAction::StepIn { .. } => "Step In",
                DebugControlAction::StepOut { .. } => "Step Out",
                DebugControlAction::Pause { .. } => "Pause",
                DebugControlAction::Restart { .. } => "Restart",
                DebugControlAction::Shutdown => "Shutdown",
                DebugControlAction::Evaluate { .. } => "Evaluate",
            };
            format!("Control debug session: {action_name}").into()
        } else {
            "Control debug session".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|error| format!("Failed to receive tool input: {error}"))?;

            let session_entity = project
                .read_with(cx, |project, cx| {
                    project
                        .dap_store()
                        .read(cx)
                        .session_by_id(SessionId(input.session_id))
                })
                .ok_or_else(|| format!("Debug session with ID {} not found", input.session_id))?;

            match input.action {
                DebugControlAction::Continue { thread_id } => {
                    session_entity.update(cx, |session, cx| {
                        session.continue_thread(ThreadId(thread_id), cx);
                    });
                    Ok(format!("Continue command sent to thread {thread_id}"))
                }
                DebugControlAction::StepOver { thread_id } => {
                    session_entity.update(cx, |session, cx| {
                        session.step_over(ThreadId(thread_id), SteppingGranularity::Line, cx);
                    });
                    Ok(format!("Step Over command sent to thread {thread_id}"))
                }
                DebugControlAction::StepIn { thread_id } => {
                    session_entity.update(cx, |session, cx| {
                        session.step_in(ThreadId(thread_id), SteppingGranularity::Line, cx);
                    });
                    Ok(format!("Step In command sent to thread {thread_id}"))
                }
                DebugControlAction::StepOut { thread_id } => {
                    session_entity.update(cx, |session, cx| {
                        session.step_out(ThreadId(thread_id), SteppingGranularity::Line, cx);
                    });
                    Ok(format!("Step Out command sent to thread {thread_id}"))
                }
                DebugControlAction::Pause { thread_id } => {
                    session_entity.update(cx, |session, cx| {
                        session.pause_thread(ThreadId(thread_id), cx);
                    });
                    Ok(format!("Pause command sent to thread {thread_id}"))
                }
                DebugControlAction::Restart { args } => {
                    session_entity.update(cx, |session, cx| {
                        session.restart(args, cx);
                    });
                    Ok("Restart command sent to session".to_string())
                }
                DebugControlAction::Shutdown => {
                    let shutdown_task =
                        session_entity.update(cx, |session, cx| session.shutdown(cx));
                    shutdown_task.await;
                    Ok("Shutdown command sent to session".to_string())
                }
                DebugControlAction::Evaluate {
                    expression,
                    stack_frame_id,
                } => {
                    let eval_task = session_entity.read_with(cx, |session, _cx| {
                        session.dap_evaluate_expression(expression.clone(), stack_frame_id)
                    });
                    let response = eval_task.await.map_err(|error| {
                        format!("Failed to evaluate expression '{expression}': {error}")
                    })?;
                    let type_suffix = response
                        .type_
                        .map(|type_name| format!(" ({type_name})"))
                        .unwrap_or_default();
                    Ok(format!(
                        "Expression: {expression}\nResult: {}{}\nVariables reference: {}",
                        response.result, type_suffix, response.variables_reference
                    ))
                }
            }
        })
    }
}
