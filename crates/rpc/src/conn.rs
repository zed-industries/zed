use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{SinkExt as _, Stream, StreamExt as _};
use std::{io, task::Poll};

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
    ) -> (Self, Self, postage::watch::Sender<Option<()>>) {
        let (kill_tx, mut kill_rx) = postage::watch::channel_with(None);
        postage::stream::Stream::try_recv(&mut kill_rx).unwrap();

        let (a_tx, a_rx) = Self::channel(kill_rx.clone(), executor.clone());
        let (b_tx, b_rx) = Self::channel(kill_rx, executor);
        (
            Self { tx: a_tx, rx: b_rx },
            Self { tx: b_tx, rx: a_rx },
            kill_tx,
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    fn channel(
        kill_rx: postage::watch::Receiver<Option<()>>,
        executor: std::sync::Arc<gpui::executor::Background>,
    ) -> (
        Box<dyn Send + Unpin + futures::Sink<WebSocketMessage, Error = WebSocketError>>,
        Box<dyn Send + Unpin + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>>,
    ) {
        use futures::channel::mpsc;
        use io::{Error, ErrorKind};
        use std::sync::Arc;

        let (tx, rx) = mpsc::unbounded::<WebSocketMessage>();
        let tx = tx
            .sink_map_err(|e| WebSocketError::from(Error::new(ErrorKind::Other, e)))
            .with({
                let executor = Arc::downgrade(&executor);
                let kill_rx = kill_rx.clone();
                move |msg| {
                    let kill_rx = kill_rx.clone();
                    let executor = executor.clone();
                    Box::pin(async move {
                        if let Some(executor) = executor.upgrade() {
                            executor.simulate_random_delay().await;
                        }
                        if kill_rx.borrow().is_none() {
                            Ok(msg)
                        } else {
                            Err(Error::new(ErrorKind::Other, "connection killed").into())
                        }
                    })
                }
            });
        let rx = rx.then(move |msg| {
            let executor = Arc::downgrade(&executor);
            Box::pin(async move {
                if let Some(executor) = executor.upgrade() {
                    executor.simulate_random_delay().await;
                }
                msg
            })
        });
        let rx = KillableReceiver { kill_rx, rx };

        (Box::new(tx), Box::new(rx))
    }
}

struct KillableReceiver<S> {
    rx: S,
    kill_rx: postage::watch::Receiver<Option<()>>,
}

impl<S: Unpin + Stream<Item = WebSocketMessage>> Stream for KillableReceiver<S> {
    type Item = Result<WebSocketMessage, WebSocketError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(Some(()))) = self.kill_rx.poll_next_unpin(cx) {
            Poll::Ready(Some(Err(io::Error::new(
                io::ErrorKind::Other,
                "connection killed",
            )
            .into())))
        } else {
            self.rx.poll_next_unpin(cx).map(|value| value.map(Ok))
        }
    }
}
