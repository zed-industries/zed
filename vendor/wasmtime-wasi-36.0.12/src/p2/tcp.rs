use crate::p2::bindings::sockets::tcp::ErrorCode;
use crate::p2::{
    DynInputStream, DynOutputStream, InputStream, OutputStream, Pollable, SocketError,
    SocketResult, StreamError,
};
use crate::runtime::{AbortOnDropJoinHandle, with_ambient_tokio_runtime};
use crate::sockets::util::{
    get_unicast_hop_limit, is_valid_address_family, is_valid_remote_address,
    is_valid_unicast_address, receive_buffer_size, send_buffer_size, set_keep_alive_count,
    set_keep_alive_idle_time, set_keep_alive_interval, set_receive_buffer_size,
    set_send_buffer_size, set_unicast_hop_limit, tcp_bind,
};
use crate::sockets::{DEFAULT_TCP_BACKLOG, SocketAddressFamily};
use anyhow::Result;
use cap_net_ext::AddressFamily;
use futures::Future;
use io_lifetimes::AsSocketlike;
use io_lifetimes::views::SocketlikeView;
use rustix::io::Errno;
use rustix::net::sockopt;
use std::io;
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Poll, Waker};
use tokio::sync::Mutex;

/// The state of a TCP socket.
///
/// This represents the various states a socket can be in during the
/// activities of binding, listening, accepting, and connecting.
enum TcpState {
    /// The initial state for a newly-created socket.
    Default(tokio::net::TcpSocket),

    /// Binding started via `start_bind`.
    BindStarted(tokio::net::TcpSocket),

    /// Binding finished via `finish_bind`. The socket has an address but
    /// is not yet listening for connections.
    Bound(tokio::net::TcpSocket),

    /// Listening started via `listen_start`.
    ListenStarted(tokio::net::TcpSocket),

    /// The socket is now listening and waiting for an incoming connection.
    Listening {
        listener: tokio::net::TcpListener,
        pending_accept: Option<io::Result<tokio::net::TcpStream>>,
    },

    /// An outgoing connection is started via `start_connect`.
    Connecting(Pin<Box<dyn Future<Output = io::Result<tokio::net::TcpStream>> + Send>>),

    /// An outgoing connection is ready to be established.
    ConnectReady(io::Result<tokio::net::TcpStream>),

    /// An outgoing connection has been established.
    Connected {
        stream: Arc<tokio::net::TcpStream>,

        // WASI is single threaded, so in practice these Mutexes should never be contended:
        reader: Arc<Mutex<TcpReader>>,
        writer: Arc<Mutex<TcpWriter>>,
    },

    Closed,
}

impl std::fmt::Debug for TcpState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(_) => f.debug_tuple("Default").finish(),
            Self::BindStarted(_) => f.debug_tuple("BindStarted").finish(),
            Self::Bound(_) => f.debug_tuple("Bound").finish(),
            Self::ListenStarted(_) => f.debug_tuple("ListenStarted").finish(),
            Self::Listening { pending_accept, .. } => f
                .debug_struct("Listening")
                .field("pending_accept", pending_accept)
                .finish(),
            Self::Connecting(_) => f.debug_tuple("Connecting").finish(),
            Self::ConnectReady(_) => f.debug_tuple("ConnectReady").finish(),
            Self::Connected { .. } => f.debug_tuple("Connected").finish(),
            Self::Closed => write!(f, "Closed"),
        }
    }
}

/// A host TCP socket, plus associated bookkeeping.
pub struct TcpSocket {
    /// The current state in the bind/listen/accept/connect progression.
    tcp_state: TcpState,

    /// The desired listen queue size.
    listen_backlog_size: u32,

    family: SocketAddressFamily,

    // The socket options below are not automatically inherited from the listener
    // on all platforms. So we keep track of which options have been explicitly
    // set and manually apply those values to newly accepted clients.
    #[cfg(target_os = "macos")]
    receive_buffer_size: Option<u64>,
    #[cfg(target_os = "macos")]
    send_buffer_size: Option<u64>,
    #[cfg(target_os = "macos")]
    hop_limit: Option<u8>,
    #[cfg(target_os = "macos")]
    keep_alive_idle_time: Option<u64>,
}

