use std::error::Error;
use std::sync::{LazyLock, OnceLock};
use std::{
    borrow::Cow,
    io::{Cursor, Read as _},
    mem,
    pin::Pin,
    task::Poll,
    time::Duration,
};

use gpui_util::defer;

use anyhow::anyhow;
use bytes::{BufMut, Bytes, BytesMut};
use futures::{
    AsyncRead, FutureExt as _, SinkExt as _, Stream as _, TryStreamExt as _, channel::mpsc,
};
use http_client::{RedirectPolicy, Url, http};
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    redirect,
};

const DEFAULT_CAPACITY: usize = 4096;
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
static REDACT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"key=[^&]+").unwrap());

pub struct ReqwestClient {
    client: reqwest::Client,
    proxy: Option<Url>,
    user_agent: Option<HeaderValue>,
    handle: tokio::runtime::Handle,
    response_body_mode: ResponseBodyMode,
}

#[derive(Clone)]
enum ResponseBodyMode {
    CallerPolled,
    TokioPumped,
}

impl ReqwestClient {
    fn builder(read_timeout: Option<Duration>) -> reqwest::ClientBuilder {
        let builder = reqwest::Client::builder()
            .use_rustls_tls()
            .connect_timeout(Duration::from_secs(10))
            // Detect and drop connections that have silently gone bad on a
            // flaky path (NAT timeouts, resets) instead of reusing them. A
            // stale reused HTTP/2 connection is a common source of
            // `BadRecordMac` TLS errors against long-lived endpoints.
            .tcp_keepalive(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(30))
            .http2_keep_alive_interval(Duration::from_secs(15))
            .http2_keep_alive_timeout(Duration::from_secs(10))
            .http2_keep_alive_while_idle(true);
        match read_timeout {
            Some(read_timeout) => builder.read_timeout(read_timeout),
            None => builder,
        }
    }

    pub fn new() -> Self {
        let client = Self::builder(None)
            .build()
            .expect("Failed to initialize HTTP client");
        Self::from_client(client)
    }

    pub fn user_agent(agent: &str) -> anyhow::Result<Self> {
        let mut map = HeaderMap::new();
        map.insert(http::header::USER_AGENT, HeaderValue::from_str(agent)?);
        let client = Self::builder(None).default_headers(map).build()?;
        Ok(Self::from_client(client))
    }

    pub fn proxy_and_user_agent(proxy: Option<Url>, user_agent: &str) -> anyhow::Result<Self> {
        Self::build_proxy_client(proxy, user_agent, None, None)
    }

    /// Like [`ReqwestClient::proxy_and_user_agent`], but applies a per-read
    /// idle timeout while keeping all timeout-bound response body work on
    /// `tokio_handle`. The response returned through [`http_client::HttpClient`]
    /// is executor-neutral and may be consumed outside Tokio.
    ///
    /// `read_timeout` resets whenever the response body yields bytes, including
    /// application-level keep-alive data. Callers streaming long-lived responses
    /// should size it comfortably above the provider's keep-alive interval.
    ///
    /// On macOS the timeout's monotonic clock pauses during system sleep, so it
    /// does not fire from a suspend alone. Callers that need prompt detection of
    /// a connection killed while suspended must re-validate the stream on wake.
    pub fn proxy_user_agent_and_read_timeout(
        proxy: Option<Url>,
        user_agent: &str,
        read_timeout: Duration,
        tokio_handle: tokio::runtime::Handle,
    ) -> anyhow::Result<Self> {
        Self::build_proxy_client(proxy, user_agent, Some(read_timeout), Some(tokio_handle))
    }

