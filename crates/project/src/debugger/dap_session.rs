use super::dap_command::{
    self, ContinueCommand, DapCommand, DisconnectCommand, EvaluateCommand, NextCommand,
    PauseCommand, RestartCommand, RestartStackFrameCommand, ScopesCommand, SetVariableValueCommand,
    StepBackCommand, StepCommand, StepInCommand, StepOutCommand, TerminateCommand,
    TerminateThreadsCommand, VariablesCommand,
};
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashMap, IndexMap};
use dap::client::{DebugAdapterClient, DebugAdapterClientId};
use dap::requests::Request;
use dap::{
    Capabilities, ContinueArguments, EvaluateArgumentsContext, Module, Source, SteppingGranularity,
};
use futures::{future::Shared, FutureExt};
use gpui::{App, AppContext, Context, Entity, Task};
use rpc::AnyProtoClient;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::btree_map::Entry as BTreeMapEntry;
use std::u64;
use std::{
    any::Any,
    collections::hash_map::Entry,
    hash::{Hash, Hasher},
    sync::Arc,
};
use task::DebugAdapterConfig;
use text::{PointUtf16, ToPointUtf16};
use util::ResultExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugSessionId(pub usize);

impl DebugSessionId {
    pub fn from_proto(session_id: u64) -> Self {
        Self(session_id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct ThreadId(pub u64);

impl ThreadId {
    pub const MIN: ThreadId = ThreadId(u64::MIN);
    pub const MAX: ThreadId = ThreadId(u64::MAX);
}

#[derive(Clone)]
pub struct Variable {
    dap: dap::Variable,
    variables: Vec<Variable>,
}

impl From<dap::Variable> for Variable {
    fn from(dap: dap::Variable) -> Self {
        Self {
            dap,
            variables: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Scope {
    pub dap: dap::Scope,
    pub variables: Vec<Variable>,
}

impl From<dap::Scope> for Scope {
    fn from(scope: dap::Scope) -> Self {
        Self {
            dap: scope,
            variables: vec![],
        }
    }
}

#[derive(Clone)]
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

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum ThreadStatus {
    #[default]
    Running,
    Stopped,
    Exited,
    Ended,
}

pub struct Thread {
    dap: dap::Thread,
    stack_frames: Vec<StackFrame>,
    _status: ThreadStatus,
    _has_stopped: bool,
}

impl From<dap::Thread> for Thread {
    fn from(dap: dap::Thread) -> Self {
        Self {
            dap,
            stack_frames: vec![],
            _status: ThreadStatus::default(),
            _has_stopped: false,
        }
    }
}

type UpstreamProjectId = u64;

pub struct RemoteConnection {
    client: AnyProtoClient,
    upstream_project_id: UpstreamProjectId,
}

impl RemoteConnection {
    fn send_proto_client_request<R: DapCommand>(
        &self,
        request: R,
        client_id: DebugAdapterClientId,
        cx: &mut App,
    ) -> Task<Result<R::Response>> {
        let message = request.to_proto(client_id, self.upstream_project_id);
        let upstream_client = self.client.clone();
        cx.background_executor().spawn(async move {
            let response = upstream_client.request(message).await?;
            request.response_from_proto(response)
        })
    }
    fn request_remote<R: DapCommand>(
        &self,
        request: R,
        client_id: DebugAdapterClientId,
        cx: &mut App,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        return self.send_proto_client_request::<R>(request, client_id, cx);
    }
}

pub enum Mode {
    Local(Arc<DebugAdapterClient>),
    Remote(RemoteConnection),
}

impl From<RemoteConnection> for Mode {
    fn from(value: RemoteConnection) -> Self {
        Self::Remote(value)
    }
}

impl From<Arc<DebugAdapterClient>> for Mode {
    fn from(client: Arc<DebugAdapterClient>) -> Self {
        Mode::Local(client)
    }
}

impl Mode {
    fn request_local<R: DapCommand>(
        connection: &Arc<DebugAdapterClient>,
        caps: &Capabilities,
        request: R,
        cx: &mut Context<Client>,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        if !request.is_supported(&caps) {
            return Task::ready(Err(anyhow!(
                "Request {} is not supported",
                R::DapRequest::COMMAND
            )));
        }

        let request = Arc::new(request);

        let request_clone = request.clone();
        let connection = connection.clone();
        let request_task = cx.background_executor().spawn(async move {
            let args = request_clone.to_dap();
            connection.request::<R::DapRequest>(args).await
        });

        cx.background_executor().spawn(async move {
            let response = request.response_from_dap(request_task.await?);
            response
        })
    }

    fn request_dap<R: DapCommand>(
        &self,
        caps: &Capabilities,
        client_id: DebugAdapterClientId,
        request: R,
        cx: &mut Context<Client>,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static + Send,
    {
        match self {
            Mode::Local(debug_adapter_client) => {
                Self::request_local(&debug_adapter_client, caps, request, cx)
            }
            Mode::Remote(remote_connection) => {
                remote_connection.request_remote(request, client_id, cx)
            }
        }
    }
}

/// Represents a current state of a single debug adapter and provides ways to mutate it.
pub struct Client {
    mode: Mode,

    pub(super) capabilities: Capabilities,
    client_id: DebugAdapterClientId,
    modules: Vec<dap::Module>,
    loaded_sources: Vec<dap::Source>,
    threads: IndexMap<ThreadId, Thread>,
    requests: HashMap<RequestSlot, Shared<Task<Option<()>>>>,
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
        self.0.as_any().type_id().hash(state);
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

impl Client {
    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

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
        if let Entry::Vacant(vacant) = self.requests.entry(request.into()) {
            let command = vacant.key().0.clone().as_any_arc().downcast::<T>().unwrap();

            let task = Self::request_inner::<Arc<T>>(
                &self.capabilities,
                self.client_id,
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
        client_id: DebugAdapterClientId,
        mode: &Mode,
        request: T,
        process_result: impl FnOnce(&mut Self, &T::Response, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) -> Task<Option<T::Response>> {
        let request = mode.request_dap(&capabilities, client_id, request, cx);
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
            self.client_id,
            &self.mode,
            request,
            process_result,
            cx,
        )
    }

    pub fn invalidate(&mut self, cx: &mut Context<Self>) {
        self.requests.clear();
        self.modules.clear();
        self.loaded_sources.clear();
        cx.notify();
    }

    pub fn threads(&mut self, cx: &mut Context<Self>) -> Vec<dap::Thread> {
        self.fetch(
            dap_command::ThreadsCommand,
            |this, result, cx| {
                this.threads.extend(
                    result
                        .iter()
                        .map(|thread| (ThreadId(thread.id), Thread::from(thread.clone()))),
                );
            },
            cx,
        );
        self.threads
            .values()
            .map(|thread| thread.dap.clone())
            .collect()
    }

    pub fn modules(&mut self, cx: &mut Context<Self>) -> &[Module] {
        self.fetch(
            dap_command::ModulesCommand,
            |this, result, cx| {
                this.modules = result.clone();
                cx.notify();
            },
            cx,
        );
        &self.modules
    }

    pub fn handle_module_event(&mut self, event: &dap::ModuleEvent, cx: &mut Context<Self>) {
        match event.reason {
            dap::ModuleEventReason::New => self.modules.push(event.module.clone()),
            dap::ModuleEventReason::Changed => {
                if let Some(module) = self.modules.iter_mut().find(|m| m.id == event.module.id) {
                    *module = event.module.clone();
                }
            }
            dap::ModuleEventReason::Removed => self.modules.retain(|m| m.id != event.module.id),
        }
        cx.notify();
    }

    pub fn loaded_sources(&mut self, cx: &mut Context<Self>) -> &[Source] {
        self.fetch(
            dap_command::LoadedSourcesCommand,
            |this, result, cx| {
                this.loaded_sources = result.clone();
                cx.notify();
            },
            cx,
        );
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

    fn shutdown(&mut self, cx: &mut Context<Self>) {
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
            Mode::Local(ref adapter_client) => Some(adapter_client.clone()),
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
        event: &dap::LoadedSourceEvent,
        cx: &mut Context<Self>,
    ) {
        match event.reason {
            dap::LoadedSourceEventReason::New => self.loaded_sources.push(event.source.clone()),
            dap::LoadedSourceEventReason::Changed => {
                let updated_source =
                    if let Some(ref_id) = event.source.source_reference.filter(|&r| r != 0) {
                        self.loaded_sources
                            .iter_mut()
                            .find(|s| s.source_reference == Some(ref_id))
                    } else if let Some(path) = &event.source.path {
                        self.loaded_sources
                            .iter_mut()
                            .find(|s| s.path.as_ref() == Some(path))
                    } else {
                        self.loaded_sources
                            .iter_mut()
                            .find(|s| s.name == event.source.name)
                    };

                if let Some(loaded_source) = updated_source {
                    *loaded_source = event.source.clone();
                }
            }
            dap::LoadedSourceEventReason::Removed => {
                self.loaded_sources.retain(|source| *source != event.source)
            }
        }
        cx.notify();
    }

    pub fn stack_frames(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) -> Vec<StackFrame> {
        self.fetch(
            super::dap_command::StackTraceCommand {
                thread_id: thread_id.0,
                start_frame: None,
                levels: None,
            },
            move |this, stack_frames, cx| {
                let entry = this.threads.entry(thread_id).and_modify(|thread| {
                    thread.stack_frames = stack_frames.iter().cloned().map(From::from).collect();
                });
                debug_assert!(
                    matches!(entry, indexmap::map::Entry::Occupied(_)),
                    "Sent request for thread_id that doesn't exist"
                );

                cx.notify();
            },
            cx,
        );

        self.threads
            .get(&thread_id)
            .map(|thread| thread.stack_frames.clone())
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
                    if let Some(stack_frame) = thread
                        .stack_frames
                        .iter_mut()
                        .find(|frame| frame.dap.id == stack_frame_id)
                    {
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
                thread.stack_frames.iter().find_map(|stack_frame| {
                    (stack_frame.dap.id == stack_frame_id).then(|| stack_frame.scopes.clone())
                })
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
            let stack_frame = thread
                .stack_frames
                .iter_mut()
                .find(|stack_frame| (stack_frame.dap.id == stack_frame_id))?;
            stack_frame
                .scopes
                .iter_mut()
                .find(|scope| scope.dap.variables_reference == variables_reference)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn variables(
        &mut self,
        thread_id: ThreadId,
        stack_frame_id: u64,
        session_id: DebugSessionId,
        variables_reference: u64,
        cx: &mut Context<Self>,
    ) -> Vec<Variable> {
        let command = VariablesCommand {
            stack_frame_id,
            session_id,
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
                    // This is only valid if scope.variable[x].ref_id == variables_reference
                    // otherwise we have to search the tree for the right index to add variables too
                    // todo(debugger): Fix this ^
                    scope.variables = variables.iter().cloned().map(From::from).collect();
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
            |this, _response, cx| {
                this.invalidate(cx);
            },
            cx,
        )
        .detach()
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
}

pub struct DebugSession {
    id: DebugSessionId,
    mode: DebugSessionMode,
    pub(super) states: BTreeMap<DebugAdapterClientId, Entity<Client>>,
    ignore_breakpoints: bool,
}

pub enum DebugSessionMode {
    Local(LocalDebugSession),
    Remote(RemoteDebugSession),
}

pub struct LocalDebugSession {
    configuration: DebugAdapterConfig,
}

impl LocalDebugSession {
    pub fn configuration(&self) -> &DebugAdapterConfig {
        &self.configuration
    }

    pub fn update_configuration(
        &mut self,
        f: impl FnOnce(&mut DebugAdapterConfig),
        cx: &mut Context<DebugSession>,
    ) {
        f(&mut self.configuration);
        cx.notify();
    }
}

pub struct RemoteDebugSession {
    label: String,
}

impl DebugSession {
    pub fn new_local(id: DebugSessionId, configuration: DebugAdapterConfig) -> Self {
        Self {
            id,
            ignore_breakpoints: false,
            states: BTreeMap::default(),
            mode: DebugSessionMode::Local(LocalDebugSession { configuration }),
        }
    }

    pub fn as_local(&self) -> Option<&LocalDebugSession> {
        match &self.mode {
            DebugSessionMode::Local(local) => Some(local),
            _ => None,
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalDebugSession> {
        match &mut self.mode {
            DebugSessionMode::Local(local) => Some(local),
            _ => None,
        }
    }

    pub fn new_remote(id: DebugSessionId, label: String, ignore_breakpoints: bool) -> Self {
        Self {
            id,
            ignore_breakpoints,
            states: BTreeMap::default(),
            mode: DebugSessionMode::Remote(RemoteDebugSession { label }),
        }
    }

    pub fn id(&self) -> DebugSessionId {
        self.id
    }

    pub fn name(&self) -> String {
        match &self.mode {
            DebugSessionMode::Local(local) => local.configuration.label.clone(),
            DebugSessionMode::Remote(remote) => remote.label.clone(),
        }
    }

    pub fn ignore_breakpoints(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn set_ignore_breakpoints(&mut self, ignore: bool, cx: &mut Context<Self>) {
        self.ignore_breakpoints = ignore;
        cx.notify();
    }

    pub fn client_state(&self, client_id: DebugAdapterClientId) -> Option<Entity<Client>> {
        self.states.get(&client_id).cloned()
    }

    pub(super) fn client_ids(&self) -> impl Iterator<Item = DebugAdapterClientId> + '_ {
        self.states.keys().copied()
    }

    pub fn clients(&self, cx: &App) -> Vec<Arc<DebugAdapterClient>> {
        self.states
            .values()
            .filter_map(|state| state.read(cx).adapter_client())
            .collect()
    }

    pub fn add_client(
        &mut self,
        client: impl Into<Mode>,
        client_id: DebugAdapterClientId,
        cx: &mut Context<DebugSession>,
    ) {
        if !self.states.contains_key(&client_id) {
            let mode = client.into();
            let state = cx.new(|_cx| Client {
                client_id,
                modules: Vec::default(),
                loaded_sources: Vec::default(),
                threads: IndexMap::default(),
                requests: HashMap::default(),
                capabilities: Default::default(),
                mode,
            });

            self.states.insert(client_id, state);
        }
    }

    pub(crate) fn client_by_id(
        &self,
        client_id: impl Borrow<DebugAdapterClientId>,
    ) -> Option<Entity<Client>> {
        self.states.get(client_id.borrow()).cloned()
    }

    pub(crate) fn shutdown_client(
        &mut self,
        client_id: DebugAdapterClientId,
        cx: &mut Context<Self>,
    ) {
        if let Some(client) = self.states.remove(&client_id) {
            client.update(cx, |this, cx| {
                this.shutdown(cx);
            })
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn clients_len(&self) -> usize {
        self.states.len()
    }
}
