use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::net::UnixStream as stdUnixStream;
use std::os::unix::net::UnixListener as stdUnixListener;
use std::io;

use tokio::net::{UnixStream, UnixListener, UnixDatagram};

use libc::SOCK_STREAM;

use crate::addr::UnixSocketAddr;
use crate::helpers::*;
use crate::credentials::*;

mod private {
    use super::*;
    pub trait Sealed {}
    impl Sealed for UnixStream {}
    impl Sealed for UnixListener {}
    impl Sealed for UnixDatagram {}
}

/// Extension trait for `tokio::net::UnixStream`.
///
/// Doesn't have `send_fds()` or `recv_fds()`,
/// because they would be `async` which isn't supported in traits yet.
pub trait UnixStreamExt: AsRawFd + private::Sealed {
    /// Get the address of this socket, as a type that fully supports abstract addresses.
    fn local_unix_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        get_unix_addr(self.as_raw_fd(), GetAddr::LOCAL)
    }
    /// Returns the address of the other end of this stream,
    /// as a type that fully supports abstract addresses.
    fn peer_unix_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        get_unix_addr(self.as_raw_fd(), GetAddr::PEER)
    }

    /// Creates a connection to a listening path-based or abstract named socket.
    fn connect_to_unix_addr(addr: &UnixSocketAddr) -> Result<Self, io::Error> where Self: Sized;

    /// Creates a path-based or abstract-named socket and connects to a listening socket.
    fn connect_from_to_unix_addr(from: &UnixSocketAddr,  to: &UnixSocketAddr)
    -> Result<Self, io::Error> where Self: Sized;

    /// Returns the credentials of the process that created the other end of this stream.
    fn initial_peer_credentials(&self) -> Result<ConnCredentials, io::Error> {
        peer_credentials(self.as_raw_fd())
    }
    /// Returns the SELinux security context of the process that created the other end of this stream.
    ///
    /// Will return an error on other operating systems than Linux or Android,
    /// and also if running under kubernetes.
    /// On success the number of bytes used is returned. (like `Read`)
    ///
    /// The default security context is `unconfined`, without any trailing NUL.  
    /// A buffor of 50 bytes is probably always big enough.
    fn initial_peer_selinux_context(&self,  buffer: &mut[u8]) -> Result<usize, io::Error> {
        selinux_context(self.as_raw_fd(), buffer)
    }
}

impl UnixStreamExt for UnixStream {
    fn connect_to_unix_addr(addr: &UnixSocketAddr) -> Result<Self, io::Error> {
        let socket = Socket::new(SOCK_STREAM, true)?;
        set_unix_addr(socket.as_raw_fd(), SetAddr::PEER, addr)?;
        UnixStream::from_std(unsafe { stdUnixStream::from_raw_fd(socket.into_raw_fd()) })
    }
    fn connect_from_to_unix_addr(from: &UnixSocketAddr,  to: &UnixSocketAddr)
    -> Result<Self, io::Error> {
        let socket = Socket::new(SOCK_STREAM, true)?;
        set_unix_addr(socket.as_raw_fd(), SetAddr::LOCAL, from)?;
        set_unix_addr(socket.as_raw_fd(), SetAddr::PEER, to)?;
        UnixStream::from_std(unsafe { stdUnixStream::from_raw_fd(socket.into_raw_fd()) })
    }
}

/// Extension trait for using [`UnixSocketAddr`](struct.UnixSocketAddr.html) with `tokio::net::UnixListener`.
///
/// Lacks `accept_unix_addr()` which is the most important part,
/// because it would be `async` which isn't supported in traits yet.
pub trait UnixListenerExt: AsRawFd + private::Sealed {
    type Conn;

    /// Creates a socket bound to a `UnixSocketAddr` and starts listening on it.
    fn bind_unix_addr(on: &UnixSocketAddr) -> Result<Self, io::Error> where Self: Sized;

    /// Returns the address this socket is listening on.
    fn local_unix_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        get_unix_addr(self.as_raw_fd(), GetAddr::LOCAL)
    }
}

impl UnixListenerExt for UnixListener {
    type Conn = UnixStream;

    fn bind_unix_addr(on: &UnixSocketAddr) -> Result<Self, io::Error> {
        let socket = Socket::new(SOCK_STREAM, true)?;
        set_unix_addr(socket.as_raw_fd(), SetAddr::LOCAL, on)?;
        socket.start_listening()?;
        UnixListener::from_std(unsafe { stdUnixListener::from_raw_fd(socket.into_raw_fd()) })
    }
}

