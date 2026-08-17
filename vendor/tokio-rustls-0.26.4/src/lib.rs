//! Asynchronous TLS/SSL streams for Tokio using [Rustls](https://github.com/rustls/rustls).
//!
//! # Why do I need to call `poll_flush`?
//!
//! Most TLS implementations will have an internal buffer to improve throughput,
//! and rustls is no exception.
//!
//! When we write data to `TlsStream`, we always write rustls buffer first,
//! then take out rustls encrypted data packet, and write it to data channel (like TcpStream).
//! When data channel is pending, some data may remain in rustls buffer.
//!
//! `tokio-rustls` To keep it simple and correct, [TlsStream] will behave like `BufWriter`.
//! For `TlsStream<TcpStream>`, this means that data written by `poll_write` is not guaranteed to be written to `TcpStream`.
//! You must call `poll_flush` to ensure that it is written to `TcpStream`.
//!
//! You should call `poll_flush` at the appropriate time,
//! such as when a period of `poll_write` write is complete and there is no more data to write.
//!
//! ## Why don't we write during `poll_read`?
//!
//! We did this in the early days of `tokio-rustls`, but it caused some bugs.
//! We can solve these bugs through some solutions, but this will cause performance degradation (reverse false wakeup).
//!
//! And reverse write will also prevent us implement full duplex in the future.
//!
//! see <https://github.com/tokio-rs/tls/issues/40>
//!
//! ## Why can't we handle it like `native-tls`?
//!
//! When data channel returns to pending, `native-tls` will falsely report the number of bytes it consumes.
//! This means that if data written by `poll_write` is not actually written to data channel, it will not return `Ready`.
//! Thus avoiding the call of `poll_flush`.
//!
//! but which does not conform to convention of `AsyncWrite` trait.
//! This means that if you give inconsistent data in two `poll_write`, it may cause unexpected behavior.
//!
//! see <https://github.com/tokio-rs/tls/issues/41>

#![warn(unreachable_pub, clippy::use_self)]

use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};
use std::pin::Pin;
use std::task::{Context, Poll};

pub use rustls;

use rustls::CommonState;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite, ReadBuf};

macro_rules! ready {
    ( $e:expr ) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

pub mod client;
pub use client::{Connect, FallibleConnect, TlsConnector, TlsConnectorWithAlpn};
mod common;
pub mod server;
pub use server::{Accept, FallibleAccept, LazyConfigAcceptor, StartHandshake, TlsAcceptor};

/// Unified TLS stream type
///
/// This abstracts over the inner `client::TlsStream` and `server::TlsStream`, so you can use
/// a single type to keep both client- and server-initiated TLS-encrypted connections.
#[allow(clippy::large_enum_variant)] // https://github.com/rust-lang/rust-clippy/issues/9798
#[derive(Debug)]
pub enum TlsStream<T> {
    Client(client::TlsStream<T>),
    Server(server::TlsStream<T>),
}

impl<T> TlsStream<T> {
    pub fn get_ref(&self) -> (&T, &CommonState) {
        use TlsStream::*;
        match self {
            Client(io) => {
                let (io, session) = io.get_ref();
                (io, session)
            }
            Server(io) => {
                let (io, session) = io.get_ref();
                (io, session)
            }
        }
    }

    pub fn get_mut(&mut self) -> (&mut T, &mut CommonState) {
        use TlsStream::*;
        match self {
            Client(io) => {
                let (io, session) = io.get_mut();
                (io, &mut *session)
            }
            Server(io) => {
                let (io, session) = io.get_mut();
                (io, &mut *session)
            }
        }
    }
}

impl<T> From<client::TlsStream<T>> for TlsStream<T> {
    fn from(s: client::TlsStream<T>) -> Self {
        Self::Client(s)
    }
}

impl<T> From<server::TlsStream<T>> for TlsStream<T> {
    fn from(s: server::TlsStream<T>) -> Self {
        Self::Server(s)
    }
}

#[cfg(unix)]
impl<S> AsRawFd for TlsStream<S>
where
    S: AsRawFd,
{
    fn as_raw_fd(&self) -> RawFd {
        self.get_ref().0.as_raw_fd()
    }
}

#[cfg(windows)]
impl<S> AsRawSocket for TlsStream<S>
where
    S: AsRawSocket,
{
    fn as_raw_socket(&self) -> RawSocket {
        self.get_ref().0.as_raw_socket()
    }
}

impl<T> AsyncRead for TlsStream<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).poll_read(cx, buf),
            Self::Server(x) => Pin::new(x).poll_read(cx, buf),
        }
    }
}

impl<T> AsyncBufRead for TlsStream<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    #[inline]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).poll_fill_buf(cx),
            Self::Server(x) => Pin::new(x).poll_fill_buf(cx),
        }
    }

    #[inline]
    fn consume(self: Pin<&mut Self>, amt: usize) {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).consume(amt),
            Self::Server(x) => Pin::new(x).consume(amt),
        }
    }
}

impl<T> AsyncWrite for TlsStream<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).poll_write(cx, buf),
            Self::Server(x) => Pin::new(x).poll_write(cx, buf),
        }
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).poll_write_vectored(cx, bufs),
            Self::Server(x) => Pin::new(x).poll_write_vectored(cx, bufs),
        }
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        match self {
            Self::Client(x) => x.is_write_vectored(),
            Self::Server(x) => x.is_write_vectored(),
        }
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).poll_flush(cx),
            Self::Server(x) => Pin::new(x).poll_flush(cx),
        }
    }

    #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Self::Client(x) => Pin::new(x).poll_shutdown(cx),
            Self::Server(x) => Pin::new(x).poll_shutdown(cx),
        }
    }
}
