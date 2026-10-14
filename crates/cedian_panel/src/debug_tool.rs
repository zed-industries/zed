//! `cedian_debug`: the agent drives the debug session open in Zed (S1,
//! ARCHITECTURE §22). Breakpoints go into Zed's own `BreakpointStore` (the
//! editor gutter's), a launch goes through `Workspace::start_debug_session`
//! (the Debug panel shows it), and run control and reads go to the same
//! `Session` the person moves. cedian adds no DAP client and no second
//! debugger runtime; OMP's own `debug` stays on beside it (ADR-0048 staging).

use crate::context::{self, Call};
use collections::HashSet;
use dap::client::SessionId;
use futures::channel::mpsc::{self, UnboundedSender};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AnyWindowHandle, App, AsyncApp, Entity, Task, WeakEntity};
use language::Point;
use omp_rpc::{HostTool, HostToolOutput};
use project::Project;
use project::debugger::breakpoint_store::{
    Breakpoint, BreakpointEditAction, BreakpointStore, BreakpointWithPosition,
};
use project::debugger::session::{Session, SessionEvent, ThreadId};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};
use task::{DebugScenario, SharedTaskContext, TaskContext};
use workspace::Workspace;

pub const TOOL: &str = "cedian_debug";

/// How long `continue`, `step_over` and `start` wait for the next stop by
/// default, and at most.
const WAIT_DEFAULT_MS: u64 = 10_000;
const WAIT_MAX_MS: u64 = 15_000;
/// One op's whole time: every wait inside it draws on this one budget, so
/// the agent gets the op's own answer before the app's 20 s bound
/// ([`context::APP_TIMEOUT`]) answers a timeout instead.
const OP_BUDGET: Duration = Duration::from_secs(18);
/// How long the person has to answer cedian's Ask before it is a denial:
/// the dialog lease, plus a margin so the panel's expiry answers first.
const ASK_WAIT: Duration = Duration::from_secs(5 * 60 + 30);
const MAX_FRAMES: usize = 32;
const MAX_VARS: usize = 100;
const MAX_VALUE: usize = 512;

const DESCRIPTION: &str = "cedian's own host tool. Drives the debugger session open in the cedian \
IDE: the person sees the same session, breakpoints and stack, and either of you can continue it. \
Use it instead of `debug` when the program can be launched from a debug.json scenario or a session \
is already running in the IDE. op=sessions lists sessions; op=start {scenario} launches a \
debug.json scenario by label and waits for its first stop; op=set_breakpoint / remove_breakpoint \
{path, line, condition?} (worktree-relative path, 1-based line); op=continue / step_over \
{wait_stopped_ms?} run until the next stop and answer the stopped frame and its locals; op=pause, \
op=threads, op=stack {thread?}, op=variables {frame?}, op=stop. `session` defaults to the newest \
running session, `thread` to the stopped one.";

/// What the agent asked, parsed at the tool boundary.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Op {
    Sessions,
    Start {
        scenario: String,
        wait_stopped_ms: Option<u64>,
    },
    SetBreakpoint {
        path: String,
        line: u32,
        condition: Option<String>,
    },
    RemoveBreakpoint {
        path: String,
        line: u32,
    },
    Continue {
        #[serde(flatten)]
        at: At,
        wait_stopped_ms: Option<u64>,
    },
    StepOver {
        #[serde(flatten)]
        at: At,
        wait_stopped_ms: Option<u64>,
    },
    Pause {
        #[serde(flatten)]
        at: At,
    },
    Threads {
        #[serde(flatten)]
        at: At,
    },
    Stack {
        #[serde(flatten)]
        at: At,
    },
    Variables {
        #[serde(flatten)]
        at: At,
        frame: Option<u64>,
    },
    Stop {
        #[serde(flatten)]
        at: At,
    },
}

/// Which session and thread an op is about; `None` picks the default.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct At {
    pub session: Option<u32>,
    pub thread: Option<i64>,
}

