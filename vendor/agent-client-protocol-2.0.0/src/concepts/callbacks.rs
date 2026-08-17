//! Handling incoming messages with `on_receive_*` callbacks.
//!
//! So far we've seen how to *send* messages. But ACP is bidirectional - the
//! remote peer can also send messages to you. Use callbacks to handle them.
//!
//! # Handling Requests
//!
//! Use `on_receive_request` to handle incoming requests that expect a response:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::{ValidateRequest, ValidateResponse};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! Client.builder()
//!     .on_receive_request(async |req: ValidateRequest, responder, cx| {
//!         // Process the request
//!         let is_valid = req.data.len() > 0;
//!
//!         // Send the response
//!         responder.respond(ValidateResponse { is_valid, error: None })
//!     }, agent_client_protocol::on_receive_request!())
//!     .connect_with(transport, async |cx| { Ok(()) })
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Your callback receives three arguments:
//! - The request payload (e.g., `PermissionRequest`)
//! - A [`Responder`] for sending the response
//! - A [`ConnectionTo`] for sending other messages
//!
//! # Handling Notifications
//!
//! Use `on_receive_notification` for fire-and-forget messages that don't need
//! a response:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::StatusUpdate;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! Client.builder()
//!     .on_receive_notification(async |notif: StatusUpdate, cx| {
//!         println!("Status: {}", notif.message);
//!         Ok(())
//!     }, agent_client_protocol::on_receive_notification!())
//! #   .connect_with(transport, async |_| Ok(())).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # The Request Context
//!
//! The [`Responder`] lets you send a response to the request:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::{MyRequest, MyResponse};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder()
//! #   .on_receive_request(async |req: MyRequest, responder, cx| {
//! // Send a successful response
//! responder.respond(MyResponse { status: "ok".into() })?;
//! # Ok(())
//! #   }, agent_client_protocol::on_receive_request!())
//! #   .connect_with(transport, async |_| Ok(())).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Or send an error:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::{MyRequest, MyResponse};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder()
//! #   .on_receive_request(async |req: MyRequest, responder, cx| {
//! responder.respond_with_error(agent_client_protocol::Error::invalid_params())?;
//! # Ok(())
//! #   }, agent_client_protocol::on_receive_request!())
//! #   .connect_with(transport, async |_| Ok(())).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Send at most one response per request. If a handler responds and then
//! returns an error, the first response wins and the duplicate completion is
//! ignored. Dropping a responder from a batch produces an automatic Internal
//! Error so sibling responses are not stranded; dropping the responder for an
//! individual request sends no response.
//!
//! # Multiple Handlers
//!
//! You can register multiple handlers. They're tried in order until one
//! handles the message:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::{ValidateRequest, ValidateResponse, ExecuteRequest, ExecuteResponse};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! Client.builder()
//!     .on_receive_request(async |req: ValidateRequest, responder, cx| {
//!         // Handle validation requests
//!         responder.respond(ValidateResponse { is_valid: true, error: None })
//!     }, agent_client_protocol::on_receive_request!())
//!     .on_receive_request(async |req: ExecuteRequest, responder, cx| {
//!         // Handle execution requests
//!         responder.respond(ExecuteResponse { result: "done".into() })
//!     }, agent_client_protocol::on_receive_request!())
//! #   .connect_with(transport, async |_| Ok(())).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Ordering Guarantees
//!
//! Callbacks run inside the dispatch loop and block further message processing
//! until they complete. This gives you ordering guarantees but also means you
//! need to be careful about deadlocks.
//!
//! See [Ordering](super::ordering) for the full details.
//!
//! # Next Steps
//!
//! - [Explicit Peers](super::peers) - Use `_from` variants to specify the source peer
//! - [Ordering](super::ordering) - Understand dispatch loop semantics
//!
//! [`Responder`]: crate::Responder
//! [`ConnectionTo`]: crate::ConnectionTo
