use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use cloud_api_types::websocket_protocol::{PROTOCOL_VERSION, PROTOCOL_VERSION_HEADER_NAME};
use futures::channel::mpsc::unbounded;
use futures::stream::{SplitSink, SplitStream};
use futures::{FutureExt as _, SinkExt as _, StreamExt as _, TryStreamExt as _};
use gpui::{App, Task};
use http_client::http::request;
use yawc::frame::Frame;
use yawc::{TcpWebSocket, WebSocket};

use super::{MessageStream, forward_frame};
use crate::CloudApiClient;

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(1);

pub struct Connection {
    tx: SplitSink<TcpWebSocket, Frame>,
    rx: SplitStream<TcpWebSocket>,
}

impl Connection {
    fn new(websocket: TcpWebSocket) -> Self {
        let (tx, rx) = websocket.split();
        Self { tx, rx }
    }

    pub fn spawn(self, cx: &App) -> (MessageStream, Task<()>) {
        let mut tx = self.tx;
        let rx = self.rx.fuse();
        let (message_tx, message_rx) = unbounded();
        let executor = cx.background_executor().clone();
        let task = cx.spawn(async move |_cx| {
            let keepalive_timer = executor.timer(KEEPALIVE_INTERVAL).fuse();
            futures::pin_mut!(keepalive_timer, rx);

            loop {
                futures::select_biased! {
                    _ = keepalive_timer => {
                        if tx.send(Frame::ping(Vec::new())).await.is_err() {
                            break;
                        }
                        keepalive_timer.set(executor.timer(KEEPALIVE_INTERVAL).fuse());
                    }
                    frame = rx.next() => {
                        let Some(frame) = frame else {
                            break;
                        };
                        if !forward_frame(frame, &message_tx) {
                            break;
                        }
                    }
                }
            }
        });

        (message_rx.into_stream().boxed(), task)
    }
}

impl CloudApiClient {
    pub fn connect(self: &std::sync::Arc<Self>, cx: &App) -> Result<Task<Result<Connection>>> {
        let mut connect_url = self
            .http_client
            .build_zed_cloud_url("/client/users/connect")?;
        connect_url
            .set_scheme(match connect_url.scheme() {
                "https" => "wss",
                "http" => "ws",
                scheme => Err(anyhow!("invalid URL scheme: {scheme}"))?,
            })
            .map_err(|_| anyhow!("failed to set URL scheme"))?;

        let credentials = self.credentials.read();
        let credentials = credentials.as_ref().context("no credentials provided")?;
        let authorization_header = format!("{} {}", credentials.user_id, credentials.access_token);

        Ok(gpui_tokio::Tokio::spawn_result(cx, async move {
            let websocket = WebSocket::connect(connect_url)
                .with_request(
                    request::Builder::new()
                        .header("Authorization", authorization_header)
                        .header(PROTOCOL_VERSION_HEADER_NAME, PROTOCOL_VERSION.to_string()),
                )
                .await?;

            Ok(Connection::new(websocket))
        }))
    }
}
