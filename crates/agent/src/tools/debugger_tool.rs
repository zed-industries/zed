use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use project::debugger::breakpoint_store::{
    Breakpoint, BreakpointEditAction, BreakpointState, BreakpointWithPosition,
};
use project::debugger::session::{Session, ThreadId, ThreadStatus};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use text::Point;
use util::markdown::MarkdownInlineCode;

/// Interact with the debugger to control debug sessions, set breakpoints, and inspect program state.
///
/// This tool allows you to:
/// - Set and remove breakpoints at specific file locations
/// - List all breakpoints in the project
/// - List active debug sessions
/// - Control execution (continue, pause, step over, step in, step out)
/// - Inspect stack traces and variables when stopped at a breakpoint
///
/// <guidelines>
/// - Before using debugger controls (continue, pause, step), ensure there is an active debug session
/// - When setting breakpoints, use the exact file path as it appears in the project
/// - Stack traces and variables are only available when the debugger is stopped at a breakpoint
/// - Use `list_sessions` to see available debug sessions before trying to control them
/// </guidelines>
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DebuggerToolInput {
    /// The debugger operation to perform
    pub operation: DebuggerOperation,
    /// The path to the file (required for set_breakpoint and remove_breakpoint operations)
    #[serde(default)]
    pub path: Option<String>,
    /// The 1-based line number (required for set_breakpoint and remove_breakpoint operations)
    #[serde(default)]
    pub line: Option<u32>,
    /// Whether to enable or disable the breakpoint (for set_breakpoint only)
    #[serde(default)]
    pub enabled: Option<bool>,
    /// Optional condition expression that must evaluate to true for the breakpoint to trigger (for set_breakpoint only)
    #[serde(default)]
    pub condition: Option<String>,
    /// Optional log message to output when the breakpoint is hit (for set_breakpoint only)
    #[serde(default)]
    pub log_message: Option<String>,
    /// Optional hit count condition (for set_breakpoint only)
    #[serde(default)]
    pub hit_condition: Option<String>,
    /// Optional session ID. If not provided, uses the active session.
    #[serde(default)]
    pub session_id: Option<u32>,
    /// Optional thread ID. If not provided, uses an appropriate thread based on the operation.
    #[serde(default)]
    pub thread_id: Option<i64>,
    /// Optional stack frame index (0 = top of stack). Used for get_variables operation.
    #[serde(default)]
    pub frame_index: Option<usize>,
}

/// The debugger operation to perform
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DebuggerOperation {
    /// Set or update a breakpoint at a specific file and line
    SetBreakpoint,
    /// Remove a breakpoint at a specific file and line
    RemoveBreakpoint,
    /// List all breakpoints in the project
    ListBreakpoints,
    /// List all active debug sessions
    ListSessions,
    /// Continue execution of a paused thread
    Continue,
    /// Pause execution of a running thread
    Pause,
    /// Step over the current line (execute without entering functions)
    StepOver,
    /// Step into the current line (enter function calls)
    StepIn,
    /// Step out of the current function
    StepOut,
    /// Get the stack trace for a stopped thread
    GetStackTrace,
    /// Get variables in the current scope
    GetVariables,
}

pub struct DebuggerTool {
    project: Entity<Project>,
}

