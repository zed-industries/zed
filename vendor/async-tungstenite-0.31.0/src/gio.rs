//! `gio` integration.
use tungstenite::Error;

use std::io;

use gio::prelude::*;

use futures_io::{AsyncRead, AsyncWrite};

use tungstenite::client::{uri_mode, IntoClientRequest};
use tungstenite::handshake::client::Request;
use tungstenite::handshake::server::{Callback, NoCallback};
use tungstenite::stream::Mode;

use crate::{client_async_with_config, domain, port, Response, WebSocketConfig, WebSocketStream};

/// Type alias for the stream type of the `connect_async()` functions.
pub type ConnectStream = IOStreamAsyncReadWrite<gio::SocketConnection>;

/// Connect to a given URL.
///
/// Accepts any request that implements [`IntoClientRequest`], which is often just `&str`, but can
/// be a variety of types such as `httparse::Request` or [`tungstenite::http::Request`] for more
/// complex uses.
///
/// ```no_run
/// # use tungstenite::client::IntoClientRequest;
///
/// # async fn test() {
/// use tungstenite::http::{Method, Request};
/// use async_tungstenite::gio::connect_async;
///
/// let mut request = "wss://api.example.com".into_client_request().unwrap();
/// request.headers_mut().insert("api-key", "42".parse().unwrap());
///
/// let (stream, response) = connect_async(request).await.unwrap();
/// # }
/// ```
pub async fn connect_async<R>(
    request: R,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    connect_async_with_config(request, None).await
}

/// Connect to a given URL with a given WebSocket configuration.
pub async fn connect_async_with_config<R>(
    request: R,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request: Request = request.into_client_request()?;

    let domain = domain(&request)?;
    let port = port(&request)?;

    let client = gio::SocketClient::new();

    // Make sure we check domain and mode first. URL must be valid.
    let mode = uri_mode(request.uri())?;
    if let Mode::Tls = mode {
        client.set_tls(true);
    } else {
        client.set_tls(false);
    }

    let connectable = gio::NetworkAddress::new(domain.as_str(), port);

    let socket = client
        .connect_future(&connectable)
        .await
        .map_err(to_std_io_error)?;
    let socket = IOStreamAsyncReadWrite::new(socket)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Unsupported gio::IOStream"))?;

    client_async_with_config(request, socket, config).await
}

/// Accepts a new WebSocket connection with the provided stream.
///
/// This function will internally call `server::accept` to create a
/// handshake representation and returns a future representing the
/// resolution of the WebSocket handshake. The returned future will resolve
/// to either `WebSocketStream<S>` or `Error` depending if it's successful
/// or not.
///
/// This is typically used after a socket has been accepted from a
/// `TcpListener`. That socket is then passed to this function to perform
/// the server half of the accepting a client's websocket connection.
pub async fn accept_async<S>(stream: S) -> Result<WebSocketStream<IOStreamAsyncReadWrite<S>>, Error>
where
    S: IsA<gio::IOStream> + Unpin,
{
    accept_hdr_async(stream, NoCallback).await
}

/// The same as `accept_async()` but the one can specify a websocket configuration.
/// Please refer to `accept_async()` for more details.
pub async fn accept_async_with_config<S>(
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<WebSocketStream<IOStreamAsyncReadWrite<S>>, Error>
where
    S: IsA<gio::IOStream> + Unpin,
{
    accept_hdr_async_with_config(stream, NoCallback, config).await
}

/// Accepts a new WebSocket connection with the provided stream.
///
/// This function does the same as `accept_async()` but accepts an extra callback
/// for header processing. The callback receives headers of the incoming
/// requests and is able to add extra headers to the reply.
pub async fn accept_hdr_async<S, C>(
    stream: S,
    callback: C,
) -> Result<WebSocketStream<IOStreamAsyncReadWrite<S>>, Error>
where
    S: IsA<gio::IOStream> + Unpin,
    C: Callback + Unpin,
{
    accept_hdr_async_with_config(stream, callback, None).await
}

/// The same as `accept_hdr_async()` but the one can specify a websocket configuration.
/// Please refer to `accept_hdr_async()` for more details.
pub async fn accept_hdr_async_with_config<S, C>(
    stream: S,
    callback: C,
    config: Option<WebSocketConfig>,
) -> Result<WebSocketStream<IOStreamAsyncReadWrite<S>>, Error>
where
    S: IsA<gio::IOStream> + Unpin,
    C: Callback + Unpin,
{
    let stream = IOStreamAsyncReadWrite::new(stream)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Unsupported gio::IOStream"))?;

    crate::accept_hdr_async_with_config(stream, callback, config).await
}

/// Adapter for `gio::IOStream` to provide `AsyncRead` and `AsyncWrite`.
#[derive(Debug)]
pub struct IOStreamAsyncReadWrite<T: IsA<gio::IOStream>> {
    #[allow(dead_code)]
    io_stream: T,
    read: gio::InputStreamAsyncRead<gio::PollableInputStream>,
    write: gio::OutputStreamAsyncWrite<gio::PollableOutputStream>,
}

unsafe impl<T: IsA<gio::IOStream>> Send for IOStreamAsyncReadWrite<T> {}

impl<T: IsA<gio::IOStream>> IOStreamAsyncReadWrite<T> {
    /// Create a new `gio::IOStream` adapter
    fn new(stream: T) -> Result<IOStreamAsyncReadWrite<T>, T> {
        let write = stream
            .output_stream()
            .dynamic_cast::<gio::PollableOutputStream>()
            .ok()
            .and_then(|s| s.into_async_write().ok());

        let read = stream
            .input_stream()
            .dynamic_cast::<gio::PollableInputStream>()
            .ok()
            .and_then(|s| s.into_async_read().ok());

        let (read, write) = match (read, write) {
            (Some(read), Some(write)) => (read, write),
            _ => return Err(stream),
        };

        Ok(IOStreamAsyncReadWrite {
            io_stream: stream,
            read,
            write,
        })
    }
}

use std::pin::Pin;
use std::task::{Context, Poll};

impl<T: IsA<gio::IOStream> + Unpin> AsyncRead for IOStreamAsyncReadWrite<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut Pin::get_mut(self).read).poll_read(cx, buf)
    }
}

impl<T: IsA<gio::IOStream> + Unpin> AsyncWrite for IOStreamAsyncReadWrite<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut Pin::get_mut(self).write).poll_write(cx, buf)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).write).poll_close(cx)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).write).poll_flush(cx)
    }
}

fn to_std_io_error(error: glib::Error) -> io::Error {
    match error.kind::<gio::IOErrorEnum>() {
        Some(io_error_enum) => io::Error::new(io_error_enum.into(), error),
        None => io::Error::new(io::ErrorKind::Other, error),
    }
}
