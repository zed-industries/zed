use std::error::Error;
use std::sync::{LazyLock, OnceLock};
use std::{any::type_name, borrow::Cow, mem, pin::Pin, task::Poll, time::Duration};

use anyhow::anyhow;
use bytes::{BufMut, Bytes, BytesMut};
use futures::{AsyncRead, TryStreamExt as _};
use http_client::{RedirectPolicy, Url, http};
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    redirect,
};
use smol::future::FutureExt;

const DEFAULT_CAPACITY: usize = 4096;
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
static REDACT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"key=[^&]+").unwrap());

pub struct ReqwestClient {
    client: reqwest::Client,
    proxy: Option<Url>,
    handle: tokio::runtime::Handle,
}

impl ReqwestClient {
    fn builder() -> reqwest::ClientBuilder {
        reqwest::Client::builder()
            .use_rustls_tls()
            .connect_timeout(Duration::from_secs(10))
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

    pub fn proxy_and_user_agent(proxy: Option<Url>, agent: &str) -> anyhow::Result<Self> {
        let mut map = HeaderMap::new();
        map.insert(http::header::USER_AGENT, HeaderValue::from_str(agent)?);
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
            client = client.proxy(proxy);
            client_has_proxy = true;
        } else {
            client_has_proxy = false;
        };

        let client = client
            .use_preconfigured_tls(http_client_tls::tls_config())
            .build()?;
        let mut client: ReqwestClient = client.into();
        client.proxy = client_has_proxy.then_some(proxy).flatten();
        Ok(client)
    }
}

impl From<reqwest::Client> for ReqwestClient {
    fn from(client: reqwest::Client) -> Self {
        let handle = tokio::runtime::Handle::try_current().unwrap_or_else(|_| {
            log::debug!("no tokio runtime found, creating one for Reqwest...");
            let runtime = RUNTIME.get_or_init(|| {
                tokio::runtime::Builder::new_multi_thread()
                    // Since we now have two executors, let's try to keep our footprint small
                    .worker_threads(1)
                    .enable_all()
                    .build()
                    .expect("Failed to initialize HTTP client")
            });

            runtime.handle().clone()
        });
        Self {
            client,
            handle,
            proxy: None,
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
            Poll::Pending => Poll::Pending,
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
pub fn poll_read_buf(
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
        // transparent wrapper around `[MaybeUninit<u8>]`.
        let dst = unsafe { &mut *(dst as *mut _ as *mut [std::mem::MaybeUninit<u8>]) };
        let mut buf = tokio::io::ReadBuf::uninit(dst);
        let ptr = buf.filled().as_ptr();
        let unfilled_portion = buf.initialize_unfilled();
        // SAFETY: Pin projection
        let io_pin = unsafe { Pin::new_unchecked(io) };
        std::task::ready!(io_pin.poll_read(cx, unfilled_portion)?);

        // Ensure the pointer does not change from under us
        assert_eq!(ptr, buf.filled().as_ptr());
        buf.filled().len()
    };

    // Safety: This is guaranteed to be the number of initialized (and read)
    // bytes due to the invariants provided by `ReadBuf::filled`.
    unsafe {
        buf.advance_mut(n);
    }

    Poll::Ready(Ok(n))
}

fn redact_error(mut error: reqwest::Error) -> reqwest::Error {
    if let Some(url) = error.url_mut() {
        if let Some(query) = url.query() {
            if let Cow::Owned(redacted) = REDACT_REGEX.replace_all(query, "key=REDACTED") {
                url.set_query(Some(redacted.as_str()));
            }
        }
    }
    error
}

impl http_client::HttpClient for ReqwestClient {
    fn proxy(&self) -> Option<&Url> {
        self.proxy.as_ref()
    }

    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn send(
        &self,
        req: http::Request<http_client::AsyncBody>,
    ) -> futures::future::BoxFuture<
        'static,
        Result<http_client::Response<http_client::AsyncBody>, anyhow::Error>,
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
            let mut response = handle
                .spawn(async { request.send().await })
                .await?
                .map_err(redact_error)?;

            let headers = mem::take(response.headers_mut());
            let mut builder = http::Response::builder()
                .status(response.status().as_u16())
                .version(response.version());
            *builder.headers_mut().unwrap() = headers;

            let bytes = response
                .bytes_stream()
                .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
                .into_async_read();
            let body = http_client::AsyncBody::from_reader(bytes);

            builder.body(body).map_err(|e| anyhow!(e))
        }
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use http_client::{HttpClient, Url};

    use crate::ReqwestClient;

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