    fn build_proxy_client(
        proxy: Option<Url>,
        user_agent: &str,
        read_timeout: Option<Duration>,
        tokio_handle: Option<tokio::runtime::Handle>,
    ) -> anyhow::Result<Self> {
        let user_agent = HeaderValue::from_str(user_agent)?;

        let mut map = HeaderMap::new();
        map.insert(http::header::USER_AGENT, user_agent.clone());
        let mut client = Self::builder(read_timeout).default_headers(map);
        let client_has_proxy;

        if let Some(proxy) = proxy.as_ref().and_then(|proxy_url| {
            reqwest::Proxy::all(proxy_url.clone())
                .inspect_err(|e| {
                    log::error!(
                        "Failed to parse proxy URL '{}': {}",
                        proxy_url,
                        e.source().unwrap_or(&e as &_)
                    )
                })
                .ok()
        }) {
            // Respect NO_PROXY env var
            client = client.proxy(proxy.no_proxy(reqwest::NoProxy::from_env()));
            client_has_proxy = true;
        } else {
            client_has_proxy = false;
        };

        let client = match tokio_handle.as_ref() {
            Some(tokio_handle) => {
                let _runtime_guard = tokio_handle.enter();
                client
                    .use_preconfigured_tls(http_client_tls::tls_config())
                    .build()?
            }
            None => client
                .use_preconfigured_tls(http_client_tls::tls_config())
                .build()?,
        };
        let mut client = match tokio_handle {
            Some(tokio_handle) => {
                Self::from_client_and_handle(client, tokio_handle, ResponseBodyMode::TokioPumped)
            }
            None => Self::from_client(client),
        };
        client.proxy = client_has_proxy.then_some(proxy).flatten();
        client.user_agent = Some(user_agent);
        Ok(client)
    }

    fn from_client(client: reqwest::Client) -> Self {
        let handle = tokio::runtime::Handle::try_current().unwrap_or_else(|_| {
            log::debug!("no tokio runtime found, creating one for Reqwest...");
            runtime().handle().clone()
        });
        Self::from_client_and_handle(client, handle, ResponseBodyMode::CallerPolled)
    }

    fn from_client_and_handle(
        client: reqwest::Client,
        handle: tokio::runtime::Handle,
        response_body_mode: ResponseBodyMode,
    ) -> Self {
        Self {
            client,
            handle,
            proxy: None,
            user_agent: None,
            response_body_mode,
        }
    }
}

pub fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            // Since we now have two executors, let's try to keep our footprint small
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("Failed to initialize HTTP client")
    })
}

// This struct is essentially a re-implementation of
// https://docs.rs/tokio-util/0.7.12/tokio_util/io/struct.ReaderStream.html
// except outside of Tokio's aegis
struct StreamReader {
    reader: Option<Pin<Box<dyn futures::AsyncRead + Send + Sync>>>,
    buf: BytesMut,
    capacity: usize,
}

impl StreamReader {
    fn new(reader: Pin<Box<dyn futures::AsyncRead + Send + Sync>>) -> Self {
        Self {
            reader: Some(reader),
            buf: BytesMut::new(),
            capacity: DEFAULT_CAPACITY,
        }
    }
}

impl futures::Stream for StreamReader {
    type Item = std::io::Result<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();

        let mut reader = match this.reader.take() {
            Some(r) => r,
            None => return Poll::Ready(None),
        };

        if this.buf.capacity() == 0 {
            let capacity = this.capacity;
            this.buf.reserve(capacity);
        }

        match poll_read_buf(&mut reader, cx, &mut this.buf) {
            Poll::Pending => {
                self.reader = Some(reader);

                Poll::Pending
            }
            Poll::Ready(Err(err)) => {
                self.reader = None;

                Poll::Ready(Some(Err(err)))
            }
            Poll::Ready(Ok(0)) => {
                self.reader = None;
                Poll::Ready(None)
            }
            Poll::Ready(Ok(_)) => {
                let chunk = this.buf.split();
                self.reader = Some(reader);
                Poll::Ready(Some(Ok(chunk.freeze())))
            }
        }
    }
}

