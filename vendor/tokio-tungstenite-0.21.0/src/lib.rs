//! Async WebSocket usage.
//!
//! This library is an implementation of WebSocket handshakes and streams. It
//! is based on the crate which implements all required WebSocket protocol
//! logic. So this crate basically just brings tokio support / tokio integration
//! to it.
//!
//! Each WebSocket stream implements the required `Stream` and `Sink` traits,
//! so the socket is just a stream of messages coming in and going out.

#![deny(missing_docs, unused_must_use, unused_mut, unused_imports, unused_import_braces)]

pub use tungstenite;

mod compat;
#[cfg(feature = "connect")]
mod connect;
mod handshake;
#[cfg(feature = "stream")]
mod stream;
#[cfg(any(feature = "native-tls", feature = "__rustls-tls", feature = "connect"))]
mod tls;

use std::io::{Read, Write};

use compat::{cvt, AllowStd, ContextWaker};
use futures_util::{
    sink::{Sink, SinkExt},
    stream::{FusedStream, Stream},
};
use log::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite};

#[cfg(feature = "handshake")]
use tungstenite::{
    client::IntoClientRequest,
    handshake::{
        client::{ClientHandshake, Response},
        server::{Callback, NoCallback},
        HandshakeError,
    },
};
use tungstenite::{
    error::Error as WsError,
    protocol::{Message, Role, WebSocket, WebSocketConfig},
};

#[cfg(any(feature = "native-tls", feature = "__rustls-tls", feature = "connect"))]
pub use tls::Connector;
#[cfg(any(feature = "native-tls", feature = "__rustls-tls"))]
pub use tls::{client_async_tls, client_async_tls_with_config};

#[cfg(feature = "connect")]
pub use connect::{connect_async, connect_async_with_config};

#[cfg(all(any(feature = "native-tls", feature = "__rustls-tls"), feature = "connect"))]
pub use connect::connect_async_tls_with_config;

#[cfg(feature = "stream")]
pub use stream::MaybeTlsStream;

use tungstenite::protocol::CloseFrame;

/// Creates a WebSocket handshake from a request and a stream.
/// For convenience, the user may call this with a url string, a URL,
/// or a `Request`. Calling with `Request` allows the user to add
/// a WebSocket protocol or other custom headers.
///
/// Internally, this custom creates a handshake representation and returns
/// a future representing the resolution of the WebSocket handshake. The
/// returned future will resolve to either `WebSocketStream<S>` or `Error`
/// depending on whether the handshake is successful.
///
/// This is typically used for clients who have already established, for
/// example, a TCP connection to the remote server.
#[cfg(feature = "handshake")]
pub async fn client_async<'a, R, S>(
    request: R,
    stream: S,
) -> Result<(WebSocketStream<S>, Response), WsError>
where
    R: IntoClientRequest + Unpin,
    S: AsyncRead + AsyncWrite + Unpin,
{
    client_async_with_config(request, stream, None).await
}

/// The same as `client_async()` but the one can specify a websocket configuration.
/// Please refer to `client_async()` for more details.
#[cfg(feature = "handshake")]
pub async fn client_async_with_config<'a, R, S>(
    request: R,
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<S>, Response), WsError>
where
    R: IntoClientRequest + Unpin,
    S: AsyncRead + AsyncWrite + Unpin,
{
    let f = handshake::client_handshake(stream, move |allow_std| {
        let request = request.into_client_request()?;
        let cli_handshake = ClientHandshake::start(allow_std, request, config)?;
        cli_handshake.handshake()
    });
    f.await.map_err(|e| match e {
        HandshakeError::Failure(e) => e,
        e => WsError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
    })
}

/// Accepts a new WebSocket connection with the provided stream.
///
/// This function will internally call `server::accept` to create a
/// handshake representation and returns a future representing the
/// resolution of the WebSocket handshake. The returned future will resolve
/// to either `WebSocketStream<S>` or `Error` depending if it's successful
/// or not.
///
/// This is typically used after a socket has been accepted from a
/// `TcpListener`. That socket is then passed to this function to perform
/// the server half of the accepting a client's websocket connection.
#[cfg(feature = "handshake")]
pub async fn accept_async<S>(stream: S) -> Result<WebSocketStream<S>, WsError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    accept_hdr_async(stream, NoCallback).await
}

/// The same as `accept_async()` but the one can specify a websocket configuration.
/// Please refer to `accept_async()` for more details.
#[cfg(feature = "handshake")]
pub async fn accept_async_with_config<S>(
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<WebSocketStream<S>, WsError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    accept_hdr_async_with_config(stream, NoCallback, config).await
}