impl DebuggerTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }

    fn find_session(&self, session_id: Option<u32>, cx: &App) -> Result<Entity<Session>> {
        let dap_store = self.project.read(cx).dap_store();
        let dap_store = dap_store.read(cx);

        if let Some(id) = session_id {
            dap_store
                .session_by_id(dap::client::SessionId(id))
                .ok_or_else(|| anyhow!("No debug session found with ID {}", id))
        } else {
            // Try to get active session first
            if let Some((session, _)) = self.project.read(cx).active_debug_session(cx) {
                return Ok(session);
            }
            // Fall back to first available session
            dap_store
                .sessions()
                .next()
                .cloned()
                .ok_or_else(|| anyhow!("No active debug session. Start a debug session first."))
        }
    }

    fn find_stopped_thread(
        session: &Entity<Session>,
        thread_id: Option<i64>,
        cx: &mut App,
    ) -> Result<ThreadId> {
        session.update(cx, |session, cx| {
            let threads = session.threads(cx);

            if let Some(tid) = thread_id {
                let thread_id = ThreadId::from(tid);
                if threads
                    .iter()
                    .any(|(t, _)| ThreadId::from(t.id) == thread_id)
                {
                    return Ok(thread_id);
                }
                return Err(anyhow!("Thread {} not found", tid));
            }

            // Find first stopped thread
            threads
                .iter()
                .find(|(_, status)| *status == ThreadStatus::Stopped)
                .map(|(t, _)| ThreadId::from(t.id))
                .ok_or_else(|| {
                    anyhow!("No stopped thread found. The debugger must be paused at a breakpoint.")
                })
        })
    }

    fn find_running_thread(
        session: &Entity<Session>,
        thread_id: Option<i64>,
        cx: &mut App,
    ) -> Result<ThreadId> {
        session.update(cx, |session, cx| {
            let threads = session.threads(cx);

            if let Some(tid) = thread_id {
                let thread_id = ThreadId::from(tid);
                if threads
                    .iter()
                    .any(|(t, _)| ThreadId::from(t.id) == thread_id)
                {
                    return Ok(thread_id);
                }
                return Err(anyhow!("Thread {} not found", tid));
            }

            // Find first running thread
            threads
                .iter()
                .find(|(_, status)| *status == ThreadStatus::Running)
                .map(|(t, _)| ThreadId::from(t.id))
                .ok_or_else(|| anyhow!("No running thread found."))
        })
    }

    fn find_any_thread(
        session: &Entity<Session>,
        thread_id: Option<i64>,
        cx: &mut App,
    ) -> Result<ThreadId> {
        session.update(cx, |session, cx| {
            let threads = session.threads(cx);

            if let Some(tid) = thread_id {
                let thread_id = ThreadId::from(tid);
                if threads
                    .iter()
                    .any(|(t, _)| ThreadId::from(t.id) == thread_id)
                {
                    return Ok(thread_id);
                }
                return Err(anyhow!("Thread {} not found", tid));
            }

            // Find any thread, preferring stopped ones
            threads
                .iter()
                .find(|(_, status)| *status == ThreadStatus::Stopped)
                .or_else(|| threads.first())
                .map(|(t, _)| ThreadId::from(t.id))
                .ok_or_else(|| anyhow!("No threads found in the debug session."))
        })
    }
}

impl AgentTool for DebuggerTool {
    type Input = DebuggerToolInput;
    type Output = String;

