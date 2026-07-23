use std::time::Duration;

use anyhow::{Result, anyhow};

use futures::channel::mpsc::unbounded;
use futures::stream::SplitStream;
use futures::{FutureExt as _, StreamExt as _, TryStreamExt as _};
use gpui::{App, Task};
use yawc::WebSocket;

use super::{MessageStream, forward_frame};
use crate::CloudApiClient;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct Connection {
    rx: SplitStream<WebSocket>,
}

impl Connection {
    fn new(websocket: WebSocket) -> Self {
        let (_, rx) = websocket.split();
        Self { rx }
    }

    pub fn spawn(self, cx: &App) -> (MessageStream, Task<()>) {
        let mut rx = self.rx;
        let (message_tx, message_rx) = unbounded();
        let task = cx.spawn(async move |_cx| {
            while let Some(frame) = rx.next().await {
                let frame = match frame {
                    Ok(frame) => frame,
                    Err(error) => {
                        let error = anyhow!("Cloud WebSocket error: {error}");
                        if message_tx.unbounded_send(Err(error)).is_err() {
                            break;
                        }
                        continue;
                    }
                };
                if !forward_frame(frame, &message_tx) {
                    break;
                }
            }
        });

        (message_rx.into_stream().boxed(), task)
    }
}

impl CloudApiClient {
    pub fn connect(self: &std::sync::Arc<Self>, cx: &App) -> Result<Task<Result<Connection>>> {
        let client = self.clone();
        let executor = cx.background_executor().clone();
        Ok(cx.spawn(async move |_cx| {
            let mut connect_url = client
                .http_client
                .build_zed_cloud_url("/client/users/connect")?;
            connect_url
                .set_scheme(match connect_url.scheme() {
                    "https" => "wss",
                    "http" => "ws",
                    scheme => return Err(anyhow!("invalid URL scheme: {scheme}")),
                })
                .map_err(|_| anyhow!("failed to set URL scheme"))?;


            let connect = WebSocket::connect(connect_url).fuse();
            let timeout = executor.timer(CONNECT_TIMEOUT).fuse();
            futures::pin_mut!(connect, timeout);
            let websocket = futures::select_biased! {
                result = connect => result.map_err(|error| anyhow!("failed to connect to Cloud WebSocket: {error}"))?,
                _ = timeout => return Err(anyhow!("timed out connecting to Cloud WebSocket")),
            };

            Ok(Connection::new(websocket))
        }))
    }
}
