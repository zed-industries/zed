use super::{
    breakpoint_store::{BreakpointState, BreakpointStore, SourceBreakpoint},
    dap_store::DapStore,
    session::{OutputToken, Session, SessionEvent, SessionStateEvent, ThreadId, ThreadStatus},
};
use anyhow::{Context as _, Result, anyhow};
use dap::{
    StackFrameId, StackFramePresentationHint, SteppingGranularity, VariableReference,
    client::SessionId,
};
use futures::{FutureExt as _, select_biased};
use gpui::{App, AsyncApp, Entity, Subscription, Task};
use parking_lot::Mutex;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

#[derive(Clone)]
pub struct AgentDebuggerApi {
    dap_store: Entity<DapStore>,
    breakpoint_store: Entity<BreakpointStore>,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerSession {
    pub session_id: SessionId,
    pub parent_session_id: Option<SessionId>,
    pub child_session_ids: Vec<SessionId>,
    pub label: Option<String>,
    pub adapter: String,
    pub status: AgentDebuggerSessionStatus,
    pub is_attached: bool,
    pub has_ever_stopped: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentDebuggerSessionStatus {
    Booting,
    Running,
    Stopped,
    Terminated,
}

#[derive(Clone, Debug)]
pub struct AgentSourceBreakpoint {
    pub path: PathBuf,
    pub line: u32,
    pub enabled: bool,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
    pub log_message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AgentSourceBreakpointInput {
    pub path: PathBuf,
    pub line: u32,
    pub enabled: bool,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
    pub log_message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AgentBreakpointEditResult {
    pub path: PathBuf,
    pub line: u32,
    pub changed: bool,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerSnapshotLimits {
    pub max_frames: usize,
    pub max_variables_per_scope: usize,
    pub max_variable_value_length: usize,
    pub max_output_events: usize,
    pub max_output_bytes: usize,
    pub max_source_context_lines: usize,
}

impl Default for AgentDebuggerSnapshotLimits {
    fn default() -> Self {
        Self {
            max_frames: 20,
            max_variables_per_scope: 50,
            max_variable_value_length: 1024,
            max_output_events: 100,
            max_output_bytes: 16 * 1024,
            max_source_context_lines: 5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerSnapshot {
    pub session: AgentDebuggerSession,
    pub threads: Vec<AgentDebuggerThread>,
    pub output: Vec<AgentDebuggerOutputEvent>,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerThread {
    pub thread_id: ThreadId,
    pub name: String,
    pub status: AgentDebuggerThreadStatus,
    pub frames: Vec<AgentDebuggerStackFrame>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentDebuggerThreadStatus {
    Running,
    Stopped,
    Stepping,
    Exited,
    Ended,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerStackFrame {
    pub frame_id: StackFrameId,
    pub name: String,
    pub source_path: Option<PathBuf>,
    pub line: u64,
    pub column: u64,
    pub scopes: Vec<AgentDebuggerScope>,
    pub source_context: Option<AgentSourceContext>,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerScope {
    pub name: String,
    pub expensive: bool,
    pub variables_reference: VariableReference,
    pub variables: Vec<AgentDebuggerVariable>,
    pub variables_truncated: bool,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerVariable {
    pub name: String,
    pub value: String,
    pub type_name: Option<String>,
    pub variables_reference: VariableReference,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub value_truncated: bool,
}

#[derive(Clone, Debug)]
pub struct AgentSourceContext {
    pub start_line: u32,
    pub lines: Vec<AgentSourceContextLine>,
    pub truncated_before: bool,
    pub truncated_after: bool,
}

#[derive(Clone, Debug)]
pub struct AgentSourceContextLine {
    pub line: u32,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerOutputEvent {
    pub category: Option<String>,
    pub output: String,
    pub output_truncated: bool,
    pub source_path: Option<PathBuf>,
    pub line: Option<u64>,
    pub column: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentDebuggerStepKind {
    In,
    Out,
    Over,
}

#[derive(Clone, Debug)]
pub struct AgentDebuggerControlResult {
    pub status: AgentDebuggerWaitStatus,
    pub stopped_thread_id: Option<ThreadId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentDebuggerWaitStatus {
    Stopped,
    TimedOut,
    SessionEnded,
}

struct AgentDebuggerStopWait {
    receiver: futures::channel::oneshot::Receiver<AgentDebuggerWaitEvent>,
    _stopped_subscription: Subscription,
    _shutdown_subscription: Subscription,
}

#[derive(Clone, Copy, Debug)]
enum AgentDebuggerWaitEvent {
    Stopped(Option<ThreadId>),
    SessionEnded,
}

impl AgentDebuggerApi {
    pub fn new(dap_store: Entity<DapStore>, breakpoint_store: Entity<BreakpointStore>) -> Self {
        Self {
            dap_store,
            breakpoint_store,
        }
    }

    pub fn list_sessions(&self, cx: &App) -> Vec<AgentDebuggerSession> {
        self.dap_store
            .read(cx)
            .sessions()
            .map(|session| Self::session_summary(session, cx))
            .collect()
    }

    pub fn list_breakpoints(&self, cx: &App) -> Vec<AgentSourceBreakpoint> {
        self.breakpoint_store
            .read(cx)
            .all_source_breakpoints(cx)
            .into_values()
            .flatten()
            .map(AgentSourceBreakpoint::from_project_breakpoint)
            .collect()
    }

    pub fn set_source_breakpoint(
        &self,
        breakpoint: AgentSourceBreakpointInput,
        cx: &mut App,
    ) -> Task<Result<AgentBreakpointEditResult>> {
        let breakpoint_store = self.breakpoint_store.clone();
        cx.spawn(async move |cx| {
            let source_breakpoint = breakpoint.to_project_breakpoint()?;
            let path = source_breakpoint.path.as_ref().to_path_buf();
            let line = breakpoint.line;
            let changed = breakpoint_store
                .update(cx, |breakpoint_store, cx| {
                    breakpoint_store.set_source_breakpoint(source_breakpoint, cx)
                })
                .await?;

            Ok(AgentBreakpointEditResult {
                path,
                line,
                changed,
            })
        })
    }

    pub fn remove_source_breakpoint(
        &self,
        path: PathBuf,
        line: u32,
        cx: &mut App,
    ) -> Task<Result<AgentBreakpointEditResult>> {
        let breakpoint_store = self.breakpoint_store.clone();
        cx.spawn(async move |cx| {
            let row = line_to_row(line)?;
            let path = Arc::<Path>::from(path);
            let changed = breakpoint_store.update(cx, |breakpoint_store, cx| {
                breakpoint_store.remove_source_breakpoint(path.clone(), row, cx)
            })?;

            Ok(AgentBreakpointEditResult {
                path: path.as_ref().to_path_buf(),
                line,
                changed,
            })
        })
    }

    pub fn snapshot(
        &self,
        session_id: SessionId,
        limits: AgentDebuggerSnapshotLimits,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerSnapshot>> {
        let dap_store = self.dap_store.clone();
        let breakpoint_store = self.breakpoint_store.clone();
        cx.spawn(async move |cx| {
            let session = session_by_id(&dap_store, session_id, cx)?;
            let mut notes = Vec::new();
            let session_summary = session.read_with(cx, |session, cx| {
                AgentDebuggerApi::session_summary_for_session(session, cx)
            });
            let output = session.read_with(cx, |session, _| {
                bounded_output(session, &limits, &mut notes)
            });
            let dap_threads = session
                .update(cx, |session, _| session.agent_fetch_threads())
                .await?;
            let mut remaining_frames = limits.max_frames;
            let mut frames_truncated = false;
            let mut threads = Vec::new();

            if limits.max_frames == 0 {
                notes.push("Stack frames omitted because max_frames is 0".to_string());
            }

            for dap_thread in dap_threads {
                let thread_id = ThreadId(dap_thread.id);
                let status = session.read_with(cx, |session, _| session.thread_status(thread_id));
                let mut frames = Vec::new();

                if status == ThreadStatus::Stopped && remaining_frames > 0 {
                    let requested_frames = remaining_frames.saturating_add(1);
                    let mut fetched_frames = session
                        .update(cx, |session, _| {
                            session.agent_fetch_stack_frames(thread_id, requested_frames)
                        })
                        .await?
                        .into_iter()
                        .filter(|frame| {
                            !(frame.id == 0
                                && frame.line == 0
                                && frame.column == 0
                                && frame.presentation_hint
                                    == Some(StackFramePresentationHint::Label))
                        })
                        .collect::<Vec<_>>();

                    if fetched_frames.len() > remaining_frames {
                        frames_truncated = true;
                        fetched_frames.truncate(remaining_frames);
                    }

                    remaining_frames = remaining_frames.saturating_sub(fetched_frames.len());

                    for frame in fetched_frames {
                        frames.push(
                            stack_frame_snapshot(
                                &session,
                                &breakpoint_store,
                                frame,
                                &limits,
                                &mut notes,
                                cx,
                            )
                            .await?,
                        );
                    }
                }

                threads.push(AgentDebuggerThread {
                    thread_id,
                    name: dap_thread.name,
                    status: AgentDebuggerThreadStatus::from_thread_status(status),
                    frames,
                });
            }

            if remaining_frames == limits.max_frames
                && !threads
                    .iter()
                    .any(|thread| thread.status == AgentDebuggerThreadStatus::Stopped)
            {
                notes.push(
                    "No stopped threads; stack frames and variables were not requested".to_string(),
                );
            } else if frames_truncated {
                notes.push(format!(
                    "Stack frames truncated to {} frame(s)",
                    limits.max_frames
                ));
            }

            Ok(AgentDebuggerSnapshot {
                session: session_summary,
                threads,
                output,
                notes,
            })
        })
    }

    pub fn continue_thread(
        &self,
        session_id: SessionId,
        thread_id: ThreadId,
        timeout: Duration,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerControlResult>> {
        let dap_store = self.dap_store.clone();
        cx.spawn(async move |cx| {
            let session = session_by_id(&dap_store, session_id, cx)?;
            let stop_wait = subscribe_to_stop(session.clone(), cx)?;
            session
                .update(cx, |session, cx| {
                    session.agent_continue_thread(thread_id, cx)
                })
                .await?;
            wait_for_stop_or_timeout(stop_wait, timeout, cx).await
        })
    }

    pub fn pause_thread(
        &self,
        session_id: SessionId,
        thread_id: ThreadId,
        timeout: Duration,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerControlResult>> {
        let dap_store = self.dap_store.clone();
        cx.spawn(async move |cx| {
            let session = session_by_id(&dap_store, session_id, cx)?;
            if session.read_with(cx, |session, _| session.thread_status(thread_id))
                == ThreadStatus::Stopped
            {
                return Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::Stopped,
                    stopped_thread_id: Some(thread_id),
                });
            }

            let stop_wait = subscribe_to_stop(session.clone(), cx)?;
            session
                .update(cx, |session, cx| session.agent_pause_thread(thread_id, cx))
                .await?;
            wait_for_stop_or_timeout(stop_wait, timeout, cx).await
        })
    }

    pub fn step_thread(
        &self,
        session_id: SessionId,
        thread_id: ThreadId,
        step_kind: AgentDebuggerStepKind,
        timeout: Duration,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerControlResult>> {
        let dap_store = self.dap_store.clone();
        cx.spawn(async move |cx| {
            let session = session_by_id(&dap_store, session_id, cx)?;
            let stop_wait = subscribe_to_stop(session.clone(), cx)?;
            session
                .update(cx, |session, cx| match step_kind {
                    AgentDebuggerStepKind::In => {
                        session.agent_step_in(thread_id, SteppingGranularity::Line, cx)
                    }
                    AgentDebuggerStepKind::Out => {
                        session.agent_step_out(thread_id, SteppingGranularity::Line, cx)
                    }
                    AgentDebuggerStepKind::Over => {
                        session.agent_step_over(thread_id, SteppingGranularity::Line, cx)
                    }
                })
                .await?;
            wait_for_stop_or_timeout(stop_wait, timeout, cx).await
        })
    }

    pub fn run_to_line(
        &self,
        session_id: SessionId,
        thread_id: ThreadId,
        path: PathBuf,
        line: u32,
        timeout: Duration,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerControlResult>> {
        let dap_store = self.dap_store.clone();
        cx.spawn(async move |cx| {
            let row = line_to_row(line)?;
            let session = session_by_id(&dap_store, session_id, cx)?;
            let stop_wait = subscribe_to_stop(session.clone(), cx)?;
            let breakpoint = SourceBreakpoint {
                row,
                path: Arc::<Path>::from(path),
                message: None,
                condition: None,
                hit_condition: None,
                state: BreakpointState::Enabled,
            };
            session
                .update(cx, |session, cx| {
                    session.agent_run_to_position(breakpoint, thread_id, cx)
                })
                .await?;
            wait_for_stop_or_timeout(stop_wait, timeout, cx).await
        })
    }

    fn session_summary(session: &Entity<Session>, cx: &App) -> AgentDebuggerSession {
        session.read_with(cx, |session, cx| {
            Self::session_summary_for_session(session, cx)
        })
    }

    fn session_summary_for_session(session: &Session, cx: &App) -> AgentDebuggerSession {
        let mut child_session_ids = session.child_session_ids().into_iter().collect::<Vec<_>>();
        child_session_ids.sort();
        let status = if session.is_terminated() {
            AgentDebuggerSessionStatus::Terminated
        } else if session.is_building() {
            AgentDebuggerSessionStatus::Booting
        } else if session.any_stopped_thread() {
            AgentDebuggerSessionStatus::Stopped
        } else {
            AgentDebuggerSessionStatus::Running
        };

        AgentDebuggerSession {
            session_id: session.session_id(),
            parent_session_id: session.parent_id(cx),
            child_session_ids,
            label: session.label().map(|label| label.to_string()),
            adapter: session.adapter().to_string(),
            status,
            is_attached: session.is_attached(),
            has_ever_stopped: session.has_ever_stopped(),
        }
    }
}

impl AgentSourceBreakpoint {
    fn from_project_breakpoint(breakpoint: SourceBreakpoint) -> Self {
        Self {
            path: breakpoint.path.as_ref().to_path_buf(),
            line: breakpoint.row.saturating_add(1),
            enabled: breakpoint.state.is_enabled(),
            condition: breakpoint
                .condition
                .as_ref()
                .map(|condition| condition.to_string()),
            hit_condition: breakpoint
                .hit_condition
                .as_ref()
                .map(|hit_condition| hit_condition.to_string()),
            log_message: breakpoint
                .message
                .as_ref()
                .map(|message| message.to_string()),
        }
    }
}

impl AgentSourceBreakpointInput {
    fn to_project_breakpoint(&self) -> Result<SourceBreakpoint> {
        let row = line_to_row(self.line)?;
        Ok(SourceBreakpoint {
            row,
            path: Arc::<Path>::from(self.path.clone()),
            message: self.log_message.clone().map(Arc::<str>::from),
            condition: self.condition.clone().map(Arc::<str>::from),
            hit_condition: self.hit_condition.clone().map(Arc::<str>::from),
            state: if self.enabled {
                BreakpointState::Enabled
            } else {
                BreakpointState::Disabled
            },
        })
    }
}

impl AgentDebuggerThreadStatus {
    fn from_thread_status(status: ThreadStatus) -> Self {
        match status {
            ThreadStatus::Running => Self::Running,
            ThreadStatus::Stopped => Self::Stopped,
            ThreadStatus::Stepping => Self::Stepping,
            ThreadStatus::Exited => Self::Exited,
            ThreadStatus::Ended => Self::Ended,
        }
    }
}

fn line_to_row(line: u32) -> Result<u32> {
    line.checked_sub(1)
        .with_context(|| "Debugger source breakpoint lines are 1-based")
}

fn session_by_id(
    dap_store: &Entity<DapStore>,
    session_id: SessionId,
    cx: &mut AsyncApp,
) -> Result<Entity<Session>> {
    dap_store
        .read_with(cx, |dap_store, _| dap_store.session_by_id(session_id))
        .with_context(|| format!("Could not find debugger session {:?}", session_id))
}

fn subscribe_to_stop(session: Entity<Session>, cx: &mut AsyncApp) -> Result<AgentDebuggerStopWait> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    let sender = Arc::new(Mutex::new(Some(sender)));
    let stopped_sender = sender.clone();
    let stopped_subscription = cx.update(|cx| {
        cx.subscribe(&session, move |_, event: &SessionEvent, _| {
            if let SessionEvent::Stopped(thread_id) = event
                && let Some(sender) = stopped_sender.lock().take()
            {
                sender
                    .send(AgentDebuggerWaitEvent::Stopped(*thread_id))
                    .ok();
            }
        })
    });
    let shutdown_subscription = cx.update(|cx| {
        cx.subscribe(&session, move |_, event: &SessionStateEvent, _| {
            if matches!(event, SessionStateEvent::Shutdown)
                && let Some(sender) = sender.lock().take()
            {
                sender.send(AgentDebuggerWaitEvent::SessionEnded).ok();
            }
        })
    });

    Ok(AgentDebuggerStopWait {
        receiver,
        _stopped_subscription: stopped_subscription,
        _shutdown_subscription: shutdown_subscription,
    })
}

async fn wait_for_stop_or_timeout(
    stop_wait: AgentDebuggerStopWait,
    timeout: Duration,
    cx: &mut AsyncApp,
) -> Result<AgentDebuggerControlResult> {
    let AgentDebuggerStopWait {
        receiver,
        _stopped_subscription,
        _shutdown_subscription,
    } = stop_wait;
    let mut receiver = receiver.fuse();
    let mut timer = cx.background_executor().timer(timeout).fuse();

    select_biased! {
        event = receiver => {
            match event.map_err(|_| anyhow!("Debugger stop waiter was dropped before completion"))? {
                AgentDebuggerWaitEvent::Stopped(stopped_thread_id) => Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::Stopped,
                    stopped_thread_id,
                }),
                AgentDebuggerWaitEvent::SessionEnded => Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::SessionEnded,
                    stopped_thread_id: None,
                }),
            }
        }
        _ = timer => Ok(AgentDebuggerControlResult {
            status: AgentDebuggerWaitStatus::TimedOut,
            stopped_thread_id: None,
        }),
    }
}

async fn stack_frame_snapshot(
    session: &Entity<Session>,
    breakpoint_store: &Entity<BreakpointStore>,
    frame: dap::StackFrame,
    limits: &AgentDebuggerSnapshotLimits,
    notes: &mut Vec<String>,
    cx: &mut AsyncApp,
) -> Result<AgentDebuggerStackFrame> {
    let mut scopes = Vec::new();
    let dap_scopes = session
        .update(cx, |session, _| session.agent_fetch_scopes(frame.id))
        .await?;

    for scope in dap_scopes {
        let variables = if scope.variables_reference == 0 || limits.max_variables_per_scope == 0 {
            if scope.variables_reference != 0 && limits.max_variables_per_scope == 0 {
                notes.push(format!(
                    "Variables for scope `{}` omitted because max_variables_per_scope is 0",
                    scope.name
                ));
            }
            Vec::new()
        } else {
            session
                .update(cx, |session, cx| {
                    session.agent_fetch_variables(
                        scope.variables_reference,
                        limits.max_variables_per_scope,
                        cx,
                    )
                })
                .await?
        };

        let known_variable_count = scope
            .named_variables
            .unwrap_or(0)
            .saturating_add(scope.indexed_variables.unwrap_or(0));
        let variables_truncated = if limits.max_variables_per_scope == 0 {
            scope.variables_reference != 0
        } else {
            known_variable_count > variables.len() as u64
                || variables.len() >= limits.max_variables_per_scope
        };
        if variables_truncated && limits.max_variables_per_scope > 0 {
            notes.push(format!(
                "Variables for scope `{}` truncated to {} variable(s)",
                scope.name,
                variables.len()
            ));
        }

        let variables = variables
            .into_iter()
            .map(|variable| variable_snapshot(variable, limits.max_variable_value_length))
            .collect::<Vec<_>>();
        if variables.iter().any(|variable| variable.value_truncated) {
            notes.push(format!(
                "Variable values for scope `{}` truncated to {} byte(s)",
                scope.name, limits.max_variable_value_length
            ));
        }

        scopes.push(AgentDebuggerScope {
            name: scope.name,
            expensive: scope.expensive,
            variables_reference: scope.variables_reference,
            variables,
            variables_truncated,
        });
    }

    let source_path = frame
        .source
        .as_ref()
        .and_then(|source| source.path.as_ref())
        .map(PathBuf::from);
    let source_context = source_context_for_frame(
        breakpoint_store,
        frame.source.as_ref(),
        frame.line,
        limits.max_source_context_lines,
        notes,
        cx,
    )
    .await?;

    Ok(AgentDebuggerStackFrame {
        frame_id: frame.id,
        name: frame.name,
        source_path,
        line: frame.line,
        column: frame.column,
        scopes,
        source_context,
    })
}

async fn source_context_for_frame(
    breakpoint_store: &Entity<BreakpointStore>,
    source: Option<&dap::Source>,
    line: u64,
    max_source_context_lines: usize,
    notes: &mut Vec<String>,
    cx: &mut AsyncApp,
) -> Result<Option<AgentSourceContext>> {
    if max_source_context_lines == 0 {
        return Ok(None);
    }

    let Some(path) = source.and_then(|source| source.path.as_ref()) else {
        return Ok(None);
    };
    let row = line
        .checked_sub(1)
        .and_then(|line| u32::try_from(line).ok());
    let Some(row) = row else {
        notes.push(format!(
            "Source context for `{path}` omitted because the debugger reported invalid line {line}"
        ));
        return Ok(None);
    };

    let path = Arc::<Path>::from(Path::new(path));
    match breakpoint_store
        .update(cx, |breakpoint_store, cx| {
            breakpoint_store.source_context_for_path(
                path.clone(),
                row,
                max_source_context_lines,
                cx,
            )
        })
        .await
    {
        Ok(context) => {
            if context.truncated_before || context.truncated_after {
                notes.push(format!(
                    "Source context for `{}` line {} truncated to {} line(s)",
                    path.display(),
                    line,
                    context.lines.len()
                ));
            }

            Ok(Some(AgentSourceContext {
                start_line: context.start_row.saturating_add(1),
                lines: context
                    .lines
                    .into_iter()
                    .enumerate()
                    .map(|(index, text)| AgentSourceContextLine {
                        line: context
                            .start_row
                            .saturating_add(u32::try_from(index).unwrap_or(u32::MAX))
                            .saturating_add(1),
                        text,
                    })
                    .collect(),
                truncated_before: context.truncated_before,
                truncated_after: context.truncated_after,
            }))
        }
        Err(error) => {
            notes.push(format!(
                "Source context for `{}` omitted: {error}",
                path.display()
            ));
            Ok(None)
        }
    }
}

fn variable_snapshot(variable: dap::Variable, max_value_length: usize) -> AgentDebuggerVariable {
    let (value, value_truncated) = truncate_string(variable.value, max_value_length);
    AgentDebuggerVariable {
        name: variable.name,
        value,
        type_name: variable.type_,
        variables_reference: variable.variables_reference,
        named_variables: variable.named_variables,
        indexed_variables: variable.indexed_variables,
        value_truncated,
    }
}

fn bounded_output(
    session: &Session,
    limits: &AgentDebuggerSnapshotLimits,
    notes: &mut Vec<String>,
) -> Vec<AgentDebuggerOutputEvent> {
    let (events, output_token) = session.output(OutputToken(0));
    let events = events.cloned().collect::<Vec<_>>();
    if output_token.0 > events.len() {
        notes.push(format!(
            "Debugger output ring retained {} of {} event(s)",
            events.len(),
            output_token.0
        ));
    }

    if limits.max_output_events == 0 || limits.max_output_bytes == 0 {
        if !events.is_empty() {
            notes.push("Debugger output omitted by output limits".to_string());
        }
        return Vec::new();
    }

    let mut bytes = 0usize;
    let mut selected_events = Vec::new();
    let mut truncated_by_events = false;
    let mut truncated_by_bytes = false;

    for event in events.iter().rev() {
        if selected_events.len() >= limits.max_output_events {
            truncated_by_events = true;
            break;
        }

        let event_bytes = event.output.len();
        if bytes.saturating_add(event_bytes) > limits.max_output_bytes {
            if selected_events.is_empty() {
                let mut event = output_event_snapshot(event.clone());
                let (output, truncated) = truncate_string(event.output, limits.max_output_bytes);
                event.output = output;
                event.output_truncated = truncated;
                selected_events.push(event);
            }
            truncated_by_bytes = true;
            break;
        }

        bytes += event_bytes;
        selected_events.push(output_event_snapshot(event.clone()));
    }

    selected_events.reverse();

    if truncated_by_events {
        notes.push(format!(
            "Debugger output truncated to the latest {} event(s)",
            selected_events.len()
        ));
    }
    if truncated_by_bytes {
        notes.push(format!(
            "Debugger output truncated to {} byte(s)",
            limits.max_output_bytes
        ));
    }

    selected_events
}

fn output_event_snapshot(event: dap::OutputEvent) -> AgentDebuggerOutputEvent {
    AgentDebuggerOutputEvent {
        category: event.category.map(|category| format!("{category:?}")),
        output: event.output,
        output_truncated: false,
        source_path: event
            .source
            .and_then(|source| source.path)
            .map(PathBuf::from),
        line: event.line,
        column: event.column,
    }
}

fn truncate_string(mut value: String, max_length: usize) -> (String, bool) {
    if value.len() <= max_length {
        return (value, false);
    }

    let mut boundary = max_length;
    while boundary > 0 && !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    value.truncate(boundary);
    (value, true)
}
