use std::pin::Pin;
#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

use anyhow::Result;
use cloud_api_types::websocket_protocol::MessageToClient;
use futures::channel::mpsc::{UnboundedSender, unbounded};
#[cfg(not(target_family = "wasm"))]
use futures::stream::SplitSink;
use futures::stream::SplitStream;
#[cfg(not(target_family = "wasm"))]
use futures::{FutureExt as _, SinkExt as _};
use futures::{Stream, StreamExt as _, TryStreamExt as _};
use gpui::{App, Task};
use yawc::frame::{Frame, OpCode};

#[cfg(not(target_family = "wasm"))]
use yawc::TcpWebSocket as WebSocket;
#[cfg(target_family = "wasm")]
use yawc::WebSocket;

#[cfg(not(target_family = "wasm"))]
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(1);

pub type MessageStream = Pin<Box<dyn Stream<Item = Result<MessageToClient>>>>;

pub struct Connection {
    #[cfg(not(target_family = "wasm"))]
    tx: SplitSink<WebSocket, Frame>,
    rx: SplitStream<WebSocket>,
}

impl Connection {
    pub fn new(websocket: WebSocket) -> Self {
        #[cfg(not(target_family = "wasm"))]
        let (tx, rx) = websocket.split();
        #[cfg(target_family = "wasm")]
        let (_, rx) = websocket.split();

        Self {
            #[cfg(not(target_family = "wasm"))]
            tx,
            rx,
        }
    }

    pub fn spawn(self, cx: &App) -> (MessageStream, Task<()>) {
        let (message_tx, message_rx) = unbounded();

        #[cfg(not(target_family = "wasm"))]
        let task = {
            let mut tx = self.tx;
            let rx = self.rx.fuse();
            let executor = cx.background_executor().clone();
            cx.spawn(async move |_cx| {
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
            })
        };

        #[cfg(target_family = "wasm")]
        let task = {
            let mut rx = self.rx;
            cx.spawn(async move |_cx| {
                while let Some(frame) = rx.next().await {
                    let frame = match frame {
                        Ok(frame) => frame,
                        Err(error) => {
                            let error = anyhow::anyhow!("Cloud WebSocket error: {error}");
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
            })
        };

        (message_rx.into_stream().boxed(), task)
    }
}

fn forward_frame(frame: Frame, message_tx: &UnboundedSender<Result<MessageToClient>>) -> bool {
    match frame.opcode() {
        OpCode::Binary => message_tx
            .unbounded_send(MessageToClient::deserialize(frame.payload()))
            .is_ok(),
        OpCode::Close => false,
        OpCode::Continuation | OpCode::Text | OpCode::Ping | OpCode::Pong => true,
    }
}