impl TcpSocket {
    /// Create a new socket in the given family.
    pub fn new(family: AddressFamily) -> io::Result<Self> {
        with_ambient_tokio_runtime(|| {
            let (socket, family) = match family {
                AddressFamily::Ipv4 => {
                    let socket = tokio::net::TcpSocket::new_v4()?;
                    (socket, SocketAddressFamily::Ipv4)
                }
                AddressFamily::Ipv6 => {
                    let socket = tokio::net::TcpSocket::new_v6()?;
                    sockopt::set_ipv6_v6only(&socket, true)?;
                    (socket, SocketAddressFamily::Ipv6)
                }
            };

            Self::from_state(TcpState::Default(socket), family)
        })
    }

    /// Create a `TcpSocket` from an existing socket.
    fn from_state(state: TcpState, family: SocketAddressFamily) -> io::Result<Self> {
        Ok(Self {
            tcp_state: state,
            listen_backlog_size: DEFAULT_TCP_BACKLOG,
            family,
            #[cfg(target_os = "macos")]
            receive_buffer_size: None,
            #[cfg(target_os = "macos")]
            send_buffer_size: None,
            #[cfg(target_os = "macos")]
            hop_limit: None,
            #[cfg(target_os = "macos")]
            keep_alive_idle_time: None,
        })
    }

    fn as_std_view(&self) -> SocketResult<SocketlikeView<'_, std::net::TcpStream>> {
        use crate::p2::bindings::sockets::network::ErrorCode;

        match &self.tcp_state {
            TcpState::Default(socket) | TcpState::Bound(socket) => {
                Ok(socket.as_socketlike_view::<std::net::TcpStream>())
            }
            TcpState::Connected { stream, .. } => {
                Ok(stream.as_socketlike_view::<std::net::TcpStream>())
            }
            TcpState::Listening { listener, .. } => {
                Ok(listener.as_socketlike_view::<std::net::TcpStream>())
            }

            TcpState::BindStarted(..)
            | TcpState::ListenStarted(..)
            | TcpState::Connecting(..)
            | TcpState::ConnectReady(..)
            | TcpState::Closed => Err(ErrorCode::InvalidState.into()),
        }
    }
}

impl TcpSocket {
    pub fn start_bind(&mut self, local_address: SocketAddr) -> Result<(), ErrorCode> {
        let tokio_socket = match &self.tcp_state {
            TcpState::Default(socket) => socket,
            TcpState::BindStarted(..) => return Err(Errno::ALREADY.into()),
            _ => return Err(ErrorCode::InvalidState),
        };

        if !is_valid_unicast_address(local_address.ip())
            || !is_valid_address_family(local_address.ip(), self.family)
        {
            return Err(ErrorCode::InvalidArgument);
        };

        {
            tcp_bind(&tokio_socket, local_address)?;

            self.tcp_state = match std::mem::replace(&mut self.tcp_state, TcpState::Closed) {
                TcpState::Default(socket) => TcpState::BindStarted(socket),
                _ => unreachable!(),
            };

            Ok(())
        }
    }

    pub fn finish_bind(&mut self) -> SocketResult<()> {
        match std::mem::replace(&mut self.tcp_state, TcpState::Closed) {
            TcpState::BindStarted(socket) => {
                self.tcp_state = TcpState::Bound(socket);
                Ok(())
            }
            current_state => {
                // Reset the state so that the outside world doesn't see this socket as closed
                self.tcp_state = current_state;
                Err(ErrorCode::NotInProgress.into())
            }
        }
    }

