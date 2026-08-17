#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

//! # agent-client-protocol -- the Agent Client Protocol (ACP) SDK
//!
//! **agent-client-protocol** is a Rust SDK for building [Agent-Client Protocol (ACP)][acp] applications.
//! ACP is a protocol for communication between AI agents and their clients (IDEs, CLIs, etc.),
//! enabling features like tool use, permission requests, and streaming responses.
//!
//! [acp]: https://agentclientprotocol.com/
//!
//! ## What can you build with agent-client-protocol?
//!
//! - **Clients** that talk to ACP agents (like building your own Claude Code interface)
//! - **Proxies** that add capabilities to existing agents (like adding custom tools via MCP)
//! - **Agents** that respond to prompts with AI-powered responses
//!
//! ## Quick Start: Connecting to an Agent
//!
//! The most common use case is connecting to an existing ACP agent as a client.
//! Here's a minimal example that initializes a connection, creates a session,
//! and sends a prompt:
//!
//! ```no_run
//! use agent_client_protocol::Client;
//! use agent_client_protocol::schema::{ProtocolVersion, v1::InitializeRequest};
//!
//! # async fn run(transport: impl agent_client_protocol::ConnectTo<agent_client_protocol::Client>) -> agent_client_protocol::Result<()> {
//! Client.builder()
//!     .name("my-client")
//!     .connect_with(transport, async |cx| {
//!         // Step 1: Initialize the connection
//!         cx.send_request(InitializeRequest::new(ProtocolVersion::V1))
//!             .block_task().await?;
//!
//!         // Step 2: Create a session and send a prompt
//!         cx.build_session_cwd()?
//!             .block_task()
//!             .run_until(async |mut session| {
//!                 session.send_prompt("What is 2 + 2?")?;
//!                 let response = session.read_to_string().await?;
//!                 println!("{}", response);
//!                 Ok(())
//!             })
//!             .await
//!     })
//!     .await
//! # }
//! ```
//!
//! For a complete working example, see [`yolo_one_shot_client.rs`][yolo].
//!
//! [yolo]: https://github.com/agentclientprotocol/rust-sdk/blob/main/src/agent-client-protocol/examples/yolo_one_shot_client.rs
//!
//! ## Cookbook
//!
//! The [`agent_client_protocol_cookbook`] crate contains practical guides and examples:
//!
//! - Connecting as a client
//! - Global MCP server
//! - Per-session MCP server with workspace context
//! - Building agents and reusable components
//! - Running proxies with the conductor
//!
//! [`agent_client_protocol_cookbook`]: https://docs.rs/agent-client-protocol-cookbook
//!
//! ## Core Concepts
//!
//! The [`concepts`] module provides detailed explanations of how agent-client-protocol works,
//! including connections, sessions, callbacks, ordering guarantees, and more.
//!
//! ## Related Crates
//!
//! - [`agent-client-protocol-conductor`] - Binary for running proxy chains
//!
//! [`agent-client-protocol-conductor`]: https://crates.io/crates/agent-client-protocol-conductor

/// Capability management for the `_meta.symposium` object
mod capabilities;
/// Component abstraction for agents and proxies
pub mod component;
/// Core concepts for understanding and using agent-client-protocol
pub mod concepts;
/// JSON-RPC connection and handler infrastructure
mod jsonrpc;
/// Runtime-agnostic MCP server support, including optional attachment to ACP sessions.
pub mod mcp_server;
/// Role types for ACP connections
pub mod role;
/// ACP protocol schema types - all message types, requests, responses, and supporting types
pub mod schema;
/// Utility functions and types
pub mod util;

pub use capabilities::*;

pub use jsonrpc::{
    Builder, ByteStreams, Channel, ConnectionTo, Dispatch, DynamicHandlerGuard,
    HandleConnectionClose, HandleDispatchFrom, Handled, INCOMING_TRANSPORT_CLOSED_REASON,
    IntoHandled, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, Lines,
    NullClose, NullHandler, RawJsonRpcMessage, RawJsonRpcParams, Responder, ResponseRouter,
    SentRequest, TransportBatch, TransportBatchEntry, TransportFrame, UntypedMessage,
    is_incoming_transport_closed,
    run::{ChainRun, NullRun, RunWithConnectionTo},
};
pub use jsonrpc::{RequestCancellation, is_cancel_request_notification};

