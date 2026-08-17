//! Building proxies that intercept and modify messages.
//!
//! A **proxy** sits between a client and an agent, intercepting messages
//! in both directions. This is how you add capabilities like MCP tools,
//! logging, or message transformation.
//!
//! # The Proxy Role Type
//!
//! Proxies use the [`Proxy`] role type, which has two peers:
//!
//! - [`Client`] - messages from/to the client direction
//! - [`Agent`] - messages from/to the agent direction
//!
//! Unlike simpler links, there's no default peer - you must always specify
//! which direction you're communicating with.
//!
//! # Default Forwarding
//!
//! By default, [`Proxy`] forwards all messages it doesn't handle.
//! This means a minimal proxy that does nothing is just:
//!
//! ```
//! # use agent_client_protocol::{Proxy, Conductor, ConnectTo};
//! # async fn example(transport: impl ConnectTo<Proxy>) -> Result<(), agent_client_protocol::Error> {
//! Proxy.builder()
//!     .connect_to(transport)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! All messages pass through unchanged.
//!
//! # Intercepting Messages
//!
//! To intercept specific messages, use `on_receive_*_from` with explicit peers:
//!
//! ```
//! # use agent_client_protocol::{Proxy, Client, Agent, Conductor, ConnectTo};
//! # use agent_client_protocol_test::ProcessRequest;
//! # async fn example(transport: impl ConnectTo<Proxy>) -> Result<(), agent_client_protocol::Error> {
//! Proxy.builder()
//!     // Intercept requests from the client
//!     .on_receive_request_from(Client, async |req: ProcessRequest, responder, cx| {
//!         // Modify the request
//!         let modified = ProcessRequest {
//!             data: format!("prefix: {}", req.data),
//!         };
//!
//!         // Forward to agent and relay the response back
//!         cx.send_request_to(Agent, modified)
//!             .forward_response_to(responder)
//!     }, agent_client_protocol::on_receive_request!())
//!     .connect_to(transport)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Messages you don't handle are forwarded automatically.
//!
//! # Adding MCP Servers
//!
//! A common use case is adding tools via MCP. You can add them globally
//! (available in all sessions) or per-session.
//!
//! These ACP attachment APIs require the `unstable_mcp_over_acp` feature.
//!
//! ## Global MCP Server
//!
//! ```ignore
//! # use agent_client_protocol::{Proxy, Conductor, ConnectTo};
//! # use agent_client_protocol::mcp_server::McpServer;
//! # use agent_client_protocol_rmcp::McpServerExt;
//! # async fn example(transport: impl ConnectTo<Proxy>) -> Result<(), agent_client_protocol::Error> {
//! # let my_mcp_server = McpServer::<Conductor, _>::builder("tools").build();
//! Proxy.builder()
//!     .with_mcp_server(my_mcp_server)
//!     .connect_to(transport)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Per-Session MCP Server
//!
//! ```ignore
//! # use agent_client_protocol::{Proxy, Client, Conductor, ConnectTo};
//! # use agent_client_protocol::schema::v1::NewSessionRequest;
//! # use agent_client_protocol::mcp_server::McpServer;
//! # use agent_client_protocol_rmcp::McpServerExt;
//! # async fn example(transport: impl ConnectTo<Proxy>) -> Result<(), agent_client_protocol::Error> {
//! Proxy.builder()
//!     .on_receive_request_from(Client, async |req: NewSessionRequest, responder, cx| {
//!         let my_mcp_server = McpServer::<Conductor, _>::builder("tools").build();
//!         cx.build_session_from(req)
//!             .with_mcp_server(my_mcp_server)?
//!             .on_proxy_session_start(responder, async |session_id| {
//!                 // Session started with MCP server attached
//!                 Ok(())
//!             })
//!     }, agent_client_protocol::on_receive_request!())
//!     .connect_to(transport)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # The Conductor
//!
//! Proxies don't run standalone - they're orchestrated by a **conductor**.
//! The conductor:
//!
//! - Spawns proxy processes
//! - Chains them together
//! - Connects the final proxy to the agent
//!
//! The [`agent-client-protocol-conductor`] crate provides a conductor binary. You configure
//! it with a list of proxies to run.
//!
//! # Proxy Chains
//!
//! Multiple proxies can be chained:
//!
//! ```text
//! Client <-> Proxy A <-> Proxy B <-> Agent
//! ```
//!
//! Each proxy sees messages from its perspective:
//! - `Client` is "toward the client" (Proxy A, or conductor if first)
//! - `Agent` is "toward the agent" (Proxy B, or agent if last)
//!
//! Messages flow through each proxy in order. Each can inspect, modify,
//! or handle messages before they continue.
//!
//! # Summary
//!
//! | Task | Approach |
//! |------|----------|
//! | Forward everything | Just `connect_to(transport)` |
//! | Intercept specific messages | `on_receive_*_from` with explicit peers |
//! | Add global tools | `with_mcp_server` on builder |
//! | Add per-session tools | `with_mcp_server` on session builder |
//!
//! [`Proxy`]: crate::Proxy
//! [`Client`]: crate::Client
//! [`Agent`]: crate::Agent
//! [`agent-client-protocol-conductor`]: https://crates.io/crates/agent-client-protocol-conductor
