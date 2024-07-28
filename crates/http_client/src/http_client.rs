pub mod github;

pub use anyhow::{anyhow, Result};
use derive_more::Deref;
use futures::{future::BoxFuture, AsyncRead, AsyncReadExt as _};
use futures_lite::FutureExt;
use isahc::config::{Configurable, RedirectPolicy};
pub use isahc::http;
use isahc::AsyncBody as IsahcBody;
pub use isahc::{
    http::{Method, Request, Response, StatusCode, Uri},
    Error, HttpClient as IsahcHttpClient,
};
use smol::Unblock;
#[cfg(feature = "test-support")]
use std::fmt;
use std::{
    borrow::Cow,
    io::{Cursor, Read},
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
    time::Duration,
};
pub use url::Url;

/// The body of an HTTP Request
pub type HttpBody = IsahcBody;

// A type to implement Read on the inner dyn Read
struct UreqReader(Box<dyn Read + Send + 'static>);

impl Read for UreqReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

/// Based on the implementation of AsyncBody in
/// https://github.com/sagebind/isahc/blob/5c533f1ef4d6bdf1fd291b5103c22110f41d0bf0/src/body/mod.rs
struct HttpBodyNew(Inner);

enum Inner {
    /// An empty body.
    Empty,

    /// A body stored in memory.
    SyncReader(std::io::Cursor<Cow<'static, [u8]>>),

    /// An asynchronous reader.
    AsyncReader(Pin<Box<dyn futures::AsyncRead + Send + Sync>>),

    /// A compatibility layer over our old isahc client, to make it compatible with ureq
    UReqReader(smol::Unblock<UreqReader>),
}

impl HttpBodyNew {
    /// Create a streaming body that reads from the given reader.
    pub fn from_reader<R>(read: R) -> Self
    where
        R: AsyncRead + Send + Sync + 'static,
    {
        Self(Inner::AsyncReader(Box::pin(read)))
    }
}

impl std::io::Read for HttpBodyNew {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            Inner::Empty => Ok(0),
            Inner::SyncReader(cursor) => cursor.read(buf),
            Inner::AsyncReader(async_reader) => smol::block_on(async_reader.read(buf)),
            Inner::UReqReader(unblock_reader) => smol::block_on(unblock_reader.read(buf)),
        }
    }
}

impl futures::AsyncRead for HttpBodyNew {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        // SAFETY: Standard Enum pin projection
        let inner = unsafe { &mut self.get_unchecked_mut().0 };
        match inner {
            Inner::Empty => Poll::Ready(Ok(0)),
            // Blocking call is over an in-memory buffer
            Inner::SyncReader(cursor) => Poll::Ready(cursor.read(buf)),
            Inner::AsyncReader(async_reader) => {
                AsyncRead::poll_read(async_reader.as_mut(), cx, buf)
            }
            Inner::UReqReader(unblock_reader) => AsyncRead::poll_read(
                unsafe { Pin::new_unchecked(unblock_reader).as_mut() },
                cx,
                buf,
            ),
        }
    }
}

impl Default for HttpBodyNew {
    fn default() -> Self {
        Self(Inner::Empty)
    }
}

impl From<()> for HttpBodyNew {
    fn from(_: ()) -> Self {
        Self(Inner::Empty)
    }
}

impl From<Vec<u8>> for HttpBodyNew {
    fn from(body: Vec<u8>) -> Self {
        Self(Inner::SyncReader(Cursor::new(Cow::Owned(body))))
    }
}

impl From<&'_ [u8]> for HttpBodyNew {
    fn from(body: &[u8]) -> Self {
        body.to_vec().into()
    }
}

impl From<String> for HttpBodyNew {
    fn from(body: String) -> Self {
        body.into_bytes().into()
    }
}

impl From<&'_ str> for HttpBodyNew {
    fn from(body: &str) -> Self {
        body.as_bytes().into()
    }
}