/// Extension trait for `tokio::net::UnixDatagram`.
///
/// Only has the parts that don't require `async`, which isn't supported in traits yet.
pub trait UnixDatagramExt: AsRawFd + private::Sealed {
    /// Create a socket bound to a path or abstract name.
    ///
    /// # Examples
    ///
    #[cfg_attr(any(target_os="linux", target_os="android"), doc="```")]
    #[cfg_attr(not(any(target_os="linux", target_os="android")), doc="```no_run")]
    /// # use tokio::net::UnixDatagram;
    /// # use uds::{tokio::UnixDatagramExt, UnixSocketAddr};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), std::io::Error> {
    /// let addr = UnixSocketAddr::new("@abstract")?;
    /// let socket = UnixDatagram::bind_unix_addr(&addr)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// This is equivalent of:
    ///
    /// ```
    /// # use tokio::net::UnixDatagram;
    /// # use uds::{tokio::UnixDatagramExt, UnixSocketAddr};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), std::io::Error> {
    /// # let addr = UnixSocketAddr::new("me_async")?;
    /// let socket = UnixDatagram::unbound()?;
    /// socket.bind_to_unix_addr(&addr)?;
    /// # let _ = std::fs::remove_file("me_async");
    /// # Ok(())
    /// # }
    /// ```
    fn bind_unix_addr(addr: &UnixSocketAddr) -> Result<Self, io::Error> where Self: Sized;

    /// Returns the address of this socket, as a type that fully supports abstract addresses.
    fn local_unix_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        get_unix_addr(self.as_raw_fd(), GetAddr::LOCAL)
    }
    /// Returns the address of the connected socket, as a type that fully supports abstract addresses.
    fn peer_unix_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        get_unix_addr(self.as_raw_fd(), GetAddr::PEER)
    }

    /// Creates a path or abstract name for the socket.
    fn bind_to_unix_addr(&self,  addr: &UnixSocketAddr) -> Result<(), io::Error> {
        set_unix_addr(self.as_raw_fd(), SetAddr::LOCAL, addr)
    }
    /// Connects the socket to a path-based or abstract named socket.
    fn connect_to_unix_addr(&self,  addr: &UnixSocketAddr) -> Result<(), io::Error> {
        set_unix_addr(self.as_raw_fd(), SetAddr::PEER, addr)
    }

    /// Returns the credentials of the process that created a socket pair.
    ///
    /// This information is only available on Linux, and only for sockets that
    /// was created with `pair()` or the underlying `socketpair()`.
    /// For sockets that have merely been "connected" to an address
    /// or not connected at all, an error of kind `NotConnected`
    /// or `InvalidInput` is returned.
    ///
    /// The use cases of this function gotta be very narrow:
    ///
    /// * It will return the credentials of the current process unless
    ///   the side of the socket this method is called on was received via
    ///   FD-passing or inherited from a parent.
    /// * If it was created by the direct parent process,
    ///   one might as well use `getppid()` and go from there?
    /// * A returned pid can be repurposed by the OS before the call returns.
    /// * uids or groups will be those in effect when the pair was created,
    ///   and will not reflect changes in privileges.
    ///
    /// Despite these limitations, the feature is supported by Linux at least
    /// (but not macOS or FreeBSD), so might as well expose it.
    fn initial_pair_credentials(&self) -> Result<ConnCredentials, io::Error> {
        peer_credentials(self.as_raw_fd())
    }
    /// Returns the SELinux security context of the process that created a socket pair.
    ///
    /// Has the same limitations and gotchas as `initial_pair_credentials()`,
    /// and will return an error on other OSes than Linux or Android
    /// or if running under kubernetes.
    ///
    /// The default security context is the string `unconfined`.
    fn initial_pair_selinux_context(&self,  buffer: &mut[u8]) -> Result<usize, io::Error> {
        selinux_context(self.as_raw_fd(), buffer)
    }
}

impl UnixDatagramExt for UnixDatagram {
    fn bind_unix_addr(addr: &UnixSocketAddr) -> Result<Self, io::Error> {
        match UnixDatagram::unbound() {
            Ok(socket) => match socket.bind_to_unix_addr(addr) {
                Ok(()) => Ok(socket),
                Err(e) => Err(e),
            }
            Err(e) => Err(e)
        }
    }
}
