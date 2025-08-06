use std::time::Duration;

use futures::stream::{SplitSink, SplitStream};
use futures::{FutureExt as _, SinkExt as _, StreamExt, pin_mut};
use gpui::{App, BackgroundExecutor, Task};
use yawc::WebSocket;
use yawc::frame::{FrameView, OpCode};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(1);

pub struct Connection {
    tx: SplitSink<WebSocket, FrameView>,
    rx: SplitStream<WebSocket>,
}

impl Connection {
    pub fn new(ws: WebSocket) -> Self {
        let (tx, rx) = ws.split();

        Self { tx, rx }
    }

    pub fn spawn(self, cx: &App) -> Task<()> {
        let (mut tx, rx) = (self.tx, self.rx);

        let handle_io = |executor: BackgroundExecutor| async move {
            // Send messages on this frequency so the connection isn't closed.
            let keepalive_timer = executor.timer(KEEPALIVE_INTERVAL).fuse();
            futures::pin_mut!(keepalive_timer);

            let rx = rx.fuse();
            pin_mut!(rx);

            loop {
                futures::select_biased! {
                    _ = keepalive_timer => {
                        let _ = tx.send(FrameView::ping("ping")).await;

                        keepalive_timer.set(executor.timer(KEEPALIVE_INTERVAL).fuse());
                    }
                    frame = rx.next() => {
                        let Some(frame) = frame else {
                            break;
                        };

                        println!("OpCode: {:?}", frame.opcode);
                        match frame.opcode {
                            OpCode::Binary => {
                                println!("Got payload: {:?}", frame.payload);
                            }
                            OpCode::Text => {
                                let text = std::str::from_utf8(&frame.payload).unwrap();
                                println!("received: {text}");
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

        cx.spawn(async move |cx| handle_io(cx.background_executor().clone()).await)
    }
}
