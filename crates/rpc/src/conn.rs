use async_tungstenite::tungstenite::Message as WebSocketMessage;
use futures::{SinkExt as _, StreamExt as _};

pub struct Connection {
    pub(crate) tx:
        Box<dyn 'static + Send + Unpin + futures::Sink<WebSocketMessage, Error = anyhow::Error>>,
    pub(crate) rx:
        Box<dyn 'static + Send + Unpin + futures::Stream<Item = anyhow::Result<WebSocketMessage>>>,
}

impl Connection {
    pub fn new<S>(stream: S) -> Self
    where
        S: 'static
            + Send
            + Unpin
            + futures::Sink<WebSocketMessage, Error = anyhow::Error>
            + futures::Stream<Item = anyhow::Result<WebSocketMessage>>,
    {
        let (tx, rx) = stream.split();
        Self {
            tx: Box::new(tx),
            rx: Box::new(rx),
        }
    }

    pub async fn send(&mut self, message: WebSocketMessage) -> anyhow::Result<()> {
        self.tx.send(message).await
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn in_memory(
        executor: gpui::BackgroundExecutor,
    ) -> (Self, Self, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        use std::sync::{
            Arc,
            atomic::{AtomicBool, Ordering::SeqCst},
        };

        let killed = Arc::new(AtomicBool::new(false));
        let (a_tx, a_rx) = channel(killed.clone(), executor.clone());
        let (b_tx, b_rx) = channel(killed.clone(), executor);
        return (
            Self { tx: a_tx, rx: b_rx },
            Self { tx: b_tx, rx: a_rx },
            killed,
        );

        #[allow(clippy::type_complexity)]
        fn channel(
            killed: Arc<AtomicBool>,
            executor: gpui::BackgroundExecutor,
        ) -> (
            Box<dyn Send + Unpin + futures::Sink<WebSocketMessage, Error = anyhow::Error>>,
            Box<dyn Send + Unpin + futures::Stream<Item = anyhow::Result<WebSocketMessage>>>,
        ) {
            use anyhow::anyhow;
            use futures::channel::mpsc;
            use std::io::Error;

            let (tx, rx) = mpsc::unbounded::<WebSocketMessage>();

            let tx = tx.sink_map_err(|error| anyhow!(error)).with({
                let killed = killed.clone();
                let executor = executor.clone();
                move |msg| {
                    let killed = killed.clone();
                    let executor = executor.clone();
                    Box::pin(async move {
                        executor.simulate_random_delay().await;

                        // Writes to a half-open TCP connection will error.
                        if killed.load(SeqCst) {
                            std::io::Result::Err(Error::other("connection lost"))?;
                        }

                        Ok(msg)
                    })
                }
            });

            let rx = rx.then({
                move |msg| {
                    let killed = killed.clone();
                    let executor = executor.clone();
                    Box::pin(async move {
                        executor.simulate_random_delay().await;

                        // Reads from a half-open TCP connection will hang.
                        if killed.load(SeqCst) {
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
