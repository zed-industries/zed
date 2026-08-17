use std::io::{IoSlice, IoSliceMut};
use std::net::SocketAddr;
use std::pin::Pin;

use async_io::Async;

use crate::io::{self, Read, Write};
use crate::net::ToSocketAddrs;
use crate::sync::Arc;
use crate::task::{Context, Poll};

/// A TCP stream between a local and a remote socket.
///
/// A `TcpStream` can either be created by connecting to an endpoint, via the [`connect`] method,
/// or by [accepting] a connection from a [listener].  It can be read or written to using the
/// [`AsyncRead`], [`AsyncWrite`], and related extension traits in [`futures::io`].
///
/// The connection will be closed when the value is dropped. The reading and writing portions of
/// the connection can also be shut down individually with the [`shutdown`] method.
///
/// This type is an async version of [`std::net::TcpStream`].
///
/// [`connect`]: struct.TcpStream.html#method.connect
/// [accepting]: struct.TcpListener.html#method.accept
/// [listener]: struct.TcpListener.html
/// [`AsyncRead`]: https://docs.rs/futures/0.3/futures/io/trait.AsyncRead.html
/// [`AsyncWrite`]: https://docs.rs/futures/0.3/futures/io/trait.AsyncWrite.html
/// [`futures::io`]: https://docs.rs/futures/0.3/futures/io/index.html
/// [`shutdown`]: struct.TcpStream.html#method.shutdown
/// [`std::net::TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
///
/// ## Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::net::TcpStream;
/// use async_std::prelude::*;
///
/// let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
/// stream.write_all(b"hello world").await?;
///
/// let mut buf = vec![0u8; 1024];
/// let n = stream.read(&mut buf).await?;
/// #
/// # Ok(()) }) }
/// ```
#[derive(Debug, Clone)]
pub struct TcpStream {
    pub(super) watcher: Arc<Async<std::net::TcpStream>>,
}

impl TcpStream {
    /// Creates a new TCP stream connected to the specified address.
    ///
    /// This method will create a new TCP socket and attempt to connect it to the `addr`
    /// provided. The [returned future] will be resolved once the stream has successfully
    /// connected, or it will return an error if one occurs.
    ///
    /// [returned future]: struct.Connect.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:0").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn connect<A: ToSocketAddrs>(addrs: A) -> io::Result<TcpStream> {
        let mut last_err = None;
        let addrs = addrs.to_socket_addrs().await?;

