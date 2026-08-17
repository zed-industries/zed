//! Request cancellation with `$/cancel_request`.
//!
//! The SDK exposes the ACP `$/cancel_request` protocol-level notification:
//! either side may send it to ask the peer to cancel one outstanding JSON-RPC
//! request by ID.
//!
//! Cancellation is **cooperative**. A peer may ignore `$/cancel_request`, may
//! finish with normal data, or may respond to the original request with
//! [`Error::request_cancelled`] (`-32800`). The requesting side always
//! receives a response to the original request; cancellation only changes
//! *which* response that is. Unhandled notifications are ignored by the SDK
//! so peers that do not support cancellation simply will not act on it.
//!
//! # Cancelling outgoing requests
//!
//! To cancel a request sent through [`ConnectionTo::send_request`], keep the
//! returned [`SentRequest`] and call [`cancel`][`SentRequest::cancel`] on it:
//!
//! ```
//! # use agent_client_protocol::{ConnectionTo, Error, UntypedRole};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(cx: ConnectionTo<UntypedRole>) -> Result<(), Error> {
//! let request = cx.send_request(MyRequest {});
//! request.cancel()?;
//!
//! // The peer still responds to the request: with normal data if it raced
//! // ahead, or with the standard cancellation error.
//! let result = request.block_task().await;
//! # let _ = result;
//! # Ok(())
//! # }
//! ```
//!
//! The [`SentRequest`] remembers the peer and any proxy wrapping used for the
//! original request, so this also works for requests sent through
//! [`ConnectionTo::send_request_to`].
//!
//! Dropping a [`SentRequest`] before the SDK receives a response also sends
//! `$/cancel_request`. This covers abandoned request handles and futures. For a
//! request whose eventual response should be ignored, but which should continue
//! running on the peer, call [`detach`][`SentRequest::detach`] instead; the
//! eventual response is discarded, but no cancellation is sent. The peer is
//! still expected to answer the JSON-RPC request eventually; use a notification
//! instead when no response is expected at all. Once the SDK routes a response
//! for the request, automatic cancellation is disarmed: the peer has already
//! answered, even if caller code has not yet consumed the handle with
//! [`block_task`], [`on_receiving_result`], or [`forward_response_to`], and even
//! if a dispatch handler claimed the response.
//!
//! # Handling cancellation of incoming requests
//!
//! For incoming requests, get the request-local cancellation marker from the
//! [`Responder`]. This keeps cancellation handling next to the request work it
//! controls:
//!
//! ```
//! # use agent_client_protocol::{ConnectionTo, Error, Responder, UntypedRole};
//! # use agent_client_protocol_test::{MyRequest, MyResponse};
//! # async fn example(request: MyRequest, responder: Responder<MyResponse>, cx: ConnectionTo<UntypedRole>) -> Result<(), Error> {
//! # async fn run_request(_request: MyRequest) -> Result<MyResponse, Error> { todo!() }
//! let cancellation = responder.cancellation();
//!
//! cx.spawn(async move {
//!     let response = cancellation.run_until_cancelled(run_request(request)).await;
//!     responder.respond_with_result(response)
//! })?;
//! # Ok(())
//! # }
//! ```
//!
//! [`run_until_cancelled`] is the simple path for handlers that should stop
//! work and reply with the standard cancellation error as soon as cancellation
//! is requested; it drops the work future when cancellation wins. If the
//! handler needs cleanup, partial results, or custom cancellation behavior,
//! use [`cancelled`][`RequestCancellation::cancelled`] or
//! [`is_cancelled`][`RequestCancellation::is_cancelled`] directly inside the
//! request work instead.
//!
//! Cancellation markers are only updated when the connection can process the
//! incoming `$/cancel_request` notification. Long-running handlers and ordered
//! [`SentRequest`] callbacks should return quickly and move work into
//! [`ConnectionTo::spawn`] or another task; see the
//! [ordering](super::ordering) chapter.
//!
//! # Proxies
//!
//! When proxying with [`forward_response_to`], the SDK observes the upstream
//! [`Responder`] cancellation marker and forwards cancellation to the
//! downstream request automatically. The downstream response (normal data or a
//! cancellation error) is still forwarded back upstream.
//!
//! Because cancellation propagates per hop this way, the raw notification is
//! never tunneled across hops: [`ConnectionTo::send_proxied_message_to`] drops
//! `$/cancel_request` notifications rather than forwarding a `requestId` that
//! was allocated on a different connection and would be meaningless to the
//! next peer.
//!
//! ## Custom methods on proxies
//!
//! A proxy that intercepts a method with its own handler decides what
//! cancellation means for it. The SDK always records the cancellation on the
//! request's [`Responder`] marker before the handler chain runs; what happens
//! next is up to the handler that owns the request:
//!
//! - **Handle locally**: react to [`Responder::cancellation`] like any
//!   request handler (ignore it, finish early, or respond with
//!   [`Error::request_cancelled`]).
//! - **Forward and propagate**: use [`forward_response_to`], or, when the
//!   forwarding needs custom logic (rewriting the request, post-processing
//!   the result), register the upstream marker explicitly with
//!   [`forward_cancellation_from`] before consuming the handle:
//!
//! ```
//! # use agent_client_protocol::{ConnectionTo, Error, Responder, UntypedRole};
//! # use agent_client_protocol_test::{MyRequest, MyResponse};
//! # async fn example(request: MyRequest, responder: Responder<MyResponse>, backend: ConnectionTo<UntypedRole>) -> Result<(), Error> {
//! backend
//!     .send_request(request)
//!     .forward_cancellation_from(responder.cancellation())
//!     .on_receiving_result(async move |result| {
//!         // Custom result handling before responding upstream.
//!         responder.respond_with_result(result)
//!     })?;
//! # Ok(())
//! # }
//! ```
//!
//! - **Absorb**: consume the handle without registering the marker
//!   ([`on_receiving_result`] or [`block_task`] alone); the upstream marker is
//!   still set, but nothing is sent downstream and the request runs to
//!   completion there.
//! - **Custom routing**: claim the `$/cancel_request` notification itself in a
//!   handler (user handlers run before the generic forwarding fallbacks) and
//!   translate it manually when you control the relevant hop-local request IDs.
//!
//! # Low-level access
//!
//! Register [`CancelRequestNotification`] (or [`ProtocolLevelNotification`])
//! directly only when you need low-level access to cancellation notifications,
//! such as custom routing or protocol tracing:
//!
//! ```
//! # use agent_client_protocol::{ConnectionTo, Error, UntypedRole};
//! use agent_client_protocol::schema::v1::CancelRequestNotification;
//!
//! # fn example() {
//! let builder = UntypedRole.builder().on_receive_notification(
//!     async |cancel: CancelRequestNotification, _cx: ConnectionTo<UntypedRole>| {
//!         // Mark the matching in-flight operation cancelled.
//!         let _request_id = cancel.request_id;
//!         Ok(())
//!     },
//!     agent_client_protocol::on_receive_notification!(),
//! );
//! # let _ = builder;
//! # }
//! ```
//!
//! Such a handler observes cancellation notifications but does not replace
//! the built-in handling: the SDK updates the [`Responder`] cancellation
//! markers for every incoming `$/cancel_request` before the handler chain
//! runs, even when a handler claims the notification.
//!
//! If you are implementing custom routing and already know the JSON-RPC request
//! ID on the peer connection you are targeting, use
//! [`ConnectionTo::send_cancel_request_to`]. Most code should use
//! [`SentRequest::cancel`] instead, because the request handle already knows the
//! correct peer, request ID, and proxy wrapping.
//!
//! [`block_task`]: crate::SentRequest::block_task
//! [`on_receiving_result`]: crate::SentRequest::on_receiving_result
//! [`forward_response_to`]: crate::SentRequest::forward_response_to
//! [`run_until_cancelled`]: crate::RequestCancellation::run_until_cancelled
//! [`RequestCancellation`]: crate::RequestCancellation
//! [`RequestCancellation::cancelled`]: crate::RequestCancellation::cancelled
//! [`RequestCancellation::is_cancelled`]: crate::RequestCancellation::is_cancelled
//! [`ConnectionTo::send_request`]: crate::ConnectionTo::send_request
//! [`ConnectionTo::send_request_to`]: crate::ConnectionTo::send_request_to
//! [`ConnectionTo::send_proxied_message_to`]: crate::ConnectionTo::send_proxied_message_to
//! [`ConnectionTo::spawn`]: crate::ConnectionTo::spawn
//! [`SentRequest`]: crate::SentRequest
//! [`SentRequest::cancel`]: crate::SentRequest::cancel
//! [`SentRequest::detach`]: crate::SentRequest::detach
//! [`forward_cancellation_from`]: crate::SentRequest::forward_cancellation_from
//! [`ConnectionTo::send_cancel_request_to`]: crate::ConnectionTo::send_cancel_request_to
//! [`Responder::cancellation`]: crate::Responder::cancellation
//! [`Responder`]: crate::Responder
//! [`Error::request_cancelled`]: crate::Error::request_cancelled
//! [`CancelRequestNotification`]: crate::schema::v1::CancelRequestNotification
//! [`ProtocolLevelNotification`]: crate::schema::v1::ProtocolLevelNotification
