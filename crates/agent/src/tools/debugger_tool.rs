use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context as _, Result, anyhow};
use dap::{DapRegistry, client::SessionId};
use gpui::{App, Entity, SharedString, Task, WeakEntity};
use language_model::LanguageModelToolResultContent;
use project::{Project, WorktreeId, debugger::agent_api::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{path::PathBuf, rc::Rc, sync::Arc, time::Duration};
use task::{DebugScenario, SharedTaskContext};
use util::{markdown::MarkdownInlineCode, paths::normalize_lexically};

use crate::{
    AgentTool, DebugSessionRequest, Thread, ThreadEnvironment, ToolCallEventStream, ToolInput,
    ToolPermissionContext,
    sandboxing::{SandboxRequest, sandboxing_enabled},
};

const DEFAULT_CONTROL_TIMEOUT_MS: u64 = 30_000;
const MAX_CONTROL_TIMEOUT_MS: u64 = 300_000;
const MAX_SNAPSHOT_FRAMES: usize = 200;
const MAX_SNAPSHOT_VARIABLES_PER_SCOPE: usize = 500;
const MAX_SNAPSHOT_VARIABLE_VALUE_LENGTH: usize = 16 * 1024;
const MAX_SNAPSHOT_OUTPUT_EVENTS: usize = 1_000;
const MAX_SNAPSHOT_OUTPUT_BYTES: usize = 1024 * 1024;
const MAX_SNAPSHOT_SOURCE_CONTEXT_LINES: usize = 200;

/// Interact with Zed's debugger. Read-only operations such as `snapshot`,
/// `list_sessions`, `list_breakpoints`, and `list_adapters` are available in
/// Ask mode. Operations that start sessions, change breakpoints, or control
/// execution require Write mode and user permission.
///
/// Prefer `snapshot` when inspecting a paused debug session: it returns a
/// bounded view of threads, stack frames, source context, variables, and recent
/// output in one call. Use `list_sessions` first when there are multiple active
/// sessions.
///
/// <guidelines>
/// - In Ask mode, only use read-only operations.
/// - Before controlling execution, inspect `list_sessions` or `snapshot` and use
///   explicit `session_id` and `thread_id` when possible.
/// - `continue`, `step`, `pause`, and `run_to_line` wait for the debugger to
///   stop, exit, or time out, then return a fresh snapshot.
/// - `start_session` runs code through Zed's debugger UI; use an existing debug
///   scenario shape with adapter, label, and adapter-specific config.
/// - Do not use this tool for expression evaluation; evaluation is intentionally
///   not available.
/// </guidelines>
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum DebuggerToolInput {
    /// List active debug sessions.
    ListSessions,
    /// Inspect debugger state for a session.
    Snapshot(SnapshotInput),
    /// List source breakpoints in the project.
    ListBreakpoints,
    /// Add or update source breakpoints. Requires Write mode and permission.
    SetBreakpoints { breakpoints: Vec<BreakpointInput> },
    /// Remove source breakpoints. Requires Write mode and permission.
    RemoveBreakpoints {
        breakpoints: Vec<BreakpointLocationInput>,
    },
    /// Continue, pause, step, or run to a line. Requires Write mode and permission.
    Control(ControlInput),
    /// List registered debug adapters and their configuration schemas.
    ListAdapters,
    /// Start a debug session through Zed's debugger UI. Requires Write mode and permission.
    StartSession(StartSessionInput),
    /// Stop a debug session. Requires Write mode and permission.
    StopSession { session_id: u64 },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SnapshotInput {
    /// DAP session id. When omitted, uses the active debug session, otherwise the first session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<u64>,
    /// Optional bounds for returned stack, variables, output, and source context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<SnapshotLimitsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SnapshotLimitsInput {
    /// Maximum total stack frames across all stopped threads. Defaults to 20; maximum 200.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_frames: Option<usize>,
    /// Maximum variables per scope. Defaults to 50; maximum 500.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_variables_per_scope: Option<usize>,
    /// Maximum bytes per variable value. Defaults to 1024; maximum 16384.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_variable_value_length: Option<usize>,
    /// Maximum recent output events. Defaults to 100; maximum 1000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_events: Option<usize>,
    /// Maximum recent output bytes. Defaults to 16384; maximum 1048576.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_bytes: Option<usize>,
    /// Maximum source context lines around each frame. Defaults to 5; maximum 200.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_source_context_lines: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BreakpointInput {
    /// Absolute source path as reported by the debugger, or a project-resolvable absolute path.
    pub path: PathBuf,
    /// 1-based line number.
    pub line: u32,
    /// Whether the breakpoint should be enabled. Defaults to true.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Optional condition expression that must be true for the breakpoint to stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// Optional hit count condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
    /// Optional logpoint message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BreakpointLocationInput {
    /// Absolute source path as reported by the debugger, or a project-resolvable absolute path.
    pub path: PathBuf,
    /// 1-based line number.
    pub line: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ControlInput {
    /// DAP session id. When omitted, uses the active debug session, otherwise the first session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<u64>,
    /// DAP thread id. When omitted, chooses a suitable thread based on the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i64>,
    /// Execution control action.
    pub action: ControlAction,
    /// Source path for `run_to_line`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    /// 1-based line for `run_to_line`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Maximum time to wait for the debugger to stop. Defaults to 30000ms; maximum 300000ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    /// Optional bounds for the snapshot returned after control completes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_limits: Option<SnapshotLimitsInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ControlAction {
    Continue,
    Pause,
    StepOver,
    StepIn,
    StepOut,
    RunToLine,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StartSessionInput {
    /// Debug scenario to start. This is the same shape as Zed debug scenarios:
    /// include `adapter`, `label`, and adapter-specific launch/attach config.
    pub scenario: DebugScenario,
    /// Optional worktree id. Omit to use the active buffer's worktree or first visible worktree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DebuggerToolOutput {
    Success {
        operation: String,
        message: String,
        data: Value,
    },
    Error {
        operation: Option<String>,
        error: String,
    },
}

impl From<DebuggerToolOutput> for LanguageModelToolResultContent {
    fn from(output: DebuggerToolOutput) -> Self {
        match &output {
            DebuggerToolOutput::Success {
                operation,
                message,
                data,
            } => {
                let data = serde_json::to_string_pretty(data).unwrap_or_else(|error| {
                    format!("<failed to serialize debugger output: {error}>")
                });
                format!("Debugger `{operation}` succeeded: {message}\n\n```json\n{data}\n```")
                    .into()
            }
            DebuggerToolOutput::Error { operation, error } => {
                let operation = operation.as_deref().unwrap_or("unknown");
                format!("Debugger `{operation}` failed: {error}").into()
            }
        }
    }
}

pub struct DebuggerTool {
    project: Entity<Project>,
    environment: Rc<dyn ThreadEnvironment>,
    thread: WeakEntity<Thread>,
}

impl DebuggerTool {
    pub fn new(
        project: Entity<Project>,
        environment: Rc<dyn ThreadEnvironment>,
        thread: WeakEntity<Thread>,
    ) -> Self {
        Self {
            project,
            environment,
            thread,
        }
    }

    fn api(&self, cx: &App) -> AgentDebuggerApi {
        let project = self.project.read(cx);
        AgentDebuggerApi::new(project.dap_store(), project.breakpoint_store())
    }

    fn is_ask_profile(&self, cx: &App) -> bool {
        self.thread
            .read_with(cx, |thread, _| thread.profile().as_str() == "ask")
            .unwrap_or(false)
    }
}

impl AgentTool for DebuggerTool {
    type Input = DebuggerToolInput;
    type Output = DebuggerToolOutput;

    const NAME: &'static str = "debugger";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => initial_title_for_input(&input),
            Err(value) => value
                .get("operation")
                .and_then(|value| value.as_str())
                .map(|operation| format!("Debugger: {operation}").into())
                .unwrap_or_else(|| "Debugger".into()),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|error| DebuggerToolOutput::Error {
                    operation: None,
                    error: format!("Failed to receive debugger tool input: {error}"),
                })?;
            let operation = operation_name(&input).to_string();
            match self
                .run_operation(input, operation.clone(), event_stream, cx)
                .await
            {
                Ok(output) => Ok(output),
                Err(error) => Err(DebuggerToolOutput::Error {
                    operation: Some(operation),
                    error: error.to_string(),
                }),
            }
        })
    }
}

