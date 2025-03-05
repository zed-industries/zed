use crate::project_settings::ProjectSettings;

use super::breakpoint_store::{BreakpointStore, BreakpointStoreEvent};
use super::dap_command::{
    self, ConfigurationDone, ContinueCommand, DapCommand, DisconnectCommand, EvaluateCommand,
    Initialize, Launch, LoadedSourcesCommand, LocalDapCommand, LocationsCommand, ModulesCommand,
    NextCommand, PauseCommand, RestartCommand, RestartStackFrameCommand, ScopesCommand,
    SetVariableValueCommand, StackTraceCommand, StepBackCommand, StepCommand, StepInCommand,
    StepOutCommand, TerminateCommand, TerminateThreadsCommand, ThreadsCommand, VariablesCommand,
};
use super::dap_store::DapAdapterDelegate;
use anyhow::{anyhow, Result};
use collections::{HashMap, IndexMap, IndexSet};
use dap::adapters::{DebugAdapter, DebugAdapterBinary};
use dap::OutputEventCategory;
use dap::{
    adapters::{DapDelegate, DapStatus},
    client::{DebugAdapterClient, SessionId},
    messages::{Events, Message},
    Capabilities, ContinueArguments, EvaluateArgumentsContext, Module, Source, SourceBreakpoint,
    StackFrameId, SteppingGranularity, StoppedEvent, VariableReference,
};
use dap_adapters::build_adapter;
use futures::channel::oneshot;
use futures::{future::Shared, FutureExt};
use gpui::{
    App, AppContext, AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, Task, WeakEntity,
};
use rpc::AnyProtoClient;
use serde_json::{json, Value};
use settings::Settings;
use smol::stream::StreamExt;
use std::any::TypeId;
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
    Exited,
    Ended,
}

#[derive(Debug)]
pub struct Thread {
    dap: dap::Thread,
    stack_frame_ids: IndexSet<StackFrameId>,
    _has_stopped: bool,
}

