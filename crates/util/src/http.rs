use crate::http_proxy_from_env;
pub use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use isahc::config::{Configurable, RedirectPolicy};
pub use isahc::{
    http::{Method, StatusCode, Uri},
    Error,
};
pub use isahc::{AsyncBody, Request, Response};
use parking_lot::Mutex;
use smol::future::FutureExt;
#[cfg(feature = "test-support")]
use std::fmt;
use std::{sync::Arc, time::Duration};
pub use url::Url;

pub struct ZedHttpClient {
    pub zed_host: Mutex<String>,
    client: Box<dyn HttpClient>,
}

impl ZedHttpClient {
    pub fn zed_url(&self, path: &str) -> String {
        format!("{}{}", self.zed_host.lock(), path)
    }

    pub fn zed_api_url(&self, path: &str) -> String {
        let zed_host = self.zed_host.lock().clone();

        let host = match zed_host.as_ref() {
            "https://zed.dev" => "https://api.zed.dev",
            "https://staging.zed.dev" => "https://api-staging.zed.dev",
            "http://localhost:3000" => "http://localhost:8080",
            other => other,
        };

        format!("{}{}", host, path)
    }
}

impl HttpClient for Arc<ZedHttpClient> {
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }
}

impl HttpClient for ZedHttpClient {
    fn send(&self, req: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>, Error>> {
        self.client.send(req)
    }
}

pub fn zed_client(zed_host: &str) -> Arc<ZedHttpClient> {
    Arc::new(ZedHttpClient {
        zed_host: Mutex::new(zed_host.to_string()),
        client: Box::new(
            isahc::HttpClient::builder()
                .connect_timeout(Duration::from_secs(5))
                .low_speed_timeout(100, Duration::from_secs(5))
                .proxy(http_proxy_from_env())
                .build()
                .unwrap(),
        ),
    })
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
pub struct FakeHttpClient {
    handler: Box<
        dyn 'static
            + Send
            + Sync
            + Fn(Request<AsyncBody>) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>>,
    >,
}

#[cfg(feature = "test-support")]
impl FakeHttpClient {
    pub fn create<Fut, F>(handler: F) -> Arc<ZedHttpClient>
    where
        Fut: 'static + Send + futures::Future<Output = Result<Response<AsyncBody>, Error>>,
        F: 'static + Send + Sync + Fn(Request<AsyncBody>) -> Fut,
    {
        Arc::new(ZedHttpClient {
            zed_host: Mutex::new("http://test.example".into()),
            client: Box::new(Self {
                handler: Box::new(move |req| Box::pin(handler(req))),
            }),
        })
    }

    pub fn with_404_response() -> Arc<ZedHttpClient> {
        Self::create(|_| async move {
            Ok(Response::builder()
                .status(404)
                .body(Default::default())
                .unwrap())
        })
    }

    pub fn with_200_response() -> Arc<ZedHttpClient> {
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
