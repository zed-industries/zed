use futures::{channel::mpsc, SinkExt as _, StreamExt as _};
use gpui::{Task, View, WindowContext};
use http_client::{AsyncBody, HttpClientWithUrl, Request};
use jupyter_protocol::{ExecutionState, JupyterMessage, KernelInfoReply};
use runtimelib::JupyterKernelspec;

use futures::StreamExt;
use smol::io::AsyncReadExt as _;

use crate::Session;

use super::RunningKernel;
use anyhow::Result;
use jupyter_websocket_client::{
    JupyterWebSocketReader, JupyterWebSocketWriter, KernelSpecsResponse, RemoteServer,
};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct RemoteKernelSpecification {
    pub name: String,
    pub url: String,
    pub token: String,
    pub kernelspec: JupyterKernelspec,
}

pub async fn list_remote_kernelspecs(
    remote_server: RemoteServer,
    http_client: Arc<HttpClientWithUrl>,
) -> Result<KernelSpecsResponse> {
    let url = remote_server.api_url("/kernelspecs");

    let request = Request::builder()
        .method("GET")
        .uri(&url)
        .header("Authorization", format!("token {}", remote_server.token))
        .body(AsyncBody::default())?;

    let response = http_client.send(request).await?;

    if response.status().is_success() {
        let mut body = response.into_body();

        let mut body_bytes = Vec::new();
        body.read_to_end(&mut body_bytes).await?;

        let kernel_specs: KernelSpecsResponse = serde_json::from_slice(&body_bytes)?;
        Ok(kernel_specs)
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch kernel specs: {}",
            response.status()
        ))
    }
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
    pub fn new(
        kernelspec: RemoteKernelSpecification,
        working_directory: std::path::PathBuf,
        session: View<Session>,
        cx: &mut WindowContext,
    ) -> Task<Result<Box<dyn RunningKernel>>> {
        let remote_server = RemoteServer {
            base_url: kernelspec.url,
            token: kernelspec.token,
        };
        cx.spawn(|cx| async move {
            // todo: launch a kernel to get a kernel ID
            let kernel_id = "d77b481b-2f14-4528-af0a-6c4c9ca98085";

            let kernel_socket = remote_server.connect_to_kernel(kernel_id).await?;

            let (mut w, mut r): (JupyterWebSocketWriter, JupyterWebSocketReader) =
                kernel_socket.split();

            let (request_tx, mut request_rx) =
                futures::channel::mpsc::channel::<JupyterMessage>(100);

            let routing_task = cx.background_executor().spawn({
                async move {
                    while let Some(message) = request_rx.next().await {
                        w.send(message).await.ok();
                    }
                    Ok(())
                }
            });

            let receiving_task = cx.spawn({
                let session = session.clone();

                |mut cx| async move {
                    while let Some(message) = r.next().await {
                        match message {
                            Ok(message) => {
                                session
                                    .update(&mut cx, |session, cx| {
                                        session.route(&message, cx);
                                    })
                                    .ok();
                            }
                            Err(e) => {
                                log::error!("Error receiving message: {:?}", e);
                            }
                        }
                    }
                    Ok(())
                }
            });

            anyhow::Ok(Box::new(Self {
                _routing_task: routing_task,
                _receiving_task: receiving_task,
                remote_server,
                working_directory,
                request_tx,
                // todo(kyle): pull this from the kernel API to start with
                execution_state: ExecutionState::Idle,
                kernel_info: None,
            }) as Box<dyn RunningKernel>)
        })
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
