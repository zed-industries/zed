//! `cedian_debug`: the agent drives the debug session open in Zed (S1,
//! ARCHITECTURE §22). Breakpoints go into Zed's own `BreakpointStore` (the
//! editor gutter's), a launch goes through `Workspace::start_debug_session`
//! (the Debug panel shows it), and run control and reads go to the same
//! `Session` the person moves. cedian adds no DAP client and no second
//! debugger runtime; OMP's own `debug` stays on beside it (ADR-0048 staging).

use crate::context::{self, Call};
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
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use task::{SharedTaskContext, TaskContext};
use workspace::Workspace;

pub const TOOL: &str = "cedian_debug";

/// How long `continue`, `step_over` and `start` wait for the next stop by
/// default, and at most: under the app's 20 s bound.
const WAIT_DEFAULT_MS: u64 = 10_000;
const WAIT_MAX_MS: u64 = 15_000;
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

/// The tool OMP calls. Each call is parsed here, on OMP's reader thread,
/// and crosses to the GPUI thread over `calls`.
pub(crate) fn tool(calls: UnboundedSender<DebugCall>) -> HostTool {
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
        let reply = context::cross(&calls, op, &what, || ctx.is_cancelled())?;
        let text = serde_json::to_string(&reply).map_err(|e| e.to_string())?;
        Ok(HostToolOutput::Text(text))
    })
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
}

/// Answer one op on the GPUI thread.
pub fn answer(op: &Op, debugger: &Debugger, cx: &mut App) -> Task<Result<Reply, String>> {
    let op = op.clone();
    let root = folder(&debugger.project, cx);
    match op {
        Op::Sessions => Task::ready(Ok(Reply::Sessions {
            sessions: sessions(&debugger.project, cx),
        })),
        Op::Start {
            scenario,
            wait_stopped_ms,
        } => start(debugger.clone(), scenario, wait(wait_stopped_ms), root, cx),
        Op::SetBreakpoint {
            path,
            line,
            condition,
        } => breakpoint(debugger, path, line, Some(condition), cx),
        Op::RemoveBreakpoint { path, line } => breakpoint(debugger, path, line, None, cx),
        Op::Continue {
            at,
            wait_stopped_ms,
        } => run(debugger, at, Run::Continue, wait(wait_stopped_ms), root, cx),
        Op::StepOver {
            at,
            wait_stopped_ms,
        } => run(debugger, at, Run::StepOver, wait(wait_stopped_ms), root, cx),
        Op::Pause { at } => run(debugger, at, Run::Pause, wait(None), root, cx),
        Op::Threads { at } => read(debugger, at, cx, |session, _, cx| {
            let threads = threads(session, cx);
            (!threads.is_empty()).then(|| Reply::Threads {
                session: session.session_id().0,
                threads,
            })
        }),
        Op::Stack { at } => read(debugger, at, cx, move |session, thread, cx| {
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
        Op::Variables { at, frame } => read(debugger, at, cx, move |session, thread, cx| {
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
        }),
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

/// Re-run `check` on every change of `session` until it answers, or `bound`
/// passes (then `None`). Session reads are lazy caches: the first read
/// sends the request and answers empty, and the answer arrives as a notify.
async fn until<T: 'static>(
    session: &Entity<Session>,
    bound: Duration,
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
    let timeout = cx.background_executor().timer(bound).fuse();
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
    bound: Duration,
    cx: &mut AsyncApp,
) -> Result<Reply, String> {
    let id = session.read_with(cx, |s, _| s.session_id().0);
    let stop = until(&session, bound, cx, |s, _| {
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
    let reply = until(&session, bound, cx, |s, cx| {
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
    bound: Duration,
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
        let reply = next_stop(session, root, stopped, bound, cx).await;
        drop(subscription);
        reply
    })
}

/// Run `check` against the picked session until it answers (bounded by
/// the app's own timeout).
fn read(
    debugger: &Debugger,
    at: At,
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
        until(&session, Duration::from_millis(WAIT_MAX_MS), cx, |s, cx| {
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
        let set = store.read_with(cx, |s, cx| {
            s.breakpoint_at_row(&abs, line - 1, cx).is_some()
        });
        Ok(Reply::Breakpoint { path, line, set })
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
    let wanted = condition.is_some();
    if existing.is_some() == wanted {
        // Already as asked: setting twice or removing a missing one is a no-op.
        return Ok(());
    }
    let position = match existing {
        Some((_, existing)) => existing,
        None => {
            let mut bp = Breakpoint::new_standard();
            bp.condition = condition.flatten().map(Into::into);
            BreakpointWithPosition {
                position: snapshot.anchor_before(Point::new(row, 0)),
                bp,
            }
        }
    };
    let buffer = buffer.clone();
    store.update(cx, |store, cx| {
        store.toggle_breakpoint(buffer, position, BreakpointEditAction::Toggle, cx)
    });
    Ok(())
}

/// Launch the debug.json scenario labelled `label` the way the New Process
/// modal does, so the session opens in the Debug panel, and wait for its
/// first stop. An unknown label is refused before any adapter starts.
fn start(
    debugger: Debugger,
    label: String,
    bound: Duration,
    root: Option<Arc<Path>>,
    cx: &mut App,
) -> Task<Result<Reply, String>> {
    let project = debugger.project.clone();
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
    cx.spawn(async move |cx| {
        let (_, scenarios) = listed.await;
        let Some(scenario) = scenarios
            .iter()
            .map(|(_, s)| s)
            .find(|s| s.label.as_ref() == label)
            .cloned()
        else {
            let known: Vec<&str> = scenarios.iter().map(|(_, s)| s.label.as_ref()).collect();
            return Err(format!(
                "denied: no debug.json scenario labelled {label:?} in this worktree (known: {})",
                if known.is_empty() {
                    "none".to_string()
                } else {
                    known.join(", ")
                }
            ));
        };
        let (Some(workspace), Some(window)) = (debugger.workspace.clone(), debugger.window) else {
            return Err("no workspace window to open the Debug panel in".to_string());
        };
        let (new_tx, mut new_rx) = mpsc::unbounded::<Entity<Session>>();
        let _observe = cx.update(|cx| {
            cx.observe_new::<Session>(move |_, _, cx| {
                new_tx.unbounded_send(cx.entity()).ok();
            })
        });
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
        let timeout = cx.background_executor().timer(bound).fuse();
        futures::pin_mut!(timeout);
        let session = futures::select_biased! {
            session = new_rx.next().fuse() => session,
            _ = timeout => None,
        }
        .ok_or_else(|| format!("the debug session for {label:?} did not start"))?;
        let (stopped, subscription) = cx.update(|cx| watch_stop(&session, cx));
        let running = until(&session, bound, cx, |s, _| {
            (s.adapter_client().is_some() || s.is_terminated()).then_some(())
        })
        .await;
        if running.is_none() {
            return Err(format!("the debug adapter for {label:?} did not start"));
        }
        let reply = next_stop(session, root, stopped, bound, cx).await;
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

    #[test]
    fn long_values_are_cut() {
        assert_eq!(cut("x".repeat(600)).chars().count(), MAX_VALUE + 1);
        assert_eq!(cut("42".into()), "42");
    }
}
