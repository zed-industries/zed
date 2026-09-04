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

impl AgentDebuggerSessionStatus {
    pub fn is_booting(&self) -> bool {
        matches!(self, Self::Booting)
    }
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
    /// Human-readable observations about the control outcome, surfaced in the
    /// tool response (e.g. "stopped at a breakpoint before reaching the
    /// run-to-line target").
    pub notes: Vec<String>,
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
        preferred_thread_id: Option<ThreadId>,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerSnapshot>> {
        let dap_store = self.dap_store.clone();
        let breakpoint_store = self.breakpoint_store.clone();
        cx.spawn(async move |cx| {
            let session = session_by_id(&dap_store, session_id, cx)?;
            snapshot_session(session, breakpoint_store, limits, preferred_thread_id, cx).await
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn snapshot_session_for_test(
        &self,
        session: Entity<Session>,
        limits: AgentDebuggerSnapshotLimits,
        cx: &mut App,
    ) -> Task<Result<AgentDebuggerSnapshot>> {
        let breakpoint_store = self.breakpoint_store.clone();
        cx.spawn(async move |cx| {
            snapshot_session(session, breakpoint_store, limits, None, cx).await
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
            if let Some(thread_id) = wait_for_boot_stop(&session, thread_id, timeout, cx).await? {
                return Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::Stopped,
                    stopped_thread_id: Some(thread_id),
                    notes: Vec::new(),
                });
            }
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
            wait_for_session_ready(&session, timeout, cx).await?;
            // A stale `Stopped` status is not proof the debuggee is halted —
            // Delve reports a synthetic "Current" thread as stopped while the
            // program keeps running. Only treat the session as already paused
            // when a stack trace actually succeeds.
            let already_stopped = session.read_with(cx, |session, _| session.thread_status(thread_id))
                == ThreadStatus::Stopped
                && wait_for_stack_trace(&session, Some(thread_id), cx).await;
            if already_stopped {
                return Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::Stopped,
                    stopped_thread_id: Some(thread_id),
                    notes: Vec::new(),
                });
            }

            // Some adapters acknowledge a pause without actually halting
            // (Debugpy needs a second pause to process the interrupt; Delve
            // can report a stale stop). Retry a bounded number of times, and
            // only treat a stop as real once a stack trace succeeds.
            let mut last_result = None;
            for _attempt in 0..3 {
                let stop_wait = subscribe_to_stop(session.clone(), cx)?;
                session
                    .update(cx, |session, cx| session.agent_pause_thread(thread_id, cx))
                    .await?;
                let result = wait_for_stop_or_timeout(stop_wait, timeout, cx).await?;
                match result.status {
                    AgentDebuggerWaitStatus::Stopped => {
                        if wait_for_stack_trace(&session, result.stopped_thread_id, cx).await {
                            return Ok(result);
                        }
                        last_result = Some(result);
                    }
                    AgentDebuggerWaitStatus::TimedOut => {
                        last_result = Some(result);
                    }
                    AgentDebuggerWaitStatus::SessionEnded => return Ok(result),
                }
            }

            if last_result
                .as_ref()
                .is_some_and(|result| result.status == AgentDebuggerWaitStatus::Stopped)
            {
                anyhow::bail!(
                    "Pause was acknowledged but the debuggee did not halt: stack traces still fail while the debuggee is running (adapter quirk)"
                );
            }
            Ok(last_result.unwrap_or(AgentDebuggerControlResult {
                status: AgentDebuggerWaitStatus::TimedOut,
                stopped_thread_id: None,
                notes: Vec::new(),
            }))
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
            if let Some(thread_id) = wait_for_boot_stop(&session, thread_id, timeout, cx).await? {
                return Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::Stopped,
                    stopped_thread_id: Some(thread_id),
                    notes: Vec::new(),
                });
            }
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
            if let Some(thread_id) = wait_for_boot_stop(&session, thread_id, timeout, cx).await? {
                return Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::Stopped,
                    stopped_thread_id: Some(thread_id),
                    notes: Vec::new(),
                });
            }
            let stop_wait = subscribe_to_stop(session.clone(), cx)?;
            let breakpoint = SourceBreakpoint {
                row,
                path: Arc::<Path>::from(path.clone()),
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
            let mut result = wait_for_stop_or_timeout(stop_wait, timeout, cx).await?;
            if result.status == AgentDebuggerWaitStatus::Stopped
                && let Some(stopped_thread_id) = result.stopped_thread_id
                && let Ok(frames) = session
                    .update(cx, |session, _| {
                        session.agent_fetch_stack_frames(stopped_thread_id, 1)
                    })
                    .await
                && let Some(top_frame) = frames.first()
            {
                // A breakpoint set earlier can fire before the run-to-line
                // target; tell the agent where execution actually stopped.
                if top_frame.line != u64::from(line) {
                    result.notes.push(format!(
                        "Stopped at line {} ({}) before reaching the run-to-line target line {line} — a breakpoint may have fired first",
                        top_frame.line, top_frame.name
                    ));
                }
            } else if result.status == AgentDebuggerWaitStatus::TimedOut {
                result
                    .notes
                    .push(format!("Timed out before reaching line {line}"));
            }
            Ok(result)
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

async fn snapshot_session(
    session: Entity<Session>,
    breakpoint_store: Entity<BreakpointStore>,
    limits: AgentDebuggerSnapshotLimits,
    preferred_thread_id: Option<ThreadId>,
    cx: &mut AsyncApp,
) -> Result<AgentDebuggerSnapshot> {
    let mut notes = Vec::new();
    let session_summary = session.read_with(cx, |session, cx| {
        AgentDebuggerApi::session_summary_for_session(session, cx)
    });
    let output = session.read_with(cx, |session, _| {
        bounded_output(session, &limits, &mut notes)
    });
    if session_summary.status == AgentDebuggerSessionStatus::Terminated {
        notes.push("Session has ended; threads were not requested".to_string());
        return Ok(AgentDebuggerSnapshot {
            session: session_summary,
            threads: Vec::new(),
            output,
            notes,
        });
    }

    // The adapter connects asynchronously after the session is registered;
    // fetching threads while the session is still booting races the
    // connection and fails with "no adapter running". Wait for the session
    // to become ready first, and report a clear note instead of failing if
    // the adapter never finishes booting.
    if session.read_with(cx, |session, _| session.is_building()) {
        if let Err(error) = wait_for_session_ready(&session, SNAPSHOT_BOOT_WAIT_TIMEOUT, cx).await {
            notes.push(format!(
                "Session is still starting; threads were not requested ({error})"
            ));
            return Ok(AgentDebuggerSnapshot {
                session: session_summary,
                threads: Vec::new(),
                output,
                notes,
            });
        }
    }

    let dap_threads = session
        .update(cx, |session, _| session.agent_fetch_threads())
        .await?;
    let adapter = session.read_with(cx, |session, _| session.adapter().to_string());
    let mut remaining_frames = limits.max_frames;
    let mut frames_truncated = false;
    let mut threads = Vec::new();

    if limits.max_frames == 0 {
        notes.push("Stack frames omitted because max_frames is 0".to_string());
    }

    // Collect the stopped threads, prioritizing the thread that reported the
    // stop. Adapters list worker threads before the main thread, and a single
    // global frame budget consumed in adapter order would let workers starve
    // the thread the model actually cares about.
    let mut stopped_threads = dap_threads
        .iter()
        .filter(|dap_thread| {
            session.read_with(cx, |session, _| {
                session.thread_status(ThreadId(dap_thread.id))
            }) == ThreadStatus::Stopped
        })
        .collect::<Vec<_>>();
    if let Some(preferred_thread_id) = preferred_thread_id
        && let Some(index) = stopped_threads
            .iter()
            .position(|dap_thread| ThreadId(dap_thread.id) == preferred_thread_id)
    {
        let preferred = stopped_threads.remove(index);
        stopped_threads.insert(0, preferred);
    }

    let mut skipped_thread_names = Vec::new();
    for (index, dap_thread) in stopped_threads.iter().enumerate() {
        let thread_id = ThreadId(dap_thread.id);

        // Delve synthesizes a "Dummy" thread for the paused program; it has
        // no stack and only adds noise to the snapshot.
        if adapter == "Delve" && dap_thread.name == "Dummy" {
            notes.push("Delve synthetic `Dummy` thread omitted from the snapshot".to_string());
            continue;
        }

        let mut frames = Vec::new();
        if remaining_frames == 0 {
            skipped_thread_names.push(dap_thread.name.clone());
        } else {
            // Reserve at least one frame for each stopped thread that hasn't
            // been processed yet, so earlier threads can't exhaust the budget
            // before later ones get any frames.
            let remaining_stopped = stopped_threads.len() - index;
            let reserved_for_rest = remaining_stopped
                .saturating_sub(1)
                .min(remaining_frames.saturating_sub(1));
            let available = remaining_frames - reserved_for_rest;
            let requested_frames = available.saturating_add(1);
            let fetched_frames = session
                .update(cx, |session, _| {
                    session.agent_fetch_stack_frames(thread_id, requested_frames)
                })
                .await;
            let mut fetched_frames = match fetched_frames {
                Ok(fetched_frames) => fetched_frames
                    .into_iter()
                    .filter(|frame| {
                        !(frame.id == 0
                            && frame.line == 0
                            && frame.column == 0
                            && frame.presentation_hint == Some(StackFramePresentationHint::Label))
                    })
                    .collect::<Vec<_>>(),
                Err(error) => {
                    notes.push(format!(
                        "Stack frames for thread `{}` ({thread_id:?}) omitted: {error}",
                        dap_thread.name
                    ));
                    Vec::new()
                }
            };

            if fetched_frames.len() > available {
                frames_truncated = true;
                fetched_frames.truncate(available);
            }

            remaining_frames = remaining_frames.saturating_sub(fetched_frames.len());

            for (frame_index, frame) in fetched_frames.into_iter().enumerate() {
                frames.push(
                    stack_frame_snapshot(
                        &session,
                        &breakpoint_store,
                        frame,
                        frame_index,
                        &limits,
                        &mut notes,
                        cx,
                    )
                    .await,
                );
            }
        }

        threads.push(AgentDebuggerThread {
            thread_id,
            name: dap_thread.name.clone(),
            status: AgentDebuggerThreadStatus::Stopped,
            frames,
        });
    }
    for thread_name in skipped_thread_names {
        notes.push(format!(
            "Stack frames for thread `{thread_name}` omitted: max_frames budget exhausted"
        ));
    }

    for dap_thread in dap_threads.iter().filter(|dap_thread| {
        session.read_with(cx, |session, _| {
            session.thread_status(ThreadId(dap_thread.id))
        }) != ThreadStatus::Stopped
    }) {
        let thread_id = ThreadId(dap_thread.id);
        let status = session.read_with(cx, |session, _| session.thread_status(thread_id));
        threads.push(AgentDebuggerThread {
            thread_id,
            name: dap_thread.name.clone(),
            status: AgentDebuggerThreadStatus::from_thread_status(status),
            frames: Vec::new(),
        });
    }

    if remaining_frames == limits.max_frames
        && !threads
            .iter()
            .any(|thread| thread.status == AgentDebuggerThreadStatus::Stopped)
    {
        notes.push("No stopped threads; stack frames and variables were not requested".to_string());
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

async fn wait_for_session_ready(
    session: &Entity<Session>,
    timeout: Duration,
    cx: &mut AsyncApp,
) -> Result<()> {
    let mut remaining = timeout;
    loop {
        if !session.read_with(cx, |session, _| session.is_building()) {
            return Ok(());
        }
        if remaining.is_zero() {
            anyhow::bail!("timed out waiting for debugger session to start");
        }
        let sleep = remaining.min(Duration::from_millis(50));
        cx.background_executor().timer(sleep).await;
        remaining = remaining.saturating_sub(sleep);
    }
}

/// How long a snapshot waits for a booting session to become ready before
/// giving up with a note (matches the adapter TCP connect timeout).
const SNAPSHOT_BOOT_WAIT_TIMEOUT: Duration = Duration::from_secs(15);

/// Polls a short window for a successful stack trace, which is the only
/// reliable signal that the debuggee is actually halted.
async fn wait_for_stack_trace(
    session: &Entity<Session>,
    thread_id: Option<ThreadId>,
    cx: &mut AsyncApp,
) -> bool {
    let Some(thread_id) = thread_id else {
        return false;
    };
    for _ in 0..30 {
        if session
            .update(cx, |session, _| {
                session.agent_fetch_stack_frames(thread_id, 1)
            })
            .await
            .is_ok()
        {
            return true;
        }
        cx.background_executor()
            .timer(Duration::from_millis(200))
            .await;
    }
    false
}

/// Waits for a session that is still booting to finish, and reports whether the
/// given thread ended up stopped — which happens when a breakpoint is hit
/// during launch. In that case the caller should return the current stop instead
/// of sending a continue/step that would resume straight past it.
async fn wait_for_boot_stop(
    session: &Entity<Session>,
    thread_id: ThreadId,
    timeout: Duration,
    cx: &mut AsyncApp,
) -> Result<Option<ThreadId>> {
    let was_booting = session.read_with(cx, |session, _| session.is_building());
    if !was_booting {
        return Ok(None);
    }
    wait_for_session_ready(session, timeout, cx).await?;
    let stopped = session.read_with(cx, |session, _| session.thread_status(thread_id))
        == ThreadStatus::Stopped;
    Ok(stopped.then_some(thread_id))
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
                    notes: Vec::new(),
                }),
                AgentDebuggerWaitEvent::SessionEnded => Ok(AgentDebuggerControlResult {
                    status: AgentDebuggerWaitStatus::SessionEnded,
                    stopped_thread_id: None,
                    notes: Vec::new(),
                }),
            }
        }
        _ = timer => Ok(AgentDebuggerControlResult {
            status: AgentDebuggerWaitStatus::TimedOut,
            stopped_thread_id: None,
            notes: Vec::new(),
        }),
    }
}

