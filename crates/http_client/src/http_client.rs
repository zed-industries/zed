mod async_body;
pub mod github;

pub use anyhow::{Result, anyhow};
pub use async_body::{AsyncBody, Inner};
use derive_more::Deref;
pub use http::{self, Method, Request, Response, StatusCode, Uri};

use futures::future::BoxFuture;
use http::request::Builder;
#[cfg(feature = "test-support")]
use std::fmt;
use std::{
    any::type_name,
    sync::{Arc, Mutex},
};
pub use url::Url;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RedirectPolicy {
    #[default]
    NoFollow,
    FollowLimit(u32),
    FollowAll,
}
pub struct FollowRedirects(pub bool);

pub trait HttpRequestExt {
    /// Whether or not to follow redirects
    fn follow_redirects(self, follow: RedirectPolicy) -> Self;
}

impl HttpRequestExt for http::request::Builder {
    fn follow_redirects(self, follow: RedirectPolicy) -> Self {
        self.extension(follow)
    }
}

pub trait HttpClient: 'static + Send + Sync {
    fn type_name(&self) -> &'static str;

    fn send(
        &self,
        req: http::Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>>;

    fn get<'a>(
        &'a self,
        uri: &str,
        body: AsyncBody,
        follow_redirects: bool,
    ) -> BoxFuture<'a, Result<Response<AsyncBody>, anyhow::Error>> {
        let request = Builder::new()
            .uri(uri)
            .follow_redirects(if follow_redirects {
                RedirectPolicy::FollowAll
            } else {
                RedirectPolicy::NoFollow
            })
            .body(body);

        match request {
            Ok(request) => Box::pin(async move { self.send(request).await }),
            Err(e) => Box::pin(async move { Err(e.into()) }),
        }
    }

    fn post_json<'a>(
        &'a self,
        uri: &str,
        body: AsyncBody,
    ) -> BoxFuture<'a, Result<Response<AsyncBody>, anyhow::Error>> {
        let request = Builder::new()
            .uri(uri)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(body);

        match request {
            Ok(request) => Box::pin(async move { self.send(request).await }),
            Err(e) => Box::pin(async move { Err(e.into()) }),
        }
    }

    fn proxy(&self) -> Option<&Url>;
}

/// An [`HttpClient`] that may have a proxy.
#[derive(Deref)]
pub struct HttpClientWithProxy {
    #[deref]
    client: Arc<dyn HttpClient>,
    proxy: Option<Url>,
}

impl HttpClientWithProxy {
    /// Returns a new [`HttpClientWithProxy`] with the given proxy URL.
    pub fn new(client: Arc<dyn HttpClient>, proxy_url: Option<String>) -> Self {
        let proxy_url = proxy_url
            .and_then(|proxy| proxy.parse().ok())
            .or_else(read_proxy_from_env);

        Self::new_url(client, proxy_url)
    }
    pub fn new_url(client: Arc<dyn HttpClient>, proxy_url: Option<Url>) -> Self {
        Self {
            client,
            proxy: proxy_url,
        }
    }
}

impl HttpClient for HttpClientWithProxy {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&Url> {
        self.proxy.as_ref()
    }

    fn type_name(&self) -> &'static str {
        self.client.type_name()
    }
}

impl HttpClient for Arc<HttpClientWithProxy> {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&Url> {
        self.proxy.as_ref()
    }

    fn type_name(&self) -> &'static str {
        self.client.type_name()
    }
}

/// An [`HttpClient`] that has a base URL.
pub struct HttpClientWithUrl {
    base_url: Mutex<String>,
    client: HttpClientWithProxy,
}

impl std::ops::Deref for HttpClientWithUrl {
    type Target = HttpClientWithProxy;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl HttpClientWithUrl {
    /// Returns a new [`HttpClientWithUrl`] with the given base URL.
    pub fn new(
        client: Arc<dyn HttpClient>,
        base_url: impl Into<String>,
        proxy_url: Option<String>,
    ) -> Self {
        let client = HttpClientWithProxy::new(client, proxy_url);

        Self {
            base_url: Mutex::new(base_url.into()),
            client,
        }
    }

    pub fn new_url(
        client: Arc<dyn HttpClient>,
        base_url: impl Into<String>,
        proxy_url: Option<Url>,
    ) -> Self {
        let client = HttpClientWithProxy::new_url(client, proxy_url);

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

    /// Builds a Zed LLM URL using the given path.
    pub fn build_zed_llm_url(&self, path: &str, query: &[(&str, &str)]) -> Result<Url> {
        let base_url = self.base_url();
        let base_api_url = match base_url.as_ref() {
            "https://zed.dev" => "https://llm.zed.dev",
            "https://staging.zed.dev" => "https://llm-staging.zed.dev",
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
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&Url> {
        self.client.proxy.as_ref()
    }

    fn type_name(&self) -> &'static str {
        self.client.type_name()
    }
}

impl HttpClient for HttpClientWithUrl {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&Url> {
        self.client.proxy.as_ref()
    }

    fn type_name(&self) -> &'static str {
        self.client.type_name()
    }
}

pub fn read_proxy_from_env() -> Option<Url> {
    const ENV_VARS: &[&str] = &[
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ];

    ENV_VARS
        .iter()
        .find_map(|var| std::env::var(var).ok())
        .and_then(|env| env.parse().ok())
}

pub struct BlockedHttpClient;

impl BlockedHttpClient {
    pub fn new() -> Self {
        BlockedHttpClient
    }
}

impl HttpClient for BlockedHttpClient {
    fn send(
        &self,
        _req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>> {
        Box::pin(async {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "BlockedHttpClient disallowed request",
            )
            .into())
        })
    }

    fn proxy(&self) -> Option<&Url> {
        None
    }

    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

#[cfg(feature = "test-support")]
type FakeHttpHandler = Box<
    dyn Fn(Request<AsyncBody>) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>>
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
        Fut: futures::Future<Output = Result<Response<AsyncBody>, anyhow::Error>> + Send + 'static,
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
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>> {
        let future = (self.handler)(req);
        future
    }

    fn proxy(&self) -> Option<&Url> {
        None
    }

    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}
