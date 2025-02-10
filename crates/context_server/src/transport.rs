mod sse_transport;
mod stdio_transport;

use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;

pub use sse_transport::*;
pub use stdio_transport::*;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;
    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}
