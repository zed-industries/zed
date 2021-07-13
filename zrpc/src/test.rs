use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Channel {
    tx: futures::channel::mpsc::UnboundedSender<WebSocketMessage>,
    rx: futures::channel::mpsc::UnboundedReceiver<WebSocketMessage>,
}

impl Channel {
    pub fn new() -> Self {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        Self { tx, rx }
    }

    pub fn bidirectional() -> (Self, Self) {
        let (a_tx, a_rx) = futures::channel::mpsc::unbounded();
        let (b_tx, b_rx) = futures::channel::mpsc::unbounded();
        let a = Self { tx: a_tx, rx: b_rx };
        let b = Self { tx: b_tx, rx: a_rx };
        (a, b)
    }
}

impl futures::Sink<WebSocketMessage> for Channel {
    type Error = WebSocketError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx)
            .poll_ready(cx)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err).into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: WebSocketMessage) -> Result<(), Self::Error> {
        Pin::new(&mut self.tx)
            .start_send(item)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err).into())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx)
            .poll_flush(cx)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err).into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx)
            .poll_close(cx)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err).into())
    }
}

impl futures::Stream for Channel {
    type Item = Result<WebSocketMessage, WebSocketError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.rx)
            .poll_next(cx)
            .map(|i| i.map(|i| Ok(i)))
    }
}
