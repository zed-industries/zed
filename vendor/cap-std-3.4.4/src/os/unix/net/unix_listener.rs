use crate::os::unix::net::{Incoming, SocketAddr, UnixStream};
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::os::unix;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{fmt, io};

/// A structure representing a Unix domain socket server.
///
/// This corresponds to [`std::os::unix::net::UnixListener`].
///
/// This `UnixListener` has no `bind` method. To bind it to a socket address,
/// first obtain a [`Dir`] containing the path, and then call
/// [`Dir::bind_unix_listener`].
///
/// [`std::os::unix::net::UnixListener`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::bind_unix_listener`]: struct.Dir.html#method.bind_unix_listener
pub struct UnixListener {
    std: unix::net::UnixListener,
}

impl UnixListener {
    /// Constructs a new instance of `Self` from the given
    /// `std::os::unix::net::UnixListener`.
    ///
    /// This grants access the resources the `std::os::unix::net::UnixListener`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: unix::net::UnixListener) -> Self {
        Self { std }
    }

    /// Accepts a new incoming connection to this listener.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::accept`].
    ///
    /// [`std::os::unix::net::UnixListener::accept`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.accept
    #[inline]
    pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        self.std
            .accept()
            .map(|(unix_stream, addr)| (UnixStream::from_std(unix_stream), addr))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::try_clone`].
    ///
    /// [`std::os::unix::net::UnixListener::try_clone`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let unix_listener = self.std.try_clone()?;
        Ok(Self::from_std(unix_listener))
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::local_addr`].
    ///
    /// [`std::os::unix::net::UnixListener::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixListener::set_nonblocking`].
    ///
    /// [`std::os::unix::net::UnixListener::set_nonblocking`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.set_nonblocking
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::take_error`].
    ///
    /// [`std::os::unix::net::UnixListener::take_error`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.take_error
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Returns an iterator over incoming connections.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::incoming`].
    ///
    /// [`std::os::unix::net::UnixListener::incoming`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.incoming
    #[inline]
    pub fn incoming(&self) -> Incoming {
        let incoming = self.std.incoming();
        Incoming::from_std(incoming)
    }
}

impl FromRawFd for UnixListener {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(unix::net::UnixListener::from_raw_fd(fd))
    }
}

impl From<OwnedFd> for UnixListener {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(unix::net::UnixListener::from(fd))
    }
}

impl AsRawFd for UnixListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl AsFd for UnixListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

impl IntoRawFd for UnixListener {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

impl From<UnixListener> for OwnedFd {
    #[inline]
    fn from(listener: UnixListener) -> OwnedFd {
        listener.std.into()
    }
}

impl<'a> IntoIterator for &'a UnixListener {
    type IntoIter = Incoming<'a>;
    type Item = io::Result<UnixStream>;

    #[inline]
    fn into_iter(self) -> Incoming<'a> {
        let incoming = self.std.into_iter();
        Incoming::from_std(incoming)
    }
}

impl fmt::Debug for UnixListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