impl DebuggerTool {
    async fn run_operation(
        self: Arc<Self>,
        input: DebuggerToolInput,
        operation: String,
        event_stream: ToolCallEventStream,
        cx: &mut gpui::AsyncApp,
    ) -> Result<DebuggerToolOutput> {
        match input {
            DebuggerToolInput::ListSessions => {
                let data = cx.update(|cx| sessions_to_json(self.api(cx).list_sessions(cx)));
                Ok(success(operation, "listed debug sessions", data))
            }
            DebuggerToolInput::Snapshot(input) => {
                let limits = limits_from_input(input.limits)?;
                let (api, session_id) = cx.update(|cx| {
                    let api = self.api(cx);
                    let session_id = resolve_session_id(&self.project, &api, input.session_id, cx)?;
                    anyhow::Ok((api, session_id))
                })?;
                let snapshot_task = cx.update(|cx| api.snapshot(session_id, limits, cx));
                let snapshot = snapshot_task.await?;
                Ok(success(
                    operation,
                    "captured debugger snapshot",
                    snapshot_to_json(snapshot),
                ))
            }
            DebuggerToolInput::ListBreakpoints => {
                let data = cx.update(|cx| breakpoints_to_json(self.api(cx).list_breakpoints(cx)));
                Ok(success(operation, "listed breakpoints", data))
            }
            DebuggerToolInput::ListAdapters => {
                let data = cx
                    .update(|cx| serde_json::to_value(DapRegistry::global(cx).adapters_schema()))?;
                Ok(success(operation, "listed debug adapters", data))
            }
            DebuggerToolInput::SetBreakpoints { breakpoints } => {
                self.ensure_write_mode(&operation, cx)?;
                let breakpoints = breakpoints
                    .into_iter()
                    .map(|breakpoint| resolve_breakpoint_input(&self.project, breakpoint, cx))
                    .collect::<Result<Vec<_>>>()?;
                authorize_debugger_operation(
                    &event_stream,
                    "Set debugger breakpoint(s)",
                    breakpoint_permission_inputs(&operation, breakpoints.iter())?,
                    cx,
                )
                .await?;

                let api = cx.update(|cx| self.api(cx));
                let mut results = Vec::new();
                for breakpoint in breakpoints {
                    let task = cx
                        .update(|cx| api.set_source_breakpoint(breakpoint.into_agent_input(), cx));
                    let result = task.await?;
                    results.push(breakpoint_edit_result_to_json(result));
                }
                Ok(success(
                    operation,
                    "set breakpoint(s)",
                    Value::Array(results),
                ))
            }
            DebuggerToolInput::RemoveBreakpoints { breakpoints } => {
                self.ensure_write_mode(&operation, cx)?;
                let breakpoints = breakpoints
                    .into_iter()
                    .map(|breakpoint| resolve_breakpoint_location(&self.project, breakpoint, cx))
                    .collect::<Result<Vec<_>>>()?;
                authorize_debugger_operation(
                    &event_stream,
                    "Remove debugger breakpoint(s)",
                    breakpoint_location_permission_inputs(&operation, breakpoints.iter()),
                    cx,
                )
                .await?;

                let api = cx.update(|cx| self.api(cx));
                let mut results = Vec::new();
                for breakpoint in breakpoints {
                    let task = cx.update(|cx| {
                        api.remove_source_breakpoint(breakpoint.path, breakpoint.line, cx)
                    });
                    let result = task.await?;
                    results.push(breakpoint_edit_result_to_json(result));
                }
                Ok(success(
                    operation,
                    "removed breakpoint(s)",
                    Value::Array(results),
                ))
            }
            DebuggerToolInput::Control(input) => {
                self.ensure_write_mode(&operation, cx)?;
                validate_control_timeout(input.timeout_ms)?;
                let snapshot_limits = limits_from_input(input.snapshot_limits.clone())?;
                let resolved_input = self.resolve_control_input(input, cx).await?;
                let action = resolved_input.action;
                authorize_debugger_operation(
                    &event_stream,
                    format!("Debugger {}", action.label()),
                    permission_inputs(&operation, [control_permission_input(&resolved_input)]),
                    cx,
                )
                .await?;

                let (session_id, control_result) = self.run_control(resolved_input, cx).await?;
                let api = cx.update(|cx| self.api(cx));
                let snapshot_task = cx.update(|cx| api.snapshot(session_id, snapshot_limits, cx));
                let snapshot = snapshot_task.await?;
                Ok(success(
                    operation,
                    "controlled debugger execution and captured snapshot",
                    json!({
                        "control": control_result_to_json(control_result),
                        "snapshot": snapshot_to_json(snapshot),
                    }),
                ))
            }
            DebuggerToolInput::StartSession(input) => {
                self.ensure_write_mode(&operation, cx)?;
                authorize_debugger_operation(
                    &event_stream,
                    format!(
                        "Start debug session {}",
                        MarkdownInlineCode(&input.scenario.label)
                    ),
                    start_session_permission_inputs(&operation, &input)?,
                    cx,
                )
                .await?;

                if cx.update(|cx| sandboxing_enabled(cx)) {
                    let request = SandboxRequest {
                        unsandboxed: true,
                        ..Default::default()
                    };
                    let approve = cx.update(|cx| {
                        event_stream.authorize_sandbox(
                            request,
                            "Start debug session outside the agent terminal sandbox".to_string(),
                            cx,
                        )
                    });
                    approve.await?;
                }

                let request = DebugSessionRequest {
                    scenario: input.scenario,
                    task_context: SharedTaskContext::default(),
                    active_buffer: None,
                    worktree_id: input.worktree_id.map(WorktreeId::from_proto),
                };
                let info = self.environment.start_debug_session(request, cx).await?;
                Ok(success(
                    operation,
                    "started debug session",
                    json!({
                        "session_id": info.session_id,
                        "label": info.label,
                        "adapter": info.adapter,
                    }),
                ))
            }
            DebuggerToolInput::StopSession { session_id } => {
                self.ensure_write_mode(&operation, cx)?;
                authorize_debugger_operation(
                    &event_stream,
                    format!("Stop debug session {session_id}"),
                    permission_inputs(
                        &operation,
                        [format!("stop_session session_id:{session_id}")],
                    ),
                    cx,
                )
                .await?;
                let project = self.project.clone();
                let shutdown = cx.update(|cx| {
                    let dap_store = project.read(cx).dap_store();
                    dap_store.update(cx, |dap_store, cx| {
                        dap_store.shutdown_session(SessionId::from_proto(session_id), cx)
                    })
                });
                shutdown.await?;
                Ok(success(
                    operation,
                    "stopped debug session",
                    json!({ "session_id": session_id }),
                ))
            }
        }
    }

