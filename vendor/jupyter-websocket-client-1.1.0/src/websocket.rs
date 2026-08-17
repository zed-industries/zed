use anyhow::{Context, Result};
use async_tungstenite::{tokio::ConnectStream, tungstenite::Message, WebSocketStream};
use futures::{Sink, SinkExt as _, Stream, StreamExt};

use crate::binary_protocol::{default_channel_for_message, deserialize_v1, serialize_v1};
use jupyter_protocol::{JupyterConnection, JupyterMessage};
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};

/// Protocol mode for WebSocket communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtocolMode {
    /// JSON text messages (legacy)
    #[default]
    Json,
    /// Binary v1 protocol (jupyter-server-documents)
    BinaryV1,
}

#[derive(Debug)]
pub struct JupyterWebSocket {
    pub inner: WebSocketStream<ConnectStream>,
    /// Protocol mode for sending/receiving messages
    pub protocol_mode: ProtocolMode,
}

impl Stream for JupyterWebSocket {
    type Item = Result<JupyterMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        // Loop to skip control frames (ping/pong) and unexpected frame types
        loop {
            match self.inner.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(msg))) => match msg {
                    Message::Text(text) => {
                        return Poll::Ready(Some(
                            serde_json::from_str(&text)
                                .context("Failed to parse JSON")
                                .and_then(|value| {
                                    JupyterMessage::from_value(value)
                                        .context("Failed to create JupyterMessage")
                                }),
                        ));
                    }
                    Message::Binary(data) => {
                        return Poll::Ready(Some(
                            deserialize_v1(&data)
                                .map(|(_channel, msg)| msg)
                                .context("Failed to parse binary v1 message"),
                        ));
                    }
                    // Ping/Pong are handled automatically by tungstenite - skip and continue
                    Message::Ping(_) | Message::Pong(_) => {
                        continue;
                    }
                    // Close frame signals end of stream
                    Message::Close(_) => {
                        return Poll::Ready(None);
                    }
                    // Raw frames are unexpected - log and skip them
                    Message::Frame(_) => {
                        eprintln!("jupyter-websocket-client: received unexpected raw frame, skipping");
                        continue;
                    }
                },
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e.into()))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

impl Sink<JupyterMessage> for JupyterWebSocket {
    type Error = anyhow::Error;

    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready_unpin(cx).map_err(Into::into)
    }

    fn start_send(mut self: Pin<&mut Self>, item: JupyterMessage) -> Result<(), Self::Error> {
        let ws_message = match self.protocol_mode {
            ProtocolMode::Json => {
                let message_str =
                    serde_json::to_string(&item).context("Failed to serialize JupyterMessage")?;
                Message::Text(message_str.into())
            }
            ProtocolMode::BinaryV1 => {
                let channel = default_channel_for_message(&item);
                let data =
                    serialize_v1(&item, channel).context("Failed to serialize to binary v1")?;
                Message::Binary(data.into())
            }
        };
        self.inner.start_send_unpin(ws_message).map_err(Into::into)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_flush_unpin(cx).map_err(Into::into)
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_close_unpin(cx).map_err(Into::into)
    }
}

impl JupyterConnection for JupyterWebSocket {}

pub type JupyterWebSocketReader = futures::stream::SplitStream<JupyterWebSocket>;
pub type JupyterWebSocketWriter = futures::stream::SplitSink<JupyterWebSocket, JupyterMessage>;
