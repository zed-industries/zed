use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::{SplitSink, SplitStream};
use futures::{Sink, Stream, StreamExt};
use yawc::WebSocket;
use yawc::close::CloseCode;
use yawc::frame::{FrameView, OpCode};

#[derive(Debug, Clone)]
pub struct CloseFrame {
    pub code: CloseCode,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Ping,
    Pong,
    Close(Option<CloseFrame>),
}

impl Message {
    pub fn into_frame_view(self) -> FrameView {
        match self {
            Message::Text(text) => FrameView::text(text),
            Message::Binary(data) => FrameView::binary(data),
            Message::Ping => FrameView::ping(Vec::new()),
            Message::Pong => FrameView::pong(Vec::new()),
            Message::Close(frame) => {
                if let Some(frame) = frame {
                    FrameView::close(frame.code, frame.reason)
                } else {
                    FrameView::close(CloseCode::Normal, "")
                }
            }
        }
    }

    pub fn from_frame_view(frame: FrameView) -> Option<Self> {
        match frame.opcode {
            OpCode::Text => String::from_utf8(frame.payload.to_vec())
                .ok()
                .map(Message::Text),
            OpCode::Binary => Some(Message::Binary(frame.payload.to_vec())),
            OpCode::Ping => Some(Message::Ping),
            OpCode::Pong => Some(Message::Pong),
            OpCode::Close => {
                if frame.payload.len() >= 2 {
                    let code = u16::from_be_bytes([frame.payload[0], frame.payload[1]]);
                    let reason = String::from_utf8_lossy(&frame.payload[2..]).into_owned();
                    Some(Message::Close(Some(CloseFrame {
                        code: CloseCode::from(code),
                        reason,
                    })))
                } else {
                    Some(Message::Close(None))
                }
            }
            _ => None,
        }
    }
}

pub struct Connection {
    rx: SplitSink<WebSocket, FrameView>,
    tx: SplitStream<WebSocket>,
}

impl Connection {
    pub fn new(ws: WebSocket) -> Self {
        let (rx, tx) = ws.split();

        Self { rx, tx }
    }
}

pub enum WebSocketAdapter {
    Yawc(WebSocket),
    Stream {
        rx: Box<dyn Stream<Item = anyhow::Result<Message>> + Send + Unpin>,
        tx: Box<dyn Sink<Message, Error = anyhow::Error> + Send + Unpin>,
    },
}

impl WebSocketAdapter {
    pub fn new(ws: WebSocket) -> Self {
        Self::Yawc(ws)
    }

    pub fn new_from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = anyhow::Result<Message>>
            + Sink<Message, Error = anyhow::Error>
            + Send
            + Unpin
            + 'static,
    {
        let (tx, rx) = stream.split();
        Self::Stream {
            rx: Box::new(rx),
            tx: Box::new(tx),
        }
    }
}

impl Stream for WebSocketAdapter {
    type Item = anyhow::Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut *self {
            WebSocketAdapter::Yawc(ws) => match Pin::new(ws).poll_next(cx) {
                Poll::Ready(Some(frame)) => {
                    if let Some(msg) = Message::from_frame_view(frame) {
                        Poll::Ready(Some(Ok(msg)))
                    } else {
                        self.poll_next(cx)
                    }
                }
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            },
            WebSocketAdapter::Stream { rx, .. } => Pin::new(rx).poll_next(cx),
        }
    }
}

impl Sink<Message> for WebSocketAdapter {
    type Error = anyhow::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketAdapter::Yawc(ws) => {
                Pin::new(ws).poll_ready(cx).map_err(|e| anyhow::anyhow!(e))
            }
            WebSocketAdapter::Stream { tx, .. } => Pin::new(tx).poll_ready(cx),
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        match &mut *self {
            WebSocketAdapter::Yawc(ws) => Pin::new(ws)
                .start_send(item.into_frame_view())
                .map_err(|e| anyhow::anyhow!(e)),
            WebSocketAdapter::Stream { tx, .. } => Pin::new(tx).start_send(item),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketAdapter::Yawc(ws) => {
                Pin::new(ws).poll_flush(cx).map_err(|e| anyhow::anyhow!(e))
            }
            WebSocketAdapter::Stream { tx, .. } => Pin::new(tx).poll_flush(cx),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketAdapter::Yawc(ws) => {
                Pin::new(ws).poll_close(cx).map_err(|e| anyhow::anyhow!(e))
            }
            WebSocketAdapter::Stream { tx, .. } => Pin::new(tx).poll_close(cx),
        }
    }
}
