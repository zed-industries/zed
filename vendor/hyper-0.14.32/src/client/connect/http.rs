use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::io;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{self, Poll};
use std::time::Duration;

use futures_util::future::Either;
use http::uri::{Scheme, Uri};
use pin_project_lite::pin_project;
use tokio::net::{TcpSocket, TcpStream};
use tokio::time::Sleep;
use tracing::{debug, trace, warn};

use super::dns::{self, resolve, GaiResolver, Resolve};
use super::{Connected, Connection};
//#[cfg(feature = "runtime")] use super::dns::TokioThreadpoolGaiResolver;

/// A connector for the `http` scheme.
///
/// Performs DNS resolution in a thread pool, and then connects over TCP.
///
/// # Note
///
/// Sets the [`HttpInfo`](HttpInfo) value on responses, which includes
/// transport information such as the remote socket address used.
#[cfg_attr(docsrs, doc(cfg(feature = "tcp")))]
#[derive(Clone)]
pub struct HttpConnector<R = GaiResolver> {
    config: Arc<Config>,
    resolver: R,
}

/// Extra information about the transport when an HttpConnector is used.
///
/// # Example
///
/// ```
/// # async fn doc() -> hyper::Result<()> {
/// use hyper::Uri;
/// use hyper::client::{Client, connect::HttpInfo};
///
/// let client = Client::new();
/// let uri = Uri::from_static("http://example.com");
///
/// let res = client.get(uri).await?;
/// res
///     .extensions()
///     .get::<HttpInfo>()
///     .map(|info| {
///         println!("remote addr = {}", info.remote_addr());
///     });
/// # Ok(())
/// # }
/// ```
///
/// # Note
///
/// If a different connector is used besides [`HttpConnector`](HttpConnector),
/// this value will not exist in the extensions. Consult that specific
/// connector to see what "extra" information it might provide to responses.
#[derive(Clone, Debug)]
pub struct HttpInfo {
    remote_addr: SocketAddr,
    local_addr: SocketAddr,
}

#[derive(Clone)]
struct Config {
    connect_timeout: Option<Duration>,
    enforce_http: bool,
    happy_eyeballs_timeout: Option<Duration>,
    keep_alive_timeout: Option<Duration>,
    local_address_ipv4: Option<Ipv4Addr>,
    local_address_ipv6: Option<Ipv6Addr>,
    nodelay: bool,
    reuse_address: bool,
    send_buffer_size: Option<usize>,
    recv_buffer_size: Option<usize>,
}

// ===== impl HttpConnector =====

impl HttpConnector {
    /// Construct a new HttpConnector.
    pub fn new() -> HttpConnector {
        HttpConnector::new_with_resolver(GaiResolver::new())
    }
}

/*
#[cfg(feature = "runtime")]
impl HttpConnector<TokioThreadpoolGaiResolver> {
    /// Construct a new HttpConnector using the `TokioThreadpoolGaiResolver`.
    ///
    /// This resolver **requires** the threadpool runtime to be used.
    pub fn new_with_tokio_threadpool_resolver() -> Self {
        HttpConnector::new_with_resolver(TokioThreadpoolGaiResolver::new())
    }
}
*/

impl<R> HttpConnector<R> {
    /// Construct a new HttpConnector.
    ///
    /// Takes a [`Resolver`](crate::client::connect::dns#resolvers-are-services) to handle DNS lookups.
    pub fn new_with_resolver(resolver: R) -> HttpConnector<R> {
        HttpConnector {
            config: Arc::new(Config {
                connect_timeout: None,
                enforce_http: true,
                happy_eyeballs_timeout: Some(Duration::from_millis(300)),
                keep_alive_timeout: None,
                local_address_ipv4: None,
                local_address_ipv6: None,
                nodelay: false,
                reuse_address: false,
                send_buffer_size: None,
                recv_buffer_size: None,
            }),
            resolver,
        }
    }