/// Implementation from <https://docs.rs/tokio-util/0.7.12/src/tokio_util/util/poll_buf.rs.html>
/// Specialized for this use case
fn poll_read_buf(
    io: &mut Pin<Box<dyn futures::AsyncRead + Send + Sync>>,
    cx: &mut std::task::Context<'_>,
    buf: &mut BytesMut,
) -> Poll<std::io::Result<usize>> {
    if !buf.has_remaining_mut() {
        return Poll::Ready(Ok(0));
    }

    let n = {
        let dst = buf.chunk_mut();

        // Safety: `chunk_mut()` returns a `&mut UninitSlice`, and `UninitSlice` is a
        // transparent wrapper around `[std::mem::MaybeUninit<u8>]`.
        let dst = unsafe { &mut *(dst as *mut _ as *mut [std::mem::MaybeUninit<u8>]) };
        let mut read_buf = tokio::io::ReadBuf::uninit(dst);
        let unfilled_portion = read_buf.initialize_unfilled();
        // SAFETY: Pin projection
        let io_pin = unsafe { Pin::new_unchecked(io) };
        // `futures::AsyncRead` reports the byte count as the poll's return
        // value; `read_buf.filled()` stays empty because the reader writes
        // through the initialized slice without advancing the `ReadBuf`.
        std::task::ready!(io_pin.poll_read(cx, unfilled_portion)?)
    };

    // Safety: `initialize_unfilled()` zero-initialized the entire spare
    // capacity, so the first `n` bytes are initialized no matter how many the
    // reader actually wrote, and `advance_mut` panics rather than exceeding
    // the capacity if `n` overstates the slice length.
    unsafe {
        buf.advance_mut(n);
    }

    Poll::Ready(Ok(n))
}

enum ResponseBodyEvent {
    Chunk(Bytes),
    Error(std::io::Error),
    End,
}

struct TokioResponseBody {
    receiver: mpsc::Receiver<ResponseBodyEvent>,
    current_chunk: Option<Cursor<Bytes>>,
    producer_abort_handle: tokio::task::AbortHandle,
    completed: bool,
}

impl AsyncRead for TokioResponseBody {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buffer: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        if buffer.is_empty() || self.completed {
            return Poll::Ready(Ok(0));
        }

