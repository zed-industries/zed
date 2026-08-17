//! `AsHandleOrSocket` and related `From` impls.

use super::types::{BorrowedHandleOrSocket, OwnedHandleOrSocket};
use super::AsRawHandleOrSocket;
use io_lifetimes::{AsHandle, AsSocket};
use std::fs::File;
use std::io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::process::{ChildStderr, ChildStdin, ChildStdout};

/// Like [`AsHandle`] and [`AsSocket`], but implementable by types which
/// can implement either one.
pub trait AsHandleOrSocket {
    /// Like [`AsHandle::as_handle`] and [`AsSocket::as_socket`]
    /// but can return either type.
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_>;
}

impl AsHandleOrSocket for OwnedHandleOrSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        unsafe { BorrowedHandleOrSocket::borrow_raw(self.as_raw_handle_or_socket()) }
    }
}

impl AsHandleOrSocket for BorrowedHandleOrSocket<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        *self
    }
}

impl AsHandleOrSocket for Stdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for StdinLock<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for Stdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for StdoutLock<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for Stderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for StderrLock<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for ChildStdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for ChildStdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for ChildStderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl AsHandleOrSocket for TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl AsHandleOrSocket for UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::io::Stdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::io::Stdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::io::Stderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::fs::File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

// async_std's `ChildStdin`, `ChildStdout`, and `ChildStderr` don't implement
// `AsFd` or `AsHandle`.

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::io::Stdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::io::Stdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::io::Stderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::fs::File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::process::ChildStdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::process::ChildStdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::process::ChildStderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsHandleOrSocket for os_pipe::PipeReader {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsHandleOrSocket for os_pipe::PipeWriter {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "socket2")]
impl AsHandleOrSocket for socket2::Socket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable when we have impls for mio
impl AsHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable when we have impls for mio
impl AsHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable when we have impls for mio
impl AsHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl From<File> for OwnedHandleOrSocket {
    #[inline]
    fn from(file: File) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(file.into())
    }
}

impl From<ChildStdin> for OwnedHandleOrSocket {
    #[inline]
    fn from(stdin: ChildStdin) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(stdin.into())
    }
}

impl From<ChildStdout> for OwnedHandleOrSocket {
    #[inline]
    fn from(stdout: ChildStdout) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(stdout.into())
    }
}

impl From<ChildStderr> for OwnedHandleOrSocket {
    #[inline]
    fn from(stderr: ChildStderr) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(stderr.into())
    }
}

impl From<TcpStream> for OwnedHandleOrSocket {
    #[inline]
    fn from(stream: TcpStream) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(stream.into())
    }
}

impl From<TcpListener> for OwnedHandleOrSocket {
    #[inline]
    fn from(listener: TcpListener) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(listener.into())
    }
}

impl From<UdpSocket> for OwnedHandleOrSocket {
    #[inline]
    fn from(socket: UdpSocket) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(socket.into())
    }
}

#[cfg(feature = "os_pipe")]
impl From<os_pipe::PipeReader> for OwnedHandleOrSocket {
    #[inline]
    fn from(reader: os_pipe::PipeReader) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(reader.into())
    }
}

#[cfg(feature = "os_pipe")]
impl From<os_pipe::PipeWriter> for OwnedHandleOrSocket {
    #[inline]
    fn from(writer: os_pipe::PipeWriter) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(writer.into())
    }
}

#[cfg(feature = "socket2")]
impl From<socket2::Socket> for OwnedHandleOrSocket {
    #[inline]
    fn from(socket: socket2::Socket) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(socket.into())
    }
}

#[cfg(feature = "use_mio_net")]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable when we have impls for mio
impl From<mio::net::TcpStream> for OwnedHandleOrSocket {
    #[inline]
    fn from(stream: mio::net::TcpStream) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(stream.into())
    }
}

#[cfg(feature = "use_mio_net")]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable when we have impls for mio
impl From<mio::net::TcpListener> for OwnedHandleOrSocket {
    #[inline]
    fn from(listener: mio::net::TcpListener) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(listener.into())
    }
}

#[cfg(feature = "use_mio_net")]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable when we have impls for mio
impl From<mio::net::UdpSocket> for OwnedHandleOrSocket {
    #[inline]
    fn from(socket: mio::net::UdpSocket) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(socket.into())
    }
}

#[cfg(feature = "async-std")]
impl From<async_std::fs::File> for OwnedHandleOrSocket {
    #[inline]
    fn from(file: async_std::fs::File) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(file.into())
    }
}

#[cfg(feature = "async-std")]
impl From<async_std::net::TcpStream> for OwnedHandleOrSocket {
    #[inline]
    fn from(stream: async_std::net::TcpStream) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(stream.into())
    }
}

#[cfg(feature = "async-std")]
impl From<async_std::net::TcpListener> for OwnedHandleOrSocket {
    #[inline]
    fn from(listener: async_std::net::TcpListener) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(listener.into())
    }
}

#[cfg(feature = "async-std")]
impl From<async_std::net::UdpSocket> for OwnedHandleOrSocket {
    #[inline]
    fn from(socket: async_std::net::UdpSocket) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(socket.into())
    }
}
