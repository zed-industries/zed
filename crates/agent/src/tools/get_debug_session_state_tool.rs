use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use dap::{Variable, client::SessionId};
use gpui::{App, AsyncApp, Entity, SharedString, Task};
use project::{
    Project,
    debugger::session::{ThreadId, ThreadStatus},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Write, sync::Arc};

const DEFAULT_STACK_FRAME_LIMIT: u64 = 20;

/// Gets the current state of a debug session including threads, stack trace, scopes, and variables.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetDebugSessionStateInput {
    /// The ID of the debug session to query (obtained from list_debug_sessions).
    pub session_id: u32,
    /// Optional thread ID to query stack trace for.
    /// If not specified, and there are stopped threads, the tool will retrieve the backtrace for the first stopped thread it finds.
    pub thread_id: Option<i64>,
    /// Optional stack frame ID to query variables for.
    /// If not specified, and a stopped thread is selected/queried, the tool will retrieve scopes and variables for the top stack frame.
    pub stack_frame_id: Option<u64>,
    /// Optional variables reference ID to query nested variables (e.g. fields of an object, elements of an array).
    pub variables_reference: Option<u64>,
}

pub struct GetDebugSessionStateTool {
    project: Entity<Project>,
}

impl GetDebugSessionStateTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for GetDebugSessionStateTool {
    type Input = GetDebugSessionStateInput;
    type Output = String;

    const NAME: &'static str = "get_debug_session_state";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(&self, _input: Result<Self::Input, Value>, _cx: &mut App) -> SharedString {
        "Get debug session state".into()
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

            if let Some(variables_reference) = input.variables_reference {
                let variables_task = session_entity.read_with(cx, |session, _cx| {
                    session.dap_variables(variables_reference)
                });
                let variables = variables_task.await.map_err(|error| {
                    format!("Failed to get variables for reference {variables_reference}: {error}")
                })?;

                let mut output = format!("Variables for reference {variables_reference}:\n");
                for variable in variables {
                    let type_suffix = variable
                        .type_
                        .as_ref()
                        .map(|type_name| format!(" ({type_name})"))
                        .unwrap_or_default();
                    writeln!(
                        output,
                        "- {} = {}{}{}",
                        variable.name,
                        variable.value,
                        type_suffix,
                        variable_reference_suffix(&variable)
                    )
                    .ok();
                }
                return Ok(output);
            }

            if let Some(stack_frame_id) = input.stack_frame_id {
                let scopes_task =
                    session_entity.read_with(cx, |session, _cx| session.dap_scopes(stack_frame_id));
                let scopes = scopes_task.await.map_err(|error| {
                    format!("Failed to get scopes for stack frame {stack_frame_id}: {error}")
                })?;

                let mut output = format!("Scopes for stack frame {stack_frame_id}:\n");
                for scope in &scopes {
                    writeln!(
                        output,
                        "- {} (variables reference: {})",
                        scope.name, scope.variables_reference
                    )
                    .ok();
                }

                if let Some(first_scope) = scopes.first() {
                    let variables_reference = first_scope.variables_reference;
                    let variables_task = session_entity
                        .read_with(cx, |session, _cx| session.dap_variables(variables_reference));
                    let variables = variables_task.await.map_err(|error| {
                        format!("Failed to get variables for reference {variables_reference}: {error}")
                    })?;
                    writeln!(output, "\nVariables in scope '{}':", first_scope.name).ok();
                    for variable in variables {
                        let type_suffix = variable
                            .type_
                            .as_ref()
                            .map(|type_name| format!(" ({type_name})"))
                            .unwrap_or_default();
                        writeln!(
                            output,
                            "  - {} = {}{}{}",
                            variable.name,
                            variable.value,
                            type_suffix,
                            variable_reference_suffix(&variable)
                        )
                        .ok();
                    }
                }

                return Ok(output);
            }

            let mut output = String::new();
            output.push_str(&breakpoints_summary(&project, cx));
            output.push_str("Threads:\n");

            let threads_task = session_entity.read_with(cx, |session, _cx| session.dap_threads());
            let threads = match threads_task.await {
                Ok(threads) => threads,
                Err(error) => {
                    writeln!(output, "Failed to get debug session threads: {error}").ok();
                    return Ok(output);
                }
            };

