use futures::{channel::mpsc, SinkExt as _, StreamExt as _};
use gpui::{AppContext, Task};
use jupyter_protocol::{ExecutionState, JupyterMessage, KernelInfoReply};
// todo(kyle): figure out if this needs to be different
use runtimelib::JupyterKernelspec;

use super::RunningKernel;
use anyhow::Result;
use jupyter_websocket_client::{JupyterWebSocketReader, JupyterWebSocketWriter, RemoteServer};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct RemoteKernelSpecification {
    pub name: String,
    pub url: String,
    pub token: String,
    pub kernelspec: JupyterKernelspec,
}

impl PartialEq for RemoteKernelSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.url == other.url
    }
}

impl Eq for RemoteKernelSpecification {}

pub struct RemoteRunningKernel {
    remote_server: RemoteServer,
    _receiving_task: Task<Result<()>>,
    _routing_task: Task<Result<()>>,
    pub working_directory: std::path::PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
}

impl RemoteRunningKernel {
    pub async fn new(
        kernelspec: RemoteKernelSpecification,
        working_directory: std::path::PathBuf,
        cx: &mut AppContext,
    ) -> anyhow::Result<(Self, mpsc::Receiver<JupyterMessage>)> {
        let remote_server = RemoteServer {
            base_url: kernelspec.url,
            token: kernelspec.token,
        };

        // todo: launch a kernel to get a kernel ID
        let kernel_id = "not-implemented";

        let kernel_socket = remote_server.connect_to_kernel(kernel_id).await?;

        let (mut w, mut r): (JupyterWebSocketWriter, JupyterWebSocketReader) =
            kernel_socket.split();

        let (mut messages_tx, messages_rx) = mpsc::channel::<JupyterMessage>(100);

        let (request_tx, mut request_rx) = futures::channel::mpsc::channel::<JupyterMessage>(100);

        let routing_task = cx.background_executor().spawn({
            async move {
                while let Some(message) = request_rx.next().await {
                    w.send(message).await.ok();
                }
                Ok(())
            }
        });

        let receiving_task = cx.background_executor().spawn({
            async move {
                while let Some(message) = r.next().await {
                    match message {
                        Ok(message) => {
                            messages_tx.send(message).await.ok();
                        }
                        Err(e) => {
                            log::error!("Error receiving message: {:?}", e);
                        }
                    }
                }
                Ok(())
            }
        });

        anyhow::Ok((
            Self {
                _routing_task: routing_task,
                _receiving_task: receiving_task,
                remote_server,
                working_directory,
                request_tx,
                execution_state: ExecutionState::Idle,
                kernel_info: None,
            },
            messages_rx,
        ))
    }
}

impl Debug for RemoteRunningKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteRunningKernel")
            // custom debug that keeps tokens out of logs
            .field("remote_server url", &self.remote_server.base_url)
            .field("working_directory", &self.working_directory)
            .field("request_tx", &self.request_tx)
            .field("execution_state", &self.execution_state)
            .field("kernel_info", &self.kernel_info)
            .finish()
    }
}

impl RunningKernel for RemoteRunningKernel {
    fn request_tx(&self) -> futures::channel::mpsc::Sender<runtimelib::JupyterMessage> {
        self.request_tx.clone()
    }

    fn working_directory(&self) -> &std::path::PathBuf {
        &self.working_directory
    }

    fn execution_state(&self) -> &runtimelib::ExecutionState {
        &self.execution_state
    }

    fn set_execution_state(&mut self, state: runtimelib::ExecutionState) {
        self.execution_state = state;
    }

    fn kernel_info(&self) -> Option<&runtimelib::KernelInfoReply> {
        self.kernel_info.as_ref()
    }

    fn set_kernel_info(&mut self, info: runtimelib::KernelInfoReply) {
        self.kernel_info = Some(info);
    }

    fn force_shutdown(&mut self) -> anyhow::Result<()> {
        unimplemented!("force_shutdown")
    }
}