    fn ensure_write_mode(&self, operation: &str, cx: &gpui::AsyncApp) -> Result<()> {
        if cx.update(|cx| self.is_ask_profile(cx)) {
            anyhow::bail!(
                "debugger.{operation} is not available in Ask mode. Switch to Write mode to start sessions, change breakpoints, or control execution."
            );
        }
        Ok(())
    }

    async fn resolve_control_input(
        &self,
        mut input: ControlInput,
        cx: &mut gpui::AsyncApp,
    ) -> Result<ResolvedControlInput> {
        if input.action == ControlAction::RunToLine {
            let path = input
                .path
                .take()
                .context("path is required for debugger control run_to_line")?;
            input.path = Some(resolve_debugger_path(&self.project, path, cx)?);
            input
                .line
                .context("line is required for debugger control run_to_line")?;
        }

        let (api, session_id, thread_id) = cx.update(|cx| {
            let api = self.api(cx);
            let session_id = resolve_session_id(&self.project, &api, input.session_id, cx)?;
            let thread_id = input.thread_id.map(project::debugger::session::ThreadId);
            anyhow::Ok((api, session_id, thread_id))
        })?;
        let thread_id = match thread_id {
            Some(thread_id) => thread_id,
            None => choose_thread_for_action(&api, session_id, input.action, cx).await?,
        };

        Ok(ResolvedControlInput {
            session_id,
            thread_id,
            action: input.action,
            path: input.path,
            line: input.line,
            timeout_ms: input.timeout_ms,
        })
    }

