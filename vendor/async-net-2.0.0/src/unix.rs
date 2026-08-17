//! Unix domain sockets.
//!
//! This module is an async version of [`std::os::unix::net`].

use std::fmt;
use std::io::{self, Read as _, Write as _};
use std::net::Shutdown;
#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[doc(no_inline)]
pub use std::os::unix::net::SocketAddr;

use async_io::Async;
use futures_lite::{prelude::*, ready};

/// A Unix server, listening for connections.
///
/// After creating a [`UnixListener`] by [`bind`][`UnixListener::bind()`]ing it to an address, it
/// listens for incoming connections. These can be accepted by calling
/// [`accept()`][`UnixListener::accept()`] or by awaiting items from the async stream of
/// [`incoming`][`UnixListener::incoming()`] connections.
///
/// Cloning a [`UnixListener`] creates another handle to the same socket. The socket will be closed
/// when all handles to it are dropped.
///
/// # Examples
///
/// ```no_run
/// use async_net::unix::UnixListener;
/// use futures_lite::prelude::*;
///
/// # futures_lite::future::block_on(async {
/// let listener = UnixListener::bind("/tmp/socket")?;
/// let mut incoming = listener.incoming();
///
/// while let Some(stream) = incoming.next().await {
///     let mut stream = stream?;
///     stream.write_all(b"hello").await?;
/// }
/// # std::io::Result::Ok(()) });
/// ```
#[derive(Clone, Debug)]
pub struct UnixListener {
    inner: Arc<Async<std::os::unix::net::UnixListener>>,
}

impl UnixListener {
    fn new(inner: Arc<Async<std::os::unix::net::UnixListener>>) -> UnixListener {
        UnixListener { inner }
    }

    /// Creates a new [`UnixListener`] bound to the given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixListener;
    /// use futures_lite::prelude::*;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = UnixListener::bind("/tmp/socket")?;
    /// let mut incoming = listener.incoming();
    ///
    /// while let Some(stream) = incoming.next().await {
    ///     let mut stream = stream?;
    ///     stream.write_all(b"hello").await?;
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<UnixListener> {
        let listener = Async::<std::os::unix::net::UnixListener>::bind(path)?;
        Ok(UnixListener::new(Arc::new(listener)))
    }

    /// Accepts a new incoming connection.
    ///
    /// Returns a TCP stream and the address it is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixListener;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = UnixListener::bind("/tmp/socket")?;
    /// let (stream, addr) = listener.accept().await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        let (stream, addr) = self.inner.accept().await?;
        Ok((UnixStream::new(Arc::new(stream)), addr))
    }

    /// Returns a stream of incoming connections.
    ///
    /// Iterating over this stream is equivalent to calling [`accept()`][`UnixListener::accept()`]
    /// in a loop. The stream of connections is infinite, i.e awaiting the next connection will
    /// never result in [`None`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixListener;
    /// use futures_lite::prelude::*;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = UnixListener::bind("/tmp/socket")?;
    /// let mut incoming = listener.incoming();
    ///
    /// while let Some(stream) = incoming.next().await {
    ///     let mut stream = stream?;
    ///     stream.write_all(b"hello").await?;
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn incoming(&self) -> Incoming<'_> {
        Incoming {
            incoming: Box::pin(self.inner.incoming()),
        }
    }

    /// Returns the local address this listener is bound to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixListener;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = UnixListener::bind("/tmp/socket")?;
    /// println!("Local address is {:?}", listener.local_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }
}

impl From<Async<std::os::unix::net::UnixListener>> for UnixListener {
    fn from(listener: Async<std::os::unix::net::UnixListener>) -> UnixListener {
        UnixListener::new(Arc::new(listener))
    }
}

impl TryFrom<std::os::unix::net::UnixListener> for UnixListener {
    type Error = io::Error;

    fn try_from(listener: std::os::unix::net::UnixListener) -> io::Result<UnixListener> {
        Ok(UnixListener::new(Arc::new(Async::new(listener)?)))
    }
}

impl From<UnixListener> for Arc<Async<std::os::unix::net::UnixListener>> {
    fn from(val: UnixListener) -> Self {
        val.inner
    }
}

