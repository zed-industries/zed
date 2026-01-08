use super::breakpoint_store::{
    BreakpointStore, BreakpointStoreEvent, BreakpointUpdatedReason, SourceBreakpoint,
};
use super::dap_command::{
    self, Attach, ConfigurationDone, ContinueCommand, DataBreakpointInfoCommand, DisconnectCommand,
    EvaluateCommand, Initialize, Launch, LoadedSourcesCommand, LocalDapCommand, LocationsCommand,
    ModulesCommand, NextCommand, PauseCommand, RestartCommand, RestartStackFrameCommand,
    ScopesCommand, SetDataBreakpointsCommand, SetExceptionBreakpoints, SetVariableValueCommand,
    StackTraceCommand, StepBackCommand, StepCommand, StepInCommand, StepOutCommand,
    TerminateCommand, TerminateThreadsCommand, ThreadsCommand, VariablesCommand,
};
use super::dap_store::DapStore;
use crate::debugger::breakpoint_store::BreakpointSessionState;
use crate::debugger::dap_command::{DataBreakpointContext, ReadMemory};
use crate::debugger::memory::{self, Memory, MemoryIterator, MemoryPageBuilder, PageAddress};
use anyhow::{Context as _, Result, anyhow, bail};
use base64::Engine;
use collections::{HashMap, HashSet, IndexMap};
use dap::adapters::{DebugAdapterBinary, DebugAdapterName};
use dap::messages::Response;
use dap::requests::{Request, RunInTerminal, StartDebugging};
use dap::transport::TcpTransport;
use dap::{
    Capabilities, ContinueArguments, EvaluateArgumentsContext, Module, Source, StackFrameId,
    SteppingGranularity, StoppedEvent, VariableReference,
    client::{DebugAdapterClient, SessionId},
    messages::{Events, Message},
};
use dap::{
    ExceptionBreakpointsFilter, ExceptionFilterOptions, OutputEvent, OutputEventCategory,
    RunInTerminalRequestArguments, StackFramePresentationHint, StartDebuggingRequestArguments,
    StartDebuggingRequestArgumentsRequest, VariablePresentationHint, WriteMemoryArguments,
};
use futures::channel::mpsc::UnboundedSender;
use futures::channel::{mpsc, oneshot};
use futures::io::BufReader;
use futures::{AsyncBufReadExt as _, SinkExt, StreamExt, TryStreamExt};
use futures::{FutureExt, future::Shared};
use gpui::{
    App, AppContext, AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, SharedString,
    Task, WeakEntity,
};
use http_client::HttpClient;
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smol::net::{TcpListener, TcpStream};
use std::any::TypeId;
use std::collections::{BTreeMap, VecDeque};
use std::net::Ipv4Addr;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use std::u64;
use std::{
    any::Any,
    collections::hash_map::Entry,
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};
use task::TaskContext;
use text::{PointUtf16, ToPointUtf16};
use url::Url;
use util::command::new_smol_command;
use util::{ResultExt, debug_panic, maybe};
use worktree::Worktree;

const MAX_TRACKED_OUTPUT_EVENTS: usize = 5000;
const DEBUG_HISTORY_LIMIT: usize = 10;

#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct ThreadId(pub i64);

impl From<i64> for ThreadId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    pub dap: dap::StackFrame,
    pub scopes: Vec<dap::Scope>,
}

