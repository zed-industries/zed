use core::task::{Context, Poll};
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;

use futures_util::ready;
use hyper::server::{
    accept::Accept,
    conn::{AddrIncoming, AddrStream},
};
use rustls::{ServerConfig, ServerConnection};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

mod builder;
pub use builder::AcceptorBuilder;
use builder::WantsTlsConfig;

/// A TLS acceptor that can be used with hyper servers.
pub struct TlsAcceptor<A = AddrIncoming> {
    config: Arc<ServerConfig>,
    acceptor: A,
}

/// An Acceptor for the `https` scheme.
impl TlsAcceptor {
    /// Provides a builder for a `TlsAcceptor`.
    pub fn builder() -> AcceptorBuilder<WantsTlsConfig> {
        AcceptorBuilder::new()
    }

    /// Creates a new `TlsAcceptor` from a `ServerConfig` and an `AddrIncoming`.
    pub fn new(config: Arc<ServerConfig>, incoming: AddrIncoming) -> Self {
        Self {
            config,
            acceptor: incoming,
        }
    }
}

impl<A> Accept for TlsAcceptor<A>
where
    A: Accept<Error = io::Error> + Unpin,
    A::Conn: AsyncRead + AsyncWrite + Unpin,
{
    type Conn = TlsStream<A::Conn>;
    type Error = io::Error;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let pin = self.get_mut();
        Poll::Ready(match ready!(Pin::new(&mut pin.acceptor).poll_accept(cx)) {
            Some(Ok(sock)) => Some(Ok(TlsStream::new(sock, pin.config.clone()))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        })
    }
}

impl<C, I> From<(C, I)> for TlsAcceptor
where
    C: Into<Arc<ServerConfig>>,
    I: Into<AddrIncoming>,
{
    fn from((config, incoming): (C, I)) -> Self {
        Self::new(config.into(), incoming.into())
    }
}

/// A TLS stream constructed by a [`TlsAcceptor`].
// tokio_rustls::server::TlsStream doesn't expose constructor methods,
// so we have to TlsAcceptor::accept and handshake to have access to it
// TlsStream implements AsyncRead/AsyncWrite by handshaking with tokio_rustls::Accept first
pub struct TlsStream<C = AddrStream> {
    state: State<C>,
}

impl<C: AsyncRead + AsyncWrite + Unpin> TlsStream<C> {
    fn new(stream: C, config: Arc<ServerConfig>) -> Self {
        let accept = tokio_rustls::TlsAcceptor::from(config).accept(stream);
        Self {
            state: State::Handshaking(accept),
        }
    }
    /// Returns a reference to the underlying IO stream.
    ///
    /// This should always return `Some`, except if an error has already been yielded.
    pub fn io(&self) -> Option<&C> {
        match &self.state {
            State::Handshaking(accept) => accept.get_ref(),
            State::Streaming(stream) => Some(stream.get_ref().0),
        }
    }

    /// Returns a reference to the underlying [`rustls::ServerConnection'].
    ///
    /// This will start yielding `Some` only after the handshake has completed.
    pub fn connection(&self) -> Option<&ServerConnection> {
        match &self.state {
            State::Handshaking(_) => None,
            State::Streaming(stream) => Some(stream.get_ref().1),
        }
    }
}

impl<C: AsyncRead + AsyncWrite + Unpin> AsyncRead for TlsStream<C> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<io::Result<()>> {
        let pin = self.get_mut();
        let accept = match &mut pin.state {
            State::Handshaking(accept) => accept,
            State::Streaming(stream) => return Pin::new(stream).poll_read(cx, buf),
        };

        let mut stream = match ready!(Pin::new(accept).poll(cx)) {
            Ok(stream) => stream,
            Err(err) => return Poll::Ready(Err(err)),
        };

        let result = Pin::new(&mut stream).poll_read(cx, buf);
        pin.state = State::Streaming(stream);
        result
    }
}

impl<C: AsyncRead + AsyncWrite + Unpin> AsyncWrite for TlsStream<C> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let pin = self.get_mut();
        let accept = match &mut pin.state {
            State::Handshaking(accept) => accept,
            State::Streaming(stream) => return Pin::new(stream).poll_write(cx, buf),
        };

        let mut stream = match ready!(Pin::new(accept).poll(cx)) {
            Ok(stream) => stream,
            Err(err) => return Poll::Ready(Err(err)),
        };

        let result = Pin::new(&mut stream).poll_write(cx, buf);
        pin.state = State::Streaming(stream);
        result
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

enum State<C> {
    Handshaking(tokio_rustls::Accept<C>),
    Streaming(tokio_rustls::server::TlsStream<C>),
}
