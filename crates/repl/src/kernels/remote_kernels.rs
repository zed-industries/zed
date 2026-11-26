use futures::{SinkExt as _, channel::mpsc};
use gpui::{App, AppContext as _, BackgroundExecutor, Entity, Task, Window};
use http_client::{AsyncBody, HttpClient, Request};
use jupyter_protocol::{
    Channel, ExecutionState, JupyterKernelspec, JupyterMessage, JupyterMessageContent,
    KernelInfoReply,
};
use util::ResultExt as _;

use async_tungstenite::tokio::connect_async_with_tls_connector;
use async_tungstenite::tungstenite::{
    Error as SocketError,
    error::{ProtocolError, SubProtocolError},
};
use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};

use futures::StreamExt;
use smol::io::AsyncReadExt as _;

use super::{KernelSession, RunningKernel};
use anyhow::Result;
use jupyter_websocket_client::binary_protocol::KERNEL_WEBSOCKET_PROTOCOL;
use jupyter_websocket_client::{
    JupyterWebSocket, JupyterWebSocketReader, JupyterWebSocketWriter, KernelLaunchRequest,
    KernelSpecsResponse, ProtocolMode, RemoteServer,
};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct RemoteKernelSpecification {
    pub name: String,
    pub url: String,
    pub token: String,
    pub kernelspec: JupyterKernelspec,
}

async fn launch_remote_kernel(
    remote_server: &RemoteServer,
    http_client: Arc<dyn HttpClient>,
    kernel_name: &str,
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

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to launch kernel: {}",
        response.status()
    );

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

pub(super) struct RemoteKernelShutdown {
    executor: BackgroundExecutor,
    request: Option<futures::future::BoxFuture<'static, Result<()>>>,
}

impl Debug for RemoteKernelShutdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RemoteKernelShutdown")
    }
}

impl RemoteKernelShutdown {
    pub(super) async fn finish_launch<T: Send + 'static>(
        executor: &BackgroundExecutor,
        launch: impl Future<Output = Result<T>> + Send + 'static,
    ) -> Result<T> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        executor
            .spawn(async move {
                if !sender.is_canceled() {
                    drop(sender.send(launch.await));
                }
            })
            .detach();
        receiver.await?
    }

    pub(super) fn new(
        executor: BackgroundExecutor,
        request: impl Future<Output = Result<()>> + Send + 'static,
    ) -> Self {
        Self {
            executor,
            request: Some(Box::pin(request)),
        }
    }

    pub(super) fn shutdown(&mut self) -> Task<Result<()>> {
        match self.request.take() {
            Some(request) => self.executor.spawn(request),
            None => Task::ready(Ok(())),
        }
    }
}

impl Drop for RemoteKernelShutdown {
    fn drop(&mut self) {
        if let Some(request) = self.request.take() {
            self.executor
                .spawn(async move {
                    request.await.log_err();
                })
                .detach();
        }
    }
}

