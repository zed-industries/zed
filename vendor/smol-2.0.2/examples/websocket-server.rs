//! A WebSocket+TLS echo server based on `async-tungstenite` and `async-native-tls`.
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

use std::net::{TcpListener, TcpStream};
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::{Context as _, Result};
use async_native_tls::{Identity, TlsAcceptor, TlsStream};
use async_tungstenite::{tungstenite, WebSocketStream};
use futures::sink::{Sink, SinkExt};
use smol::{future, prelude::*, Async};
use tungstenite::Message;

/// Echoes messages from the client back to it.
async fn echo(mut stream: WsStream) -> Result<()> {
    let msg = stream.next().await.context("expected a message")??;
    stream.send(Message::text(msg.to_string())).await?;
    Ok(())
}

/// Listens for incoming connections and serves them.
async fn listen(listener: Async<TcpListener>, tls: Option<TlsAcceptor>) -> Result<()> {
    let host = match &tls {
        None => format!("ws://{}", listener.get_ref().local_addr()?),
        Some(_) => format!("wss://{}", listener.get_ref().local_addr()?),
    };
    println!("Listening on {}", host);

    loop {
        // Accept the next connection.
        let (stream, _) = listener.accept().await?;
        println!("Accepted client: {}", stream.get_ref().peer_addr()?);

        match &tls {
            None => {
                let stream = WsStream::Plain(async_tungstenite::accept_async(stream).await?);
                smol::spawn(echo(stream)).detach();
            }
            Some(tls) => {
                // In case of WSS, establish a secure TLS connection first.
                let stream = tls.accept(stream).await?;
                let stream = WsStream::Tls(async_tungstenite::accept_async(stream).await?);
                smol::spawn(echo(stream)).detach();
            }
        }
    }
}

fn main() -> Result<()> {
    // Initialize TLS with the local certificate, private key, and password.
    let identity = Identity::from_pkcs12(include_bytes!("identity.pfx"), "password")?;
    let tls = TlsAcceptor::from(native_tls::TlsAcceptor::new(identity)?);

    // Start WS and WSS servers.
    smol::block_on(async {
        let ws = listen(Async::<TcpListener>::bind(([127, 0, 0, 1], 9000))?, None);
        let wss = listen(
            Async::<TcpListener>::bind(([127, 0, 0, 1], 9001))?,
            Some(tls),
        );
        future::try_zip(ws, wss).await?;
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