    /// Option to enforce all `Uri`s have the `http` scheme.
    ///
    /// Enabled by default.
    #[inline]
    pub fn enforce_http(&mut self, is_enforced: bool) {
        self.config_mut().enforce_http = is_enforced;
    }

    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration.
    ///
    /// If `None`, the option will not be set.
    ///
    /// Default is `None`.
    #[inline]
    pub fn set_keepalive(&mut self, dur: Option<Duration>) {
        self.config_mut().keep_alive_timeout = dur;
    }

    /// Set that all sockets have `SO_NODELAY` set to the supplied value `nodelay`.
    ///
    /// Default is `false`.
    #[inline]
    pub fn set_nodelay(&mut self, nodelay: bool) {
        self.config_mut().nodelay = nodelay;
    }

    /// Sets the value of the SO_SNDBUF option on the socket.
    #[inline]
    pub fn set_send_buffer_size(&mut self, size: Option<usize>) {
        self.config_mut().send_buffer_size = size;
    }

    /// Sets the value of the SO_RCVBUF option on the socket.
    #[inline]
    pub fn set_recv_buffer_size(&mut self, size: Option<usize>) {
        self.config_mut().recv_buffer_size = size;
    }

    /// Set that all sockets are bound to the configured address before connection.
    ///
    /// If `None`, the sockets will not be bound.
    ///
    /// Default is `None`.
    #[inline]
    pub fn set_local_address(&mut self, addr: Option<IpAddr>) {
        let (v4, v6) = match addr {
            Some(IpAddr::V4(a)) => (Some(a), None),
            Some(IpAddr::V6(a)) => (None, Some(a)),
            _ => (None, None),
        };

        let cfg = self.config_mut();

        cfg.local_address_ipv4 = v4;
        cfg.local_address_ipv6 = v6;
    }

    /// Set that all sockets are bound to the configured IPv4 or IPv6 address (depending on host's
    /// preferences) before connection.
    #[inline]
    pub fn set_local_addresses(&mut self, addr_ipv4: Ipv4Addr, addr_ipv6: Ipv6Addr) {
        let cfg = self.config_mut();

        cfg.local_address_ipv4 = Some(addr_ipv4);
        cfg.local_address_ipv6 = Some(addr_ipv6);
    }

    /// Set the connect timeout.
    ///
    /// If a domain resolves to multiple IP addresses, the timeout will be
    /// evenly divided across them.
    ///
    /// Default is `None`.
    #[inline]
    pub fn set_connect_timeout(&mut self, dur: Option<Duration>) {
        self.config_mut().connect_timeout = dur;
    }

    /// Set timeout for [RFC 6555 (Happy Eyeballs)][RFC 6555] algorithm.
    ///
    /// If hostname resolves to both IPv4 and IPv6 addresses and connection
    /// cannot be established using preferred address family before timeout
    /// elapses, then connector will in parallel attempt connection using other
    /// address family.
    ///
    /// If `None`, parallel connection attempts are disabled.
    ///
    /// Default is 300 milliseconds.
    ///
    /// [RFC 6555]: https://tools.ietf.org/html/rfc6555
    #[inline]
    pub fn set_happy_eyeballs_timeout(&mut self, dur: Option<Duration>) {
        self.config_mut().happy_eyeballs_timeout = dur;
    }

    /// Set that all socket have `SO_REUSEADDR` set to the supplied value `reuse_address`.
    ///
    /// Default is `false`.
    #[inline]
    pub fn set_reuse_address(&mut self, reuse_address: bool) -> &mut Self {
        self.config_mut().reuse_address = reuse_address;
        self
    }

    // private

    fn config_mut(&mut self) -> &mut Config {
        // If the are HttpConnector clones, this will clone the inner
        // config. So mutating the config won't ever affect previous
        // clones.
        Arc::make_mut(&mut self.config)
    }
}

