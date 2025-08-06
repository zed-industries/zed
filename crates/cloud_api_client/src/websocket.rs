use futures::stream::{SplitSink, SplitStream};
use futures::{Sink, Stream, StreamExt};
use http::Request;
use std::pin::Pin;
use std::task::{Context, Poll};
use yawc::{
    WebSocket,
    close::CloseCode,
    frame::{FrameView, OpCode},
};

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

/// Generate a random WebSocket key for the Sec-WebSocket-Key header.
/// This follows RFC 6455: a base64-encoded 16-byte random value.
fn generate_websocket_key() -> String {
    use base64::Engine as _;
    use rand::RngCore;
    let mut key = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut key);
    base64::engine::general_purpose::STANDARD.encode(key)
}

/// Create a WebSocket handshake request builder with all required headers.
/// This mimics what tungstenite's IntoClientRequest does for URL strings.
pub fn build_websocket_request(
    url: &url::Url,
    custom_headers: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> anyhow::Result<http::request::Builder> {
    // Build the Host header value (include port if non-default)
    let host = match (url.host_str(), url.port()) {
        (Some(h), Some(p)) => format!("{}:{}", h, p),
        (Some(h), None) => h.to_string(),
        _ => return Err(anyhow::anyhow!("missing host in URL")),
    };

    let mut request_builder = Request::builder()
        .uri(url.as_str())
        .method("GET")
        .version(http::Version::HTTP_11)
        // Required WebSocket headers (RFC 6455)
        .header("Host", host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_websocket_key());

    // TODO: Configure WebSocket compression.
    // if let Some(compression) = options.compression.as_ref() {
    //     let extensions = WebSocketExtensions::from(compression);
    //     let header_value = extensions.to_string().parse().unwrap();
    //     req.headers_mut()
    //         .insert(header::SEC_WEBSOCKET_EXTENSIONS, header_value);
    // }

    // Add custom headers
    for (name, value) in custom_headers {
        request_builder = request_builder.header(name.as_ref(), value.as_ref());
    }

    Ok(request_builder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;

    #[test]
    fn test_generate_websocket_key() {
        // Test that we generate valid WebSocket keys
        let key1 = generate_websocket_key();
        let key2 = generate_websocket_key();

        // Keys should be different (random)
        assert_ne!(key1, key2);

        // Keys should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&key1)
            .expect("should be valid base64");

        // Should be exactly 16 bytes
        assert_eq!(decoded.len(), 16);

        // Base64 encoding of 16 bytes should be 24 characters
        // (16 * 8 / 6 = 21.33, rounded up to 24 with padding)
        assert_eq!(key1.len(), 24);

        // Should end with "==" due to padding
        assert!(key1.ends_with("=="));
    }

    #[test]
    fn test_build_websocket_request() {
        let url = url::Url::parse("wss://example.com:9000/path").unwrap();
        let custom_headers = vec![
            ("Authorization", "Bearer token123"),
            ("X-Custom-Header", "value"),
        ];

        let request_builder = build_websocket_request(&url, custom_headers).unwrap();
        let request = request_builder.body(()).unwrap();

        // Check required WebSocket headers
        assert_eq!(request.headers().get("Host").unwrap(), "example.com:9000");
        assert_eq!(request.headers().get("Connection").unwrap(), "Upgrade");
        assert_eq!(request.headers().get("Upgrade").unwrap(), "websocket");
        assert_eq!(
            request.headers().get("Sec-WebSocket-Version").unwrap(),
            "13"
        );

        // Check that Sec-WebSocket-Key exists and is valid
        let ws_key = request.headers().get("Sec-WebSocket-Key").unwrap();
        assert_eq!(ws_key.len(), 24);

        // Check custom headers
        assert_eq!(
            request.headers().get("Authorization").unwrap(),
            "Bearer token123"
        );
        assert_eq!(request.headers().get("X-Custom-Header").unwrap(), "value");

        // Check other request properties
        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(request.version(), http::Version::HTTP_11);
        assert_eq!(request.uri().to_string(), "wss://example.com:9000/path");
    }

    #[test]
    fn test_build_websocket_request_no_custom_headers() {
        let url = url::Url::parse("ws://localhost/").unwrap();
        let request_builder = build_websocket_request(&url, Vec::<(&str, &str)>::new()).unwrap();
        let request = request_builder.body(()).unwrap();

        // Should still have all required headers
        assert_eq!(request.headers().get("Host").unwrap(), "localhost");
        assert_eq!(request.headers().get("Connection").unwrap(), "Upgrade");
        assert_eq!(request.headers().get("Upgrade").unwrap(), "websocket");
        assert_eq!(
            request.headers().get("Sec-WebSocket-Version").unwrap(),
            "13"
        );
        assert!(request.headers().contains_key("Sec-WebSocket-Key"));
    }

    #[test]
    fn test_message_conversions() {
        // Test Text message
        let text = "Hello, WebSocket!";
        let msg = Message::Text(text.to_string());
        let frame = msg.clone().into_frame_view();
        assert_eq!(frame.opcode, OpCode::Text);
        assert_eq!(frame.payload.as_ref(), text.as_bytes());

        // Test Binary message
        let data = vec![1, 2, 3, 4, 5];
        let msg = Message::Binary(data.clone());
        let frame = msg.into_frame_view();
        assert_eq!(frame.opcode, OpCode::Binary);
        assert_eq!(frame.payload.as_ref(), &data);

        // Test Ping message
        let msg = Message::Ping;
        let frame = msg.into_frame_view();
        assert_eq!(frame.opcode, OpCode::Ping);

        // Test Pong message
        let msg = Message::Pong;
        let frame = msg.into_frame_view();
        assert_eq!(frame.opcode, OpCode::Pong);

        // Test Close message with reason
        let msg = Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "Normal closure".to_string(),
        }));
        let frame = msg.into_frame_view();
        assert_eq!(frame.opcode, OpCode::Close);
        // Close frames encode the status code in the first 2 bytes
        assert_eq!(frame.payload[0], 0x03); // 1000 = 0x03E8, high byte
        assert_eq!(frame.payload[1], 0xE8); // 1000 = 0x03E8, low byte

        // Test Close message without reason
        let msg = Message::Close(None);
        let frame = msg.into_frame_view();
        assert_eq!(frame.opcode, OpCode::Close);
    }
}
