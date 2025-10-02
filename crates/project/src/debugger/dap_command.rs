use std::sync::Arc;

use anyhow::{Context as _, Ok, Result};
use base64::Engine;
use dap::{
    Capabilities, ContinueArguments, ExceptionFilterOptions, InitializeRequestArguments,
    InitializeRequestArgumentsPathFormat, NextArguments, SetVariableResponse, SourceBreakpoint,
    StepInArguments, StepOutArguments, SteppingGranularity, ValueFormat, Variable,
    VariablesArgumentsFilter,
    client::SessionId,
    proto_conversions::ProtoConversion,
    requests::{Continue, Next},
};

use rpc::proto;
use serde_json::Value;
use util::ResultExt;

pub trait LocalDapCommand: 'static + Send + Sync + std::fmt::Debug {
    type Response: 'static + Send + std::fmt::Debug;
    type DapRequest: 'static + Send + dap::requests::Request;
    /// Is this request idempotent? Is it safe to cache the response for as long as the execution environment is unchanged?
    const CACHEABLE: bool = false;

    fn is_supported(_capabilities: &Capabilities) -> bool {
        true
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments;

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response>;
}

pub trait DapCommand: LocalDapCommand {
    type ProtoRequest: 'static + Send;
    type ProtoResponse: 'static + Send;

    #[allow(dead_code)]
    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId;

    #[allow(dead_code)]
    fn from_proto(request: &Self::ProtoRequest) -> Self;

    #[allow(unused)]
    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest;

    #[allow(dead_code)]
    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse;

    #[allow(unused)]
    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response>;
}

impl<T: LocalDapCommand> LocalDapCommand for Arc<T> {
    type Response = T::Response;
    type DapRequest = T::DapRequest;

    fn is_supported(capabilities: &Capabilities) -> bool {
        T::is_supported(capabilities)
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        T::to_dap(self)
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        T::response_from_dap(self, message)
    }
}

impl<T: DapCommand> DapCommand for Arc<T> {
    type ProtoRequest = T::ProtoRequest;
    type ProtoResponse = T::ProtoResponse;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        T::client_id_from_proto(request)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Arc::new(T::from_proto(request))
    }

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        T::to_proto(self, debug_client_id, upstream_project_id)
    }

    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        T::response_to_proto(debug_client_id, message)
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        T::response_from_proto(self, message)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct StepCommand {
    pub thread_id: i64,
    pub granularity: Option<SteppingGranularity>,
    pub single_thread: Option<bool>,
}

