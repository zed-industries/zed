pub mod github;

pub use anyhow::{anyhow, Result};
use derive_more::Deref;
use futures::{future::BoxFuture, AsyncRead};
use futures_lite::FutureExt;
use isahc::config::{Configurable, RedirectPolicy};
pub use isahc::{
    http::{Method, StatusCode, Uri},
    AsyncBody, Error, HttpClient as IsahcHttpClient, Request, Response,
};
#[cfg(feature = "test-support")]
use std::fmt;
use std::{
    borrow::Cow,
    io::{Cursor, Read},
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};
pub use url::Url;

// A type to implement Read on the inner dyn Read
struct UreqReader {
    reader: Box<dyn Read + Send + 'static>,
}

impl Read for UreqReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

pub struct NewResponse(smol::Unblock<UreqReader>);

impl From<ureq::Response> for NewResponse {
    fn from(response: ureq::Response) -> Self {
        NewResponse(smol::Unblock::new(UreqReader {
            reader: response.into_reader(),
        }))
    }
}

impl AsyncRead for NewResponse {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        // SAFETY: Standard pin projection
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        inner.poll_read(cx, buf)
    }
}

/// Based on the implementation of AsyncBody in
/// https://github.com/sagebind/isahc/blob/5c533f1ef4d6bdf1fd291b5103c22110f41d0bf0/src/body/mod.rs
pub enum NewBody {
    /// An empty body.
    Empty,

    /// A body stored in memory.
    SyncReader(std::io::Cursor<Cow<'static, [u8]>>),

    /// An asynchronous reader.
    AsyncReader(smol::io::BlockOn<Pin<Box<dyn AsyncRead + Send + Sync>>>),
}

impl std::io::Read for NewBody {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            NewBody::Empty => Ok(0),
            NewBody::SyncReader(cursor) => cursor.read(buf),
            NewBody::AsyncReader(async_reader) => async_reader.read(buf),
        }
    }
}

impl Default for NewBody {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<()> for NewBody {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}

impl From<Vec<u8>> for NewBody {
    fn from(body: Vec<u8>) -> Self {
        Self::SyncReader(Cursor::new(Cow::Owned(body)))
    }
}

impl From<&'_ [u8]> for NewBody {
    fn from(body: &[u8]) -> Self {
        body.to_vec().into()
    }
}

impl From<String> for NewBody {
    fn from(body: String) -> Self {
        body.into_bytes().into()
    }
}

impl From<&'_ str> for NewBody {
    fn from(body: &str) -> Self {
        body.as_bytes().into()
    }
}

impl<T: Into<Self>> From<Option<T>> for NewBody {
    fn from(body: Option<T>) -> Self {
        match body {
            Some(body) => body.into(),
            None => Self::Empty,
        }
    }
}

// Example of how to do an async request with the blocking IO
// fn post<'a>(uri: &str, body: NewBody) -> BoxFuture<'a, Result<NewResponse, anyhow::Error>> {
//     let post = ureq::post(uri);

//     async move {
//         let response = smol::unblock(move || post.send(body))
//             .await
//             .map_err(|e| anyhow!("Error: {:?}", e))?;
//         Ok(response.into())
//     }
//     .boxed()
// }

pub trait HttpClient: Send + Sync {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>>;

    fn get<'a>(
        &'a self,
        uri: &str,
        body: AsyncBody,
        follow_redirects: bool,
    ) -> BoxFuture<'a, Result<Response<AsyncBody>, Error>> {
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
        body: AsyncBody,
    ) -> BoxFuture<'a, Result<Response<AsyncBody>, Error>> {
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
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&Uri> {
        self.proxy.as_ref()
    }
}

impl HttpClient for Arc<HttpClientWithProxy> {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
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
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&Uri> {
        self.client.proxy.as_ref()
    }
}

impl HttpClient for HttpClientWithUrl {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
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
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        let client = self.clone();
        Box::pin(async move { client.send_async(req).await })
    }

    fn proxy(&self) -> Option<&Uri> {
        None
    }
}

#[cfg(feature = "test-support")]
type FakeHttpHandler = Box<
    dyn Fn(Request<AsyncBody>) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>>
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
        Fut: futures::Future<Output = Result<Response<AsyncBody>, Error>> + Send + 'static,
        F: Fn(Request<AsyncBody>) -> Fut + Send + Sync + 'static,
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
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        let future = (self.handler)(req);
        Box::pin(async move { future.await.map(Into::into) })
    }

    fn proxy(&self) -> Option<&Uri> {
        None
    }
}
