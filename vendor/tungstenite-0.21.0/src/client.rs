//! Methods to connect to a WebSocket as a client.

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    result::Result as StdResult,
};

use http::{request::Parts, Uri};
use log::*;

use url::Url;

use crate::{
    handshake::client::{generate_key, Request, Response},
    protocol::WebSocketConfig,
    stream::MaybeTlsStream,
};

use crate::{
    error::{Error, Result, UrlError},
    handshake::{client::ClientHandshake, HandshakeError},
    protocol::WebSocket,
    stream::{Mode, NoDelay},
};

/// Connect to the given WebSocket in blocking mode.
///
/// Uses a websocket configuration passed as an argument to the function. Calling it with `None` is
/// equal to calling `connect()` function.
///
/// The URL may be either ws:// or wss://.
/// To support wss:// URLs, you must activate the TLS feature on the crate level. Please refer to the
/// project's [README][readme] for more information on available features.
///
/// This function "just works" for those who wants a simple blocking solution
/// similar to `std::net::TcpStream`. If you want a non-blocking or other
/// custom stream, call `client` instead.
///
/// This function uses `native_tls` or `rustls` to do TLS depending on the feature flags enabled. If
/// you want to use other TLS libraries, use `client` instead. There is no need to enable any of
/// the `*-tls` features if you don't call `connect` since it's the only function that uses them.
///
/// [readme]: https://github.com/snapview/tungstenite-rs/#features
pub fn connect_with_config<Req: IntoClientRequest>(
    request: Req,
    config: Option<WebSocketConfig>,
    max_redirects: u8,
) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
    fn try_client_handshake(
        request: Request,
        config: Option<WebSocketConfig>,
    ) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
        let uri = request.uri();
        let mode = uri_mode(uri)?;
        let host = request.uri().host().ok_or(Error::Url(UrlError::NoHostName))?;
        let host = if host.starts_with('[') { &host[1..host.len() - 1] } else { host };
        let port = uri.port_u16().unwrap_or(match mode {
            Mode::Plain => 80,
            Mode::Tls => 443,
        });
        let addrs = (host, port).to_socket_addrs()?;
        let mut stream = connect_to_some(addrs.as_slice(), request.uri())?;
        NoDelay::set_nodelay(&mut stream, true)?;

        #[cfg(not(any(feature = "native-tls", feature = "__rustls-tls")))]
        let client = client_with_config(request, MaybeTlsStream::Plain(stream), config);
        #[cfg(any(feature = "native-tls", feature = "__rustls-tls"))]
        let client = crate::tls::client_tls_with_config(request, stream, config, None);

        client.map_err(|e| match e {
            HandshakeError::Failure(f) => f,
            HandshakeError::Interrupted(_) => panic!("Bug: blocking handshake not blocked"),
        })
    }

    fn create_request(parts: &Parts, uri: &Uri) -> Request {
        let mut builder =
            Request::builder().uri(uri.clone()).method(parts.method.clone()).version(parts.version);
        *builder.headers_mut().expect("Failed to create `Request`") = parts.headers.clone();
        builder.body(()).expect("Failed to create `Request`")
    }

    let (parts, _) = request.into_client_request()?.into_parts();
    let mut uri = parts.uri.clone();

    for attempt in 0..(max_redirects + 1) {
        let request = create_request(&parts, &uri);

        match try_client_handshake(request, config) {
            Err(Error::Http(res)) if res.status().is_redirection() && attempt < max_redirects => {
                if let Some(location) = res.headers().get("Location") {
                    uri = location.to_str()?.parse::<Uri>()?;
                    debug!("Redirecting to {:?}", uri);
                    continue;
                } else {
                    warn!("No `Location` found in redirect");
                    return Err(Error::Http(res));
                }
            }
            other => return other,
        }
    }

    unreachable!("Bug in a redirect handling logic")
}

/// Connect to the given WebSocket in blocking mode.
///
/// The URL may be either ws:// or wss://.
/// To support wss:// URLs, feature `native-tls` or `rustls-tls` must be turned on.
///
/// This function "just works" for those who wants a simple blocking solution
/// similar to `std::net::TcpStream`. If you want a non-blocking or other
/// custom stream, call `client` instead.
///
/// This function uses `native_tls` or `rustls` to do TLS depending on the feature flags enabled. If
/// you want to use other TLS libraries, use `client` instead. There is no need to enable any of
/// the `*-tls` features if you don't call `connect` since it's the only function that uses them.
pub fn connect<Req: IntoClientRequest>(
    request: Req,
) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
    connect_with_config(request, None, 3)
}

