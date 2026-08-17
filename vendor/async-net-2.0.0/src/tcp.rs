use std::fmt;
use std::io::{self, IoSlice, Read as _, Write as _};
use std::net::{Shutdown, SocketAddr};
#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, AsSocket, BorrowedSocket, OwnedSocket, RawSocket};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use async_io::Async;
use futures_lite::{prelude::*, ready};

use crate::addr::AsyncToSocketAddrs;

/// A TCP server, listening for connections.
///
/// After creating a [`TcpListener`] by [`bind`][`TcpListener::bind()`]ing it to an address, it
/// listens for incoming TCP connections. These can be accepted by calling
/// [`accept()`][`TcpListener::accept()`] or by awaiting items from the stream of
/// [`incoming`][`TcpListener::incoming()`] connections.
///
/// Cloning a [`TcpListener`] creates another handle to the same socket. The socket will be closed
/// when all handles to it are dropped.
///
/// The Transmission Control Protocol is specified in [IETF RFC 793].
///
/// [IETF RFC 793]: https://tools.ietf.org/html/rfc793
///
/// # Examples
///
/// ```no_run
/// use async_net::TcpListener;
/// use futures_lite::prelude::*;
///
/// # futures_lite::future::block_on(async {
/// let listener = TcpListener::bind("127.0.0.1:8080").await?;
/// let mut incoming = listener.incoming();
///
/// while let Some(stream) = incoming.next().await {
///     let mut stream = stream?;
///     stream.write_all(b"hello").await?;
/// }
/// # std::io::Result::Ok(()) });
/// ```
#[derive(Clone, Debug)]
pub struct TcpListener {
    inner: Arc<Async<std::net::TcpListener>>,
}

impl TcpListener {
    fn new(inner: Arc<Async<std::net::TcpListener>>) -> TcpListener {
        TcpListener { inner }
    }

    /// Creates a new [`TcpListener`] bound to the given address.
    ///
    /// Binding with a port number of 0 will request that the operating system assigns an available
    /// port to this listener. The assigned port can be queried via the
    /// [`local_addr()`][`TcpListener::local_addr()`] method.
    ///
    /// If `addr` yields multiple addresses, binding will be attempted with each of the addresses
    /// until one succeeds and returns the listener. If none of the addresses succeed in creating a
    /// listener, the error from the last attempt is returned.
    ///
    /// # Examples
    ///
    /// Create a TCP listener bound to `127.0.0.1:80`:
    ///
    /// ```no_run
    /// use async_net::TcpListener;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = TcpListener::bind("127.0.0.1:80").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    ///
    /// Create a TCP listener bound to `127.0.0.1:80`. If that address is unavailable, then try
    /// binding to `127.0.0.1:443`:
    ///
    /// ```no_run
    /// use async_net::{SocketAddr, TcpListener};
    ///
    /// # futures_lite::future::block_on(async {
    /// let addrs = [
    ///     SocketAddr::from(([127, 0, 0, 1], 80)),
    ///     SocketAddr::from(([127, 0, 0, 1], 443)),
    /// ];
    /// let listener = TcpListener::bind(&addrs[..]).await.unwrap();
    /// # std::io::Result::Ok(()) });
    pub async fn bind<A: AsyncToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        let mut last_err = None;

        for addr in addr.to_socket_addrs().await? {
            match Async::<std::net::TcpListener>::bind(addr) {
                Ok(listener) => return Ok(TcpListener::new(Arc::new(listener))),
                Err(err) => last_err = Some(err),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve to any of the addresses",
            )
        }))
    }

    /// Returns the local address this listener is bound to.
    ///
    /// # Examples
    ///
    /// Bind to port 0 and then see which port was assigned by the operating system:
    ///
    /// ```no_run
    /// use async_net::{SocketAddr, TcpListener};
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = TcpListener::bind("127.0.0.1:0").await?;
    /// println!("Listening on {}", listener.local_addr()?);
    /// # std::io::Result::Ok(()) });
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }

    /// Accepts a new incoming connection.
    ///
    /// Returns a TCP stream and the address it is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpListener;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = TcpListener::bind("127.0.0.1:8080").await?;
    /// let (stream, addr) = listener.accept().await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let (stream, addr) = self.inner.accept().await?;
        Ok((TcpStream::new(Arc::new(stream)), addr))
    }

    /// Returns a stream of incoming connections.
    ///
    /// Iterating over this stream is equivalent to calling [`accept()`][`TcpListener::accept()`]
    /// in a loop. The stream of connections is infinite, i.e awaiting the next connection will
    /// never result in [`None`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpListener;
    /// use futures_lite::prelude::*;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = TcpListener::bind("127.0.0.1:0").await?;
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

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This option configures the time-to-live field that is used in every packet sent from this
    /// socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpListener;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = TcpListener::bind("127.0.0.1:80").await?;
    /// listener.set_ttl(100)?;
    /// assert_eq!(listener.ttl()?, 100);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn ttl(&self) -> io::Result<u32> {
        self.inner.get_ref().ttl()
    }

    /// Sets the value of the `IP_TTL` option for this socket.
    ///
    /// This option configures the time-to-live field that is used in every packet sent from this
    /// socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpListener;
    ///
    /// # futures_lite::future::block_on(async {
    /// let listener = TcpListener::bind("127.0.0.1:80").await?;
    /// listener.set_ttl(100)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.inner.get_ref().set_ttl(ttl)
    }
}

