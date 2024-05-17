pub mod github;

pub use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use futures_lite::FutureExt;
use isahc::config::{Configurable, RedirectPolicy};
pub use isahc::{
    http::{Method, StatusCode, Uri},
    AsyncBody, Error, HttpClient as IsahcHttpClient, Request, Response,
};
#[cfg(feature = "test-support")]
use std::fmt;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
pub use url::Url;

fn get_proxy(proxy: Option<String>) -> Option<isahc::http::Uri> {
    macro_rules! try_env {
        ($($env:literal),+) => {
            $(
                if let Ok(env) = std::env::var($env) {
                    return env.parse::<isahc::http::Uri>().ok();
                }
            )+
        };
    }

    proxy
        .and_then(|input| {
            input
                .parse::<isahc::http::Uri>()
                .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                .ok()
        })
        .or_else(|| {
            try_env!(
                "ALL_PROXY",
                "all_proxy",
                "HTTPS_PROXY",
                "https_proxy",
                "HTTP_PROXY",
                "http_proxy"
            );
            None
        })
}

/// An [`HttpClient`] that has a base URL.
pub struct HttpClientWithUrl {
    base_url: Mutex<String>,
    client: Arc<dyn HttpClient>,
    proxy: Option<String>,
}

impl HttpClientWithUrl {
    /// Returns a new [`HttpClientWithUrl`] with the given base URL.
    pub fn new(base_url: impl Into<String>, unparsed_proxy: Option<String>) -> Self {
        let parsed_proxy = get_proxy(unparsed_proxy);
        let proxy_string = parsed_proxy.as_ref().map(|p| {
            // Map proxy settings from `http://localhost:10809` to `http://127.0.0.1:10809`
            // NodeRuntime without environment information can not parse `localhost`
            // correctly.
            // TODO: map to `[::1]` if we are using ipv6
            p.to_string()
                .to_ascii_lowercase()
                .replace("localhost", "127.0.0.1")
        });
        Self {
            base_url: Mutex::new(base_url.into()),
            client: client(parsed_proxy),
            proxy: proxy_string,
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

    fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }
}

impl HttpClient for HttpClientWithUrl {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }

    fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }
}

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

    fn proxy(&self) -> Option<&str>;
}

pub fn client(proxy: Option<isahc::http::Uri>) -> Arc<dyn HttpClient> {
    Arc::new(
        isahc::HttpClient::builder()
            .connect_timeout(Duration::from_secs(5))
            .low_speed_timeout(100, Duration::from_secs(5))
            .proxy(proxy)
            .build()
            .unwrap(),
    )
}

impl HttpClient for isahc::HttpClient {
    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
        let client = self.clone();
        Box::pin(async move { client.send_async(req).await })
    }

    fn proxy(&self) -> Option<&str> {
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
            client: Arc::new(Self {
                handler: Box::new(move |req| Box::pin(handler(req))),
            }),
            proxy: None,
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

    fn proxy(&self) -> Option<&str> {
        None
    }
}