    async fn run_control(
        &self,
        input: ResolvedControlInput,
        cx: &mut gpui::AsyncApp,
    ) -> Result<(SessionId, AgentDebuggerControlResult)> {
        let timeout = Duration::from_millis(control_timeout_ms(input.timeout_ms)?);
        let session_id = input.session_id;
        let thread_id = input.thread_id;
        let api = cx.update(|cx| self.api(cx));

        match input.action {
            ControlAction::Continue => {
                let task = cx.update(|cx| api.continue_thread(session_id, thread_id, timeout, cx));
                task.await
            }
            ControlAction::Pause => {
                let task = cx.update(|cx| api.pause_thread(session_id, thread_id, timeout, cx));
                task.await
            }
            ControlAction::StepOver => {
                let task = cx.update(|cx| {
                    api.step_thread(
                        session_id,
                        thread_id,
                        AgentDebuggerStepKind::Over,
                        timeout,
                        cx,
                    )
                });
                task.await
            }
            ControlAction::StepIn => {
                let task = cx.update(|cx| {
                    api.step_thread(
                        session_id,
                        thread_id,
                        AgentDebuggerStepKind::In,
                        timeout,
                        cx,
                    )
                });
                task.await
            }
            ControlAction::StepOut => {
                let task = cx.update(|cx| {
                    api.step_thread(
                        session_id,
                        thread_id,
                        AgentDebuggerStepKind::Out,
                        timeout,
                        cx,
                    )
                });
                task.await
            }
            ControlAction::RunToLine => {
                let path = input
                    .path
                    .context("path is required for debugger control run_to_line")?;
                let line = input
                    .line
                    .context("line is required for debugger control run_to_line")?;
                let task =
                    cx.update(|cx| api.run_to_line(session_id, thread_id, path, line, timeout, cx));
                task.await
            }
        }
        .map(|result| (session_id, result))
    }
}

impl BreakpointInput {
    fn into_agent_input(self) -> AgentSourceBreakpointInput {
        AgentSourceBreakpointInput {
            path: self.path,
            line: self.line,
            enabled: self.enabled,
            condition: self.condition,
            hit_condition: self.hit_condition,
            log_message: self.log_message,
        }
    }
}

struct ResolvedControlInput {
    session_id: SessionId,
    thread_id: project::debugger::session::ThreadId,
    action: ControlAction,
    path: Option<PathBuf>,
    line: Option<u32>,
    timeout_ms: Option<u64>,
}

impl ControlAction {
    fn label(self) -> &'static str {
        match self {
            ControlAction::Continue => "continue",
            ControlAction::Pause => "pause",
            ControlAction::StepOver => "step over",
            ControlAction::StepIn => "step in",
            ControlAction::StepOut => "step out",
            ControlAction::RunToLine => "run to line",
        }
    }

    fn permission_name(self) -> &'static str {
        match self {
            ControlAction::Continue => "continue",
            ControlAction::Pause => "pause",
            ControlAction::StepOver => "step_over",
            ControlAction::StepIn => "step_in",
            ControlAction::StepOut => "step_out",
            ControlAction::RunToLine => "run_to_line",
        }
    }
}