            let mut stopped_thread_id = None;
            for thread in &threads {
                let status = session_entity.read_with(cx, |session, _cx| {
                    session.thread_status(ThreadId(thread.id))
                });
                let status_label = status.label().to_lowercase();
                writeln!(
                    output,
                    "- ID: {}, Name: {}, Status: {}",
                    thread.id, thread.name, status_label
                )
                .ok();

                if status == ThreadStatus::Stopped && stopped_thread_id.is_none()
                {
                    stopped_thread_id = Some(thread.id);
                }
            }

            let query_thread_id = input.thread_id.or(stopped_thread_id);
            if let Some(thread_id) = query_thread_id {
                writeln!(output, "\nStack Trace for thread {thread_id}:").ok();
                let stack_frames_task = session_entity.read_with(cx, |session, _cx| {
                    session.dap_stack_trace(ThreadId(thread_id), Some(DEFAULT_STACK_FRAME_LIMIT))
                });
                let frames = stack_frames_task.await.map_err(|error| {
                    format!("Failed to get stack trace for thread {thread_id}: {error}")
                })?;

                for frame in &frames {
                    let source_info = frame
                        .source
                        .as_ref()
                        .and_then(|source| source.path.as_ref())
                        .map(|path| format!(" at {path}:{}", frame.line))
                        .unwrap_or_default();
                    writeln!(
                        output,
                        "- Frame {}: {}{}",
                        frame.id, frame.name, source_info
                    )
                    .ok();
                }

                if let Some(top_frame) = frames.first() {
                    let stack_frame_id = top_frame.id;
                    let scopes_task = session_entity
                        .read_with(cx, |session, _cx| session.dap_scopes(stack_frame_id));
                    let scopes = scopes_task.await.map_err(|error| {
                        format!("Failed to get scopes for stack frame {stack_frame_id}: {error}")
                    })?;
                    writeln!(output, "\nScopes for top frame {}:", top_frame.id).ok();
                    for scope in &scopes {
                        writeln!(
                            output,
                            "  - {} (variables reference: {})",
                            scope.name, scope.variables_reference
                        )
                        .ok();
                    }

                    if let Some(first_scope) = scopes.first() {
                        let variables_reference = first_scope.variables_reference;
                        let variables_task = session_entity.read_with(cx, |session, _cx| {
                            session.dap_variables(variables_reference)
                        });
                        let variables = variables_task.await.map_err(|error| {
                            format!(
                                "Failed to get variables for reference {variables_reference}: {error}"
                            )
                        })?;
                        writeln!(output, "\nVariables in local scope '{}':", first_scope.name).ok();
                        for variable in variables {
                            let type_suffix = variable
                                .type_
                                .as_ref()
                                .map(|type_name| format!(" ({type_name})"))
                                .unwrap_or_default();
                            writeln!(
                                output,
                                "    - {} = {}{}{}",
                                variable.name,
                                variable.value,
                                type_suffix,
                                variable_reference_suffix(&variable)
                            )
                            .ok();
                        }
                    }
                }
            }

            Ok(output)
        })
    }
}

fn variable_reference_suffix(variable: &Variable) -> String {
    if variable.variables_reference == 0 {
        String::new()
    } else {
        format!(" (variables reference: {})", variable.variables_reference)
    }
}

pub(super) fn breakpoints_summary(project: &Entity<Project>, cx: &mut AsyncApp) -> String {
    project.read_with(cx, |project, cx| {
        let breakpoint_store = project.breakpoint_store().read(cx);
        let all_breakpoints = breakpoint_store.all_source_breakpoints(cx);

        if all_breakpoints.is_empty() {
            return String::new();
        }

        let mut output = String::from("Breakpoints:\n");
        for (path, breakpoints) in all_breakpoints {
            for breakpoint in breakpoints {
                let state = if breakpoint.state.is_disabled() {
                    "disabled"
                } else {
                    "enabled"
                };
                write!(
                    output,
                    "- {}:{} ({state})",
                    path.display(),
                    breakpoint.row + 1
                )
                .ok();
                if let Some(condition) = breakpoint.condition.as_ref() {
                    write!(output, ", condition: {condition}").ok();
                }
                if let Some(hit_condition) = breakpoint.hit_condition.as_ref() {
                    write!(output, ", hit condition: {hit_condition}").ok();
                }
                if let Some(message) = breakpoint.message.as_ref() {
                    write!(output, ", log message: {message}").ok();
                }
                writeln!(output).ok();
            }
        }
        output.push('\n');
        output
    })
}