/// What the tool answers, as JSON text.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "reply", rename_all = "snake_case")]
pub enum Reply {
    Sessions {
        sessions: Vec<SessionInfo>,
    },
    Breakpoint {
        path: String,
        line: u32,
        set: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        condition: Option<String>,
    },
    Stopped {
        session: u32,
        thread: i64,
        top: Option<Frame>,
        locals: Vec<Var>,
    },
    Running {
        session: u32,
    },
    Exited {
        session: u32,
    },
    Threads {
        session: u32,
        threads: Vec<ThreadInfo>,
    },
    Stack {
        session: u32,
        thread: i64,
        frames: Vec<Frame>,
    },
    Variables {
        session: u32,
        frame: u64,
        scopes: Vec<ScopeVars>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SessionInfo {
    pub id: u32,
    pub label: Option<String>,
    pub adapter: String,
    pub state: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ThreadInfo {
    pub id: i64,
    pub name: String,
    pub stopped: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Frame {
    pub id: u64,
    pub name: String,
    pub path: Option<String>,
    pub line: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScopeVars {
    pub scope: String,
    pub vars: Vec<Var>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Var {
    pub name: String,
    pub value: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

pub(crate) type DebugCall = Call<Op, Reply>;

/// cedian's gate for one op (ADR-0073 decision 5, ADR-0012): reads and
/// breakpoints are allowed, `start` and run control on a session the
/// person started are asked, an unknown scenario or a path outside the
/// folder is denied before anything runs.
#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    Allow,
    /// The question the person sees.
    Ask(String),
    /// Why, as the agent reads it.
    Deny(String),
}

/// The gate's verdict as it crosses back to OMP's thread: an Ask carries
/// the person's answer to come.
pub(crate) enum Verdict {
    Allow,
    Deny(String),
    Ask(std::sync::mpsc::Receiver<bool>),
}

/// The verdict, and the op that runs if allowed: the one judged, pinned to
/// the session the gate resolved.
pub(crate) type GateCall = Call<Op, (Verdict, Op)>;

/// The tool OMP calls. Each call is parsed here, on OMP's reader thread,
/// crosses to the GPUI thread over `gates` for cedian's verdict, waits for
/// the person when it is an Ask, and then crosses over `calls` to run.
pub(crate) fn tool(
    gates: UnboundedSender<GateCall>,
    calls: UnboundedSender<DebugCall>,
) -> HostTool {
    let parameters = json!({
        "type": "object",
        "properties": {
            "op": {"type": "string", "enum": [
                "sessions", "start", "set_breakpoint", "remove_breakpoint", "continue",
                "pause", "step_over", "threads", "stack", "variables", "stop"
            ]},
            "session": {"type": "integer", "description": "session id from op=sessions or op=start; default: the newest running session"},
            "scenario": {"type": "string", "description": "op=start: label of a debug.json scenario in this worktree"},
            "path": {"type": "string", "description": "worktree-relative file (set/remove_breakpoint)"},
            "line": {"type": "integer", "description": "1-based"},
            "condition": {"type": "string"},
            "thread": {"type": "integer"},
            "frame": {"type": "integer", "description": "op=variables: frame id from op=stack; default: the top frame"},
            "wait_stopped_ms": {"type": "integer", "description": "start/continue/step_over: wait up to this for the next stop, max 15000, default 10000"}
        },
        "required": ["op"]
    });
    let Value::Object(parameters) = parameters else {
        unreachable!("the schema is an object")
    };
    HostTool::new(TOOL, DESCRIPTION, parameters, move |args, ctx| {
        let op = parse(args)?;
        let what = format!("{TOOL} {}", op_name(&op));
        let (verdict, op) = context::cross(&gates, op, &what, || ctx.is_cancelled())?;
        match verdict {
            Verdict::Allow => {}
            Verdict::Deny(why) => return Err(format!("denied: {why}").into()),
            Verdict::Ask(answer) => {
                if !person_allows(&answer, || ctx.is_cancelled()) {
                    return Err(format!("denied: the person did not allow {what}").into());
                }
            }
        }
        let reply = context::cross(&calls, op, &what, || ctx.is_cancelled())?;
        let text = serde_json::to_string(&reply).map_err(|e| e.to_string())?;
        Ok(HostToolOutput::Text(text))
    })
}

/// Block OMP's thread until the person answers cedian's Ask. No answer in
/// [`ASK_WAIT`], a closed dialog or a cancel is a denial (ADR-0013).
fn person_allows(
    answer: &std::sync::mpsc::Receiver<bool>,
    is_cancelled: impl Fn() -> bool,
) -> bool {
    let deadline = Instant::now() + ASK_WAIT;
    loop {
        match answer.recv_timeout(Duration::from_millis(50)) {
            Ok(allow) => return allow,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return false,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if is_cancelled() || Instant::now() >= deadline {
                    return false;
                }
            }
        }
    }
}

pub fn parse(args: Map<String, Value>) -> Result<Op, String> {
    serde_json::from_value(Value::Object(args)).map_err(|e| format!("{TOOL}: {e}"))
}

pub(crate) fn op_name(op: &Op) -> &'static str {
    match op {
        Op::Sessions => "sessions",
        Op::Start { .. } => "start",
        Op::SetBreakpoint { .. } => "set_breakpoint",
        Op::RemoveBreakpoint { .. } => "remove_breakpoint",
        Op::Continue { .. } => "continue",
        Op::StepOver { .. } => "step_over",
        Op::Pause { .. } => "pause",
        Op::Threads { .. } => "threads",
        Op::Stack { .. } => "stack",
        Op::Variables { .. } => "variables",
        Op::Stop { .. } => "stop",
    }
}

/// What the GPUI side needs: the project, and the workspace and its window
/// that a launch goes through.
#[derive(Clone)]
pub struct Debugger {
    pub project: Entity<Project>,
    pub workspace: Option<WeakEntity<Workspace>>,
    pub window: Option<AnyWindowHandle>,
    /// The sessions the agent's `start` launched; any other is the person's.
    pub agent_sessions: Rc<RefCell<HashSet<u32>>>,
}

/// cedian's gate for `op` (see [`Gate`]), with the op that runs if it
/// passes: run control is pinned to the session the gate judged, so an
/// approved Ask acts on the session its dialog named, whatever starts
/// while the person decides.
pub fn gate(op: &Op, debugger: &Debugger, cx: &mut App) -> Task<Result<(Gate, Op), String>> {
    let mut pinned = op.clone();
    if let Some(at) = run_control(&mut pinned)
        && let Ok(session) = pick(&debugger.project, at, cx)
    {
        let session = session.read(cx);
        let id = session.session_id().0;
        at.session = Some(id);
        let gate = if debugger.agent_sessions.borrow().contains(&id) {
            Gate::Allow
        } else {
            Gate::Ask(format!(
                "The agent asks to {} your debug session {id}{}",
                op_name(op).replace('_', " "),
                session
                    .label()
                    .map(|l| format!(" (\"{l}\")"))
                    .unwrap_or_default()
            ))
        };
        return Task::ready(Ok((gate, pinned)));
    }
    let gate = verdict(op, debugger, cx);
    cx.spawn(async move |_| Ok((gate.await?, pinned)))
}

/// The session and thread a run-control op acts on.
fn run_control(op: &mut Op) -> Option<&mut At> {
    match op {
        Op::Continue { at, .. } | Op::StepOver { at, .. } | Op::Pause { at } | Op::Stop { at } => {
            Some(at)
        }
        _ => None,
    }
}

fn verdict(op: &Op, debugger: &Debugger, cx: &mut App) -> Task<Result<Gate, String>> {
    match op {
        // Run control with no session to act on: the op itself answers why.
        Op::Sessions
        | Op::Threads { .. }
        | Op::Stack { .. }
        | Op::Variables { .. }
        | Op::Continue { .. }
        | Op::StepOver { .. }
        | Op::Pause { .. }
        | Op::Stop { .. } => Task::ready(Ok(Gate::Allow)),
        Op::SetBreakpoint { path, .. } | Op::RemoveBreakpoint { path, .. } => {
            let project = debugger.project.clone();
            let Some(host) = context::host(debugger.workspace.as_ref(), &project, cx) else {
                return Task::ready(Ok(Gate::Deny("no folder is open".to_string())));
            };
            let inside = context::inside(&host, &project, path, cx);
            cx.spawn(async move |_| {
                Ok(match inside.await {
                    Ok(_) => Gate::Allow,
                    Err(why) => Gate::Deny(why),
                })
            })
        }
        Op::Start { scenario, .. } => {
            let found = scenario_for(&debugger.project, scenario.clone(), cx);
            cx.spawn(async move |_| {
                Ok(match found.await {
                    Ok((_, scenario)) => Gate::Ask(format!(
                        "The agent asks to start the debug scenario \"{}\" ({})",
                        scenario.label, scenario.adapter
                    )),
                    Err(why) => Gate::Deny(why),
                })
            })
        }
    }
}

/// The deadline one op's waits share (see [`OP_BUDGET`]).
#[derive(Clone, Copy, Debug)]
struct Deadline(Instant);

impl Deadline {
    fn new(cx: &App) -> Self {
        Self(cx.background_executor().now() + OP_BUDGET)
    }

    /// A timer for one wait: `cap` at most, and never past the deadline.
    fn timer(&self, cap: Option<Duration>, cx: &AsyncApp) -> Task<()> {
        let executor = cx.background_executor();
        let left = self.0.saturating_duration_since(executor.now());
        executor.timer(cap.map_or(left, |cap| cap.min(left)))
    }
}

/// Answer one op on the GPUI thread.
pub fn answer(op: &Op, debugger: &Debugger, cx: &mut App) -> Task<Result<Reply, String>> {
    let op = op.clone();
    let root = folder(&debugger.project, cx);
    let deadline = Deadline::new(cx);
    match op {
        Op::Sessions => Task::ready(Ok(Reply::Sessions {
            sessions: sessions(&debugger.project, cx),
        })),
        Op::Start {
            scenario,
            wait_stopped_ms,
        } => start(
            debugger.clone(),
            scenario,
            deadline,
            wait(wait_stopped_ms),
            root,
            cx,
        ),
        Op::SetBreakpoint {
            path,
            line,
            condition,
        } => breakpoint(debugger, path, line, Some(condition), cx),
        Op::RemoveBreakpoint { path, line } => breakpoint(debugger, path, line, None, cx),
        Op::Continue {
            at,
            wait_stopped_ms,
        } => run(
            debugger,
            at,
            Run::Continue,
            deadline,
            wait(wait_stopped_ms),
            root,
            cx,
        ),
        Op::StepOver {
            at,
            wait_stopped_ms,
        } => run(
            debugger,
            at,
            Run::StepOver,
            deadline,
            wait(wait_stopped_ms),
            root,
            cx,
        ),
        Op::Pause { at } => run(debugger, at, Run::Pause, deadline, wait(None), root, cx),
        Op::Threads { at } => read(debugger, at, deadline, cx, |session, _, cx| {
            let threads = threads(session, cx);
            (!threads.is_empty()).then(|| Reply::Threads {
                session: session.session_id().0,
                threads,
            })
        }),
        Op::Stack { at } => read(debugger, at, deadline, cx, move |session, thread, cx| {
            let thread = thread?;
            let frames = session.stack_frames(thread, cx).ok()?;
            (!frames.is_empty()).then(|| Reply::Stack {
                session: session.session_id().0,
                thread: thread.0,
                frames: frames
                    .iter()
                    .take(MAX_FRAMES)
                    .map(|f| frame(&f.dap, root.as_deref()))
                    .collect(),
            })
        }),
        Op::Variables { at, frame } => {
            read(debugger, at, deadline, cx, move |session, thread, cx| {
                let thread = thread?;
                let frame = match frame {
                    Some(frame) => frame,
                    None => session.stack_frames(thread, cx).ok()?.first()?.dap.id,
                };
                let scopes = session.scopes(frame, cx).to_vec();
                if scopes.is_empty() {
                    return None;
                }
                let scopes = scopes
                    .iter()
                    .map(|scope| {
                        Some(ScopeVars {
                            scope: scope.name.clone(),
                            vars: vars(session, scope.variables_reference, cx)?,
                        })
                    })
                    .collect::<Option<Vec<_>>>()?;
                Some(Reply::Variables {
                    session: session.session_id().0,
                    frame,
                    scopes,
                })
            })
        }
        Op::Stop { at } => {
            let session = match pick(&debugger.project, &at, cx) {
                Ok(session) => session,
                Err(e) => return Task::ready(Err(e)),
            };
            let id = session.read(cx).session_id().0;
            let done = session.update(cx, |s, cx| s.shutdown(cx));
            cx.spawn(async move |_| {
                done.await;
                Ok(Reply::Exited { session: id })
            })
        }
    }
}

fn wait(ms: Option<u64>) -> Duration {
    Duration::from_millis(ms.unwrap_or(WAIT_DEFAULT_MS).min(WAIT_MAX_MS))
}

fn state(session: &Session) -> &'static str {
    if session.is_terminated() {
        "exited"
    } else if session.any_stopped_thread() {
        "stopped"
    } else {
        "running"
    }
}

fn sessions(project: &Entity<Project>, cx: &App) -> Vec<SessionInfo> {
    let mut sessions: Vec<SessionInfo> = project
        .read(cx)
        .dap_store()
        .read(cx)
        .sessions()
        .map(|s| {
            let s = s.read(cx);
            SessionInfo {
                id: s.session_id().0,
                label: s.label().map(|l| l.to_string()),
                adapter: s.adapter().to_string(),
                state: state(s),
            }
        })
        .collect();
    sessions.sort_by_key(|s| s.id);
    sessions
}

/// The session `at` names, or the newest one still running.
fn pick(project: &Entity<Project>, at: &At, cx: &App) -> Result<Entity<Session>, String> {
    let store = project.read(cx).dap_store().read(cx);
    match at.session {
        Some(id) => store
            .session_by_id(SessionId(id))
            .ok_or_else(|| format!("no debug session {id}; op=sessions lists them")),
        None => store
            .sessions()
            .filter(|s| !s.read(cx).is_terminated())
            .max_by_key(|s| s.read(cx).session_id().0)
            .cloned()
            .ok_or_else(|| {
                "no debug session is running; launch one with op=start {scenario}".to_string()
            }),
    }
}

/// A frame as the agent reads it: a path inside the folder is
/// worktree-relative, any other as the adapter gave it.
fn frame(f: &dap::StackFrame, root: Option<&Path>) -> Frame {
    let path = f.source.as_ref().and_then(|s| s.path.as_deref()).map(|p| {
        root.and_then(|root| Path::new(p).strip_prefix(root).ok())
            .map_or_else(|| p.to_string(), |rel| rel.to_string_lossy().into_owned())
    });
    Frame {
        id: f.id,
        name: f.name.clone(),
        path,
        line: f.line,
    }
}

/// The folder's root, which frame paths are made relative to.
fn folder(project: &Entity<Project>, cx: &App) -> Option<Arc<Path>> {
    let tree = project.read(cx).visible_worktrees(cx).next()?;
    Some(tree.read(cx).abs_path())
}

/// The threads; empty until the adapter has answered.
fn threads(session: &mut Session, cx: &mut gpui::Context<Session>) -> Vec<ThreadInfo> {
    session
        .threads(cx)
        .into_iter()
        .map(|(t, status)| ThreadInfo {
            id: t.id,
            name: t.name,
            stopped: status == project::debugger::session::ThreadStatus::Stopped,
        })
        .collect()
}

/// The thread an op is about: the one asked for, else the stopped one, else
/// the first.
fn thread_of(
    session: &mut Session,
    asked: Option<i64>,
    cx: &mut gpui::Context<Session>,
) -> Option<ThreadId> {
    if let Some(id) = asked {
        return Some(ThreadId(id));
    }
    let threads = threads(session, cx);
    threads
        .iter()
        .find(|t| t.stopped)
        .or(threads.first())
        .map(|t| ThreadId(t.id))
}

fn vars(
    session: &mut Session,
    reference: u64,
    cx: &mut gpui::Context<Session>,
) -> Option<Vec<Var>> {
    let vars = session.variables(reference, cx);
    (!vars.is_empty()).then(|| {
        vars.into_iter()
            .take(MAX_VARS)
            .map(|v| Var {
                name: v.name,
                value: cut(v.value),
                type_: v.type_,
            })
            .collect()
    })
}

fn cut(mut value: String) -> String {
    if value.len() > MAX_VALUE {
        let mut end = MAX_VALUE;
        while !value.is_char_boundary(end) {
            end -= 1;
        }
        value.truncate(end);
        value.push('…');
    }
    value
}

/// Re-run `check` on every change of `session` until it answers, or `cap`
/// or the op's deadline passes (then `None`). Session reads are lazy
/// caches: the first read sends the request and answers empty, and the
/// answer arrives as a notify.
async fn until<T: 'static>(
    session: &Entity<Session>,
    deadline: Deadline,
    cap: Option<Duration>,
    cx: &mut AsyncApp,
    mut check: impl FnMut(&mut Session, &mut gpui::Context<Session>) -> Option<T>,
) -> Option<T> {
    let (tx, mut changed) = mpsc::unbounded::<()>();
    let events = tx.clone();
    let _subscriptions = cx.update(|cx| {
        (
            cx.observe(session, move |_, _| {
                tx.unbounded_send(()).ok();
            }),
            cx.subscribe(session, move |_, _: &SessionEvent, _| {
                events.unbounded_send(()).ok();
            }),
        )
    });
    let timeout = deadline.timer(cap, cx).fuse();
    futures::pin_mut!(timeout);
    loop {
        if let Some(answer) = session.update(cx, |s, cx| check(s, cx)) {
            return Some(answer);
        }
        futures::select_biased! {
            _ = changed.next() => {}
            _ = timeout => return None,
        }
    }
}

/// Wait for the session's next stop (or its end), then read the stopped
/// frame and its locals.
async fn next_stop(
    session: Entity<Session>,
    root: Option<Arc<Path>>,
    stopped: Rc<Cell<Option<Option<ThreadId>>>>,
    deadline: Deadline,
    stop_wait: Duration,
    cx: &mut AsyncApp,
) -> Result<Reply, String> {
    let id = session.read_with(cx, |s, _| s.session_id().0);
    let stop = until(&session, deadline, Some(stop_wait), cx, |s, _| {
        if s.is_terminated() {
            return Some(None);
        }
        stopped.get().map(Some)
    })
    .await;
    let thread = match stop {
        None => return Ok(Reply::Running { session: id }),
        Some(None) => return Ok(Reply::Exited { session: id }),
        Some(Some(thread)) => thread,
    };
    // What is known so far, answered if the adapter is slow to fill in
    // the frame or its locals.
    let mut known = Reply::Stopped {
        session: id,
        thread: thread.map_or(0, |t| t.0),
        top: None,
        locals: Vec::new(),
    };
    let reply = until(&session, deadline, None, cx, |s, cx| {
        if s.is_terminated() {
            return Some(Reply::Exited { session: id });
        }
        let thread = match thread {
            Some(t) => t,
            None => thread_of(s, None, cx)?,
        };
        // Threads first: the session asks for frames only of known threads.
        threads(s, cx);
        let frames = s.stack_frames(thread, cx).ok()?;
        let top = frames.first()?.dap.clone();
        known = Reply::Stopped {
            session: id,
            thread: thread.0,
            top: Some(frame(&top, root.as_deref())),
            locals: Vec::new(),
        };
        let scopes = s.scopes(top.id, cx).to_vec();
        let scope = scopes.iter().find(|scope| !scope.expensive)?;
        let locals = vars(s, scope.variables_reference, cx)?;
        Some(Reply::Stopped {
            session: id,
            thread: thread.0,
            top: Some(frame(&top, root.as_deref())),
            locals,
        })
    })
    .await;
    Ok(reply.unwrap_or(known))
}

/// Note the session's next `Stopped` event from now on.
fn watch_stop(
    session: &Entity<Session>,
    cx: &mut App,
) -> (Rc<Cell<Option<Option<ThreadId>>>>, gpui::Subscription) {
    let stopped = Rc::new(Cell::new(None));
    let seen = stopped.clone();
    let subscription = cx.subscribe(session, move |_, event: &SessionEvent, _| {
        if let SessionEvent::Stopped(thread) = event {
            seen.set(Some(*thread));
        }
    });
    (stopped, subscription)
}

#[derive(Clone, Copy)]
enum Run {
    Continue,
    StepOver,
    Pause,
}

fn run(
    debugger: &Debugger,
    at: At,
    run: Run,
    deadline: Deadline,
    stop_wait: Duration,
    root: Option<Arc<Path>>,
    cx: &mut App,
) -> Task<Result<Reply, String>> {
    let session = match pick(&debugger.project, &at, cx) {
        Ok(session) => session,
        Err(e) => return Task::ready(Err(e)),
    };
    let thread = session.update(cx, |s, cx| thread_of(s, at.thread, cx));
    let Some(thread) = thread else {
        // Threads are not known yet: ask, then the agent retries.
        return Task::ready(Err(
            "the session has no threads yet; try again or op=threads".to_string(),
        ));
    };
    let (stopped, subscription) = watch_stop(&session, cx);
    session.update(cx, |s, cx| match run {
        Run::Continue => s.continue_program(thread, cx),
        Run::StepOver => s.step_over(thread, dap::SteppingGranularity::Statement, cx),
        Run::Pause => s.pause_thread(thread, cx),
    });
    cx.spawn(async move |cx| {
        let reply = next_stop(session, root, stopped, deadline, stop_wait, cx).await;
        drop(subscription);
        reply
    })
}

/// Run `check` against the picked session until it answers (bounded by
/// the app's own timeout).
fn read(
    debugger: &Debugger,
    at: At,
    deadline: Deadline,
    cx: &mut App,
    check: impl Fn(&mut Session, Option<ThreadId>, &mut gpui::Context<Session>) -> Option<Reply>
    + 'static,
) -> Task<Result<Reply, String>> {
    let session = match pick(&debugger.project, &at, cx) {
        Ok(session) => session,
        Err(e) => return Task::ready(Err(e)),
    };
    let asked = at.thread;
    cx.spawn(async move |cx| {
        until(&session, deadline, None, cx, |s, cx| {
            let thread = thread_of(s, asked, cx);
            check(s, thread, cx)
        })
        .await
        .ok_or_else(|| "the debug adapter did not answer in time".to_string())
    })
}

/// Set (`condition` is `Some`) or remove a breakpoint in Zed's own store,
/// on a file inside the folder (the `cedian://` path rules: never private,
/// never outside, symlinks followed).
fn breakpoint(
    debugger: &Debugger,
    path: String,
    line: u32,
    condition: Option<Option<String>>,
    cx: &mut App,
) -> Task<Result<Reply, String>> {
    if line == 0 {
        return Task::ready(Err("line is 1-based".to_string()));
    }
    let project = debugger.project.clone();
    let Some(host) = context::host(debugger.workspace.as_ref(), &project, cx) else {
        return Task::ready(Err("no folder is open".to_string()));
    };
    let inside = context::inside(&host, &project, &path, cx);
    cx.spawn(async move |cx| {
        let abs = inside.await?;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(&abs, cx))
            .await
            .map_err(|e| e.to_string())?;
        let store = project.read_with(cx, |p, _| p.breakpoint_store());
        cx.update(|cx| set(&store, &buffer, &abs, line - 1, condition, cx))?;
        let now = store.read_with(cx, |s, cx| {
            s.breakpoint_at_row(&abs, line - 1, cx)
                .map(|(_, bp)| bp.bp.condition.map(|c| c.to_string()))
        });
        Ok(Reply::Breakpoint {
            path,
            line,
            set: now.is_some(),
            condition: now.flatten(),
        })
    })
}

fn set(
    store: &Entity<BreakpointStore>,
    buffer: &Entity<language::Buffer>,
    abs: &Path,
    row: u32,
    condition: Option<Option<String>>,
    cx: &mut App,
) -> Result<(), String> {
    let snapshot = buffer.read(cx).snapshot();
    if row > snapshot.max_point().row {
        return Err(format!(
            "line {} is past the end of the file ({} lines)",
            row + 1,
            snapshot.max_point().row + 1
        ));
    }
    let existing = store.read(cx).breakpoint_at_row(abs, row, cx);
    let (position, action) = match (existing, condition) {
        // Removing a missing one is a no-op.
        (None, None) => return Ok(()),
        (Some((_, existing)), None) => (existing, BreakpointEditAction::Toggle),
        (None, Some(condition)) => {
            let mut bp = Breakpoint::new_standard();
            bp.condition = condition.map(Into::into);
            let position = BreakpointWithPosition {
                position: snapshot.anchor_before(Point::new(row, 0)),
                bp,
            };
            (position, BreakpointEditAction::Toggle)
        }
        // Set again with a new condition: the condition changes.
        (Some((_, existing)), Some(Some(condition)))
            if existing.bp.condition.as_deref() != Some(condition.as_str()) =>
        {
            (
                existing,
                BreakpointEditAction::EditCondition(condition.into()),
            )
        }
        // Already as asked.
        (Some(_), Some(_)) => return Ok(()),
    };
    let buffer = buffer.clone();
    store.update(cx, |store, cx| {
        store.toggle_breakpoint(buffer, position, action, cx)
    });
    Ok(())
}

/// The debug.json scenario labelled `label` in the folder's worktree, as
/// the New Process modal lists it, or why there is none.
fn scenario_for(
    project: &Entity<Project>,
    label: String,
    cx: &mut App,
) -> Task<Result<(project::WorktreeId, DebugScenario), String>> {
    let Some(tree) = project.read(cx).visible_worktrees(cx).next() else {
        return Task::ready(Err("no folder is open".to_string()));
    };
    let tree = tree.read(cx).id();
    let Some(inventory) = project
        .read(cx)
        .task_store()
        .read(cx)
        .task_inventory()
        .cloned()
    else {
        return Task::ready(Err("this project has no task inventory".to_string()));
    };
    let contexts = project::TaskContexts {
        active_worktree_context: Some((tree, TaskContext::default())),
        ..Default::default()
    };
    let listed = inventory.update(cx, |inventory, cx| {
        inventory.list_debug_scenarios(&contexts, Vec::new(), Vec::new(), false, cx)
    });
    cx.spawn(async move |_| {
        let (_, scenarios) = listed.await;
        match scenarios
            .iter()
            .map(|(_, s)| s)
            .find(|s| s.label.as_ref() == label)
        {
            Some(scenario) => Ok((tree, scenario.clone())),
            None => {
                let known: Vec<&str> = scenarios.iter().map(|(_, s)| s.label.as_ref()).collect();
                Err(format!(
                    "no debug.json scenario labelled {label:?} in this worktree (known: {})",
                    if known.is_empty() {
                        "none".to_string()
                    } else {
                        known.join(", ")
                    }
                ))
            }
        }
    })
}

/// Launch the debug.json scenario labelled `label` the way the New Process
/// modal does, so the session opens in the Debug panel, and wait for its
/// first stop. The session is the one this launch made: new since the
/// call, with the scenario's label. An unknown label is refused before any
/// adapter starts.
fn start(
    debugger: Debugger,
    label: String,
    deadline: Deadline,
    stop_wait: Duration,
    root: Option<Arc<Path>>,
    cx: &mut App,
) -> Task<Result<Reply, String>> {
    let project = debugger.project.clone();
    let found = scenario_for(&project, label.clone(), cx);
    cx.spawn(async move |cx| {
        let (tree, scenario) = found.await.map_err(|why| format!("denied: {why}"))?;
        let (Some(workspace), Some(window)) = (debugger.workspace.clone(), debugger.window) else {
            return Err("no workspace window to open the Debug panel in".to_string());
        };
        let ids = |cx: &App| -> Vec<(u32, Option<String>)> {
            project
                .read(cx)
                .dap_store()
                .read(cx)
                .sessions()
                .map(|s| {
                    let s = s.read(cx);
                    (s.session_id().0, s.label().map(|l| l.to_string()))
                })
                .collect()
        };
        let before: HashSet<u32> = cx.update(|cx| ids(cx).into_iter().map(|(id, _)| id).collect());
        let adapter = scenario.adapter.clone();
        window
            .update(cx, |_, window, cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.start_debug_session(
                        scenario,
                        SharedTaskContext::default(),
                        None,
                        Some(tree),
                        window,
                        cx,
                    )
                })
            })
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
        // Zed makes the launch's session inside that call; a session made
        // later (the person's, a child) is never this launch's.
        let session = cx.update(|cx| {
            let id = ids(cx)
                .into_iter()
                .filter(|(id, l)| !before.contains(id) && l.as_deref() == Some(label.as_str()))
                .map(|(id, _)| id)
                .max()?;
            project
                .read(cx)
                .dap_store()
                .read(cx)
                .session_by_id(SessionId(id))
        });
        let Some(session) = session else {
            return Err(format!(
                "the debug session for {label:?} did not start (is the {adapter:?} adapter installed?)"
            ));
        };
        let (stopped, subscription) = cx.update(|cx| {
            debugger
                .agent_sessions
                .borrow_mut()
                .insert(session.read(cx).session_id().0);
            watch_stop(&session, cx)
        });
        let reply = next_stop(session, root, stopped, deadline, stop_wait, cx).await;
        drop(subscription);
        reply
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: Value) -> Map<String, Value> {
        let Value::Object(m) = v else { panic!() };
        m
    }

    #[test]
    fn ops_parse_at_the_boundary() {
        assert_eq!(
            parse(args(
                json!({"op": "set_breakpoint", "path": "src/main.c", "line": 5})
            )),
            Ok(Op::SetBreakpoint {
                path: "src/main.c".into(),
                line: 5,
                condition: None
            })
        );
        assert_eq!(
            parse(args(json!({"op": "continue", "session": 2}))),
            Ok(Op::Continue {
                at: At {
                    session: Some(2),
                    thread: None
                },
                wait_stopped_ms: None
            })
        );
        assert!(parse(args(json!({"op": "debug"}))).is_err());
        assert!(
            parse(args(json!({"op": "start"})))
                .unwrap_err()
                .contains("scenario")
        );
    }

    #[test]
    fn the_tool_never_takes_an_omp_tool_name() {
        assert_ne!(TOOL, "debug");
        assert!(TOOL.starts_with("cedian_"));
    }

    /// F1: the waits inside one op draw on one deadline, so a chain of
    /// them (start: launch, stop, frame, locals) ends before the app's 20 s
    /// bound answers OMP a timeout instead of the op's own reply.
    #[gpui::test]
    async fn chained_waits_end_by_the_ops_deadline(cx: &mut gpui::TestAppContext) {
        assert!(OP_BUDGET < context::APP_TIMEOUT);
        let deadline = cx.update(|cx| Deadline::new(cx));
        let done = Rc::new(Cell::new(false));
        let flag = done.clone();
        cx.spawn(async move |cx| {
            for _ in 0..4 {
                deadline
                    .timer(Some(Duration::from_millis(WAIT_MAX_MS)), &cx)
                    .await;
            }
            flag.set(true);
        })
        .detach();
        let step = Duration::from_millis(500);
        let mut elapsed = Duration::ZERO;
        while elapsed < context::APP_TIMEOUT - step {
            cx.executor().advance_clock(step);
            cx.run_until_parked();
            elapsed += step;
        }
        assert!(done.get(), "four waits outlived the app's bound");
    }

    #[test]
    fn long_values_are_cut() {
        assert_eq!(cut("x".repeat(600)).chars().count(), MAX_VALUE + 1);
        assert_eq!(cut("42".into()), "42");
    }
}
