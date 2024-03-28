use crate::http_proxy_from_env;
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

/// An [`HttpClient`] that has a base URL.
pub struct HttpClientWithUrl {
    base_url: Mutex<String>,
    client: Arc<dyn HttpClient>,
}

impl HttpClientWithUrl {
    /// Returns a new [`HttpClientWithUrl`] with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Mutex::new(base_url.into()),
            client: client(),
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
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }
}

impl HttpClient for HttpClientWithUrl {
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }
}

pub trait HttpClient: Send + Sync {
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>>;

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
}

pub fn client() -> Arc<dyn HttpClient> {
    Arc::new(
        isahc::HttpClient::builder()
            .connect_timeout(Duration::from_secs(5))
            .low_speed_timeout(100, Duration::from_secs(5))
            .proxy(http_proxy_from_env())
            .build()
            .unwrap(),
    )
}

impl HttpClient for isahc::HttpClient {
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>> {
        Box::pin(async move { self.send_async(req).await })
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
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>> {
        let future = (self.handler)(req);
        Box::pin(async move { future.await.map(Into::into) })
    }
}