static INVALID_NOT_HTTP: &str = "invalid URL, scheme is not http";
static INVALID_MISSING_SCHEME: &str = "invalid URL, scheme is missing";
static INVALID_MISSING_HOST: &str = "invalid URL, host is missing";

// R: Debug required for now to allow adding it to debug output later...
impl<R: fmt::Debug> fmt::Debug for HttpConnector<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpConnector").finish()
    }
}

impl<R> tower_service::Service<Uri> for HttpConnector<R>
where
    R: Resolve + Clone + Send + Sync + 'static,
    R::Future: Send,
{
    type Response = TcpStream;
    type Error = ConnectError;
    type Future = HttpConnecting<R>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        ready!(self.resolver.poll_ready(cx)).map_err(ConnectError::dns)?;
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, dst: Uri) -> Self::Future {
        let mut self_ = self.clone();
        HttpConnecting {
            fut: Box::pin(async move { self_.call_async(dst).await }),
            _marker: PhantomData,
        }
    }
}

fn get_host_port<'u>(config: &Config, dst: &'u Uri) -> Result<(&'u str, u16), ConnectError> {
    trace!(
        "Http::connect; scheme={:?}, host={:?}, port={:?}",
        dst.scheme(),
        dst.host(),
        dst.port(),
    );

    if config.enforce_http {
        if dst.scheme() != Some(&Scheme::HTTP) {
            return Err(ConnectError {
                msg: INVALID_NOT_HTTP.into(),
                cause: None,
            });
        }
    } else if dst.scheme().is_none() {
        return Err(ConnectError {
            msg: INVALID_MISSING_SCHEME.into(),
            cause: None,
        });
    }

    let host = match dst.host() {
        Some(s) => s,
        None => {
            return Err(ConnectError {
                msg: INVALID_MISSING_HOST.into(),
                cause: None,
            })
        }
    };
    let port = match dst.port() {
        Some(port) => port.as_u16(),
        None => {
            if dst.scheme() == Some(&Scheme::HTTPS) {
                443
            } else {
                80
            }
        }
    };

    Ok((host, port))
}

impl<R> HttpConnector<R>
where
    R: Resolve,
{
    async fn call_async(&mut self, dst: Uri) -> Result<TcpStream, ConnectError> {
        let config = &self.config;

        let (host, port) = get_host_port(config, &dst)?;
        let host = host.trim_start_matches('[').trim_end_matches(']');

        // If the host is already an IP addr (v4 or v6),
        // skip resolving the dns and start connecting right away.
        let addrs = if let Some(addrs) = dns::SocketAddrs::try_parse(host, port) {
            addrs
        } else {
            let addrs = resolve(&mut self.resolver, dns::Name::new(host.into()))
                .await
                .map_err(ConnectError::dns)?;
            let addrs = addrs
                .map(|mut addr| {
                    addr.set_port(port);
                    addr
                })
                .collect();
            dns::SocketAddrs::new(addrs)
        };

        let c = ConnectingTcp::new(addrs, config);

        let sock = c.connect().await?;

        if let Err(e) = sock.set_nodelay(config.nodelay) {
            warn!("tcp set_nodelay error: {}", e);
        }

        Ok(sock)
    }
}

impl Connection for TcpStream {
    fn connected(&self) -> Connected {
        let connected = Connected::new();
        if let (Ok(remote_addr), Ok(local_addr)) = (self.peer_addr(), self.local_addr()) {
            connected.extra(HttpInfo {
                remote_addr,
                local_addr,
            })
        } else {
            connected
        }
    }
}

impl HttpInfo {
    /// Get the remote address of the transport used.
    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }

    /// Get the local address of the transport used.
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}

