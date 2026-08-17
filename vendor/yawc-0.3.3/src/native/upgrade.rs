//! WebSocket upgrade handling for server-side connections.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use hyper_util::rt::TokioIo;
use pin_project::pin_project;
use sha1::{Digest, Sha1};

use bytes::Bytes;

use super::{HttpStream, Negotiation, Role, WebSocket};

#[cfg(feature = "axum")]
use {
    super::{Options, MAX_PAYLOAD_READ, MAX_READ_BUFFER},
    crate::{compression::WebSocketExtensions, Result},
    http_body_util::Empty,
    hyper::{header, Response},
    std::future::Future,
};

/// Represents an incoming WebSocket upgrade request that can be converted into a WebSocket connection.
///
/// Available when the `axum` feature is enabled, this struct integrates with the axum web framework
/// to handle WebSocket upgrades in axum route handlers.
///
/// # Example
/// ```rust
/// use axum::{
///     routing::get,
///     response::IntoResponse,
/// };
/// use yawc::{IncomingUpgrade, CompressionLevel, Options};
///
/// struct AppState;
///
/// async fn ws_handler(
///     ws: IncomingUpgrade,
///     state: AppState,
/// ) -> impl IntoResponse {
///     let options = Options::default()
///         .with_compression_level(CompressionLevel::best());
///
///     let (response, fut) = ws.upgrade(options).unwrap();
///
///     // Spawn handler for upgraded WebSocket connection
///     tokio::spawn(async move {
///         let ws = fut.await.unwrap();
///         // Handle WebSocket connection
///     });
///
///     response
/// }
/// ```
///
/// # Compression Support
/// Supports per-message compression via the `permessage-deflate` extension when enabled.
/// The compression settings can be configured through `Options` during upgrade.
///
/// # Protocol
/// - Handles `Sec-WebSocket-Key` validation
/// - Manages protocol switching from HTTP to WebSocket
/// - Negotiates extensions like compression
/// - Returns response headers for upgrade handshake
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
#[cfg(feature = "axum")]
pub struct IncomingUpgrade {
    /// The Sec-WebSocket-Key header value from the client. This value is used for the WebSocket
    /// handshake as required by RFC 6455.
    key: String,

    /// The Hyper upgrade future used to complete the protocol switch from HTTP to WebSocket.
    /// This handles the low-level protocol transition after handshake is complete.
    on_upgrade: hyper::upgrade::OnUpgrade,

    /// Optional WebSocket extensions negotiated with the client, such as permessage-deflate compression.
    /// These configure protocol-level behavior if enabled.
    extensions: Option<WebSocketExtensions>,
}

#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
#[cfg(feature = "axum")]
impl IncomingUpgrade {
    /// Upgrades an HTTP request to a WebSocket connection with the given options
    ///
    /// Creates an HTTP response for the upgrade and constructs the upgrade future
    /// that will complete the protocol switch. Handles negotiation of extensions
    /// like compression between client and server capabilities.
    ///
    /// # Parameters
    /// - `options`: Configuration options for the WebSocket connection like compression settings
    ///
    /// # Returns
    /// A tuple containing:
    /// - The HTTP upgrade response to send to the client
    /// - An upgrade future that completes into a WebSocket connection
    ///
    /// # Examples
    /// ```rust
    /// use axum::{response::IntoResponse, extract::State};
    /// use yawc::{IncomingUpgrade, Options};
    ///
    /// async fn handler(
    ///     ws: IncomingUpgrade,
    ///     State(state): State<()>,
    /// ) -> impl IntoResponse {
    ///     let options = Options::default();
    ///     let (response, upgrade) = ws.upgrade(options).unwrap();
    ///
    ///     tokio::spawn(async move {
    ///         let websocket = upgrade.await.unwrap();
    ///         // Handle WebSocket connection
    ///     });
    ///
    ///     response
    /// }
    /// ```
    ///
    /// # Protocol Details
    /// The upgrade process:
    /// 1. Creates upgrade response with standard WebSocket headers
    /// 2. Negotiates any extensions like compression
    /// 3. Returns response to be sent to client
    /// 4. UpgradeFut completes after response sent
    ///
    /// When compression is enabled:
    /// - Client and server negotiate compression settings
    /// - Per-message compression parameters are synchronized
    /// - Compression context is initialized after upgrade
    pub fn upgrade(self, options: Options) -> Result<(Response<Empty<Bytes>>, UpgradeFut)> {
        let builder = Response::builder()
            .status(hyper::StatusCode::SWITCHING_PROTOCOLS)
            .header(header::CONNECTION, "upgrade")
            .header(header::UPGRADE, "websocket")
            .header(header::SEC_WEBSOCKET_ACCEPT, self.key);

        let (builder, extensions) = match (self.extensions, options.compression.as_ref()) {
            (Some(client_offer), Some(server_offer)) => {
                let offer = server_offer.merge(&client_offer);
                let response = builder.header(header::SEC_WEBSOCKET_EXTENSIONS, offer.to_string());
                (response, Some(offer))
            }
            _ => (builder, None),
        };

        let response = builder
            .body(Empty::new())
            .expect("bug: failed to build response");

        // max read buffer should be at least 2 times the payload read if not specified
        let max_read_buffer = options.max_read_buffer.unwrap_or(
            options
                .max_payload_read
                .map(|payload_read| payload_read * 2)
                .unwrap_or(MAX_READ_BUFFER),
        );

        let stream = UpgradeFut {
            inner: self.on_upgrade,
            negotiation: Some(Negotiation {
                extensions,
                compression_level: options
                    .compression
                    .as_ref()
                    .map(|compression| compression.level),
                max_payload_read: options.max_payload_read.unwrap_or(MAX_PAYLOAD_READ),
                max_backpressure_write_boundary: options.max_backpressure_write_boundary,
                fragmentation: options.fragmentation.clone(),
                max_read_buffer,
                utf8: options.check_utf8,
            }),
        };

        Ok((response, stream))
    }
}