    fn name() -> &'static str {
        "debugger"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => match input.operation {
                DebuggerOperation::SetBreakpoint => {
                    if let (Some(path), Some(line)) = (&input.path, input.line) {
                        format!("Set breakpoint at {}:{}", MarkdownInlineCode(path), line).into()
                    } else {
                        "Set breakpoint".into()
                    }
                }
                DebuggerOperation::RemoveBreakpoint => {
                    if let (Some(path), Some(line)) = (&input.path, input.line) {
                        format!("Remove breakpoint at {}:{}", MarkdownInlineCode(path), line).into()
                    } else {
                        "Remove breakpoint".into()
                    }
                }
                DebuggerOperation::ListBreakpoints => "List breakpoints".into(),
                DebuggerOperation::ListSessions => "List debug sessions".into(),
                DebuggerOperation::Continue => "Continue execution".into(),
                DebuggerOperation::Pause => "Pause execution".into(),
                DebuggerOperation::StepOver => "Step over".into(),
                DebuggerOperation::StepIn => "Step into".into(),
                DebuggerOperation::StepOut => "Step out".into(),
                DebuggerOperation::GetStackTrace => "Get stack trace".into(),
                DebuggerOperation::GetVariables => "Get variables".into(),
            },
            Err(_) => "Debugger operation".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        match input.operation {
            DebuggerOperation::SetBreakpoint => {
                let path = match input.path {
                    Some(p) => p,
                    None => {
                        return Task::ready(Err(anyhow!(
                            "path is required for set_breakpoint operation"
                        )));
                    }
                };
                let line = match input.line {
                    Some(l) => l,
                    None => {
                        return Task::ready(Err(anyhow!(
                            "line is required for set_breakpoint operation"
                        )));
                    }
                };
                let enabled = input.enabled;
                let condition = input.condition;
                let log_message = input.log_message;
                let hit_condition = input.hit_condition;
                let project = self.project.clone();
                cx.spawn(async move |cx| {
                    let (buffer, breakpoint_store, abs_path) = cx.update(|cx| {
                        let project_path = {
                            let project_ref = project.read(cx);
                            project_ref.find_project_path(&path, cx)
                        };
                        let Some(project_path) = project_path else {
                            return Err(anyhow!("Could not find path {} in project", path));
                        };

                        let breakpoint_store = project.read(cx).breakpoint_store();
                        let buffer_task = project.update(cx, |project, cx| {
                            project.open_buffer(project_path.clone(), cx)
                        });

                        // Get absolute path for the breakpoint store
                        let worktree = project
                            .read(cx)
                            .worktree_for_id(project_path.worktree_id, cx);
                        let abs_path =
                            worktree.map(|wt| wt.read(cx).absolutize(&project_path.path));

                        Ok((buffer_task, breakpoint_store, abs_path))
                    })??;

                    let buffer = buffer.await?;
                    let abs_path =
                        abs_path.ok_or_else(|| anyhow!("Could not determine absolute path"))?;

                    cx.update(|cx| {
                        let snapshot = buffer.read(cx).snapshot();
                        let row = line.saturating_sub(1);
                        let point = Point::new(row, 0);
                        let position = snapshot.anchor_before(point);

                        let state = match enabled {
                            Some(true) | None => BreakpointState::Enabled,
                            Some(false) => BreakpointState::Disabled,
                        };

                        let breakpoint = Breakpoint {
                            message: log_message.map(|s| s.into()),
                            hit_condition: hit_condition.map(|s| s.into()),
                            condition: condition.map(|s| s.into()),
                            state,
                        };

                        let breakpoint_with_position = BreakpointWithPosition {
                            position,
                            bp: breakpoint,
                        };

                        let action = BreakpointEditAction::Toggle;

                        breakpoint_store.update(cx, |store, cx| {
                            store.toggle_breakpoint(buffer, breakpoint_with_position, action, cx);
                        });

                        Ok(format!("Breakpoint set at {}:{}", abs_path.display(), line))
                    })?
                })
            }

            DebuggerOperation::RemoveBreakpoint => {
                let path = match input.path {
                    Some(p) => p,
                    None => {
                        return Task::ready(Err(anyhow!(
                            "path is required for remove_breakpoint operation"
                        )));
                    }
                };
                let line = match input.line {
                    Some(l) => l,
                    None => {
                        return Task::ready(Err(anyhow!(
                            "line is required for remove_breakpoint operation"
                        )));
                    }
                };
                let project = self.project.clone();
                cx.spawn(async move |cx| {
                    cx.update(|cx| {
                        let project = project.read(cx);
                        let Some(project_path) = project.find_project_path(&path, cx) else {
                            return Err(anyhow!("Could not find path {} in project", path));
                        };

                        let worktree = project
                            .worktree_for_id(project_path.worktree_id, cx)
                            .ok_or_else(|| anyhow!("Worktree not found"))?;
                        let abs_path = worktree.read(cx).absolutize(&project_path.path);

                        let breakpoint_store = project.breakpoint_store();
                        let row = line.saturating_sub(1);

                        let result = breakpoint_store
                            .read(cx)
                            .breakpoint_at_row(&abs_path, row, cx);

                        if let Some((buffer, breakpoint)) = result {
                            breakpoint_store.update(cx, |store, cx| {
                                store.toggle_breakpoint(
                                    buffer,
                                    breakpoint,
                                    BreakpointEditAction::Toggle,
                                    cx,
                                );
                            });
                            Ok(format!("Breakpoint removed at {}:{}", path, line))
                        } else {
                            Ok(format!("No breakpoint found at {}:{}", path, line))
                        }
                    })?
                })
            }

            DebuggerOperation::ListBreakpoints => {
                let breakpoint_store = self.project.read(cx).breakpoint_store();
                let breakpoints = breakpoint_store.read(cx).all_source_breakpoints(cx);

                let mut output = String::new();
                if breakpoints.is_empty() {
                    output.push_str("No breakpoints set.");
                } else {
                    writeln!(output, "Breakpoints:").ok();
                    for (path, bps) in &breakpoints {
                        for bp in bps {
                            let state = if bp.state.is_enabled() {
                                "enabled"
                            } else {
                                "disabled"
                            };
                            let mut details = vec![state.to_string()];

                            if let Some(ref cond) = bp.condition {
                                details.push(format!("condition: {}", cond));
                            }
                            if let Some(ref msg) = bp.message {
                                details.push(format!("log: {}", msg));
                            }
                            if let Some(ref hit) = bp.hit_condition {
                                details.push(format!("hit: {}", hit));
                            }

                            writeln!(
                                output,
                                "  - {}:{} [{}]",
                                path.display(),
                                bp.row + 1,
                                details.join(", ")
                            )
                            .ok();
                        }
                    }
                }
                Task::ready(Ok(output))
            }

            DebuggerOperation::ListSessions => {
                let dap_store = self.project.read(cx).dap_store();
                let sessions: Vec<_> = dap_store.read(cx).sessions().cloned().collect();

                let mut output = String::new();
                if sessions.is_empty() {
                    output.push_str("No active debug sessions.");
                } else {
                    writeln!(output, "Debug sessions:").ok();
                    for session in sessions {
                        let session_ref = session.read(cx);
                        let session_id = session_ref.session_id();
                        let adapter = session_ref.adapter();
                        let label = session_ref.label();
                        let is_terminated = session_ref.is_terminated();

                        let status = if is_terminated {
                            "terminated"
                        } else if session_ref.is_building() {
                            "building"
                        } else {
                            "running"
                        };

                        let label_str = label.as_ref().map(|l| l.as_ref()).unwrap_or("unnamed");
                        writeln!(
                            output,
                            "  - Session {} ({}): {} [{}]",
                            session_id.0, adapter, label_str, status
                        )
                        .ok();
                    }
                }
                Task::ready(Ok(output))
            }

            DebuggerOperation::Continue => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_stopped_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                session.update(cx, |session, cx| {
                    session.continue_thread(tid, cx);
                });

                Task::ready(Ok(format!("Continued execution of thread {}", tid.0)))
            }