pin_project! {
    // Not publicly exported (so missing_docs doesn't trigger).
    //
    // We return this `Future` instead of the `Pin<Box<dyn Future>>` directly
    // so that users don't rely on it fitting in a `Pin<Box<dyn Future>>` slot
    // (and thus we can change the type in the future).
    #[must_use = "futures do nothing unless polled"]
    #[allow(missing_debug_implementations)]
    pub struct HttpConnecting<R> {
        #[pin]
        fut: BoxConnecting,
        _marker: PhantomData<R>,
    }
}

type ConnectResult = Result<TcpStream, ConnectError>;
type BoxConnecting = Pin<Box<dyn Future<Output = ConnectResult> + Send>>;

impl<R: Resolve> Future for HttpConnecting<R> {
    type Output = ConnectResult;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        self.project().fut.poll(cx)
    }
}

// Not publicly exported (so missing_docs doesn't trigger).
pub struct ConnectError {
    msg: Box<str>,
    cause: Option<Box<dyn StdError + Send + Sync>>,
}

impl ConnectError {
    fn new<S, E>(msg: S, cause: E) -> ConnectError
    where
        S: Into<Box<str>>,
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        ConnectError {
            msg: msg.into(),
            cause: Some(cause.into()),
        }
    }

    fn dns<E>(cause: E) -> ConnectError
    where
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        ConnectError::new("dns error", cause)
    }

    fn m<S, E>(msg: S) -> impl FnOnce(E) -> ConnectError
    where
        S: Into<Box<str>>,
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        move |cause| ConnectError::new(msg, cause)
    }
}

impl fmt::Debug for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.cause {
            f.debug_tuple("ConnectError")
                .field(&self.msg)
                .field(cause)
                .finish()
        } else {
            self.msg.fmt(f)
        }
    }
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)?;

        if let Some(ref cause) = self.cause {
            write!(f, ": {}", cause)?;
        }

        Ok(())
    }
}

impl StdError for ConnectError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.cause.as_ref().map(|e| &**e as _)
    }
}

struct ConnectingTcp<'a> {
    preferred: ConnectingTcpRemote,
    fallback: Option<ConnectingTcpFallback>,
    config: &'a Config,
}

impl<'a> ConnectingTcp<'a> {
    fn new(remote_addrs: dns::SocketAddrs, config: &'a Config) -> Self {
        if let Some(fallback_timeout) = config.happy_eyeballs_timeout {
            let (preferred_addrs, fallback_addrs) = remote_addrs
                .split_by_preference(config.local_address_ipv4, config.local_address_ipv6);
            if fallback_addrs.is_empty() {
                return ConnectingTcp {
                    preferred: ConnectingTcpRemote::new(preferred_addrs, config.connect_timeout),
                    fallback: None,
                    config,
                };
            }

            ConnectingTcp {
                preferred: ConnectingTcpRemote::new(preferred_addrs, config.connect_timeout),
                fallback: Some(ConnectingTcpFallback {
                    delay: tokio::time::sleep(fallback_timeout),
                    remote: ConnectingTcpRemote::new(fallback_addrs, config.connect_timeout),
                }),
                config,
            }
        } else {
            ConnectingTcp {
                preferred: ConnectingTcpRemote::new(remote_addrs, config.connect_timeout),
                fallback: None,
                config,
            }
        }
    }
}

struct ConnectingTcpFallback {
    delay: Sleep,
    remote: ConnectingTcpRemote,
}

struct ConnectingTcpRemote {
    addrs: dns::SocketAddrs,
    connect_timeout: Option<Duration>,
}

impl ConnectingTcpRemote {
    fn new(addrs: dns::SocketAddrs, connect_timeout: Option<Duration>) -> Self {
        let connect_timeout = connect_timeout
            .map(|t| t.checked_div(addrs.len() as u32))
            .flatten();

        Self {
            addrs,
            connect_timeout,
        }
    }
}