fn connect_to_some(addrs: &[SocketAddr], uri: &Uri) -> Result<TcpStream> {
    for addr in addrs {
        debug!("Trying to contact {} at {}...", uri, addr);
        if let Ok(stream) = TcpStream::connect(addr) {
            return Ok(stream);
        }
    }
    Err(Error::Url(UrlError::UnableToConnect(uri.to_string())))
}

/// Get the mode of the given URL.
///
/// This function may be used to ease the creation of custom TLS streams
/// in non-blocking algorithms or for use with TLS libraries other than `native_tls` or `rustls`.
pub fn uri_mode(uri: &Uri) -> Result<Mode> {
    match uri.scheme_str() {
        Some("ws") => Ok(Mode::Plain),
        Some("wss") => Ok(Mode::Tls),
        _ => Err(Error::Url(UrlError::UnsupportedUrlScheme)),
    }
}

/// Do the client handshake over the given stream given a web socket configuration. Passing `None`
/// as configuration is equal to calling `client()` function.
///
/// Use this function if you need a nonblocking handshake support or if you
/// want to use a custom stream like `mio::net::TcpStream` or `openssl::ssl::SslStream`.
/// Any stream supporting `Read + Write` will do.
pub fn client_with_config<Stream, Req>(
    request: Req,
    stream: Stream,
    config: Option<WebSocketConfig>,
) -> StdResult<(WebSocket<Stream>, Response), HandshakeError<ClientHandshake<Stream>>>
where
    Stream: Read + Write,
    Req: IntoClientRequest,
{
    ClientHandshake::start(stream, request.into_client_request()?, config)?.handshake()
}

/// Do the client handshake over the given stream.
///
/// Use this function if you need a nonblocking handshake support or if you
/// want to use a custom stream like `mio::net::TcpStream` or `openssl::ssl::SslStream`.
/// Any stream supporting `Read + Write` will do.
pub fn client<Stream, Req>(
    request: Req,
    stream: Stream,
) -> StdResult<(WebSocket<Stream>, Response), HandshakeError<ClientHandshake<Stream>>>
where
    Stream: Read + Write,
    Req: IntoClientRequest,
{
    client_with_config(request, stream, None)
}

/// Trait for converting various types into HTTP requests used for a client connection.
///
/// This trait is implemented by default for string slices, strings, `url::Url`, `http::Uri` and
/// `http::Request<()>`. Note that the implementation for `http::Request<()>` is trivial and will
/// simply take your request and pass it as is further without altering any headers or URLs, so
/// be aware of this. If you just want to connect to the endpoint with a certain URL, better pass
/// a regular string containing the URL in which case `tungstenite-rs` will take care for generating
/// the proper `http::Request<()>` for you.
pub trait IntoClientRequest {
    /// Convert into a `Request` that can be used for a client connection.
    fn into_client_request(self) -> Result<Request>;
}

impl<'a> IntoClientRequest for &'a str {
    fn into_client_request(self) -> Result<Request> {
        self.parse::<Uri>()?.into_client_request()
    }
}

impl<'a> IntoClientRequest for &'a String {
    fn into_client_request(self) -> Result<Request> {
        <&str as IntoClientRequest>::into_client_request(self)
    }
}

impl IntoClientRequest for String {
    fn into_client_request(self) -> Result<Request> {
        <&str as IntoClientRequest>::into_client_request(&self)
    }
}

impl<'a> IntoClientRequest for &'a Uri {
    fn into_client_request(self) -> Result<Request> {
        self.clone().into_client_request()
    }
}

impl IntoClientRequest for Uri {
    fn into_client_request(self) -> Result<Request> {
        let authority = self.authority().ok_or(Error::Url(UrlError::NoHostName))?.as_str();
        let host = authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority);

        if host.is_empty() {
            return Err(Error::Url(UrlError::EmptyHostName));
        }

        let req = Request::builder()
            .method("GET")
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .uri(self)
            .body(())?;
        Ok(req)
    }
}

impl<'a> IntoClientRequest for &'a Url {
    fn into_client_request(self) -> Result<Request> {
        self.as_str().into_client_request()
    }
}

impl IntoClientRequest for Url {
    fn into_client_request(self) -> Result<Request> {
        self.as_str().into_client_request()
    }
}

impl IntoClientRequest for Request {
    fn into_client_request(self) -> Result<Request> {
        Ok(self)
    }
}

impl<'h, 'b> IntoClientRequest for httparse::Request<'h, 'b> {
    fn into_client_request(self) -> Result<Request> {
        use crate::handshake::headers::FromHttparse;
        Request::from_httparse(self)
    }
}