/// Implementation of extracting a WebSocket upgrade from an Axum request.
///
/// This implementation allows the `IncomingUpgrade` to be used as an extractor in axum route handlers,
/// performing the necessary validations and setup for a WebSocket upgrade. It verifies required headers
/// like `Sec-WebSocket-Key` and `Sec-WebSocket-Version`, and extracts any offered WebSocket extensions.
///
/// # Errors
/// Returns `StatusCode::BAD_REQUEST` if:
/// - The `Sec-WebSocket-Key` header is missing
/// - The `Sec-WebSocket-Version` is not "13"
/// - The request is not an upgrade request
///
/// # Example
/// ```rust
/// use axum::routing::get;
/// use axum::response::IntoResponse;
/// use yawc::IncomingUpgrade;
///
/// async fn ws_upgrade(ws: yawc::IncomingUpgrade) -> impl IntoResponse {
///     // handle
/// }
///
/// let app: axum::Router<()> = axum::Router::new()
///     .route("/ws", get(ws_upgrade));
/// ```
#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> axum_core::extract::FromRequestParts<S> for IncomingUpgrade
where
    S: Sync,
{
    type Rejection = hyper::StatusCode;

    /// Extracts WebSocket upgrade parameters from HTTP request parts
    ///
    /// Validates WebSocket protocol requirements and extracts:
    /// - The client's Sec-WebSocket-Key
    /// - The protocol version (must be 13)
    /// - Any extension offers from the client
    /// - The upgrade future from Hyper
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = std::result::Result<Self, Self::Rejection>> + Send {
        use std::str::FromStr;

        async move {
            let key = parts
                .headers
                .get(header::SEC_WEBSOCKET_KEY)
                .ok_or(http::StatusCode::BAD_REQUEST)?;

            if parts
                .headers
                .get(header::SEC_WEBSOCKET_VERSION)
                .map(|v| v.as_bytes())
                != Some(b"13")
            {
                return Err(hyper::StatusCode::BAD_REQUEST);
            }

            let extensions = parts
                .headers
                .get(header::SEC_WEBSOCKET_EXTENSIONS)
                .and_then(|h| h.to_str().ok())
                .map(WebSocketExtensions::from_str)
                .and_then(std::result::Result::ok);

            let on_upgrade = parts
                .extensions
                .remove::<hyper::upgrade::OnUpgrade>()
                .ok_or(hyper::StatusCode::BAD_REQUEST)?;

            Ok(Self {
                on_upgrade,
                extensions,
                key: sec_websocket_protocol(key.as_bytes()),
            })
        }
    }
}

pub(super) fn sec_websocket_protocol(key: &[u8]) -> String {
    use base64::prelude::*;
    let mut sha1 = Sha1::new();
    sha1.update(key);
    sha1.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"); // magic string
    let result = sha1.finalize();
    BASE64_STANDARD.encode(&result[..])
}

/// Future that completes the WebSocket upgrade process on a server, returning a WebSocket stream.
///
/// This future is returned by the [`WebSocket::upgrade`](super::WebSocket::upgrade) and
/// [`WebSocket::upgrade_with_options`](super::WebSocket::upgrade_with_options) functions after initiating
/// a WebSocket protocol upgrade. It manages completion of the HTTP upgrade handshake and initializes
/// a WebSocket connection with the negotiated parameters.
///
/// # Important
/// The associated HTTP upgrade response must be sent to the client before polling this future.
/// The future will not complete until the response is sent and the HTTP connection is upgraded
/// to the WebSocket protocol.
///
/// # Example
/// ```no_run
/// use hyper::{
///     body::{Bytes, Incoming},
///     server::conn::http1,
///     service::service_fn,
///     Request, Response,
/// };
/// use http_body_util::Empty;
/// use yawc::{Result, WebSocket, UpgradeFut, Options};
///
/// async fn handle_client(fut: UpgradeFut) -> yawc::Result<()> {
///     let ws = fut.await?;
///     // use `ws`
///     Ok(())
/// }
///
/// async fn server_upgrade(mut req: Request<Incoming>) -> yawc::Result<Response<Empty<Bytes>>> {
///     let (response, fut) = WebSocket::upgrade_with_options(
///         &mut req,
///         Options::default()
///     )?;
///
///     tokio::task::spawn(async move {
///         if let Err(e) = handle_client(fut).await {
///             eprintln!("Error in websocket connection: {}", e);
///         }
///     });
///
///     Ok(response)
/// }
/// ```
///
/// # Fields
/// - `inner`: The underlying hyper upgrade future that completes the protocol switch
/// - `negotiation`: Parameters negotiated during the upgrade, like compression settings
#[pin_project]
#[derive(Debug)]
pub struct UpgradeFut {
    #[pin]
    pub(super) inner: hyper::upgrade::OnUpgrade,
    pub(super) negotiation: Option<Negotiation>,
}

impl std::future::Future for UpgradeFut {
    type Output = hyper::Result<WebSocket<HttpStream>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let upgraded = match this.inner.poll(cx) {
            Poll::Ready(x) => x,
            Poll::Pending => return Poll::Pending,
        };

        let io = TokioIo::new(upgraded?);
        let negotiation = this.negotiation.take().unwrap();

        Poll::Ready(Ok(WebSocket::new(
            Role::Server,
            HttpStream::from(io),
            Bytes::new(),
            negotiation,
        )))
    }
}