        loop {
            if let Some(current_chunk) = self.current_chunk.as_mut() {
                let byte_count = current_chunk.read(buffer)?;
                if byte_count > 0 {
                    return Poll::Ready(Ok(byte_count));
                }
                self.current_chunk = None;
            }

            match Pin::new(&mut self.receiver).poll_next(cx) {
                Poll::Ready(Some(ResponseBodyEvent::Chunk(chunk))) => {
                    self.current_chunk = Some(Cursor::new(chunk));
                }
                Poll::Ready(Some(ResponseBodyEvent::Error(error))) => {
                    self.completed = true;
                    return Poll::Ready(Err(error));
                }
                Poll::Ready(Some(ResponseBodyEvent::End)) => {
                    self.completed = true;
                    return Poll::Ready(Ok(0));
                }
                Poll::Ready(None) => {
                    self.completed = true;
                    return Poll::Ready(Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Tokio response body task ended before completing the response",
                    )));
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

impl Drop for TokioResponseBody {
    fn drop(&mut self) {
        if !self.completed {
            self.producer_abort_handle.abort();
        }
    }
}

async fn pump_response_body(
    response: reqwest::Response,
    mut sender: mpsc::Sender<ResponseBodyEvent>,
) {
    let mut stream = response.bytes_stream();
    loop {
        match stream.try_next().await {
            Ok(Some(chunk)) => {
                if sender.send(ResponseBodyEvent::Chunk(chunk)).await.is_err() {
                    return;
                }
            }
            Ok(None) => {
                if sender.send(ResponseBodyEvent::End).await.is_err() {
                    return;
                }
                return;
            }
            Err(error) => {
                let error = std::io::Error::other(redact_error(error));
                if sender.send(ResponseBodyEvent::Error(error)).await.is_err() {
                    return;
                }
                return;
            }
        }
    }
}

fn redact_error(mut error: reqwest::Error) -> reqwest::Error {
    if let Some(url) = error.url_mut()
        && let Some(query) = url.query()
        && let Cow::Owned(redacted) = REDACT_REGEX.replace_all(query, "key=REDACTED")
    {
        url.set_query(Some(redacted.as_str()));
    }
    error
}

impl http_client::HttpClient for ReqwestClient {
    fn proxy(&self) -> Option<&Url> {
        self.proxy.as_ref()
    }

    fn user_agent(&self) -> Option<&HeaderValue> {
        self.user_agent.as_ref()
    }

    fn send(
        &self,
        req: http::Request<http_client::AsyncBody>,
    ) -> futures::future::BoxFuture<
        'static,
        anyhow::Result<http_client::Response<http_client::AsyncBody>>,
    > {
        let (parts, body) = req.into_parts();

        let mut request = self.client.request(parts.method, parts.uri.to_string());
        request = request.headers(parts.headers);
        if let Some(redirect_policy) = parts.extensions.get::<RedirectPolicy>() {
            request = request.redirect_policy(match redirect_policy {
                RedirectPolicy::NoFollow => redirect::Policy::none(),
                RedirectPolicy::FollowLimit(limit) => redirect::Policy::limited(*limit as usize),
                RedirectPolicy::FollowAll => redirect::Policy::limited(100),
            });
        }
        let request = request.body(match body.0 {
            http_client::Inner::Empty => reqwest::Body::default(),
            http_client::Inner::Bytes(cursor) => cursor.into_inner().into(),
            http_client::Inner::AsyncReader(stream) => {
                reqwest::Body::wrap_stream(StreamReader::new(stream))
            }
        });

        let handle = self.handle.clone();
        let response_body_mode = self.response_body_mode.clone();
        async move {
            let join_handle = handle.spawn(async move {
                let mut response = request.send().await.map_err(redact_error)?;
                let headers = mem::take(response.headers_mut());
                let mut builder = http::Response::builder()
                    .status(response.status().as_u16())
                    .version(response.version());
                *builder.headers_mut().unwrap() = headers;

                let body = match response_body_mode {
                    ResponseBodyMode::CallerPolled => {
                        let bytes = response
                            .bytes_stream()
                            .map_err(futures::io::Error::other)
                            .into_async_read();
                        http_client::AsyncBody::from_reader(bytes)
                    }
                    ResponseBodyMode::TokioPumped => {
                        let (sender, receiver) = mpsc::channel(1);
                        let producer = tokio::spawn(pump_response_body(response, sender));
                        http_client::AsyncBody::from_reader(TokioResponseBody {
                            receiver,
                            current_chunk: None,
                            producer_abort_handle: producer.abort_handle(),
                            completed: false,
                        })
                    }
                };

                builder.body(body).map_err(|error| anyhow!(error))
            });
            let abort_handle = join_handle.abort_handle();
            let _abort_on_drop = defer(move || abort_handle.abort());
            join_handle.await?
        }
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read as _, Write as _};
    use std::net::{TcpListener, TcpStream};
    use std::time::Duration;

    use futures::AsyncReadExt as _;
    use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest, Url};

    use crate::ReqwestClient;

    /// Regression test: `StreamReader::poll_next` used to drop the reader it
    /// `take()`s whenever the reader returned `Poll::Pending`, so the next
    /// poll reported end-of-stream and streamed request bodies were silently
    /// truncated. Readers backed by real I/O (e.g. `async_fs::File`) return
    /// `Pending` on their very first read, so their uploads sent zero bytes.
    #[test]
    fn test_streamed_body_survives_pending_reader() {
        let payload: Vec<u8> = (0..30_000usize).map(|byte| (byte % 251) as u8).collect();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let expected_payload = payload.clone();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut buffer = [0u8; 8192];
            loop {
                let read = stream.read(&mut buffer).unwrap();
                assert_ne!(read, 0, "client closed the connection mid-request");
                request.extend_from_slice(&buffer[..read]);
                if let Some(position) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                    let body_start = position + 4;
                    while request.len() - body_start < expected_payload.len() {
                        let read = stream.read(&mut buffer).unwrap();
                        assert_ne!(read, 0, "client closed the connection mid-body");
                        request.extend_from_slice(&buffer[..read]);
                    }
                    assert_eq!(&request[body_start..], &expected_payload);
                    break;
                }
            }
            stream
                .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
                .unwrap();
        });

        // A reader that returns `Pending` before every chunk, like a reader
        // backed by real I/O would.
        struct PendingFirstReader {
            data: std::io::Cursor<Vec<u8>>,
            ready: bool,
        }

        impl futures::AsyncRead for PendingFirstReader {
            fn poll_read(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut [u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                if self.ready {
                    self.ready = false;
                    std::task::Poll::Ready(self.data.read(buf))
                } else {
                    self.ready = true;
                    cx.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            }
        }

        let reader = PendingFirstReader {
            data: std::io::Cursor::new(payload.clone()),
            ready: false,
        };

        let client = ReqwestClient::new();
        let request = HttpRequest::builder()
            .method(Method::PUT)
            .uri(format!("http://{address}/upload"))
            .header("Content-Length", payload.len().to_string())
            .body(AsyncBody::from_reader(reader))
            .unwrap();
        let response = futures::executor::block_on(client.send(request)).unwrap();
        assert!(response.status().is_success());
        server.join().unwrap();
    }

    #[test]
    fn test_timeout_response_body_can_be_read_outside_tokio() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            read_request_headers(&mut stream);
            stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\ncontent-length: 13\r\nconnection: close\r\n\r\nresponse body",
                )
                .unwrap();
        });
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        let client = ReqwestClient::proxy_user_agent_and_read_timeout(
            None,
            "test",
            Duration::from_secs(1),
            runtime.handle().clone(),
        )
        .unwrap();
        let request = HttpRequest::builder()
            .uri(format!("http://{address}"))
            .body(AsyncBody::default())
            .unwrap();

        let mut response = futures::executor::block_on(client.send(request)).unwrap();
        let mut response_body = String::new();
        futures::executor::block_on(response.body_mut().read_to_string(&mut response_body))
            .unwrap();

        assert_eq!(response_body, "response body");
        let byte_count =
            futures::executor::block_on(response.body_mut().read(&mut [0; 1])).unwrap();
        assert_eq!(byte_count, 0);
        server.join().unwrap();
    }

    #[test]
    fn test_response_body_read_timeout_outside_tokio() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            read_request_headers(&mut stream);
            stream
                .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 1\r\nconnection: close\r\n\r\n")
                .unwrap();
            std::thread::sleep(Duration::from_millis(250));
        });
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        let client = ReqwestClient::proxy_user_agent_and_read_timeout(
            None,
            "test",
            Duration::from_millis(50),
            runtime.handle().clone(),
        )
        .unwrap();
        let request = HttpRequest::builder()
            .uri(format!("http://{address}"))
            .body(AsyncBody::default())
            .unwrap();

        let mut response = futures::executor::block_on(client.send(request)).unwrap();
        let error = futures::executor::block_on(response.body_mut().read_to_end(&mut Vec::new()))
            .unwrap_err();

        assert!(
            error
                .get_ref()
                .and_then(|source| source.downcast_ref::<reqwest::Error>())
                .is_some_and(reqwest::Error::is_timeout),
            "expected a reqwest timeout, got {error:#}",
        );
        server.join().unwrap();
    }

    #[test]
    fn test_proxy_uri() {
        let client = ReqwestClient::new();
        assert_eq!(client.proxy(), None);

        let proxy = Url::parse("http://localhost:10809").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy.clone()), "test").unwrap();
        assert_eq!(client.proxy(), Some(&proxy));

        let proxy = Url::parse("https://localhost:10809").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy.clone()), "test").unwrap();
        assert_eq!(client.proxy(), Some(&proxy));

        let proxy = Url::parse("socks4://localhost:10808").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy.clone()), "test").unwrap();
        assert_eq!(client.proxy(), Some(&proxy));

        let proxy = Url::parse("socks4a://localhost:10808").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy.clone()), "test").unwrap();
        assert_eq!(client.proxy(), Some(&proxy));

        let proxy = Url::parse("socks5://localhost:10808").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy.clone()), "test").unwrap();
        assert_eq!(client.proxy(), Some(&proxy));

        let proxy = Url::parse("socks5h://localhost:10808").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy.clone()), "test").unwrap();
        assert_eq!(client.proxy(), Some(&proxy));
    }

    #[test]
    fn test_invalid_proxy_uri() {
        let proxy = Url::parse("socks://127.0.0.1:20170").unwrap();
        let client = ReqwestClient::proxy_and_user_agent(Some(proxy), "test").unwrap();
        assert!(
            client.proxy.is_none(),
            "An invalid proxy URL should add no proxy to the client!"
        )
    }

    fn read_request_headers(stream: &mut TcpStream) {
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            let byte_count = stream.read(&mut buffer).unwrap();
            assert_ne!(
                byte_count, 0,
                "client closed before sending request headers"
            );
            request.extend_from_slice(&buffer[..byte_count]);
        }
    }
}
