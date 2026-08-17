//! A WebSocket+TLS client based on `async-tungstenite` and `async-native-tls`.
//!
//! First start a server:
//!
//! ```
//! cargo run --example websocket-server
//! ```
//!
//! Then start a client:
//!
//! ```
//! cargo run --example websocket-client
//! ```

use std::net::{TcpStream, ToSocketAddrs};
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::{bail, Context as _, Result};
use async_native_tls::{Certificate, TlsConnector, TlsStream};
use async_tungstenite::{tungstenite, WebSocketStream};
use futures::sink::{Sink, SinkExt};
use smol::{prelude::*, Async};
use tungstenite::handshake::client::Response;
use tungstenite::Message;
use url::Url;

/// Connects to a WebSocket address (optionally secured by TLS).
async fn connect(addr: &str, tls: TlsConnector) -> Result<(WsStream, Response)> {
    // Parse the address.
    let url = Url::parse(addr)?;
    let host = url.host_str().context("cannot parse host")?.to_string();
    let port = url.port_or_known_default().context("cannot guess port")?;

    // Resolve the address.
    let socket_addr = {
        let host = host.clone();
        smol::unblock(move || (host.as_str(), port).to_socket_addrs())
            .await?
            .next()
            .context("cannot resolve address")?
    };

    // Connect to the address.
    match url.scheme() {
        "ws" => {
            let stream = Async::<TcpStream>::connect(socket_addr).await?;
            let (stream, resp) = async_tungstenite::client_async(addr, stream).await?;
            Ok((WsStream::Plain(stream), resp))
        }
        "wss" => {
            // In case of WSS, establish a secure TLS connection first.
            let stream = Async::<TcpStream>::connect(socket_addr).await?;
            let stream = tls.connect(host, stream).await?;
            let (stream, resp) = async_tungstenite::client_async(addr, stream).await?;
            Ok((WsStream::Tls(stream), resp))
        }
        scheme => bail!("unsupported scheme: {}", scheme),
    }
}

fn main() -> Result<()> {
    // Initialize TLS with the local certificate.
    let mut builder = native_tls::TlsConnector::builder();
    builder.add_root_certificate(Certificate::from_pem(include_bytes!("certificate.pem"))?);
    let tls = TlsConnector::from(builder);

    smol::block_on(async {
        // Connect to the server.
        let (mut stream, resp) = connect("wss://127.0.0.1:9001", tls).await?;
        dbg!(resp);

        // Send a message and receive a response.
        stream.send(Message::text("Hello!")).await?;
        dbg!(stream.next().await);

        Ok(())
    })
}

/// A WebSocket or WebSocket+TLS connection.
enum WsStream {
    /// A plain WebSocket connection.
    Plain(WebSocketStream<Async<TcpStream>>),

    /// A WebSocket connection secured by TLS.
    Tls(WebSocketStream<TlsStream<Async<TcpStream>>>),
}

impl Sink<Message> for WsStream {
    type Error = tungstenite::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WsStream::Plain(s) => Pin::new(s).poll_ready(cx),
            WsStream::Tls(s) => Pin::new(s).poll_ready(cx),
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        match &mut *self {
            WsStream::Plain(s) => Pin::new(s).start_send(item),
            WsStream::Tls(s) => Pin::new(s).start_send(item),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WsStream::Plain(s) => Pin::new(s).poll_flush(cx),
            WsStream::Tls(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WsStream::Plain(s) => Pin::new(s).poll_close(cx),
            WsStream::Tls(s) => Pin::new(s).poll_close(cx),
        }
    }
}

impl Stream for WsStream {
    type Item = tungstenite::Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut *self {
            WsStream::Plain(s) => Pin::new(s).poll_next(cx),
            WsStream::Tls(s) => Pin::new(s).poll_next(cx),
        }
    }
}