impl From<dap::Thread> for Thread {
    fn from(dap: dap::Thread) -> Self {
        Self {
            dap,
            stack_frame_ids: Default::default(),
            _has_stopped: false,
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
pub struct LocalMode {
    client: Arc<DebugAdapterClient>,
    config: DebugAdapterConfig,
    breakpoint_store: Entity<BreakpointStore>,
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
        breakpoint_store: Entity<BreakpointStore>,
        config: DebugAdapterConfig,
        delegate: DapAdapterDelegate,
        messages_tx: futures::channel::mpsc::UnboundedSender<Message>,
        cx: AsyncApp,
    ) -> Task<Result<(Self, Capabilities)>> {
        cx.spawn(move |mut cx| async move {
            let (adapter, binary) = Self::get_adapter_binary(&config, delegate, &mut cx).await?;

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

            #[cfg(any(test, feature = "test-support"))]
            {
                client
                    .on_request::<dap::requests::Initialize, _>(move |_, _| {
                        Ok(dap::Capabilities {
                            supports_step_back: Some(false),
                            ..Default::default()
                        })
                    })
                    .await;

                client
                    .on_request::<dap::requests::Launch, _>(move |_, _| Ok(()))
                    .await;

                client.fake_event(Events::Initialized(None)).await;
            }

            let session = Self {
                client,
                config,
                breakpoint_store: breakpoint_store.clone(),
            };

            Self::initialize(session, adapter, initialized_rx, &mut cx).await
        })
    }

    fn send_breakpoints(
        &mut self,
        ignore_breakpoints: bool,
        last_updated_path: Option<Arc<Path>>,
        cx: &mut App,
    ) -> Task<std::vec::Vec<Result<std::vec::Vec<dap::Breakpoint>>>> {
        let mut breakpoint_tasks = Vec::new();
        let mut breakpoints = self
            .breakpoint_store
            .update(cx, |store, cx| store.all_breakpoints(cx));

        if let Some(last_updated_path) = last_updated_path {
            breakpoints.entry(last_updated_path).or_default();
        }

        for (path, breakpoints) in breakpoints {
            let breakpoints = if ignore_breakpoints {
                vec![]
            } else {
                breakpoints
                    .into_iter()
                    .map(|bp| SourceBreakpoint {
                        line: bp.position as u64 + 1,
                        column: None,
                        condition: None,
                        hit_condition: None,
                        log_message: bp.kind.log_message().as_deref().map(Into::into),
                        mode: None,
                    })
                    .collect()
            };

            breakpoint_tasks.push(self.request(
                dap_command::SetBreakpoints {
                    source: client_source(&path),
                    breakpoints,
                },
                cx.background_executor().clone(),
            ));
        }

        let task = futures::future::join_all(breakpoint_tasks);
        cx.background_spawn(task)
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

        let mut raw = adapter.request_args(&this.config);
        merge_json_value_into(
            this.config.initialize_args.clone().unwrap_or(json!({})),
            &mut raw,
        );

        // Of relevance: https://github.com/microsoft/vscode/issues/4902#issuecomment-368583522
        let launch = this.request(Launch { raw }, cx.background_executor().clone());
        let that = this.clone();

        let configuration_done_supported = ConfigurationDone::is_supported(&capabilities);
        let configuration_sequence = async move {
            let _ = initialized_rx.await?;

            cx.update(|cx| that.clone().send_breakpoints(false, None, cx))?
                .await;

            if configuration_done_supported {
                that.request(ConfigurationDone, cx.background_executor().clone())
                    .await?;
            }

            anyhow::Ok(())
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

    fn thread_exited(&mut self, thread_id: ThreadId) {
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

/// Represents a current state of a single debug adapter and provides ways to mutate it.
pub struct Session {
    mode: Mode,
    pub(super) capabilities: Capabilities,
    id: SessionId,
    parent_id: Option<SessionId>,
    ignore_breakpoints: bool,
    modules: Vec<dap::Module>,
    loaded_sources: Vec<dap::Source>,
    last_processed_output: usize,
    output: Vec<dap::OutputEvent>,
    threads: IndexMap<ThreadId, Thread>,
    variables: HashMap<VariableReference, Vec<dap::Variable>>,
    stack_frames: IndexMap<StackFrameId, StackFrame>,
    locations: HashMap<u64, dap::LocationsResponse>,
    thread_states: ThreadStates,
    is_session_terminated: bool,
    requests: HashMap<TypeId, HashMap<RequestSlot, Shared<Task<Option<()>>>>>,
    _background_tasks: Vec<Task<()>>,
}

trait CacheableCommand: 'static + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, rhs: &dyn CacheableCommand) -> bool;
    fn dyn_hash(&self, hasher: &mut dyn Hasher);
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
    fn cacheable_command_id(&self) -> TypeId;
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

    fn cacheable_command_id(&self) -> TypeId {
        T::command_id()
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

pub enum SessionEvent {
    Modules,
    LoadedSources,
    Stopped,
    StackTrace,
    Variables,
}

impl EventEmitter<SessionEvent> for Session {}

// local session will send breakpoint updates to DAP for all new breakpoints
// remote side will only send breakpoint updates when it is a breakpoint created by that peer
// BreakpointStore notifies session on breakpoint changes
impl Session {
    pub(crate) fn local(
        breakpoint_store: Entity<BreakpointStore>,
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
                breakpoint_store.clone(),
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

                cx.subscribe(&breakpoint_store, |this, _, event, cx| match event {
                    BreakpointStoreEvent::BreakpointsUpdated(path) => {
                        let ignore = this.ignore_breakpoints;
                        if let Some(local) = this.as_local_mut() {
                            local
                                .send_breakpoints(ignore, Some(path.clone()), cx)
                                .detach();
                        };
                    }
                    BreakpointStoreEvent::ActiveDebugLineChanged => {}
                })
                .detach();

                Self {
                    mode: Mode::Local(mode),
                    id: session_id,
                    parent_id: parent_session.map(|session| session.read(cx).id),
                    variables: Default::default(),
                    capabilities,
                    thread_states: ThreadStates::default(),
                    ignore_breakpoints: false,
                    last_processed_output: 0,
                    output: Vec::default(),
                    requests: HashMap::default(),
                    modules: Vec::default(),
                    loaded_sources: Vec::default(),
                    threads: IndexMap::default(),
                    stack_frames: IndexMap::default(),
                    locations: Default::default(),
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
            ignore_breakpoints,
            variables: Default::default(),
            stack_frames: Default::default(),
            thread_states: ThreadStates::default(),
            last_processed_output: 0,
            output: Vec::default(),
            requests: HashMap::default(),
            modules: Vec::default(),
            loaded_sources: Vec::default(),
            threads: IndexMap::default(),
            _background_tasks: Vec::default(),
            locations: Default::default(),
            is_session_terminated: false,
        }
    }

    pub fn session_id(&self) -> SessionId {
        self.id
    }

    pub fn parent_id(&self) -> Option<SessionId> {
        self.parent_id
    }

    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    pub fn configuration(&self) -> Option<DebugAdapterConfig> {
        if let Mode::Local(local_mode) = &self.mode {
            Some(local_mode.config.clone())
        } else {
            None
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.is_session_terminated
    }

    pub fn is_local(&self) -> bool {
        matches!(self.mode, Mode::Local(_))
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalMode> {
        match &mut self.mode {
            Mode::Local(local_mode) => Some(local_mode),
            Mode::Remote(_) => None,
        }
    }

    pub fn as_local(&self) -> Option<&LocalMode> {
        match &self.mode {
            Mode::Local(local_mode) => Some(local_mode),
            Mode::Remote(_) => None,
        }
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

    pub(crate) fn respond_to_client(
        &self,
        response: dap::messages::Response,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(local_session) = self.as_local().cloned() else {
            unreachable!("Cannot respond to remote client");
        };

        cx.background_spawn(async move {
            local_session
                .client
                .send_message(Message::Response(response))
                .await
        })
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

        self.threads
            .values_mut()
            .for_each(|thread| thread.stack_frame_ids.clear());

        self.invalidate_command_type(TypeId::of::<dap_command::GenericCommand>());
        cx.emit(SessionEvent::Stopped);
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
                    self.thread_states.all_threads_continued();
                } else {
                    self.thread_states
                        .thread_continued(ThreadId(event.thread_id));
                }
                self.invalidate_command_type(TypeId::of::<dap_command::GenericCommand>());
            }
            Events::Exited(_event) => {}
            Events::Terminated(_) => {
                self.is_session_terminated = true;
            }
            Events::Thread(event) => {
                let thread_id = ThreadId(event.thread_id);

                match event.reason {
                    dap::ThreadEventReason::Started => {
                        self.thread_states.thread_continued(thread_id);
                    }
                    dap::ThreadEventReason::Exited => {
                        self.thread_states.thread_exited(thread_id);
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

                self.output.push(event);
                cx.notify();
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

        if !self.thread_states.any_stopped_thread() {
            return;
        }

        let request_map = self.requests.entry(T::command_id()).or_default();

        if let Entry::Vacant(vacant) = request_map.entry(request.into()) {
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
            cx.notify();
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

    fn invalidate_command_type(&mut self, key: TypeId) {
        self.requests
            .entry(key)
            .and_modify(|request_map| request_map.clear());
    }

    fn invalidate_state(&mut self, key: &RequestSlot) {
        self.requests
            .entry(key.0.cacheable_command_id())
            .and_modify(|request_map| {
                request_map.remove(&key);
            });
    }

    /// This function should be called after changing state not before
    fn invalidate(&mut self, cx: &mut Context<Self>) {
        self.requests.clear();
        cx.notify();
    }

    pub fn thread_status(&self, thread_id: ThreadId) -> ThreadStatus {
        self.thread_states.thread_status(thread_id)
    }

    pub fn threads(&mut self, cx: &mut Context<Self>) -> Vec<(dap::Thread, ThreadStatus)> {
        self.fetch(
            dap_command::ThreadsCommand,
            |this, result, cx| {
                this.threads = result
                    .iter()
                    .map(|thread| (ThreadId(thread.id), Thread::from(thread.clone())))
                    .collect();

                this.invalidate_command_type(StackTraceCommand::command_id());
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
        self.fetch(
            dap_command::ModulesCommand,
            |this, result, cx| {
                this.modules = result.iter().cloned().collect();
                cx.emit(SessionEvent::Modules);
                cx.notify();
            },
            cx,
        );

        &self.modules
    }

    pub fn ignore_breakpoints(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &App) -> Task<Result<()>> {
        self.set_ignore_breakpoints(!self.ignore_breakpoints, cx)
    }

    pub(crate) fn set_ignore_breakpoints(&mut self, ignore: bool, _cx: &App) -> Task<Result<()>> {
        if self.ignore_breakpoints == ignore {
            return Task::ready(Err(anyhow!(
                "Can't set ignore breakpoint to state it's already at"
            )));
        }
        self.ignore_breakpoints = ignore;
        // todo(debugger): We need to propagate this change to downstream sessions and send a message to upstream sessions
        todo!();
        /*
                let mut tasks: Vec<Task<()>> = Vec::new();
        >>>>>>> debugger

                for (_abs_path, serialized_breakpoints) in self
                    .breakpoint_store
                    .read_with(cx, |store, cx| store.all_breakpoints(true, cx))
                    .into_iter()
                {
                    let _source_breakpoints = if self.ignore_breakpoints {
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
                })*/
    }

    pub fn breakpoints_enabled(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn loaded_sources(&mut self, cx: &mut Context<Self>) -> &[Source] {
        self.fetch(
            dap_command::LoadedSourcesCommand,
            |this, result, cx| {
                this.loaded_sources = result.iter().cloned().collect();
                cx.emit(SessionEvent::LoadedSources);
                cx.notify();
            },
            cx,
        );

        &self.loaded_sources
    }

    fn empty_response(&mut self, _: &(), _cx: &mut Context<Self>) {}

    fn clear_active_debug_line(&mut self, _: &(), cx: &mut Context<'_, Session>) {
        self.as_local()
            .expect("Message handler will only run in local mode")
            .breakpoint_store
            .update(cx, |store, cx| store.set_active_position(None, cx))
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

    pub fn shutdown(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let task = if self
            .capabilities
            .supports_terminate_request
            .unwrap_or_default()
        {
            self.request(
                TerminateCommand {
                    restart: Some(false),
                },
                Self::clear_active_debug_line,
                cx,
            )
        } else {
            self.request(
                DisconnectCommand {
                    restart: Some(false),
                    terminate_debuggee: Some(true),
                    suspend_debuggee: Some(false),
                },
                Self::clear_active_debug_line,
                cx,
            )
        };

        cx.background_spawn(async move {
            let _ = task.await;
        })
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

    pub fn stack_frames(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) -> Vec<StackFrame> {
        if self.thread_states.thread_status(thread_id) == ThreadStatus::Stopped
            && self.requests.contains_key(&ThreadsCommand::command_id())
        {
            self.fetch(
                super::dap_command::StackTraceCommand {
                    thread_id: thread_id.0,
                    start_frame: None,
                    levels: None,
                },
                move |this, stack_frames, cx| {
                    let entry = this.threads.entry(thread_id).and_modify(|thread| {
                        thread.stack_frame_ids =
                            stack_frames.iter().map(|frame| frame.id).collect();
                    });
                    debug_assert!(
                        matches!(entry, indexmap::map::Entry::Occupied(_)),
                        "Sent request for thread_id that doesn't exist"
                    );

                    this.stack_frames.extend(
                        stack_frames
                            .into_iter()
                            .cloned()
                            .map(|frame| (frame.id, StackFrame::from(frame))),
                    );

                    this.invalidate_command_type(ScopesCommand::command_id());
                    this.invalidate_command_type(VariablesCommand::command_id());

                    cx.emit(SessionEvent::StackTrace);
                    cx.notify();
                },
                cx,
            );
        }

        self.threads
            .get(&thread_id)
            .map(|thread| {
                thread
                    .stack_frame_ids
                    .iter()
                    .filter_map(|id| self.stack_frames.get(id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn scopes(&mut self, stack_frame_id: u64, cx: &mut Context<Self>) -> &[dap::Scope] {
        if self.requests.contains_key(&ThreadsCommand::command_id())
            && self
                .requests
                .contains_key(&dap_command::StackTraceCommand::command_id())
        {
            self.fetch(
                ScopesCommand { stack_frame_id },
                move |this, scopes, cx| {

                    for scope in scopes {
                        this.variables(scope.variables_reference, cx);
                    }

                    let entry = this
                        .stack_frames
                        .entry(stack_frame_id)
                        .and_modify(|stack_frame| {
                            stack_frame.scopes = scopes.clone();
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

        self.stack_frames
            .get(&stack_frame_id)
            .map(|frame| frame.scopes.as_slice())
            .unwrap_or_default()
    }

    #[allow(clippy::too_many_arguments)]
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
                this.variables
                    .insert(variables_reference, variables.clone());

                cx.emit(SessionEvent::Variables);
            },
            cx,
        );

        self.variables
            .get(&variables_reference)
            .cloned()
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
                    location_reference: None,
                });

                // TODO(debugger): only invalidate variables & scopes
                this.invalidate(cx);
            },
            cx,
        )
        .detach();
    }

    pub fn location(
        &mut self,
        reference: u64,
        cx: &mut Context<Self>,
    ) -> Option<dap::LocationsResponse> {
        self.fetch(
            LocationsCommand { reference },
            move |this, response, _| {
                this.locations.insert(reference, response.clone());
            },
            cx,
        );
        self.locations.get(&reference).cloned()
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
                Self::clear_active_debug_line,
                cx,
            )
            .detach();
        } else {
            self.shutdown(cx).detach();
        }
    }
}