        for addr in addrs {
            match Async::<std::net::TcpStream>::connect(addr).await {
                Ok(stream) => {
                    return Ok(TcpStream {
                        watcher: Arc::new(stream),
                    });
                }
                Err(e) => {
                    last_err = Some(e);
                    continue;
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve to any addresses",
            )
        }))
    }

    /// Returns the local address that this stream is connected to.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// let addr = stream.local_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().local_addr()
    }

    /// Returns the remote address that this stream is connected to.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// let peer = stream.peer_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().peer_addr()
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_ttl`].
    ///
    /// [`set_ttl`]: #method.set_ttl
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_ttl(100)?;
    /// assert_eq!(stream.ttl()?, 100);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn ttl(&self) -> io::Result<u32> {
        self.watcher.get_ref().ttl()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_ttl(100)?;
    /// assert_eq!(stream.ttl()?, 100);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.watcher.get_ref().set_ttl(ttl)
    }

    /// Receives data on the socket from the remote address to which it is connected, without
    /// removing that data from the queue.
    ///
    /// On success, returns the number of bytes peeked.
    ///
    /// Successive calls return the same data. This is accomplished by passing `MSG_PEEK` as a flag
    /// to the underlying `recv` system call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8000").await?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let n = stream.peek(&mut buf).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.watcher.peek(buf).await
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// For more information about this option, see [`set_nodelay`].
    ///
    /// [`set_nodelay`]: #method.set_nodelay
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_nodelay(true)?;
    /// assert_eq!(stream.nodelay()?, true);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn nodelay(&self) -> io::Result<bool> {
        self.watcher.get_ref().nodelay()
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// If set, this option disables the Nagle algorithm. This means that
    /// segments are always sent as soon as possible, even if there is only a
    /// small amount of data. When not set, data is buffered until there is a
    /// sufficient amount to send out, thereby avoiding the frequent sending of
    /// small packets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_nodelay(true)?;
    /// assert_eq!(stream.nodelay()?, true);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.watcher.get_ref().set_nodelay(nodelay)
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This method will cause all pending and future I/O on the specified portions to return
    /// immediately with an appropriate value (see the documentation of [`Shutdown`]).
    ///
    /// [`Shutdown`]: https://doc.rust-lang.org/std/net/enum.Shutdown.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use std::net::Shutdown;
    ///
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// stream.shutdown(Shutdown::Both)?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.watcher.get_ref().shutdown(how)
    }
}

impl Read for TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }

    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read_vectored(cx, bufs)
    }
}

impl Read for &TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.watcher).poll_read(cx, buf)
    }

    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.watcher).poll_read_vectored(cx, bufs)
    }
}

impl Write for TcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write_vectored(cx, bufs)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}

impl Write for &TcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.watcher).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.watcher).poll_write_vectored(cx, bufs)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.watcher).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.watcher).poll_close(cx)
    }
}

impl From<std::net::TcpStream> for TcpStream {
    /// Converts a `std::net::TcpStream` into its asynchronous equivalent.
    fn from(stream: std::net::TcpStream) -> TcpStream {
        TcpStream {
            watcher: Arc::new(Async::new(stream).expect("TcpStream is known to be good")),
        }
    }
}

impl std::convert::TryFrom<TcpStream> for std::net::TcpStream {
    type Error = io::Error;
    /// Converts a `TcpStream` into its synchronous equivalent.
    fn try_from(stream: TcpStream) -> io::Result<std::net::TcpStream> {
        let inner = Arc::try_unwrap(stream.watcher)
            .map_err(|_| io::Error::new(
                io::ErrorKind::Other,
                "Cannot convert TcpStream to synchronous: multiple references",
            ))?
            .into_inner()?;
        inner.set_nonblocking(false)?;
        Ok(inner)
    }
}

cfg_unix! {
    use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

    impl AsRawFd for TcpStream {
        fn as_raw_fd(&self) -> RawFd {
            self.watcher.get_ref().as_raw_fd()
        }
    }

    impl FromRawFd for TcpStream {
        unsafe fn from_raw_fd(fd: RawFd) -> TcpStream {
            std::net::TcpStream::from_raw_fd(fd).into()
        }
    }

    impl IntoRawFd for TcpStream {
        fn into_raw_fd(self) -> RawFd {
            // TODO(stjepang): This does not mean `RawFd` is now the sole owner of the file
            // descriptor because it's possible that there are other clones of this `TcpStream`
            // using it at the same time. We should probably document that behavior.
            self.as_raw_fd()
        }
    }

    cfg_io_safety! {
        use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

        impl AsFd for TcpStream {
            fn as_fd(&self) -> BorrowedFd<'_> {
                self.watcher.get_ref().as_fd()
            }
        }

        impl From<OwnedFd> for TcpStream {
            fn from(fd: OwnedFd) -> TcpStream {
                std::net::TcpStream::from(fd).into()
            }
        }

        impl From<TcpStream> for OwnedFd {
            fn from(stream: TcpStream) -> OwnedFd {
                stream.watcher.get_ref().try_clone().unwrap().into()
            }
        }
    }
}

cfg_windows! {
    use crate::os::windows::io::{
        RawSocket, AsRawSocket, FromRawSocket, IntoRawSocket
    };

    impl AsRawSocket for TcpStream {
        fn as_raw_socket(&self) -> RawSocket {
            self.watcher.get_ref().as_raw_socket()
        }
    }

    impl FromRawSocket for TcpStream {
        unsafe fn from_raw_socket(handle: RawSocket) -> TcpStream {
            std::net::TcpStream::from_raw_socket(handle).into()
        }
    }

    impl IntoRawSocket for TcpStream {
        fn into_raw_socket(self) -> RawSocket {
            // TODO(stjepang): This does not mean `RawFd` is now the sole owner of the file
            // descriptor because it's possible that there are other clones of this `TcpStream`
            // using it at the same time. We should probably document that behavior.
            self.as_raw_socket()
        }
    }

    cfg_io_safety! {
        use crate::os::windows::io::{AsSocket, BorrowedSocket, OwnedSocket};

        impl AsSocket for TcpStream {
            fn as_socket(&self) -> BorrowedSocket<'_> {
                self.watcher.get_ref().as_socket()
            }
        }

        impl From<OwnedSocket> for TcpStream {
            fn from(fd: OwnedSocket) -> TcpStream {
                std::net::TcpStream::from(fd).into()
            }
        }

        impl From<TcpStream> for OwnedSocket {
            fn from(stream: TcpStream) -> OwnedSocket {
                stream.watcher.get_ref().try_clone().unwrap().into()
            }
        }
    }
}
