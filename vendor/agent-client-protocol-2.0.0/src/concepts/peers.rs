//! Explicit peers: using `_to` and `_from` variants.
//!
//! So far, we've used methods like `send_request` and `on_receive_request`
//! without specifying *who* we're sending to or receiving from. That's because
//! each role type has a **default peer**.
//!
//! # Default Peers
//!
//! For simple role types, there's only one peer to talk to:
//!
//! | Role Type | Default Peer |
//! |-----------|--------------|
//! | [`Client`] | The agent |
//! | [`Agent`] | The client |
//!
//! So when you write:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol::schema::{ProtocolVersion, v1::InitializeRequest};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! // As a client
//! cx.send_request(InitializeRequest::new(ProtocolVersion::V1));
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! The request automatically goes to the agent, because that's the only peer
//! a client can talk to.
//!
//! # Explicit Peer Methods
//!
//! Every method has an explicit variant that takes a peer argument:
//!
//! | Default method | Explicit variant |
//! |----------------|------------------|
//! | `send_request` | `send_request_to(peer, request)` |
//! | `send_notification` | `send_notification_to(peer, request)` |
//! | `on_receive_request` | `on_receive_request_from(peer, callback)` |
//! | `on_receive_notification` | `on_receive_notification_from(peer, callback)` |
//!
//! For simple role types, the explicit form is equivalent:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! # let req = MyRequest {};
//! // These are equivalent for Client:
//! let request = cx.send_request(req.clone());
//! let same = cx.send_request_to(Agent, req);
//! # let _ = (request, same);
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Why Explicit Peers Matter
//!
//! Explicit peers become essential when working with proxies. A proxy sits
//! between a client and an agent, so it has *two* peers:
//!
//! - [`Client`] - the client (or previous proxy in the chain)
//! - [`Agent`] - the agent (or next proxy in the chain)
//!
//! When writing proxy code, you need to specify which direction:
//!
//! ```
//! # use agent_client_protocol::{Proxy, Client, Agent, Conductor, ConnectTo};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(transport: impl ConnectTo<Proxy>) -> Result<(), agent_client_protocol::Error> {
//! Proxy.builder()
//!     // Receive a request from the client
//!     .on_receive_request_from(Client, async |req: MyRequest, responder, cx| {
//!         // Forward it to the agent
//!         cx.send_request_to(Agent, req)
//!             .forward_response_to(responder)
//!     }, agent_client_protocol::on_receive_request!())
//!     .connect_to(transport)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! See [Proxies and Conductors](super::proxies) for more on building proxies.
//!
//! # Available Peer Types
//!
//! | Peer Type | Represents |
//! |-----------|------------|
//! | [`Client`] | The client direction |
//! | [`Agent`] | The agent direction |
//! | [`Conductor`] | The conductor (for proxies) |
//!
//! # Next Steps
//!
//! - [Ordering](super::ordering) - Understand dispatch loop semantics
//! - [Proxies and Conductors](super::proxies) - Build proxies that use explicit peers
//!
//! [`Client`]: crate::Client
//! [`Agent`]: crate::Agent
//! [`Conductor`]: crate::Conductor
