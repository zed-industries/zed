use futures::{SinkExt as _, StreamExt as _};
use async_tungstenite::tungstenite::Message as TungsteniteMessage;

// Original connection type for tungstenite (used by /rpc endpoint)
pub struct Connection {
    pub(crate) tx:
        Box<dyn 'static + Send + Unpin + futures::Sink<TungsteniteMessage, Error = anyhow::Error>>,
    pub(crate) rx:
        Box<dyn 'static + Send + Unpin + futures::Stream<Item = anyhow::Result<TungsteniteMessage>>>,
}

impl Connection {
    pub fn new<S>(stream: S) -> Self
    where
        S: 'static
            + Send
            + Unpin
            + futures::Sink<TungsteniteMessage, Error = anyhow::Error>
            + futures::Stream<Item = anyhow::Result<TungsteniteMessage>>,
    {
        let (tx, rx) = stream.split();
        Self {
            tx: Box::new(tx),
            rx: Box::new(rx),
        }
    }

    pub async fn send(&mut self, message: TungsteniteMessage) -> anyhow::Result<()> {
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
            Box<dyn 'static + Send + Unpin + futures::Sink<TungsteniteMessage, Error = anyhow::Error>>,
            Box<
                dyn 'static
                    + Send
                    + Unpin
                    + futures::Stream<Item = anyhow::Result<TungsteniteMessage>>,
            >,
        ) {
            use anyhow::anyhow;
            use futures::channel::mpsc;
            use std::io::{Error, ErrorKind};

            let (tx, rx) = mpsc::unbounded::<TungsteniteMessage>();

            let tx = tx.sink_map_err(|err| anyhow!(err)).with({
                let killed = killed.clone();
                let executor = executor.clone();
                move |msg| {
                    let killed = killed.clone();
                    let executor = executor.clone();
                    Box::pin(async move {
                        if killed.load(SeqCst) {
                            std::io::Result::Err(Error::new(ErrorKind::Other, "connection lost"))?;
                        }

                        executor.timer(std::time::Duration::from_millis(2)).await;
                        Ok(msg)
                    })
                }
            });

            let rx = rx.map({
                move |msg| {
                    if killed.load(SeqCst) {
                        std::io::Result::Err(Error::new(ErrorKind::Other, "connection lost"))?;
                    }
                    Ok(msg)
                }
            });

            (Box::new(tx), Box::new(rx))
        }
    }
}

// New connection type for yawc (used by /cloud endpoint)
pub struct YawcConnection {
    pub(crate) _adapter: crate::websocket_yawc::WebSocketAdapter,
}

impl YawcConnection {
    pub fn new(adapter: crate::websocket_yawc::WebSocketAdapter) -> Self {
        Self { _adapter: adapter }
    }
}