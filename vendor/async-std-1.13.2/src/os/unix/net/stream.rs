//! Unix-specific networking extensions.

use std::fmt;
use std::net::Shutdown;
use std::os::unix::net::UnixStream as StdUnixStream;
use std::pin::Pin;

use async_io::Async;

use super::SocketAddr;
use crate::io::{self, Read, Write};
use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::path::Path;
use crate::sync::Arc;
use crate::task::{Context, Poll};

/// A Unix stream socket.
///
/// This type is an async version of [`std::os::unix::net::UnixStream`].
///
/// [`std::os::unix::net::UnixStream`]:
/// https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::os::unix::net::UnixStream;
/// use async_std::prelude::*;
///
/// let mut stream = UnixStream::connect("/tmp/socket").await?;
/// stream.write_all(b"hello world").await?;
///
/// let mut response = Vec::new();
/// stream.read_to_end(&mut response).await?;
/// #
/// # Ok(()) }) }
/// ```
#[derive(Clone)]
pub struct UnixStream {
    pub(super) watcher: Arc<Async<StdUnixStream>>,
}

impl UnixStream {
    /// Connects to the socket to the specified address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixStream;
    ///
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<UnixStream> {
        let path = path.as_ref().to_owned();
        let stream = Arc::new(Async::<StdUnixStream>::connect(path).await?);

        Ok(UnixStream { watcher: stream })
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// Returns two streams which are connected to each other.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixStream;
    ///
    /// let stream = UnixStream::pair()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn pair() -> io::Result<(UnixStream, UnixStream)> {
        let (a, b) = Async::<StdUnixStream>::pair()?;
        let a = UnixStream {
            watcher: Arc::new(a),
        };
        let b = UnixStream {
            watcher: Arc::new(b),
        };
        Ok((a, b))
    }

    /// Returns the socket address of the local half of this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixStream;
    ///
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// let addr = stream.local_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().local_addr()
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixStream;
    ///
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// let peer = stream.peer_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().peer_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O calls on the specified portions to
    /// immediately return with an appropriate value (see the documentation of [`Shutdown`]).
    ///
    /// [`Shutdown`]: https://doc.rust-lang.org/std/net/enum.Shutdown.html
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixStream;
    /// use std::net::Shutdown;
    ///
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// stream.shutdown(Shutdown::Both)?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.watcher.get_ref().shutdown(how)
    }
}

impl Read for UnixStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }
}

impl Read for &UnixStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.watcher).poll_read(cx, buf)
    }
}

impl Write for UnixStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}

impl Write for &UnixStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.watcher).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.watcher).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.watcher).poll_close(cx)
    }
}

impl fmt::Debug for UnixStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("UnixStream");
        builder.field("fd", &self.as_raw_fd());

        if let Ok(addr) = self.local_addr() {
            builder.field("local", &addr);
        }

        if let Ok(addr) = self.peer_addr() {
            builder.field("peer", &addr);
        }

        builder.finish()
    }
}

impl From<StdUnixStream> for UnixStream {
    /// Converts a `std::os::unix::net::UnixStream` into its asynchronous equivalent.
    fn from(stream: StdUnixStream) -> UnixStream {
        let stream = Async::new(stream).expect("UnixStream is known to be good");
        UnixStream {
            watcher: Arc::new(stream),
        }
    }
}

impl std::convert::TryFrom<UnixStream> for StdUnixStream {
    type Error = io::Error;
    /// Converts a `UnixStream` into its synchronous equivalent.
    fn try_from(stream: UnixStream) -> io::Result<StdUnixStream> {
        let inner = Arc::try_unwrap(stream.watcher)
            .map_err(|_| io::Error::new(
                io::ErrorKind::Other,
                "Cannot convert UnixStream to synchronous: multiple references",
            ))?
            .into_inner()?;
        inner.set_nonblocking(false)?;
        Ok(inner)
    }
}

impl AsRawFd for UnixStream {
    fn as_raw_fd(&self) -> RawFd {
        self.watcher.as_raw_fd()
    }
}

impl FromRawFd for UnixStream {
    unsafe fn from_raw_fd(fd: RawFd) -> UnixStream {
        let stream = std::os::unix::net::UnixStream::from_raw_fd(fd);
        stream.into()
    }
}

impl IntoRawFd for UnixStream {
    fn into_raw_fd(self) -> RawFd {
        (*self.watcher).get_ref().try_clone().unwrap().into_raw_fd()
    }
}

cfg_io_safety! {
    use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

    impl AsFd for UnixStream {
        fn as_fd(&self) -> BorrowedFd<'_> {
            self.watcher.get_ref().as_fd()
        }
    }

    impl From<OwnedFd> for UnixStream {
        fn from(fd: OwnedFd) -> UnixStream {
            std::os::unix::net::UnixStream::from(fd).into()
        }
    }

    impl From<UnixStream> for OwnedFd {
        fn from(stream: UnixStream) -> OwnedFd {
            stream.watcher.get_ref().try_clone().unwrap().into()
        }
    }
}
