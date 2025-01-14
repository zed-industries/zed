use anyhow::{Ok, Result};
use dap::{
    client::DebugAdapterClientId,
    proto_conversions::ProtoConversion,
    requests::{Continue, Next},
    ContinueArguments, NextArguments, StepInArguments, StepOutArguments, SteppingGranularity,
};
use gpui::{AsyncAppContext, WeakModel};
use rpc::proto;
use util::ResultExt;

use crate::dap_store::DapStore;

pub trait DapCommand: 'static + Sized + Send + std::fmt::Debug + Clone {
    type Response: 'static + Send + std::fmt::Debug;
    type DapRequest: 'static + Send + dap::requests::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

    fn handle_response(
        &self,
        _dap_store: WeakModel<DapStore>,
        _client_id: &DebugAdapterClientId,
        response: Result<Self::Response>,
        _cx: &mut AsyncAppContext,
    ) -> Result<Self::Response> {
        response
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId;

    fn from_proto(request: &Self::ProtoRequest) -> Self;

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
        upstream_project_id: u64,
    ) -> Self::ProtoRequest;

    fn response_to_proto(
        debug_client_id: &DebugAdapterClientId,
        message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response;

    fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response>;

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments;

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response>;
}

#[derive(Debug, Clone)]
pub struct StepCommand {
    pub thread_id: u64,
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

#[derive(Debug, Clone)]
pub(crate) struct NextCommand {
    pub inner: StepCommand,
}

impl DapCommand for NextCommand {
    type Response = <Next as dap::requests::Request>::Response;
    type DapRequest = Next;
    type ProtoRequest = proto::DapNextRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            inner: StepCommand::from_proto(request.clone()),
        }
    }

    fn response_to_proto(
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StepInCommand {
    pub inner: StepCommand,
}

impl DapCommand for StepInCommand {
    type Response = <dap::requests::StepIn as dap::requests::Request>::Response;
    type DapRequest = dap::requests::StepIn;
    type ProtoRequest = proto::DapStepInRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
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
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StepOutCommand {
    pub inner: StepCommand,
}

impl DapCommand for StepOutCommand {
    type Response = <dap::requests::StepOut as dap::requests::Request>::Response;
    type DapRequest = dap::requests::StepOut;
    type ProtoRequest = proto::DapStepOutRequest;

    fn handle_response(
        &self,
        dap_store: WeakModel<DapStore>,
        client_id: &DebugAdapterClientId,
        response: Result<Self::Response>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self::Response> {
        if response.is_ok() {
            dap_store
                .update(cx, |this, cx| {
                    if let Some((client, project_id)) = this.downstream_client() {
                        let thread_message = proto::UpdateThreadStatus {
                            project_id: *project_id,
                            client_id: client_id.to_proto(),
                            thread_id: self.inner.thread_id,
                            status: proto::DebuggerThreadStatus::Running.into(),
                        };

                        cx.emit(crate::dap_store::DapStoreEvent::UpdateThreadStatus(
                            thread_message.clone(),
                        ));

                        client.send(thread_message).log_err();
                    }
                })
                .log_err();
        }
        response
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
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
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StepBackCommand {
    pub inner: StepCommand,
}

impl DapCommand for StepBackCommand {
    type Response = <dap::requests::StepBack as dap::requests::Request>::Response;
    type DapRequest = dap::requests::StepBack;
    type ProtoRequest = proto::DapStepBackRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
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
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ContinueCommand {
    pub args: ContinueArguments,
}

impl DapCommand for ContinueCommand {
    type Response = <Continue as dap::requests::Request>::Response;
    type DapRequest = Continue;
    type ProtoRequest = proto::DapContinueRequest;

    fn handle_response(
        &self,
        dap_store: WeakModel<DapStore>,
        client_id: &DebugAdapterClientId,
        response: Result<Self::Response>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self::Response> {
        if response.is_ok() {
            dap_store
                .update(cx, |this, cx| {
                    if let Some((client, project_id)) = this.downstream_client() {
                        let thread_message = proto::UpdateThreadStatus {
                            project_id: *project_id,
                            client_id: client_id.to_proto(),
                            thread_id: self.args.thread_id,
                            status: proto::DebuggerThreadStatus::Running.into(),
                        };

                        cx.emit(crate::dap_store::DapStoreEvent::UpdateThreadStatus(
                            thread_message.clone(),
                        ));

                        client.send(thread_message).log_err();
                    }
                })
                .log_err();
        }
        response
    }

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
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

    fn to_dap(&self) -> <Self::DapRequest as dap::requests::Request>::Arguments {
        self.args.clone()
    }

    fn response_from_dap(
        &self,
        message: <Self::DapRequest as dap::requests::Request>::Response,
    ) -> Result<Self::Response> {
        Ok(message)
    }

    fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(Self::Response {
            all_threads_continued: message.all_threads_continued,
        })
    }

    fn response_to_proto(
        debug_client_id: &DebugAdapterClientId,
        message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::DapContinueResponse {
            client_id: debug_client_id.to_proto(),
            all_threads_continued: message.all_threads_continued,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PauseCommand {
    pub thread_id: u64,
}

impl DapCommand for PauseCommand {
    type Response = <dap::requests::Pause as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Pause;
    type ProtoRequest = proto::DapPauseRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            thread_id: request.thread_id,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
        upstream_project_id: u64,
    ) -> proto::DapPauseRequest {
        proto::DapPauseRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_id: self.thread_id,
        }
    }

    fn response_to_proto(
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
    }

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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DisconnectCommand {
    pub restart: Option<bool>,
    pub terminate_debuggee: Option<bool>,
    pub suspend_debuggee: Option<bool>,
}

impl DapCommand for DisconnectCommand {
    type Response = <dap::requests::Disconnect as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Disconnect;
    type ProtoRequest = proto::DapDisconnectRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
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
        debug_client_id: &DebugAdapterClientId,
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
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
    }

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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TerminateThreadsCommand {
    pub thread_ids: Option<Vec<u64>>,
}

impl DapCommand for TerminateThreadsCommand {
    type Response = <dap::requests::TerminateThreads as dap::requests::Request>::Response;
    type DapRequest = dap::requests::TerminateThreads;
    type ProtoRequest = proto::DapTerminateThreadsRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
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
        debug_client_id: &DebugAdapterClientId,
        upstream_project_id: u64,
    ) -> proto::DapTerminateThreadsRequest {
        proto::DapTerminateThreadsRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            thread_ids: self.thread_ids.clone().unwrap_or_default(),
        }
    }

    fn response_to_proto(
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TerminateCommand {
    pub restart: Option<bool>,
}

impl DapCommand for TerminateCommand {
    type Response = <dap::requests::Terminate as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Terminate;
    type ProtoRequest = proto::DapTerminateRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            restart: request.restart,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
        upstream_project_id: u64,
    ) -> proto::DapTerminateRequest {
        proto::DapTerminateRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            restart: self.restart,
        }
    }

    fn response_to_proto(
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RestartCommand {
    pub raw: serde_json::Value,
}

impl DapCommand for RestartCommand {
    type Response = <dap::requests::Restart as dap::requests::Request>::Response;
    type DapRequest = dap::requests::Restart;
    type ProtoRequest = proto::DapRestartRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
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
        debug_client_id: &DebugAdapterClientId,
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
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RestartStackFrameCommand {
    pub stack_frame_id: u64,
}

impl DapCommand for RestartStackFrameCommand {
    type Response = <dap::requests::RestartFrame as dap::requests::Request>::Response;
    type DapRequest = dap::requests::RestartFrame;
    type ProtoRequest = proto::DapRestartStackFrameRequest;

    fn client_id_from_proto(request: &Self::ProtoRequest) -> DebugAdapterClientId {
        DebugAdapterClientId::from_proto(request.client_id)
    }

    fn from_proto(request: &Self::ProtoRequest) -> Self {
        Self {
            stack_frame_id: request.stack_frame_id,
        }
    }

    fn to_proto(
        &self,
        debug_client_id: &DebugAdapterClientId,
        upstream_project_id: u64,
    ) -> proto::DapRestartStackFrameRequest {
        proto::DapRestartStackFrameRequest {
            project_id: upstream_project_id,
            client_id: debug_client_id.to_proto(),
            stack_frame_id: self.stack_frame_id,
        }
    }

    fn response_to_proto(
        _debug_client_id: &DebugAdapterClientId,
        _message: Self::Response,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        proto::Ack {}
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

    fn response_from_proto(
        self,
        _message: <Self::ProtoRequest as proto::RequestMessage>::Response,
    ) -> Result<Self::Response> {
        Ok(())
    }
}