impl<T: Into<Self>> From<Option<T>> for HttpBodyNew {
    fn from(body: Option<T>) -> Self {
        match body {
            Some(body) => body.into(),
            None => Self(Inner::Empty),
        }
    }
}

impl From<ureq::Response> for HttpBodyNew {
    fn from(value: ureq::Response) -> Self {
        HttpBodyNew(Inner::UReqReader(Unblock::new(UreqReader(
            value.into_reader(),
        ))))
    }
}

pub trait HttpClient: Send + Sync {
    // TODO: Make a better API for this once we have ureq in place
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>>;

    fn send(
        &self,
        req: Request<HttpBody>,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        self.send_with_redirect_policy(req, false)
    }

    fn get<'a>(
        &'a self,
        uri: &str,
        body: HttpBody,
        follow_redirects: bool,
    ) -> BoxFuture<'a, Result<Response<HttpBody>, Error>> {
        let request = isahc::Request::builder()
            .redirect_policy(if follow_redirects {
                RedirectPolicy::Follow
            } else {
                RedirectPolicy::None
            })
            .method(Method::GET)
            .uri(uri)
            .body(body);
        match request {
            Ok(request) => self.send(request),
            Err(error) => async move { Err(error.into()) }.boxed(),
        }
    }

    fn post_json<'a>(
        &'a self,
        uri: &str,
        body: HttpBody,
    ) -> BoxFuture<'a, Result<Response<HttpBody>, Error>> {
        let request = isahc::Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(body);
        match request {
            Ok(request) => self.send(request),
            Err(error) => async move { Err(error.into()) }.boxed(),
        }
    }

    fn proxy(&self) -> Option<&Uri>;
}

/// An [`HttpClient`] that may have a proxy.
#[derive(Deref)]
pub struct HttpClientWithProxy {
    #[deref]
    client: Arc<dyn HttpClient>,
    proxy: Option<Uri>,
}

impl HttpClientWithProxy {
    /// Returns a new [`HttpClientWithProxy`] with the given proxy URL.
    pub fn new(proxy_url: Option<String>) -> Self {
        let proxy_url = proxy_url
            .and_then(|input| {
                input
                    .parse::<Uri>()
                    .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                    .ok()
            })
            .or_else(read_proxy_from_env);

        Self {
            client: client(proxy_url.clone()),
            proxy: proxy_url,
        }
    }
}

impl HttpClient for HttpClientWithProxy {
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        self.client.send_with_redirect_policy(req, follow_redirects)
    }

    fn proxy(&self) -> Option<&Uri> {
        self.proxy.as_ref()
    }
}

impl HttpClient for Arc<HttpClientWithProxy> {
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        self.client.send_with_redirect_policy(req, follow_redirects)
    }

    fn proxy(&self) -> Option<&Uri> {
        self.proxy.as_ref()
    }
}

/// An [`HttpClient`] that has a base URL.
pub struct HttpClientWithUrl {
    base_url: Mutex<String>,
    client: HttpClientWithProxy,
}

impl HttpClientWithUrl {
    /// Returns a new [`HttpClientWithUrl`] with the given base URL.
    pub fn new(base_url: impl Into<String>, proxy_url: Option<String>) -> Self {
        let client = HttpClientWithProxy::new(proxy_url);

        Self {
            base_url: Mutex::new(base_url.into()),
            client,
        }
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> String {
        self.base_url
            .lock()
            .map_or_else(|_| Default::default(), |url| url.clone())
    }

    /// Sets the base URL.
    pub fn set_base_url(&self, base_url: impl Into<String>) {
        let base_url = base_url.into();
        self.base_url
            .lock()
            .map(|mut url| {
                *url = base_url;
            })
            .ok();
    }

    /// Builds a URL using the given path.
    pub fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url(), path)
    }

    /// Builds a Zed API URL using the given path.
    pub fn build_zed_api_url(&self, path: &str, query: &[(&str, &str)]) -> Result<Url> {
        let base_url = self.base_url();
        let base_api_url = match base_url.as_ref() {
            "https://zed.dev" => "https://api.zed.dev",
            "https://staging.zed.dev" => "https://api-staging.zed.dev",
            "http://localhost:3000" => "http://localhost:8080",
            other => other,
        };

        Ok(Url::parse_with_params(
            &format!("{}{}", base_api_url, path),
            query,
        )?)
    }
}

