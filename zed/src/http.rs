pub use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use std::sync::Arc;
pub use surf::{
    http::{Method, Request, Response as ServerResponse},
    Response, Url,
};

pub trait HttpClient: Send + Sync {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response>>;
}

pub fn client() -> Arc<dyn HttpClient> {
    Arc::new(surf::client())
}

impl HttpClient for surf::Client {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response>> {
        Box::pin(async move {
            Ok(self
                .send(req)
                .await
                .map_err(|e| anyhow!("http request failed: {}", e))?)
        })
    }
}