/// Accepts a new WebSocket connection with the provided stream.
///
/// This function does the same as `accept_async()` but accepts an extra callback
/// for header processing. The callback receives headers of the incoming
/// requests and is able to add extra headers to the reply.
#[cfg(feature = "handshake")]
pub async fn accept_hdr_async<S, C>(stream: S, callback: C) -> Result<WebSocketStream<S>, WsError>
where
    S: AsyncRead + AsyncWrite + Unpin,
    C: Callback + Unpin,
{
    accept_hdr_async_with_config(stream, callback, None).await
}

/// The same as `accept_hdr_async()` but the one can specify a websocket configuration.
/// Please refer to `accept_hdr_async()` for more details.
#[cfg(feature = "handshake")]
pub async fn accept_hdr_async_with_config<S, C>(
    stream: S,
    callback: C,
    config: Option<WebSocketConfig>,
) -> Result<WebSocketStream<S>, WsError>
where
    S: AsyncRead + AsyncWrite + Unpin,
    C: Callback + Unpin,
{
    let f = handshake::server_handshake(stream, move |allow_std| {
        tungstenite::accept_hdr_with_config(allow_std, callback, config)
    });
    f.await.map_err(|e| match e {
        HandshakeError::Failure(e) => e,
        e => WsError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
    })
}

/// A wrapper around an underlying raw stream which implements the WebSocket
/// protocol.
///
/// A `WebSocketStream<S>` represents a handshake that has been completed
/// successfully and both the server and the client are ready for receiving
/// and sending data. Message from a `WebSocketStream<S>` are accessible
/// through the respective `Stream` and `Sink`. Check more information about
/// them in `futures-rs` crate documentation or have a look on the examples
/// and unit tests for this crate.
#[derive(Debug)]
pub struct WebSocketStream<S> {
    inner: WebSocket<AllowStd<S>>,
    closing: bool,
    ended: bool,
    /// Tungstenite is probably ready to receive more data.
    ///
    /// `false` once start_send hits `WouldBlock` errors.
    /// `true` initially and after `flush`ing.
    ready: bool,
}

impl<S> WebSocketStream<S> {
    /// Convert a raw socket into a WebSocketStream without performing a
    /// handshake.
    pub async fn from_raw_socket(stream: S, role: Role, config: Option<WebSocketConfig>) -> Self
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        handshake::without_handshake(stream, move |allow_std| {
            WebSocket::from_raw_socket(allow_std, role, config)
        })
        .await
    }

    /// Convert a raw socket into a WebSocketStream without performing a
    /// handshake.
    pub async fn from_partially_read(
        stream: S,
        part: Vec<u8>,
        role: Role,
        config: Option<WebSocketConfig>,
    ) -> Self
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        handshake::without_handshake(stream, move |allow_std| {
            WebSocket::from_partially_read(allow_std, part, role, config)
        })
        .await
    }

    pub(crate) fn new(ws: WebSocket<AllowStd<S>>) -> Self {
        Self { inner: ws, closing: false, ended: false, ready: true }
    }

    fn with_context<F, R>(&mut self, ctx: Option<(ContextWaker, &mut Context<'_>)>, f: F) -> R
    where
        S: Unpin,
        F: FnOnce(&mut WebSocket<AllowStd<S>>) -> R,
        AllowStd<S>: Read + Write,
    {
        trace!("{}:{} WebSocketStream.with_context", file!(), line!());
        if let Some((kind, ctx)) = ctx {
            self.inner.get_mut().set_waker(kind, ctx.waker());
        }
        f(&mut self.inner)
    }

    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &S
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        self.inner.get_ref().get_ref()
    }

    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut S
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        self.inner.get_mut().get_mut()
    }

    /// Returns a reference to the configuration of the tungstenite stream.
    pub fn get_config(&self) -> &WebSocketConfig {
        self.inner.get_config()
    }

    /// Close the underlying web socket
    pub async fn close(&mut self, msg: Option<CloseFrame<'_>>) -> Result<(), WsError>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let msg = msg.map(|msg| msg.into_owned());
        self.send(Message::Close(msg)).await
    }
}

