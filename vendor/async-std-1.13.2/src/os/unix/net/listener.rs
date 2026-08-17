//! Unix-specific networking extensions.

use std::fmt;
use std::os::unix::net::UnixListener as StdUnixListener;
use std::os::unix::net::UnixStream as StdUnixStream;
use std::pin::Pin;

use async_io::Async;

use super::SocketAddr;
use super::UnixStream;
use crate::io;
use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::path::Path;
use crate::stream::Stream;
use crate::sync::Arc;
use crate::task::{ready, Context, Poll};

/// A Unix domain socket server, listening for connections.
///
/// After creating a `UnixListener` by [`bind`]ing it to a socket address, it listens for incoming
/// connections. These can be accepted by awaiting elements from the async stream of [`incoming`]
/// connections.
///
/// The socket will be closed when the value is dropped.
///
/// This type is an async version of [`std::os::unix::net::UnixListener`].
///
/// [`std::os::unix::net::UnixListener`]:
/// https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html
/// [`bind`]: #method.bind
/// [`incoming`]: #method.incoming
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::os::unix::net::UnixListener;
/// use async_std::prelude::*;
///
/// let listener = UnixListener::bind("/tmp/socket").await?;
/// let mut incoming = listener.incoming();
///
/// while let Some(stream) = incoming.next().await {
///     let mut stream = stream?;
///     stream.write_all(b"hello world").await?;
/// }
/// #
/// # Ok(()) }) }
/// ```
pub struct UnixListener {
    watcher: Async<StdUnixListener>,
}

impl UnixListener {
    /// Creates a Unix datagram listener bound to the given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixListener;
    ///
    /// let listener = UnixListener::bind("/tmp/socket").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn bind<P: AsRef<Path>>(path: P) -> io::Result<UnixListener> {
        let path = path.as_ref().to_owned();
        let listener = Async::<StdUnixListener>::bind(path)?;

        Ok(UnixListener { watcher: listener })
    }

    /// Accepts a new incoming connection to this listener.
    ///
    /// When a connection is established, the corresponding stream and address will be returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixListener;
    ///
    /// let listener = UnixListener::bind("/tmp/socket").await?;
    /// let (socket, addr) = listener.accept().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        let (stream, addr) = self.watcher.accept().await?;

        Ok((
            UnixStream {
                watcher: Arc::new(stream),
            },
            addr,
        ))
    }

    /// Returns a stream of incoming connections.
    ///
    /// Iterating over this stream is equivalent to calling [`accept`] in a loop. The stream of
    /// connections is infinite, i.e awaiting the next connection will never result in [`None`].
    ///
    /// [`accept`]: #method.accept
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixListener;
    /// use async_std::prelude::*;
    ///
    /// let listener = UnixListener::bind("/tmp/socket").await?;
    /// let mut incoming = listener.incoming();
    ///
    /// while let Some(stream) = incoming.next().await {
    ///     let mut stream = stream?;
    ///     stream.write_all(b"hello world").await?;
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn incoming(&self) -> Incoming<'_> {
        Incoming {
            incoming: Box::pin(self.watcher.incoming()),
        }
    }

    /// Returns the local socket address of this listener.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixListener;
    ///
    /// let listener = UnixListener::bind("/tmp/socket").await?;
    /// let addr = listener.local_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().local_addr()
    }
}

impl fmt::Debug for UnixListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("UnixListener");
        builder.field("fd", &self.as_raw_fd());

        if let Ok(addr) = self.local_addr() {
            builder.field("local", &addr);
        }

        builder.finish()
    }
}

/// A stream of incoming Unix domain socket connections.
///
/// This stream is infinite, i.e awaiting the next connection will never result in [`None`]. It is
/// created by the [`incoming`] method on [`UnixListener`].
///
/// This type is an async version of [`std::os::unix::net::Incoming`].
///
/// [`std::os::unix::net::Incoming`]: https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
/// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
/// [`incoming`]: struct.UnixListener.html#method.incoming
/// [`UnixListener`]: struct.UnixListener.html
pub struct Incoming<'a> {
    incoming: Pin<Box<dyn Stream<Item = io::Result<Async<StdUnixStream>>> + Send + Sync + 'a>>,
}

impl Stream for Incoming<'_> {
    type Item = io::Result<UnixStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let res = ready!(Pin::new(&mut self.incoming).poll_next(cx));
        Poll::Ready(res.map(|res| res.map(|stream| UnixStream { watcher: Arc::new(stream) })))
    }
}

impl fmt::Debug for Incoming<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Incoming")
            .finish()
    }
}

impl From<StdUnixListener> for UnixListener {
    /// Converts a `std::os::unix::net::UnixListener` into its asynchronous equivalent.
    fn from(listener: StdUnixListener) -> UnixListener {
        UnixListener {
            watcher: Async::new(listener).expect("UnixListener is known to be good"),
        }
    }
}

impl std::convert::TryFrom<UnixListener> for StdUnixListener {
    type Error = io::Error;
    /// Converts a `UnixListener` into its synchronous equivalent.
    fn try_from(listener: UnixListener) -> io::Result<StdUnixListener> {
        let inner = listener.watcher.into_inner()?;
        inner.set_nonblocking(false)?;
        Ok(inner)
    }
}

impl AsRawFd for UnixListener {
    fn as_raw_fd(&self) -> RawFd {
        self.watcher.as_raw_fd()
    }
}

impl FromRawFd for UnixListener {
    unsafe fn from_raw_fd(fd: RawFd) -> UnixListener {
        let listener = std::os::unix::net::UnixListener::from_raw_fd(fd);
        listener.into()
    }
}

impl IntoRawFd for UnixListener {
    fn into_raw_fd(self) -> RawFd {
        self.watcher.into_inner().unwrap().into_raw_fd()
    }
}

cfg_io_safety! {
    use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

    impl AsFd for UnixListener {
        fn as_fd(&self) -> BorrowedFd<'_> {
            self.watcher.get_ref().as_fd()
        }
    }

    impl From<OwnedFd> for UnixListener {
        fn from(fd: OwnedFd) -> UnixListener {
            std::os::unix::net::UnixListener::from(fd).into()
        }
    }

    impl From<UnixListener> for OwnedFd {
        fn from(stream: UnixListener) -> OwnedFd {
            stream.watcher.into_inner().unwrap().into()
        }
    }
}