fn default_true() -> bool {
    true
}

fn success(operation: String, message: impl Into<String>, data: Value) -> DebuggerToolOutput {
    DebuggerToolOutput::Success {
        operation,
        message: message.into(),
        data,
    }
}

async fn authorize_debugger_operation(
    event_stream: &ToolCallEventStream,
    title: impl Into<String>,
    input_values: Vec<String>,
    cx: &mut gpui::AsyncApp,
) -> Result<()> {
    let title = title.into();
    let task = cx.update(|cx| {
        event_stream.authorize(
            title,
            ToolPermissionContext::new(DebuggerTool::NAME, input_values),
            cx,
        )
    });
    task.await
}

fn permission_inputs(operation: &str, values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut inputs = values.into_iter().collect::<Vec<_>>();
    if inputs.is_empty() {
        inputs.push(operation.to_string());
    } else {
        for input in &mut inputs {
            *input = format!("{operation} {input}");
        }
    }
    inputs
}

fn breakpoint_permission_inputs<'a>(
    operation: &str,
    breakpoints: impl IntoIterator<Item = &'a BreakpointInput>,
) -> Result<Vec<String>> {
    let inputs = breakpoints
        .into_iter()
        .map(|breakpoint| {
            let condition =
                permission_value_to_string(&breakpoint.condition, "breakpoint condition")?;
            let hit_condition =
                permission_value_to_string(&breakpoint.hit_condition, "breakpoint hit condition")?;
            let log_message =
                permission_value_to_string(&breakpoint.log_message, "breakpoint log message")?;
            anyhow::Ok(format!(
                "path:{} line:{} enabled:{} condition:{} hit_condition:{} log_message:{}",
                breakpoint.path.display(),
                breakpoint.line,
                breakpoint.enabled,
                condition,
                hit_condition,
                log_message
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(permission_inputs(operation, inputs))
}

fn breakpoint_location_permission_inputs<'a>(
    operation: &str,
    breakpoints: impl IntoIterator<Item = &'a BreakpointLocationInput>,
) -> Vec<String> {
    permission_inputs(
        operation,
        breakpoints.into_iter().map(|breakpoint| {
            format!(
                "path:{} line:{}",
                breakpoint.path.display(),
                breakpoint.line
            )
        }),
    )
}

fn control_permission_input(input: &ResolvedControlInput) -> String {
    let mut value = format!(
        "action:{} session_id:{} thread_id:{}",
        input.action.permission_name(),
        input.session_id.0,
        input.thread_id.0
    );

    if input.action == ControlAction::RunToLine {
        let path = input
            .path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "missing".to_string());
        let line = input
            .line
            .map(|line| line.to_string())
            .unwrap_or_else(|| "missing".to_string());
        value.push_str(&format!(" path:{path} line:{line}"));
    }

    value
}

#[cfg(test)]
pub fn control_permission_inputs_for_test(
    operation: &str,
    input: ControlInput,
    resolved_session_id: u64,
    resolved_thread_id: i64,
) -> Vec<String> {
    permission_inputs(
        operation,
        [control_permission_input(&ResolvedControlInput {
            session_id: SessionId::from_proto(resolved_session_id),
            thread_id: project::debugger::session::ThreadId(resolved_thread_id),
            action: input.action,
            path: input.path,
            line: input.line,
            timeout_ms: input.timeout_ms,
        })],
    )
}

fn start_session_permission_inputs(
    operation: &str,
    input: &StartSessionInput,
) -> Result<Vec<String>> {
    let scenario = &input.scenario;
    let worktree_id = input
        .worktree_id
        .map(|worktree_id| worktree_id.to_string())
        .unwrap_or_else(|| "none".to_string());
    let build_task = scenario
        .build
        .as_ref()
        .map(|build| permission_value_to_string(build, "start_session build"))
        .transpose()?
        .unwrap_or_else(|| "null".to_string());
    let tcp_connection = scenario
        .tcp_connection
        .as_ref()
        .map(|tcp_connection| {
            permission_value_to_string(tcp_connection, "start_session tcp_connection")
        })
        .transpose()?
        .unwrap_or_else(|| "null".to_string());
    let config = permission_value_to_string(&scenario.config, "start_session config")?;

    Ok(permission_inputs(
        operation,
        [format!(
            "adapter:{} label:{} worktree_id:{} build_task:{} tcp_connection:{} config:{}",
            scenario.adapter, scenario.label, worktree_id, build_task, tcp_connection, config
        )],
    ))
}

fn permission_value_to_string(value: &impl Serialize, value_name: &str) -> Result<String> {
    let value = serde_json::to_value(value).with_context(|| {
        format!("Failed to serialize debugger {value_name} for permission matching")
    })?;
    serde_json::to_string(&sort_json_value(value)).with_context(|| {
        format!("Failed to serialize debugger {value_name} JSON for permission matching")
    })
}

fn sort_json_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(sort_json_value).collect()),
        Value::Object(object) => {
            let mut entries = object.into_iter().collect::<Vec<_>>();
            entries.sort_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));

            let mut sorted = serde_json::Map::new();
            for (key, value) in entries {
                sorted.insert(key, sort_json_value(value));
            }
            Value::Object(sorted)
        }
        value => value,
    }
}