impl ConnectingTcpRemote {
    async fn connect(&mut self, config: &Config) -> Result<TcpStream, ConnectError> {
        let mut err = None;
        for addr in &mut self.addrs {
            debug!("connecting to {}", addr);
            match connect(&addr, config, self.connect_timeout)?.await {
                Ok(tcp) => {
                    debug!("connected to {}", addr);
                    return Ok(tcp);
                }
                Err(e) => {
                    trace!("connect error for {}: {:?}", addr, e);
                    err = Some(e);
                }
            }
        }

        match err {
            Some(e) => Err(e),
            None => Err(ConnectError::new(
                "tcp connect error",
                std::io::Error::new(std::io::ErrorKind::NotConnected, "Network unreachable"),
            )),
        }
    }
}

fn bind_local_address(
    socket: &socket2::Socket,
    dst_addr: &SocketAddr,
    local_addr_ipv4: &Option<Ipv4Addr>,
    local_addr_ipv6: &Option<Ipv6Addr>,
) -> io::Result<()> {
    match (*dst_addr, local_addr_ipv4, local_addr_ipv6) {
        (SocketAddr::V4(_), Some(addr), _) => {
            socket.bind(&SocketAddr::new(addr.clone().into(), 0).into())?;
        }
        (SocketAddr::V6(_), _, Some(addr)) => {
            socket.bind(&SocketAddr::new(addr.clone().into(), 0).into())?;
        }
        _ => {
            if cfg!(windows) {
                // Windows requires a socket be bound before calling connect
                let any: SocketAddr = match *dst_addr {
                    SocketAddr::V4(_) => ([0, 0, 0, 0], 0).into(),
                    SocketAddr::V6(_) => ([0, 0, 0, 0, 0, 0, 0, 0], 0).into(),
                };
                socket.bind(&any.into())?;
            }
        }
    }

    Ok(())
}

fn connect(
    addr: &SocketAddr,
    config: &Config,
    connect_timeout: Option<Duration>,
) -> Result<impl Future<Output = Result<TcpStream, ConnectError>>, ConnectError> {
    // TODO(eliza): if Tokio's `TcpSocket` gains support for setting the
    // keepalive timeout, it would be nice to use that instead of socket2,
    // and avoid the unsafe `into_raw_fd`/`from_raw_fd` dance...
    use socket2::{Domain, Protocol, Socket, TcpKeepalive, Type};
    use std::convert::TryInto;

    let domain = Domain::for_address(*addr);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))
        .map_err(ConnectError::m("tcp open error"))?;

    // When constructing a Tokio `TcpSocket` from a raw fd/socket, the user is
    // responsible for ensuring O_NONBLOCK is set.
    socket
        .set_nonblocking(true)
        .map_err(ConnectError::m("tcp set_nonblocking error"))?;

    if let Some(dur) = config.keep_alive_timeout {
        let conf = TcpKeepalive::new().with_time(dur);
        if let Err(e) = socket.set_tcp_keepalive(&conf) {
            warn!("tcp set_keepalive error: {}", e);
        }
    }

    bind_local_address(
        &socket,
        addr,
        &config.local_address_ipv4,
        &config.local_address_ipv6,
    )
    .map_err(ConnectError::m("tcp bind local error"))?;

    #[cfg(unix)]
    let socket = unsafe {
        // Safety: `from_raw_fd` is only safe to call if ownership of the raw
        // file descriptor is transferred. Since we call `into_raw_fd` on the
        // socket2 socket, it gives up ownership of the fd and will not close
        // it, so this is safe.
        use std::os::unix::io::{FromRawFd, IntoRawFd};
        TcpSocket::from_raw_fd(socket.into_raw_fd())
    };
    #[cfg(windows)]
    let socket = unsafe {
        // Safety: `from_raw_socket` is only safe to call if ownership of the raw
        // Windows SOCKET is transferred. Since we call `into_raw_socket` on the
        // socket2 socket, it gives up ownership of the SOCKET and will not close
        // it, so this is safe.
        use std::os::windows::io::{FromRawSocket, IntoRawSocket};
        TcpSocket::from_raw_socket(socket.into_raw_socket())
    };

    if config.reuse_address {
        if let Err(e) = socket.set_reuseaddr(true) {
            warn!("tcp set_reuse_address error: {}", e);
        }
    }

    if let Some(size) = config.send_buffer_size {
        if let Err(e) = socket.set_send_buffer_size(size.try_into().unwrap_or(std::u32::MAX)) {
            warn!("tcp set_buffer_size error: {}", e);
        }
    }

    if let Some(size) = config.recv_buffer_size {
        if let Err(e) = socket.set_recv_buffer_size(size.try_into().unwrap_or(std::u32::MAX)) {
            warn!("tcp set_recv_buffer_size error: {}", e);
        }
    }

    let connect = socket.connect(*addr);
    Ok(async move {
        match connect_timeout {
            Some(dur) => match tokio::time::timeout(dur, connect).await {
                Ok(Ok(s)) => Ok(s),
                Ok(Err(e)) => Err(e),
                Err(e) => Err(io::Error::new(io::ErrorKind::TimedOut, e)),
            },
            None => connect.await,
        }
        .map_err(ConnectError::m("tcp connect error"))
    })
}

