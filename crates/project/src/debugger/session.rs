use crate::project_settings::ProjectSettings;

use super::breakpoint_store::{BreakpointStore, BreakpointStoreEvent};
use super::dap_command::{
    self, ConfigurationDone, ContinueCommand, DapCommand, DisconnectCommand, EvaluateCommand,
    Initialize, Launch, LoadedSourcesCommand, LocalDapCommand, ModulesCommand, NextCommand,
    PauseCommand, RestartCommand, RestartStackFrameCommand, ScopesCommand, SetVariableValueCommand,
    StackTraceCommand, StepBackCommand, StepCommand, StepInCommand, StepOutCommand,
    TerminateCommand, TerminateThreadsCommand, ThreadsCommand, VariablesCommand,
};
use super::dap_store::DapAdapterDelegate;
use anyhow::{anyhow, Result};
use collections::{HashMap, IndexMap};
use dap::adapters::{DebugAdapter, DebugAdapterBinary};
use dap::{
    adapters::{DapDelegate, DapStatus, DebugAdapterName},
    client::{DebugAdapterClient, SessionId},
    messages::{self, Events, Message},
    requests::SetBreakpoints,
    Capabilities, ContinueArguments, EvaluateArgumentsContext, Module, SetBreakpointsArguments,
    Source, SourceBreakpoint, SteppingGranularity, StoppedEvent,
};
use dap::{DebugAdapterKind, StartDebuggingRequestArguments};
use dap_adapters::build_adapter;
use futures::channel::oneshot;
use futures::{future::join_all, future::Shared, FutureExt};
use gpui::{App, AppContext, AsyncApp, BackgroundExecutor, Context, Entity, Task, WeakEntity};
use rpc::AnyProtoClient;
use serde_json::{json, Value};
use settings::Settings;
use smol::stream::StreamExt;
use std::path::PathBuf;
use std::u64;
use std::{
    any::Any,
    collections::hash_map::Entry,
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};
use task::DebugAdapterConfig;
use text::{PointUtf16, ToPointUtf16};
use util::{merge_json_value_into, ResultExt};

#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct ThreadId(pub u64);

impl ThreadId {
    pub const MIN: ThreadId = ThreadId(u64::MIN);
    pub const MAX: ThreadId = ThreadId(u64::MAX);
}