impl StepCommand {
    fn from_proto(message: proto::DapNextRequest) -> Self {
        const LINE: i32 = proto::SteppingGranularity::Line as i32;
        const INSTRUCTION: i32 = proto::SteppingGranularity::Instruction as i32;

        let granularity = message.granularity.map(|granularity| match granularity {
            LINE => SteppingGranularity::Line,
            INSTRUCTION => SteppingGranularity::Instruction,
            _ => SteppingGranularity::Statement,
        });

        Self {
            thread_id: message.thread_id,
            granularity,
            single_thread: message.single_thread,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct NextCommand {
    pub inner: StepCommand,
}

impl LocalDapCommand for NextCommand {
    type Response = <Next as dap::requests::Request>::Response;
    type DapRequest = Next;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        NextArguments {
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity,
        }
    }
    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for NextCommand {
    type ProtoRequest = proto::DapNextRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            inner: StepCommand::from_proto(request.clone()),
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapNextRequest {
        proto::DapNextRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity.map(|gran| gran.to_proto() as i32),
        }
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct StepInCommand {
    pub inner: StepCommand,
}

impl LocalDapCommand for StepInCommand {
    type Response = <dap::requests::StepIn as dap::requests::Request>::Response;
    type DapRequest = dap::requests::StepIn;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        StepInArguments {
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            target_id: None,
            granularity: self.inner.granularity,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for StepInCommand {
    type ProtoRequest = proto::DapStepInRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            inner: StepCommand::from_proto(proto::DapNextRequest {
                project_id: request.project_id,
                client_id: request.client_id,
                thread_id: request.thread_id,
                single_thread: request.single_thread,
                granularity: request.granularity,
            }),
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapStepInRequest {
        proto::DapStepInRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity.map(|gran| gran.to_proto() as i32),
            target_id: None,
        }
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct StepOutCommand {
    pub inner: StepCommand,
}

impl LocalDapCommand for StepOutCommand {
    type Response = <dap::requests::StepOut as dap::requests::Request>::Response;
    type DapRequest = dap::requests::StepOut;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        StepOutArguments {
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for StepOutCommand {
    type ProtoRequest = proto::DapStepOutRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            inner: StepCommand::from_proto(proto::DapNextRequest {
                project_id: request.project_id,
                client_id: request.client_id,
                thread_id: request.thread_id,
                single_thread: request.single_thread,
                granularity: request.granularity,
            }),
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapStepOutRequest {
        proto::DapStepOutRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity.map(|gran| gran.to_proto() as i32),
        }
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct StepBackCommand {
    pub inner: StepCommand,
}
impl LocalDapCommand for StepBackCommand {
    type Response = <dap::requests::StepBack as dap::requests::Request>::Response;
    type DapRequest = dap::requests::StepBack;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_step_back.unwrap_or_default()
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::StepBackArguments {
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for StepBackCommand {
    type ProtoRequest = proto::DapStepBackRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            inner: StepCommand::from_proto(proto::DapNextRequest {
                project_id: request.project_id,
                client_id: request.client_id,
                thread_id: request.thread_id,
                single_thread: request.single_thread,
                granularity: request.granularity,
            }),
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapStepBackRequest {
        proto::DapStepBackRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.inner.thread_id,
            single_thread: self.inner.single_thread,
            granularity: self.inner.granularity.map(|gran| gran.to_proto() as i32),
        }
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct ContinueCommand {
    pub args: ContinueArguments,
}

impl LocalDapCommand for ContinueCommand {
    type Response = <Continue as dap::requests::Request>::Response;
    type DapRequest = Continue;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        self.args.clone()
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

impl DapCommand for ContinueCommand {
    type ProtoRequest = proto::DapContinueRequest;
    type ProtoResponse = proto::DapContinueResponse;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapContinueRequest {
        proto::DapContinueRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.args.thread_id,
            single_thread: self.args.single_thread,
        }
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            args: ContinueArguments {
                thread_id: request.thread_id,
                single_thread: request.single_thread,
            },
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(Self::Response {
            all_threads_continued: message.all_threads_continued,
        })
    }

    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapContinueResponse {
            client_id: debug_client_id.to_proto(),
            all_threads_continued: message.all_threads_continued,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct PauseCommand {
    pub thread_id: i64,
}

impl LocalDapCommand for PauseCommand {
    type Response = <dap::requests::Pause as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Pause;
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::PauseArguments {
            thread_id: self.thread_id,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for PauseCommand {
    type ProtoRequest = proto::DapPauseRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            thread_id: request.thread_id,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapPauseRequest {
        proto::DapPauseRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.thread_id,
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct DisconnectCommand {
    pub restart: Option<bool>,
    pub terminate_debuggee: Option<bool>,
    pub suspend_debuggee: Option<bool>,
}

impl LocalDapCommand for DisconnectCommand {
    type Response = <dap::requests::Disconnect as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Disconnect;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::DisconnectArguments {
            restart: self.restart,
            terminate_debuggee: self.terminate_debuggee,
            suspend_debuggee: self.suspend_debuggee,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for DisconnectCommand {
    type ProtoRequest = proto::DapDisconnectRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            restart: request.restart,
            terminate_debuggee: request.terminate_debuggee,
            suspend_debuggee: request.suspend_debuggee,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapDisconnectRequest {
        proto::DapDisconnectRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            restart: self.restart,
            terminate_debuggee: self.terminate_debuggee,
            suspend_debuggee: self.suspend_debuggee,
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct TerminateThreadsCommand {
    pub thread_ids: Option<Vec<i64>>,
}

impl LocalDapCommand for TerminateThreadsCommand {
    type Response = <dap::requests::TerminateThreads as dap::requests::Request>::Response;
    type DapRequest = dap::requests::TerminateThreads;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities
            .supports_terminate_threads_request
            .unwrap_or_default()
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::TerminateThreadsArguments {
            thread_ids: self.thread_ids.clone(),
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for TerminateThreadsCommand {
    type ProtoRequest = proto::DapTerminateThreadsRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        let thread_ids = if request.thread_ids.is_empty() {
            None
        } else {
            Some(request.thread_ids.clone())
        };

        Self { thread_ids }
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapTerminateThreadsRequest {
        proto::DapTerminateThreadsRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_ids: self.thread_ids.clone().unwrap_or_default(),
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct TerminateCommand {
    pub restart: Option<bool>,
}

impl LocalDapCommand for TerminateCommand {
    type Response = <dap::requests::Terminate as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Terminate;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_terminate_request.unwrap_or_default()
    }
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::TerminateArguments {
            restart: self.restart,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for TerminateCommand {
    type ProtoRequest = proto::DapTerminateRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            restart: request.restart,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapTerminateRequest {
        proto::DapTerminateRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            restart: self.restart,
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct RestartCommand {
    pub raw: serde_json::Value,
}

impl LocalDapCommand for RestartCommand {
    type Response = <dap::requests::Restart as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Restart;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_restart_request.unwrap_or_default()
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::RestartArguments {
            raw: self.raw.clone(),
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for RestartCommand {
    type ProtoRequest = proto::DapRestartRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            raw: serde_json::from_slice(&request.raw_args)
                .log_err()
                .unwrap_or(serde_json::Value::Null),
        }
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapRestartRequest {
        let raw_args = serde_json::to_vec(&self.raw).log_err().unwrap_or_default();

        proto::DapRestartRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            raw_args,
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VariablesCommand {
    pub variables_reference: u64,
    pub filter: Option<VariablesArgumentsFilter>,
    pub start: Option<u64>,
    pub count: Option<u64>,
    pub format: Option<ValueFormat>,
}

impl LocalDapCommand for VariablesCommand {
    type Response = Vec<Variable>;
    type DapRequest = dap::requests::Variables;
    const CACHEABLE: bool = true;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::VariablesArguments {
            variables_reference: self.variables_reference,
            filter: self.filter,
            start: self.start,
            count: self.count,
            format: self.format.clone(),
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.variables)
    }
}

impl DapCommand for VariablesCommand {
    type ProtoRequest = proto::VariablesRequest;
    type ProtoResponse = proto::DapVariables;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::VariablesRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            variables_reference: self.variables_reference,
            filter: None,
            start: self.start,
            count: self.count,
            format: None,
        }
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            variables_reference: request.variables_reference,
            filter: None,
            start: request.start,
            count: request.count,
            format: None,
        }
    }

    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapVariables {
            client_id: debug_client_id.to_proto(),
            variables: message.to_proto(),
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(Vec::from_proto(message.variables))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct SetVariableValueCommand {
    pub name: String,
    pub value: String,
    pub variables_reference: u64,
}
impl LocalDapCommand for SetVariableValueCommand {
    type Response = SetVariableResponse;
    type DapRequest = dap::requests::SetVariable;
    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_set_variable.unwrap_or_default()
    }
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::SetVariableArguments {
            format: None,
            name: self.name.clone(),
            value: self.value.clone(),
            variables_reference: self.variables_reference,
        }
    }
    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

impl DapCommand for SetVariableValueCommand {
    type ProtoRequest = proto::DapSetVariableValueRequest;
    type ProtoResponse = proto::DapSetVariableValueResponse;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::DapSetVariableValueRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            variables_reference: self.variables_reference,
            value: self.value.clone(),
            name: self.name.clone(),
        }
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            variables_reference: request.variables_reference,
            name: request.name.clone(),
            value: request.value.clone(),
        }
    }

    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapSetVariableValueResponse {
            client_id: debug_client_id.to_proto(),
            value: message.value,
            variable_type: message.type_,
            named_variables: message.named_variables,
            variables_reference: message.variables_reference,
            indexed_variables: message.indexed_variables,
            memory_reference: message.memory_reference,
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(SetVariableResponse {
            value: message.value,
            type_: message.variable_type,
            variables_reference: message.variables_reference,
            named_variables: message.named_variables,
            indexed_variables: message.indexed_variables,
            memory_reference: message.memory_reference,
            value_location_reference: None, // TODO
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct RestartStackFrameCommand {
    pub stack_frame_id: u64,
}

impl LocalDapCommand for RestartStackFrameCommand {
    type Response = <dap::requests::RestartFrame as dap::requests::Request>::Response;
    type DapRequest = dap::requests::RestartFrame;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_restart_frame.unwrap_or_default()
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::RestartFrameArguments {
            frame_id: self.stack_frame_id,
        }
    }

    fn response_from_dap(
        &self,
        _message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

impl DapCommand for RestartStackFrameCommand {
    type ProtoRequest = proto::DapRestartStackFrameRequest;
    type ProtoResponse = proto::Ack;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            stack_frame_id: request.stack_frame_id,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapRestartStackFrameRequest {
        proto::DapRestartStackFrameRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            stack_frame_id: self.stack_frame_id,
        }
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        _message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::Ack {}
    }

    fn response_from_proto(&self, _message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct ModulesCommand;

impl LocalDapCommand for ModulesCommand {
    type Response = Vec<dap::Module>;
    type DapRequest = dap::requests::Modules;
    const CACHEABLE: bool = true;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_modules_request.unwrap_or_default()
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::ModulesArguments {
            start_module: None,
            module_count: None,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.modules)
    }
}

impl DapCommand for ModulesCommand {
    type ProtoRequest = proto::DapModulesRequest;
    type ProtoResponse = proto::DapModulesResponse;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(_request: &Self::ProtoRequest) -> Self {
        Self {}
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapModulesRequest {
        proto::DapModulesRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
        }
    }

    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapModulesResponse {
            modules: message
                .into_iter()
                .map(|module| module.to_proto())
                .collect(),
            client_id: debug_client_id.to_proto(),
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(message
            .modules
            .into_iter()
            .filter_map(|module| dap::Module::from_proto(module).ok())
            .collect())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct LoadedSourcesCommand;

impl LocalDapCommand for LoadedSourcesCommand {
    type Response = Vec<dap::Source>;
    type DapRequest = dap::requests::LoadedSources;
    const CACHEABLE: bool = true;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities
            .supports_loaded_sources_request
            .unwrap_or_default()
    }
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::LoadedSourcesArguments {}
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.sources)
    }
}

impl DapCommand for LoadedSourcesCommand {
    type ProtoRequest = proto::DapLoadedSourcesRequest;
    type ProtoResponse = proto::DapLoadedSourcesResponse;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(_request: &Self::ProtoRequest) -> Self {
        Self {}
    }

    fn to_proto(
        &self,
        debug_client_id: SessionId,
        upstream_project_id: u64,
    ) -> proto::DapLoadedSourcesRequest {
        proto::DapLoadedSourcesRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
        }
    }

    fn response_to_proto(
        debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapLoadedSourcesResponse {
            sources: message
                .into_iter()
                .map(|source| source.to_proto())
                .collect(),
            client_id: debug_client_id.to_proto(),
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(message
            .sources
            .into_iter()
            .map(dap::Source::from_proto)
            .collect())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct StackTraceCommand {
    pub thread_id: i64,
    pub start_frame: Option<u64>,
    pub levels: Option<u64>,
}

impl LocalDapCommand for StackTraceCommand {
    type Response = Vec<dap::StackFrame>;
    type DapRequest = dap::requests::StackTrace;
    const CACHEABLE: bool = true;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::StackTraceArguments {
            thread_id: self.thread_id,
            start_frame: self.start_frame,
            levels: self.levels,
            format: None,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.stack_frames)
    }
}

impl DapCommand for StackTraceCommand {
    type ProtoRequest = proto::DapStackTraceRequest;
    type ProtoResponse = proto::DapStackTraceResponse;

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::DapStackTraceRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.thread_id,
            start_frame: self.start_frame,
            stack_trace_levels: self.levels,
        }
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            thread_id: request.thread_id,
            start_frame: request.start_frame,
            levels: request.stack_trace_levels,
        }
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(message
            .frames
            .into_iter()
            .map(dap::StackFrame::from_proto)
            .collect())
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapStackTraceResponse {
            frames: message.to_proto(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct ScopesCommand {
    pub stack_frame_id: u64,
}

impl LocalDapCommand for ScopesCommand {
    type Response = Vec<dap::Scope>;
    type DapRequest = dap::requests::Scopes;
    const CACHEABLE: bool = true;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::ScopesArguments {
            frame_id: self.stack_frame_id,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.scopes)
    }
}

impl DapCommand for ScopesCommand {
    type ProtoRequest = proto::DapScopesRequest;
    type ProtoResponse = proto::DapScopesResponse;

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::DapScopesRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            stack_frame_id: self.stack_frame_id,
        }
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            stack_frame_id: request.stack_frame_id,
        }
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(Vec::from_proto(message.scopes))
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapScopesResponse {
            scopes: message.to_proto(),
        }
    }
}

impl LocalDapCommand for super::session::CompletionsQuery {
    type Response = dap::CompletionsResponse;
    type DapRequest = dap::requests::Completions;
    const CACHEABLE: bool = true;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::CompletionsArguments {
            text: self.query.clone(),
            frame_id: self.frame_id,
            column: self.column,
            line: None,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities
            .supports_completions_request
            .unwrap_or_default()
    }
}

impl DapCommand for super::session::CompletionsQuery {
    type ProtoRequest = proto::DapCompletionRequest;
    type ProtoResponse = proto::DapCompletionResponse;

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::DapCompletionRequest {
            client_id: debug_client_id.to_proto(),
            project_id: upstream_project_id,
            frame_id: self.frame_id,
            query: self.query.clone(),
            column: self.column,
            line: self.line,
        }
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            query: request.query.clone(),
            frame_id: request.frame_id,
            column: request.column,
            line: request.line,
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(dap::CompletionsResponse {
            targets: Vec::from_proto(message.completions),
        })
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapCompletionResponse {
            client_id: _debug_client_id.to_proto(),
            completions: message.targets.to_proto(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct EvaluateCommand {
    pub expression: String,
    pub frame_id: Option<u64>,
    pub context: Option<dap::EvaluateArgumentsContext>,
    pub source: Option<dap::Source>,
}

impl LocalDapCommand for EvaluateCommand {
    type Response = dap::EvaluateResponse;
    type DapRequest = dap::requests::Evaluate;
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::EvaluateArguments {
            expression: self.expression.clone(),
            frame_id: self.frame_id,
            context: self.context.clone(),
            source: self.source.clone(),
            line: None,
            column: None,
            format: None,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}
impl DapCommand for EvaluateCommand {
    type ProtoRequest = proto::DapEvaluateRequest;
    type ProtoResponse = proto::DapEvaluateResponse;

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::DapEvaluateRequest {
            client_id: debug_client_id.to_proto(),
            project_id: upstream_project_id,
            expression: self.expression.clone(),
            frame_id: self.frame_id,
            context: self
                .context
                .clone()
                .map(|context| context.to_proto().into()),
        }
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            expression: request.expression.clone(),
            frame_id: request.frame_id,
            context: Some(dap::EvaluateArgumentsContext::from_proto(request.context())),
            source: None,
        }
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(dap::EvaluateResponse {
            result: message.result.clone(),
            type_: message.evaluate_type.clone(),
            presentation_hint: None,
            variables_reference: message.variable_reference,
            named_variables: message.named_variables,
            indexed_variables: message.indexed_variables,
            memory_reference: message.memory_reference,
            value_location_reference: None, //TODO
        })
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapEvaluateResponse {
            result: message.result,
            evaluate_type: message.type_,
            variable_reference: message.variables_reference,
            named_variables: message.named_variables,
            indexed_variables: message.indexed_variables,
            memory_reference: message.memory_reference,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct ThreadsCommand;

impl LocalDapCommand for ThreadsCommand {
    type Response = Vec<dap::Thread>;
    type DapRequest = dap::requests::Threads;
    const CACHEABLE: bool = true;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::ThreadsArgument {}
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.threads)
    }
}

impl DapCommand for ThreadsCommand {
    type ProtoRequest = proto::DapThreadsRequest;
    type ProtoResponse = proto::DapThreadsResponse;

    fn to_proto(&self, debug_client_id: SessionId, upstream_project_id: u64) -> Self::ProtoRequest {
        proto::DapThreadsRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
        }
    }

    fn from_proto(_request: &Self::ProtoRequest) -> Self {
        Self {}
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(request.client_id)
    }

    fn response_from_proto(&self, message: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(Vec::from_proto(message.threads))
    }

    fn response_to_proto(
        _debug_client_id: SessionId,
        message: Self::Response,
    ) -> Self::ProtoResponse {
        proto::DapThreadsResponse {
            threads: message.to_proto(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(super) struct Initialize {
    pub(super) adapter_id: String,
}

fn dap_client_capabilities(adapter_id: String) -> InitializeRequestArguments {
    InitializeRequestArguments {
        client_id: Some("zed".to_owned()),
        client_name: Some("Zed".to_owned()),
        adapter_id,
        locale: Some("en-US".to_owned()),
        path_format: Some(InitializeRequestArgumentsPathFormat::Path),
        supports_variable_type: Some(true),
        supports_variable_paging: Some(false),
        supports_run_in_terminal_request: Some(true),
        supports_memory_references: Some(true),
        supports_progress_reporting: Some(false),
        supports_invalidated_event: Some(false),
        lines_start_at1: Some(true),
        columns_start_at1: Some(true),
        supports_memory_event: Some(false),
        supports_args_can_be_interpreted_by_shell: Some(false),
        supports_start_debugging_request: Some(true),
        supports_ansistyling: Some(true),
    }
}

impl LocalDapCommand for Initialize {
    type Response = Capabilities;
    type DapRequest = dap::requests::Initialize;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap_client_capabilities(self.adapter_id.clone())
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(super) struct ConfigurationDone {}

impl LocalDapCommand for ConfigurationDone {
    type Response = ();
    type DapRequest = dap::requests::ConfigurationDone;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities
            .supports_configuration_done_request
            .unwrap_or_default()
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::ConfigurationDoneArguments {}
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(super) struct Launch {
    pub(super) raw: Value,
}

impl LocalDapCommand for Launch {
    type Response = ();
    type DapRequest = dap::requests::Launch;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::LaunchRequestArguments {
            raw: self.raw.clone(),
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(super) struct Attach {
    pub(super) raw: Value,
}

impl LocalDapCommand for Attach {
    type Response = ();
    type DapRequest = dap::requests::Attach;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::AttachRequestArguments {
            raw: self.raw.clone(),
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(super) struct SetBreakpoints {
    pub(super) source: dap::Source,
    pub(super) breakpoints: Vec<SourceBreakpoint>,
    pub(super) source_modified: Option<bool>,
}

impl LocalDapCommand for SetBreakpoints {
    type Response = Vec<dap::Breakpoint>;
    type DapRequest = dap::requests::SetBreakpoints;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::SetBreakpointsArguments {
            lines: None,
            source_modified: self.source_modified,
            source: self.source.clone(),
            breakpoints: Some(self.breakpoints.clone()),
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.breakpoints)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DataBreakpointContext {
    Variable {
        variables_reference: u64,
        name: String,
        bytes: Option<u64>,
    },
    Expression {
        expression: String,
        frame_id: Option<u64>,
    },
    Address {
        address: String,
        bytes: Option<u64>,
    },
}

impl DataBreakpointContext {
    pub fn human_readable_label(&self) -> String {
        match self {
            DataBreakpointContext::Variable { name, .. } => format!("Variable: {}", name),
            DataBreakpointContext::Expression { expression, .. } => {
                format!("Expression: {}", expression)
            }
            DataBreakpointContext::Address { address, bytes } => {
                let mut label = format!("Address: {}", address);
                if let Some(bytes) = bytes {
                    label.push_str(&format!(
                        " ({} byte{})",
                        bytes,
                        if *bytes == 1 { "" } else { "s" }
                    ));
                }
                label
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct DataBreakpointInfoCommand {
    pub context: Arc<DataBreakpointContext>,
    pub mode: Option<String>,
}

impl LocalDapCommand for DataBreakpointInfoCommand {
    type Response = dap::DataBreakpointInfoResponse;
    type DapRequest = dap::requests::DataBreakpointInfo;
    const CACHEABLE: bool = true;

    // todo(debugger): We should expand this trait in the future to take a &self
    // Depending on this command is_supported could be differentb
    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_data_breakpoints.unwrap_or(false)
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        let (variables_reference, name, frame_id, as_address, bytes) = match &*self.context {
            DataBreakpointContext::Variable {
                variables_reference,
                name,
                bytes,
            } => (
                Some(*variables_reference),
                name.clone(),
                None,
                Some(false),
                *bytes,
            ),
            DataBreakpointContext::Expression {
                expression,
                frame_id,
            } => (None, expression.clone(), *frame_id, Some(false), None),
            DataBreakpointContext::Address { address, bytes } => {
                (None, address.clone(), None, Some(true), *bytes)
            }
        };

        dap::DataBreakpointInfoArguments {
            variables_reference,
            name,
            frame_id,
            bytes,
            as_address,
            mode: self.mode.clone(),
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct SetDataBreakpointsCommand {
    pub breakpoints: Vec<dap::DataBreakpoint>,
}

impl LocalDapCommand for SetDataBreakpointsCommand {
    type Response = Vec<dap::Breakpoint>;
    type DapRequest = dap::requests::SetDataBreakpoints;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities.supports_data_breakpoints.unwrap_or(false)
    }

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::SetDataBreakpointsArguments {
            breakpoints: self.breakpoints.clone(),
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.breakpoints)
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(super) enum SetExceptionBreakpoints {
    Plain {
        filters: Vec<String>,
    },
    WithOptions {
        filters: Vec<ExceptionFilterOptions>,
    },
}

impl LocalDapCommand for SetExceptionBreakpoints {
    type Response = Vec<dap::Breakpoint>;
    type DapRequest = dap::requests::SetExceptionBreakpoints;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        match self {
            SetExceptionBreakpoints::Plain { filters } => dap::SetExceptionBreakpointsArguments {
                filters: filters.clone(),
                exception_options: None,
                filter_options: None,
            },
            SetExceptionBreakpoints::WithOptions { filters } => {
                dap::SetExceptionBreakpointsArguments {
                    filters: vec![],
                    filter_options: Some(filters.clone()),
                    exception_options: None,
                }
            }
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message.breakpoints.unwrap_or_default())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(super) struct LocationsCommand {
    pub(super) reference: u64,
}

impl LocalDapCommand for LocationsCommand {
    type Response = dap::LocationsResponse;
    type DapRequest = dap::requests::Locations;
    const CACHEABLE: bool = true;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::LocationsArguments {
            location_reference: self.reference,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}

impl DapCommand for LocationsCommand {
    type ProtoRequest = proto::DapLocationsRequest;
    type ProtoResponse = proto::DapLocationsResponse;

    fn client_id_from_proto(message: &Self::ProtoRequest) -> SessionId {
        SessionId::from_proto(message.session_id)
    }

    fn from_proto(message: &Self::ProtoRequest) -> Self {
        Self {
            reference: message.location_reference,
        }
    }

    fn to_proto(&self, session_id: SessionId, project_id: u64) -> Self::ProtoRequest {
        proto::DapLocationsRequest {
            project_id,
            session_id: session_id.to_proto(),
            location_reference: self.reference,
        }
    }

    fn response_to_proto(_: SessionId, response: Self::Response) -> Self::ProtoResponse {
        proto::DapLocationsResponse {
            source: Some(response.source.to_proto()),
            line: response.line,
            column: response.column,
            end_line: response.end_line,
            end_column: response.end_column,
        }
    }

    fn response_from_proto(&self, response: Self::ProtoResponse) -> Result<Self::Response> {
        Ok(dap::LocationsResponse {
            source: response
                .source
                .map(<dap::Source as ProtoConversion>::from_proto)
                .context("Missing `source` field in Locations proto")?,
            line: response.line,
            column: response.column,
            end_line: response.end_line,
            end_column: response.end_column,
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct ReadMemory {
    pub(crate) memory_reference: String,
    pub(crate) offset: Option<u64>,
    pub(crate) count: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReadMemoryResponse {
    pub(super) address: Arc<str>,
    pub(super) unreadable_bytes: Option<u64>,
    pub(super) content: Arc<[u8]>,
}

impl LocalDapCommand for ReadMemory {
    type Response = ReadMemoryResponse;
    type DapRequest = dap::requests::ReadMemory;
    const CACHEABLE: bool = true;

    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities
            .supports_read_memory_request
            .unwrap_or_default()
    }
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        dap::ReadMemoryArguments {
            memory_reference: self.memory_reference.clone(),
            offset: self.offset,
            count: self.count,
        }
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        let data = if let Some(data) = message.data {
            base64::engine::general_purpose::STANDARD
                .decode(data)
                .log_err()
                .context("parsing base64 data from DAP's ReadMemory response")?
        } else {
            vec![]
        };

        Ok(ReadMemoryResponse {
            address: message.address.into(),
            content: data.into(),
            unreadable_bytes: message.unreadable_bytes,
        })
    }
}

impl LocalDapCommand for dap::WriteMemoryArguments {
    type Response = dap::WriteMemoryResponse;
    type DapRequest = dap::requests::WriteMemory;
    fn is_supported(capabilities: &Capabilities) -> bool {
        capabilities
            .supports_write_memory_request
            .unwrap_or_default()
    }
    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        self.clone()
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }
}