impl From<Async<std::net::TcpListener>> for TcpListener {
    fn from(listener: Async<std::net::TcpListener>) -> TcpListener {
        TcpListener::new(Arc::new(listener))
    }
}

impl TryFrom<std::net::TcpListener> for TcpListener {
    type Error = io::Error;

    fn try_from(listener: std::net::TcpListener) -> io::Result<TcpListener> {
        Ok(TcpListener::new(Arc::new(Async::new(listener)?)))
    }
}

impl From<TcpListener> for Arc<Async<std::net::TcpListener>> {
    fn from(val: TcpListener) -> Self {
        val.inner
    }
}

#[cfg(unix)]
impl AsRawFd for TcpListener {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for TcpListener {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.get_ref().as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<OwnedFd> for TcpListener {
    type Error = io::Error;

    fn try_from(value: OwnedFd) -> Result<Self, Self::Error> {
        Self::try_from(std::net::TcpListener::from(value))
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpListener {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for TcpListener {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.inner.get_ref().as_socket()
    }
}

#[cfg(windows)]
impl TryFrom<OwnedSocket> for TcpListener {
    type Error = io::Error;

    fn try_from(value: OwnedSocket) -> Result<Self, Self::Error> {
        Self::try_from(std::net::TcpListener::from(value))
    }
}

/// A stream of incoming TCP connections.
///
/// This stream is infinite, i.e awaiting the next connection will never result in [`None`]. It is
/// created by the [`TcpListener::incoming()`] method.
pub struct Incoming<'a> {
    incoming:
        Pin<Box<dyn Stream<Item = io::Result<Async<std::net::TcpStream>>> + Send + Sync + 'a>>,
}

impl Stream for Incoming<'_> {
    type Item = io::Result<TcpStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let res = ready!(Pin::new(&mut self.incoming).poll_next(cx));
        Poll::Ready(res.map(|res| res.map(|stream| TcpStream::new(Arc::new(stream)))))
    }
}

impl fmt::Debug for Incoming<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incoming {{ ... }}")
    }
}

/// A TCP connection.
///
/// A [`TcpStream`] can be created by [`connect`][`TcpStream::connect()`]ing to an endpoint or by
/// [`accept`][`TcpListener::accept()`]ing an incoming connection.
///
/// [`TcpStream`] is a bidirectional stream that implements traits [`AsyncRead`] and
/// [`AsyncWrite`].
///
/// Cloning a [`TcpStream`] creates another handle to the same socket. The socket will be closed
/// when all handles to it are dropped. The reading and writing portions of the connection can also
/// be shut down individually with the [`shutdown()`][`TcpStream::shutdown()`] method.
///
/// The Transmission Control Protocol is specified in [IETF RFC 793].
///
/// [IETF RFC 793]: https://tools.ietf.org/html/rfc793
///
/// # Examples
///
/// ```no_run
/// use async_net::TcpStream;
/// use futures_lite::prelude::*;
///
/// # futures_lite::future::block_on(async {
/// let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
/// stream.write_all(b"hello").await?;
///
/// let mut buf = vec![0u8; 1024];
/// let n = stream.read(&mut buf).await?;
/// # std::io::Result::Ok(()) });
/// ```
pub struct TcpStream {
    inner: Arc<Async<std::net::TcpStream>>,
    readable: Option<async_io::ReadableOwned<std::net::TcpStream>>,
    writable: Option<async_io::WritableOwned<std::net::TcpStream>>,
}

impl UnwindSafe for TcpStream {}
impl RefUnwindSafe for TcpStream {}

impl TcpStream {
    fn new(inner: Arc<Async<std::net::TcpStream>>) -> TcpStream {
        TcpStream {
            inner,
            readable: None,
            writable: None,
        }
    }

    /// Creates a TCP connection to the specified address.
    ///
    /// This method will create a new TCP socket and attempt to connect it to the provided `addr`,
    ///
    /// If `addr` yields multiple addresses, connecting will be attempted with each of the
    /// addresses until connecting to one succeeds. If none of the addresses result in a successful
    /// connection, the error from the last connect attempt is returned.
    ///
    /// # Examples
    ///
    /// Connect to `example.com:80`:
    ///
    /// ```
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("example.com:80").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    ///
    /// Connect to `127.0.0.1:8080`. If that fails, then try connecting to `127.0.0.1:8081`:
    ///
    /// ```no_run
    /// use async_net::{SocketAddr, TcpStream};
    ///
    /// # futures_lite::future::block_on(async {
    /// let addrs = [
    ///     SocketAddr::from(([127, 0, 0, 1], 8080)),
    ///     SocketAddr::from(([127, 0, 0, 1], 8081)),
    /// ];
    /// let stream = TcpStream::connect(&addrs[..]).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn connect<A: AsyncToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        let mut last_err = None;

