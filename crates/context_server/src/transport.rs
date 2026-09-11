pub mod http;
mod stdio_transport;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::oauth::WwwAuthenticate;

pub use http::*;
pub use stdio_transport::*;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;
    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;

    /// Called after the MCP initialize handshake completes so transports that
    /// need the negotiated version (currently only HTTP, which must attach an
    /// `MCP-Protocol-Version` header from 2025-06-18 onward) can pick it up.
    fn set_protocol_version(&self, _version: &str) {}

    /// The authentication challenge from the last `401 Unauthorized` response
    /// this transport gave up on, if any (currently only set by the HTTP
    /// transport).
    ///
    /// The challenge is recorded right before the failed send tears down the
    /// client's output loop. Observers of the client's shutdown read it from
    /// here, so a 401 can initiate the OAuth flow even when it arrived on a
    /// notification, with no request in flight to carry a typed error.
    fn auth_challenge(&self) -> Option<WwwAuthenticate> {
        None
    }
}