impl HttpClient for Arc<HttpClientWithUrl> {
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        self.client.send_with_redirect_policy(req, follow_redirects)
    }

    fn proxy(&self) -> Option<&Uri> {
        self.client.proxy.as_ref()
    }
}

impl HttpClient for HttpClientWithUrl {
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        self.client.send_with_redirect_policy(req, follow_redirects)
    }

    fn proxy(&self) -> Option<&Uri> {
        self.client.proxy.as_ref()
    }
}

pub fn client(proxy: Option<Uri>) -> Arc<dyn HttpClient> {
    Arc::new(HttpClientWithProxy {
        client: Arc::new(
            isahc::HttpClient::builder()
                .connect_timeout(Duration::from_secs(5))
                .low_speed_timeout(100, Duration::from_secs(5))
                .proxy(proxy.clone())
                .build()
                .unwrap(),
        ),
        proxy,
    })
}

fn read_proxy_from_env() -> Option<Uri> {
    const ENV_VARS: &[&str] = &[
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ];

    for var in ENV_VARS {
        if let Ok(env) = std::env::var(var) {
            return env.parse::<Uri>().ok();
        }
    }

    None
}

impl HttpClient for isahc::HttpClient {
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        _follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        let client = self.clone();
        Box::pin(async move { client.send_async(req).await })
    }

    fn proxy(&self) -> Option<&Uri> {
        None
    }
}

#[cfg(feature = "test-support")]
type FakeHttpHandler = Box<
    dyn Fn(Request<HttpBody>) -> BoxFuture<'static, Result<Response<HttpBody>, Error>>
        + Send
        + Sync
        + 'static,
>;

#[cfg(feature = "test-support")]
pub struct FakeHttpClient {
    handler: FakeHttpHandler,
}

#[cfg(feature = "test-support")]
impl FakeHttpClient {
    pub fn create<Fut, F>(handler: F) -> Arc<HttpClientWithUrl>
    where
        Fut: futures::Future<Output = Result<Response<HttpBody>, Error>> + Send + 'static,
        F: Fn(Request<HttpBody>) -> Fut + Send + Sync + 'static,
    {
        Arc::new(HttpClientWithUrl {
            base_url: Mutex::new("http://test.example".into()),
            client: HttpClientWithProxy {
                client: Arc::new(Self {
                    handler: Box::new(move |req| Box::pin(handler(req))),
                }),
                proxy: None,
            },
        })
    }

    pub fn with_404_response() -> Arc<HttpClientWithUrl> {
        Self::create(|_| async move {
            Ok(Response::builder()
                .status(404)
                .body(Default::default())
                .unwrap())
        })
    }

    pub fn with_200_response() -> Arc<HttpClientWithUrl> {
        Self::create(|_| async move {
            Ok(Response::builder()
                .status(200)
                .body(Default::default())
                .unwrap())
        })
    }
}

#[cfg(feature = "test-support")]
impl fmt::Debug for FakeHttpClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FakeHttpClient").finish()
    }
}

#[cfg(feature = "test-support")]
impl HttpClient for FakeHttpClient {
    fn send_with_redirect_policy(
        &self,
        req: Request<HttpBody>,
        _follow_redirects: bool,
    ) -> BoxFuture<'static, Result<Response<HttpBody>, Error>> {
        let future = (self.handler)(req);
        Box::pin(async move { future.await.map(Into::into) })
    }

    fn proxy(&self) -> Option<&Uri> {
        None
    }
}