fn operation_name(input: &DebuggerToolInput) -> &'static str {
    match input {
        DebuggerToolInput::ListSessions => "list_sessions",
        DebuggerToolInput::Snapshot(_) => "snapshot",
        DebuggerToolInput::ListBreakpoints => "list_breakpoints",
        DebuggerToolInput::SetBreakpoints { .. } => "set_breakpoints",
        DebuggerToolInput::RemoveBreakpoints { .. } => "remove_breakpoints",
        DebuggerToolInput::Control(_) => "control",
        DebuggerToolInput::ListAdapters => "list_adapters",
        DebuggerToolInput::StartSession(_) => "start_session",
        DebuggerToolInput::StopSession { .. } => "stop_session",
    }
}

fn initial_title_for_input(input: &DebuggerToolInput) -> SharedString {
    match input {
        DebuggerToolInput::ListSessions => "List debug sessions".into(),
        DebuggerToolInput::Snapshot(input) => input
            .session_id
            .map(|session_id| format!("Inspect debug session {session_id}").into())
            .unwrap_or_else(|| "Inspect debugger".into()),
        DebuggerToolInput::ListBreakpoints => "List debugger breakpoints".into(),
        DebuggerToolInput::SetBreakpoints { breakpoints } => {
            if breakpoints.len() == 1 {
                let breakpoint = &breakpoints[0];
                format!(
                    "Set debugger breakpoint at {}:{}",
                    MarkdownInlineCode(&breakpoint.path.to_string_lossy()),
                    breakpoint.line
                )
                .into()
            } else {
                format!("Set {} debugger breakpoints", breakpoints.len()).into()
            }
        }
        DebuggerToolInput::RemoveBreakpoints { breakpoints } => {
            if breakpoints.len() == 1 {
                let breakpoint = &breakpoints[0];
                format!(
                    "Remove debugger breakpoint at {}:{}",
                    MarkdownInlineCode(&breakpoint.path.to_string_lossy()),
                    breakpoint.line
                )
                .into()
            } else {
                format!("Remove {} debugger breakpoints", breakpoints.len()).into()
            }
        }
        DebuggerToolInput::Control(input) => match input.action {
            ControlAction::RunToLine => match (input.path.as_deref(), input.line) {
                (Some(path), Some(line)) => format!(
                    "Debugger run to line at {}:{}",
                    MarkdownInlineCode(&path.to_string_lossy()),
                    line
                )
                .into(),
                _ => "Debugger run to line".into(),
            },
            ControlAction::Continue
            | ControlAction::Pause
            | ControlAction::StepOver
            | ControlAction::StepIn
            | ControlAction::StepOut => format!("Debugger {}", input.action.label()).into(),
        },
        DebuggerToolInput::ListAdapters => "List debug adapters".into(),
        DebuggerToolInput::StartSession(input) => format!(
            "Start debug session {}",
            MarkdownInlineCode(&input.scenario.label)
        )
        .into(),
        DebuggerToolInput::StopSession { session_id } => {
            format!("Stop debug session {session_id}").into()
        }
    }
}

fn limits_from_input(input: Option<SnapshotLimitsInput>) -> Result<AgentDebuggerSnapshotLimits> {
    let mut limits = AgentDebuggerSnapshotLimits::default();
    if let Some(input) = input {
        if let Some(value) = input.max_frames {
            limits.max_frames = validate_snapshot_limit("max_frames", value, MAX_SNAPSHOT_FRAMES)?;
        }
        if let Some(value) = input.max_variables_per_scope {
            limits.max_variables_per_scope = validate_snapshot_limit(
                "max_variables_per_scope",
                value,
                MAX_SNAPSHOT_VARIABLES_PER_SCOPE,
            )?;
        }
        if let Some(value) = input.max_variable_value_length {
            limits.max_variable_value_length = validate_snapshot_limit(
                "max_variable_value_length",
                value,
                MAX_SNAPSHOT_VARIABLE_VALUE_LENGTH,
            )?;
        }
        if let Some(value) = input.max_output_events {
            limits.max_output_events =
                validate_snapshot_limit("max_output_events", value, MAX_SNAPSHOT_OUTPUT_EVENTS)?;
        }
        if let Some(value) = input.max_output_bytes {
            limits.max_output_bytes =
                validate_snapshot_limit("max_output_bytes", value, MAX_SNAPSHOT_OUTPUT_BYTES)?;
        }
        if let Some(value) = input.max_source_context_lines {
            limits.max_source_context_lines = validate_snapshot_limit(
                "max_source_context_lines",
                value,
                MAX_SNAPSHOT_SOURCE_CONTEXT_LINES,
            )?;
        }
    }
    Ok(limits)
}

