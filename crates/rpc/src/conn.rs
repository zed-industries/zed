use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{SinkExt as _, StreamExt as _};

pub struct Connection {
    pub(crate) tx:
        Box<dyn 'static + Send + Unpin + futures::Sink<WebSocketMessage, Error = WebSocketError>>,
    pub(crate) rx: Box<
        dyn 'static
            + Send
            + Unpin
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>,
    >,
}

impl Connection {
    pub fn new<S>(stream: S) -> Self
    where
        S: 'static
            + Send
            + Unpin
            + futures::Sink<WebSocketMessage, Error = WebSocketError>
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>,
    {
        let (tx, rx) = stream.split();
        Self {
            tx: Box::new(tx),
            rx: Box::new(rx),
        }
    }

    pub async fn send(&mut self, message: WebSocketMessage) -> Result<(), WebSocketError> {
        self.tx.send(message).await
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn in_memory(
        executor: std::sync::Arc<gpui::executor::Background>,
    ) -> (Self, Self, postage::barrier::Sender) {
        use postage::prelude::Stream;

        let (kill_tx, kill_rx) = postage::barrier::channel();
        let (a_tx, a_rx) = channel(kill_rx.clone(), executor.clone());
        let (b_tx, b_rx) = channel(kill_rx, executor);
        return (
            Self { tx: a_tx, rx: b_rx },
            Self { tx: b_tx, rx: a_rx },
            kill_tx,
        );

        fn channel(
            kill_rx: postage::barrier::Receiver,
            executor: std::sync::Arc<gpui::executor::Background>,
        ) -> (
            Box<dyn Send + Unpin + futures::Sink<WebSocketMessage, Error = WebSocketError>>,
            Box<
                dyn Send + Unpin + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>,
            >,
        ) {
            use futures::channel::mpsc;
            use std::{
                io::{Error, ErrorKind},
                sync::Arc,
            };

            let (tx, rx) = mpsc::unbounded::<WebSocketMessage>();

            let tx = tx
                .sink_map_err(|e| WebSocketError::from(Error::new(ErrorKind::Other, e)))
                .with({
                    let kill_rx = kill_rx.clone();
                    let executor = Arc::downgrade(&executor);
                    move |msg| {
                        let mut kill_rx = kill_rx.clone();
                        let executor = executor.clone();
                        Box::pin(async move {
                            if let Some(executor) = executor.upgrade() {
                                executor.simulate_random_delay().await;
                            }

                            // Writes to a half-open TCP connection will error.
                            if kill_rx.try_recv().is_ok() {
                                std::io::Result::Err(
                                    Error::new(ErrorKind::Other, "connection lost").into(),
                                )?;
                            }

                            Ok(msg)
                        })
                    }
                });

            let rx = rx.then({
                let kill_rx = kill_rx.clone();
                let executor = Arc::downgrade(&executor);
                move |msg| {
                    let mut kill_rx = kill_rx.clone();
                    let executor = executor.clone();
                    Box::pin(async move {
                        if let Some(executor) = executor.upgrade() {
                            executor.simulate_random_delay().await;
                        }

                        // Reads from a half-open TCP connection will hang.
                        if kill_rx.try_recv().is_ok() {
                            futures::future::pending::<()>().await;
                        }

                        Ok(msg)
                    })
                }
            });

            (Box::new(tx), Box::new(rx))
        }
    }
}
