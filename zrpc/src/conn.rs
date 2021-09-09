use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{SinkExt as _, StreamExt as _};

pub struct Conn {
    pub(crate) tx:
        Box<dyn 'static + Send + Unpin + futures::Sink<WebSocketMessage, Error = WebSocketError>>,
    pub(crate) rx: Box<
        dyn 'static
            + Send
            + Unpin
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>,
    >,
}

impl Conn {
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
    pub fn in_memory() -> (Self, Self) {
        use futures::SinkExt as _;
        use futures::StreamExt as _;
        use std::io::{Error, ErrorKind};

        let (a_tx, a_rx) = futures::channel::mpsc::unbounded::<WebSocketMessage>();
        let (b_tx, b_rx) = futures::channel::mpsc::unbounded::<WebSocketMessage>();
        (
            Self {
                tx: Box::new(a_tx.sink_map_err(|e| Error::new(ErrorKind::Other, e).into())),
                rx: Box::new(b_rx.map(Ok)),
            },
            Self {
                tx: Box::new(b_tx.sink_map_err(|e| Error::new(ErrorKind::Other, e).into())),
                rx: Box::new(a_rx.map(Ok)),
            },
        )
    }
}
