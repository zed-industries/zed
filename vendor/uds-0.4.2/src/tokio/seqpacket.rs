use crate::{nonblocking, UnixSocketAddr, ConnCredentials};

use std::io::{self, IoSlice, IoSliceMut};
use std::net::Shutdown;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::path::Path;

use tokio_crate::io::Interest;
use tokio_crate::io::unix::AsyncFd;

/// An I/O object representing a Unix Sequenced-packet socket.
pub struct UnixSeqpacketConn {
    io: AsyncFd<nonblocking::UnixSeqpacketConn>,
}

impl UnixSeqpacketConn {
    /// Connects to the socket named by path.
    ///
    /// This function will create a new Unix socket and connects to the path
    /// specified, associating the returned stream with the default event loop's
    /// handle.
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let conn = nonblocking::UnixSeqpacketConn::connect(path)?;
        Self::from_nonblocking(conn)
    }
    /// Connects to an unix seqpacket server listening at `addr`.
    pub fn connect_addr(addr: &UnixSocketAddr) -> io::Result<Self> {
        let conn = nonblocking::UnixSeqpacketConn::connect_unix_addr(addr)?;
        Self::from_nonblocking(conn)
    }
    /// Binds to an address before connecting to a listening seqpacet socket.
    pub fn connect_from_addr(from: &UnixSocketAddr,  to: &UnixSocketAddr)
    -> io::Result<Self> {
        let conn = nonblocking::UnixSeqpacketConn::connect_from_to_unix_addr(from, to)?;
        Self::from_nonblocking(conn)
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This function will create a pair of interconnected Unix sockets for
    /// communicating back and forth between one another. Each socket will
    /// be associated with the default event loop's handle.
    pub fn pair() -> Result<(UnixSeqpacketConn, UnixSeqpacketConn), io::Error> {
        let (a, b) = nonblocking::UnixSeqpacketConn::pair()?;
        let a = Self::from_nonblocking(a)?;
        let b = Self::from_nonblocking(b)?;

        Ok((a, b))
    }

    /// Creates a tokio-compatible socket from an existing nonblocking socket.
    pub fn from_nonblocking(conn: nonblocking::UnixSeqpacketConn) -> Result<Self, io::Error> {
        match AsyncFd::new(conn) {
            Ok(io) => Ok(Self { io }),
            Err(e) => Err(e),
        }
    }
    /// Deregisters the connection and returns the underlying non-blocking type.
    pub fn into_nonblocking(self) -> nonblocking::UnixSeqpacketConn {
        self.io.into_inner()
    }
    /// Creates a tokio-compatible socket from a raw file descriptor.
    ///
    /// This function is provided instead of implementing [`FromRawFd`](std::os::unix::io::FromRawFd)
    /// because registering with the reactor might fail.
    ///
    /// # Safety
    ///
    /// The file descriptor must represent a connected seqpacket socket.
    pub unsafe fn from_raw_fd(fd: RawFd) -> Result<Self, io::Error> {
        Self::from_nonblocking(nonblocking::UnixSeqpacketConn::from_raw_fd(fd))
    }

    /// Shuts down the read, write, or both halves of this connection.
    pub fn shutdown(&self,  how: Shutdown) -> Result<(), io::Error> {
        self.io.get_ref().shutdown(how)
    }

    /// Returns the address of this side of the connection.
    pub fn local_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        self.io.get_ref().local_unix_addr()
    }
    /// Returns the address of the other side of the connection.
    pub fn peer_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        self.io.get_ref().peer_unix_addr()
    }

    /// Returns information about the process of the peer when the connection was established.
    ///
    /// See documentation of the returned type for details.
    pub fn initial_peer_credentials(&self) -> Result<ConnCredentials, io::Error> {
        self.io.get_ref().initial_peer_credentials()
    }
    /// Returns the SELinux security context of the process that created the other
    /// end of this connection.
    ///
    /// Will return an error on other operating systems than Linux or Android,
    /// and also if running inside kubernetes.
    /// On success the number of bytes used is returned. (like `Read`)
    ///
    /// The default security context is `unconfined`, without any trailing NUL.  
    /// A buffor of 50 bytes is probably always big enough.
    pub fn initial_peer_selinux_context(&self,  buffer: &mut[u8]) -> Result<usize, io::Error> {
        self.io.get_ref().initial_peer_selinux_context(buffer)
    }

    /// Returns the value of the `SO_ERROR` option.
    pub fn take_error(&self) -> Result<Option<io::Error>, io::Error> {
        self.io.get_ref().take_error()
    }
}

impl UnixSeqpacketConn {
    /// Sends a packet to the socket's peer.
    pub async fn send(&mut self,  packet: &[u8]) -> io::Result<usize> {
        self.io.async_io(Interest::WRITABLE, |conn| conn.send(packet) ).await
    }
    /// Receives a packet from the socket's peer.
    pub async fn recv(&mut self,  buffer: &mut[u8]) -> io::Result<usize> {
        self.io.async_io(Interest::READABLE, |conn| conn.recv(buffer) ).await
    }