            DebuggerOperation::Pause => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_running_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                session.update(cx, |session, cx| {
                    session.pause_thread(tid, cx);
                });

                Task::ready(Ok(format!("Paused thread {}", tid.0)))
            }

            DebuggerOperation::StepOver => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_stopped_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                session.update(cx, |session, cx| {
                    session.step_over(tid, dap::SteppingGranularity::Line, cx);
                });

                Task::ready(Ok(format!("Stepped over on thread {}", tid.0)))
            }

            DebuggerOperation::StepIn => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_stopped_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                session.update(cx, |session, cx| {
                    session.step_in(tid, dap::SteppingGranularity::Line, cx);
                });

                Task::ready(Ok(format!("Stepped into on thread {}", tid.0)))
            }

            DebuggerOperation::StepOut => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_stopped_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                session.update(cx, |session, cx| {
                    session.step_out(tid, dap::SteppingGranularity::Line, cx);
                });

                Task::ready(Ok(format!("Stepped out on thread {}", tid.0)))
            }

            DebuggerOperation::GetStackTrace => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_any_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                let stack_frames = session.update(cx, |session, cx| session.stack_frames(tid, cx));

                match stack_frames {
                    Ok(frames) => {
                        let mut output = String::new();
                        if frames.is_empty() {
                            output
                                .push_str("No stack frames available. The thread may be running.");
                        } else {
                            writeln!(output, "Stack trace for thread {}:", tid.0).ok();
                            for (i, frame) in frames.iter().enumerate() {
                                let location = frame
                                    .dap
                                    .source
                                    .as_ref()
                                    .and_then(|s| s.path.as_ref())
                                    .map(|p| format!("{}:{}", p, frame.dap.line))
                                    .unwrap_or_else(|| "unknown".to_string());

                                writeln!(output, "  #{} {} at {}", i, frame.dap.name, location)
                                    .ok();
                            }
                        }
                        Task::ready(Ok(output))
                    }
                    Err(e) => Task::ready(Err(e)),
                }
            }

            DebuggerOperation::GetVariables => {
                let session_id = input.session_id;
                let thread_id = input.thread_id;
                let frame_index = input.frame_index;
                let session = match self.find_session(session_id, cx) {
                    Ok(s) => s,
                    Err(e) => return Task::ready(Err(e)),
                };

                let tid = match Self::find_stopped_thread(&session, thread_id, cx) {
                    Ok(t) => t,
                    Err(e) => return Task::ready(Err(e)),
                };

                let frame_idx = frame_index.unwrap_or(0);

                let result = session.update(cx, |session, cx| {
                    let frames = session.stack_frames(tid, cx)?;

                    let frame = frames.get(frame_idx).ok_or_else(|| {
                        anyhow!(
                            "Stack frame index {} out of range (0-{})",
                            frame_idx,
                            frames.len().saturating_sub(1)
                        )
                    })?;

                    let frame_id = frame.dap.id;
                    let frame_name = frame.dap.name.clone();

                    // Get scopes and collect them to avoid borrow issues
                    let scopes: Vec<_> = session.scopes(frame_id, cx).to_vec();

                    let mut output = String::new();
                    if scopes.is_empty() {
                        output.push_str("No variables available in the current scope.");
                    } else {
                        writeln!(
                            output,
                            "Variables in frame #{} ({}):",
                            frame_idx, frame_name
                        )
                        .ok();

                        for scope in &scopes {
                            writeln!(output, "\n  {}:", scope.name).ok();

                            let variables = session.variables(scope.variables_reference.into(), cx);
                            for var in variables {
                                let type_info = var
                                    .type_
                                    .as_ref()
                                    .map(|t| format!(" ({})", t))
                                    .unwrap_or_default();

                                writeln!(output, "    {} = {}{}", var.name, var.value, type_info)
                                    .ok();
                            }
                        }
                    }

                    Ok(output)
                });

                Task::ready(result)
            }
        }
    }
}
