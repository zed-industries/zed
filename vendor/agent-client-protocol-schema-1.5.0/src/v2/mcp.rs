//! MCP-over-ACP transport types.

use std::sync::Arc;

use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_with::{DefaultOnError, serde_as, skip_serializing_none};

use super::{McpServerAcpId, Meta};
use crate::IntoOption;

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// A unique identifier for an active MCP-over-ACP connection.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct McpConnectionId(pub Arc<str>);

impl McpConnectionId {
    /// Wraps a protocol string as a typed [`McpConnectionId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request parameters for `mcp/connect`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = MCP_CONNECT_METHOD_NAME))]
#[non_exhaustive]
pub struct ConnectMcpRequest {
    /// The ACP MCP server ID that was provided by the component declaring the MCP server.
    pub server_id: McpServerAcpId,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl ConnectMcpRequest {
    /// Builds [`ConnectMcpRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(server_id: impl Into<McpServerAcpId>) -> Self {
        Self {
            server_id: server_id.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Response to `mcp/connect`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = MCP_CONNECT_METHOD_NAME))]
#[non_exhaustive]
pub struct ConnectMcpResponse {
    /// The unique identifier for this MCP-over-ACP connection.
    pub connection_id: McpConnectionId,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl ConnectMcpResponse {
    /// Builds [`ConnectMcpResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(connection_id: impl Into<McpConnectionId>) -> Self {
        Self {
            connection_id: connection_id.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request parameters for `mcp/message`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "both", "x-method" = MCP_MESSAGE_METHOD_NAME))]
#[non_exhaustive]
pub struct MessageMcpRequest {
    /// The MCP-over-ACP connection this message is sent on.
    pub connection_id: McpConnectionId,
    /// The inner MCP method name.
    pub method: String,
    /// Optional inner MCP params.
    ///
    /// If omitted or set to `null`, the inner MCP message has no params.
    #[serde(default)]
    pub params: Option<serde_json::Map<String, serde_json::Value>>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl MessageMcpRequest {
    /// Builds [`MessageMcpRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(connection_id: impl Into<McpConnectionId>, method: impl Into<String>) -> Self {
        Self {
            connection_id: connection_id.into(),
            method: method.into(),
            params: None,
            meta: None,
        }
    }

    /// Optional inner MCP params.
    ///
    /// If omitted or set to `null`, the inner MCP message has no params.
    #[must_use]
    pub fn params(
        mut self,
        params: impl IntoOption<serde_json::Map<String, serde_json::Value>>,
    ) -> Self {
        self.params = params.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Notification parameters for `mcp/message`.
///
/// This is used when the wrapped MCP message is a notification and the outer JSON-RPC
/// envelope has no `id`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "both", "x-method" = MCP_MESSAGE_METHOD_NAME))]
#[non_exhaustive]
pub struct MessageMcpNotification {
    /// The MCP-over-ACP connection this message is sent on.
    pub connection_id: McpConnectionId,
    /// The inner MCP method name.
    pub method: String,
    /// Optional inner MCP params.
    ///
    /// If omitted or set to `null`, the inner MCP message has no params.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub params: Option<serde_json::Map<String, serde_json::Value>>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl MessageMcpNotification {
    /// Builds [`MessageMcpNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(connection_id: impl Into<McpConnectionId>, method: impl Into<String>) -> Self {
        Self {
            connection_id: connection_id.into(),
            method: method.into(),
            params: None,
            meta: None,
        }
    }

    /// Optional inner MCP params.
    ///
    /// If omitted or set to `null`, the inner MCP message has no params.
    #[must_use]
    pub fn params(
        mut self,
        params: impl IntoOption<serde_json::Map<String, serde_json::Value>>,
    ) -> Self {
        self.params = params.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Response to `mcp/message`.
///
/// This is the inner MCP response result payload. Any JSON value is valid.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From)]
#[serde(transparent)]
#[schemars(extend("x-side" = "both", "x-method" = MCP_MESSAGE_METHOD_NAME))]
#[non_exhaustive]
pub struct MessageMcpResponse(#[schemars(with = "serde_json::Value")] pub Arc<RawValue>);

impl MessageMcpResponse {
    /// Builds [`MessageMcpResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(result: Arc<RawValue>) -> Self {
        Self(result)
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request parameters for `mcp/disconnect`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = MCP_DISCONNECT_METHOD_NAME))]
#[non_exhaustive]
pub struct DisconnectMcpRequest {
    /// The MCP-over-ACP connection to close.
    pub connection_id: McpConnectionId,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl DisconnectMcpRequest {
    /// Builds [`DisconnectMcpRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(connection_id: impl Into<McpConnectionId>) -> Self {
        Self {
            connection_id: connection_id.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Response to `mcp/disconnect`.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = MCP_DISCONNECT_METHOD_NAME))]
#[non_exhaustive]
pub struct DisconnectMcpResponse {
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl DisconnectMcpResponse {
    /// Builds [`DisconnectMcpResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Method name for opening an MCP-over-ACP connection.
pub(crate) const MCP_CONNECT_METHOD_NAME: &str = "mcp/connect";
/// Method name for exchanging MCP-over-ACP messages.
pub(crate) const MCP_MESSAGE_METHOD_NAME: &str = "mcp/message";
/// Method name for closing an MCP-over-ACP connection.
pub(crate) const MCP_DISCONNECT_METHOD_NAME: &str = "mcp/disconnect";
