pub mod http;
mod stdio_transport;

use anyhow::Result;
use async_trait::async_trait;
use futures::{FutureExt as _, Stream, channel::oneshot, select};
use std::pin::Pin;

use crate::oauth::WwwAuthenticate;

pub use http::*;
pub use stdio_transport::*;

#[derive(Clone, Debug)]
pub enum TransportShutdownReason {
    AuthRequired(WwwAuthenticate),
    SessionExpired,
    Disconnected,
    Other,
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;

    /// Implementations must resolve `issued_tx` immediately before attempting
    /// the transport send, or drop it if the message is never issued.
    async fn send_cancellable(
        &self,
        message: String,
        cancel_rx: Option<oneshot::Receiver<()>>,
        issued_tx: Option<oneshot::Sender<()>>,
    ) -> Result<()> {
        if let Some(issued_tx) = issued_tx
            && issued_tx.send(()).is_err()
        {
            log::trace!("context server request-issued receiver was dropped");
        }
        let Some(cancel_rx) = cancel_rx else {
            return self.send(message).await;
        };
        let mut send = std::pin::pin!(self.send(message).fuse());
        let mut cancel_rx = std::pin::pin!(cancel_rx.fuse());
        select! {
            result = send => result,
            _ = cancel_rx => Ok(()),
        }
    }

    fn supports_concurrent_sends(&self) -> bool {
        false
    }

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

    fn shutdown_reason(&self) -> TransportShutdownReason {
        self.auth_challenge()
            .map(TransportShutdownReason::AuthRequired)
            .unwrap_or(TransportShutdownReason::Other)
    }
}