pub struct RemoteRunningKernel {
    shutdown: RemoteKernelShutdown,
    _connection: Task<()>,
    pub working_directory: std::path::PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub stdin_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
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
            session.update(cx, |session, cx| session.check_launch(cx))?;
            let authorization = HeaderValue::from_str(&format!("token {}", remote_server.token))?;
            let executor = cx.background_executor().clone();
            let (ws_url, shutdown) =
                RemoteKernelShutdown::finish_launch(cx.background_executor(), {
                    let authorization = authorization.clone();
                    async move {
                        let kernel_id = launch_remote_kernel(
                            &remote_server,
                            http_client.clone(),
                            &kernelspec.name,
                        )
                        .await?;

                        let url = remote_server.api_url(&format!("/kernels/{kernel_id}"));
                        let ws_url = format!("{}/channels", url.replacen("http", "ws", 1));
                        let shutdown = RemoteKernelShutdown::new(executor, {
                            async move {
                                let request = Request::builder()
                                    .method("DELETE")
                                    .uri(url)
                                    .header("Authorization", authorization)
                                    .body(AsyncBody::default())?;
                                let response = http_client.send(request).await?;
                                anyhow::ensure!(
                                    response.status().is_success()
                                        || response.status().as_u16() == 404,
                                    "Failed to shut down kernel: {}",
                                    response.status()
                                );
                                Ok(())
                            }
                        });

                        Ok((ws_url, shutdown))
                    }
                })
                .await?;
            session.update(cx, |session, cx| session.check_launch(cx))?;
            let mut req: Request<()> = ws_url.into_client_request()?;
            let headers = req.headers_mut();
            headers.insert("Authorization", authorization);

            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&format!(
                    "Zed/{} ({}; {})",
                    "repl",
                    std::env::consts::OS,
                    std::env::consts::ARCH
                ))?,
            );

            headers.insert(
                "Sec-WebSocket-Protocol",
                HeaderValue::from_static(KERNEL_WEBSOCKET_PROTOCOL),
            );
            let connect = |request| {
                async_compat::Compat::new(connect_async_with_tls_connector(
                    request,
                    Some(Arc::new(http_client_tls::tls_config()).into()),
                ))
            };
            let (ws_stream, protocol_mode) = match connect(req.clone()).await {
                Ok((stream, _)) => (stream, ProtocolMode::BinaryV1),
                Err(SocketError::Protocol(ProtocolError::SecWebSocketSubProtocolError(
                    SubProtocolError::NoSubProtocol,
                ))) => {
                    req.headers_mut().remove("Sec-WebSocket-Protocol");
                    (connect(req).await?.0, ProtocolMode::Json)
                }
                Err(error) => return Err(error.into()),
            };

            let kernel_socket = JupyterWebSocket {
                inner: ws_stream,
                protocol_mode,
            };
            let (mut w, mut r): (JupyterWebSocketWriter, JupyterWebSocketReader) =
                kernel_socket.split();

            let (request_tx, mut request_rx) =
                futures::channel::mpsc::channel::<JupyterMessage>(100);

            let routing = cx.background_spawn({
                async move {
                    while let Some(mut message) = request_rx.next().await {
                        message.channel = Some(match message.content {
                            JupyterMessageContent::InterruptRequest(_)
                            | JupyterMessageContent::ShutdownRequest(_) => Channel::Control,
                            JupyterMessageContent::InputReply(_) => Channel::Stdin,
                            _ => Channel::Shell,
                        });
                        w.send(message).await?;
                    }
                    Ok(())
                }
            });

            let connection = cx.spawn({
                let session = session.downgrade();

                async move |cx| {
                    let receiving = async {
                        while let Some(message) = r.next().await {
                            let message = match message {
                                Ok(message) => message,
                                Err(error) if error.is::<SocketError>() => return Err(error),
                                Err(error) => {
                                    log::warn!("Ignoring unsupported Jupyter message: {error}");
                                    continue;
                                }
                            };
                            session.update_in(cx, |session, window, cx| {
                                session.route(&message, window, cx);
                            })?;
                        }
                        anyhow::Ok(())
                    };
                    let result = futures::future::select(routing, Box::pin(receiving))
                        .await
                        .factor_first()
                        .0;
                    let reason = match result {
                        Ok(()) => "Remote kernel disconnected.".to_owned(),
                        Err(error) => format!("Remote kernel connection failed: {error}"),
                    };
                    session
                        .update(cx, |session, cx| session.kernel_errored(reason, cx))
                        .ok();
                }
            });

            let stdin_tx = request_tx.clone();

            anyhow::Ok(Box::new(Self {
                _connection: connection,
                shutdown,
                working_directory,
                request_tx,
                stdin_tx,
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
            .field("working_directory", &self.working_directory)
            .field("request_tx", &self.request_tx)
            .field("execution_state", &self.execution_state)
            .field("kernel_info", &self.kernel_info)
            .finish()
    }
}

impl RunningKernel for RemoteRunningKernel {
    fn request_tx(&self) -> futures::channel::mpsc::Sender<jupyter_protocol::JupyterMessage> {
        self.request_tx.clone()
    }

    fn stdin_tx(&self) -> futures::channel::mpsc::Sender<jupyter_protocol::JupyterMessage> {
        self.stdin_tx.clone()
    }

    fn working_directory(&self) -> &std::path::PathBuf {
        &self.working_directory
    }

    fn execution_state(&self) -> &jupyter_protocol::ExecutionState {
        &self.execution_state
    }

    fn set_execution_state(&mut self, state: jupyter_protocol::ExecutionState) {
        self.execution_state = state;
    }

    fn kernel_info(&self) -> Option<&jupyter_protocol::KernelInfoReply> {
        self.kernel_info.as_ref()
    }

    fn set_kernel_info(&mut self, info: jupyter_protocol::KernelInfoReply) {
        self.kernel_info = Some(info);
    }

    fn force_shutdown(&mut self, _window: &mut Window, _cx: &mut App) -> Task<anyhow::Result<()>> {
        self.shutdown.shutdown()
    }

    fn kill(&mut self) {
        self.request_tx.close_channel();
        self.stdin_tx.close_channel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::oneshot;

    #[gpui::test]
    async fn cancel_while_server_is_creating_kernel_still_releases_it(
        executor: BackgroundExecutor,
    ) {
        let (accepted, wait_accepted) = oneshot::channel();
        let (respond, response) = oneshot::channel();
        let (shutdown, wait_shutdown) = oneshot::channel();
        let launch = executor.spawn({
            let executor = executor.clone();
            async move {
                RemoteKernelShutdown::finish_launch(&executor, {
                    let executor = executor.clone();
                    async move {
                        accepted.send(()).unwrap();
                        let kernel_id = response.await?;
                        Ok(RemoteKernelShutdown::new(executor, async move {
                            shutdown.send(kernel_id).unwrap();
                            Ok(())
                        }))
                    }
                })
                .await
            }
        });
        wait_accepted.await.unwrap();
        drop(launch);
        respond.send("temporary-kernel").unwrap();
        assert_eq!(wait_shutdown.await.unwrap(), "temporary-kernel");
    }
}
