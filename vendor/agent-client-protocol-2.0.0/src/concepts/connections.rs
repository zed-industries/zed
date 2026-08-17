//! Establishing connections using role types and connection builders.
//!
//! To communicate over ACP, you need to establish a connection. This involves
//! choosing a **role type** that matches your role and using a **connection builder**
//! to configure and run the connection.
//!
//! # Choosing a Role Type
//!
//! Your role type determines what messages you can send and who you can send them to.
//! Choose based on what you're building:
//!
//! | You are building... | Use this role type |
//! |---------------------|-------------------|
//! | A client that talks to an agent | [`Client`] |
//! | An agent that responds to clients | [`Agent`] |
//! | A proxy in a conductor chain | [`Proxy`] |
//!
//! # The Connection Builder Pattern
//!
//! Every role type has a `builder()` method that returns a connection builder.
//! The builder lets you configure handlers, then connect to a transport:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! Client.builder()
//!     .name("my-client")
//!     .connect_with(transport, async |cx| {
//!         // Use `cx` to send requests and handle responses
//!         Ok(())
//!     })
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # The Connection Context
//!
//! Inside `connect_with`, you receive a [`ConnectionTo`] (connection context) that
//! lets you interact with the remote peer:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol::schema::{ProtocolVersion, v1::InitializeRequest};
//! # use agent_client_protocol_test::StatusUpdate;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! // Send a request and wait for the response
//! let response = cx.send_request(InitializeRequest::new(ProtocolVersion::V1))
//!     .block_task()
//!     .await?;
//!
//! // Send a notification (fire-and-forget)
//! cx.send_notification(StatusUpdate { message: "hello".into() })?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Clean Incoming EOF
//!
//! [`Builder::connect_to`](crate::Builder::connect_to) is reactive: it returns
//! `Ok(())` when the incoming transport reaches clean EOF, after draining
//! responses and notifications already accepted by its outgoing queue through
//! the transport sink.
//! [`Builder::connect_with`](crate::Builder::connect_with) is foreground-owned:
//! EOF fails pending requests, but does not cancel unrelated work in its
//! closure. This avoids dropping application futures at an arbitrary await
//! point.
//!
//! Use [`ConnectionTo::incoming_closed`](crate::ConnectionTo::incoming_closed)
//! to await EOF directly, or [`Builder::on_close`](crate::Builder::on_close)
//! for cleanup and application-specific shutdown policy:
//!
//! ```
//! # use agent_client_protocol::{Client, ConnectTo, Error};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), Error> {
//! Client.builder()
//!     .on_close(async |_cx| {
//!         // Notify application-owned work here. Returning an error also
//!         // terminates a still-running connect_with foreground.
//!         Ok(())
//!     })
//!     .connect_with(transport, async |cx| {
//!         cx.incoming_closed().await;
//!         Ok(())
//!     })
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Every request still waiting for a response at EOF is completed with an
//! internal error whose data contains
//! `{"reason":"incoming_transport_closed","method":"..."}`. Requests made
//! after EOF fail the same way; use
//! [`is_incoming_transport_closed`](crate::is_incoming_transport_closed) to
//! identify this error. In `connect_with`, notification and response sends
//! remain available so applications can choose their own half-close policy;
//! reactive `connect_to` stops accepting them when its final drain begins.
//!
//! Pending requests are failed before close callbacks begin. The close signal
//! is published after callbacks finish, so a callback must not await
//! [`ConnectionTo::incoming_closed`](crate::ConnectionTo::incoming_closed)
//! itself.
//!
//! # Sending Requests
//!
//! When you call `send_request()`, you get back a [`SentRequest`] that represents
//! the pending response. You have two main ways to handle it:
//!
//! ## Option 1: Block and wait
//!
//! Use `block_task()` when you need the response before continuing:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! let response = cx.send_request(MyRequest {})
//!     .block_task()
//!     .await?;
//! // Use response here
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Option 2: Schedule a callback
//!
//! Use `on_receiving_result()` when you want to handle the response asynchronously:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.send_request(MyRequest {})
//!     .on_receiving_result(async |result| {
//!         match result {
//!             Ok(response) => { /* handle success */ }
//!             Err(error) => { /* handle error */ }
//!         }
//!         Ok(())
//!     })?;
//! // Continues immediately, callback runs when response arrives
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! See [Ordering](super::ordering) for important details about how these differ.
//!
//! ## Dropping a `SentRequest`
//!
//! Dropping a [`SentRequest`] before the SDK has received the response sends a
//! `$/cancel_request` notification asking the peer to cancel the request, then
//! discards the response when it arrives. For a request whose eventual response
//! should be ignored, but which should keep running on the peer, call
//! [`SentRequest::detach`] instead. If no response is expected at all, use a
//! notification. See the request cancellation chapter
//! ([`concepts::cancellation`](super::cancellation)) for details.
//!
//! # Next Steps
//!
//! - [Sessions](super::sessions) - Create multi-turn conversations
//! - [Callbacks](super::callbacks) - Handle incoming requests from the remote peer
//!
//! [`Client`]: crate::Client
//! [`Agent`]: crate::Agent
//! [`Proxy`]: crate::Proxy
//! [`ConnectionTo`]: crate::ConnectionTo
//! [`SentRequest`]: crate::SentRequest
//! [`SentRequest::detach`]: crate::SentRequest::detach