fn validate_snapshot_limit(field_name: &str, value: usize, maximum: usize) -> Result<usize> {
    if value > maximum {
        anyhow::bail!(
            "debugger snapshot limit `{field_name}` must be at most {maximum}, got {value}"
        );
    }
    Ok(value)
}

fn validate_control_timeout(timeout_ms: Option<u64>) -> Result<()> {
    control_timeout_ms(timeout_ms).map(|_| ())
}

fn control_timeout_ms(timeout_ms: Option<u64>) -> Result<u64> {
    let timeout_ms = timeout_ms.unwrap_or(DEFAULT_CONTROL_TIMEOUT_MS);
    if timeout_ms > MAX_CONTROL_TIMEOUT_MS {
        anyhow::bail!(
            "debugger control `timeout_ms` must be at most {MAX_CONTROL_TIMEOUT_MS}, got {timeout_ms}"
        );
    }
    Ok(timeout_ms)
}

fn thread_picker_limits() -> AgentDebuggerSnapshotLimits {
    AgentDebuggerSnapshotLimits {
        max_frames: 0,
        max_variables_per_scope: 0,
        max_variable_value_length: 0,
        max_output_events: 0,
        max_output_bytes: 0,
        max_source_context_lines: 0,
    }
}

fn resolve_breakpoint_input(
    project: &Entity<Project>,
    mut breakpoint: BreakpointInput,
    cx: &gpui::AsyncApp,
) -> Result<BreakpointInput> {
    breakpoint.path = resolve_debugger_path(project, breakpoint.path, cx)?;
    Ok(breakpoint)
}

fn resolve_breakpoint_location(
    project: &Entity<Project>,
    mut breakpoint: BreakpointLocationInput,
    cx: &gpui::AsyncApp,
) -> Result<BreakpointLocationInput> {
    breakpoint.path = resolve_debugger_path(project, breakpoint.path, cx)?;
    Ok(breakpoint)
}

fn resolve_debugger_path(
    project: &Entity<Project>,
    path: PathBuf,
    cx: &gpui::AsyncApp,
) -> Result<PathBuf> {
    if path.is_absolute() {
        return normalize_debugger_path(path);
    }

    project.read_with(cx, |project, cx| {
        let project_path = project.find_project_path(&path, cx).ok_or_else(|| {
            anyhow!(
                "Could not resolve debugger source path `{}` in this project",
                path.display()
            )
        })?;
        let worktree = project
            .worktree_for_id(project_path.worktree_id, cx)
            .with_context(|| format!("Could not find worktree for `{}`", path.display()))?;
        normalize_debugger_path(worktree.read(cx).absolutize(&project_path.path))
    })
}

fn normalize_debugger_path(path: PathBuf) -> Result<PathBuf> {
    normalize_lexically(&path).with_context(|| {
        format!(
            "Could not normalize debugger source path `{}`",
            path.display()
        )
    })
}

fn resolve_session_id(
    project: &Entity<Project>,
    api: &AgentDebuggerApi,
    session_id: Option<u64>,
    cx: &App,
) -> Result<SessionId> {
    if let Some(session_id) = session_id {
        return Ok(SessionId::from_proto(session_id));
    }
    if let Some((session, _)) = project.read(cx).active_debug_session(cx) {
        return Ok(session.read(cx).session_id());
    }
    api.list_sessions(cx)
        .first()
        .map(|session| session.session_id)
        .ok_or_else(|| anyhow!("No active debug sessions. Start a debug session first."))
}

async fn choose_thread_for_action(
    api: &AgentDebuggerApi,
    session_id: SessionId,
    action: ControlAction,
    cx: &mut gpui::AsyncApp,
) -> Result<project::debugger::session::ThreadId> {
    let snapshot_task = cx.update(|cx| api.snapshot(session_id, thread_picker_limits(), cx));
    let snapshot = snapshot_task.await?;
    let preferred_status = match action {
        ControlAction::Pause => AgentDebuggerThreadStatus::Running,
        ControlAction::Continue
        | ControlAction::StepOver
        | ControlAction::StepIn
        | ControlAction::StepOut
        | ControlAction::RunToLine => AgentDebuggerThreadStatus::Stopped,
    };
    if let Some(thread) = snapshot
        .threads
        .iter()
        .find(|thread| thread.status == preferred_status)
    {
        return Ok(thread.thread_id);
    }

    let has_threads = !snapshot.threads.is_empty();
    match action {
        ControlAction::Pause => {
            if has_threads {
                // Some adapters accept a pause-by-thread-id even when no thread
                // is currently running, so fall back to the first thread.
                Ok(snapshot.threads[0].thread_id)
            } else {
                Err(anyhow!(
                    "No debugger threads available in session {:?}. The session must be running before it can be paused.",
                    session_id
                ))
            }
        }
        ControlAction::Continue
        | ControlAction::StepOver
        | ControlAction::StepIn
        | ControlAction::StepOut
        | ControlAction::RunToLine => {
            if has_threads {
                Err(anyhow!(
                    "No stopped debugger thread is available in session {:?}. The debugger must be paused at a breakpoint before this action can run; pause the session or wait for a breakpoint to hit.",
                    session_id
                ))
            } else {
                Err(anyhow!(
                    "No debugger threads available in session {:?}. Inspect a snapshot to confirm the session is still running.",
                    session_id
                ))
            }
        }
    }
}