    /// Sends a packet assembled from multiple byte slices.
    pub async fn send_vectored<'a, 'b>
    (&'a mut self,  slices: &'b [IoSlice<'b>]) -> io::Result<usize> {
        self.io.async_io(Interest::WRITABLE, |conn| conn.send_vectored(slices) ).await
    }
    /// Receives a packet and places the bytes across multiple buffers.
    pub async fn recv_vectored<'a, 'b>
    (&'a mut self,  buffers: &'b mut [IoSliceMut<'b>]) -> io::Result<usize> {
        self.io.async_io(
                Interest::READABLE,
                |conn| conn.recv_vectored(buffers).map(|(received, _)| received )
        ).await
    }

    /// Receives a packet without removing it from the incoming queue.
    pub async fn peek(&mut self,  buffer: &mut[u8]) -> io::Result<usize> {
        self.io.async_io(Interest::READABLE, |conn| conn.peek(buffer) ).await
    }
    /// Reads a packet into multiple buffers without removing it from the incoming queue.
    pub async fn peek_vectored<'a, 'b>
    (&'a mut self,  buffers: &'b mut [IoSliceMut<'b>]) -> io::Result<usize> {
        self.io.async_io(
                Interest::READABLE,
                |conn| conn.peek_vectored(buffers).map(|(received, _)| received )
        ).await
    }

    /// Sends a packet with associated file descriptors.
    pub async fn send_fds(&mut self,  bytes: &[u8],  fds: &[RawFd]) -> io::Result<usize> {
        self.io.async_io(Interest::WRITABLE, |conn| conn.send_fds(bytes, fds) ).await
    }
    /// Receives a packet and associated file descriptors.
    pub async fn recv_fds(&mut self,  byte_buffer: &mut[u8],  fd_buffer: &mut[RawFd])
    -> io::Result<(usize, bool, usize)> {
        self.io.async_io(Interest::READABLE, |conn| conn.recv_fds(byte_buffer, fd_buffer) ).await
    }
}

impl AsRef<nonblocking::UnixSeqpacketConn> for UnixSeqpacketConn {
    fn as_ref(&self) -> &nonblocking::UnixSeqpacketConn {
        self.io.get_ref()
    }
}

impl AsRawFd for UnixSeqpacketConn {
    fn as_raw_fd(&self) -> RawFd {
        self.io.get_ref().as_raw_fd()
    }
}

impl IntoRawFd for UnixSeqpacketConn {
    fn into_raw_fd(self) -> RawFd {
        self.io.into_inner().into_raw_fd()
    }
}



/// An I/O object representing a Unix Sequenced-packet socket.
pub struct UnixSeqpacketListener {
    io: AsyncFd<nonblocking::UnixSeqpacketListener>,
}

impl UnixSeqpacketListener {
    /// Creates a socket that listens for seqpacket connections on the specified socket file.
    pub fn bind<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        match nonblocking::UnixSeqpacketListener::bind(path.as_ref()) {
            Ok(listener) => Self::from_nonblocking(listener),
            Err(e) => Err(e),
        }
    }
    /// Creates a socket that listens for seqpacket connections on the specified address.
    pub fn bind_addr(addr: &UnixSocketAddr) -> Result<Self, io::Error> {
        match nonblocking::UnixSeqpacketListener::bind_unix_addr(addr) {
            Ok(listener) => Self::from_nonblocking(listener),
            Err(e) => Err(e),
        }
    }

    /// Creates a tokio-compatible listener from an existing nonblocking listener.
    pub fn from_nonblocking(listener: nonblocking::UnixSeqpacketListener)
    -> Result<Self, io::Error> {
        match AsyncFd::with_interest(listener, Interest::READABLE) {
            Ok(io) => Ok(Self { io }),
            Err(e) => Err(e),
        }
    }
    /// Deregisters the listener and returns the underlying non-blocking type.
    pub fn into_nonblocking(self) -> nonblocking::UnixSeqpacketListener {
        self.io.into_inner()
    }
    /// Creates a tokio-compatible listener from a raw file descriptor.
    ///
    /// This function is provided instead of implementing [`FromRawFd`](std::os::unix::io::FromRawFd)
    /// because registering with the reactor might fail.
    ///
    /// # Safety
    ///
    /// The file descriptor must represent a non-blocking seqpacket listener.
    pub unsafe fn from_raw_fd(fd: RawFd) -> Result<Self, io::Error> {
        Self::from_nonblocking(nonblocking::UnixSeqpacketListener::from_raw_fd(fd))
    }

    /// Accepts a new incoming connection to this listener.
    pub async fn accept(&mut self) -> io::Result<(UnixSeqpacketConn, UnixSocketAddr)> {
        let (conn, addr) = self.io.async_io(
                Interest::READABLE,
                |inner| inner.accept_unix_addr()
        ).await?;
        let conn = UnixSeqpacketConn::from_nonblocking(conn)?;
        Ok((conn, addr))
    }

    /// Returns the address the socket is listening on.
    pub fn local_addr(&self) -> Result<UnixSocketAddr, io::Error> {
        self.io.get_ref().local_unix_addr()
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This might never produce any errors for listeners. It is therefore
    /// unlikely to be useful, but is provided for parity with
    /// `std::unix::net::UnixListener`.
    pub fn take_error(&self) -> Result<Option<io::Error>, io::Error> {
        self.io.get_ref().take_error()
    }
}

impl AsRef<nonblocking::UnixSeqpacketListener> for UnixSeqpacketListener {
    fn as_ref(&self) -> &nonblocking::UnixSeqpacketListener {
        self.io.get_ref()
    }
}

impl AsRawFd for UnixSeqpacketListener {
    fn as_raw_fd(&self) -> RawFd {
        self.io.get_ref().as_raw_fd()
    }
}

impl IntoRawFd for UnixSeqpacketListener {
    fn into_raw_fd(self) -> RawFd {
        self.io.into_inner().into_raw_fd()
    }
}
