use std::error::Error;
use std::sync::{LazyLock, OnceLock};
use std::{borrow::Cow, mem, pin::Pin, task::Poll, time::Duration};

use gpui_util::defer;

use anyhow::anyhow;
use bytes::{BufMut, Bytes, BytesMut};
use futures::{AsyncRead, FutureExt as _, TryStreamExt as _};
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
}

impl ReqwestClient {
    fn builder() -> reqwest::ClientBuilder {
        reqwest::Client::builder()
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
            .http2_keep_alive_while_idle(true)
    }

    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("Failed to initialize HTTP client")
            .into()
    }

    pub fn user_agent(agent: &str) -> anyhow::Result<Self> {
        let mut map = HeaderMap::new();
        map.insert(http::header::USER_AGENT, HeaderValue::from_str(agent)?);
        let client = Self::builder().default_headers(map).build()?;
        Ok(client.into())
    }

    pub fn proxy_and_user_agent(proxy: Option<Url>, user_agent: &str) -> anyhow::Result<Self> {
        let user_agent = HeaderValue::from_str(user_agent)?;

        let mut map = HeaderMap::new();
        map.insert(http::header::USER_AGENT, user_agent.clone());
        let mut client = Self::builder().default_headers(map);
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

        let client = client
            .use_preconfigured_tls(http_client_tls::tls_config())
            .build()?;
        let mut client: ReqwestClient = client.into();
        client.proxy = client_has_proxy.then_some(proxy).flatten();
        client.user_agent = Some(user_agent);
        Ok(client)
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

impl From<reqwest::Client> for ReqwestClient {
    fn from(client: reqwest::Client) -> Self {
        let handle = tokio::runtime::Handle::try_current().unwrap_or_else(|_| {
            log::debug!("no tokio runtime found, creating one for Reqwest...");
            runtime().handle().clone()
        });
        Self {
            client,
            handle,
            proxy: None,
            user_agent: None,
        }
    }
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
        async move {
            let join_handle = handle.spawn(async { request.send().await });
            let abort_handle = join_handle.abort_handle();
            let _abort_on_drop = defer(move || abort_handle.abort());

            let mut response = join_handle.await?.map_err(redact_error)?;

            let headers = mem::take(response.headers_mut());
            let mut builder = http::Response::builder()
                .status(response.status().as_u16())
                .version(response.version());
            *builder.headers_mut().unwrap() = headers;

            let bytes = response
                .bytes_stream()
                .map_err(futures::io::Error::other)
                .into_async_read();
            let body = http_client::AsyncBody::from_reader(bytes);

            builder.body(body).map_err(|e| anyhow!(e))
        }
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read as _, Write as _};
    use std::net::TcpListener;

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
}