async fn stack_frame_snapshot(
    session: &Entity<Session>,
    breakpoint_store: &Entity<BreakpointStore>,
    frame: dap::StackFrame,
    frame_index: usize,
    limits: &AgentDebuggerSnapshotLimits,
    notes: &mut Vec<String>,
    cx: &mut AsyncApp,
) -> AgentDebuggerStackFrame {
    let source_path = frame
        .source
        .as_ref()
        .and_then(|source| source.path.as_ref())
        .map(PathBuf::from);

    let mut scopes = Vec::new();
    // Frames without a source path are adapter noise (worker threads, loader
    // disassembly). Only fetch scopes for the top frames and for frames that
    // resolve to a real file: scope/variable fetch per frame is the dominant
    // snapshot cost on adapters with huge Global/Static scopes.
    if frame_index > 1 && source_path.is_none() {
        notes.push(format!(
            "Scopes for frame `{}` ({}) omitted: no source path",
            frame.name, frame.id
        ));
    } else {
        let dap_scopes = match session
            .update(cx, |session, _| session.agent_fetch_scopes(frame.id))
            .await
        {
            Ok(scopes) => scopes,
            Err(error) => {
                notes.push(format!(
                    "Scopes for frame `{}` ({}) omitted: {error}",
                    frame.name, frame.id
                ));
                Vec::new()
            }
        };

        for scope in dap_scopes {
            // The agent doesn't need raw register dumps, and some adapters (GDB)
            // report hundreds of register variables regardless of the limits.
            if scope.name == "Registers" {
                notes.push("Registers scope omitted from the snapshot".to_string());
                continue;
            }

            // Global/Static scopes can be enormous (JS, CodeLLDB) and are
            // rarely what the agent needs to diagnose a stop; locals live in
            // the per-frame scopes.
            if scope.name == "Global" || scope.name == "Static" {
                notes.push(format!(
                    "`{}` scope omitted from frame `{}`",
                    scope.name, frame.name
                ));
                continue;
            }

            let mut variables_unavailable = false;
            let variables = if scope.variables_reference == 0 || limits.max_variables_per_scope == 0
            {
                if scope.variables_reference != 0 && limits.max_variables_per_scope == 0 {
                    notes.push(format!(
                        "Variables for scope `{}` omitted because max_variables_per_scope is 0",
                        scope.name
                    ));
                }
                Vec::new()
            } else {
                match session
                    .update(cx, |session, cx| {
                        session.agent_fetch_variables(
                            scope.variables_reference,
                            limits.max_variables_per_scope,
                            cx,
                        )
                    })
                    .await
                {
                    Ok(variables) => variables,
                    Err(error) => {
                        notes.push(format!(
                            "Variables for scope `{}` omitted: {error}",
                            scope.name
                        ));
                        variables_unavailable = true;
                        Vec::new()
                    }
                }
            };

            let known_variable_count = scope
                .named_variables
                .unwrap_or(0)
                .saturating_add(scope.indexed_variables.unwrap_or(0));
            let variables_truncated = if variables_unavailable {
                true
            } else if limits.max_variables_per_scope == 0 {
                scope.variables_reference != 0
            } else {
                known_variable_count > variables.len() as u64
                    || variables.len() >= limits.max_variables_per_scope
            };
            if variables_truncated && !variables_unavailable && limits.max_variables_per_scope > 0 {
                notes.push(format!(
                    "Variables for scope `{}` truncated to {} variable(s)",
                    scope.name,
                    variables.len()
                ));
            }

            // Delve exposes an uninitialized `~rN` return slot before the
            // return executes; it's adapter noise, not program state.
            let variables = variables
                .into_iter()
                .filter(|variable| !variable.name.starts_with("~r"))
                .collect::<Vec<_>>();

            let mut filtered_unavailable = 0usize;
            let mut expanded_aggregates = 0usize;
            let mut snapshot_variables = Vec::with_capacity(variables.len());
            for variable in variables.into_iter() {
                // CodeLLDB reports optimized-out variables as "<variable not
                // available>"; they carry no information for the agent.
                if matches!(
                    variable.value.as_str(),
                    "<not available>" | "<variable not available>" | "<optimized out>"
                ) {
                    filtered_unavailable += 1;
                    continue;
                }

                // Some adapters (gdb-dap) return aggregates with an empty
                // value string and only a variables_reference; expand one
                // level so the agent can see the contents.
                if expanded_aggregates < 4
                    && variable.value.is_empty()
                    && variable.variables_reference != 0
                    && variable
                        .indexed_variables
                        .is_some_and(|count| (1..=8).contains(&count))
                {
                    expanded_aggregates += 1;
                    let children = session
                        .update(cx, |session, cx| {
                            session.agent_fetch_variables(variable.variables_reference, 8, cx)
                        })
                        .await
                        .unwrap_or_default();
                    if !children.is_empty() {
                        let summary = format!(
                            "[{}]",
                            children
                                .iter()
                                .map(|child| child.value.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                        let (value, value_truncated) =
                            truncate_string(summary, limits.max_variable_value_length);
                        snapshot_variables.push(AgentDebuggerVariable {
                            name: variable.name,
                            value,
                            type_name: variable.type_,
                            variables_reference: variable.variables_reference,
                            named_variables: variable.named_variables,
                            indexed_variables: variable.indexed_variables,
                            value_truncated,
                        });
                        continue;
                    }
                }

                snapshot_variables.push(variable_snapshot(
                    variable,
                    limits.max_variable_value_length,
                ));
            }
            if filtered_unavailable > 0 {
                notes.push(format!(
                    "Variables for scope `{}` filtered {} `<not available>` entr{}",
                    scope.name,
                    filtered_unavailable,
                    if filtered_unavailable == 1 {
                        "y"
                    } else {
                        "ies"
                    }
                ));
            }
            if snapshot_variables
                .iter()
                .any(|variable| variable.value_truncated)
            {
                notes.push(format!(
                    "Variable values for scope `{}` truncated to {} byte(s)",
                    scope.name, limits.max_variable_value_length
                ));
            }

            scopes.push(AgentDebuggerScope {
                name: scope.name,
                expensive: scope.expensive,
                variables_reference: scope.variables_reference,
                variables: snapshot_variables,
                variables_truncated,
            });
        }
    }

    let source_context = match source_context_for_frame(
        breakpoint_store,
        frame.source.as_ref(),
        frame.line,
        limits.max_source_context_lines,
        notes,
        cx,
    )
    .await
    {
        Ok(context) => context,
        Err(error) => {
            notes.push(format!(
                "Source context for frame `{}` ({}) omitted: {error}",
                frame.name, frame.id
            ));
            None
        }
    };

    AgentDebuggerStackFrame {
        frame_id: frame.id,
        name: frame.name,
        source_path,
        line: frame.line,
        column: frame.column,
        scopes,
        source_context,
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FakeFs, Project};
    use dap::adapters::{DebugAdapterBinary, DebugAdapterName};
    use dap::client::DebugAdapterClient;
    use dap::{StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest};
    use gpui::{BackgroundExecutor, TestAppContext};
    use serde_json::json;
    use settings::SettingsStore;
    use task::SharedTaskContext;
    use util::path;

    /// Exercises the deterministic half of the boot-race fix: when a control
    /// operation is issued while the session is still booting, it must *wait*
    /// for the adapter to become ready instead of racing it and failing with
    /// "no adapter running". A session that never leaves `Booting` therefore
    /// times out with an explicit error.
    #[gpui::test]
    async fn test_control_waits_for_boot_instead_of_racing_adapter(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(path!("/project"), json!({})).await;

        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
        });

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

        // `Session::new` starts in the `Booting` state; never calling `boot`
        // keeps it there, which is exactly the window the fix must guard.
        let session = cx.update(|cx| {
            let breakpoint_store = project.read(cx).breakpoint_store();
            Session::new(
                breakpoint_store,
                SessionId(1),
                None,
                None,
                DebugAdapterName("fake-adapter".into()),
                SharedTaskContext::default(),
                crate::debugger::session::SessionQuirks::default(),
                None,
                None,
                None,
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                cx.spawn(async move |cx| {
                    wait_for_boot_stop(&session, ThreadId(1), Duration::from_millis(20), cx).await
                })
            })
            .await;

        let error = result.expect_err("should time out while the session is still booting");
        assert!(
            error
                .to_string()
                .contains("timed out waiting for debugger session to start"),
            "unexpected error: {error}"
        );
    }

    /// Exercises the other half of the boot-race fix: when a control operation
    /// is issued while the session is still booting and a breakpoint was hit
    /// during that boot, the operation must return the already-hit stop rather
    /// than sending a continue that resumes straight past it.
    #[gpui::test]
    async fn test_wait_for_boot_stop_returns_breakpoint_hit_during_boot(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(path!("/project"), json!({})).await;
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
        });
        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

        let client = Arc::new(
            DebugAdapterClient::start(
                SessionId(1),
                DebugAdapterBinary {
                    command: Some("command".into()),
                    arguments: Default::default(),
                    envs: Default::default(),
                    cwd: None,
                    connection: None,
                    request_args: StartDebuggingRequestArguments {
                        configuration: serde_json::Value::Null,
                        request: StartDebuggingRequestArgumentsRequest::Launch,
                    },
                },
                Box::new(|_| {}),
                &mut cx.to_async(),
            )
            .await
            .unwrap(),
        );

        let worktree = cx.update(|cx| project.read(cx).worktrees(cx).next().unwrap().downgrade());

        let session = cx.update(|cx| {
            Session::new(
                project.read(cx).breakpoint_store(),
                SessionId(1),
                None,
                None,
                DebugAdapterName("fake-adapter".into()),
                SharedTaskContext::default(),
                crate::debugger::session::SessionQuirks::default(),
                None,
                None,
                None,
                cx,
            )
        });

        // Simulate the breakpoint hit during boot: shortly after this control
        // op starts waiting, the session finishes booting with the thread
        // already stopped.
        cx.update(|cx| {
            cx.spawn({
                let session = session.clone();
                let client = client.clone();
                let worktree = worktree.clone();
                let executor = executor.clone();
                async move |cx| {
                    cx.background_executor()
                        .timer(Duration::from_millis(5))
                        .await;
                    session.update(cx, |this, _| {
                        this.set_running_with_stopped_thread_for_test(
                            ThreadId(1),
                            client,
                            worktree,
                            executor,
                        );
                    });
                }
            })
            .detach();
        });

        let result = cx
            .update(|cx| {
                cx.spawn(async move |cx| {
                    wait_for_boot_stop(&session, ThreadId(1), Duration::from_secs(1), cx).await
                })
            })
            .await;

        assert_eq!(result.unwrap(), Some(ThreadId(1)));
    }
}
