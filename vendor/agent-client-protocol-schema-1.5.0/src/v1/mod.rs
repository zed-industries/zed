//! Agent Client Protocol version 1 types.

mod agent;
mod client;
mod content;
#[cfg(feature = "unstable_elicitation")]
mod elicitation;
mod error;
mod ext;
#[cfg(feature = "unstable_mcp_over_acp")]
mod mcp;
#[cfg(feature = "unstable_nes")]
mod nes;
mod plan;
mod protocol_level;
mod tool_call;

pub use crate::rpc::{JsonRpcBatch, JsonRpcMessage, Notification, Request, RequestId};
pub use agent::*;
pub use client::*;
pub use content::*;
use derive_more::{Display, From};
#[cfg(feature = "unstable_elicitation")]
pub use elicitation::*;
pub use error::*;
pub use ext::*;
#[cfg(feature = "unstable_mcp_over_acp")]
pub use mcp::*;
#[cfg(feature = "unstable_nes")]
pub use nes::*;
pub use plan::*;
pub use protocol_level::*;
pub use serde_json::value::RawValue;
pub use tool_call::*;

/// JSON-RPC response envelope using this protocol version's error type.
pub type Response<Result> = crate::rpc::Response<Result, Error>;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A unique identifier for a conversation session between a client and agent.
///
/// Sessions maintain their own context, conversation history, and state,
/// allowing multiple independent interactions with the same agent.
///
/// See protocol docs: [Session ID](https://agentclientprotocol.com/protocol/session-setup#session-id)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct SessionId(pub Arc<str>);

impl SessionId {
    /// Wraps a protocol string as a typed [`SessionId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}