    pub fn start_connect(&mut self, remote_address: SocketAddr) -> SocketResult<()> {
        match self.tcp_state {
            TcpState::Default(..) | TcpState::Bound(..) => {}

            TcpState::Connecting(..) | TcpState::ConnectReady(..) => {
                return Err(ErrorCode::ConcurrencyConflict.into());
            }

            _ => return Err(ErrorCode::InvalidState.into()),
        };

        if !is_valid_unicast_address(remote_address.ip())
            || !is_valid_remote_address(remote_address)
            || !is_valid_address_family(remote_address.ip(), self.family)
        {
            return Err(ErrorCode::InvalidArgument.into());
        };

        let (TcpState::Default(tokio_socket) | TcpState::Bound(tokio_socket)) =
            std::mem::replace(&mut self.tcp_state, TcpState::Closed)
        else {
            unreachable!();
        };

        let future = tokio_socket.connect(remote_address);

        self.tcp_state = TcpState::Connecting(Box::pin(future));
        Ok(())
    }

    pub fn finish_connect(&mut self) -> SocketResult<(DynInputStream, DynOutputStream)> {
        let previous_state = std::mem::replace(&mut self.tcp_state, TcpState::Closed);
        let result = match previous_state {
            TcpState::ConnectReady(result) => result,
            TcpState::Connecting(mut future) => {
                let mut cx = std::task::Context::from_waker(Waker::noop());
                match with_ambient_tokio_runtime(|| future.as_mut().poll(&mut cx)) {
                    Poll::Ready(result) => result,
                    Poll::Pending => {
                        self.tcp_state = TcpState::Connecting(future);
                        return Err(ErrorCode::WouldBlock.into());
                    }
                }
            }
            previous_state => {
                self.tcp_state = previous_state;
                return Err(ErrorCode::NotInProgress.into());
            }
        };

        match result {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let reader = Arc::new(Mutex::new(TcpReader::new(stream.clone())));
                let writer = Arc::new(Mutex::new(TcpWriter::new(stream.clone())));
                self.tcp_state = TcpState::Connected {
                    stream,
                    reader: reader.clone(),
                    writer: writer.clone(),
                };
                let input: DynInputStream = Box::new(TcpReadStream(reader));
                let output: DynOutputStream = Box::new(TcpWriteStream(writer));
                Ok((input, output))
            }
            Err(err) => {
                self.tcp_state = TcpState::Closed;
                Err(err.into())
            }
        }
    }

    pub fn start_listen(&mut self) -> SocketResult<()> {
        match std::mem::replace(&mut self.tcp_state, TcpState::Closed) {
            TcpState::Bound(tokio_socket) => {
                self.tcp_state = TcpState::ListenStarted(tokio_socket);
                Ok(())
            }
            TcpState::ListenStarted(tokio_socket) => {
                self.tcp_state = TcpState::ListenStarted(tokio_socket);
                Err(ErrorCode::ConcurrencyConflict.into())
            }
            previous_state => {
                self.tcp_state = previous_state;
                Err(ErrorCode::InvalidState.into())
            }
        }
    }

    pub fn finish_listen(&mut self) -> SocketResult<()> {
        let tokio_socket = match std::mem::replace(&mut self.tcp_state, TcpState::Closed) {
            TcpState::ListenStarted(tokio_socket) => tokio_socket,
            previous_state => {
                self.tcp_state = previous_state;
                return Err(ErrorCode::NotInProgress.into());
            }
        };

        match with_ambient_tokio_runtime(|| tokio_socket.listen(self.listen_backlog_size)) {
            Ok(listener) => {
                self.tcp_state = TcpState::Listening {
                    listener,
                    pending_accept: None,
                };
                Ok(())
            }
            Err(err) => {
                self.tcp_state = TcpState::Closed;

                Err(match Errno::from_io_error(&err) {
                    // See: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen#:~:text=WSAEMFILE
                    // According to the docs, `listen` can return EMFILE on Windows.
                    // This is odd, because we're not trying to create a new socket
                    // or file descriptor of any kind. So we rewrite it to less
                    // surprising error code.
                    //
                    // At the time of writing, this behavior has never been experimentally
                    // observed by any of the wasmtime authors, so we're relying fully
                    // on Microsoft's documentation here.
                    #[cfg(windows)]
                    Some(Errno::MFILE) => Errno::NOBUFS.into(),

                    _ => err.into(),
                })
            }
        }
    }

    pub fn accept(&mut self) -> SocketResult<(Self, DynInputStream, DynOutputStream)> {
        let TcpState::Listening {
            listener,
            pending_accept,
        } = &mut self.tcp_state
        else {
            return Err(ErrorCode::InvalidState.into());
        };

        let result = match pending_accept.take() {
            Some(result) => result,
            None => {
                let mut cx = std::task::Context::from_waker(Waker::noop());
                match with_ambient_tokio_runtime(|| listener.poll_accept(&mut cx))
                    .map_ok(|(stream, _)| stream)
                {
                    Poll::Ready(result) => result,
                    Poll::Pending => Err(Errno::WOULDBLOCK.into()),
                }
            }
        };

        let client = result.map_err(|err| match Errno::from_io_error(&err) {
            // From: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept#:~:text=WSAEINPROGRESS
            // > WSAEINPROGRESS: A blocking Windows Sockets 1.1 call is in progress,
            // > or the service provider is still processing a callback function.
            //
            // wasi-sockets doesn't have an equivalent to the EINPROGRESS error,
            // because in POSIX this error is only returned by a non-blocking
            // `connect` and wasi-sockets has a different solution for that.
            #[cfg(windows)]
            Some(Errno::INPROGRESS) => Errno::INTR.into(),

            // Normalize Linux' non-standard behavior.
            //
            // From https://man7.org/linux/man-pages/man2/accept.2.html:
            // > Linux accept() passes already-pending network errors on the
            // > new socket as an error code from accept(). This behavior
            // > differs from other BSD socket implementations. (...)
            #[cfg(target_os = "linux")]
            Some(
                Errno::CONNRESET
                | Errno::NETRESET
                | Errno::HOSTUNREACH
                | Errno::HOSTDOWN
                | Errno::NETDOWN
                | Errno::NETUNREACH
                | Errno::PROTO
                | Errno::NOPROTOOPT
                | Errno::NONET
                | Errno::OPNOTSUPP,
            ) => Errno::CONNABORTED.into(),

            _ => err,
        })?;

        #[cfg(target_os = "macos")]
        {
            // Manually inherit socket options from listener. We only have to
            // do this on platforms that don't already do this automatically
            // and only if a specific value was explicitly set on the listener.

            if let Some(size) = self.receive_buffer_size {
                _ = set_receive_buffer_size(&client, size); // Ignore potential error.
            }

            if let Some(size) = self.send_buffer_size {
                _ = set_send_buffer_size(&client, size); // Ignore potential error.
            }

            // For some reason, IP_TTL is inherited, but IPV6_UNICAST_HOPS isn't.
            if let (SocketAddressFamily::Ipv6, Some(ttl)) = (self.family, self.hop_limit) {
                _ = rustix::net::sockopt::set_ipv6_unicast_hops(&client, Some(ttl)); // Ignore potential error.
            }

            if let Some(value) = self.keep_alive_idle_time {
                _ = set_keep_alive_idle_time(&client, value); // Ignore potential error.
            }
        }

        let client = Arc::new(client);

        let reader = Arc::new(Mutex::new(TcpReader::new(client.clone())));
        let writer = Arc::new(Mutex::new(TcpWriter::new(client.clone())));

        let input: DynInputStream = Box::new(TcpReadStream(reader.clone()));
        let output: DynOutputStream = Box::new(TcpWriteStream(writer.clone()));
        let tcp_socket = TcpSocket::from_state(
            TcpState::Connected {
                stream: client,
                reader,
                writer,
            },
            self.family,
        )?;

        Ok((tcp_socket, input, output))
    }

    pub fn local_address(&self) -> SocketResult<SocketAddr> {
        let view = match self.tcp_state {
            TcpState::Default(..) => return Err(ErrorCode::InvalidState.into()),
            TcpState::BindStarted(..) => return Err(ErrorCode::ConcurrencyConflict.into()),
            _ => self.as_std_view()?,
        };

        Ok(view.local_addr()?)
    }

    pub fn remote_address(&self) -> SocketResult<SocketAddr> {
        let view = match self.tcp_state {
            TcpState::Connected { .. } => self.as_std_view()?,
            TcpState::Connecting(..) | TcpState::ConnectReady(..) => {
                return Err(ErrorCode::ConcurrencyConflict.into());
            }
            _ => return Err(ErrorCode::InvalidState.into()),
        };

        Ok(view.peer_addr()?)
    }

    pub fn is_listening(&self) -> bool {
        matches!(self.tcp_state, TcpState::Listening { .. })
    }

    pub fn address_family(&self) -> SocketAddressFamily {
        self.family
    }

    pub fn set_listen_backlog_size(&mut self, value: u32) -> SocketResult<()> {
        const MIN_BACKLOG: u32 = 1;
        const MAX_BACKLOG: u32 = i32::MAX as u32; // OS'es will most likely limit it down even further.

        if value == 0 {
            return Err(ErrorCode::InvalidArgument.into());
        }

        // Silently clamp backlog size. This is OK for us to do, because operating systems do this too.
        let value = value.clamp(MIN_BACKLOG, MAX_BACKLOG);

        match &self.tcp_state {
            TcpState::Default(..) | TcpState::Bound(..) => {
                // Socket not listening yet. Stash value for first invocation to `listen`.
            }
            TcpState::Listening { listener, .. } => {
                // Try to update the backlog by calling `listen` again.
                // Not all platforms support this. We'll only update our own value if the OS supports changing the backlog size after the fact.

                rustix::net::listen(&listener, value.try_into().unwrap())
                    .map_err(|_| ErrorCode::NotSupported)?;
            }
            _ => return Err(ErrorCode::InvalidState.into()),
        }
        self.listen_backlog_size = value;

        Ok(())
    }

    pub fn keep_alive_enabled(&self) -> SocketResult<bool> {
        let view = &*self.as_std_view()?;
        Ok(sockopt::socket_keepalive(view)?)
    }

    pub fn set_keep_alive_enabled(&self, value: bool) -> SocketResult<()> {
        let view = &*self.as_std_view()?;
        Ok(sockopt::set_socket_keepalive(view, value)?)
    }

    pub fn keep_alive_idle_time(&self) -> SocketResult<std::time::Duration> {
        let view = &*self.as_std_view()?;
        Ok(sockopt::tcp_keepidle(view)?)
    }

    pub fn set_keep_alive_idle_time(&mut self, value: u64) -> SocketResult<()> {
        {
            let view = &*self.as_std_view()?;
            set_keep_alive_idle_time(view, value)?;
        }

        #[cfg(target_os = "macos")]
        {
            self.keep_alive_idle_time = Some(value);
        }

        Ok(())
    }

    pub fn keep_alive_interval(&self) -> SocketResult<std::time::Duration> {
        let view = &*self.as_std_view()?;
        Ok(sockopt::tcp_keepintvl(view)?)
    }

    pub fn set_keep_alive_interval(&self, duration: std::time::Duration) -> SocketResult<()> {
        let view = &*self.as_std_view()?;
        Ok(set_keep_alive_interval(view, duration)?)
    }

    pub fn keep_alive_count(&self) -> SocketResult<u32> {
        let view = &*self.as_std_view()?;
        Ok(sockopt::tcp_keepcnt(view)?)
    }

    pub fn set_keep_alive_count(&self, value: u32) -> SocketResult<()> {
        let view = &*self.as_std_view()?;
        Ok(set_keep_alive_count(view, value)?)
    }

    pub fn hop_limit(&self) -> SocketResult<u8> {
        let view = &*self.as_std_view()?;

        let ttl = get_unicast_hop_limit(view, self.family)?;
        Ok(ttl)
    }

    pub fn set_hop_limit(&mut self, value: u8) -> SocketResult<()> {
        {
            let view = &*self.as_std_view()?;

            set_unicast_hop_limit(view, self.family, value)?;
        }

        #[cfg(target_os = "macos")]
        {
            self.hop_limit = Some(value);
        }

        Ok(())
    }

    pub fn receive_buffer_size(&self) -> SocketResult<u64> {
        let view = &*self.as_std_view()?;

        Ok(receive_buffer_size(view)?)
    }

    pub fn set_receive_buffer_size(&mut self, value: u64) -> SocketResult<()> {
        {
            let view = &*self.as_std_view()?;

            set_receive_buffer_size(view, value)?;
        }

        #[cfg(target_os = "macos")]
        {
            self.receive_buffer_size = Some(value);
        }

        Ok(())
    }

    pub fn send_buffer_size(&self) -> SocketResult<u64> {
        let view = &*self.as_std_view()?;

        Ok(send_buffer_size(view)?)
    }

    pub fn set_send_buffer_size(&mut self, value: u64) -> SocketResult<()> {
        {
            let view = &*self.as_std_view()?;

            set_send_buffer_size(view, value)?;
        }

        #[cfg(target_os = "macos")]
        {
            self.send_buffer_size = Some(value);
        }

        Ok(())
    }

    pub fn shutdown(&self, how: Shutdown) -> SocketResult<()> {
        let TcpState::Connected { reader, writer, .. } = &self.tcp_state else {
            return Err(ErrorCode::InvalidState.into());
        };

        if let Shutdown::Both | Shutdown::Read = how {
            try_lock_for_socket(reader)?.shutdown();
        }

        if let Shutdown::Both | Shutdown::Write = how {
            try_lock_for_socket(writer)?.shutdown();
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Pollable for TcpSocket {
    async fn ready(&mut self) {
        match &mut self.tcp_state {
            TcpState::Default(..)
            | TcpState::BindStarted(..)
            | TcpState::Bound(..)
            | TcpState::ListenStarted(..)
            | TcpState::ConnectReady(..)
            | TcpState::Closed
            | TcpState::Connected { .. } => {
                // No async operation in progress.
            }
            TcpState::Connecting(future) => {
                self.tcp_state = TcpState::ConnectReady(future.as_mut().await);
            }
            TcpState::Listening {
                listener,
                pending_accept,
            } => match pending_accept {
                Some(_) => {}
                None => {
                    let result = futures::future::poll_fn(|cx| {
                        listener.poll_accept(cx).map_ok(|(stream, _)| stream)
                    })
                    .await;
                    *pending_accept = Some(result);
                }
            },
        }
    }
}

struct TcpReader {
    stream: Arc<tokio::net::TcpStream>,
    closed: bool,
}

impl TcpReader {
    fn new(stream: Arc<tokio::net::TcpStream>) -> Self {
        Self {
            stream,
            closed: false,
        }
    }
    fn read(&mut self, size: usize) -> Result<bytes::Bytes, StreamError> {
        if self.closed {
            return Err(StreamError::Closed);
        }
        if size == 0 {
            return Ok(bytes::Bytes::new());
        }

        let mut buf = bytes::BytesMut::with_capacity(size.min(crate::MAX_READ_SIZE_ALLOC));
        let n = match self.stream.try_read_buf(&mut buf) {
            // A 0-byte read indicates that the stream has closed.
            Ok(0) => {
                self.closed = true;
                return Err(StreamError::Closed);
            }
            Ok(n) => n,

            // Failing with `EWOULDBLOCK` is how we differentiate between a closed channel and no
            // data to read right now.
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => 0,

            Err(e) => {
                self.closed = true;
                return Err(StreamError::LastOperationFailed(e.into()));
            }
        };

        buf.truncate(n);
        Ok(buf.freeze())
    }

    fn shutdown(&mut self) {
        native_shutdown(&self.stream, Shutdown::Read);
        self.closed = true;
    }

    async fn ready(&mut self) {
        if self.closed {
            return;
        }

        self.stream.readable().await.unwrap();
    }
}

struct TcpReadStream(Arc<Mutex<TcpReader>>);

#[async_trait::async_trait]
impl InputStream for TcpReadStream {
    fn read(&mut self, size: usize) -> Result<bytes::Bytes, StreamError> {
        try_lock_for_stream(&self.0)?.read(size)
    }
}

#[async_trait::async_trait]
impl Pollable for TcpReadStream {
    async fn ready(&mut self) {
        self.0.lock().await.ready().await
    }
}

const SOCKET_READY_SIZE: usize = 1024 * 1024 * 1024;

struct TcpWriter {
    stream: Arc<tokio::net::TcpStream>,
    state: WriteState,
}

enum WriteState {
    Ready,
    Writing(AbortOnDropJoinHandle<io::Result<()>>),
    Closing(AbortOnDropJoinHandle<io::Result<()>>),
    Closed,
    Error(io::Error),
}

impl TcpWriter {
    fn new(stream: Arc<tokio::net::TcpStream>) -> Self {
        Self {
            stream,
            state: WriteState::Ready,
        }
    }

    fn try_write_portable(stream: &tokio::net::TcpStream, buf: &[u8]) -> io::Result<usize> {
        stream.try_write(buf).map_err(|error| {
            match Errno::from_io_error(&error) {
                // Windows returns `WSAESHUTDOWN` when writing to a shut down socket.
                // We normalize this to EPIPE, because that is what the other platforms return.
                // See: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send#:~:text=WSAESHUTDOWN
                #[cfg(windows)]
                Some(Errno::SHUTDOWN) => io::Error::new(io::ErrorKind::BrokenPipe, error),

                _ => error,
            }
        })
    }

    /// Write `bytes` in a background task, remembering the task handle for use in a future call to
    /// `write_ready`
    fn background_write(&mut self, mut bytes: bytes::Bytes) {
        assert!(matches!(self.state, WriteState::Ready));

        let stream = self.stream.clone();
        self.state = WriteState::Writing(crate::runtime::spawn(async move {
            // Note: we are not using the AsyncWrite impl here, and instead using the TcpStream
            // primitive try_write, which goes directly to attempt a write with mio. This has
            // two advantages: 1. this operation takes a &TcpStream instead of a &mut TcpStream
            // required to AsyncWrite, and 2. it eliminates any buffering in tokio we may need
            // to flush.
            while !bytes.is_empty() {
                stream.writable().await?;
                match Self::try_write_portable(&stream, &bytes) {
                    Ok(n) => {
                        let _ = bytes.split_to(n);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                    Err(e) => return Err(e),
                }
            }

            Ok(())
        }));
    }

    fn write(&mut self, mut bytes: bytes::Bytes) -> Result<(), StreamError> {
        match self.state {
            WriteState::Ready => {}
            WriteState::Closed => return Err(StreamError::Closed),
            WriteState::Writing(_) | WriteState::Closing(_) | WriteState::Error(_) => {
                return Err(StreamError::Trap(anyhow::anyhow!(
                    "unpermitted: must call check_write first"
                )));
            }
        }
        while !bytes.is_empty() {
            match Self::try_write_portable(&self.stream, &bytes) {
                Ok(n) => {
                    let _ = bytes.split_to(n);
                }

                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // As `try_write` indicated that it would have blocked, we'll perform the write
                    // in the background to allow us to return immediately.
                    self.background_write(bytes);

                    return Ok(());
                }

                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                    self.state = WriteState::Closed;
                    return Err(StreamError::Closed);
                }

                Err(e) => return Err(StreamError::LastOperationFailed(e.into())),
            }
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), StreamError> {
        // `flush` is a no-op here, as we're not managing any internal buffer. Additionally,
        // `write_ready` will join the background write task if it's active, so following `flush`
        // with `write_ready` will have the desired effect.
        match self.state {
            WriteState::Ready
            | WriteState::Writing(_)
            | WriteState::Closing(_)
            | WriteState::Error(_) => Ok(()),
            WriteState::Closed => Err(StreamError::Closed),
        }
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        match mem::replace(&mut self.state, WriteState::Closed) {
            WriteState::Writing(task) => {
                self.state = WriteState::Writing(task);
                return Ok(0);
            }
            WriteState::Closing(task) => {
                self.state = WriteState::Closing(task);
                return Ok(0);
            }
            WriteState::Ready => {
                self.state = WriteState::Ready;
            }
            WriteState::Closed => return Err(StreamError::Closed),
            WriteState::Error(e) => return Err(StreamError::LastOperationFailed(e.into())),
        }

        let writable = self.stream.writable();
        futures::pin_mut!(writable);
        if crate::runtime::poll_noop(writable).is_none() {
            return Ok(0);
        }
        Ok(SOCKET_READY_SIZE)
    }

    fn shutdown(&mut self) {
        self.state = match mem::replace(&mut self.state, WriteState::Closed) {
            // No write in progress, immediately shut down:
            WriteState::Ready => {
                native_shutdown(&self.stream, Shutdown::Write);
                WriteState::Closed
            }

            // Schedule the shutdown after the current write has finished:
            WriteState::Writing(write) => {
                let stream = self.stream.clone();
                WriteState::Closing(crate::runtime::spawn(async move {
                    let result = write.await;
                    native_shutdown(&stream, Shutdown::Write);
                    result
                }))
            }

            s => s,
        };
    }

    async fn cancel(&mut self) {
        match mem::replace(&mut self.state, WriteState::Closed) {
            WriteState::Writing(task) | WriteState::Closing(task) => _ = task.cancel().await,
            _ => {}
        }
    }

    async fn ready(&mut self) {
        match &mut self.state {
            WriteState::Writing(task) => {
                self.state = match task.await {
                    Ok(()) => WriteState::Ready,
                    Err(e) => WriteState::Error(e),
                }
            }
            WriteState::Closing(task) => {
                self.state = match task.await {
                    Ok(()) => WriteState::Closed,
                    Err(e) => WriteState::Error(e),
                }
            }
            _ => {}
        }

        if let WriteState::Ready = self.state {
            self.stream.writable().await.unwrap();
        }
    }
}

struct TcpWriteStream(Arc<Mutex<TcpWriter>>);

#[async_trait::async_trait]
impl OutputStream for TcpWriteStream {
    fn write(&mut self, bytes: bytes::Bytes) -> Result<(), StreamError> {
        try_lock_for_stream(&self.0)?.write(bytes)
    }

    fn flush(&mut self) -> Result<(), StreamError> {
        try_lock_for_stream(&self.0)?.flush()
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        try_lock_for_stream(&self.0)?.check_write()
    }

    async fn cancel(&mut self) {
        self.0.lock().await.cancel().await
    }
}

#[async_trait::async_trait]
impl Pollable for TcpWriteStream {
    async fn ready(&mut self) {
        self.0.lock().await.ready().await
    }
}

fn native_shutdown(stream: &tokio::net::TcpStream, how: Shutdown) {
    _ = stream
        .as_socketlike_view::<std::net::TcpStream>()
        .shutdown(how);
}

fn try_lock_for_stream<T>(mutex: &Mutex<T>) -> Result<tokio::sync::MutexGuard<'_, T>, StreamError> {
    mutex
        .try_lock()
        .map_err(|_| StreamError::trap("concurrent access to resource not supported"))
}

fn try_lock_for_socket<T>(mutex: &Mutex<T>) -> Result<tokio::sync::MutexGuard<'_, T>, SocketError> {
    mutex.try_lock().map_err(|_| {
        SocketError::trap(anyhow::anyhow!(
            "concurrent access to resource not supported"
        ))
    })
}