        for addr in addr.to_socket_addrs().await? {
            match Async::<std::net::TcpStream>::connect(addr).await {
                Ok(stream) => return Ok(TcpStream::new(Arc::new(stream))),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not connect to any of the addresses",
            )
        }))
    }

    /// Returns the local address this stream is bound to.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("example.com:80").await?;
    /// println!("Local address is {}", stream.local_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }

    /// Returns the remote address this stream is connected to.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("example.com:80").await?;
    /// println!("Connected to {}", stream.peer_addr()?);
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
    /// [`Shutdown`]: https://doc.rust-lang.org/std/net/enum.Shutdown.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::{Shutdown, TcpStream};
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// stream.shutdown(Shutdown::Both)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.inner.get_ref().shutdown(how)
    }

    /// Receives data without removing it from the queue.
    ///
    /// On success, returns the number of bytes peeked.
    ///
    /// Successive calls return the same data. This is accomplished by passing `MSG_PEEK` as a flag
    /// to the underlying `recv` system call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let n = stream.peek(&mut buf).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.peek(buf).await
    }

    /// Gets the value of the `TCP_NODELAY` option for this socket.
    ///
    /// If set to `true`, this option disables the [Nagle algorithm][nagle-wiki]. This means that
    /// written data is always sent as soon as possible, even if there is only a small amount of
    /// it.
    ///
    /// When set to `false`, written data is buffered until there is a certain amount to send out,
    /// thereby avoiding the frequent sending of small packets.
    ///
    /// [nagle-wiki]: https://en.wikipedia.org/wiki/Nagle%27s_algorithm
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// println!("TCP_NODELAY is set to {}", stream.nodelay()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn nodelay(&self) -> io::Result<bool> {
        self.inner.get_ref().nodelay()
    }

    /// Sets the value of the `TCP_NODELAY` option for this socket.
    ///
    /// If set to `true`, this option disables the [Nagle algorithm][nagle-wiki]. This means that
    /// written data is always sent as soon as possible, even if there is only a small amount of
    /// it.
    ///
    /// When set to `false`, written data is buffered until there is a certain amount to send out,
    /// thereby avoiding the frequent sending of small packets.
    ///
    /// [nagle-wiki]: https://en.wikipedia.org/wiki/Nagle%27s_algorithm
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// stream.set_nodelay(false)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.inner.get_ref().set_nodelay(nodelay)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This option configures the time-to-live field that is used in every packet sent from this
    /// socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// println!("IP_TTL is set to {}", stream.ttl()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn ttl(&self) -> io::Result<u32> {
        self.inner.get_ref().ttl()
    }

    /// Sets the value of the `IP_TTL` option for this socket.
    ///
    /// This option configures the time-to-live field that is used in every packet sent from this
    /// socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::TcpStream;
    ///
    /// # futures_lite::future::block_on(async {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// stream.set_ttl(100)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.inner.get_ref().set_ttl(ttl)
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Clone for TcpStream {
    fn clone(&self) -> TcpStream {
        TcpStream::new(self.inner.clone())
    }
}

impl From<Async<std::net::TcpStream>> for TcpStream {
    fn from(stream: Async<std::net::TcpStream>) -> TcpStream {
        TcpStream::new(Arc::new(stream))
    }
}

impl From<TcpStream> for Arc<Async<std::net::TcpStream>> {
    fn from(val: TcpStream) -> Self {
        val.inner
    }
}

impl TryFrom<std::net::TcpStream> for TcpStream {
    type Error = io::Error;

    fn try_from(stream: std::net::TcpStream) -> io::Result<TcpStream> {
        Ok(TcpStream::new(Arc::new(Async::new(stream)?)))
    }
}

#[cfg(unix)]
impl AsRawFd for TcpStream {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for TcpStream {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.get_ref().as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<OwnedFd> for TcpStream {
    type Error = io::Error;

    fn try_from(value: OwnedFd) -> Result<Self, Self::Error> {
        Self::try_from(std::net::TcpStream::from(value))
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpStream {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for TcpStream {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.inner.get_ref().as_socket()
    }
}

#[cfg(windows)]
impl TryFrom<OwnedSocket> for TcpStream {
    type Error = io::Error;

    fn try_from(value: OwnedSocket) -> Result<Self, Self::Error> {
        Self::try_from(std::net::TcpStream::from(value))
    }
}

impl AsyncRead for TcpStream {
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

impl AsyncWrite for TcpStream {
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

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        loop {
            // Attempt the non-blocking operation.
            match self.inner.get_ref().write_vectored(bufs) {
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
}