#[cfg(feature = "unstable_protocol_v2")]
pub use role::acp::AgentProtocolRouter;
#[cfg(feature = "unstable_protocol_v2")]
pub use role::acp::ClientProtocolConnector;
pub use role::{
    Role, RoleId, UntypedRole,
    acp::{Agent, Client, Conductor, Proxy},
};

pub use component::{ConnectTo, DynConnectTo};

/// Implementation details used by the derive macros.
#[doc(hidden)]
pub mod __private {
    pub use serde;
    pub use serde_json;
}

// Re-export BoxFuture for implementing SDK traits that return boxed futures.
pub use futures::future::BoxFuture;

// Re-export commonly used infrastructure types for convenience
pub use schema::v1::{Error, ErrorCode, Result};

// Re-export derive macros for custom JSON-RPC types
pub use agent_client_protocol_derive::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};

mod session;
pub use session::*;

mod acp_agent;
pub use acp_agent::{AcpAgent, AcpAgentConfig, LineDirection};

mod stdio;
pub use stdio::Stdio;

/// This is a hack that must be given as the final argument of
/// the MCP server builder's `tool_fn_mut` method when defining tools.
///
/// The `agent-client-protocol-rmcp` crate provides the builder this macro is
/// typically used with.
/// Look away, lest ye be blinded by its vileness!
///
/// Fine, if you MUST know, it's a horrific workaround for not having
/// [return-type notation](https://github.com/rust-lang/rust/issues/109417)
/// and for [this !@$#!%! bug](https://github.com/rust-lang/rust/issues/110338).
/// Trust me, the need for it hurts me more than it hurts you. --nikomatsakis
#[macro_export]
macro_rules! tool_fn_mut {
    () => {
        |func, params, context| Box::pin(func(params, context))
    };
}

/// This is a hack that must be given as the final argument of
/// the MCP server builder's `tool_fn` method when defining stateless concurrent tools.
///
/// The `agent-client-protocol-rmcp` crate provides the builder this macro is
/// typically used with.
/// See [`tool_fn_mut!`] for the gory details.
#[macro_export]
macro_rules! tool_fn {
    () => {
        |func, params, context| Box::pin(func(params, context))
    };
}

/// This macro is used for the value of the `to_future_hack` parameter of
/// [`Builder::on_receive_request`] and [`Builder::on_receive_request_from`].
///
/// It expands to `|f, req, responder, cx| Box::pin(f(req, responder, cx))`.
///
/// This is needed until [return-type notation](https://github.com/rust-lang/rust/issues/109417)
/// is stabilized.
#[macro_export]
macro_rules! on_receive_request {
    () => {
        |f: &mut _, req, responder, cx| Box::pin(f(req, responder, cx))
    };
}

/// This macro is used for the value of the `to_future_hack` parameter of
/// [`Builder::on_receive_notification`] and [`Builder::on_receive_notification_from`].
///
/// It expands to `|f, notif, cx| Box::pin(f(notif, cx))`.
///
/// This is needed until [return-type notation](https://github.com/rust-lang/rust/issues/109417)
/// is stabilized.
#[macro_export]
macro_rules! on_receive_notification {
    () => {
        |f: &mut _, notif, cx| Box::pin(f(notif, cx))
    };
}

/// This macro is used for the value of the `to_future_hack` parameter of
/// [`Builder::on_receive_dispatch`] and [`Builder::on_receive_dispatch_from`].
///
/// It expands to `|f, dispatch, cx| Box::pin(f(dispatch, cx))`.
///
/// This is needed until [return-type notation](https://github.com/rust-lang/rust/issues/109417)
/// is stabilized.
#[macro_export]
macro_rules! on_receive_dispatch {
    () => {
        |f: &mut _, dispatch, cx| Box::pin(f(dispatch, cx))
    };
}