#[cfg(unix)]
impl AsRawFd for UnixListener {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for UnixListener {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.get_ref().as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<OwnedFd> for UnixListener {
    type Error = io::Error;

    fn try_from(value: OwnedFd) -> Result<Self, Self::Error> {
        Self::try_from(std::os::unix::net::UnixListener::from(value))
    }
}

#[cfg(windows)]
impl AsRawSocket for UnixListener {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}

/// A stream of incoming Unix connections.
///
/// This stream is infinite, i.e awaiting the next connection will never result in [`None`]. It is
/// created by the [`UnixListener::incoming()`] method.
pub struct Incoming<'a> {
    incoming: Pin<
        Box<
            dyn Stream<Item = io::Result<Async<std::os::unix::net::UnixStream>>> + Send + Sync + 'a,
        >,
    >,
}

impl Stream for Incoming<'_> {
    type Item = io::Result<UnixStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let res = ready!(Pin::new(&mut self.incoming).poll_next(cx));
        Poll::Ready(res.map(|res| res.map(|stream| UnixStream::new(Arc::new(stream)))))
    }
}

impl fmt::Debug for Incoming<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incoming {{ ... }}")
    }
}

/// A Unix connection.
///
/// A [`UnixStream`] can be created by [`connect`][`UnixStream::connect()`]ing to an endpoint or by
/// [`accept`][`UnixListener::accept()`]ing an incoming connection.
///
/// [`UnixStream`] is a bidirectional stream that implements traits [`AsyncRead`] and
/// [`AsyncWrite`].
///
/// Cloning a [`UnixStream`] creates another handle to the same socket. The socket will be closed
/// when all handles to it are dropped. The reading and writing portions of the connection can also
/// be shut down individually with the [`shutdown()`][`UnixStream::shutdown()`] method.
///
/// # Examples
///
/// ```no_run
/// use async_net::unix::UnixStream;
/// use futures_lite::prelude::*;
///
/// # futures_lite::future::block_on(async {
/// let mut stream = UnixStream::connect("/tmp/socket").await?;
/// stream.write_all(b"hello").await?;
///
/// let mut buf = vec![0u8; 1024];
/// let n = stream.read(&mut buf).await?;
/// # std::io::Result::Ok(()) });
/// ```
pub struct UnixStream {
    inner: Arc<Async<std::os::unix::net::UnixStream>>,
    readable: Option<async_io::ReadableOwned<std::os::unix::net::UnixStream>>,
    writable: Option<async_io::WritableOwned<std::os::unix::net::UnixStream>>,
}

impl UnwindSafe for UnixStream {}
impl RefUnwindSafe for UnixStream {}

impl UnixStream {
    fn new(inner: Arc<Async<std::os::unix::net::UnixStream>>) -> UnixStream {
        UnixStream {
            inner,
            readable: None,
            writable: None,
        }
    }

    /// Creates a Unix connection to given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<UnixStream> {
        let stream = Async::<std::os::unix::net::UnixStream>::connect(path).await?;
        Ok(UnixStream::new(Arc::new(stream)))
    }

    /// Creates a pair of connected Unix sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let (stream1, stream2) = UnixStream::pair()?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn pair() -> io::Result<(UnixStream, UnixStream)> {
        let (a, b) = Async::<std::os::unix::net::UnixStream>::pair()?;
        Ok((UnixStream::new(Arc::new(a)), UnixStream::new(Arc::new(b))))
    }

    /// Returns the local address this socket is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// println!("Local address is {:?}", stream.local_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }

    /// Returns the remote address this socket is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// println!("Connected to {:?}", stream.peer_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().peer_addr()
    }

    /// Shuts down the read half, write half, or both halves of this connection.
    ///
    /// This method will cause all pending and future I/O in the given directions to return
    /// immediately with an appropriate value (see the documentation of [`Shutdown`]).
    ///
    /// ```no_run
    /// use async_net::{Shutdown, unix::UnixStream};
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = UnixStream::connect("/tmp/socket").await?;
    /// stream.shutdown(Shutdown::Both)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.inner.get_ref().shutdown(how)
    }
}