pub enum VariableListContainer {
    Scope(Scope),
    Variable(Variable),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToggledState {
    Toggled,
    UnToggled,
    Leaf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub dap: dap::Variable,
    pub toggled_state: ToggledState,
    pub depth: u8,
}

impl From<dap::Variable> for Variable {
    fn from(dap: dap::Variable) -> Self {
        Self {
            toggled_state: if dap.variables_reference == 0 {
                ToggledState::Leaf
            } else {
                ToggledState::UnToggled
            },
            dap,
            depth: 2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub dap: dap::Scope,
    pub variables: Vec<Variable>,
    pub is_toggled: bool,
}

impl From<dap::Scope> for Scope {
    fn from(scope: dap::Scope) -> Self {
        Self {
            dap: scope,
            variables: vec![],
            is_toggled: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    pub dap: dap::StackFrame,
    pub scopes: Vec<Scope>,
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
    Exited,
    Ended,
}

type StackFrameId = u64;

#[derive(Debug)]
pub struct Thread {
    dap: dap::Thread,
    stack_frames: IndexMap<StackFrameId, StackFrame>,
    has_stopped: bool,
}

impl From<dap::Thread> for Thread {
    fn from(dap: dap::Thread) -> Self {
        Self {
            dap,
            stack_frames: IndexMap::new(),
            has_stopped: false,
        }
    }
}

type UpstreamProjectId = u64;

struct RemoteConnection {
    client: AnyProtoClient,
    upstream_project_id: UpstreamProjectId,
}

impl RemoteConnection {
    fn send_proto_client_request<R: DapCommand>(
        &self,
        request: R,
        session_id: SessionId,
        cx: &mut App,
    ) -> Task<Result<R::Response>> {
        let message = request.to_proto(session_id, self.upstream_project_id);
        let upstream_client = self.client.clone();
        cx.background_executor().spawn(async move {
            let response = upstream_client.request(message).await?;
            request.response_from_proto(response)
        })
    }
    fn request<R: DapCommand>(
        &self,
        request: R,
        session_id: SessionId,
        cx: &mut App,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        return self.send_proto_client_request::<R>(request, session_id, cx);
    }
}

enum Mode {
    Local(LocalMode),
    Remote(RemoteConnection),
}

#[derive(Clone)]
struct LocalMode {
    client: Arc<DebugAdapterClient>,
}

enum ReverseRequest {
    RunInTerminal(),
}

fn client_source(abs_path: &Path) -> dap::Source {
    dap::Source {
        name: abs_path
            .file_name()
            .map(|filename| filename.to_string_lossy().to_string()),
        path: Some(abs_path.to_string_lossy().to_string()),
        source_reference: None,
        presentation_hint: None,
        origin: None,
        sources: None,
        adapter_data: None,
        checksums: None,
    }
}

impl LocalMode {
    fn new(
        session_id: SessionId,
        parent_session: Option<Entity<Session>>,
        breakpoints: Entity<BreakpointStore>,
        disposition: DebugAdapterConfig,
        delegate: DapAdapterDelegate,
        messages_tx: futures::channel::mpsc::UnboundedSender<Message>,
        cx: AsyncApp,
    ) -> Task<Result<(Self, Capabilities)>> {
        cx.spawn(move |mut cx| async move {
            let (adapter, binary) =
                Self::get_adapter_binary(&disposition, delegate, &mut cx).await?;

            let (initialized_tx, initialized_rx) = oneshot::channel();
            let mut initialized_tx = Some(initialized_tx);
            let message_handler = Box::new(move |message, _cx: &mut App| {
                let Message::Event(event) = &message else {
                    messages_tx.unbounded_send(message).ok();
                    return;
                };
                if let Events::Initialized(_) = **event {
                    if let Some(tx) = initialized_tx.take() {
                        tx.send(()).ok();
                    }
                } else {
                    messages_tx.unbounded_send(message).ok();
                }
            });

            let client = Arc::new(
                if let Some(client) = parent_session
                    .and_then(|session| cx.update(|cx| session.read(cx).adapter_client()).ok())
                    .flatten()
                {
                    client
                        .reconnect(session_id, binary, message_handler, cx.clone())
                        .await?
                } else {
                    DebugAdapterClient::start(session_id, binary, message_handler, cx.clone())
                        .await?
                },
            );
            let session = Self { client };

            Self::initialize(
                session,
                adapter,
                &disposition,
                breakpoints,
                initialized_rx,
                &mut cx,
            )
            .await
        })
    }

    async fn get_adapter_binary(
        disposition: &DebugAdapterConfig,
        delegate: DapAdapterDelegate,
        cx: &mut AsyncApp,
    ) -> Result<(Arc<dyn DebugAdapter>, DebugAdapterBinary)> {
        let adapter = build_adapter(&disposition.kind).await?;

        let binary = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .dap
                .get(&adapter.name())
                .and_then(|s| s.binary.as_ref().map(PathBuf::from))
        })?;

        let binary = match adapter
            .get_binary(&delegate, &disposition, binary, cx)
            .await
        {
            Err(error) => {
                delegate.update_status(
                    adapter.name(),
                    DapStatus::Failed {
                        error: error.to_string(),
                    },
                );

                return Err(error);
            }
            Ok(mut binary) => {
                delegate.update_status(adapter.name(), DapStatus::None);

                let shell_env = delegate.shell_env().await;
                let mut envs = binary.envs.unwrap_or_default();
                envs.extend(shell_env);
                binary.envs = Some(envs);

                binary
            }
        };

        Ok((adapter, binary))
    }

    async fn initialize(
        this: Self,
        adapter: Arc<dyn DebugAdapter>,
        disposition: &DebugAdapterConfig,
        breakpoints: Entity<BreakpointStore>,
        initialized_rx: oneshot::Receiver<()>,
        cx: &mut AsyncApp,
    ) -> Result<(Self, Capabilities)> {
        let capabilities = this
            .request(
                Initialize {
                    adapter_id: adapter.name().to_string().to_owned(),
                },
                cx.background_executor().clone(),
            )
            .await?;

        let mut raw = adapter.request_args(disposition);
        merge_json_value_into(
            disposition.initialize_args.clone().unwrap_or(json!({})),
            &mut raw,
        );

        // Of relevance: https://github.com/microsoft/vscode/issues/4902#issuecomment-368583522
        let launch = this.request(Launch { raw }, cx.background_executor().clone());
        let that = this.clone();
        let breakpoints = breakpoints.update(cx, |this, cx| this.all_breakpoints(true, cx))?;

        let configuration_done_supported = ConfigurationDone::is_supported(&capabilities);
        let configuration_sequence = async move {
            let _ = initialized_rx.await?;

            let mut breakpoint_tasks = Vec::new();

            for (path, breakpoints) in breakpoints.iter() {
                breakpoint_tasks.push(
                    that.request(
                        dap_command::SetBreakpoints {
                            source: client_source(&path),
                            breakpoints: breakpoints
                                .iter()
                                .map(|breakpoint| breakpoint.to_source_breakpoint())
                                .collect(),
                        },
                        cx.background_executor().clone(),
                    ),
                );
            }
            let _ = futures::future::join_all(breakpoint_tasks).await;

            if configuration_done_supported {
                that.request(ConfigurationDone, cx.background_executor().clone())
                    .await?;
            }

            anyhow::Result::<_, anyhow::Error>::Ok(())
        };

        let _ = futures::future::join(configuration_sequence, launch).await;

        Ok((this, capabilities))
    }

    fn request<R: LocalDapCommand>(
        &self,
        request: R,
        executor: BackgroundExecutor,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        let request = Arc::new(request);

        let request_clone = request.clone();
        let connection = self.client.clone();
        let request_task = executor.spawn(async move {
            let args = request_clone.to_dap();
            connection.request::<R::DapRequest>(args).await
        });

        executor.spawn(async move {
            let response = request.response_from_dap(request_task.await?);
            response
        })
    }
}
impl From<RemoteConnection> for Mode {
    fn from(value: RemoteConnection) -> Self {
        Self::Remote(value)
    }
}

impl Mode {
    fn request_dap<R: DapCommand>(
        &self,
        session_id: SessionId,
        request: R,
        cx: &mut Context<Session>,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        match self {
            Mode::Local(debug_adapter_client) => {
                debug_adapter_client.request(request, cx.background_executor().clone())
            }
            Mode::Remote(remote_connection) => remote_connection.request(request, session_id, cx),
        }
    }
}

#[derive(Default)]
struct ThreadStates {
    global_state: Option<ThreadStatus>,
    known_thread_states: IndexMap<ThreadId, ThreadStatus>,
}

impl ThreadStates {
    fn all_threads_stopped(&mut self) {
        self.global_state = Some(ThreadStatus::Stopped);
        self.known_thread_states.clear();
    }

    fn all_threads_continued(&mut self) {
        self.global_state = Some(ThreadStatus::Running);
        self.known_thread_states.clear();
    }

    fn thread_stopped(&mut self, thread_id: ThreadId) {
        self.known_thread_states
            .insert(thread_id, ThreadStatus::Stopped);
    }

    fn thread_continued(&mut self, thread_id: ThreadId) {
        self.known_thread_states
            .insert(thread_id, ThreadStatus::Running);
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

    fn any_stopped_thread(&self) -> bool {
        self.global_state
            .is_some_and(|state| state == ThreadStatus::Stopped)
            || self
                .known_thread_states
                .values()
                .any(|status| *status == ThreadStatus::Stopped)
    }
}

type VariableId = u64;

/// Represents a current state of a single debug adapter and provides ways to mutate it.
pub struct Session {
    mode: Mode,
    config: DebugAdapterConfig,
    pub(super) capabilities: Capabilities,
    id: SessionId,
    parent_id: Option<SessionId>,
    breakpoint_store: Entity<BreakpointStore>,
    ignore_breakpoints: bool,
    modules: Vec<dap::Module>,
    loaded_sources: Vec<dap::Source>,
    last_processed_output: usize,
    output: Vec<dap::OutputEvent>,
    threads: IndexMap<ThreadId, Thread>,
    requests: HashMap<RequestSlot, Shared<Task<Option<()>>>>,
    variables: HashMap<VariableId, Vec<Variable>>,
    thread_states: ThreadStates,
    is_session_terminated: bool,
    _background_tasks: Vec<Task<()>>,
}

trait CacheableCommand: 'static + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, rhs: &dyn CacheableCommand) -> bool;
    fn dyn_hash(&self, hasher: &mut dyn Hasher);
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T> CacheableCommand for T
where
    T: DapCommand + PartialEq + Eq + Hash,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, rhs: &dyn CacheableCommand) -> bool {
        rhs.as_any()
            .downcast_ref::<Self>()
            .map_or(false, |rhs| self == rhs)
    }
    fn dyn_hash(&self, mut hasher: &mut dyn Hasher) {
        T::hash(self, &mut hasher);
    }
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

pub(crate) struct RequestSlot(Arc<dyn CacheableCommand>);

impl<T: DapCommand + PartialEq + Eq + Hash> From<T> for RequestSlot {
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
        self.0.as_any().type_id().hash(state)
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
// local session will send breakpoint updates to DAP for all new breakpoints
// remote side will only send breakpoint updates when it is a breakpoint created by that peer
// BreakpointStore notifies session on breakpoint changes
impl Session {
    pub(crate) fn local(
        breakpoints: Entity<BreakpointStore>,
        session_id: SessionId,
        parent_session: Option<Entity<Session>>,
        delegate: DapAdapterDelegate,
        config: DebugAdapterConfig,
        start_debugging_requests_tx: futures::channel::mpsc::UnboundedSender<(SessionId, Message)>,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let (message_tx, mut message_rx) = futures::channel::mpsc::unbounded();

        cx.spawn(move |mut cx| async move {
            let (mode, capabilities) = LocalMode::new(
                session_id,
                parent_session.clone(),
                breakpoints.clone(),
                config.clone(),
                delegate,
                message_tx,
                cx.clone(),
            )
            .await?;

            cx.new(|cx| {
                let _background_tasks =
                    vec![cx.spawn(move |this: WeakEntity<Self>, mut cx| async move {
                        while let Some(message) = message_rx.next().await {
                            if let Message::Event(event) = message {
                                let Ok(_) = this.update(&mut cx, |session, cx| {
                                    session.handle_dap_event(event, cx);
                                }) else {
                                    break;
                                };
                            } else {
                                let Ok(_) = start_debugging_requests_tx
                                    .unbounded_send((session_id, message))
                                else {
                                    break;
                                };
                            }
                        }
                    })];

                Self {
                    mode: Mode::Local(mode),
                    id: session_id,
                    parent_id: parent_session.map(|session| session.read(cx).id),
                    breakpoint_store: breakpoints,
                    variables: Default::default(),
                    config,
                    capabilities,
                    thread_states: ThreadStates::default(),
                    ignore_breakpoints: false,
                    last_processed_output: 0,
                    output: Vec::default(),
                    requests: HashMap::default(),
                    modules: Vec::default(),
                    loaded_sources: Vec::default(),
                    threads: IndexMap::default(),
                    _background_tasks,
                    is_session_terminated: false,
                }
            })
        })
    }

    pub(crate) fn remote(
        session_id: SessionId,
        client: AnyProtoClient,
        upstream_project_id: u64,
        breakpoint_store: Entity<BreakpointStore>,
        ignore_breakpoints: bool,
    ) -> Self {
        Self {
            mode: Mode::Remote(RemoteConnection {
                client,
                upstream_project_id,
            }),
            id: session_id,
            parent_id: None,
            capabilities: Capabilities::default(),
            breakpoint_store,
            ignore_breakpoints,
            variables: Default::default(),
            thread_states: ThreadStates::default(),
            last_processed_output: 0,
            output: Vec::default(),
            requests: HashMap::default(),
            modules: Vec::default(),
            loaded_sources: Vec::default(),
            threads: IndexMap::default(),
            _background_tasks: Vec::default(),
            config: todo!(),
            is_session_terminated: false,
        }
    }

    pub fn session_id(&self) -> SessionId {
        self.id
    }

    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    pub fn configuration(&self) -> DebugAdapterConfig {
        self.config.clone()
    }

    pub fn is_terminated(&self) -> bool {
        self.is_session_terminated
    }

    pub fn is_local(&self) -> bool {
        matches!(self.mode, Mode::Local(_))
    }

    pub fn output(&self) -> Vec<dap::OutputEvent> {
        self.output.iter().cloned().collect()
    }

    pub fn last_processed_output(&self) -> usize {
        self.last_processed_output
    }

    pub fn set_last_processed_output(&mut self, last_processed_output: usize) {
        self.last_processed_output = last_processed_output;
    }

    fn handle_stopped_event(&mut self, event: StoppedEvent, cx: &mut Context<Self>) {
        // todo(debugger): We should query for all threads here if we don't get a thread id
        // maybe in both cases too?
        if event.all_threads_stopped.unwrap_or_default() {
            self.thread_states.all_threads_stopped();
        } else if let Some(thread_id) = event.thread_id {
            self.thread_states.thread_stopped(ThreadId(thread_id));
        } else {
            // TODO(debugger): all threads should be stopped
        }

        // todo(debugger): We should see if we could only invalidate the thread that stopped
        // instead of everything right now.
        self.invalidate(cx);
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
                    self.thread_states.all_threads_continued();
                } else {
                    self.thread_states
                        .thread_continued(ThreadId(event.thread_id));
                }
                self.invalidate(cx);
            }
            Events::Exited(_event) => {}
            Events::Terminated(_) => {
                self.is_session_terminated = true;
            }
            Events::Thread(event) => {
                match event.reason {
                    dap::ThreadEventReason::Started => {
                        self.thread_states
                            .thread_continued(ThreadId(event.thread_id));
                    }
                    _ => {}
                }
                self.invalidate_state(&ThreadsCommand.into());
            }
            Events::Output(event) => {
                self.output.push(event);
            }
            Events::Breakpoint(_) => {}
            Events::Module(_) => {
                self.invalidate_state(&ModulesCommand.into());
            }
            Events::LoadedSource(_) => {
                self.invalidate_state(&LoadedSourcesCommand.into());
            }
            Events::Capabilities(event) => {
                self.capabilities = self.capabilities.merge(event.capabilities);
                cx.notify();
            }
            Events::Memory(_) => {}
            Events::Process(_) => {}
            Events::ProgressEnd(_) => {}
            Events::ProgressStart(_) => {}
            Events::ProgressUpdate(_) => {}
            Events::Invalidated(_) => {}
            Events::Other(_) => {}
        }
    }

