pub mod http;
mod stdio_transport;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

pub use http::*;
pub use stdio_transport::*;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;
    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}