impl fmt::Debug for UnixStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Clone for UnixStream {
    fn clone(&self) -> UnixStream {
        UnixStream::new(self.inner.clone())
    }
}

impl From<Async<std::os::unix::net::UnixStream>> for UnixStream {
    fn from(stream: Async<std::os::unix::net::UnixStream>) -> UnixStream {
        UnixStream::new(Arc::new(stream))
    }
}

impl TryFrom<std::os::unix::net::UnixStream> for UnixStream {
    type Error = io::Error;

    fn try_from(stream: std::os::unix::net::UnixStream) -> io::Result<UnixStream> {
        Ok(UnixStream::new(Arc::new(Async::new(stream)?)))
    }
}

impl From<UnixStream> for Arc<Async<std::os::unix::net::UnixStream>> {
    fn from(val: UnixStream) -> Self {
        val.inner
    }
}

#[cfg(unix)]
impl AsRawFd for UnixStream {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for UnixStream {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.get_ref().as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<OwnedFd> for UnixStream {
    type Error = io::Error;

    fn try_from(value: OwnedFd) -> Result<Self, Self::Error> {
        Self::try_from(std::os::unix::net::UnixStream::from(value))
    }
}

#[cfg(windows)]
impl AsRawSocket for UnixStream {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}

impl AsyncRead for UnixStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            // Attempt the non-blocking operation.
            match self.inner.get_ref().read(buf) {
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
                res => {
                    self.readable = None;
                    return Poll::Ready(res);
                }
            }

            // Initialize the future to wait for readiness.
            if self.readable.is_none() {
                self.readable = Some(self.inner.clone().readable_owned());
            }

            // Poll the future for readiness.
            if let Some(f) = &mut self.readable {
                let res = ready!(Pin::new(f).poll(cx));
                self.readable = None;
                res?;
            }
        }
    }
}

impl AsyncWrite for UnixStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            // Attempt the non-blocking operation.
            match self.inner.get_ref().write(buf) {
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
                res => {
                    self.writable = None;
                    return Poll::Ready(res);
                }
            }

            // Initialize the future to wait for readiness.
            if self.writable.is_none() {
                self.writable = Some(self.inner.clone().writable_owned());
            }

            // Poll the future for readiness.
            if let Some(f) = &mut self.writable {
                let res = ready!(Pin::new(f).poll(cx));
                self.writable = None;
                res?;
            }
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        loop {
            // Attempt the non-blocking operation.
            match self.inner.get_ref().flush() {
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
                res => {
                    self.writable = None;
                    return Poll::Ready(res);
                }
            }

            // Initialize the future to wait for readiness.
            if self.writable.is_none() {
                self.writable = Some(self.inner.clone().writable_owned());
            }

            // Poll the future for readiness.
            if let Some(f) = &mut self.writable {
                let res = ready!(Pin::new(f).poll(cx));
                self.writable = None;
                res?;
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(self.inner.get_ref().shutdown(Shutdown::Write))
    }
}

/// A Unix datagram socket.
///
/// After creating a [`UnixDatagram`] by [`bind`][`UnixDatagram::bind()`]ing it to a path, data can
/// be [sent to] and [received from] any other socket address.
///
/// Cloning a [`UnixDatagram`] creates another handle to the same socket. The socket will be closed
/// when all handles to it are dropped. The reading and writing portions of the socket can also be
/// shut down individually with the [`shutdown()`][`UnixStream::shutdown()`] method.
///
/// [received from]: UnixDatagram::recv_from()
/// [sent to]: UnixDatagram::send_to()
///
/// # Examples
///
/// ```no_run
/// use async_net::unix::UnixDatagram;
///
/// # futures_lite::future::block_on(async {
/// let socket = UnixDatagram::bind("/tmp/socket1")?;
/// socket.send_to(b"hello", "/tmp/socket2").await?;
///
/// let mut buf = vec![0u8; 1024];
/// let (n, addr) = socket.recv_from(&mut buf).await?;
/// # std::io::Result::Ok(()) });
/// ```
#[derive(Clone, Debug)]
pub struct UnixDatagram {
    inner: Arc<Async<std::os::unix::net::UnixDatagram>>,
}