impl<T> Stream for WebSocketStream<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    type Item = Result<Message, WsError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        trace!("{}:{} Stream.poll_next", file!(), line!());

        // The connection has been closed or a critical error has occurred.
        // We have already returned the error to the user, the `Stream` is unusable,
        // so we assume that the stream has been "fused".
        if self.ended {
            return Poll::Ready(None);
        }

        match futures_util::ready!(self.with_context(Some((ContextWaker::Read, cx)), |s| {
            trace!("{}:{} Stream.with_context poll_next -> read()", file!(), line!());
            cvt(s.read())
        })) {
            Ok(v) => Poll::Ready(Some(Ok(v))),
            Err(e) => {
                self.ended = true;
                if matches!(e, WsError::AlreadyClosed | WsError::ConnectionClosed) {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Err(e)))
                }
            }
        }
    }
}

impl<T> FusedStream for WebSocketStream<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    fn is_terminated(&self) -> bool {
        self.ended
    }
}

impl<T> Sink<Message> for WebSocketStream<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    type Error = WsError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.ready {
            Poll::Ready(Ok(()))
        } else {
            // Currently blocked so try to flush the blockage away
            (*self).with_context(Some((ContextWaker::Write, cx)), |s| cvt(s.flush())).map(|r| {
                self.ready = true;
                r
            })
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        match (*self).with_context(None, |s| s.write(item)) {
            Ok(()) => {
                self.ready = true;
                Ok(())
            }
            Err(WsError::Io(err)) if err.kind() == std::io::ErrorKind::WouldBlock => {
                // the message was accepted and queued so not an error
                // but `poll_ready` will now start trying to flush the block
                self.ready = false;
                Ok(())
            }
            Err(e) => {
                self.ready = true;
                debug!("websocket start_send error: {}", e);
                Err(e)
            }
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        (*self).with_context(Some((ContextWaker::Write, cx)), |s| cvt(s.flush())).map(|r| {
            self.ready = true;
            match r {
                // WebSocket connection has just been closed. Flushing completed, not an error.
                Err(WsError::ConnectionClosed) => Ok(()),
                other => other,
            }
        })
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.ready = true;
        let res = if self.closing {
            // After queueing it, we call `flush` to drive the close handshake to completion.
            (*self).with_context(Some((ContextWaker::Write, cx)), |s| s.flush())
        } else {
            (*self).with_context(Some((ContextWaker::Write, cx)), |s| s.close(None))
        };

        match res {
            Ok(()) => Poll::Ready(Ok(())),
            Err(WsError::ConnectionClosed) => Poll::Ready(Ok(())),
            Err(WsError::Io(err)) if err.kind() == std::io::ErrorKind::WouldBlock => {
                trace!("WouldBlock");
                self.closing = true;
                Poll::Pending
            }
            Err(err) => {
                debug!("websocket close error: {}", err);
                Poll::Ready(Err(err))
            }
        }
    }
}

/// Get a domain from an URL.
#[cfg(any(feature = "connect", feature = "native-tls", feature = "__rustls-tls"))]
#[inline]
fn domain(request: &tungstenite::handshake::client::Request) -> Result<String, WsError> {
    match request.uri().host() {
        // rustls expects IPv6 addresses without the surrounding [] brackets
        #[cfg(feature = "__rustls-tls")]
        Some(d) if d.starts_with('[') && d.ends_with(']') => Ok(d[1..d.len() - 1].to_string()),
        Some(d) => Ok(d.to_string()),
        None => Err(WsError::Url(tungstenite::error::UrlError::NoHostName)),
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "connect")]
    use crate::stream::MaybeTlsStream;
    use crate::{compat::AllowStd, WebSocketStream};
    use std::io::{Read, Write};
    #[cfg(feature = "connect")]
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    fn is_read<T: Read>() {}
    fn is_write<T: Write>() {}
    #[cfg(feature = "connect")]
    fn is_async_read<T: AsyncReadExt>() {}
    #[cfg(feature = "connect")]
    fn is_async_write<T: AsyncWriteExt>() {}
    fn is_unpin<T: Unpin>() {}

    #[test]
    fn web_socket_stream_has_traits() {
        is_read::<AllowStd<tokio::net::TcpStream>>();
        is_write::<AllowStd<tokio::net::TcpStream>>();

        #[cfg(feature = "connect")]
        is_async_read::<MaybeTlsStream<tokio::net::TcpStream>>();
        #[cfg(feature = "connect")]
        is_async_write::<MaybeTlsStream<tokio::net::TcpStream>>();

        is_unpin::<WebSocketStream<tokio::net::TcpStream>>();
        #[cfg(feature = "connect")]
        is_unpin::<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>();
    }
}