impl ConnectingTcp<'_> {
    async fn connect(mut self) -> Result<TcpStream, ConnectError> {
        match self.fallback {
            None => self.preferred.connect(self.config).await,
            Some(mut fallback) => {
                let preferred_fut = self.preferred.connect(self.config);
                futures_util::pin_mut!(preferred_fut);

                let fallback_fut = fallback.remote.connect(self.config);
                futures_util::pin_mut!(fallback_fut);

                let fallback_delay = fallback.delay;
                futures_util::pin_mut!(fallback_delay);

                let (result, future) =
                    match futures_util::future::select(preferred_fut, fallback_delay).await {
                        Either::Left((result, _fallback_delay)) => {
                            (result, Either::Right(fallback_fut))
                        }
                        Either::Right(((), preferred_fut)) => {
                            // Delay is done, start polling both the preferred and the fallback
                            futures_util::future::select(preferred_fut, fallback_fut)
                                .await
                                .factor_first()
                        }
                    };

                if result.is_err() {
                    // Fallback to the remaining future (could be preferred or fallback)
                    // if we get an error
                    future.await
                } else {
                    result
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use ::http::Uri;

    use super::super::sealed::{Connect, ConnectSvc};
    use super::{Config, ConnectError, HttpConnector};

    async fn connect<C>(
        connector: C,
        dst: Uri,
    ) -> Result<<C::_Svc as ConnectSvc>::Connection, <C::_Svc as ConnectSvc>::Error>
    where
        C: Connect,
    {
        connector.connect(super::super::sealed::Internal, dst).await
    }

    #[tokio::test]
    async fn test_errors_enforce_http() {
        let dst = "https://example.domain/foo/bar?baz".parse().unwrap();
        let connector = HttpConnector::new();

        let err = connect(connector, dst).await.unwrap_err();
        assert_eq!(&*err.msg, super::INVALID_NOT_HTTP);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn get_local_ips() -> (Option<std::net::Ipv4Addr>, Option<std::net::Ipv6Addr>) {
        use std::net::{IpAddr, TcpListener};

        let mut ip_v4 = None;
        let mut ip_v6 = None;

        let ips = pnet_datalink::interfaces()
            .into_iter()
            .flat_map(|i| i.ips.into_iter().map(|n| n.ip()));

        for ip in ips {
            match ip {
                IpAddr::V4(ip) if TcpListener::bind((ip, 0)).is_ok() => ip_v4 = Some(ip),
                IpAddr::V6(ip) if TcpListener::bind((ip, 0)).is_ok() => ip_v6 = Some(ip),
                _ => (),
            }

            if ip_v4.is_some() && ip_v6.is_some() {
                break;
            }
        }

        (ip_v4, ip_v6)
    }

    #[tokio::test]
    async fn test_errors_missing_scheme() {
        let dst = "example.domain".parse().unwrap();
        let mut connector = HttpConnector::new();
        connector.enforce_http(false);

        let err = connect(connector, dst).await.unwrap_err();
        assert_eq!(&*err.msg, super::INVALID_MISSING_SCHEME);
    }

    // NOTE: pnet crate that we use in this test doesn't compile on Windows
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[tokio::test]
    async fn local_address() {
        use std::net::{IpAddr, TcpListener};
        let _ = pretty_env_logger::try_init();

        let (bind_ip_v4, bind_ip_v6) = get_local_ips();
        let server4 = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = server4.local_addr().unwrap().port();
        let server6 = TcpListener::bind(&format!("[::1]:{}", port)).unwrap();

        let assert_client_ip = |dst: String, server: TcpListener, expected_ip: IpAddr| async move {
            let mut connector = HttpConnector::new();

            match (bind_ip_v4, bind_ip_v6) {
                (Some(v4), Some(v6)) => connector.set_local_addresses(v4, v6),
                (Some(v4), None) => connector.set_local_address(Some(v4.into())),
                (None, Some(v6)) => connector.set_local_address(Some(v6.into())),
                _ => unreachable!(),
            }

            connect(connector, dst.parse().unwrap()).await.unwrap();

            let (_, client_addr) = server.accept().unwrap();

            assert_eq!(client_addr.ip(), expected_ip);
        };

        if let Some(ip) = bind_ip_v4 {
            assert_client_ip(format!("http://127.0.0.1:{}", port), server4, ip.into()).await;
        }

        if let Some(ip) = bind_ip_v6 {
            assert_client_ip(format!("http://[::1]:{}", port), server6, ip.into()).await;
        }
    }

    #[test]
    #[cfg_attr(not(feature = "__internal_happy_eyeballs_tests"), ignore)]
    fn client_happy_eyeballs() {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener};
        use std::time::{Duration, Instant};

        use super::dns;
        use super::ConnectingTcp;

        let _ = pretty_env_logger::try_init();
        let server4 = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = server4.local_addr().unwrap();
        let _server6 = TcpListener::bind(&format!("[::1]:{}", addr.port())).unwrap();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let local_timeout = Duration::default();
        let unreachable_v4_timeout = measure_connect(unreachable_ipv4_addr()).1;
        let unreachable_v6_timeout = measure_connect(unreachable_ipv6_addr()).1;
        let fallback_timeout = std::cmp::max(unreachable_v4_timeout, unreachable_v6_timeout)
            + Duration::from_millis(250);

        let scenarios = &[
            // Fast primary, without fallback.
            (&[local_ipv4_addr()][..], 4, local_timeout, false),
            (&[local_ipv6_addr()][..], 6, local_timeout, false),
            // Fast primary, with (unused) fallback.
            (
                &[local_ipv4_addr(), local_ipv6_addr()][..],
                4,
                local_timeout,
                false,
            ),
            (
                &[local_ipv6_addr(), local_ipv4_addr()][..],
                6,
                local_timeout,
                false,
            ),
            // Unreachable + fast primary, without fallback.
            (
                &[unreachable_ipv4_addr(), local_ipv4_addr()][..],
                4,
                unreachable_v4_timeout,
                false,
            ),
            (
                &[unreachable_ipv6_addr(), local_ipv6_addr()][..],
                6,
                unreachable_v6_timeout,
                false,
            ),
            // Unreachable + fast primary, with (unused) fallback.
            (
                &[
                    unreachable_ipv4_addr(),
                    local_ipv4_addr(),
                    local_ipv6_addr(),
                ][..],
                4,
                unreachable_v4_timeout,
                false,
            ),
            (
                &[
                    unreachable_ipv6_addr(),
                    local_ipv6_addr(),
                    local_ipv4_addr(),
                ][..],
                6,
                unreachable_v6_timeout,
                true,
            ),
            // Slow primary, with (used) fallback.
            (
                &[slow_ipv4_addr(), local_ipv4_addr(), local_ipv6_addr()][..],
                6,
                fallback_timeout,
                false,
            ),
            (
                &[slow_ipv6_addr(), local_ipv6_addr(), local_ipv4_addr()][..],
                4,
                fallback_timeout,
                true,
            ),
            // Slow primary, with (used) unreachable + fast fallback.
            (
                &[slow_ipv4_addr(), unreachable_ipv6_addr(), local_ipv6_addr()][..],
                6,
                fallback_timeout + unreachable_v6_timeout,
                false,
            ),
            (
                &[slow_ipv6_addr(), unreachable_ipv4_addr(), local_ipv4_addr()][..],
                4,
                fallback_timeout + unreachable_v4_timeout,
                true,
            ),
        ];

        // Scenarios for IPv6 -> IPv4 fallback require that host can access IPv6 network.
        // Otherwise, connection to "slow" IPv6 address will error-out immediately.
        let ipv6_accessible = measure_connect(slow_ipv6_addr()).0;

        for &(hosts, family, timeout, needs_ipv6_access) in scenarios {
            if needs_ipv6_access && !ipv6_accessible {
                continue;
            }

            let (start, stream) = rt
                .block_on(async move {
                    let addrs = hosts
                        .iter()
                        .map(|host| (host.clone(), addr.port()).into())
                        .collect();
                    let cfg = Config {
                        local_address_ipv4: None,
                        local_address_ipv6: None,
                        connect_timeout: None,
                        keep_alive_timeout: None,
                        happy_eyeballs_timeout: Some(fallback_timeout),
                        nodelay: false,
                        reuse_address: false,
                        enforce_http: false,
                        send_buffer_size: None,
                        recv_buffer_size: None,
                    };
                    let connecting_tcp = ConnectingTcp::new(dns::SocketAddrs::new(addrs), &cfg);
                    let start = Instant::now();
                    Ok::<_, ConnectError>((start, ConnectingTcp::connect(connecting_tcp).await?))
                })
                .unwrap();
            let res = if stream.peer_addr().unwrap().is_ipv4() {
                4
            } else {
                6
            };
            let duration = start.elapsed();

            // Allow actual duration to be +/- 150ms off.
            let min_duration = if timeout >= Duration::from_millis(150) {
                timeout - Duration::from_millis(150)
            } else {
                Duration::default()
            };
            let max_duration = timeout + Duration::from_millis(150);

            assert_eq!(res, family);
            assert!(duration >= min_duration);
            assert!(duration <= max_duration);
        }

        fn local_ipv4_addr() -> IpAddr {
            Ipv4Addr::new(127, 0, 0, 1).into()
        }

        fn local_ipv6_addr() -> IpAddr {
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).into()
        }

        fn unreachable_ipv4_addr() -> IpAddr {
            Ipv4Addr::new(127, 0, 0, 2).into()
        }

        fn unreachable_ipv6_addr() -> IpAddr {
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 2).into()
        }

        fn slow_ipv4_addr() -> IpAddr {
            // RFC 6890 reserved IPv4 address.
            Ipv4Addr::new(198, 18, 0, 25).into()
        }

        fn slow_ipv6_addr() -> IpAddr {
            // RFC 6890 reserved IPv6 address.
            Ipv6Addr::new(2001, 2, 0, 0, 0, 0, 0, 254).into()
        }

        fn measure_connect(addr: IpAddr) -> (bool, Duration) {
            let start = Instant::now();
            let result =
                std::net::TcpStream::connect_timeout(&(addr, 80).into(), Duration::from_secs(1));

            let reachable = result.is_ok() || result.unwrap_err().kind() == io::ErrorKind::TimedOut;
            let duration = start.elapsed();
            (reachable, duration)
        }
    }
}