impl UnixDatagram {
    fn new(inner: Arc<Async<std::os::unix::net::UnixDatagram>>) -> UnixDatagram {
        UnixDatagram { inner }
    }

    /// Creates a new [`UnixDatagram`] bound to the given address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::bind("/tmp/socket")?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<UnixDatagram> {
        let socket = Async::<std::os::unix::net::UnixDatagram>::bind(path)?;
        Ok(UnixDatagram::new(Arc::new(socket)))
    }

    /// Creates a Unix datagram socket not bound to any address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn unbound() -> io::Result<UnixDatagram> {
        let socket = Async::<std::os::unix::net::UnixDatagram>::unbound()?;
        Ok(UnixDatagram::new(Arc::new(socket)))
    }

    /// Creates a pair of connected Unix datagram sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let (socket1, socket2) = UnixDatagram::pair()?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn pair() -> io::Result<(UnixDatagram, UnixDatagram)> {
        let (a, b) = Async::<std::os::unix::net::UnixDatagram>::pair()?;
        Ok((
            UnixDatagram::new(Arc::new(a)),
            UnixDatagram::new(Arc::new(b)),
        ))
    }

    /// Connects the Unix datagram socket to the given address.
    ///
    /// When connected, methods [`send()`][`UnixDatagram::send()`] and
    /// [`recv()`][`UnixDatagram::recv()`] will use the specified address for sending and receiving
    /// messages. Additionally, a filter will be applied to
    /// [`recv_from()`][`UnixDatagram::recv_from()`] so that it only receives messages from that
    /// same address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket")?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn connect<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let p = path.as_ref();
        self.inner.get_ref().connect(p)
    }

    /// Returns the local address this socket is bound to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::bind("/tmp/socket")?;
    /// println!("Bound to {:?}", socket.local_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }

    /// Returns the remote address this socket is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket")?;
    /// println!("Connected to {:?}", socket.peer_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().peer_addr()
    }

    /// Receives data from an address.
    ///
    /// On success, returns the number of bytes received and the address data came from.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::bind("/tmp/socket")?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let (n, addr) = socket.recv_from(&mut buf).await?;
    /// println!("Received {} bytes from {:?}", n, addr);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.inner.recv_from(buf).await
    }

    /// Sends data to the given address.
    ///
    /// On success, returns the number of bytes sent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// socket.send_to(b"hello", "/tmp/socket").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn send_to<P: AsRef<Path>>(&self, buf: &[u8], path: P) -> io::Result<usize> {
        self.inner.send_to(buf, path.as_ref()).await
    }

    /// Receives data from the connected address.
    ///
    /// On success, returns the number of bytes received.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket")?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let n = socket.recv(&mut buf).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.recv(buf).await
    }

    /// Sends data to the connected address.
    ///
    /// On success, returns the number of bytes sent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::unix::UnixDatagram;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// socket.connect("/tmp/socket")?;
    /// socket.send(b"hello").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.inner.send(buf).await
    }

    /// Shuts down the read half, write half, or both halves of this socket.
    ///
    /// This method will cause all pending and future I/O in the given directions to return
    /// immediately with an appropriate value (see the documentation of [`Shutdown`]).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::{Shutdown, unix::UnixDatagram};
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UnixDatagram::unbound()?;
    /// socket.shutdown(Shutdown::Both)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.inner.get_ref().shutdown(how)
    }
}

impl From<Async<std::os::unix::net::UnixDatagram>> for UnixDatagram {
    fn from(socket: Async<std::os::unix::net::UnixDatagram>) -> UnixDatagram {
        UnixDatagram::new(Arc::new(socket))
    }
}

impl TryFrom<std::os::unix::net::UnixDatagram> for UnixDatagram {
    type Error = io::Error;

    fn try_from(socket: std::os::unix::net::UnixDatagram) -> io::Result<UnixDatagram> {
        Ok(UnixDatagram::new(Arc::new(Async::new(socket)?)))
    }
}

impl From<UnixDatagram> for Arc<Async<std::os::unix::net::UnixDatagram>> {
    fn from(val: UnixDatagram) -> Self {
        val.inner
    }
}

#[cfg(unix)]
impl AsRawFd for UnixDatagram {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for UnixDatagram {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}