fn sessions_to_json(sessions: Vec<AgentDebuggerSession>) -> Value {
    Value::Array(sessions.into_iter().map(session_to_json).collect())
}

fn session_to_json(session: AgentDebuggerSession) -> Value {
    json!({
        "session_id": session.session_id.0,
        "parent_session_id": session.parent_session_id.map(|id| id.0),
        "child_session_ids": session.child_session_ids.into_iter().map(|id| id.0).collect::<Vec<_>>(),
        "label": session.label,
        "adapter": session.adapter,
        "status": format!("{:?}", session.status).to_lowercase(),
        "is_attached": session.is_attached,
        "has_ever_stopped": session.has_ever_stopped,
    })
}

fn breakpoints_to_json(breakpoints: Vec<AgentSourceBreakpoint>) -> Value {
    Value::Array(
        breakpoints
            .into_iter()
            .map(|breakpoint| {
                json!({
                    "path": breakpoint.path,
                    "line": breakpoint.line,
                    "enabled": breakpoint.enabled,
                    "condition": breakpoint.condition,
                    "hit_condition": breakpoint.hit_condition,
                    "log_message": breakpoint.log_message,
                })
            })
            .collect(),
    )
}

fn breakpoint_edit_result_to_json(result: AgentBreakpointEditResult) -> Value {
    json!({
        "path": result.path,
        "line": result.line,
        "changed": result.changed,
    })
}

fn control_result_to_json(result: AgentDebuggerControlResult) -> Value {
    json!({
        "status": format!("{:?}", result.status).to_lowercase(),
        "stopped_thread_id": result.stopped_thread_id.map(|thread_id| thread_id.0),
    })
}

fn snapshot_to_json(snapshot: AgentDebuggerSnapshot) -> Value {
    json!({
        "session": session_to_json(snapshot.session),
        "threads": snapshot.threads.into_iter().map(thread_to_json).collect::<Vec<_>>(),
        "output": snapshot.output.into_iter().map(output_to_json).collect::<Vec<_>>(),
        "notes": snapshot.notes,
    })
}

fn thread_to_json(thread: AgentDebuggerThread) -> Value {
    json!({
        "thread_id": thread.thread_id.0,
        "name": thread.name,
        "status": format!("{:?}", thread.status).to_lowercase(),
        "frames": thread.frames.into_iter().map(frame_to_json).collect::<Vec<_>>(),
    })
}

fn frame_to_json(frame: AgentDebuggerStackFrame) -> Value {
    json!({
        "frame_id": frame.frame_id,
        "name": frame.name,
        "source_path": frame.source_path,
        "line": frame.line,
        "column": frame.column,
        "scopes": frame.scopes.into_iter().map(scope_to_json).collect::<Vec<_>>(),
        "source_context": frame.source_context.map(source_context_to_json),
    })
}

fn scope_to_json(scope: AgentDebuggerScope) -> Value {
    json!({
        "name": scope.name,
        "expensive": scope.expensive,
        "variables_reference": scope.variables_reference,
        "variables_truncated": scope.variables_truncated,
        "variables": scope.variables.into_iter().map(variable_to_json).collect::<Vec<_>>(),
    })
}

fn variable_to_json(variable: AgentDebuggerVariable) -> Value {
    json!({
        "name": variable.name,
        "value": variable.value,
        "type": variable.type_name,
        "variables_reference": variable.variables_reference,
        "named_variables": variable.named_variables,
        "indexed_variables": variable.indexed_variables,
        "value_truncated": variable.value_truncated,
    })
}

fn source_context_to_json(context: AgentSourceContext) -> Value {
    json!({
        "start_line": context.start_line,
        "truncated_before": context.truncated_before,
        "truncated_after": context.truncated_after,
        "lines": context.lines.into_iter().map(|line| {
            json!({
                "line": line.line,
                "text": line.text,
            })
        }).collect::<Vec<_>>(),
    })
}

fn output_to_json(output: AgentDebuggerOutputEvent) -> Value {
    json!({
        "category": output.category,
        "output": output.output,
        "output_truncated": output.output_truncated,
        "source_path": output.source_path,
        "line": output.line,
        "column": output.column,
    })
}
