pub mod http;
pub mod sse;
mod stdio_transport;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::Stream;
use http_client::HttpClient;
use std::{pin::Pin, sync::Arc};
use url::Url;

pub use self::http::*;
pub use self::sse::*;
pub use stdio_transport::*;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;
    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}

pub fn build_transport(
    http_client: Arc<dyn HttpClient>,
    endpoint: &Url,
) -> Result<Arc<dyn Transport>> {
    match endpoint.scheme() {
        "http" | "https" => Ok(Arc::new(HttpTransport::new(
            http_client,
            endpoint.to_string(),
        ))),
        "sse" => Ok(Arc::new(SseTransport::new(
            http_client,
            endpoint.to_string(),
        ))),
        _ => Err(anyhow!("unsupported scheme {}", endpoint.scheme())),
    }
}
