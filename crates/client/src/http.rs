pub use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use isahc::{
    config::{Configurable, RedirectPolicy},
    AsyncBody,
};
pub use isahc::{
    http::{Method, Uri},
    Error,
};
use std::sync::Arc;
pub use url::Url;

pub type Request = isahc::Request<AsyncBody>;
pub type Response = isahc::Response<AsyncBody>;

pub trait HttpClient: Send + Sync {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response, Error>>;

    fn get<'a>(
        &'a self,
        uri: &str,
        body: AsyncBody,
        follow_redirects: bool,
    ) -> BoxFuture<'a, Result<Response, Error>> {
        self.send(
            isahc::Request::builder()
                .redirect_policy(if follow_redirects {
                    RedirectPolicy::Follow
                } else {
                    RedirectPolicy::None
                })
                .method(Method::GET)
                .uri(uri)
                .body(body)
                .unwrap(),
        )
    }
}

pub fn client() -> Arc<dyn HttpClient> {
    Arc::new(isahc::HttpClient::builder().build().unwrap())
}

impl HttpClient for isahc::HttpClient {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response, Error>> {
        Box::pin(async move { self.send_async(req).await })
    }
}