    fn _handle_start_debugging_request(&mut self, _request: messages::Request) {}

    fn _handle_run_in_terminal_request(&mut self, _request: messages::Request) {}

    pub(crate) fn _wait_for_request<R: DapCommand + PartialEq + Eq + Hash>(
        &self,
        request: R,
    ) -> Option<Shared<Task<Option<()>>>> {
        let request_slot = RequestSlot::from(request);
        self.requests.get(&request_slot).cloned()
    }

    /// Ensure that there's a request in flight for the given command, and if not, send it. Use this to run requests that are idempotent.
    fn fetch<T: DapCommand + PartialEq + Eq + Hash>(
        &mut self,
        request: T,
        process_result: impl FnOnce(&mut Self, &T::Response, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) {
        const {
            assert!(
                T::CACHEABLE,
                "Only requests marked as cacheable should invoke `fetch`"
            );
        }
        if let Entry::Vacant(vacant) = self.requests.entry(request.into()) {
            let command = vacant.key().0.clone().as_any_arc().downcast::<T>().unwrap();

            let task = Self::request_inner::<Arc<T>>(
                &self.capabilities,
                self.id,
                &self.mode,
                command,
                process_result,
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
        }
    }

    fn request_inner<T: DapCommand + PartialEq + Eq + Hash>(
        capabilities: &Capabilities,
        session_id: SessionId,
        mode: &Mode,
        request: T,
        process_result: impl FnOnce(&mut Self, &T::Response, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) -> Task<Option<T::Response>> {
        if !T::is_supported(&capabilities) {
            return Task::ready(None);
        }
        let request = mode.request_dap(session_id, request, cx);
        cx.spawn(|this, mut cx| async move {
            let result = request.await.log_err()?;
            this.update(&mut cx, |this, cx| {
                process_result(this, &result, cx);
            })
            .log_err();
            Some(result)
        })
    }

    fn request<T: DapCommand + PartialEq + Eq + Hash>(
        &self,
        request: T,
        process_result: impl FnOnce(&mut Self, &T::Response, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) -> Task<Option<T::Response>> {
        Self::request_inner(
            &self.capabilities,
            self.id,
            &self.mode,
            request,
            process_result,
            cx,
        )
    }

    fn invalidate_state(&mut self, key: &RequestSlot) {
        self.requests.remove(&key);
    }

    /// This function should be called after changing state not before
    fn invalidate(&mut self, cx: &mut Context<Self>) {
        self.requests.clear();
        self.modules.clear();
        self.loaded_sources.clear();
        cx.notify();
    }

    pub fn thread_status(&self, thread_id: ThreadId) -> ThreadStatus {
        self.thread_states.thread_status(thread_id)
    }

    pub fn threads(&mut self, cx: &mut Context<Self>) -> Vec<(dap::Thread, ThreadStatus)> {
        self.fetch(
            dap_command::ThreadsCommand,
            |this, result, cx| {
                let v = this.threads.keys().copied().collect::<Vec<_>>();
                for thread_id in v {
                    this.invalidate_state(
                        &StackTraceCommand {
                            thread_id: thread_id.0,
                            start_frame: None,
                            levels: None,
                        }
                        .into(),
                    );
                }
                this.threads.extend(
                    result
                        .iter()
                        .map(|thread| (ThreadId(thread.id), Thread::from(thread.clone()))),
                );
                cx.notify();
            },
            cx,
        );
        self.threads
            .values()
            .map(|thread| {
                (
                    thread.dap.clone(),
                    self.thread_states.thread_status(ThreadId(thread.dap.id)),
                )
            })
            .collect()
    }

    pub fn modules(&mut self, cx: &mut Context<Self>) -> &[Module] {
        if self.thread_states.any_stopped_thread() {
            self.fetch(
                dap_command::ModulesCommand,
                |this, result, cx| {
                    this.modules = result.iter().cloned().collect();
                    cx.notify();
                },
                cx,
            );
        }

        &self.modules
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &App) -> Task<Result<()>> {
        self.set_ignore_breakpoints(!self.ignore_breakpoints, cx)
    }

    pub(crate) fn set_ignore_breakpoints(&mut self, ignore: bool, cx: &App) -> Task<Result<()>> {
        if self.ignore_breakpoints == ignore {
            return Task::ready(Err(anyhow!(
                "Can't set ignore breakpoint to state it's already at"
            )));
        }

        // todo(debugger): We need to propagate this change to downstream sessions and send a message to upstream sessions

        self.ignore_breakpoints = ignore;
        let mut tasks: Vec<Task<()>> = Vec::new();

        for (abs_path, serialized_breakpoints) in self
            .breakpoint_store
            .read_with(cx, |store, cx| store.all_breakpoints(true, cx))
            .into_iter()
        {
            let source_breakpoints = if self.ignore_breakpoints {
                serialized_breakpoints
                    .iter()
                    .map(|bp| bp.to_source_breakpoint())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            todo!(
                r#"tasks.push(self.send_breakpoints(
                abs_path,
                source_breakpoints,
                self.ignore_breakpoints,
                false,
                cx,
                ));"#
            );
        }

        cx.background_executor().spawn(async move {
            join_all(tasks).await;
            Ok(())
        })
    }

    pub fn breakpoints_enabled(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn loaded_sources(&mut self, cx: &mut Context<Self>) -> &[Source] {
        if self.thread_states.any_stopped_thread() {
            self.fetch(
                dap_command::LoadedSourcesCommand,
                |this, result, cx| {
                    this.loaded_sources = result.iter().cloned().collect();
                    cx.notify();
                },
                cx,
            );
        }

        &self.loaded_sources
    }

    fn empty_response(&mut self, _: &(), _cx: &mut Context<Self>) {}

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
        if self.capabilities.supports_restart_request.unwrap_or(false) {
            self.request(
                RestartCommand {
                    raw: args.unwrap_or(Value::Null),
                },
                Self::empty_response,
                cx,
            )
            .detach();
        } else {
            self.request(
                DisconnectCommand {
                    restart: Some(false),
                    terminate_debuggee: Some(true),
                    suspend_debuggee: Some(false),
                },
                Self::empty_response,
                cx,
            )
            .detach();
        }
    }

    pub(super) fn shutdown(&mut self, cx: &mut Context<Self>) {
        if self
            .capabilities
            .supports_terminate_request
            .unwrap_or_default()
        {
            self.request(
                TerminateCommand {
                    restart: Some(false),
                },
                Self::empty_response,
                cx,
            )
            .detach();
        } else {
            self.request(
                DisconnectCommand {
                    restart: Some(false),
                    terminate_debuggee: Some(true),
                    suspend_debuggee: Some(false),
                },
                Self::empty_response,
                cx,
            )
            .detach();
        }
    }

    pub fn completions(
        &mut self,
        query: CompletionsQuery,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<dap::CompletionItem>>> {
        let task = self.request(query, |_, _, _| {}, cx);

        cx.background_executor().spawn(async move {
            anyhow::Ok(
                task.await
                    .map(|response| response.targets)
                    .ok_or_else(|| anyhow!("failed to fetch completions"))?,
            )
        })
    }

    pub fn continue_thread(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        self.request(
            ContinueCommand {
                args: ContinueArguments {
                    thread_id: thread_id.0,
                    single_thread: Some(true),
                },
            },
            |_, _, _| {}, // todo: what do we do about the payload here?
            cx,
        )
        .detach();
    }

    pub fn adapter_client(&self) -> Option<Arc<DebugAdapterClient>> {
        match self.mode {
            Mode::Local(ref adapter_client) => Some(adapter_client.client.clone()),
            Mode::Remote(_) => None,
        }
    }

    pub fn step_over(
        &mut self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
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

        self.request(command, Self::empty_response, cx).detach();
    }

    pub fn step_in(
        &self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
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

        self.request(command, Self::empty_response, cx).detach();
    }

    pub fn step_out(
        &self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
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

        self.request(command, Self::empty_response, cx).detach();
    }

    pub fn step_back(
        &self,
        thread_id: ThreadId,
        granularity: SteppingGranularity,
        cx: &mut Context<Self>,
    ) {
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

        self.request(command, Self::empty_response, cx).detach();
    }

    pub fn handle_loaded_source_event(
        &mut self,
        _: &dap::LoadedSourceEvent,
        cx: &mut Context<Self>,
    ) {
        self.invalidate_state(&LoadedSourcesCommand.into());
        cx.notify();
    }

    pub fn stack_frames(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) -> Vec<StackFrame> {
        if self.thread_states.thread_status(thread_id) == ThreadStatus::Stopped {
            self.fetch(
                super::dap_command::StackTraceCommand {
                    thread_id: thread_id.0,
                    start_frame: None,
                    levels: None,
                },
                move |this, stack_frames, cx| {
                    let entry = this.threads.entry(thread_id).and_modify(|thread| {
                        thread.stack_frames = stack_frames
                            .iter()
                            .cloned()
                            .map(|frame| (frame.id, frame.into()))
                            .collect();
                    });
                    debug_assert!(
                        matches!(entry, indexmap::map::Entry::Occupied(_)),
                        "Sent request for thread_id that doesn't exist"
                    );

                    cx.notify();
                },
                cx,
            );
        }

        self.threads
            .get(&thread_id)
            .map(|thread| thread.stack_frames.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn scopes(
        &mut self,
        thread_id: ThreadId,
        stack_frame_id: u64,
        cx: &mut Context<Self>,
    ) -> Vec<Scope> {
        self.fetch(
            ScopesCommand {
                thread_id: thread_id.0,
                stack_frame_id,
            },
            move |this, scopes, cx| {
                this.threads.entry(thread_id).and_modify(|thread| {
                    if let Some(stack_frame) = thread.stack_frames.get_mut(&stack_frame_id) {
                        stack_frame.scopes = scopes.iter().cloned().map(From::from).collect();
                        cx.notify();
                    }
                });
            },
            cx,
        );
        self.threads
            .get(&thread_id)
            .and_then(|thread| {
                thread
                    .stack_frames
                    .get(&stack_frame_id)
                    .map(|stack_frame| stack_frame.scopes.clone())
            })
            .unwrap_or_default()
    }

    fn find_scope(
        &mut self,
        thread_id: ThreadId,
        stack_frame_id: u64,
        variables_reference: u64,
    ) -> Option<&mut Scope> {
        self.threads.get_mut(&thread_id).and_then(|thread| {
            thread
                .stack_frames
                .get_mut(&stack_frame_id)
                .and_then(|frame| {
                    frame
                        .scopes
                        .iter_mut()
                        .find(|scope| scope.dap.variables_reference == variables_reference)
                })
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn variables(
        &mut self,
        thread_id: ThreadId,
        stack_frame_id: u64,
        variables_reference: u64,
        cx: &mut Context<Self>,
    ) -> Vec<Variable> {
        let command = VariablesCommand {
            stack_frame_id,
            thread_id: thread_id.0,
            variables_reference,
            filter: None,
            start: None,
            count: None,
            format: None,
        };

        self.fetch(
            command,
            move |this, variables, cx| {
                if let Some(scope) = this.find_scope(thread_id, stack_frame_id, variables_reference)
                {
                    this.variables.insert(
                        variables_reference,
                        variables.iter().cloned().map(From::from).collect(),
                    );
                    cx.notify();
                }
            },
            cx,
        );

        self.find_scope(thread_id, stack_frame_id, variables_reference)
            .map(|scope| scope.variables.clone())
            .unwrap_or_default()
    }

    pub fn set_variable_value(
        &mut self,
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
                |this, _response, cx| {
                    this.invalidate(cx);
                },
                cx,
            )
            .detach()
        }
    }

    pub fn evaluate(
        &mut self,
        expression: String,
        context: Option<EvaluateArgumentsContext>,
        frame_id: Option<u64>,
        source: Option<Source>,
        cx: &mut Context<Self>,
    ) {
        self.request(
            EvaluateCommand {
                expression,
                context,
                frame_id,
                source,
            },
            |this, response, cx| {
                this.output.push(dap::OutputEvent {
                    category: None,
                    output: response.result.clone(),
                    group: None,
                    variables_reference: Some(response.variables_reference),
                    source: None,
                    line: None,
                    column: None,
                    data: None,
                });

                // TODO(debugger): only invalidate variables & scopes
                this.invalidate(cx);
            },
            cx,
        )
        .detach();
    }

    pub fn disconnect_client(&mut self, cx: &mut Context<Self>) {
        let command = DisconnectCommand {
            restart: Some(false),
            terminate_debuggee: Some(true),
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
                Self::empty_response,
                cx,
            )
            .detach();
        }
    }

    pub fn variable_list(
        &mut self,
        selected_thread_id: ThreadId,
        stack_frame_id: u64,
        cx: &mut Context<Self>,
    ) -> Vec<VariableListContainer> {
        self.scopes(selected_thread_id, stack_frame_id, cx)
            .iter()
            .cloned()
            .flat_map(|scope| {
                if scope.is_toggled {
                    self.variables(
                        selected_thread_id,
                        stack_frame_id,
                        scope.dap.variables_reference,
                        cx,
                    );
                }

                let mut stack = vec![scope.dap.variables_reference];
                let head = VariableListContainer::Scope(scope);
                let mut variables = vec![head];

                while let Some(reference) = stack.pop() {
                    if let Some(children) = self.variables.get(&reference) {
                        for variable in children {
                            if variable.toggled_state == ToggledState::Toggled {
                                stack.push(variable.dap.variables_reference);
                            }

                            variables.push(VariableListContainer::Variable(variable.clone()));
                        }
                    }
                }

                variables
            })
            .collect()
    }
}
