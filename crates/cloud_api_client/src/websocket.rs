use std::pin::Pin;
use std::time::Duration;

use anyhow::Result;
use cloud_api_types::websocket_protocol::MessageToClient;
use futures::channel::mpsc::unbounded;
use futures::stream::{SplitSink, SplitStream};
use futures::{FutureExt as _, SinkExt as _, Stream, StreamExt as _, TryStreamExt as _, pin_mut};
use gpui::{App, BackgroundExecutor, Task};
use yawc::WebSocket;
use yawc::frame::{FrameView, OpCode};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(1);

pub type MessageStream = Pin<Box<dyn Stream<Item = Result<MessageToClient>>>>;

pub struct Connection {
    tx: SplitSink<WebSocket, FrameView>,
    rx: SplitStream<WebSocket>,
}

impl Connection {
    pub fn new(ws: WebSocket) -> Self {
        let (tx, rx) = ws.split();

        Self { tx, rx }
    }

    pub fn spawn(self, cx: &App) -> (MessageStream, Task<()>) {
        let (mut tx, rx) = (self.tx, self.rx);

        let (message_tx, message_rx) = unbounded();

        let handle_io = |executor: BackgroundExecutor| async move {
            // Send messages on this frequency so the connection isn't closed.
            let keepalive_timer = executor.timer(KEEPALIVE_INTERVAL).fuse();
            futures::pin_mut!(keepalive_timer);

            let rx = rx.fuse();
            pin_mut!(rx);

            loop {
                futures::select_biased! {
                    _ = keepalive_timer => {
                        let _ = tx.send(FrameView::ping(Vec::new())).await;

                        keepalive_timer.set(executor.timer(KEEPALIVE_INTERVAL).fuse());
                    }
                    frame = rx.next() => {
                        let Some(frame) = frame else {
                            break;
                        };

                        match frame.opcode {
                            OpCode::Binary => {
                                let message_result = MessageToClient::deserialize(&frame.payload);
                                message_tx.unbounded_send(message_result).ok();
                            }
                            OpCode::Close => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        };

        let task = cx.spawn(async move |cx| handle_io(cx.background_executor().clone()).await);

        (message_rx.into_stream().boxed(), task)
    }
}
