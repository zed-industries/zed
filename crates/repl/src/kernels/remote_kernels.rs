use futures::{SinkExt as _, channel::mpsc};
use gpui::{App, AppContext as _, Entity, Task, Window};
use http_client::{AsyncBody, HttpClient, Request};
use jupyter_protocol::{ExecutionState, JupyterKernelspec, JupyterMessage, KernelInfoReply};

use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};

use futures::StreamExt;
use smol::io::AsyncReadExt as _;

use super::{KernelSession, RunningKernel};
use anyhow::Result;
use jupyter_websocket_client::{
    JupyterWebSocket, JupyterWebSocketReader, JupyterWebSocketWriter, KernelLaunchRequest,
    KernelSpecsResponse, RemoteServer,
};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct RemoteKernelSpecification {
    pub name: String,
    pub url: String,
    pub token: String,
    pub kernelspec: JupyterKernelspec,
}

pub async fn launch_remote_kernel(
    remote_server: &RemoteServer,
    http_client: Arc<dyn HttpClient>,
    kernel_name: &str,
    _path: &str,
) -> Result<String> {
    //
    let kernel_launch_request = KernelLaunchRequest {
        name: kernel_name.to_string(),
        // Note: since the path we have locally may not be the same as the one on the remote server,
        // we don't send it. We'll have to evaluate this decision along the way.
        path: None,
    };

    let kernel_launch_request = serde_json::to_string(&kernel_launch_request)?;

    let request = Request::builder()
        .method("POST")
        .uri(&remote_server.api_url("/kernels"))
        .header("Authorization", format!("token {}", remote_server.token))
        .body(AsyncBody::from(kernel_launch_request))?;

    let response = http_client.send(request).await?;

    if !response.status().is_success() {
        let mut body = String::new();
        response.into_body().read_to_string(&mut body).await?;
        anyhow::bail!("Failed to launch kernel: {body}");
    }

    let mut body = String::new();
    response.into_body().read_to_string(&mut body).await?;

    let response: jupyter_websocket_client::Kernel = serde_json::from_str(&body)?;

    Ok(response.id)
}

pub async fn list_remote_kernelspecs(
    remote_server: RemoteServer,
    http_client: Arc<dyn HttpClient>,
) -> Result<Vec<RemoteKernelSpecification>> {
    let url = remote_server.api_url("/kernelspecs");

    let request = Request::builder()
        .method("GET")
        .uri(&url)
        .header("Authorization", format!("token {}", remote_server.token))
        .body(AsyncBody::default())?;

    let response = http_client.send(request).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to fetch kernel specs: {}",
        response.status()
    );
    let mut body = response.into_body();

    let mut body_bytes = Vec::new();
    body.read_to_end(&mut body_bytes).await?;

    let kernel_specs: KernelSpecsResponse = serde_json::from_slice(&body_bytes)?;

    let remote_kernelspecs = kernel_specs
        .kernelspecs
        .into_iter()
        .map(|(name, spec)| RemoteKernelSpecification {
            name,
            url: remote_server.base_url.clone(),
            token: remote_server.token.clone(),
            kernelspec: spec.spec,
        })
        .collect::<Vec<RemoteKernelSpecification>>();

    anyhow::ensure!(!remote_kernelspecs.is_empty(), "No kernel specs found");
    Ok(remote_kernelspecs)
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
    http_client: Arc<dyn HttpClient>,
    pub working_directory: std::path::PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
    pub kernel_id: String,
}

impl RemoteRunningKernel {
    pub fn new<S: KernelSession + 'static>(
        kernelspec: RemoteKernelSpecification,
        working_directory: std::path::PathBuf,
        session: Entity<S>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn RunningKernel>>> {
        let remote_server = RemoteServer {
            base_url: kernelspec.url,
            token: kernelspec.token,
        };

        let http_client = cx.http_client();

        window.spawn(cx, async move |cx| {
            let kernel_id = launch_remote_kernel(
                &remote_server,
                http_client.clone(),
                &kernelspec.name,
                working_directory.to_str().unwrap_or_default(),
            )
            .await?;

            let ws_url = format!(
                "{}/api/kernels/{}/channels?token={}",
                remote_server.base_url.replace("http", "ws"),
                kernel_id,
                remote_server.token
            );

            let mut req: Request<()> = ws_url.into_client_request()?;
            let headers = req.headers_mut();

            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&format!(
                    "Zed/{} ({}; {})",
                    "repl",
                    std::env::consts::OS,
                    std::env::consts::ARCH
                ))?,
            );

            let response = connect_async(req).await;

            let (ws_stream, _response) = response?;

            let kernel_socket = JupyterWebSocket { inner: ws_stream };

            let (mut w, mut r): (JupyterWebSocketWriter, JupyterWebSocketReader) =
                kernel_socket.split();

            let (request_tx, mut request_rx) =
                futures::channel::mpsc::channel::<JupyterMessage>(100);

            let routing_task = cx.background_spawn({
                async move {
                    while let Some(message) = request_rx.next().await {
                        w.send(message).await.ok();
                    }
                    Ok(())
                }
            });

            let receiving_task = cx.spawn({
                let session = session.clone();

                async move |cx| {
                    while let Some(message) = r.next().await {
                        match message {
                            Ok(message) => {
                                session
                                    .update_in(cx, |session, window, cx| {
                                        session.route(&message, window, cx);
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
                kernel_id,
                http_client: http_client.clone(),
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

    fn force_shutdown(&mut self, window: &mut Window, cx: &mut App) -> Task<anyhow::Result<()>> {
        let url = self
            .remote_server
            .api_url(&format!("/kernels/{}", self.kernel_id));
        let token = self.remote_server.token.clone();
        let http_client = self.http_client.clone();

        window.spawn(cx, async move |_| {
            let request = Request::builder()
                .method("DELETE")
                .uri(&url)
                .header("Authorization", format!("token {}", token))
                .body(AsyncBody::default())?;

            let response = http_client.send(request).await?;

            anyhow::ensure!(
                response.status().is_success(),
                "Failed to shutdown kernel: {}",
                response.status()
            );
            Ok(())
        })
    }
}