impl From<dap::StackFrame> for StackFrame {
    fn from(stack_frame: dap::StackFrame) -> Self {
        Self {
            scopes: vec![],
            dap: stack_frame,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ThreadStatus {
    #[default]
    Running,
    Stopped,
    Stepping,
    Exited,
    Ended,
}

impl ThreadStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ThreadStatus::Running => "Running",
            ThreadStatus::Stopped => "Stopped",
            ThreadStatus::Stepping => "Stepping",
            ThreadStatus::Exited => "Exited",
            ThreadStatus::Ended => "Ended",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Thread {
    dap: dap::Thread,
    stack_frames: Vec<StackFrame>,
    stack_frames_error: Option<SharedString>,
    _has_stopped: bool,
}

impl From<dap::Thread> for Thread {
    fn from(dap: dap::Thread) -> Self {
        Self {
            dap,
            stack_frames: Default::default(),
            stack_frames_error: None,
            _has_stopped: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Watcher {
    pub expression: SharedString,
    pub value: SharedString,
    pub variables_reference: u64,
    pub presentation_hint: Option<VariablePresentationHint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataBreakpointState {
    pub dap: dap::DataBreakpoint,
    pub is_enabled: bool,
    pub context: Arc<DataBreakpointContext>,
}

pub enum SessionState {
    /// Represents a session that is building/initializing
    /// even if a session doesn't have a pre build task this state
    /// is used to run all the async tasks that are required to start the session
    Booting(Option<Task<Result<()>>>),
    Running(RunningMode),
}

#[derive(Clone)]
pub struct RunningMode {
    client: Arc<DebugAdapterClient>,
    binary: DebugAdapterBinary,
    tmp_breakpoint: Option<SourceBreakpoint>,
    worktree: WeakEntity<Worktree>,
    executor: BackgroundExecutor,
    is_started: bool,
    has_ever_stopped: bool,
    messages_tx: UnboundedSender<Message>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SessionQuirks {
    pub compact: bool,
    pub prefer_thread_name: bool,
}

fn client_source(abs_path: &Path) -> dap::Source {
    dap::Source {
        name: abs_path
            .file_name()
            .map(|filename| filename.to_string_lossy().into_owned()),
        path: Some(abs_path.to_string_lossy().into_owned()),
        source_reference: None,
        presentation_hint: None,
        origin: None,
        sources: None,
        adapter_data: None,
        checksums: None,
    }
}

impl RunningMode {
    async fn new(
        session_id: SessionId,
        parent_session: Option<Entity<Session>>,
        worktree: WeakEntity<Worktree>,
        binary: DebugAdapterBinary,
        messages_tx: futures::channel::mpsc::UnboundedSender<Message>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let message_handler = Box::new({
            let messages_tx = messages_tx.clone();
            move |message| {
                messages_tx.unbounded_send(message).ok();
            }
        });

        let client = if let Some(client) =
            parent_session.and_then(|session| cx.update(|cx| session.read(cx).adapter_client()))
        {
            client
                .create_child_connection(session_id, binary.clone(), message_handler, cx)
                .await?
        } else {
            DebugAdapterClient::start(session_id, binary.clone(), message_handler, cx).await?
        };

        Ok(Self {
            client: Arc::new(client),
            worktree,
            tmp_breakpoint: None,
            binary,
            executor: cx.background_executor().clone(),
            is_started: false,
            has_ever_stopped: false,
            messages_tx,
        })
    }

    pub(crate) fn worktree(&self) -> &WeakEntity<Worktree> {
        &self.worktree
    }

    fn unset_breakpoints_from_paths(&self, paths: &Vec<Arc<Path>>, cx: &mut App) -> Task<()> {
        let tasks: Vec<_> = paths
            .iter()
            .map(|path| {
                self.request(dap_command::SetBreakpoints {
                    source: client_source(path),
                    source_modified: None,
                    breakpoints: vec![],
                })
            })
            .collect();

        cx.background_spawn(async move {
            futures::future::join_all(tasks)
                .await
                .iter()
                .for_each(|res| match res {
                    Ok(_) => {}
                    Err(err) => {
                        log::warn!("Set breakpoints request failed: {}", err);
                    }
                });
        })
    }

    fn send_breakpoints_from_path(
        &self,
        abs_path: Arc<Path>,
        reason: BreakpointUpdatedReason,
        breakpoint_store: &Entity<BreakpointStore>,
        cx: &mut App,
    ) -> Task<()> {
        let breakpoints =
            breakpoint_store
                .read(cx)
                .source_breakpoints_from_path(&abs_path, cx)
                .into_iter()
                .filter(|bp| bp.state.is_enabled())
                .chain(self.tmp_breakpoint.iter().filter_map(|breakpoint| {
                    breakpoint.path.eq(&abs_path).then(|| breakpoint.clone())
                }))
                .map(Into::into)
                .collect();

        let raw_breakpoints = breakpoint_store
            .read(cx)
            .breakpoints_from_path(&abs_path)
            .into_iter()
            .filter(|bp| bp.bp.state.is_enabled())
            .collect::<Vec<_>>();

        let task = self.request(dap_command::SetBreakpoints {
            source: client_source(&abs_path),
            source_modified: Some(matches!(reason, BreakpointUpdatedReason::FileSaved)),
            breakpoints,
        });
        let session_id = self.client.id();
        let breakpoint_store = breakpoint_store.downgrade();
        cx.spawn(async move |cx| match cx.background_spawn(task).await {
            Ok(breakpoints) => {
                let breakpoints =
                    breakpoints
                        .into_iter()
                        .zip(raw_breakpoints)
                        .filter_map(|(dap_bp, zed_bp)| {
                            Some((
                                zed_bp,
                                BreakpointSessionState {
                                    id: dap_bp.id?,
                                    verified: dap_bp.verified,
                                },
                            ))
                        });
                breakpoint_store
                    .update(cx, |this, _| {
                        this.mark_breakpoints_verified(session_id, &abs_path, breakpoints);
                    })
                    .ok();
            }
            Err(err) => log::warn!("Set breakpoints request failed for path: {}", err),
        })
    }

    fn send_exception_breakpoints(
        &self,
        filters: Vec<ExceptionBreakpointsFilter>,
        supports_filter_options: bool,
    ) -> Task<Result<Vec<dap::Breakpoint>>> {
        let arg = if supports_filter_options {
            SetExceptionBreakpoints::WithOptions {
                filters: filters
                    .into_iter()
                    .map(|filter| ExceptionFilterOptions {
                        filter_id: filter.filter,
                        condition: None,
                        mode: None,
                    })
                    .collect(),
            }
        } else {
            SetExceptionBreakpoints::Plain {
                filters: filters.into_iter().map(|filter| filter.filter).collect(),
            }
        };
        self.request(arg)
    }

    fn send_source_breakpoints(
        &self,
        ignore_breakpoints: bool,
        breakpoint_store: &Entity<BreakpointStore>,
        cx: &App,
    ) -> Task<HashMap<Arc<Path>, anyhow::Error>> {
        let mut breakpoint_tasks = Vec::new();
        let breakpoints = breakpoint_store.read(cx).all_source_breakpoints(cx);
        let mut raw_breakpoints = breakpoint_store.read_with(cx, |this, _| this.all_breakpoints());
        debug_assert_eq!(raw_breakpoints.len(), breakpoints.len());
        let session_id = self.client.id();
        for (path, breakpoints) in breakpoints {
            let breakpoints = if ignore_breakpoints {
                vec![]
            } else {
                breakpoints
                    .into_iter()
                    .filter(|bp| bp.state.is_enabled())
                    .map(Into::into)
                    .collect()
            };

            let raw_breakpoints = raw_breakpoints
                .remove(&path)
                .unwrap_or_default()
                .into_iter()
                .filter(|bp| bp.bp.state.is_enabled());
            let error_path = path.clone();
            let send_request = self
                .request(dap_command::SetBreakpoints {
                    source: client_source(&path),
                    source_modified: Some(false),
                    breakpoints,
                })
                .map(|result| result.map_err(move |e| (error_path, e)));

            let task = cx.spawn({
                let breakpoint_store = breakpoint_store.downgrade();
                async move |cx| {
                    let breakpoints = cx.background_spawn(send_request).await?;

                    let breakpoints = breakpoints.into_iter().zip(raw_breakpoints).filter_map(
                        |(dap_bp, zed_bp)| {
                            Some((
                                zed_bp,
                                BreakpointSessionState {
                                    id: dap_bp.id?,
                                    verified: dap_bp.verified,
                                },
                            ))
                        },
                    );
                    breakpoint_store
                        .update(cx, |this, _| {
                            this.mark_breakpoints_verified(session_id, &path, breakpoints);
                        })
                        .ok();

                    Ok(())
                }
            });
            breakpoint_tasks.push(task);
        }

        cx.background_spawn(async move {
            futures::future::join_all(breakpoint_tasks)
                .await
                .into_iter()
                .filter_map(Result::err)
                .collect::<HashMap<_, _>>()
        })
    }

    fn initialize_sequence(
        &self,
        capabilities: &Capabilities,
        initialized_rx: oneshot::Receiver<()>,
        dap_store: WeakEntity<DapStore>,
        cx: &mut Context<Session>,
    ) -> Task<Result<()>> {
        let raw = self.binary.request_args.clone();

        // Of relevance: https://github.com/microsoft/vscode/issues/4902#issuecomment-368583522
        let launch = match raw.request {
            dap::StartDebuggingRequestArgumentsRequest::Launch => self.request(Launch {
                raw: raw.configuration,
            }),
            dap::StartDebuggingRequestArgumentsRequest::Attach => self.request(Attach {
                raw: raw.configuration,
            }),
        };

        let configuration_done_supported = ConfigurationDone::is_supported(capabilities);
        // From spec (on initialization sequence):
        // client sends a setExceptionBreakpoints request if one or more exceptionBreakpointFilters have been defined (or if supportsConfigurationDoneRequest is not true)
        //
        // Thus we should send setExceptionBreakpoints even if `exceptionFilters` variable is empty (as long as there were some options in the first place).
        let should_send_exception_breakpoints = capabilities
            .exception_breakpoint_filters
            .as_ref()
            .is_some_and(|filters| !filters.is_empty())
            || !configuration_done_supported;
        let supports_exception_filters = capabilities
            .supports_exception_filter_options
            .unwrap_or_default();
        let this = self.clone();
        let worktree = self.worktree().clone();
        let mut filters = capabilities
            .exception_breakpoint_filters
            .clone()
            .unwrap_or_default();
        let configuration_sequence = cx.spawn({
            async move |session, cx| {
                let adapter_name = session.read_with(cx, |this, _| this.adapter())?;
                let (breakpoint_store, adapter_defaults) =
                    dap_store.read_with(cx, |dap_store, _| {
                        (
                            dap_store.breakpoint_store().clone(),
                            dap_store.adapter_options(&adapter_name),
                        )
                    })?;
                initialized_rx.await?;
                let errors_by_path = cx
                    .update(|cx| this.send_source_breakpoints(false, &breakpoint_store, cx))
                    .await;

                dap_store.update(cx, |_, cx| {
                    let Some(worktree) = worktree.upgrade() else {
                        return;
                    };

                    for (path, error) in &errors_by_path {
                        log::error!("failed to set breakpoints for {path:?}: {error}");
                    }

                    if let Some(failed_path) = errors_by_path.keys().next() {
                        let failed_path = failed_path
                            .strip_prefix(worktree.read(cx).abs_path())
                            .unwrap_or(failed_path)
                            .display();
                        let message = format!(
                            "Failed to set breakpoints for {failed_path}{}",
                            match errors_by_path.len() {
                                0 => unreachable!(),
                                1 => "".into(),
                                2 => " and 1 other path".into(),
                                n => format!(" and {} other paths", n - 1),
                            }
                        );
                        cx.emit(super::dap_store::DapStoreEvent::Notification(message));
                    }
                })?;

                if should_send_exception_breakpoints {
                    _ = session.update(cx, |this, _| {
                        filters.retain(|filter| {
                            let is_enabled = if let Some(defaults) = adapter_defaults.as_ref() {
                                defaults
                                    .exception_breakpoints
                                    .get(&filter.filter)
                                    .map(|options| options.enabled)
                                    .unwrap_or_else(|| filter.default.unwrap_or_default())
                            } else {
                                filter.default.unwrap_or_default()
                            };
                            this.exception_breakpoints
                                .entry(filter.filter.clone())
                                .or_insert_with(|| (filter.clone(), is_enabled));
                            is_enabled
                        });
                    });

                    this.send_exception_breakpoints(filters, supports_exception_filters)
                        .await
                        .ok();
                }

                if configuration_done_supported {
                    this.request(ConfigurationDone {})
                } else {
                    Task::ready(Ok(()))
                }
                .await
            }
        });

        let task = cx.background_spawn(futures::future::try_join(launch, configuration_sequence));

        cx.spawn(async move |this, cx| {
            let result = task.await;

            this.update(cx, |this, cx| {
                if let Some(this) = this.as_running_mut() {
                    this.is_started = true;
                    cx.notify();
                }
            })
            .ok();

            result?;
            anyhow::Ok(())
        })
    }

    fn reconnect_for_ssh(&self, cx: &mut AsyncApp) -> Option<Task<Result<()>>> {
        let client = self.client.clone();
        let messages_tx = self.messages_tx.clone();
        let message_handler = Box::new(move |message| {
            messages_tx.unbounded_send(message).ok();
        });
        if client.should_reconnect_for_ssh() {
            Some(cx.spawn(async move |cx| {
                client.connect(message_handler, cx).await?;
                anyhow::Ok(())
            }))
        } else {
            None
        }
    }

    fn request<R: LocalDapCommand>(&self, request: R) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        let request = Arc::new(request);

        let request_clone = request.clone();
        let connection = self.client.clone();
        self.executor.spawn(async move {
            let args = request_clone.to_dap();
            let response = connection.request::<R::DapRequest>(args).await?;
            request.response_from_dap(response)
        })
    }
}

impl SessionState {
    pub(super) fn request_dap<R: LocalDapCommand>(&self, request: R) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        match self {
            SessionState::Running(debug_adapter_client) => debug_adapter_client.request(request),
            SessionState::Booting(_) => Task::ready(Err(anyhow!(
                "no adapter running to send request: {request:?}"
            ))),
        }
    }

    /// Did this debug session stop at least once?
    pub(crate) fn has_ever_stopped(&self) -> bool {
        match self {
            SessionState::Booting(_) => false,
            SessionState::Running(running_mode) => running_mode.has_ever_stopped,
        }
    }

    fn stopped(&mut self) {
        if let SessionState::Running(running) = self {
            running.has_ever_stopped = true;
        }
    }
}

#[derive(Default)]
struct ThreadStates {
    global_state: Option<ThreadStatus>,
    known_thread_states: IndexMap<ThreadId, ThreadStatus>,
}

impl ThreadStates {
    fn stop_all_threads(&mut self) {
        self.global_state = Some(ThreadStatus::Stopped);
        self.known_thread_states.clear();
    }

    fn exit_all_threads(&mut self) {
        self.global_state = Some(ThreadStatus::Exited);
        self.known_thread_states.clear();
    }

    fn continue_all_threads(&mut self) {
        self.global_state = Some(ThreadStatus::Running);
        self.known_thread_states.clear();
    }

    fn stop_thread(&mut self, thread_id: ThreadId) {
        self.known_thread_states
            .insert(thread_id, ThreadStatus::Stopped);
    }

    fn continue_thread(&mut self, thread_id: ThreadId) {
        self.known_thread_states
            .insert(thread_id, ThreadStatus::Running);
    }

    fn process_step(&mut self, thread_id: ThreadId) {
        self.known_thread_states
            .insert(thread_id, ThreadStatus::Stepping);
    }

    fn thread_status(&self, thread_id: ThreadId) -> ThreadStatus {
        self.thread_state(thread_id)
            .unwrap_or(ThreadStatus::Running)
    }

    fn thread_state(&self, thread_id: ThreadId) -> Option<ThreadStatus> {
        self.known_thread_states
            .get(&thread_id)
            .copied()
            .or(self.global_state)
    }

    fn exit_thread(&mut self, thread_id: ThreadId) {
        self.known_thread_states
            .insert(thread_id, ThreadStatus::Exited);
    }

    fn any_stopped_thread(&self) -> bool {
        self.global_state
            .is_some_and(|state| state == ThreadStatus::Stopped)
            || self
                .known_thread_states
                .values()
                .any(|status| *status == ThreadStatus::Stopped)
    }
}

// TODO(debugger): Wrap dap types with reference counting so the UI doesn't have to clone them on refresh
#[derive(Default)]
pub struct SessionSnapshot {
    threads: IndexMap<ThreadId, Thread>,
    thread_states: ThreadStates,
    variables: HashMap<VariableReference, Vec<dap::Variable>>,
    stack_frames: IndexMap<StackFrameId, StackFrame>,
    locations: HashMap<u64, dap::LocationsResponse>,
    modules: Vec<dap::Module>,
    loaded_sources: Vec<dap::Source>,
}

type IsEnabled = bool;

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputToken(pub usize);
/// Represents a current state of a single debug adapter and provides ways to mutate it.
pub struct Session {
    pub state: SessionState,
    active_snapshot: SessionSnapshot,
    snapshots: VecDeque<SessionSnapshot>,
    selected_snapshot_index: Option<usize>,
    id: SessionId,
    label: Option<SharedString>,
    adapter: DebugAdapterName,
    pub(super) capabilities: Capabilities,
    child_session_ids: HashSet<SessionId>,
    parent_session: Option<Entity<Session>>,
    output_token: OutputToken,
    output: Box<circular_buffer::CircularBuffer<MAX_TRACKED_OUTPUT_EVENTS, dap::OutputEvent>>,
    watchers: HashMap<SharedString, Watcher>,
    is_session_terminated: bool,
    requests: HashMap<TypeId, HashMap<RequestSlot, Shared<Task<Option<()>>>>>,
    pub(crate) breakpoint_store: Entity<BreakpointStore>,
    ignore_breakpoints: bool,
    exception_breakpoints: BTreeMap<String, (ExceptionBreakpointsFilter, IsEnabled)>,
    data_breakpoints: BTreeMap<String, DataBreakpointState>,
    background_tasks: Vec<Task<()>>,
    restart_task: Option<Task<()>>,
    task_context: TaskContext,
    memory: memory::Memory,
    quirks: SessionQuirks,
    remote_client: Option<Entity<RemoteClient>>,
    node_runtime: Option<NodeRuntime>,
    http_client: Option<Arc<dyn HttpClient>>,
    companion_port: Option<u16>,
}

trait CacheableCommand: Any + Send + Sync {
    fn dyn_eq(&self, rhs: &dyn CacheableCommand) -> bool;
    fn dyn_hash(&self, hasher: &mut dyn Hasher);
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T> CacheableCommand for T
where
    T: LocalDapCommand + PartialEq + Eq + Hash,
{
    fn dyn_eq(&self, rhs: &dyn CacheableCommand) -> bool {
        (rhs as &dyn Any).downcast_ref::<Self>() == Some(self)
    }

    fn dyn_hash(&self, mut hasher: &mut dyn Hasher) {
        T::hash(self, &mut hasher);
    }

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

pub(crate) struct RequestSlot(Arc<dyn CacheableCommand>);

impl<T: LocalDapCommand + PartialEq + Eq + Hash> From<T> for RequestSlot {
    fn from(request: T) -> Self {
        Self(Arc::new(request))
    }
}

impl PartialEq for RequestSlot {
    fn eq(&self, other: &Self) -> bool {
        self.0.dyn_eq(other.0.as_ref())
    }
}

impl Eq for RequestSlot {}

impl Hash for RequestSlot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.dyn_hash(state);
        (&*self.0 as &dyn Any).type_id().hash(state)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CompletionsQuery {
    pub query: String,
    pub column: u64,
    pub line: Option<u64>,
    pub frame_id: Option<u64>,
}

impl CompletionsQuery {
    pub fn new(
        buffer: &language::Buffer,
        cursor_position: language::Anchor,
        frame_id: Option<u64>,
    ) -> Self {
        let PointUtf16 { row, column } = cursor_position.to_point_utf16(&buffer.snapshot());
        Self {
            query: buffer.text(),
            column: column as u64,
            frame_id,
            line: Some(row as u64),
        }
    }
}

#[derive(Debug)]
pub enum SessionEvent {
    Modules,
    LoadedSources,
    Stopped(Option<ThreadId>),
    StackTrace,
    Variables,
    Watchers,
    Threads,
    InvalidateInlineValue,
    CapabilitiesLoaded,
    RunInTerminal {
        request: RunInTerminalRequestArguments,
        sender: mpsc::Sender<Result<u32>>,
    },
    DataBreakpointInfo,
    ConsoleOutput,
    HistoricSnapshotSelected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionStateEvent {
    Running,
    Shutdown,
    Restart,
    SpawnChildSession {
        request: StartDebuggingRequestArguments,
    },
}

impl EventEmitter<SessionEvent> for Session {}
impl EventEmitter<SessionStateEvent> for Session {}

// local session will send breakpoint updates to DAP for all new breakpoints
// remote side will only send breakpoint updates when it is a breakpoint created by that peer
// BreakpointStore notifies session on breakpoint changes
impl Session {
    pub(crate) fn new(
        breakpoint_store: Entity<BreakpointStore>,
        session_id: SessionId,
        parent_session: Option<Entity<Session>>,
        label: Option<SharedString>,
        adapter: DebugAdapterName,
        task_context: TaskContext,
        quirks: SessionQuirks,
        remote_client: Option<Entity<RemoteClient>>,
        node_runtime: Option<NodeRuntime>,
        http_client: Option<Arc<dyn HttpClient>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new::<Self>(|cx| {
            cx.subscribe(&breakpoint_store, |this, store, event, cx| match event {
                BreakpointStoreEvent::BreakpointsUpdated(path, reason) => {
                    if let Some(local) = (!this.ignore_breakpoints)
                        .then(|| this.as_running_mut())
                        .flatten()
                    {
                        local
                            .send_breakpoints_from_path(path.clone(), *reason, &store, cx)
                            .detach();
                    };
                }
                BreakpointStoreEvent::BreakpointsCleared(paths) => {
                    if let Some(local) = (!this.ignore_breakpoints)
                        .then(|| this.as_running_mut())
                        .flatten()
                    {
                        local.unset_breakpoints_from_paths(paths, cx).detach();
                    }
                }
                BreakpointStoreEvent::SetDebugLine | BreakpointStoreEvent::ClearDebugLines => {}
            })
            .detach();

            Self {
                state: SessionState::Booting(None),
                snapshots: VecDeque::with_capacity(DEBUG_HISTORY_LIMIT),
                selected_snapshot_index: None,
                active_snapshot: Default::default(),
                id: session_id,
                child_session_ids: HashSet::default(),
                parent_session,
                capabilities: Capabilities::default(),
                watchers: HashMap::default(),
                output_token: OutputToken(0),
                output: circular_buffer::CircularBuffer::boxed(),
                requests: HashMap::default(),
                background_tasks: Vec::default(),
                restart_task: None,
                is_session_terminated: false,
                ignore_breakpoints: false,
                breakpoint_store,
                data_breakpoints: Default::default(),
                exception_breakpoints: Default::default(),
                label,
                adapter,
                task_context,
                memory: memory::Memory::new(),
                quirks,
                remote_client,
                node_runtime,
                http_client,
                companion_port: None,
            }
        })
    }

    pub fn task_context(&self) -> &TaskContext {
        &self.task_context
    }

    pub fn worktree(&self) -> Option<Entity<Worktree>> {
        match &self.state {
            SessionState::Booting(_) => None,
            SessionState::Running(local_mode) => local_mode.worktree.upgrade(),
        }
    }

    pub fn boot(
        &mut self,
        binary: DebugAdapterBinary,
        worktree: Entity<Worktree>,
        dap_store: WeakEntity<DapStore>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let (message_tx, mut message_rx) = futures::channel::mpsc::unbounded();
        let (initialized_tx, initialized_rx) = futures::channel::oneshot::channel();

        let background_tasks = vec![cx.spawn(async move |this: WeakEntity<Session>, cx| {
            let mut initialized_tx = Some(initialized_tx);
            while let Some(message) = message_rx.next().await {
                if let Message::Event(event) = message {
                    if let Events::Initialized(_) = *event {
                        if let Some(tx) = initialized_tx.take() {
                            tx.send(()).ok();
                        }
                    } else {
                        let Ok(_) = this.update(cx, |session, cx| {
                            session.handle_dap_event(event, cx);
                        }) else {
                            break;
                        };
                    }
                } else if let Message::Request(request) = message {
                    let Ok(_) = this.update(cx, |this, cx| {
                        if request.command == StartDebugging::COMMAND {
                            this.handle_start_debugging_request(request, cx)
                                .detach_and_log_err(cx);
                        } else if request.command == RunInTerminal::COMMAND {
                            this.handle_run_in_terminal_request(request, cx)
                                .detach_and_log_err(cx);
                        }
                    }) else {
                        break;
                    };
                }
            }
        })];
        self.background_tasks = background_tasks;
        let id = self.id;
        let parent_session = self.parent_session.clone();

        cx.spawn(async move |this, cx| {
            let mode = RunningMode::new(
                id,
                parent_session,
                worktree.downgrade(),
                binary.clone(),
                message_tx,
                cx,
            )
            .await?;
            this.update(cx, |this, cx| {
                match &mut this.state {
                    SessionState::Booting(task) if task.is_some() => {
                        task.take().unwrap().detach_and_log_err(cx);
                    }
                    SessionState::Booting(_) => {}
                    SessionState::Running(_) => {
                        debug_panic!("Attempting to boot a session that is already running");
                    }
                };
                this.state = SessionState::Running(mode);
                cx.emit(SessionStateEvent::Running);
            })?;

            this.update(cx, |session, cx| session.request_initialize(cx))?
                .await?;

            let result = this
                .update(cx, |session, cx| {
                    session.initialize_sequence(initialized_rx, dap_store.clone(), cx)
                })?
                .await;

            if result.is_err() {
                let mut console = this.update(cx, |session, cx| session.console_output(cx))?;

                console
                    .send(format!(
                        "Tried to launch debugger with: {}",
                        serde_json::to_string_pretty(&binary.request_args.configuration)
                            .unwrap_or_default(),
                    ))
                    .await
                    .ok();
            }

            result
        })
    }

    pub fn session_id(&self) -> SessionId {
        self.id
    }

    pub fn child_session_ids(&self) -> HashSet<SessionId> {
        self.child_session_ids.clone()
    }

    pub fn add_child_session_id(&mut self, session_id: SessionId) {
        self.child_session_ids.insert(session_id);
    }

    pub fn remove_child_session_id(&mut self, session_id: SessionId) {
        self.child_session_ids.remove(&session_id);
    }

    pub fn parent_id(&self, cx: &App) -> Option<SessionId> {
        self.parent_session
            .as_ref()
            .map(|session| session.read(cx).id)
    }

    pub fn parent_session(&self) -> Option<&Entity<Self>> {
        self.parent_session.as_ref()
    }

    pub fn on_app_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let Some(client) = self.adapter_client() else {
            return Task::ready(());
        };

        let supports_terminate = self
            .capabilities
            .support_terminate_debuggee
            .unwrap_or(false);

        cx.background_spawn(async move {
            if supports_terminate {
                client
                    .request::<dap::requests::Terminate>(dap::TerminateArguments {
                        restart: Some(false),
                    })
                    .await
                    .ok();
            } else {
                client
                    .request::<dap::requests::Disconnect>(dap::DisconnectArguments {
                        restart: Some(false),
                        terminate_debuggee: Some(true),
                        suspend_debuggee: Some(false),
                    })
                    .await
                    .ok();
            }
        })
    }

    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    pub fn binary(&self) -> Option<&DebugAdapterBinary> {
        match &self.state {
            SessionState::Booting(_) => None,
            SessionState::Running(running_mode) => Some(&running_mode.binary),
        }
    }

    pub fn adapter(&self) -> DebugAdapterName {
        self.adapter.clone()
    }

    pub fn label(&self) -> Option<SharedString> {
        self.label.clone()
    }

    pub fn is_terminated(&self) -> bool {
        self.is_session_terminated
    }

    pub fn console_output(&mut self, cx: &mut Context<Self>) -> mpsc::UnboundedSender<String> {
        let (tx, mut rx) = mpsc::unbounded();

        cx.spawn(async move |this, cx| {
            while let Some(output) = rx.next().await {
                this.update(cx, |this, _| {
                    let event = dap::OutputEvent {
                        category: None,
                        output,
                        group: None,
                        variables_reference: None,
                        source: None,
                        line: None,
                        column: None,
                        data: None,
                        location_reference: None,
                    };
                    this.push_output(event);
                })?;
            }
            anyhow::Ok(())
        })
        .detach();

        tx
    }

    pub fn is_started(&self) -> bool {
        match &self.state {
            SessionState::Booting(_) => false,
            SessionState::Running(running) => running.is_started,
        }
    }

    pub fn is_building(&self) -> bool {
        matches!(self.state, SessionState::Booting(_))
    }

    pub fn as_running_mut(&mut self) -> Option<&mut RunningMode> {
        match &mut self.state {
            SessionState::Running(local_mode) => Some(local_mode),
            SessionState::Booting(_) => None,
        }
    }

    pub fn as_running(&self) -> Option<&RunningMode> {
        match &self.state {
            SessionState::Running(local_mode) => Some(local_mode),
            SessionState::Booting(_) => None,
        }
    }

    fn handle_start_debugging_request(
        &mut self,
        request: dap::messages::Request,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let request_seq = request.seq;

        let launch_request: Option<Result<StartDebuggingRequestArguments, _>> = request
            .arguments
            .as_ref()
            .map(|value| serde_json::from_value(value.clone()));

        let mut success = true;
        if let Some(Ok(request)) = launch_request {
            cx.emit(SessionStateEvent::SpawnChildSession { request });
        } else {
            log::error!(
                "Failed to parse launch request arguments: {:?}",
                request.arguments
            );
            success = false;
        }

        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.respond_to_client(
                    request_seq,
                    success,
                    StartDebugging::COMMAND.to_string(),
                    None,
                    cx,
                )
            })?
            .await
        })
    }

    fn handle_run_in_terminal_request(
        &mut self,
        request: dap::messages::Request,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let request_args = match serde_json::from_value::<RunInTerminalRequestArguments>(
            request.arguments.unwrap_or_default(),
        ) {
            Ok(args) => args,
            Err(error) => {
                return cx.spawn(async move |session, cx| {
                    let error = serde_json::to_value(dap::ErrorResponse {
                        error: Some(dap::Message {
                            id: request.seq,
                            format: error.to_string(),
                            variables: None,
                            send_telemetry: None,
                            show_user: None,
                            url: None,
                            url_label: None,
                        }),
                    })
                    .ok();

                    session
                        .update(cx, |this, cx| {
                            this.respond_to_client(
                                request.seq,
                                false,
                                StartDebugging::COMMAND.to_string(),
                                error,
                                cx,
                            )
                        })?
                        .await?;

                    Err(anyhow!("Failed to parse RunInTerminalRequestArguments"))
                });
            }
        };

        let seq = request.seq;

        let (tx, mut rx) = mpsc::channel::<Result<u32>>(1);
        cx.emit(SessionEvent::RunInTerminal {
            request: request_args,
            sender: tx,
        });
        cx.notify();

        cx.spawn(async move |session, cx| {
            let result = util::maybe!(async move {
                rx.next().await.ok_or_else(|| {
                    anyhow!("failed to receive response from spawn terminal".to_string())
                })?
            })
            .await;
            let (success, body) = match result {
                Ok(pid) => (
                    true,
                    serde_json::to_value(dap::RunInTerminalResponse {
                        process_id: None,
                        shell_process_id: Some(pid as u64),
                    })
                    .ok(),
                ),
                Err(error) => (
                    false,
                    serde_json::to_value(dap::ErrorResponse {
                        error: Some(dap::Message {
                            id: seq,
                            format: error.to_string(),
                            variables: None,
                            send_telemetry: None,
                            show_user: None,
                            url: None,
                            url_label: None,
                        }),
                    })
                    .ok(),
                ),
            };

            session
                .update(cx, |session, cx| {
                    session.respond_to_client(
                        seq,
                        success,
                        RunInTerminal::COMMAND.to_string(),
                        body,
                        cx,
                    )
                })?
                .await
        })
    }

    pub(super) fn request_initialize(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let adapter_id = self.adapter().to_string();
        let request = Initialize { adapter_id };

        let SessionState::Running(running) = &self.state else {
            return Task::ready(Err(anyhow!(
                "Cannot send initialize request, task still building"
            )));
        };
        let mut response = running.request(request.clone());

        cx.spawn(async move |this, cx| {
            loop {
                let capabilities = response.await;
                match capabilities {
                    Err(e) => {
                        let Ok(Some(reconnect)) = this.update(cx, |this, cx| {
                            this.as_running()
                                .and_then(|running| running.reconnect_for_ssh(&mut cx.to_async()))
                        }) else {
                            return Err(e);
                        };
                        log::info!("Failed to connect to debug adapter: {}, retrying...", e);
                        reconnect.await?;

                        let Ok(Some(r)) = this.update(cx, |this, _| {
                            this.as_running()
                                .map(|running| running.request(request.clone()))
                        }) else {
                            return Err(e);
                        };
                        response = r
                    }
                    Ok(capabilities) => {
                        this.update(cx, |session, cx| {
                            session.capabilities = capabilities;

                            cx.emit(SessionEvent::CapabilitiesLoaded);
                        })?;
                        return Ok(());
                    }
                }
            }
        })
    }

    pub(super) fn initialize_sequence(
        &mut self,
        initialize_rx: oneshot::Receiver<()>,
        dap_store: WeakEntity<DapStore>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        match &self.state {
            SessionState::Running(local_mode) => {
                local_mode.initialize_sequence(&self.capabilities, initialize_rx, dap_store, cx)
            }
            SessionState::Booting(_) => {
                Task::ready(Err(anyhow!("cannot initialize, still building")))
            }
        }
    }

    pub fn run_to_position(
        &mut self,
        breakpoint: SourceBreakpoint,
        active_thread_id: ThreadId,
        cx: &mut Context<Self>,
    ) {
        match &mut self.state {
            SessionState::Running(local_mode) => {
                if !matches!(
                    self.active_snapshot
                        .thread_states
                        .thread_state(active_thread_id),
                    Some(ThreadStatus::Stopped)
                ) {
                    return;
                };
                let path = breakpoint.path.clone();
                local_mode.tmp_breakpoint = Some(breakpoint);
                let task = local_mode.send_breakpoints_from_path(
                    path,
                    BreakpointUpdatedReason::Toggled,
                    &self.breakpoint_store,
                    cx,
                );

                cx.spawn(async move |this, cx| {
                    task.await;
                    this.update(cx, |this, cx| {
                        this.continue_thread(active_thread_id, cx);
                    })
                })
                .detach();
            }
            SessionState::Booting(_) => {}
        }
    }

    pub fn has_new_output(&self, last_update: OutputToken) -> bool {
        self.output_token.0.checked_sub(last_update.0).unwrap_or(0) != 0
    }

    pub fn output(
        &self,
        since: OutputToken,
    ) -> (impl Iterator<Item = &dap::OutputEvent>, OutputToken) {
        if self.output_token.0 == 0 {
            return (self.output.range(0..0), OutputToken(0));
        };

        let events_since = self.output_token.0.checked_sub(since.0).unwrap_or(0);

        let clamped_events_since = events_since.clamp(0, self.output.len());
        (
            self.output
                .range(self.output.len() - clamped_events_since..),
            self.output_token,
        )
    }

    pub fn respond_to_client(
        &self,
        request_seq: u64,
        success: bool,
        command: String,
        body: Option<serde_json::Value>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(local_session) = self.as_running() else {
            unreachable!("Cannot respond to remote client");
        };
        let client = local_session.client.clone();

        cx.background_spawn(async move {
            client
                .send_message(Message::Response(Response {
                    body,
                    success,
                    command,
                    seq: request_seq + 1,
                    request_seq,
                    message: None,
                }))
                .await
        })
    }

    fn session_state(&self) -> &SessionSnapshot {
        self.selected_snapshot_index
            .and_then(|ix| self.snapshots.get(ix))
            .unwrap_or_else(|| &self.active_snapshot)
    }

    fn push_to_history(&mut self) {
        if !self.has_ever_stopped() {
            return;
        }

        while self.snapshots.len() >= DEBUG_HISTORY_LIMIT {
            self.snapshots.pop_front();
        }

        self.snapshots
            .push_back(std::mem::take(&mut self.active_snapshot));
    }

    pub fn historic_snapshots(&self) -> &VecDeque<SessionSnapshot> {
        &self.snapshots
    }

    pub fn select_historic_snapshot(&mut self, ix: Option<usize>, cx: &mut Context<Session>) {
        if self.selected_snapshot_index == ix {
            return;
        }

        if self
            .selected_snapshot_index
            .is_some_and(|ix| self.snapshots.len() <= ix)
        {
            debug_panic!("Attempted to select a debug session with an out of bounds index");
            return;
        }

        self.selected_snapshot_index = ix;
        cx.emit(SessionEvent::HistoricSnapshotSelected);
        cx.notify();
    }

    pub fn active_snapshot_index(&self) -> Option<usize> {
        self.selected_snapshot_index
    }

    fn handle_stopped_event(&mut self, event: StoppedEvent, cx: &mut Context<Self>) {
        self.push_to_history();

        self.state.stopped();
        // todo(debugger): Find a clean way to get around the clone
        let breakpoint_store = self.breakpoint_store.clone();
        if let Some((local, path)) = self.as_running_mut().and_then(|local| {
            let breakpoint = local.tmp_breakpoint.take()?;
            let path = breakpoint.path;
            Some((local, path))
        }) {
            local
                .send_breakpoints_from_path(
                    path,
                    BreakpointUpdatedReason::Toggled,
                    &breakpoint_store,
                    cx,
                )
                .detach();
        };

        if event.all_threads_stopped.unwrap_or_default() || event.thread_id.is_none() {
            self.active_snapshot.thread_states.stop_all_threads();
            self.invalidate_command_type::<StackTraceCommand>();
        }

        // Event if we stopped all threads we still need to insert the thread_id
        // to our own data
        if let Some(thread_id) = event.thread_id {
            self.active_snapshot
                .thread_states
                .stop_thread(ThreadId(thread_id));

            self.invalidate_state(
                &StackTraceCommand {
                    thread_id,
                    start_frame: None,
                    levels: None,
                }
                .into(),
            );
        }

        self.invalidate_generic();
        self.active_snapshot.threads.clear();
        self.active_snapshot.variables.clear();
        cx.emit(SessionEvent::Stopped(
            event
                .thread_id
                .map(Into::into)
                .filter(|_| !event.preserve_focus_hint.unwrap_or(false)),
        ));
        cx.emit(SessionEvent::InvalidateInlineValue);
        cx.notify();
    }

    pub(crate) fn handle_dap_event(&mut self, event: Box<Events>, cx: &mut Context<Self>) {
        match *event {
            Events::Initialized(_) => {
                debug_assert!(
                    false,
                    "Initialized event should have been handled in LocalMode"
                );
            }
            Events::Stopped(event) => self.handle_stopped_event(event, cx),
            Events::Continued(event) => {
                if event.all_threads_continued.unwrap_or_default() {
                    self.active_snapshot.thread_states.continue_all_threads();
                    self.breakpoint_store.update(cx, |store, cx| {
                        store.remove_active_position(Some(self.session_id()), cx)
                    });
                } else {
                    self.active_snapshot
                        .thread_states
                        .continue_thread(ThreadId(event.thread_id));
                }
                // todo(debugger): We should be able to get away with only invalidating generic if all threads were continued
                self.invalidate_generic();
            }
            Events::Exited(_event) => {
                self.clear_active_debug_line(cx);
            }
            Events::Terminated(_) => {
                self.shutdown(cx).detach();
            }
            Events::Thread(event) => {
                let thread_id = ThreadId(event.thread_id);

                match event.reason {
                    dap::ThreadEventReason::Started => {
                        self.active_snapshot
                            .thread_states
                            .continue_thread(thread_id);
                    }
                    dap::ThreadEventReason::Exited => {
                        self.active_snapshot.thread_states.exit_thread(thread_id);
                    }
                    reason => {
                        log::error!("Unhandled thread event reason {:?}", reason);
                    }
                }
                self.invalidate_state(&ThreadsCommand.into());
                cx.notify();
            }
            Events::Output(event) => {
                if event
                    .category
                    .as_ref()
                    .is_some_and(|category| *category == OutputEventCategory::Telemetry)
                {
                    return;
                }

                self.push_output(event);
                cx.notify();
            }
            Events::Breakpoint(event) => self.breakpoint_store.update(cx, |store, _| {
                store.update_session_breakpoint(self.session_id(), event.reason, event.breakpoint);
            }),
            Events::Module(event) => {
                match event.reason {
                    dap::ModuleEventReason::New => {
                        self.active_snapshot.modules.push(event.module);
                    }
                    dap::ModuleEventReason::Changed => {
                        if let Some(module) = self
                            .active_snapshot
                            .modules
                            .iter_mut()
                            .find(|other| event.module.id == other.id)
                        {
                            *module = event.module;
                        }
                    }
                    dap::ModuleEventReason::Removed => {
                        self.active_snapshot
                            .modules
                            .retain(|other| event.module.id != other.id);
                    }
                }

                // todo(debugger): We should only send the invalidate command to downstream clients.
                // self.invalidate_state(&ModulesCommand.into());
            }
            Events::LoadedSource(_) => {
                self.invalidate_state(&LoadedSourcesCommand.into());
            }
            Events::Capabilities(event) => {
                self.capabilities = self.capabilities.merge(event.capabilities);

                // The adapter might've enabled new exception breakpoints (or disabled existing ones).
                let recent_filters = self
                    .capabilities
                    .exception_breakpoint_filters
                    .iter()
                    .flatten()
                    .map(|filter| (filter.filter.clone(), filter.clone()))
                    .collect::<BTreeMap<_, _>>();
                for filter in recent_filters.values() {
                    let default = filter.default.unwrap_or_default();
                    self.exception_breakpoints
                        .entry(filter.filter.clone())
                        .or_insert_with(|| (filter.clone(), default));
                }
                self.exception_breakpoints
                    .retain(|k, _| recent_filters.contains_key(k));
                if self.is_started() {
                    self.send_exception_breakpoints(cx);
                }

                // Remove the ones that no longer exist.
                cx.notify();
            }
            Events::Memory(_) => {}
            Events::Process(_) => {}
            Events::ProgressEnd(_) => {}
            Events::ProgressStart(_) => {}
            Events::ProgressUpdate(_) => {}
            Events::Invalidated(_) => {}
            Events::Other(event) => {
                if event.event == "launchBrowserInCompanion" {
                    let Some(request) = serde_json::from_value(event.body).ok() else {
                        log::error!("failed to deserialize launchBrowserInCompanion event");
                        return;
                    };
                    self.launch_browser_for_remote_server(request, cx);
                } else if event.event == "killCompanionBrowser" {
                    let Some(request) = serde_json::from_value(event.body).ok() else {
                        log::error!("failed to deserialize killCompanionBrowser event");
                        return;
                    };
                    self.kill_browser(request, cx);
                }
            }
        }
    }

    /// Ensure that there's a request in flight for the given command, and if not, send it. Use this to run requests that are idempotent.
    fn fetch<T: LocalDapCommand + PartialEq + Eq + Hash>(
        &mut self,
        request: T,
        process_result: impl FnOnce(&mut Self, Result<T::Response>, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) {
        const {
            assert!(
                T::CACHEABLE,
                "Only requests marked as cacheable should invoke `fetch`"
            );
        }

        if (!self.active_snapshot.thread_states.any_stopped_thread()
            && request.type_id() != TypeId::of::<ThreadsCommand>())
            || self.selected_snapshot_index.is_some()
            || self.is_session_terminated
        {
            return;
        }

        let request_map = self
            .requests
            .entry(std::any::TypeId::of::<T>())
            .or_default();

        if let Entry::Vacant(vacant) = request_map.entry(request.into()) {
            let command = vacant.key().0.clone().as_any_arc().downcast::<T>().unwrap();

            let task = Self::request_inner::<Arc<T>>(
                &self.capabilities,
                &self.state,
                command,
                |this, result, cx| {
                    process_result(this, result, cx);
                    None
                },
                cx,
            );
            let task = cx
                .background_executor()
                .spawn(async move {
                    let _ = task.await?;
                    Some(())
                })
                .shared();

            vacant.insert(task);
            cx.notify();
        }
    }

    fn request_inner<T: LocalDapCommand + PartialEq + Eq + Hash>(
        capabilities: &Capabilities,
        mode: &SessionState,
        request: T,
        process_result: impl FnOnce(
            &mut Self,
            Result<T::Response>,
            &mut Context<Self>,
        ) -> Option<T::Response>
        + 'static,
        cx: &mut Context<Self>,
    ) -> Task<Option<T::Response>> {
        if !T::is_supported(capabilities) {
            log::warn!(
                "Attempted to send a DAP request that isn't supported: {:?}",
                request
            );
            let error = Err(anyhow::Error::msg(
                "Couldn't complete request because it's not supported",
            ));
            return cx.spawn(async move |this, cx| {
                this.update(cx, |this, cx| process_result(this, error, cx))
                    .ok()
                    .flatten()
            });
        }

        let request = mode.request_dap(request);
        cx.spawn(async move |this, cx| {
            let result = request.await;
            this.update(cx, |this, cx| process_result(this, result, cx))
                .ok()
                .flatten()
        })
    }

    fn request<T: LocalDapCommand + PartialEq + Eq + Hash>(
        &self,
        request: T,
        process_result: impl FnOnce(
            &mut Self,
            Result<T::Response>,
            &mut Context<Self>,
        ) -> Option<T::Response>
        + 'static,
        cx: &mut Context<Self>,
    ) -> Task<Option<T::Response>> {
        Self::request_inner(&self.capabilities, &self.state, request, process_result, cx)
    }

    fn invalidate_command_type<Command: LocalDapCommand>(&mut self) {
        self.requests.remove(&std::any::TypeId::of::<Command>());
    }

    fn invalidate_generic(&mut self) {
        self.invalidate_command_type::<ModulesCommand>();
        self.invalidate_command_type::<LoadedSourcesCommand>();
        self.invalidate_command_type::<ThreadsCommand>();
        self.invalidate_command_type::<DataBreakpointInfoCommand>();
        self.invalidate_command_type::<ReadMemory>();
        let executor = self.as_running().map(|running| running.executor.clone());
        if let Some(executor) = executor {
            self.memory.clear(&executor);
        }
    }

    fn invalidate_state(&mut self, key: &RequestSlot) {
        self.requests
            .entry((&*key.0 as &dyn Any).type_id())
            .and_modify(|request_map| {
                request_map.remove(key);
            });
    }

    fn push_output(&mut self, event: OutputEvent) {
        self.output.push_back(event);
        self.output_token.0 += 1;
    }

    pub fn any_stopped_thread(&self) -> bool {
        self.active_snapshot.thread_states.any_stopped_thread()
    }

    pub fn thread_status(&self, thread_id: ThreadId) -> ThreadStatus {
        self.active_snapshot.thread_states.thread_status(thread_id)
    }

    pub fn threads(&mut self, cx: &mut Context<Self>) -> Vec<(dap::Thread, ThreadStatus)> {
        self.fetch(
            dap_command::ThreadsCommand,
            |this, result, cx| {
                let Some(result) = result.log_err() else {
                    return;
                };

                this.active_snapshot.threads = result
                    .into_iter()
                    .map(|thread| (ThreadId(thread.id), Thread::from(thread)))
                    .collect();

                this.invalidate_command_type::<StackTraceCommand>();
                cx.emit(SessionEvent::Threads);
                cx.notify();
            },
            cx,
        );

        let state = self.session_state();
        state
            .threads
            .values()
            .map(|thread| {
                (
                    thread.dap.clone(),
                    state.thread_states.thread_status(ThreadId(thread.dap.id)),
                )
            })
            .collect()
    }

    pub fn modules(&mut self, cx: &mut Context<Self>) -> &[Module] {
        self.fetch(
            dap_command::ModulesCommand,
            |this, result, cx| {
                let Some(result) = result.log_err() else {
                    return;
                };

                this.active_snapshot.modules = result;
                cx.emit(SessionEvent::Modules);
                cx.notify();
            },
            cx,
        );

        &self.session_state().modules
    }

    // CodeLLDB returns the size of a pointed-to-memory, which we can use to make the experience of go-to-memory better.
    pub fn data_access_size(
        &mut self,
        frame_id: Option<u64>,
        evaluate_name: &str,
        cx: &mut Context<Self>,
    ) -> Task<Option<u64>> {
        let request = self.request(
            EvaluateCommand {
                expression: format!("?${{sizeof({evaluate_name})}}"),
                frame_id,

                context: Some(EvaluateArgumentsContext::Repl),
                source: None,
            },
            |_, response, _| response.ok(),
            cx,
        );
        cx.background_spawn(async move {
            let result = request.await?;
            result.result.parse().ok()
        })
    }

    pub fn memory_reference_of_expr(
        &mut self,
        frame_id: Option<u64>,
        expression: String,
        cx: &mut Context<Self>,
    ) -> Task<Option<(String, Option<String>)>> {
        let request = self.request(
            EvaluateCommand {
                expression,
                frame_id,

                context: Some(EvaluateArgumentsContext::Repl),
                source: None,
            },
            |_, response, _| response.ok(),
            cx,
        );
        cx.background_spawn(async move {
            let result = request.await?;
            result
                .memory_reference
                .map(|reference| (reference, result.type_))
        })
    }

    pub fn write_memory(&mut self, address: u64, data: &[u8], cx: &mut Context<Self>) {
        let data = base64::engine::general_purpose::STANDARD.encode(data);
        self.request(
            WriteMemoryArguments {
                memory_reference: address.to_string(),
                data,
                allow_partial: None,
                offset: None,
            },
            |this, response, cx| {
                this.memory.clear(cx.background_executor());
                this.invalidate_command_type::<ReadMemory>();
                this.invalidate_command_type::<VariablesCommand>();
                cx.emit(SessionEvent::Variables);
                response.ok()
            },
            cx,
        )
        .detach();
    }
    pub fn read_memory(
        &mut self,
        range: RangeInclusive<u64>,
        cx: &mut Context<Self>,
    ) -> MemoryIterator {
        // This function is a bit more involved when it comes to fetching data.
        // Since we attempt to read memory in pages, we need to account for some parts
        // of memory being unreadable. Therefore, we start off by fetching a page per request.
        // In case that fails, we try to re-fetch smaller regions until we have the full range.
        let page_range = Memory::memory_range_to_page_range(range.clone());
        for page_address in PageAddress::iter_range(page_range) {
            self.read_single_page_memory(page_address, cx);
        }
        self.memory.memory_range(range)
    }

    fn read_single_page_memory(&mut self, page_start: PageAddress, cx: &mut Context<Self>) {
        _ = maybe!({
            let builder = self.memory.build_page(page_start)?;

            self.memory_read_fetch_page_recursive(builder, cx);
            Some(())
        });
    }
    fn memory_read_fetch_page_recursive(
        &mut self,
        mut builder: MemoryPageBuilder,
        cx: &mut Context<Self>,
    ) {
        let Some(next_request) = builder.next_request() else {
            // We're done fetching. Let's grab the page and insert it into our memory store.
            let (address, contents) = builder.build();
            self.memory.insert_page(address, contents);

            return;
        };
        let size = next_request.size;
        self.fetch(
            ReadMemory {
                memory_reference: format!("0x{:X}", next_request.address),
                offset: Some(0),
                count: next_request.size,
            },
            move |this, memory, cx| {
                if let Ok(memory) = memory {
                    builder.known(memory.content);
                    if let Some(unknown) = memory.unreadable_bytes {
                        builder.unknown(unknown);
                    }
                    // This is the recursive bit: if we're not yet done with
                    // the whole page, we'll kick off a new request with smaller range.
                    // Note that this function is recursive only conceptually;
                    // since it kicks off a new request with callback, we don't need to worry about stack overflow.
                    this.memory_read_fetch_page_recursive(builder, cx);
                } else {
                    builder.unknown(size);
                }
            },
            cx,
        );
    }

    pub fn ignore_breakpoints(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn toggle_ignore_breakpoints(
        &mut self,
        cx: &mut App,
    ) -> Task<HashMap<Arc<Path>, anyhow::Error>> {
        self.set_ignore_breakpoints(!self.ignore_breakpoints, cx)
    }

    pub(crate) fn set_ignore_breakpoints(
        &mut self,
        ignore: bool,
        cx: &mut App,
    ) -> Task<HashMap<Arc<Path>, anyhow::Error>> {
        if self.ignore_breakpoints == ignore {
            return Task::ready(HashMap::default());
        }

        self.ignore_breakpoints = ignore;

        if let Some(local) = self.as_running() {
            local.send_source_breakpoints(ignore, &self.breakpoint_store, cx)
        } else {
            // todo(debugger): We need to propagate this change to downstream sessions and send a message to upstream sessions
            unimplemented!()
        }
    }

    pub fn data_breakpoints(&self) -> impl Iterator<Item = &DataBreakpointState> {
        self.data_breakpoints.values()
    }

    pub fn exception_breakpoints(
        &self,
    ) -> impl Iterator<Item = &(ExceptionBreakpointsFilter, IsEnabled)> {
        self.exception_breakpoints.values()
    }

    pub fn toggle_exception_breakpoint(&mut self, id: &str, cx: &App) {
        if let Some((_, is_enabled)) = self.exception_breakpoints.get_mut(id) {
            *is_enabled = !*is_enabled;
            self.send_exception_breakpoints(cx);
        }
    }

    fn send_exception_breakpoints(&mut self, cx: &App) {
        if let Some(local) = self.as_running() {
            let exception_filters = self
                .exception_breakpoints
                .values()
                .filter_map(|(filter, is_enabled)| is_enabled.then(|| filter.clone()))
                .collect();

            let supports_exception_filters = self
                .capabilities
                .supports_exception_filter_options
                .unwrap_or_default();
            local
                .send_exception_breakpoints(exception_filters, supports_exception_filters)
                .detach_and_log_err(cx);
        } else {
            debug_assert!(false, "Not implemented");
        }
    }

    pub fn toggle_data_breakpoint(&mut self, id: &str, cx: &mut Context<'_, Session>) {
        if let Some(state) = self.data_breakpoints.get_mut(id) {
            state.is_enabled = !state.is_enabled;
            self.send_exception_breakpoints(cx);
        }
    }

    fn send_data_breakpoints(&mut self, cx: &mut Context<Self>) {
        if let Some(mode) = self.as_running() {
            let breakpoints = self
                .data_breakpoints
                .values()
                .filter_map(|state| state.is_enabled.then(|| state.dap.clone()))
                .collect();
            let command = SetDataBreakpointsCommand { breakpoints };
            mode.request(command).detach_and_log_err(cx);
        }
    }

    pub fn create_data_breakpoint(
        &mut self,
        context: Arc<DataBreakpointContext>,
        data_id: String,
        dap: dap::DataBreakpoint,
        cx: &mut Context<Self>,
    ) {
        if self.data_breakpoints.remove(&data_id).is_none() {
            self.data_breakpoints.insert(
                data_id,
                DataBreakpointState {
                    dap,
                    is_enabled: true,
                    context,
                },
            );
        }
        self.send_data_breakpoints(cx);
    }

    pub fn breakpoints_enabled(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn loaded_sources(&mut self, cx: &mut Context<Self>) -> &[Source] {
        self.fetch(
            dap_command::LoadedSourcesCommand,
            |this, result, cx| {
                let Some(result) = result.log_err() else {
                    return;
                };
                this.active_snapshot.loaded_sources = result;
                cx.emit(SessionEvent::LoadedSources);
                cx.notify();
            },
            cx,
        );
        &self.session_state().loaded_sources
    }

    fn fallback_to_manual_restart(
        &mut self,
        res: Result<()>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        if res.log_err().is_none() {
            cx.emit(SessionStateEvent::Restart);
            return None;
        }
        Some(())
    }

    fn empty_response(&mut self, res: Result<()>, _cx: &mut Context<Self>) -> Option<()> {
        res.log_err()?;
        Some(())
    }

    fn on_step_response<T: LocalDapCommand + PartialEq + Eq + Hash>(
        thread_id: ThreadId,
    ) -> impl FnOnce(&mut Self, Result<T::Response>, &mut Context<Self>) -> Option<T::Response> + 'static
    {
        move |this, response, cx| match response.log_err() {
            Some(response) => {
                this.breakpoint_store.update(cx, |store, cx| {
                    store.remove_active_position(Some(this.session_id()), cx)
                });
                Some(response)
            }
            None => {
                this.active_snapshot.thread_states.stop_thread(thread_id);
                cx.notify();
                None
            }
        }
    }

    fn clear_active_debug_line_response(
        &mut self,
        response: Result<()>,
        cx: &mut Context<Session>,
    ) -> Option<()> {
        response.log_err()?;
        self.clear_active_debug_line(cx);
        Some(())
    }

    fn clear_active_debug_line(&mut self, cx: &mut Context<Session>) {
        self.breakpoint_store.update(cx, |store, cx| {
            store.remove_active_position(Some(self.id), cx)
        });
    }

    pub fn pause_thread(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        self.request(
            PauseCommand {
                thread_id: thread_id.0,
            },
            Self::empty_response,
            cx,
        )
        .detach();
    }

    pub fn restart_stack_frame(&mut self, stack_frame_id: u64, cx: &mut Context<Self>) {
        self.request(
            RestartStackFrameCommand { stack_frame_id },
            Self::empty_response,
            cx,
        )
        .detach();
    }

    pub fn restart(&mut self, args: Option<Value>, cx: &mut Context<Self>) {
        if self.restart_task.is_some() || self.as_running().is_none() {
            return;
        }

        let supports_dap_restart =
            self.capabilities.supports_restart_request.unwrap_or(false) && !self.is_terminated();

        self.restart_task = Some(cx.spawn(async move |this, cx| {
            let _ = this.update(cx, |session, cx| {
                if supports_dap_restart {
                    session
                        .request(
                            RestartCommand {
                                raw: args.unwrap_or(Value::Null),
                            },
                            Self::fallback_to_manual_restart,
                            cx,
                        )
                        .detach();
                } else {
                    cx.emit(SessionStateEvent::Restart);
                }
            });
        }));
    }

    pub fn shutdown(&mut self, cx: &mut Context<Self>) -> Task<()> {
        if self.is_session_terminated {
            return Task::ready(());
        }

        self.is_session_terminated = true;
        self.active_snapshot.thread_states.exit_all_threads();
        cx.notify();

        let task = match &mut self.state {
            SessionState::Running(_) => {
                if self
                    .capabilities
                    .supports_terminate_request
                    .unwrap_or_default()
                {
                    self.request(
                        TerminateCommand {
                            restart: Some(false),
                        },
                        Self::clear_active_debug_line_response,
                        cx,
                    )
                } else {
                    self.request(
                        DisconnectCommand {
                            restart: Some(false),
                            terminate_debuggee: Some(true),
                            suspend_debuggee: Some(false),
                        },
                        Self::clear_active_debug_line_response,
                        cx,
                    )
                }
            }
            SessionState::Booting(build_task) => {
                build_task.take();
                Task::ready(Some(()))
            }
        };

        cx.emit(SessionStateEvent::Shutdown);

        cx.spawn(async move |this, cx| {
            task.await;
            let _ = this.update(cx, |this, _| {
                if let Some(adapter_client) = this.adapter_client() {
                    adapter_client.kill();
                }
            });
        })
    }

    pub fn completions(
        &mut self,
        query: CompletionsQuery,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<dap::CompletionItem>>> {
        let task = self.request(query, |_, result, _| result.log_err(), cx);

        cx.background_executor().spawn(async move {
            anyhow::Ok(
                task.await
                    .map(|response| response.targets)
                    .context("failed to fetch completions")?,
            )
        })
    }

    pub fn continue_thread(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        self.select_historic_snapshot(None, cx);

        let supports_single_thread_execution_requests =
            self.capabilities.supports_single_thread_execution_requests;
        self.active_snapshot
            .thread_states
            .continue_thread(thread_id);
        self.request(
            ContinueCommand {
                args: ContinueArguments {
                    thread_id: thread_id.0,
                    single_thread: supports_single_thread_execution_requests,
                },
            },
            Self::on_step_response::<ContinueCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn adapter_client(&self) -> Option<Arc<DebugAdapterClient>> {
        match self.state {
            SessionState::Running(ref local) => Some(local.client.clone()),
            SessionState::Booting(_) => None,
        }
    }

    pub fn has_ever_stopped(&self) -> bool {
        self.state.has_ever_stopped()
    }

    pub fn step_over(
        &mut self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
        self.select_historic_snapshot(None, cx);

        let supports_single_thread_execution_requests =
            self.capabilities.supports_single_thread_execution_requests;
        let supports_stepping_granularity = self
            .capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = NextCommand {
            inner: StepCommand {
                thread_id: thread_id.0,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests,
            },
        };

        self.active_snapshot.thread_states.process_step(thread_id);
        self.request(
            command,
            Self::on_step_response::<NextCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn step_in(
        &mut self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
        self.select_historic_snapshot(None, cx);

        let supports_single_thread_execution_requests =
            self.capabilities.supports_single_thread_execution_requests;
        let supports_stepping_granularity = self
            .capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = StepInCommand {
            inner: StepCommand {
                thread_id: thread_id.0,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests,
            },
        };

        self.active_snapshot.thread_states.process_step(thread_id);
        self.request(
            command,
            Self::on_step_response::<StepInCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn step_out(
        &mut self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
        self.select_historic_snapshot(None, cx);

        let supports_single_thread_execution_requests =
            self.capabilities.supports_single_thread_execution_requests;
        let supports_stepping_granularity = self
            .capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = StepOutCommand {
            inner: StepCommand {
                thread_id: thread_id.0,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests,
            },
        };

        self.active_snapshot.thread_states.process_step(thread_id);
        self.request(
            command,
            Self::on_step_response::<StepOutCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn step_back(
        &mut self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
        self.select_historic_snapshot(None, cx);

        let supports_single_thread_execution_requests =
            self.capabilities.supports_single_thread_execution_requests;
        let supports_stepping_granularity = self
            .capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = StepBackCommand {
            inner: StepCommand {
                thread_id: thread_id.0,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests,
            },
        };

        self.active_snapshot.thread_states.process_step(thread_id);

        self.request(
            command,
            Self::on_step_response::<StepBackCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn stack_frames(
        &mut self,
        thread_id: ThreadId,
        cx: &mut Context<Self>,
    ) -> Result<Vec<StackFrame>> {
        if self.active_snapshot.thread_states.thread_status(thread_id) == ThreadStatus::Stopped
            && self.requests.contains_key(&ThreadsCommand.type_id())
            && self.active_snapshot.threads.contains_key(&thread_id)
        // ^ todo(debugger): We need a better way to check that we're not querying stale data
        // We could still be using an old thread id and have sent a new thread's request
        // This isn't the biggest concern right now because it hasn't caused any issues outside of tests
        // But it very well could cause a minor bug in the future that is hard to track down
        {
            self.fetch(
                super::dap_command::StackTraceCommand {
                    thread_id: thread_id.0,
                    start_frame: None,
                    levels: None,
                },
                move |this, stack_frames, cx| {
                    let entry =
                        this.active_snapshot
                            .threads
                            .entry(thread_id)
                            .and_modify(|thread| match &stack_frames {
                                Ok(stack_frames) => {
                                    thread.stack_frames = stack_frames
                                        .iter()
                                        .cloned()
                                        .map(StackFrame::from)
                                        .collect();
                                    thread.stack_frames_error = None;
                                }
                                Err(error) => {
                                    thread.stack_frames.clear();
                                    thread.stack_frames_error = Some(error.to_string().into());
                                }
                            });
                    debug_assert!(
                        matches!(entry, indexmap::map::Entry::Occupied(_)),
                        "Sent request for thread_id that doesn't exist"
                    );
                    if let Ok(stack_frames) = stack_frames {
                        this.active_snapshot.stack_frames.extend(
                            stack_frames
                                .into_iter()
                                .filter(|frame| {
                                    // Workaround for JavaScript debug adapter sending out "fake" stack frames for delineating await points. This is fine,
                                    // except that they always use an id of 0 for it, which collides with other (valid) stack frames.
                                    !(frame.id == 0
                                        && frame.line == 0
                                        && frame.column == 0
                                        && frame.presentation_hint
                                            == Some(StackFramePresentationHint::Label))
                                })
                                .map(|frame| (frame.id, StackFrame::from(frame))),
                        );
                    }

                    this.invalidate_command_type::<ScopesCommand>();
                    this.invalidate_command_type::<VariablesCommand>();

                    cx.emit(SessionEvent::StackTrace);
                },
                cx,
            );
        }

        match self.session_state().threads.get(&thread_id) {
            Some(thread) => {
                if let Some(error) = &thread.stack_frames_error {
                    Err(anyhow!(error.to_string()))
                } else {
                    Ok(thread.stack_frames.clone())
                }
            }
            None => Ok(Vec::new()),
        }
    }

    pub fn scopes(&mut self, stack_frame_id: u64, cx: &mut Context<Self>) -> &[dap::Scope] {
        if self.requests.contains_key(&TypeId::of::<ThreadsCommand>())
            && self
                .requests
                .contains_key(&TypeId::of::<StackTraceCommand>())
        {
            self.fetch(
                ScopesCommand { stack_frame_id },
                move |this, scopes, cx| {
                    let Some(scopes) = scopes.log_err() else {
                        return
                    };

                    for scope in scopes.iter() {
                        this.variables(scope.variables_reference, cx);
                    }

                    let entry = this
                        .active_snapshot
                        .stack_frames
                        .entry(stack_frame_id)
                        .and_modify(|stack_frame| {
                            stack_frame.scopes = scopes;
                        });

                    cx.emit(SessionEvent::Variables);

                    debug_assert!(
                        matches!(entry, indexmap::map::Entry::Occupied(_)),
                        "Sent scopes request for stack_frame_id that doesn't exist or hasn't been fetched"
                    );
                },
                cx,
            );
        }

        self.session_state()
            .stack_frames
            .get(&stack_frame_id)
            .map(|frame| frame.scopes.as_slice())
            .unwrap_or_default()
    }

    pub fn variables_by_stack_frame_id(
        &self,
        stack_frame_id: StackFrameId,
        globals: bool,
        locals: bool,
    ) -> Vec<dap::Variable> {
        let state = self.session_state();
        let Some(stack_frame) = state.stack_frames.get(&stack_frame_id) else {
            return Vec::new();
        };

        stack_frame
            .scopes
            .iter()
            .filter(|scope| {
                (scope.name.to_lowercase().contains("local") && locals)
                    || (scope.name.to_lowercase().contains("global") && globals)
            })
            .filter_map(|scope| state.variables.get(&scope.variables_reference))
            .flatten()
            .cloned()
            .collect()
    }

    pub fn watchers(&self) -> &HashMap<SharedString, Watcher> {
        &self.watchers
    }

    pub fn add_watcher(
        &mut self,
        expression: SharedString,
        frame_id: u64,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let request = self.state.request_dap(EvaluateCommand {
            expression: expression.to_string(),
            context: Some(EvaluateArgumentsContext::Watch),
            frame_id: Some(frame_id),
            source: None,
        });

        cx.spawn(async move |this, cx| {
            let response = request.await?;

            this.update(cx, |session, cx| {
                session.watchers.insert(
                    expression.clone(),
                    Watcher {
                        expression,
                        value: response.result.into(),
                        variables_reference: response.variables_reference,
                        presentation_hint: response.presentation_hint,
                    },
                );
                cx.emit(SessionEvent::Watchers);
            })
        })
    }

    pub fn refresh_watchers(&mut self, frame_id: u64, cx: &mut Context<Self>) {
        let watches = self.watchers.clone();
        for (_, watch) in watches.into_iter() {
            self.add_watcher(watch.expression.clone(), frame_id, cx)
                .detach();
        }
    }

    pub fn remove_watcher(&mut self, expression: SharedString) {
        self.watchers.remove(&expression);
    }

    pub fn variables(
        &mut self,
        variables_reference: VariableReference,
        cx: &mut Context<Self>,
    ) -> Vec<dap::Variable> {
        let command = VariablesCommand {
            variables_reference,
            filter: None,
            start: None,
            count: None,
            format: None,
        };

        self.fetch(
            command,
            move |this, variables, cx| {
                let Some(variables) = variables.log_err() else {
                    return;
                };

                this.active_snapshot
                    .variables
                    .insert(variables_reference, variables);

                cx.emit(SessionEvent::Variables);
                cx.emit(SessionEvent::InvalidateInlineValue);
            },
            cx,
        );

        self.session_state()
            .variables
            .get(&variables_reference)
            .cloned()
            .unwrap_or_default()
    }

    pub fn data_breakpoint_info(
        &mut self,
        context: Arc<DataBreakpointContext>,
        mode: Option<String>,
        cx: &mut Context<Self>,
    ) -> Task<Option<dap::DataBreakpointInfoResponse>> {
        let command = DataBreakpointInfoCommand { context, mode };

        self.request(command, |_, response, _| response.ok(), cx)
    }

    pub fn set_variable_value(
        &mut self,
        stack_frame_id: u64,
        variables_reference: u64,
        name: String,
        value: String,
        cx: &mut Context<Self>,
    ) {
        if self.capabilities.supports_set_variable.unwrap_or_default() {
            self.request(
                SetVariableValueCommand {
                    name,
                    value,
                    variables_reference,
                },
                move |this, response, cx| {
                    let response = response.log_err()?;
                    this.invalidate_command_type::<VariablesCommand>();
                    this.invalidate_command_type::<ReadMemory>();
                    this.memory.clear(cx.background_executor());
                    this.refresh_watchers(stack_frame_id, cx);
                    cx.emit(SessionEvent::Variables);
                    Some(response)
                },
                cx,
            )
            .detach();
        }
    }

    pub fn evaluate(
        &mut self,
        expression: String,
        context: Option<EvaluateArgumentsContext>,
        frame_id: Option<u64>,
        source: Option<Source>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let event = dap::OutputEvent {
            category: None,
            output: format!("> {expression}"),
            group: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            data: None,
            location_reference: None,
        };
        self.push_output(event);
        let request = self.state.request_dap(EvaluateCommand {
            expression,
            context,
            frame_id,
            source,
        });
        cx.spawn(async move |this, cx| {
            let response = request.await;
            this.update(cx, |this, cx| {
                this.memory.clear(cx.background_executor());
                this.invalidate_command_type::<ReadMemory>();
                this.invalidate_command_type::<VariablesCommand>();
                cx.emit(SessionEvent::Variables);
                match response {
                    Ok(response) => {
                        let event = dap::OutputEvent {
                            category: None,
                            output: format!("< {}", &response.result),
                            group: None,
                            variables_reference: Some(response.variables_reference),
                            source: None,
                            line: None,
                            column: None,
                            data: None,
                            location_reference: None,
                        };
                        this.push_output(event);
                    }
                    Err(e) => {
                        let event = dap::OutputEvent {
                            category: None,
                            output: format!("{}", e),
                            group: None,
                            variables_reference: None,
                            source: None,
                            line: None,
                            column: None,
                            data: None,
                            location_reference: None,
                        };
                        this.push_output(event);
                    }
                };
                cx.notify();
            })
            .ok();
        })
    }

    pub fn location(
        &mut self,
        reference: u64,
        cx: &mut Context<Self>,
    ) -> Option<dap::LocationsResponse> {
        self.fetch(
            LocationsCommand { reference },
            move |this, response, _| {
                let Some(response) = response.log_err() else {
                    return;
                };
                this.active_snapshot.locations.insert(reference, response);
            },
            cx,
        );
        self.session_state().locations.get(&reference).cloned()
    }

    pub fn is_attached(&self) -> bool {
        let SessionState::Running(local_mode) = &self.state else {
            return false;
        };
        local_mode.binary.request_args.request == StartDebuggingRequestArgumentsRequest::Attach
    }

    pub fn disconnect_client(&mut self, cx: &mut Context<Self>) {
        let command = DisconnectCommand {
            restart: Some(false),
            terminate_debuggee: Some(false),
            suspend_debuggee: Some(false),
        };

        self.request(command, Self::empty_response, cx).detach()
    }

    pub fn terminate_threads(&mut self, thread_ids: Option<Vec<ThreadId>>, cx: &mut Context<Self>) {
        if self
            .capabilities
            .supports_terminate_threads_request
            .unwrap_or_default()
        {
            self.request(
                TerminateThreadsCommand {
                    thread_ids: thread_ids.map(|ids| ids.into_iter().map(|id| id.0).collect()),
                },
                Self::clear_active_debug_line_response,
                cx,
            )
            .detach();
        } else {
            self.shutdown(cx).detach();
        }
    }

    pub fn thread_state(&self, thread_id: ThreadId) -> Option<ThreadStatus> {
        self.session_state().thread_states.thread_state(thread_id)
    }

    pub fn quirks(&self) -> SessionQuirks {
        self.quirks
    }

    fn launch_browser_for_remote_server(
        &mut self,
        mut request: LaunchBrowserInCompanionParams,
        cx: &mut Context<Self>,
    ) {
        let Some(remote_client) = self.remote_client.clone() else {
            log::error!("can't launch browser in companion for non-remote project");
            return;
        };
        let Some(http_client) = self.http_client.clone() else {
            return;
        };
        let Some(node_runtime) = self.node_runtime.clone() else {
            return;
        };

        let mut console_output = self.console_output(cx);
        let task = cx.spawn(async move |this, cx| {
            let forward_ports_process = if remote_client
                .read_with(cx, |client, _| client.shares_network_interface())
            {
                request.other.insert(
                    "proxyUri".into(),
                    format!("127.0.0.1:{}", request.server_port).into(),
                );
                None
            } else {
                let port = TcpTransport::unused_port(Ipv4Addr::LOCALHOST)
                    .await
                    .context("getting port for DAP")?;
                request
                    .other
                    .insert("proxyUri".into(), format!("127.0.0.1:{port}").into());
                let mut port_forwards = vec![(port, "localhost".to_owned(), request.server_port)];

                if let Some(value) = request.params.get("url")
                    && let Some(url) = value.as_str()
                    && let Some(url) = Url::parse(url).ok()
                    && let Some(frontend_port) = url.port()
                {
                    port_forwards.push((frontend_port, "localhost".to_owned(), frontend_port));
                }

                let child = remote_client.update(cx, |client, _| {
                    let command = client.build_forward_ports_command(port_forwards)?;
                    let child = new_smol_command(command.program)
                        .args(command.args)
                        .envs(command.env)
                        .spawn()
                        .context("spawning port forwarding process")?;
                    anyhow::Ok(child)
                })?;
                Some(child)
            };

            let mut companion_process = None;
            let companion_port =
                if let Some(companion_port) = this.read_with(cx, |this, _| this.companion_port)? {
                    companion_port
                } else {
                    let task = cx.spawn(async move |cx| spawn_companion(node_runtime, cx).await);
                    match task.await {
                        Ok((port, child)) => {
                            companion_process = Some(child);
                            port
                        }
                        Err(e) => {
                            console_output
                                .send(format!("Failed to launch browser companion process: {e}"))
                                .await
                                .ok();
                            return Err(e);
                        }
                    }
                };

            let mut background_tasks = Vec::new();
            if let Some(mut forward_ports_process) = forward_ports_process {
                background_tasks.push(cx.spawn(async move |_| {
                    forward_ports_process.status().await.log_err();
                }));
            };
            if let Some(mut companion_process) = companion_process {
                if let Some(stderr) = companion_process.stderr.take() {
                    let mut console_output = console_output.clone();
                    background_tasks.push(cx.spawn(async move |_| {
                        let mut stderr = BufReader::new(stderr);
                        let mut line = String::new();
                        while let Ok(n) = stderr.read_line(&mut line).await
                            && n > 0
                        {
                            console_output
                                .send(format!("companion stderr: {line}"))
                                .await
                                .ok();
                            line.clear();
                        }
                    }));
                }
                background_tasks.push(cx.spawn({
                    let mut console_output = console_output.clone();
                    async move |_| match companion_process.status().await {
                        Ok(status) => {
                            if status.success() {
                                console_output
                                    .send("Companion process exited normally".into())
                                    .await
                                    .ok();
                            } else {
                                console_output
                                    .send(format!(
                                        "Companion process exited abnormally with {status:?}"
                                    ))
                                    .await
                                    .ok();
                            }
                        }
                        Err(e) => {
                            console_output
                                .send(format!("Failed to join companion process: {e}"))
                                .await
                                .ok();
                        }
                    }
                }));
            }

            // TODO pass wslInfo as needed

            let companion_address = format!("127.0.0.1:{companion_port}");
            let mut companion_started = false;
            for _ in 0..10 {
                if TcpStream::connect(&companion_address).await.is_ok() {
                    companion_started = true;
                    break;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
            }
            if !companion_started {
                console_output
                    .send("Browser companion failed to start".into())
                    .await
                    .ok();
                bail!("Browser companion failed to start");
            }

            let response = http_client
                .post_json(
                    &format!("http://{companion_address}/launch-and-attach"),
                    serde_json::to_string(&request)
                        .context("serializing request")?
                        .into(),
                )
                .await;
            match response {
                Ok(response) => {
                    if !response.status().is_success() {
                        console_output
                            .send("Launch request to companion failed".into())
                            .await
                            .ok();
                        return Err(anyhow!("launch request failed"));
                    }
                }
                Err(e) => {
                    console_output
                        .send("Failed to read response from companion".into())
                        .await
                        .ok();
                    return Err(e);
                }
            }

            this.update(cx, |this, _| {
                this.background_tasks.extend(background_tasks);
                this.companion_port = Some(companion_port);
            })?;

            anyhow::Ok(())
        });
        self.background_tasks.push(cx.spawn(async move |_, _| {
            task.await.log_err();
        }));
    }

    fn kill_browser(&self, request: KillCompanionBrowserParams, cx: &mut App) {
        let Some(companion_port) = self.companion_port else {
            log::error!("received killCompanionBrowser but js-debug-companion is not running");
            return;
        };
        let Some(http_client) = self.http_client.clone() else {
            return;
        };

        cx.spawn(async move |_| {
            http_client
                .post_json(
                    &format!("http://127.0.0.1:{companion_port}/kill"),
                    serde_json::to_string(&request)
                        .context("serializing request")?
                        .into(),
                )
                .await?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LaunchBrowserInCompanionParams {
    server_port: u16,
    params: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KillCompanionBrowserParams {
    launch_id: u64,
}

async fn spawn_companion(
    node_runtime: NodeRuntime,
    cx: &mut AsyncApp,
) -> Result<(u16, smol::process::Child)> {
    let binary_path = node_runtime
        .binary_path()
        .await
        .context("getting node path")?;
    let path = cx
        .spawn(async move |cx| get_or_install_companion(node_runtime, cx).await)
        .await?;
    log::info!("will launch js-debug-companion version {path:?}");

    let port = {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .context("getting port for companion")?;
        listener.local_addr()?.port()
    };

    let dir = paths::data_dir()
        .join("js_debug_companion_state")
        .to_string_lossy()
        .to_string();

    let child = new_smol_command(binary_path)
        .arg(path)
        .args([
            format!("--listen=127.0.0.1:{port}"),
            format!("--state={dir}"),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("spawning companion child process")?;

    Ok((port, child))
}

async fn get_or_install_companion(node: NodeRuntime, cx: &mut AsyncApp) -> Result<PathBuf> {
    const PACKAGE_NAME: &str = "@zed-industries/js-debug-companion-cli";

    async fn install_latest_version(dir: PathBuf, node: NodeRuntime) -> Result<PathBuf> {
        let temp_dir = tempfile::tempdir().context("creating temporary directory")?;
        node.npm_install_packages(temp_dir.path(), &[(PACKAGE_NAME, "latest")])
            .await
            .context("installing latest companion package")?;
        let version = node
            .npm_package_installed_version(temp_dir.path(), PACKAGE_NAME)
            .await
            .context("getting installed companion version")?
            .context("companion was not installed")?;
        let version_folder = dir.join(version.to_string());
        smol::fs::rename(temp_dir.path(), &version_folder)
            .await
            .context("moving companion package into place")?;
        Ok(version_folder)
    }

    let dir = paths::debug_adapters_dir().join("js-debug-companion");
    let (latest_installed_version, latest_version) = cx
        .background_spawn({
            let dir = dir.clone();
            let node = node.clone();
            async move {
                smol::fs::create_dir_all(&dir)
                    .await
                    .context("creating companion installation directory")?;

                let children = smol::fs::read_dir(&dir)
                    .await
                    .context("reading companion installation directory")?
                    .try_collect::<Vec<_>>()
                    .await
                    .context("reading companion installation directory entries")?;

                let latest_installed_version = children
                    .iter()
                    .filter_map(|child| {
                        Some((
                            child.path(),
                            semver::Version::parse(child.file_name().to_str()?).ok()?,
                        ))
                    })
                    .max_by_key(|(_, version)| version.clone());

                let latest_version = node
                    .npm_package_latest_version(PACKAGE_NAME)
                    .await
                    .log_err();
                anyhow::Ok((latest_installed_version, latest_version))
            }
        })
        .await?;

    let path = if let Some((installed_path, installed_version)) = latest_installed_version {
        if let Some(latest_version) = latest_version
            && latest_version != installed_version
        {
            cx.background_spawn(install_latest_version(dir.clone(), node.clone()))
                .detach();
        }
        Ok(installed_path)
    } else {
        cx.background_spawn(install_latest_version(dir.clone(), node.clone()))
            .await
    };

    Ok(path?
        .join("node_modules")
        .join(PACKAGE_NAME)
        .join("out")
        .join("cli.js"))
}
