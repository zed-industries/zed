//! Error handling patterns in agent-client-protocol.
//!
//! This chapter explains how errors work in agent-client-protocol callbacks and the difference
//! between *protocol errors* (sent to the peer) and *connection errors* (which
//! shut down the connection).
//!
//! # Callback Return Types
//!
//! Almost all agent-client-protocol callbacks return `Result<_, crate::Error>`.
//! What happens when you return an `Err` depends on the context. An incoming
//! request-handler error is sent to the peer as an Error Response, while an
//! incoming notification-handler error is logged without a reply. Errors from
//! connection-lifecycle callbacks can shut down the connection.
//!
//! # Sending Protocol Errors
//!
//! To choose an error response explicitly while handling a request, use the
//! request context's `respond` method:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::{ValidateRequest, ValidateResponse};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! Client.builder()
//!     .on_receive_request(async |request: ValidateRequest, responder, _cx| {
//!         if request.data.is_empty() {
//!             // Send error to peer, keep connection alive
//!             responder.respond_with_error(agent_client_protocol::Error::invalid_params())?;
//!             return Ok(());
//!         }
//!
//!         // Process valid request...
//!         responder.respond(ValidateResponse { is_valid: true, error: None })?;
//!         Ok(())
//!     }, agent_client_protocol::on_receive_request!())
//! #   .connect_with(transport, async |_| Ok(())).await?;
//! # Ok(())
//! # }
//! ```
//!
//! JSON-RPC notifications are one-way and cannot receive success or error replies.
//! The SDK logs notification parse and handler errors without answering them. If an
//! application needs to report a one-way failure, define a notification method for
//! that purpose.
//!
//! Malformed JSON and invalid request envelopes are handled at the SDK's transport
//! boundary. Applications do not need to construct uncorrelated Error Responses.
//!
//! # The `into_internal_error` Helper
//!
//! When working with external libraries that return their own error types,
//! you need to convert them to `agent_client_protocol::Error`. The
//! [`Error::into_internal_error`][crate::Error::into_internal_error] method
//! provides a convenient way to do this:
//!
//! ```
//! use agent_client_protocol::Error;
//!
//! # fn example() -> Result<(), agent_client_protocol::Error> {
//! # let data = "hello";
//! # let path = "/tmp/test.txt";
//! // Convert any error type to agent_client_protocol::Error
//! let value = serde_json::to_value(&data)
//!     .map_err(Error::into_internal_error)?;
//!
//! // Or with a file operation
//! let contents = std::fs::read_to_string(path)
//!     .map_err(Error::into_internal_error);
//! # Ok(())
//! # }
//! ```
//!
//! This wraps the original error's message in an internal error, which is
//! appropriate for unexpected failures. For expected error conditions that
//! should be communicated to the peer, create specific error types instead.
//!
//! # Error Types
//!
//! The [`Error`][crate::Error] type provides factory methods for common
//! JSON-RPC error codes:
//!
//! - [`Error::parse_error()`][crate::Error::parse_error] - Invalid JSON
//! - [`Error::invalid_request()`][crate::Error::invalid_request] - Malformed request
//! - [`Error::method_not_found()`][crate::Error::method_not_found] - Unknown method
//! - [`Error::invalid_params()`][crate::Error::invalid_params] - Bad parameters
//! - [`Error::internal_error()`][crate::Error::internal_error] - Server error
//!
//! You can add context with `.data()`:
//!
//! ```
//! let error = agent_client_protocol::Error::invalid_params()
//!     .data(serde_json::json!({
//!         "field": "timeout",
//!         "reason": "must be positive"
//!     }));
//! ```
//!
//! # Summary
//!
//! | Situation | What to do |
//! |-----------|------------|
//! | Send error response to request | `responder.respond_with_error(error)` |
//! | Handle notification failure | Log or return the error; the SDK sends no reply |
//! | Fail connection lifecycle work | Return `Err(error)` from a lifecycle callback |
//! | Convert external error | `.map_err(Error::into_internal_error)?` |
