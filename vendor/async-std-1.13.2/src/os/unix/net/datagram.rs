//! Unix-specific networking extensions.

use std::fmt;
use std::net::Shutdown;
use std::os::unix::net::UnixDatagram as StdUnixDatagram;

use async_io::Async;

use super::SocketAddr;
use crate::io;
use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::path::Path;

/// A Unix datagram socket.
///
/// After creating a `UnixDatagram` by [`bind`]ing it to a path, data can be [sent to] and
/// [received from] any other socket address.
///
/// This type is an async version of [`std::os::unix::net::UnixDatagram`].
///
/// [`std::os::unix::net::UnixDatagram`]:
/// https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html
/// [`bind`]: #method.bind
/// [received from]: #method.recv_from
/// [sent to]: #method.send_to
///
/// ## Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::os::unix::net::UnixDatagram;
///
/// let socket = UnixDatagram::bind("/tmp/socket1").await?;
/// socket.send_to(b"hello world", "/tmp/socket2").await?;
///
/// let mut buf = vec![0u8; 1024];
/// let (n, peer) = socket.recv_from(&mut buf).await?;
/// #
/// # Ok(()) }) }
/// ```
pub struct UnixDatagram {
    watcher: Async<StdUnixDatagram>,
}

impl UnixDatagram {
    fn new(socket: StdUnixDatagram) -> UnixDatagram {
        UnixDatagram {
            watcher: Async::new(socket).expect("UnixDatagram is known to be good"),
        }
    }

    /// Creates a Unix datagram socket bound to the given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::bind("/tmp/socket").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn bind<P: AsRef<Path>>(path: P) -> io::Result<UnixDatagram> {
        let path = path.as_ref().to_owned();
        let socket = Async::<StdUnixDatagram>::bind(path)?;
        Ok(UnixDatagram { watcher: socket })
    }

    /// Creates a Unix datagram which is not bound to any address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn unbound() -> io::Result<UnixDatagram> {
        let socket = StdUnixDatagram::unbound()?;
        Ok(UnixDatagram::new(socket))
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// Returns two sockets which are connected to each other.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let (socket1, socket2) = UnixDatagram::pair()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn pair() -> io::Result<(UnixDatagram, UnixDatagram)> {
        let (a, b) = StdUnixDatagram::pair()?;
        let a = UnixDatagram::new(a);
        let b = UnixDatagram::new(b);
        Ok((a, b))
    }

    /// Connects the socket to the specified address.
    ///
    /// The [`send`] method may be used to send data to the specified address. [`recv`] and
    /// [`recv_from`] will only receive data from that address.
    ///
    /// [`send`]: #method.send
    /// [`recv`]: #method.recv
    /// [`recv_from`]: #method.recv_from
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn connect<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        // TODO(stjepang): Connect the socket on a blocking pool.
        let p = path.as_ref();
        self.watcher.get_ref().connect(p)
    }

    /// Returns the address of this socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::bind("/tmp/socket").await?;
    /// let addr = socket.local_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().local_addr()
    }

    /// Returns the address of this socket's peer.
    ///
    /// The [`connect`] method will connect the socket to a peer.
    ///
    /// [`connect`]: #method.connect
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket").await?;
    /// let peer = socket.peer_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.watcher.get_ref().peer_addr()
    }

    /// Receives data from the socket.
    ///
    /// On success, returns the number of bytes read and the address from where the data came.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// let mut buf = vec![0; 1024];
    /// let (n, peer) = socket.recv_from(&mut buf).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.watcher.recv_from(buf).await
    }

    /// Receives data from the socket.
    ///
    /// On success, returns the number of bytes read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::bind("/tmp/socket").await?;
    /// let mut buf = vec![0; 1024];
    /// let n = socket.recv(&mut buf).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.watcher.recv(buf).await
    }

    /// Sends data on the socket to the specified address.
    ///
    /// On success, returns the number of bytes written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// socket.send_to(b"hello world", "/tmp/socket").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn send_to<P: AsRef<Path>>(&self, buf: &[u8], path: P) -> io::Result<usize> {
        self.watcher.send_to(buf, path.as_ref()).await
    }

    /// Sends data on the socket to the socket's peer.
    ///
    /// On success, returns the number of bytes written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket").await?;
    /// socket.send(b"hello world").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.watcher.send(buf).await
    }

    /// Shut down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O calls on the specified portions to
    /// immediately return with an appropriate value (see the documentation of [`Shutdown`]).
    ///
    /// [`Shutdown`]: https://doc.rust-lang.org/std/net/enum.Shutdown.html
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::os::unix::net::UnixDatagram;
    /// use std::net::Shutdown;
    ///
    /// let socket = UnixDatagram::unbound()?;
    /// socket.shutdown(Shutdown::Both)?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.watcher.get_ref().shutdown(how)
    }
}

impl fmt::Debug for UnixDatagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("UnixDatagram");
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

impl From<StdUnixDatagram> for UnixDatagram {
    /// Converts a `std::os::unix::net::UnixDatagram` into its asynchronous equivalent.
    fn from(datagram: StdUnixDatagram) -> UnixDatagram {
        UnixDatagram {
            watcher: Async::new(datagram).expect("UnixDatagram is known to be good"),
        }
    }
}

impl std::convert::TryFrom<UnixDatagram> for StdUnixDatagram {
    type Error = io::Error;
    /// Converts a `UnixDatagram` into its synchronous equivalent.
    fn try_from(listener: UnixDatagram) -> io::Result<StdUnixDatagram> {
        let inner = listener.watcher.into_inner()?;
        inner.set_nonblocking(false)?;
        Ok(inner)
    }
}

impl AsRawFd for UnixDatagram {
    fn as_raw_fd(&self) -> RawFd {
        self.watcher.as_raw_fd()
    }
}

impl FromRawFd for UnixDatagram {
    unsafe fn from_raw_fd(fd: RawFd) -> UnixDatagram {
        let raw = StdUnixDatagram::from_raw_fd(fd);
        let datagram = Async::<StdUnixDatagram>::new(raw).expect("invalid file descriptor");
        UnixDatagram { watcher: datagram }
    }
}

impl IntoRawFd for UnixDatagram {
    fn into_raw_fd(self) -> RawFd {
        self.watcher.into_inner().unwrap().into_raw_fd()
    }
}

cfg_io_safety! {
    use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

    impl AsFd for UnixDatagram {
        fn as_fd(&self) -> BorrowedFd<'_> {
            self.watcher.get_ref().as_fd()
        }
    }

    impl From<OwnedFd> for UnixDatagram {
        fn from(fd: OwnedFd) -> UnixDatagram {
            StdUnixDatagram::from(fd).into()
        }
    }

    impl From<UnixDatagram> for OwnedFd {
        fn from(stream: UnixDatagram) -> OwnedFd {
            stream.watcher.into_inner().unwrap().into()
        }
    }
}
