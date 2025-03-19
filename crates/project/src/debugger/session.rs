use crate::project_settings::ProjectSettings;

use super::breakpoint_store::{BreakpointStore, BreakpointStoreEvent, BreakpointUpdatedReason};
use super::dap_command::{
    self, Attach, ConfigurationDone, ContinueCommand, DapCommand, DisconnectCommand,
    EvaluateCommand, Initialize, Launch, LoadedSourcesCommand, LocalDapCommand, LocationsCommand,
    ModulesCommand, NextCommand, PauseCommand, RestartCommand, RestartStackFrameCommand,
    ScopesCommand, SetVariableValueCommand, StackTraceCommand, StepBackCommand, StepCommand,
    StepInCommand, StepOutCommand, TerminateCommand, TerminateThreadsCommand, ThreadsCommand,
    VariablesCommand,
};
use super::dap_store::DapAdapterDelegate;
use anyhow::{anyhow, Result};
use collections::{HashMap, IndexMap, IndexSet};
use dap::adapters::{DebugAdapter, DebugAdapterBinary};
use dap::messages::Response;
use dap::OutputEventCategory;
use dap::{
    adapters::{DapDelegate, DapStatus},
    client::{DebugAdapterClient, SessionId},
    messages::{Events, Message},
    Capabilities, ContinueArguments, EvaluateArgumentsContext, Module, Source, StackFrameId,
    SteppingGranularity, StoppedEvent, VariableReference,
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

impl From<u64> for ThreadId {
    fn from(id: u64) -> Self {
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
    _client: AnyProtoClient,
    _upstream_project_id: UpstreamProjectId,
}

impl RemoteConnection {
    fn send_proto_client_request<R: DapCommand>(
        &self,
        _request: R,
        _session_id: SessionId,
        cx: &mut App,
    ) -> Task<Result<R::Response>> {
        // let message = request.to_proto(session_id, self.upstream_project_id);
        // let upstream_client = self.client.clone();
        cx.background_executor().spawn(async move {
            // debugger(todo): Properly send messages when we wrap dap_commands in envelopes again
            // let response = upstream_client.request(message).await?;
            // request.response_from_proto(response)
            Err(anyhow!("Sending dap commands over RPC isn't supported yet"))
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
    adapter: Arc<dyn DebugAdapter>,
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
            let (adapter, binary) = Self::get_adapter_binary(&config, &delegate, &mut cx).await?;

            let message_handler = Box::new(move |message| {
                messages_tx.unbounded_send(message).ok();
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
                    DebugAdapterClient::start(
                        session_id,
                        adapter.name(),
                        binary,
                        message_handler,
                        cx.clone(),
                    )
                    .await?
                },
            );

            let adapter_id = adapter.name().to_string().to_owned();
            let session = Self {
                client,
                adapter,
                breakpoint_store,
                config: config.clone(),
            };

            #[cfg(any(test, feature = "test-support"))]
            {
                let dap::DebugAdapterKind::Fake((fail, caps)) = session.config.kind.clone() else {
                    panic!("Only fake debug adapter configs should be used in tests");
                };

                session
                    .client
                    .on_request::<dap::requests::Initialize, _>(move |_, _| Ok(caps.clone()))
                    .await;

                match config.request.clone() {
                    dap::DebugRequestType::Launch if fail => {
                        session
                            .client
                            .on_request::<dap::requests::Launch, _>(move |_, _| {
                                Err(dap::ErrorResponse {
                                    error: Some(dap::Message {
                                        id: 1,
                                        format: "error".into(),
                                        variables: None,
                                        send_telemetry: None,
                                        show_user: None,
                                        url: None,
                                        url_label: None,
                                    }),
                                })
                            })
                            .await;
                    }
                    dap::DebugRequestType::Launch => {
                        session
                            .client
                            .on_request::<dap::requests::Launch, _>(move |_, _| Ok(()))
                            .await;
                    }
                    dap::DebugRequestType::Attach(_) if fail => {
                        session
                            .client
                            .on_request::<dap::requests::Attach, _>(move |_, _| {
                                Err(dap::ErrorResponse {
                                    error: Some(dap::Message {
                                        id: 1,
                                        format: "error".into(),
                                        variables: None,
                                        send_telemetry: None,
                                        show_user: None,
                                        url: None,
                                        url_label: None,
                                    }),
                                })
                            })
                            .await;
                    }
                    dap::DebugRequestType::Attach(attach_config) => {
                        session
                            .client
                            .on_request::<dap::requests::Attach, _>(move |_, args| {
                                assert_eq!(
                                    json!({"request": "attach", "process_id": attach_config.process_id.unwrap()}),
                                    args.raw
                                );

                                Ok(())
                            })
                            .await;
                    }
                }

                session.client.on_request::<dap::requests::Disconnect, _>(move |_, _| Ok(())).await;
                session.client.fake_event(Events::Initialized(None)).await;
            }

            let capabilities = session
                .request(Initialize { adapter_id }, cx.background_executor().clone())
                .await?;

            Ok((session, capabilities))
        })
    }

    fn send_breakpoints_from_path(
        &self,
        abs_path: Arc<Path>,
        reason: BreakpointUpdatedReason,
        cx: &mut App,
    ) -> Task<()> {
        let breakpoints = self
            .breakpoint_store
            .read_with(cx, |store, cx| store.breakpoints_from_path(&abs_path, cx))
            .into_iter()
            .map(Into::into)
            .collect();

        let task = self.request(
            dap_command::SetBreakpoints {
                source: client_source(&abs_path),
                source_modified: Some(matches!(reason, BreakpointUpdatedReason::FileSaved)),
                breakpoints,
            },
            cx.background_executor().clone(),
        );

        cx.background_spawn(async move {
            match task.await {
                Ok(_) => {}
                Err(err) => log::warn!("Set breakpoints request failed for path: {}", err),
            }
        })
    }

    fn send_all_breakpoints(&self, ignore_breakpoints: bool, cx: &App) -> Task<()> {
        let mut breakpoint_tasks = Vec::new();
        let breakpoints = self
            .breakpoint_store
            .read_with(cx, |store, cx| store.all_breakpoints(cx));

        for (path, breakpoints) in breakpoints {
            let breakpoints = if ignore_breakpoints {
                vec![]
            } else {
                breakpoints.into_iter().map(Into::into).collect()
            };

            breakpoint_tasks.push(self.request(
                dap_command::SetBreakpoints {
                    source: client_source(&path),
                    source_modified: Some(false),
                    breakpoints,
                },
                cx.background_executor().clone(),
            ));
        }

        cx.background_spawn(async move {
            futures::future::join_all(breakpoint_tasks)
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

    async fn get_adapter_binary(
        config: &DebugAdapterConfig,
        delegate: &DapAdapterDelegate,
        cx: &mut AsyncApp,
    ) -> Result<(Arc<dyn DebugAdapter>, DebugAdapterBinary)> {
        let adapter = build_adapter(&config.kind).await?;

        let binary = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .dap
                .get(&adapter.name())
                .and_then(|s| s.binary.as_ref().map(PathBuf::from))
        })?;

        let binary = match adapter.get_binary(delegate, &config, binary, cx).await {
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

    pub fn initialize_sequence(
        &self,
        capabilities: &Capabilities,
        initialized_rx: oneshot::Receiver<()>,
        cx: &App,
    ) -> Task<Result<()>> {
        let mut raw = self.adapter.request_args(&self.config);
        merge_json_value_into(
            self.config.initialize_args.clone().unwrap_or(json!({})),
            &mut raw,
        );

        // Of relevance: https://github.com/microsoft/vscode/issues/4902#issuecomment-368583522
        let launch = match &self.config.request {
            dap::DebugRequestType::Launch => {
                self.request(Launch { raw }, cx.background_executor().clone())
            }
            dap::DebugRequestType::Attach(_) => {
                self.request(Attach { raw }, cx.background_executor().clone())
            }
        };

        let configuration_done_supported = ConfigurationDone::is_supported(capabilities);

        let configuration_sequence = cx.spawn({
            let this = self.clone();
            move |cx| async move {
                initialized_rx.await?;
                // todo(debugger) figure out if we want to handle a breakpoint response error
                // This will probably consist of letting a user know that breakpoints failed to be set
                cx.update(|cx| this.send_all_breakpoints(false, cx))?.await;

                if configuration_done_supported {
                    this.request(ConfigurationDone, cx.background_executor().clone())
                } else {
                    Task::ready(Ok(()))
                }
                .await
            }
        });

        cx.background_spawn(async move {
            futures::future::try_join(launch, configuration_sequence).await?;
            Ok(())
        })
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
    fn stop_all_threads(&mut self) {
        self.global_state = Some(ThreadStatus::Stopped);
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
const MAX_TRACKED_OUTPUT_EVENTS: usize = 5000;

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputToken(pub usize);
/// Represents a current state of a single debug adapter and provides ways to mutate it.
pub struct Session {
    mode: Mode,
    pub(super) capabilities: Capabilities,
    id: SessionId,
    parent_id: Option<SessionId>,
    ignore_breakpoints: bool,
    modules: Vec<dap::Module>,
    loaded_sources: Vec<dap::Source>,
    output_token: OutputToken,
    output: Box<circular_buffer::CircularBuffer<MAX_TRACKED_OUTPUT_EVENTS, dap::OutputEvent>>,
    threads: IndexMap<ThreadId, Thread>,
    thread_states: ThreadStates,
    variables: HashMap<VariableReference, Vec<dap::Variable>>,
    stack_frames: IndexMap<StackFrameId, StackFrame>,
    locations: HashMap<u64, dap::LocationsResponse>,
    is_session_terminated: bool,
    requests: HashMap<TypeId, HashMap<RequestSlot, Shared<Task<Option<()>>>>>,
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

pub enum SessionEvent {
    Modules,
    LoadedSources,
    Stopped(Option<ThreadId>),
    StackTrace,
    Variables,
    Threads,
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
        initialized_tx: oneshot::Sender<()>,
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
                        let mut initialized_tx = Some(initialized_tx);
                        while let Some(message) = message_rx.next().await {
                            if let Message::Event(event) = message {
                                if let Events::Initialized(_) = *event {
                                    if let Some(tx) = initialized_tx.take() {
                                        tx.send(()).ok();
                                    }
                                } else {
                                    let Ok(_) = this.update(&mut cx, |session, cx| {
                                        session.handle_dap_event(event, cx);
                                    }) else {
                                        break;
                                    };
                                }
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
                    BreakpointStoreEvent::BreakpointsUpdated(path, reason) => {
                        if let Some(local) = (!this.ignore_breakpoints)
                            .then(|| this.as_local_mut())
                            .flatten()
                        {
                            local
                                .send_breakpoints_from_path(path.clone(), *reason, cx)
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
                    output_token: OutputToken(0),
                    ignore_breakpoints: false,
                    output: circular_buffer::CircularBuffer::boxed(),
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
                _client: client,
                _upstream_project_id: upstream_project_id,
            }),
            id: session_id,
            parent_id: None,
            capabilities: Capabilities::default(),
            ignore_breakpoints,
            variables: Default::default(),
            stack_frames: Default::default(),
            thread_states: ThreadStates::default(),

            output_token: OutputToken(0),
            output: circular_buffer::CircularBuffer::boxed(),
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

    pub(super) fn initialize_sequence(
        &mut self,
        initialize_rx: oneshot::Receiver<()>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        match &self.mode {
            Mode::Local(local_mode) => {
                local_mode.initialize_sequence(&self.capabilities, initialize_rx, cx)
            }
            Mode::Remote(_) => Task::ready(Err(anyhow!("cannot initialize remote session"))),
        }
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
        let Some(local_session) = self.as_local().cloned() else {
            unreachable!("Cannot respond to remote client");
        };

        cx.background_spawn(async move {
            local_session
                .client
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

    fn handle_stopped_event(&mut self, event: StoppedEvent, cx: &mut Context<Self>) {
        if event.all_threads_stopped.unwrap_or_default() || event.thread_id.is_none() {
            self.thread_states.stop_all_threads();

            self.invalidate_command_type::<StackTraceCommand>();
        }

        // Event if we stopped all threads we still need to insert the thread_id
        // to our own data
        if let Some(thread_id) = event.thread_id {
            self.thread_states.stop_thread(ThreadId(thread_id));

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
        self.threads.clear();
        self.variables.clear();
        cx.emit(SessionEvent::Stopped(
            event
                .thread_id
                .map(Into::into)
                .filter(|_| !event.preserve_focus_hint.unwrap_or(false)),
        ));
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
                    self.thread_states.continue_all_threads();
                } else {
                    self.thread_states
                        .continue_thread(ThreadId(event.thread_id));
                }
                // todo(debugger): We should be able to get away with only invalidating generic if all threads were continued
                self.invalidate_generic();
            }
            Events::Exited(_event) => {
                self.clear_active_debug_line(cx);
            }
            Events::Terminated(_) => {
                self.is_session_terminated = true;
                self.clear_active_debug_line(cx);
            }
            Events::Thread(event) => {
                let thread_id = ThreadId(event.thread_id);

                match event.reason {
                    dap::ThreadEventReason::Started => {
                        self.thread_states.continue_thread(thread_id);
                    }
                    dap::ThreadEventReason::Exited => {
                        self.thread_states.exit_thread(thread_id);
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

                self.output.push_back(event);
                self.output_token.0 += 1;
                cx.notify();
            }
            Events::Breakpoint(_) => {}
            Events::Module(event) => {
                match event.reason {
                    dap::ModuleEventReason::New => {
                        self.modules.push(event.module);
                    }
                    dap::ModuleEventReason::Changed => {
                        if let Some(module) = self
                            .modules
                            .iter_mut()
                            .find(|other| event.module.id == other.id)
                        {
                            *module = event.module;
                        }
                    }
                    dap::ModuleEventReason::Removed => {
                        self.modules.retain(|other| event.module.id != other.id);
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
        process_result: impl FnOnce(&mut Self, Result<T::Response>, &mut Context<Self>) -> Option<T::Response>
            + 'static,
        cx: &mut Context<Self>,
    ) {
        const {
            assert!(
                T::CACHEABLE,
                "Only requests marked as cacheable should invoke `fetch`"
            );
        }

        if !self.thread_states.any_stopped_thread()
            && request.type_id() != TypeId::of::<ThreadsCommand>()
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
        process_result: impl FnOnce(&mut Self, Result<T::Response>, &mut Context<Self>) -> Option<T::Response>
            + 'static,
        cx: &mut Context<Self>,
    ) -> Task<Option<T::Response>> {
        if !T::is_supported(&capabilities) {
            log::warn!(
                "Attempted to send a DAP request that isn't supported: {:?}",
                request
            );
            let error = Err(anyhow::Error::msg(
                "Couldn't complete request because it's not supported",
            ));
            return cx.spawn(|this, mut cx| async move {
                this.update(&mut cx, |this, cx| process_result(this, error, cx))
                    .log_err()
                    .flatten()
            });
        }

        let request = mode.request_dap(session_id, request, cx);
        cx.spawn(|this, mut cx| async move {
            let result = request.await;
            this.update(&mut cx, |this, cx| process_result(this, result, cx))
                .log_err()
                .flatten()
        })
    }

    fn request<T: DapCommand + PartialEq + Eq + Hash>(
        &self,
        request: T,
        process_result: impl FnOnce(&mut Self, Result<T::Response>, &mut Context<Self>) -> Option<T::Response>
            + 'static,
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

    fn invalidate_command_type<Command: DapCommand>(&mut self) {
        self.requests.remove(&std::any::TypeId::of::<Command>());
    }

    fn invalidate_generic(&mut self) {
        self.invalidate_command_type::<ModulesCommand>();
        self.invalidate_command_type::<LoadedSourcesCommand>();
        self.invalidate_command_type::<ThreadsCommand>();
    }

    fn invalidate_state(&mut self, key: &RequestSlot) {
        self.requests
            .entry(key.0.as_any().type_id())
            .and_modify(|request_map| {
                request_map.remove(&key);
            });
    }

    pub fn thread_status(&self, thread_id: ThreadId) -> ThreadStatus {
        self.thread_states.thread_status(thread_id)
    }

    pub fn threads(&mut self, cx: &mut Context<Self>) -> Vec<(dap::Thread, ThreadStatus)> {
        self.fetch(
            dap_command::ThreadsCommand,
            |this, result, cx| {
                let result = result.log_err()?;

                this.threads = result
                    .iter()
                    .map(|thread| (ThreadId(thread.id), Thread::from(thread.clone())))
                    .collect();

                this.invalidate_command_type::<StackTraceCommand>();
                cx.emit(SessionEvent::Threads);
                cx.notify();

                Some(result)
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
                let result = result.log_err()?;

                this.modules = result.iter().cloned().collect();
                cx.emit(SessionEvent::Modules);
                cx.notify();

                Some(result)
            },
            cx,
        );

        &self.modules
    }

    pub fn ignore_breakpoints(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &mut App) -> Task<()> {
        self.set_ignore_breakpoints(!self.ignore_breakpoints, cx)
    }

    pub(crate) fn set_ignore_breakpoints(&mut self, ignore: bool, cx: &mut App) -> Task<()> {
        if self.ignore_breakpoints == ignore {
            return Task::ready(());
        }

        self.ignore_breakpoints = ignore;

        if let Some(local) = self.as_local() {
            local.send_all_breakpoints(ignore, cx)
        } else {
            // todo(debugger): We need to propagate this change to downstream sessions and send a message to upstream sessions
            unimplemented!()
        }
    }

    pub fn breakpoints_enabled(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn loaded_sources(&mut self, cx: &mut Context<Self>) -> &[Source] {
        self.fetch(
            dap_command::LoadedSourcesCommand,
            |this, result, cx| {
                let result = result.log_err()?;
                this.loaded_sources = result.iter().cloned().collect();
                cx.emit(SessionEvent::LoadedSources);
                cx.notify();
                Some(result)
            },
            cx,
        );

        &self.loaded_sources
    }

    fn empty_response(&mut self, res: Result<()>, _cx: &mut Context<Self>) -> Option<()> {
        res.log_err()?;
        Some(())
    }

    fn on_step_response<T: DapCommand + PartialEq + Eq + Hash>(
        thread_id: ThreadId,
    ) -> impl FnOnce(&mut Self, Result<T::Response>, &mut Context<Self>) -> Option<T::Response> + 'static
    {
        move |this, response, cx| match response.log_err() {
            Some(response) => Some(response),
            None => {
                this.thread_states.stop_thread(thread_id);
                cx.notify();
                None
            }
        }
    }

    fn clear_active_debug_line_response(
        &mut self,
        response: Result<()>,
        cx: &mut Context<'_, Session>,
    ) -> Option<()> {
        response.log_err()?;
        self.clear_active_debug_line(cx);
        Some(())
    }

    fn clear_active_debug_line(&mut self, cx: &mut Context<Session>) {
        self.as_local()
            .expect("Message handler will only run in local mode")
            .breakpoint_store
            .update(cx, |store, cx| {
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
        let task = self.request(query, |_, result, _| result.log_err(), cx);

        cx.background_executor().spawn(async move {
            anyhow::Ok(
                task.await
                    .map(|response| response.targets)
                    .ok_or_else(|| anyhow!("failed to fetch completions"))?,
            )
        })
    }

    pub fn continue_thread(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        self.thread_states.continue_thread(thread_id);
        self.request(
            ContinueCommand {
                args: ContinueArguments {
                    thread_id: thread_id.0,
                    single_thread: Some(true),
                },
            },
            Self::on_step_response::<ContinueCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn adapter_client(&self) -> Option<Arc<DebugAdapterClient>> {
        match self.mode {
            Mode::Local(ref local) => Some(local.client.clone()),
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

        self.thread_states.process_step(thread_id);
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

        self.thread_states.process_step(thread_id);
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

        self.thread_states.process_step(thread_id);
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

        self.thread_states.process_step(thread_id);

        self.request(
            command,
            Self::on_step_response::<StepBackCommand>(thread_id),
            cx,
        )
        .detach();
    }

    pub fn stack_frames(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) -> Vec<StackFrame> {
        if self.thread_states.thread_status(thread_id) == ThreadStatus::Stopped
            && self.requests.contains_key(&ThreadsCommand.type_id())
            && self.threads.contains_key(&thread_id)
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
                    let stack_frames = stack_frames.log_err()?;

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
                            .iter()
                            .cloned()
                            .map(|frame| (frame.id, StackFrame::from(frame))),
                    );

                    this.invalidate_command_type::<ScopesCommand>();
                    this.invalidate_command_type::<VariablesCommand>();

                    cx.emit(SessionEvent::StackTrace);
                    cx.notify();
                    Some(stack_frames)
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
        if self.requests.contains_key(&TypeId::of::<ThreadsCommand>())
            && self
                .requests
                .contains_key(&TypeId::of::<StackTraceCommand>())
        {
            self.fetch(
                ScopesCommand { stack_frame_id },
                move |this, scopes, cx| {
                    let scopes = scopes.log_err()?;

                    for scope in scopes .iter(){
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

                    Some(scopes)
                },
                cx,
            );
        }

        self.stack_frames
            .get(&stack_frame_id)
            .map(|frame| frame.scopes.as_slice())
            .unwrap_or_default()
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
                let variables = variables.log_err()?;
                this.variables
                    .insert(variables_reference, variables.clone());

                cx.emit(SessionEvent::Variables);
                Some(variables)
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
                move |this, response, cx| {
                    let response = response.log_err()?;
                    this.invalidate_command_type::<VariablesCommand>();
                    cx.notify();
                    Some(response)
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
                let response = response.log_err()?;
                this.output.push_back(dap::OutputEvent {
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

                this.invalidate_command_type::<ScopesCommand>();
                cx.notify();
                Some(response)
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
                let response = response.log_err()?;
                this.locations.insert(reference, response.clone());
                Some(response)
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
                Self::clear_active_debug_line_response,
                cx,
            )
            .detach();
        } else {
            self.shutdown(cx).detach();
        }
    }
}
