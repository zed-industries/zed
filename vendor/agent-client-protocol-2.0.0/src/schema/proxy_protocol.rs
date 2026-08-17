//! Protocol types for proxy communication.
//!
//! These types are intended to become part of the ACP protocol specification.

use crate::{JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, UntypedMessage};
use agent_client_protocol_schema::v1::{InitializeRequest, InitializeResponse};
use serde::{Deserialize, Serialize};

// =============================================================================
// Successor forwarding protocol
// =============================================================================

/// JSON-RPC method name for successor forwarding.
pub const METHOD_SUCCESSOR_MESSAGE: &str = "_proxy/successor";

/// A message being sent to the successor component.
///
/// Used in `_proxy/successor` when the proxy wants to forward a message downstream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessorMessage<M: JsonRpcMessage = UntypedMessage> {
    /// The message to be sent to the successor component.
    #[serde(flatten)]
    pub message: M,

    /// Optional `_meta` metadata.
    #[serde(
        rename = "_meta",
        alias = "meta",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<serde_json::Value>,
}

impl<M: JsonRpcMessage> JsonRpcMessage for SuccessorMessage<M> {
    fn matches_method(method: &str) -> bool {
        method == METHOD_SUCCESSOR_MESSAGE
    }

    fn method(&self) -> &str {
        METHOD_SUCCESSOR_MESSAGE
    }

    fn to_untyped_message(&self) -> Result<UntypedMessage, crate::Error> {
        UntypedMessage::new(
            METHOD_SUCCESSOR_MESSAGE,
            SuccessorMessage {
                message: self.message.to_untyped_message()?,
                meta: self.meta.clone(),
            },
        )
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_SUCCESSOR_MESSAGE {
            return Err(crate::Error::method_not_found());
        }
        let outer = crate::util::json_cast_params::<_, SuccessorMessage<UntypedMessage>>(params)?;
        if !M::matches_method(&outer.message.method) {
            return Err(crate::Error::method_not_found());
        }
        let inner = M::parse_message(&outer.message.method, &outer.message.params)?;
        Ok(SuccessorMessage {
            message: inner,
            meta: outer.meta,
        })
    }
}

impl<Req: JsonRpcRequest> JsonRpcRequest for SuccessorMessage<Req> {
    type Response = Req::Response;
}

impl<Notif: JsonRpcNotification> JsonRpcNotification for SuccessorMessage<Notif> {}

// =============================================================================
// Proxy initialization protocol
// =============================================================================

/// JSON-RPC method name for proxy initialization.
pub const METHOD_INITIALIZE_PROXY: &str = "_proxy/initialize";

/// Initialize request for proxy components.
///
/// This is sent to components that have a successor in the chain.
/// Components that receive this (instead of `InitializeRequest`) know they
/// are operating as a proxy and should forward messages to their successor.
#[derive(Debug, Clone, Serialize, Deserialize, crate::JsonRpcRequest)]
#[request(method = "_proxy/initialize", response = InitializeResponse, crate = crate)]
pub struct InitializeProxyRequest {
    /// The underlying initialize request data.
    #[serde(flatten)]
    pub initialize: InitializeRequest,
}

impl From<InitializeRequest> for InitializeProxyRequest {
    fn from(initialize: InitializeRequest) -> Self {
        Self { initialize }
    }
}
