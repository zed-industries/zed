//! Methods and notifications the agent handles/receives.
//!
//! This module defines the Agent trait and all associated types for implementing
//! an AI coding agent that follows the Agent Client Protocol (ACP).

use std::{path::PathBuf, sync::Arc};

#[cfg(any(feature = "unstable_auth_methods", feature = "unstable_llm_providers"))]
use std::collections::HashMap;

use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

#[cfg(feature = "unstable_auth_methods")]
use crate::DefaultTrueOnError;
use crate::{IntoOption, ProtocolVersion, SkipListener};

use super::{
    ClientCapabilities, ContentBlock, ExtNotification, ExtRequest, ExtResponse, Meta, SessionId,
};

#[cfg(feature = "unstable_mcp_over_acp")]
use super::mcp::{
    MCP_MESSAGE_METHOD_NAME, MessageMcpNotification, MessageMcpRequest, MessageMcpResponse,
};

#[cfg(feature = "unstable_nes")]
use super::{
    AcceptNesNotification, CloseNesRequest, CloseNesResponse, DidChangeDocumentNotification,
    DidCloseDocumentNotification, DidFocusDocumentNotification, DidOpenDocumentNotification,
    DidSaveDocumentNotification, NesCapabilities, PositionEncodingKind, RejectNesNotification,
    StartNesRequest, StartNesResponse, SuggestNesRequest, SuggestNesResponse,
};

#[cfg(feature = "unstable_nes")]
use super::{
    DOCUMENT_DID_CHANGE_METHOD_NAME, DOCUMENT_DID_CLOSE_METHOD_NAME,
    DOCUMENT_DID_FOCUS_METHOD_NAME, DOCUMENT_DID_OPEN_METHOD_NAME, DOCUMENT_DID_SAVE_METHOD_NAME,
    NES_ACCEPT_METHOD_NAME, NES_CLOSE_METHOD_NAME, NES_REJECT_METHOD_NAME, NES_START_METHOD_NAME,
    NES_SUGGEST_METHOD_NAME,
};

// Initialize

/// Request parameters for the initialize method.
///
/// Sent by the client to establish connection and negotiate capabilities.
///
/// See protocol docs: [Initialization](https://agentclientprotocol.com/protocol/initialization)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = INITIALIZE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InitializeRequest {
    /// The latest protocol version supported by the client.
    pub protocol_version: ProtocolVersion,
    /// Capabilities supported by the client.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub client_capabilities: ClientCapabilities,
    /// Information about the Client name and version sent to the Agent.
    ///
    /// Note: in future versions of the protocol, this will be required.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub client_info: Option<Implementation>,
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

impl InitializeRequest {
    /// Builds [`InitializeRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(protocol_version: ProtocolVersion) -> Self {
        Self {
            protocol_version,
            client_capabilities: ClientCapabilities::default(),
            client_info: None,
            meta: None,
        }
    }

    /// Capabilities supported by the client.
    #[must_use]
    pub fn client_capabilities(mut self, client_capabilities: ClientCapabilities) -> Self {
        self.client_capabilities = client_capabilities;
        self
    }

    /// Information about the Client name and version sent to the Agent.
    #[must_use]
    pub fn client_info(mut self, client_info: impl IntoOption<Implementation>) -> Self {
        self.client_info = client_info.into_option();
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

/// Response to the `initialize` method.
///
/// Contains the negotiated protocol version and agent capabilities.
///
/// See protocol docs: [Initialization](https://agentclientprotocol.com/protocol/initialization)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = INITIALIZE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InitializeResponse {
    /// The protocol version the client specified if supported by the agent,
    /// or the latest protocol version supported by the agent.
    ///
    /// The client should disconnect, if it doesn't support this version.
    pub protocol_version: ProtocolVersion,
    /// Capabilities supported by the agent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub agent_capabilities: AgentCapabilities,
    /// Authentication methods supported by the agent.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub auth_methods: Vec<AuthMethod>,
    /// Information about the Agent name and version sent to the Client.
    ///
    /// Note: in future versions of the protocol, this will be required.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub agent_info: Option<Implementation>,
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

impl InitializeResponse {
    /// Builds [`InitializeResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(protocol_version: ProtocolVersion) -> Self {
        Self {
            protocol_version,
            agent_capabilities: AgentCapabilities::default(),
            auth_methods: vec![],
            agent_info: None,
            meta: None,
        }
    }

    /// Capabilities supported by the agent.
    #[must_use]
    pub fn agent_capabilities(mut self, agent_capabilities: AgentCapabilities) -> Self {
        self.agent_capabilities = agent_capabilities;
        self
    }

    /// Authentication methods supported by the agent.
    #[must_use]
    pub fn auth_methods(mut self, auth_methods: Vec<AuthMethod>) -> Self {
        self.auth_methods = auth_methods;
        self
    }

    /// Information about the Agent name and version sent to the Client.
    #[must_use]
    pub fn agent_info(mut self, agent_info: impl IntoOption<Implementation>) -> Self {
        self.agent_info = agent_info.into_option();
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

/// Metadata about the implementation of the client or agent.
/// Describes the name and version of an ACP implementation, with an optional
/// title for UI representation.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Implementation {
    /// Intended for programmatic or logical use, but can be used as a display
    /// name fallback if title isn’t present.
    pub name: String,
    /// Intended for UI and end-user contexts — optimized to be human-readable
    /// and easily understood.
    ///
    /// If not provided, the name should be used for display.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Version of the implementation. Can be displayed to the user or used
    /// for debugging or metrics purposes. (e.g. "1.0.0").
    pub version: String,
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

impl Implementation {
    /// Builds [`Implementation`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            title: None,
            version: version.into(),
            meta: None,
        }
    }

    /// Intended for UI and end-user contexts — optimized to be human-readable
    /// and easily understood.
    ///
    /// If not provided, the name should be used for display.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
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

// Authentication

/// Request parameters for the authenticate method.
///
/// Specifies which authentication method to use.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = AUTHENTICATE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthenticateRequest {
    /// The ID of the authentication method to use.
    /// Must be one of the methods advertised in the initialize response.
    pub method_id: AuthMethodId,
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

impl AuthenticateRequest {
    /// Builds [`AuthenticateRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(method_id: impl Into<AuthMethodId>) -> Self {
        Self {
            method_id: method_id.into(),
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

/// Response to the `authenticate` method.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = AUTHENTICATE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthenticateResponse {
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

impl AuthenticateResponse {
    /// Builds [`AuthenticateResponse`] with the required response fields set; optional fields start unset or empty.
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

// Logout

/// Request parameters for the logout method.
///
/// Terminates the current authenticated session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = LOGOUT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LogoutRequest {
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

impl LogoutRequest {
    /// Builds [`LogoutRequest`] with the required request fields set; optional fields start unset or empty.
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

/// Response to the `logout` method.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = LOGOUT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LogoutResponse {
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

impl LogoutResponse {
    /// Builds [`LogoutResponse`] with the required response fields set; optional fields start unset or empty.
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

/// Authentication-related capabilities supported by the agent.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AgentAuthCapabilities {
    /// Whether the agent supports the logout method.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports the logout method.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub logout: Option<LogoutCapabilities>,
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

impl AgentAuthCapabilities {
    /// Builds an empty [`AgentAuthCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the agent supports the logout method.
    #[must_use]
    pub fn logout(mut self, logout: impl IntoOption<LogoutCapabilities>) -> Self {
        self.logout = logout.into_option();
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

/// Logout capabilities supported by the agent.
///
/// Supplying `{}` means the agent supports the logout method.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct LogoutCapabilities {
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

impl LogoutCapabilities {
    /// Builds an empty [`LogoutCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Typed identifier used for auth method values on the wire.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct AuthMethodId(pub Arc<str>);

impl AuthMethodId {
    /// Wraps a protocol string as a typed [`AuthMethodId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// Describes an available authentication method.
///
/// The `type` field acts as the discriminator in the serialized JSON form.
/// When no `type` is present, the method is treated as `agent`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuthMethod {
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// User provides a key that the client passes to the agent as an environment variable.
    #[cfg(feature = "unstable_auth_methods")]
    EnvVar(AuthMethodEnvVar),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Client runs an interactive terminal for the user to authenticate via a TUI.
    #[cfg(feature = "unstable_auth_methods")]
    Terminal(AuthMethodTerminal),
    /// Agent handles authentication itself.
    ///
    /// This is the default when no `type` is specified.
    #[serde(untagged)]
    Agent(AuthMethodAgent),
}

impl AuthMethod {
    /// The unique identifier for this authentication method.
    #[must_use]
    pub fn id(&self) -> &AuthMethodId {
        match self {
            Self::Agent(a) => &a.id,
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(e) => &e.id,
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(t) => &t.id,
        }
    }

    /// The human-readable name of this authentication method.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Agent(a) => &a.name,
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(e) => &e.name,
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(t) => &t.name,
        }
    }

    /// Optional description providing more details about this authentication method.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Agent(a) => a.description.as_deref(),
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(e) => e.description.as_deref(),
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(t) => t.description.as_deref(),
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(&self) -> Option<&Meta> {
        match self {
            Self::Agent(a) => a.meta.as_ref(),
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(e) => e.meta.as_ref(),
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(t) => t.meta.as_ref(),
        }
    }
}

/// Agent handles authentication itself.
///
/// This is the default authentication method type.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthMethodAgent {
    /// Unique identifier for this authentication method.
    pub id: AuthMethodId,
    /// Human-readable name of the authentication method.
    pub name: String,
    /// Optional description providing more details about this authentication method.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
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

impl AuthMethodAgent {
    /// Builds [`AuthMethodAgent`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(id: impl Into<AuthMethodId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            meta: None,
        }
    }

    /// Optional description providing more details about this authentication method.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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
/// Environment variable authentication method.
///
/// The user provides credentials that the client passes to the agent as environment variables.
#[cfg(feature = "unstable_auth_methods")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthMethodEnvVar {
    /// Unique identifier for this authentication method.
    pub id: AuthMethodId,
    /// Human-readable name of the authentication method.
    pub name: String,
    /// Optional description providing more details about this authentication method.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// The environment variables the client should set.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub vars: Vec<AuthEnvVar>,
    /// Optional link to a page where the user can obtain their credentials.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub link: Option<String>,
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

#[cfg(feature = "unstable_auth_methods")]
impl AuthMethodEnvVar {
    /// Builds [`AuthMethodEnvVar`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        id: impl Into<AuthMethodId>,
        name: impl Into<String>,
        vars: Vec<AuthEnvVar>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            vars,
            link: None,
            meta: None,
        }
    }

    /// Optional link to a page where the user can obtain their credentials.
    #[must_use]
    pub fn link(mut self, link: impl IntoOption<String>) -> Self {
        self.link = link.into_option();
        self
    }

    /// Optional description providing more details about this authentication method.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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
/// Describes a single environment variable for an [`AuthMethodEnvVar`] authentication method.
#[cfg(feature = "unstable_auth_methods")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthEnvVar {
    /// The environment variable name (e.g. `"OPENAI_API_KEY"`).
    pub name: String,
    /// Human-readable label for this variable, displayed in client UI.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub label: Option<String>,
    /// Whether this value is a secret (e.g. API key, token).
    /// Clients should use a password-style input for secret vars.
    ///
    /// Defaults to `true`.
    #[serde_as(deserialize_as = "DefaultTrueOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    #[schemars(extend("default" = true))]
    pub secret: bool,
    /// Whether this variable is optional.
    ///
    /// Defaults to `false`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "is_false")]
    #[schemars(extend("default" = false))]
    pub optional: bool,
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

#[cfg(feature = "unstable_auth_methods")]
fn default_true() -> bool {
    true
}

#[cfg(feature = "unstable_auth_methods")]
#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_true(v: &bool) -> bool {
    *v
}

#[cfg(feature = "unstable_auth_methods")]
#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_false(v: &bool) -> bool {
    !*v
}

#[cfg(feature = "unstable_auth_methods")]
impl AuthEnvVar {
    /// Creates an auth environment variable prompt with `secret` enabled and `optional` disabled.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: None,
            secret: true,
            optional: false,
            meta: None,
        }
    }

    /// Human-readable label for this variable, displayed in client UI.
    #[must_use]
    pub fn label(mut self, label: impl IntoOption<String>) -> Self {
        self.label = label.into_option();
        self
    }

    /// Whether this value is a secret (e.g. API key, token).
    /// Clients should use a password-style input for secret vars.
    #[must_use]
    pub fn secret(mut self, secret: bool) -> Self {
        self.secret = secret;
        self
    }

    /// Whether this variable is optional.
    #[must_use]
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
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
/// Terminal-based authentication method.
///
/// The client runs an interactive terminal for the user to authenticate via a TUI.
#[cfg(feature = "unstable_auth_methods")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthMethodTerminal {
    /// Unique identifier for this authentication method.
    pub id: AuthMethodId,
    /// Human-readable name of the authentication method.
    pub name: String,
    /// Optional description providing more details about this authentication method.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Additional arguments to pass when running the agent binary for terminal auth.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Additional environment variables to set when running the agent binary for terminal auth.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,
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

#[cfg(feature = "unstable_auth_methods")]
impl AuthMethodTerminal {
    /// Builds [`AuthMethodTerminal`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(id: impl Into<AuthMethodId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            args: Vec::new(),
            env: HashMap::new(),
            meta: None,
        }
    }

    /// Additional arguments to pass when running the agent binary for terminal auth.
    #[must_use]
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Additional environment variables to set when running the agent binary for terminal auth.
    #[must_use]
    pub fn env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Optional description providing more details about this authentication method.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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

// New session

/// Request parameters for creating a new session.
///
/// See protocol docs: [Creating a Session](https://agentclientprotocol.com/protocol/session-setup#creating-a-session)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_NEW_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NewSessionRequest {
    /// The working directory for this session. Must be an absolute path.
    pub cwd: PathBuf,
    /// Additional workspace roots for this session. Each path must be absolute.
    ///
    /// These expand the session's filesystem scope without changing `cwd`, which
    /// remains the base for relative paths. When omitted or empty, no
    /// additional roots are activated for the new session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<PathBuf>,
    /// List of MCP (Model Context Protocol) servers the agent should connect to.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub mcp_servers: Vec<McpServer>,
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

impl NewSessionRequest {
    /// Builds [`NewSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        Self {
            cwd: cwd.into(),
            additional_directories: vec![],
            mcp_servers: vec![],
            meta: None,
        }
    }

    /// Additional workspace roots for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories(mut self, additional_directories: Vec<PathBuf>) -> Self {
        self.additional_directories = additional_directories;
        self
    }

    /// List of MCP (Model Context Protocol) servers the agent should connect to.
    #[must_use]
    pub fn mcp_servers(mut self, mcp_servers: Vec<McpServer>) -> Self {
        self.mcp_servers = mcp_servers;
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

/// Response from creating a new session.
///
/// See protocol docs: [Creating a Session](https://agentclientprotocol.com/protocol/session-setup#creating-a-session)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_NEW_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NewSessionResponse {
    /// Unique identifier for the created session.
    ///
    /// Used in all subsequent requests for this conversation.
    pub session_id: SessionId,
    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub modes: Option<SessionModeState>,
    /// Initial session configuration options if supported by the Agent.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub config_options: Option<Vec<SessionConfigOption>>,
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

impl NewSessionResponse {
    /// Builds [`NewSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
            modes: None,
            config_options: None,
            meta: None,
        }
    }

    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[must_use]
    pub fn modes(mut self, modes: impl IntoOption<SessionModeState>) -> Self {
        self.modes = modes.into_option();
        self
    }

    /// Initial session configuration options if supported by the Agent.
    #[must_use]
    pub fn config_options(
        mut self,
        config_options: impl IntoOption<Vec<SessionConfigOption>>,
    ) -> Self {
        self.config_options = config_options.into_option();
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

// Load session

/// Request parameters for loading an existing session.
///
/// Only available if the Agent supports the `loadSession` capability.
///
/// See protocol docs: [Loading Sessions](https://agentclientprotocol.com/protocol/session-setup#loading-sessions)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_LOAD_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoadSessionRequest {
    /// List of MCP servers to connect to for this session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub mcp_servers: Vec<McpServer>,
    /// The working directory for this session. Must be an absolute path.
    pub cwd: PathBuf,
    /// Additional workspace roots to activate for this session. Each path must be absolute.
    ///
    /// When omitted or empty, no additional roots are activated. When non-empty,
    /// this is the complete resulting additional-root list for the loaded
    /// session. It may differ from any previously used or reported list as long as
    /// the request `cwd` matches the session's `cwd`.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<PathBuf>,
    /// The ID of the session to load.
    pub session_id: SessionId,
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

impl LoadSessionRequest {
    /// Builds [`LoadSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            mcp_servers: vec![],
            cwd: cwd.into(),
            additional_directories: vec![],
            session_id: session_id.into(),
            meta: None,
        }
    }

    /// Additional workspace roots to activate for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories(mut self, additional_directories: Vec<PathBuf>) -> Self {
        self.additional_directories = additional_directories;
        self
    }

    /// List of MCP servers to connect to for this session.
    #[must_use]
    pub fn mcp_servers(mut self, mcp_servers: Vec<McpServer>) -> Self {
        self.mcp_servers = mcp_servers;
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

/// Response from loading an existing session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_LOAD_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoadSessionResponse {
    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub modes: Option<SessionModeState>,
    /// Initial session configuration options if supported by the Agent.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub config_options: Option<Vec<SessionConfigOption>>,
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

impl LoadSessionResponse {
    /// Builds [`LoadSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[must_use]
    pub fn modes(mut self, modes: impl IntoOption<SessionModeState>) -> Self {
        self.modes = modes.into_option();
        self
    }

    /// Initial session configuration options if supported by the Agent.
    #[must_use]
    pub fn config_options(
        mut self,
        config_options: impl IntoOption<Vec<SessionConfigOption>>,
    ) -> Self {
        self.config_options = config_options.into_option();
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

// Fork session

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request parameters for forking an existing session.
///
/// Creates a new session based on the context of an existing one, allowing
/// operations like generating summaries without affecting the original session's history.
///
/// Only available if the Agent supports the `session.fork` capability.
#[cfg(feature = "unstable_session_fork")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_FORK_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForkSessionRequest {
    /// The ID of the session to fork.
    pub session_id: SessionId,
    /// The working directory for this session. Must be an absolute path.
    pub cwd: PathBuf,
    /// Additional workspace roots to activate for this session. Each path must be absolute.
    ///
    /// When omitted or empty, no additional roots are activated. When non-empty,
    /// this is the complete resulting additional-root list for the forked
    /// session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<PathBuf>,
    /// List of MCP servers to connect to for this session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp_servers: Vec<McpServer>,
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

#[cfg(feature = "unstable_session_fork")]
impl ForkSessionRequest {
    /// Builds [`ForkSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            session_id: session_id.into(),
            cwd: cwd.into(),
            additional_directories: vec![],
            mcp_servers: vec![],
            meta: None,
        }
    }

    /// Additional workspace roots to activate for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories(mut self, additional_directories: Vec<PathBuf>) -> Self {
        self.additional_directories = additional_directories;
        self
    }

    /// List of MCP servers to connect to for this session.
    #[must_use]
    pub fn mcp_servers(mut self, mcp_servers: Vec<McpServer>) -> Self {
        self.mcp_servers = mcp_servers;
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
/// Response from forking an existing session.
#[cfg(feature = "unstable_session_fork")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_FORK_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForkSessionResponse {
    /// Unique identifier for the newly created forked session.
    pub session_id: SessionId,
    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub modes: Option<SessionModeState>,
    /// Initial session configuration options if supported by the Agent.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub config_options: Option<Vec<SessionConfigOption>>,
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

#[cfg(feature = "unstable_session_fork")]
impl ForkSessionResponse {
    /// Builds [`ForkSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
            modes: None,
            config_options: None,
            meta: None,
        }
    }

    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[must_use]
    pub fn modes(mut self, modes: impl IntoOption<SessionModeState>) -> Self {
        self.modes = modes.into_option();
        self
    }

    /// Initial session configuration options if supported by the Agent.
    #[must_use]
    pub fn config_options(
        mut self,
        config_options: impl IntoOption<Vec<SessionConfigOption>>,
    ) -> Self {
        self.config_options = config_options.into_option();
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

// Resume session

/// Request parameters for resuming an existing session.
///
/// Resumes an existing session without returning previous messages (unlike `session/load`).
/// This is useful for agents that can resume sessions but don't implement full session loading.
///
/// Only available if the Agent supports the `sessionCapabilities.resume` capability.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_RESUME_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResumeSessionRequest {
    /// The ID of the session to resume.
    pub session_id: SessionId,
    /// The working directory for this session. Must be an absolute path.
    pub cwd: PathBuf,
    /// Additional workspace roots to activate for this session. Each path must be absolute.
    ///
    /// When omitted or empty, no additional roots are activated. When non-empty,
    /// this is the complete resulting additional-root list for the resumed
    /// session. It may differ from any previously used or reported list as long as
    /// the request `cwd` matches the session's `cwd`.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<PathBuf>,
    /// List of MCP servers to connect to for this session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp_servers: Vec<McpServer>,
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

impl ResumeSessionRequest {
    /// Builds [`ResumeSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            session_id: session_id.into(),
            cwd: cwd.into(),
            additional_directories: vec![],
            mcp_servers: vec![],
            meta: None,
        }
    }

    /// Additional workspace roots to activate for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories(mut self, additional_directories: Vec<PathBuf>) -> Self {
        self.additional_directories = additional_directories;
        self
    }

    /// List of MCP servers to connect to for this session.
    #[must_use]
    pub fn mcp_servers(mut self, mcp_servers: Vec<McpServer>) -> Self {
        self.mcp_servers = mcp_servers;
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

/// Response from resuming an existing session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_RESUME_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResumeSessionResponse {
    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub modes: Option<SessionModeState>,
    /// Initial session configuration options if supported by the Agent.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub config_options: Option<Vec<SessionConfigOption>>,
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

impl ResumeSessionResponse {
    /// Builds [`ResumeSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Initial mode state if supported by the Agent
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    #[must_use]
    pub fn modes(mut self, modes: impl IntoOption<SessionModeState>) -> Self {
        self.modes = modes.into_option();
        self
    }

    /// Initial session configuration options if supported by the Agent.
    #[must_use]
    pub fn config_options(
        mut self,
        config_options: impl IntoOption<Vec<SessionConfigOption>>,
    ) -> Self {
        self.config_options = config_options.into_option();
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

// Close session

/// Request parameters for closing an active session.
///
/// If supported, the agent **must** cancel any ongoing work related to the session
/// (treat it as if `session/cancel` was called) and then free up any resources
/// associated with the session.
///
/// Only available if the Agent supports the `sessionCapabilities.close` capability.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_CLOSE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloseSessionRequest {
    /// The ID of the session to close.
    pub session_id: SessionId,
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

impl CloseSessionRequest {
    /// Builds [`CloseSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
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

/// Response from closing a session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_CLOSE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloseSessionResponse {
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

impl CloseSessionResponse {
    /// Builds [`CloseSessionResponse`] with the required response fields set; optional fields start unset or empty.
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

// List sessions

/// Request parameters for listing existing sessions.
///
/// Only available if the Agent supports the `sessionCapabilities.list` capability.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_LIST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSessionsRequest {
    /// Filter sessions by working directory. Must be an absolute path.
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Opaque cursor token from a previous response's nextCursor field for cursor-based pagination
    #[serde(default)]
    pub cursor: Option<String>,
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

impl ListSessionsRequest {
    /// Builds [`ListSessionsRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter sessions by working directory. Must be an absolute path.
    #[must_use]
    pub fn cwd(mut self, cwd: impl IntoOption<PathBuf>) -> Self {
        self.cwd = cwd.into_option();
        self
    }

    /// Opaque cursor token from a previous response's nextCursor field for cursor-based pagination
    #[must_use]
    pub fn cursor(mut self, cursor: impl IntoOption<String>) -> Self {
        self.cursor = cursor.into_option();
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

/// Response from listing sessions.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_LIST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSessionsResponse {
    /// Array of session information objects
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub sessions: Vec<SessionInfo>,
    /// Opaque cursor token. If present, pass this in the next request's cursor parameter
    /// to fetch the next page. If absent, there are no more results.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub next_cursor: Option<String>,
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

impl ListSessionsResponse {
    /// Builds [`ListSessionsResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(sessions: Vec<SessionInfo>) -> Self {
        Self {
            sessions,
            next_cursor: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `nextCursor` field.
    #[must_use]
    pub fn next_cursor(mut self, next_cursor: impl IntoOption<String>) -> Self {
        self.next_cursor = next_cursor.into_option();
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

// Delete session

/// Request parameters for deleting an existing session from `session/list`.
///
/// Only available if the Agent supports the `sessionCapabilities.delete` capability.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_DELETE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteSessionRequest {
    /// The ID of the session to delete.
    pub session_id: SessionId,
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

impl DeleteSessionRequest {
    /// Builds [`DeleteSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
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

/// Response from deleting a session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_DELETE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteSessionResponse {
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

impl DeleteSessionResponse {
    /// Builds [`DeleteSessionResponse`] with the required response fields set; optional fields start unset or empty.
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

/// Information about a session returned by session/list
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionInfo {
    /// Unique identifier for the session
    pub session_id: SessionId,
    /// The working directory for this session. Must be an absolute path.
    pub cwd: PathBuf,
    /// Additional workspace roots reported for this session. Each path must be absolute.
    ///
    /// When present, this is the complete ordered additional-root list reported
    /// by the Agent. Omitted and empty values are equivalent: the response
    /// reports no additional roots.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<PathBuf>,

    /// Human-readable title for the session
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// ISO 8601 timestamp of last activity
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub updated_at: Option<String>,
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

impl SessionInfo {
    /// Builds [`SessionInfo`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            session_id: session_id.into(),
            cwd: cwd.into(),
            additional_directories: vec![],
            title: None,
            updated_at: None,
            meta: None,
        }
    }

    /// Additional workspace roots reported for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories(mut self, additional_directories: Vec<PathBuf>) -> Self {
        self.additional_directories = additional_directories;
        self
    }

    /// Human-readable title for the session
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// ISO 8601 timestamp of last activity
    #[must_use]
    pub fn updated_at(mut self, updated_at: impl IntoOption<String>) -> Self {
        self.updated_at = updated_at.into_option();
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

// Session modes

/// The set of modes and the one currently active.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionModeState {
    /// The current mode the Agent is in.
    pub current_mode_id: SessionModeId,
    /// The set of modes that the Agent can operate in
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub available_modes: Vec<SessionMode>,
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

impl SessionModeState {
    /// Builds [`SessionModeState`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        current_mode_id: impl Into<SessionModeId>,
        available_modes: Vec<SessionMode>,
    ) -> Self {
        Self {
            current_mode_id: current_mode_id.into(),
            available_modes,
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

/// A mode the agent can operate in.
///
/// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionMode {
    /// Stable identifier used to refer to this protocol object in later messages.
    pub id: SessionModeId,
    /// Human-readable name shown for this protocol object.
    pub name: String,
    /// Optional human-readable details shown with this protocol object.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
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

impl SessionMode {
    /// Builds [`SessionMode`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(id: impl Into<SessionModeId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `description` field.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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

/// Unique identifier for a Session Mode.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct SessionModeId(pub Arc<str>);

impl SessionModeId {
    /// Wraps a protocol string as a typed [`SessionModeId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// Request parameters for setting a session mode.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_SET_MODE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetSessionModeRequest {
    /// The ID of the session to set the mode for.
    pub session_id: SessionId,
    /// The ID of the mode to set.
    pub mode_id: SessionModeId,
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

impl SetSessionModeRequest {
    /// Builds [`SetSessionModeRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, mode_id: impl Into<SessionModeId>) -> Self {
        Self {
            session_id: session_id.into(),
            mode_id: mode_id.into(),
            meta: None,
        }
    }

    /// Sets or clears ACP `_meta` extension metadata.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Response to `session/set_mode` method.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_SET_MODE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetSessionModeResponse {
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

impl SetSessionModeResponse {
    /// Builds [`SetSessionModeResponse`] with the required response fields set; optional fields start unset or empty.
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

// Session config options

/// Unique identifier for a session configuration option.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct SessionConfigId(pub Arc<str>);

impl SessionConfigId {
    /// Wraps a protocol string as a typed [`SessionConfigId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// Unique identifier for a session configuration option value.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct SessionConfigValueId(pub Arc<str>);

impl SessionConfigValueId {
    /// Wraps a protocol string as a typed [`SessionConfigValueId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// Unique identifier for a session configuration option value group.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct SessionConfigGroupId(pub Arc<str>);

impl SessionConfigGroupId {
    /// Wraps a protocol string as a typed [`SessionConfigGroupId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// A possible value for a session configuration option.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigSelectOption {
    /// Unique identifier for this option value.
    pub value: SessionConfigValueId,
    /// Human-readable label for this option value.
    pub name: String,
    /// Optional description for this option value.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
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

impl SessionConfigSelectOption {
    /// Builds [`SessionConfigSelectOption`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(value: impl Into<SessionConfigValueId>, name: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            name: name.into(),
            description: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `description` field.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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

/// A group of possible values for a session configuration option.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigSelectGroup {
    /// Unique identifier for this group.
    pub group: SessionConfigGroupId,
    /// Human-readable label for this group.
    pub name: String,
    /// The set of option values in this group.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub options: Vec<SessionConfigSelectOption>,
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

impl SessionConfigSelectGroup {
    /// Builds [`SessionConfigSelectGroup`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        group: impl Into<SessionConfigGroupId>,
        name: impl Into<String>,
        options: Vec<SessionConfigSelectOption>,
    ) -> Self {
        Self {
            group: group.into(),
            name: name.into(),
            options,
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

/// Possible values for a session configuration option.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionConfigSelectOptions {
    /// A flat list of options with no grouping.
    Ungrouped(Vec<SessionConfigSelectOption>),
    /// A list of options grouped under headers.
    Grouped(Vec<SessionConfigSelectGroup>),
}

impl From<Vec<SessionConfigSelectOption>> for SessionConfigSelectOptions {
    fn from(options: Vec<SessionConfigSelectOption>) -> Self {
        SessionConfigSelectOptions::Ungrouped(options)
    }
}

impl From<Vec<SessionConfigSelectGroup>> for SessionConfigSelectOptions {
    fn from(groups: Vec<SessionConfigSelectGroup>) -> Self {
        SessionConfigSelectOptions::Grouped(groups)
    }
}

/// A single-value selector (dropdown) session configuration option payload.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigSelect {
    /// The currently selected value.
    pub current_value: SessionConfigValueId,
    /// The set of selectable options.
    pub options: SessionConfigSelectOptions,
}

impl SessionConfigSelect {
    /// Builds [`SessionConfigSelect`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        current_value: impl Into<SessionConfigValueId>,
        options: impl Into<SessionConfigSelectOptions>,
    ) -> Self {
        Self {
            current_value: current_value.into(),
            options: options.into(),
        }
    }
}

/// A boolean on/off toggle session configuration option payload.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigBoolean {
    /// The current value of the boolean option.
    pub current_value: bool,
}

impl SessionConfigBoolean {
    /// Builds [`SessionConfigBoolean`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(current_value: bool) -> Self {
        Self { current_value }
    }
}

/// Semantic category for a session configuration option.
///
/// This is intended to help Clients distinguish broadly common selectors (e.g. model selector vs
/// session mode selector vs thought/reasoning level) for UX purposes (keyboard shortcuts, icons,
/// placement). It MUST NOT be required for correctness. Clients MUST handle missing or unknown
/// categories gracefully.
///
/// Category names beginning with `_` are free for custom use, like other ACP extension methods.
/// Category names that do not begin with `_` are reserved for the ACP spec.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionConfigOptionCategory {
    /// Session mode selector.
    Mode,
    /// Model selector.
    Model,
    /// Model-related configuration parameter.
    ModelConfig,
    /// Thought/reasoning level selector.
    ThoughtLevel,
    /// Unknown / uncategorized selector.
    #[serde(untagged)]
    Other(String),
}

/// Type-specific session configuration option payload.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schemars(extend("discriminator" = {"propertyName": "type"}))]
#[non_exhaustive]
pub enum SessionConfigKind {
    /// Single-value selector (dropdown).
    Select(SessionConfigSelect),
    /// Boolean on/off toggle.
    Boolean(SessionConfigBoolean),
}

/// A session configuration option selector and its current state.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigOption {
    /// Unique identifier for the configuration option.
    pub id: SessionConfigId,
    /// Human-readable label for the option.
    pub name: String,
    /// Optional description for the Client to display to the user.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Optional semantic category for this option (UX only).
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub category: Option<SessionConfigOptionCategory>,
    /// Type-specific fields for this configuration option.
    #[serde(flatten)]
    pub kind: SessionConfigKind,
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

impl SessionConfigOption {
    /// Builds [`SessionConfigOption`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        id: impl Into<SessionConfigId>,
        name: impl Into<String>,
        kind: SessionConfigKind,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            category: None,
            kind,
            meta: None,
        }
    }

    /// Builds a select-style session configuration option with its current value and choices.
    #[must_use]
    pub fn select(
        id: impl Into<SessionConfigId>,
        name: impl Into<String>,
        current_value: impl Into<SessionConfigValueId>,
        options: impl Into<SessionConfigSelectOptions>,
    ) -> Self {
        Self::new(
            id,
            name,
            SessionConfigKind::Select(SessionConfigSelect::new(current_value, options)),
        )
    }

    /// Builds a boolean-style session configuration option with its current value.
    #[must_use]
    pub fn boolean(
        id: impl Into<SessionConfigId>,
        name: impl Into<String>,
        current_value: bool,
    ) -> Self {
        Self::new(
            id,
            name,
            SessionConfigKind::Boolean(SessionConfigBoolean::new(current_value)),
        )
    }

    /// Sets or clears the optional `description` field.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Sets or clears the optional `category` field.
    #[must_use]
    pub fn category(mut self, category: impl IntoOption<SessionConfigOptionCategory>) -> Self {
        self.category = category.into_option();
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

/// The value to set for a session configuration option.
///
/// The `type` field acts as the discriminator in the serialized JSON form.
/// When no `type` is present, the value is treated as a [`SessionConfigValueId`]
/// via the [`ValueId`](Self::ValueId) fallback variant.
///
/// The `type` discriminator describes the *shape* of the value, not the option
/// kind. For example every option kind that picks from a list of ids
/// (`select`, `radio`, …) would use [`ValueId`](Self::ValueId), while a
/// future freeform text option would get its own variant.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionConfigOptionValue {
    /// A boolean value (`type: "boolean"`).
    Boolean {
        /// The boolean value.
        value: bool,
    },
    /// A [`SessionConfigValueId`] string value.
    ///
    /// This is the default when `type` is absent on the wire. Unknown `type`
    /// values with string payloads also gracefully deserialize into this
    /// variant.
    #[serde(untagged)]
    ValueId {
        /// The value ID.
        value: SessionConfigValueId,
    },
}

impl SessionConfigOptionValue {
    /// Create a value-id option value (used by `select` and other id-based option types).
    #[must_use]
    pub fn value_id(id: impl Into<SessionConfigValueId>) -> Self {
        Self::ValueId { value: id.into() }
    }

    /// Create a boolean option value.
    #[must_use]
    pub fn boolean(val: bool) -> Self {
        Self::Boolean { value: val }
    }

    /// Return the inner [`SessionConfigValueId`] if this is a
    /// [`ValueId`](Self::ValueId) value.
    #[must_use]
    pub fn as_value_id(&self) -> Option<&SessionConfigValueId> {
        match self {
            Self::ValueId { value } => Some(value),
            _ => None,
        }
    }

    /// Return the inner [`bool`] if this is a [`Boolean`](Self::Boolean) value.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean { value } => Some(*value),
            _ => None,
        }
    }
}

impl From<SessionConfigValueId> for SessionConfigOptionValue {
    fn from(value: SessionConfigValueId) -> Self {
        Self::ValueId { value }
    }
}

impl From<bool> for SessionConfigOptionValue {
    fn from(value: bool) -> Self {
        Self::Boolean { value }
    }
}

impl From<&str> for SessionConfigOptionValue {
    fn from(value: &str) -> Self {
        Self::ValueId {
            value: SessionConfigValueId::new(value),
        }
    }
}

/// Request parameters for setting a session configuration option.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_SET_CONFIG_OPTION_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetSessionConfigOptionRequest {
    /// The ID of the session to set the configuration option for.
    pub session_id: SessionId,
    /// The ID of the configuration option to set.
    pub config_id: SessionConfigId,
    /// The value to set, including a `type` discriminator and the raw `value`.
    ///
    /// When `type` is absent on the wire, defaults to treating the value as a
    /// [`SessionConfigValueId`] for `select` options.
    #[serde(flatten)]
    pub value: SessionConfigOptionValue,
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

impl SetSessionConfigOptionRequest {
    /// Builds [`SetSessionConfigOptionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        config_id: impl Into<SessionConfigId>,
        value: impl Into<SessionConfigOptionValue>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            config_id: config_id.into(),
            value: value.into(),
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

/// Response to `session/set_config_option` method.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_SET_CONFIG_OPTION_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetSessionConfigOptionResponse {
    /// The full set of configuration options and their current values.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub config_options: Vec<SessionConfigOption>,
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

impl SetSessionConfigOptionResponse {
    /// Builds [`SetSessionConfigOptionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(config_options: Vec<SessionConfigOption>) -> Self {
        Self {
            config_options,
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

// MCP

/// Configuration for connecting to an MCP (Model Context Protocol) server.
///
/// MCP servers provide tools and context that the agent can use when
/// processing prompts.
///
/// See protocol docs: [MCP Servers](https://agentclientprotocol.com/protocol/session-setup#mcp-servers)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum McpServer {
    /// HTTP transport configuration
    ///
    /// Only available when the Agent capabilities indicate `mcp_capabilities.http` is `true`.
    Http(McpServerHttp),
    /// SSE transport configuration
    ///
    /// Only available when the Agent capabilities indicate `mcp_capabilities.sse` is `true`.
    Sse(McpServerSse),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// ACP transport configuration
    ///
    /// Only available when the Agent capabilities indicate `mcp_capabilities.acp` is `true`.
    /// The MCP server is provided by an ACP component and communicates over the ACP channel.
    #[cfg(feature = "unstable_mcp_over_acp")]
    Acp(McpServerAcp),
    /// Stdio transport configuration
    ///
    /// All Agents MUST support this transport.
    #[serde(untagged)]
    Stdio(McpServerStdio),
}

/// HTTP transport configuration for MCP.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct McpServerHttp {
    /// Human-readable name identifying this MCP server.
    pub name: String,
    /// URL to the MCP server.
    pub url: String,
    /// HTTP headers to set when making requests to the MCP server.
    pub headers: Vec<HttpHeader>,
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

impl McpServerHttp {
    /// Builds [`McpServerHttp`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            headers: Vec::new(),
            meta: None,
        }
    }

    /// HTTP headers to set when making requests to the MCP server.
    #[must_use]
    pub fn headers(mut self, headers: Vec<HttpHeader>) -> Self {
        self.headers = headers;
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

/// SSE transport configuration for MCP.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct McpServerSse {
    /// Human-readable name identifying this MCP server.
    pub name: String,
    /// URL to the MCP server.
    pub url: String,
    /// HTTP headers to set when making requests to the MCP server.
    pub headers: Vec<HttpHeader>,
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

impl McpServerSse {
    /// Builds [`McpServerSse`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            headers: Vec::new(),
            meta: None,
        }
    }

    /// HTTP headers to set when making requests to the MCP server.
    #[must_use]
    pub fn headers(mut self, headers: Vec<HttpHeader>) -> Self {
        self.headers = headers;
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
/// Unique identifier for an MCP server using the ACP transport.
///
/// The value is opaque and generated by the ACP component providing the MCP server. It is
/// used by `mcp/connect` to route connection requests back to the component that declared the
/// server.
#[cfg(feature = "unstable_mcp_over_acp")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct McpServerAcpId(pub Arc<str>);

#[cfg(feature = "unstable_mcp_over_acp")]
impl McpServerAcpId {
    /// Wraps a protocol string as a typed [`McpServerAcpId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// ACP transport configuration for MCP.
///
/// The MCP server is provided by an ACP component and communicates over the ACP channel
/// using `mcp/connect`, `mcp/message`, and `mcp/disconnect`.
#[serde_as]
#[skip_serializing_none]
#[cfg(feature = "unstable_mcp_over_acp")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct McpServerAcp {
    /// Human-readable name identifying this MCP server.
    pub name: String,
    /// Unique identifier for this MCP server, generated by the component providing it.
    ///
    /// Providers MUST NOT reuse an ID for multiple ACP-transport MCP servers that are visible
    /// on the same ACP connection.
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

#[cfg(feature = "unstable_mcp_over_acp")]
impl McpServerAcp {
    /// Builds [`McpServerAcp`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, id: impl Into<McpServerAcpId>) -> Self {
        Self {
            name: name.into(),
            server_id: id.into(),
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

/// Stdio transport configuration for MCP.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct McpServerStdio {
    /// Human-readable name identifying this MCP server.
    pub name: String,
    /// Absolute path to the MCP server executable.
    pub command: PathBuf,
    /// Command-line arguments to pass to the MCP server.
    pub args: Vec<String>,
    /// Environment variables to set when launching the MCP server.
    pub env: Vec<EnvVariable>,
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

impl McpServerStdio {
    /// Builds [`McpServerStdio`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, command: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            args: Vec::new(),
            env: Vec::new(),
            meta: None,
        }
    }

    /// Command-line arguments to pass to the MCP server.
    #[must_use]
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Environment variables to set when launching the MCP server.
    #[must_use]
    pub fn env(mut self, env: Vec<EnvVariable>) -> Self {
        self.env = env;
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

/// An environment variable to set when launching an MCP server.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EnvVariable {
    /// The name of the environment variable.
    pub name: String,
    /// The value to set for the environment variable.
    pub value: String,
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

impl EnvVariable {
    /// Builds [`EnvVariable`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
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

/// An HTTP header to set when making requests to the MCP server.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct HttpHeader {
    /// The name of the HTTP header.
    pub name: String,
    /// The value to set for the HTTP header.
    pub value: String,
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

impl HttpHeader {
    /// Builds [`HttpHeader`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
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

// Prompt

/// Request parameters for sending a user prompt to the agent.
///
/// Contains the user's message and any additional context.
///
/// See protocol docs: [User Message](https://agentclientprotocol.com/protocol/prompt-turn#1-user-message)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_PROMPT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PromptRequest {
    /// The ID of the session to send this user message to
    pub session_id: SessionId,
    /// The blocks of content that compose the user's message.
    ///
    /// As a baseline, the Agent MUST support [`ContentBlock::Text`] and [`ContentBlock::ResourceLink`],
    /// while other variants are optionally enabled via [`PromptCapabilities`].
    ///
    /// The Client MUST adapt its interface according to [`PromptCapabilities`].
    ///
    /// The client MAY include referenced pieces of context as either
    /// [`ContentBlock::Resource`] or [`ContentBlock::ResourceLink`].
    ///
    /// When available, [`ContentBlock::Resource`] is preferred
    /// as it avoids extra round-trips and allows the message to include
    /// pieces of context from sources the agent may not have access to.
    pub prompt: Vec<ContentBlock>,
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

impl PromptRequest {
    /// Builds [`PromptRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, prompt: Vec<ContentBlock>) -> Self {
        Self {
            session_id: session_id.into(),
            prompt,
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

/// Response from processing a user prompt.
///
/// See protocol docs: [Check for Completion](https://agentclientprotocol.com/protocol/prompt-turn#4-check-for-completion)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_PROMPT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PromptResponse {
    /// Indicates why the agent stopped processing the turn.
    pub stop_reason: StopReason,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Token usage for this turn (optional).
    #[cfg(feature = "unstable_end_turn_token_usage")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub usage: Option<Usage>,
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

impl PromptResponse {
    /// Builds [`PromptResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(stop_reason: StopReason) -> Self {
        Self {
            stop_reason,
            #[cfg(feature = "unstable_end_turn_token_usage")]
            usage: None,
            meta: None,
        }
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Token usage for this turn.
    #[cfg(feature = "unstable_end_turn_token_usage")]
    #[must_use]
    pub fn usage(mut self, usage: impl IntoOption<Usage>) -> Self {
        self.usage = usage.into_option();
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

/// Reasons why an agent stops processing a prompt turn.
///
/// See protocol docs: [Stop Reasons](https://agentclientprotocol.com/protocol/prompt-turn#stop-reasons)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StopReason {
    /// The turn ended successfully.
    EndTurn,
    /// The turn ended because the agent reached the maximum number of tokens.
    MaxTokens,
    /// The turn ended because the agent reached the maximum number of allowed
    /// agent requests between user turns.
    MaxTurnRequests,
    /// The turn ended because the agent refused to continue. The user prompt
    /// and everything that comes after it won't be included in the next
    /// prompt, so this should be reflected in the UI.
    Refusal,
    /// The turn was cancelled by the client via `session/cancel`.
    ///
    /// This stop reason MUST be returned when the client sends a `session/cancel`
    /// notification, even if the cancellation causes exceptions in underlying operations.
    /// Agents should catch these exceptions and return this semantically meaningful
    /// response to confirm successful cancellation.
    Cancelled,
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Token usage information for a prompt turn.
#[cfg(feature = "unstable_end_turn_token_usage")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Usage {
    /// Sum of all token types across session.
    pub total_tokens: u64,
    /// Total input tokens across all turns.
    pub input_tokens: u64,
    /// Total output tokens across all turns.
    pub output_tokens: u64,
    /// Total thought/reasoning tokens
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub thought_tokens: Option<u64>,
    /// Total cache read tokens.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub cached_read_tokens: Option<u64>,
    /// Total cache write tokens.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub cached_write_tokens: Option<u64>,
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

#[cfg(feature = "unstable_end_turn_token_usage")]
impl Usage {
    /// Builds [`Usage`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(total_tokens: u64, input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            total_tokens,
            input_tokens,
            output_tokens,
            thought_tokens: None,
            cached_read_tokens: None,
            cached_write_tokens: None,
            meta: None,
        }
    }

    /// Total thought/reasoning tokens
    #[must_use]
    pub fn thought_tokens(mut self, thought_tokens: impl IntoOption<u64>) -> Self {
        self.thought_tokens = thought_tokens.into_option();
        self
    }

    /// Total cache read tokens.
    #[must_use]
    pub fn cached_read_tokens(mut self, cached_read_tokens: impl IntoOption<u64>) -> Self {
        self.cached_read_tokens = cached_read_tokens.into_option();
        self
    }

    /// Total cache write tokens.
    #[must_use]
    pub fn cached_write_tokens(mut self, cached_write_tokens: impl IntoOption<u64>) -> Self {
        self.cached_write_tokens = cached_write_tokens.into_option();
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

// Providers

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Well-known API protocol identifiers for LLM providers.
///
/// Agents and clients MUST handle unknown protocol identifiers gracefully.
///
/// Protocol names beginning with `_` are free for custom use, like other ACP extension methods.
/// Protocol names that do not begin with `_` are reserved for the ACP spec.
#[cfg(feature = "unstable_llm_providers")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[expect(clippy::doc_markdown)]
pub enum LlmProtocol {
    /// Anthropic API protocol.
    Anthropic,
    /// OpenAI API protocol.
    #[serde(rename = "openai")]
    OpenAi,
    /// Azure OpenAI API protocol.
    Azure,
    /// Google Vertex AI API protocol.
    Vertex,
    /// AWS Bedrock API protocol.
    Bedrock,
    /// Unknown or custom protocol.
    #[serde(untagged)]
    Other(String),
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Current effective non-secret routing configuration for a provider.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ProviderCurrentConfig {
    /// Protocol currently used by this provider.
    pub api_type: LlmProtocol,
    /// Base URL currently used by this provider.
    pub base_url: String,
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

#[cfg(feature = "unstable_llm_providers")]
impl ProviderCurrentConfig {
    /// Builds [`ProviderCurrentConfig`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(api_type: LlmProtocol, base_url: impl Into<String>) -> Self {
        Self {
            api_type,
            base_url: base_url.into(),
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
/// Unique identifier for a configurable LLM provider.
#[cfg(feature = "unstable_llm_providers")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct ProviderId(pub Arc<str>);

#[cfg(feature = "unstable_llm_providers")]
impl ProviderId {
    /// Wraps a protocol string as a typed [`ProviderId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Information about a configurable LLM provider.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ProviderInfo {
    /// Provider identifier, for example "main" or "openai".
    pub provider_id: ProviderId,
    /// Supported protocol types for this provider.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub supported: Vec<LlmProtocol>,
    /// Whether this provider is mandatory and cannot be disabled via `providers/disable`.
    /// If true, clients must not call `providers/disable` for this provider ID.
    pub required: bool,
    /// Current effective non-secret routing config.
    /// Null or omitted means provider is disabled.
    #[serde(default)]
    pub current: Option<ProviderCurrentConfig>,
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

#[cfg(feature = "unstable_llm_providers")]
impl ProviderInfo {
    /// Builds [`ProviderInfo`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        provider_id: impl Into<ProviderId>,
        supported: Vec<LlmProtocol>,
        required: bool,
        current: impl IntoOption<ProviderCurrentConfig>,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            supported,
            required,
            current: current.into_option(),
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
/// Request parameters for `providers/list`.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = PROVIDERS_LIST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListProvidersRequest {
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

#[cfg(feature = "unstable_llm_providers")]
impl ListProvidersRequest {
    /// Builds [`ListProvidersRequest`] with the required request fields set; optional fields start unset or empty.
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Response to `providers/list`.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = PROVIDERS_LIST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListProvidersResponse {
    /// Configurable providers with current routing info suitable for UI display.
    pub providers: Vec<ProviderInfo>,
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

#[cfg(feature = "unstable_llm_providers")]
impl ListProvidersResponse {
    /// Builds [`ListProvidersResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(providers: Vec<ProviderInfo>) -> Self {
        Self {
            providers,
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
/// Request parameters for `providers/set`.
///
/// Replaces the full configuration for one provider ID.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = PROVIDERS_SET_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetProviderRequest {
    /// Provider ID to configure.
    pub provider_id: ProviderId,
    /// Protocol type for this provider.
    pub api_type: LlmProtocol,
    /// Base URL for requests sent through this provider.
    pub base_url: String,
    /// Full headers map for this provider.
    /// May include authorization, routing, or other integration-specific headers.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
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

#[cfg(feature = "unstable_llm_providers")]
impl SetProviderRequest {
    /// Builds [`SetProviderRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        provider_id: impl Into<ProviderId>,
        api_type: LlmProtocol,
        base_url: impl Into<String>,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            api_type,
            base_url: base_url.into(),
            headers: HashMap::new(),
            meta: None,
        }
    }

    /// Full headers map for this provider.
    /// May include authorization, routing, or other integration-specific headers.
    #[must_use]
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
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
/// Response to `providers/set`.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = PROVIDERS_SET_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetProviderResponse {
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

#[cfg(feature = "unstable_llm_providers")]
impl SetProviderResponse {
    /// Builds [`SetProviderResponse`] with the required response fields set; optional fields start unset or empty.
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request parameters for `providers/disable`.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = PROVIDERS_DISABLE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DisableProviderRequest {
    /// Provider ID to disable.
    pub provider_id: ProviderId,
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

#[cfg(feature = "unstable_llm_providers")]
impl DisableProviderRequest {
    /// Builds [`DisableProviderRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(provider_id: impl Into<ProviderId>) -> Self {
        Self {
            provider_id: provider_id.into(),
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
/// Response to `providers/disable`.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = PROVIDERS_DISABLE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DisableProviderResponse {
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

#[cfg(feature = "unstable_llm_providers")]
impl DisableProviderResponse {
    /// Builds [`DisableProviderResponse`] with the required response fields set; optional fields start unset or empty.
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

// Capabilities

/// Capabilities supported by the agent.
///
/// Advertised during initialization to inform the client about
/// available features and content types.
///
/// See protocol docs: [Agent Capabilities](https://agentclientprotocol.com/protocol/initialization#agent-capabilities)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AgentCapabilities {
    /// Whether the agent supports `session/load`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub load_session: bool,
    /// Prompt capabilities supported by the agent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub prompt_capabilities: PromptCapabilities,
    /// MCP capabilities supported by the agent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mcp_capabilities: McpCapabilities,
    /// Session lifecycle and prompt capabilities advertised by the agent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub session_capabilities: SessionCapabilities,
    /// Authentication-related capabilities supported by the agent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub auth: AgentAuthCapabilities,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Provider configuration capabilities supported by the agent.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports provider configuration methods.
    #[cfg(feature = "unstable_llm_providers")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub providers: Option<ProvidersCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// NES (Next Edit Suggestions) capabilities supported by the agent.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support
    /// for NES methods.
    #[cfg(feature = "unstable_nes")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub nes: Option<NesCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// The position encoding selected by the agent from the client's supported encodings.
    #[cfg(feature = "unstable_nes")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub position_encoding: Option<PositionEncodingKind>,
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

impl AgentCapabilities {
    /// Builds an empty [`AgentCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the agent supports `session/load`.
    #[must_use]
    pub fn load_session(mut self, load_session: bool) -> Self {
        self.load_session = load_session;
        self
    }

    /// Prompt capabilities supported by the agent.
    #[must_use]
    pub fn prompt_capabilities(mut self, prompt_capabilities: PromptCapabilities) -> Self {
        self.prompt_capabilities = prompt_capabilities;
        self
    }

    /// MCP capabilities supported by the agent.
    #[must_use]
    pub fn mcp_capabilities(mut self, mcp_capabilities: McpCapabilities) -> Self {
        self.mcp_capabilities = mcp_capabilities;
        self
    }

    /// Session capabilities supported by the agent.
    #[must_use]
    pub fn session_capabilities(mut self, session_capabilities: SessionCapabilities) -> Self {
        self.session_capabilities = session_capabilities;
        self
    }

    /// Authentication-related capabilities supported by the agent.
    #[must_use]
    pub fn auth(mut self, auth: AgentAuthCapabilities) -> Self {
        self.auth = auth;
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Provider configuration capabilities supported by the agent.
    #[cfg(feature = "unstable_llm_providers")]
    #[must_use]
    pub fn providers(mut self, providers: impl IntoOption<ProvidersCapabilities>) -> Self {
        self.providers = providers.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// NES (Next Edit Suggestions) capabilities supported by the agent.
    #[cfg(feature = "unstable_nes")]
    #[must_use]
    pub fn nes(mut self, nes: impl IntoOption<NesCapabilities>) -> Self {
        self.nes = nes.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// The position encoding selected by the agent from the client's supported encodings.
    #[cfg(feature = "unstable_nes")]
    #[must_use]
    pub fn position_encoding(
        mut self,
        position_encoding: impl IntoOption<PositionEncodingKind>,
    ) -> Self {
        self.position_encoding = position_encoding.into_option();
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
/// Provider configuration capabilities supported by the agent.
///
/// Supplying `{}` means the agent supports provider configuration methods.
#[cfg(feature = "unstable_llm_providers")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct ProvidersCapabilities {
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

#[cfg(feature = "unstable_llm_providers")]
impl ProvidersCapabilities {
    /// Builds an empty [`ProvidersCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Session capabilities supported by the agent.
///
/// As a baseline, all Agents **MUST** support `session/new`, `session/prompt`, `session/cancel`, and `session/update`.
///
/// Optionally, they **MAY** support other session methods and notifications by specifying additional capabilities.
///
/// Note: `session/load` is still handled by the top-level `load_session` capability. This will be unified in future versions of the protocol.
///
/// See protocol docs: [Session Capabilities](https://agentclientprotocol.com/protocol/initialization#session-capabilities)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionCapabilities {
    /// Whether the agent supports `session/list`.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports listing sessions.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub list: Option<SessionListCapabilities>,
    /// Whether the agent supports `session/delete`.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports deleting sessions from `session/list`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub delete: Option<SessionDeleteCapabilities>,
    /// Whether the agent supports `additionalDirectories` on supported session lifecycle requests.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports `additionalDirectories` on
    /// supported session lifecycle requests.
    ///
    /// Agents that also support `session/list` may return
    /// `SessionInfo.additionalDirectories` to report the complete ordered
    /// additional-root list associated with a listed session.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub additional_directories: Option<SessionAdditionalDirectoriesCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Whether the agent supports `session/fork`.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports forking sessions.
    #[cfg(feature = "unstable_session_fork")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub fork: Option<SessionForkCapabilities>,
    /// Whether the agent supports `session/resume`.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports resuming sessions.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub resume: Option<SessionResumeCapabilities>,
    /// Whether the agent supports `session/close`.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports closing sessions.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub close: Option<SessionCloseCapabilities>,
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

impl SessionCapabilities {
    /// Builds an empty [`SessionCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the agent supports `session/list`.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports listing sessions.
    #[must_use]
    pub fn list(mut self, list: impl IntoOption<SessionListCapabilities>) -> Self {
        self.list = list.into_option();
        self
    }

    /// Whether the agent supports `session/delete`.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports deleting sessions from `session/list`.
    #[must_use]
    pub fn delete(mut self, delete: impl IntoOption<SessionDeleteCapabilities>) -> Self {
        self.delete = delete.into_option();
        self
    }

    /// Whether the agent supports `additionalDirectories` on supported session lifecycle requests.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports `additionalDirectories` on
    /// supported session lifecycle requests.
    ///
    /// Agents that also support `session/list` may return
    /// `SessionInfo.additionalDirectories` to report the complete ordered
    /// additional-root list associated with a listed session.
    #[must_use]
    pub fn additional_directories(
        mut self,
        additional_directories: impl IntoOption<SessionAdditionalDirectoriesCapabilities>,
    ) -> Self {
        self.additional_directories = additional_directories.into_option();
        self
    }

    #[cfg(feature = "unstable_session_fork")]
    /// Whether the agent supports `session/fork`.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports forking sessions.
    #[must_use]
    pub fn fork(mut self, fork: impl IntoOption<SessionForkCapabilities>) -> Self {
        self.fork = fork.into_option();
        self
    }

    /// Whether the agent supports `session/resume`.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports resuming sessions.
    #[must_use]
    pub fn resume(mut self, resume: impl IntoOption<SessionResumeCapabilities>) -> Self {
        self.resume = resume.into_option();
        self
    }

    /// Whether the agent supports `session/close`.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports closing sessions.
    #[must_use]
    pub fn close(mut self, close: impl IntoOption<SessionCloseCapabilities>) -> Self {
        self.close = close.into_option();
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

/// Capabilities for the `session/list` method.
///
/// Supplying `{}` means the agent supports listing sessions.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionListCapabilities {
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

impl SessionListCapabilities {
    /// Builds an empty [`SessionListCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for the `session/delete` method.
///
/// Supplying `{}` means the agent supports deleting sessions from `session/list`.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionDeleteCapabilities {
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

impl SessionDeleteCapabilities {
    /// Builds an empty [`SessionDeleteCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for additional session directories support.
///
/// Supplying `{}` means the agent supports the `additionalDirectories` field on
/// supported session lifecycle requests. Agents that also support
/// `session/list` may return `SessionInfo.additionalDirectories` to report the
/// complete ordered additional-root list associated with a listed session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionAdditionalDirectoriesCapabilities {
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

impl SessionAdditionalDirectoriesCapabilities {
    /// Builds an empty [`SessionAdditionalDirectoriesCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Capabilities for the `session/fork` method.
///
/// Supplying `{}` means the agent supports forking sessions.
#[cfg(feature = "unstable_session_fork")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionForkCapabilities {
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

#[cfg(feature = "unstable_session_fork")]
impl SessionForkCapabilities {
    /// Builds an empty [`SessionForkCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for the `session/resume` method.
///
/// Supplying `{}` means the agent supports resuming sessions.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionResumeCapabilities {
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

impl SessionResumeCapabilities {
    /// Builds an empty [`SessionResumeCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for the `session/close` method.
///
/// Supplying `{}` means the agent supports closing sessions.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionCloseCapabilities {
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

impl SessionCloseCapabilities {
    /// Builds an empty [`SessionCloseCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Prompt capabilities supported by the agent in `session/prompt` requests.
///
/// Baseline agent functionality requires support for [`ContentBlock::Text`]
/// and [`ContentBlock::ResourceLink`] in prompt requests.
///
/// Other variants must be explicitly opted in to.
/// Capabilities for different types of content in prompt requests.
///
/// Indicates which content types beyond the baseline (text and resource links)
/// the agent can process.
///
/// See protocol docs: [Prompt Capabilities](https://agentclientprotocol.com/protocol/initialization#prompt-capabilities)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PromptCapabilities {
    /// Agent supports [`ContentBlock::Image`].
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub image: bool,
    /// Agent supports [`ContentBlock::Audio`].
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub audio: bool,
    /// Agent supports embedded context in `session/prompt` requests.
    ///
    /// When enabled, the Client is allowed to include [`ContentBlock::Resource`]
    /// in prompt requests for pieces of context that are referenced in the message.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub embedded_context: bool,
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

impl PromptCapabilities {
    /// Builds an empty [`PromptCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Agent supports [`ContentBlock::Image`].
    #[must_use]
    pub fn image(mut self, image: bool) -> Self {
        self.image = image;
        self
    }

    /// Agent supports [`ContentBlock::Audio`].
    #[must_use]
    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = audio;
        self
    }

    /// Agent supports embedded context in `session/prompt` requests.
    ///
    /// When enabled, the Client is allowed to include [`ContentBlock::Resource`]
    /// in prompt requests for pieces of context that are referenced in the message.
    #[must_use]
    pub fn embedded_context(mut self, embedded_context: bool) -> Self {
        self.embedded_context = embedded_context;
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

/// MCP capabilities supported by the agent
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct McpCapabilities {
    /// Agent supports [`McpServer::Http`].
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub http: bool,
    /// Agent supports [`McpServer::Sse`].
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub sse: bool,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Agent supports [`McpServer::Acp`].
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub acp: bool,
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

impl McpCapabilities {
    /// Builds an empty [`McpCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Agent supports [`McpServer::Http`].
    #[must_use]
    pub fn http(mut self, http: bool) -> Self {
        self.http = http;
        self
    }

    /// Agent supports [`McpServer::Sse`].
    #[must_use]
    pub fn sse(mut self, sse: bool) -> Self {
        self.sse = sse;
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Agent supports [`McpServer::Acp`].
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[must_use]
    pub fn acp(mut self, acp: bool) -> Self {
        self.acp = acp;
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

// Method schema

/// Names of all methods that agents handle.
///
/// Provides a centralized definition of method names used in the protocol.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct AgentMethodNames {
    /// Method for initializing the connection.
    pub initialize: &'static str,
    /// Method for authenticating with the agent.
    pub authenticate: &'static str,
    /// Method for listing configurable providers.
    #[cfg(feature = "unstable_llm_providers")]
    pub providers_list: &'static str,
    /// Method for setting provider configuration.
    #[cfg(feature = "unstable_llm_providers")]
    pub providers_set: &'static str,
    /// Method for disabling a provider.
    #[cfg(feature = "unstable_llm_providers")]
    pub providers_disable: &'static str,
    /// Method for creating a new session.
    pub session_new: &'static str,
    /// Method for loading an existing session.
    pub session_load: &'static str,
    /// Method for setting the mode for a session.
    pub session_set_mode: &'static str,
    /// Method for setting a configuration option for a session.
    pub session_set_config_option: &'static str,
    /// Method for sending a prompt to the agent.
    pub session_prompt: &'static str,
    /// Notification for cancelling operations.
    pub session_cancel: &'static str,
    /// Method for exchanging MCP-over-ACP messages.
    #[cfg(feature = "unstable_mcp_over_acp")]
    pub mcp_message: &'static str,
    /// Method for listing existing sessions.
    pub session_list: &'static str,
    /// Method for deleting an existing session.
    pub session_delete: &'static str,
    /// Method for forking an existing session.
    #[cfg(feature = "unstable_session_fork")]
    pub session_fork: &'static str,
    /// Method for resuming an existing session.
    pub session_resume: &'static str,
    /// Method for closing an active session.
    pub session_close: &'static str,
    /// Method for logging out of an authenticated session.
    pub logout: &'static str,
    /// Method for starting an NES session.
    #[cfg(feature = "unstable_nes")]
    pub nes_start: &'static str,
    /// Method for requesting a suggestion.
    #[cfg(feature = "unstable_nes")]
    pub nes_suggest: &'static str,
    /// Notification for accepting a suggestion.
    #[cfg(feature = "unstable_nes")]
    pub nes_accept: &'static str,
    /// Notification for rejecting a suggestion.
    #[cfg(feature = "unstable_nes")]
    pub nes_reject: &'static str,
    /// Method for closing an NES session.
    #[cfg(feature = "unstable_nes")]
    pub nes_close: &'static str,
    /// Notification for document open events.
    #[cfg(feature = "unstable_nes")]
    pub document_did_open: &'static str,
    /// Notification for document change events.
    #[cfg(feature = "unstable_nes")]
    pub document_did_change: &'static str,
    /// Notification for document close events.
    #[cfg(feature = "unstable_nes")]
    pub document_did_close: &'static str,
    /// Notification for document save events.
    #[cfg(feature = "unstable_nes")]
    pub document_did_save: &'static str,
    /// Notification for document focus events.
    #[cfg(feature = "unstable_nes")]
    pub document_did_focus: &'static str,
}

/// Constant containing all agent method names.
pub const AGENT_METHOD_NAMES: AgentMethodNames = AgentMethodNames {
    initialize: INITIALIZE_METHOD_NAME,
    authenticate: AUTHENTICATE_METHOD_NAME,
    #[cfg(feature = "unstable_llm_providers")]
    providers_list: PROVIDERS_LIST_METHOD_NAME,
    #[cfg(feature = "unstable_llm_providers")]
    providers_set: PROVIDERS_SET_METHOD_NAME,
    #[cfg(feature = "unstable_llm_providers")]
    providers_disable: PROVIDERS_DISABLE_METHOD_NAME,
    session_new: SESSION_NEW_METHOD_NAME,
    session_load: SESSION_LOAD_METHOD_NAME,
    session_set_mode: SESSION_SET_MODE_METHOD_NAME,
    session_set_config_option: SESSION_SET_CONFIG_OPTION_METHOD_NAME,
    session_prompt: SESSION_PROMPT_METHOD_NAME,
    session_cancel: SESSION_CANCEL_METHOD_NAME,
    #[cfg(feature = "unstable_mcp_over_acp")]
    mcp_message: MCP_MESSAGE_METHOD_NAME,
    session_list: SESSION_LIST_METHOD_NAME,
    session_delete: SESSION_DELETE_METHOD_NAME,
    #[cfg(feature = "unstable_session_fork")]
    session_fork: SESSION_FORK_METHOD_NAME,
    session_resume: SESSION_RESUME_METHOD_NAME,
    session_close: SESSION_CLOSE_METHOD_NAME,
    logout: LOGOUT_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    nes_start: NES_START_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    nes_suggest: NES_SUGGEST_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    nes_accept: NES_ACCEPT_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    nes_reject: NES_REJECT_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    nes_close: NES_CLOSE_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    document_did_open: DOCUMENT_DID_OPEN_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    document_did_change: DOCUMENT_DID_CHANGE_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    document_did_close: DOCUMENT_DID_CLOSE_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    document_did_save: DOCUMENT_DID_SAVE_METHOD_NAME,
    #[cfg(feature = "unstable_nes")]
    document_did_focus: DOCUMENT_DID_FOCUS_METHOD_NAME,
};

/// Method name for the initialize request.
pub(crate) const INITIALIZE_METHOD_NAME: &str = "initialize";
/// Method name for the authenticate request.
pub(crate) const AUTHENTICATE_METHOD_NAME: &str = "authenticate";
/// Method name for listing configurable providers.
#[cfg(feature = "unstable_llm_providers")]
pub(crate) const PROVIDERS_LIST_METHOD_NAME: &str = "providers/list";
/// Method name for setting provider configuration.
#[cfg(feature = "unstable_llm_providers")]
pub(crate) const PROVIDERS_SET_METHOD_NAME: &str = "providers/set";
/// Method name for disabling a provider.
#[cfg(feature = "unstable_llm_providers")]
pub(crate) const PROVIDERS_DISABLE_METHOD_NAME: &str = "providers/disable";
/// Method name for creating a new session.
pub(crate) const SESSION_NEW_METHOD_NAME: &str = "session/new";
/// Method name for loading an existing session.
pub(crate) const SESSION_LOAD_METHOD_NAME: &str = "session/load";
/// Method name for setting the mode for a session.
pub(crate) const SESSION_SET_MODE_METHOD_NAME: &str = "session/set_mode";
/// Method name for setting a configuration option for a session.
pub(crate) const SESSION_SET_CONFIG_OPTION_METHOD_NAME: &str = "session/set_config_option";
/// Method name for sending a prompt.
pub(crate) const SESSION_PROMPT_METHOD_NAME: &str = "session/prompt";
/// Method name for the cancel notification.
pub(crate) const SESSION_CANCEL_METHOD_NAME: &str = "session/cancel";
/// Method name for listing existing sessions.
pub(crate) const SESSION_LIST_METHOD_NAME: &str = "session/list";
/// Method name for deleting an existing session.
pub(crate) const SESSION_DELETE_METHOD_NAME: &str = "session/delete";
/// Method name for forking an existing session.
#[cfg(feature = "unstable_session_fork")]
pub(crate) const SESSION_FORK_METHOD_NAME: &str = "session/fork";
/// Method name for resuming an existing session.
pub(crate) const SESSION_RESUME_METHOD_NAME: &str = "session/resume";
/// Method name for closing an active session.
pub(crate) const SESSION_CLOSE_METHOD_NAME: &str = "session/close";
/// Method name for logging out of an authenticated session.
pub(crate) const LOGOUT_METHOD_NAME: &str = "logout";

/// All possible requests that a client can send to an agent.
///
/// This enum is used internally for routing RPC requests. You typically won't need
/// to use this directly.
///
/// This enum encompasses all method calls from client to agent.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum ClientRequest {
    /// Establishes the connection with a client and negotiates protocol capabilities.
    ///
    /// This method is called once at the beginning of the connection to:
    /// - Negotiate the protocol version to use
    /// - Exchange capability information between client and agent
    /// - Determine available authentication methods
    ///
    /// The agent should respond with its supported protocol version and capabilities.
    ///
    /// See protocol docs: [Initialization](https://agentclientprotocol.com/protocol/initialization)
    InitializeRequest(InitializeRequest),
    /// Authenticates the client using the specified authentication method.
    ///
    /// Called when the agent requires authentication before allowing session creation.
    /// The client provides the authentication method ID that was advertised during initialization.
    ///
    /// After successful authentication, the client can proceed to create sessions with
    /// `new_session` without receiving an `auth_required` error.
    ///
    /// See protocol docs: [Initialization](https://agentclientprotocol.com/protocol/initialization)
    AuthenticateRequest(AuthenticateRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Lists providers that can be configured by the client.
    #[cfg(feature = "unstable_llm_providers")]
    ListProvidersRequest(ListProvidersRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Replaces the configuration for a provider.
    #[cfg(feature = "unstable_llm_providers")]
    SetProviderRequest(SetProviderRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Disables a provider.
    #[cfg(feature = "unstable_llm_providers")]
    DisableProviderRequest(DisableProviderRequest),
    /// Logs out of the current authenticated state.
    ///
    /// After a successful logout, all new sessions will require authentication.
    /// There is no guarantee about the behavior of already running sessions.
    LogoutRequest(LogoutRequest),
    /// Creates a new conversation session with the agent.
    ///
    /// Sessions represent independent conversation contexts with their own history and state.
    ///
    /// The agent should:
    /// - Create a new session context
    /// - Connect to any specified MCP servers
    /// - Return a unique session ID for future requests
    ///
    /// May return an `auth_required` error if the agent requires authentication.
    ///
    /// See protocol docs: [Session Setup](https://agentclientprotocol.com/protocol/session-setup)
    NewSessionRequest(NewSessionRequest),
    /// Loads an existing session to resume a previous conversation.
    ///
    /// This method is only available if the agent advertises the `loadSession` capability.
    ///
    /// The agent should:
    /// - Restore the session context and conversation history
    /// - Connect to the specified MCP servers
    /// - Stream the entire conversation history back to the client via notifications
    ///
    /// See protocol docs: [Loading Sessions](https://agentclientprotocol.com/protocol/session-setup#loading-sessions)
    LoadSessionRequest(LoadSessionRequest),
    /// Lists existing sessions known to the agent.
    ///
    /// This method is only available if the agent advertises the `sessionCapabilities.list` capability.
    ///
    /// The agent should return metadata about sessions with optional filtering and pagination support.
    ListSessionsRequest(ListSessionsRequest),
    /// Deletes an existing session from `session/list`.
    ///
    /// This method is only available if the agent advertises the `sessionCapabilities.delete` capability.
    DeleteSessionRequest(DeleteSessionRequest),
    #[cfg(feature = "unstable_session_fork")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Forks an existing session to create a new independent session.
    ///
    /// This method is only available if the agent advertises the `session.fork` capability.
    ///
    /// The agent should create a new session with the same conversation context as the
    /// original, allowing operations like generating summaries without affecting the
    /// original session's history.
    ForkSessionRequest(ForkSessionRequest),
    /// Resumes an existing session without returning previous messages.
    ///
    /// This method is only available if the agent advertises the `sessionCapabilities.resume` capability.
    ///
    /// The agent should resume the session context, allowing the conversation to continue
    /// without replaying the message history (unlike `session/load`).
    ResumeSessionRequest(ResumeSessionRequest),
    /// Closes an active session and frees up any resources associated with it.
    ///
    /// This method is only available if the agent advertises the `sessionCapabilities.close` capability.
    ///
    /// The agent must cancel any ongoing work (as if `session/cancel` was called)
    /// and then free up any resources associated with the session.
    CloseSessionRequest(CloseSessionRequest),
    /// Sets the current mode for a session.
    ///
    /// Allows switching between different agent modes (e.g., "ask", "architect", "code")
    /// that affect system prompts, tool availability, and permission behaviors.
    ///
    /// The mode must be one of the modes advertised in `availableModes` during session
    /// creation or loading. Agents may also change modes autonomously and notify the
    /// client via `current_mode_update` notifications.
    ///
    /// This method can be called at any time during a session, whether the Agent is
    /// idle or actively generating a response.
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    SetSessionModeRequest(SetSessionModeRequest),
    /// Sets the current value for a session configuration option.
    SetSessionConfigOptionRequest(SetSessionConfigOptionRequest),
    /// Processes a user prompt within a session.
    ///
    /// This method handles the whole lifecycle of a prompt:
    /// - Receives user messages with optional context (files, images, etc.)
    /// - Processes the prompt using language models
    /// - Reports language model content and tool calls to the Clients
    /// - Requests permission to run tools
    /// - Executes any requested tool calls
    /// - Returns when the turn is complete with a stop reason
    ///
    /// See protocol docs: [Prompt Turn](https://agentclientprotocol.com/protocol/prompt-turn)
    PromptRequest(PromptRequest),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Starts an NES session.
    StartNesRequest(StartNesRequest),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Requests a code suggestion.
    SuggestNesRequest(SuggestNesRequest),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Closes an active NES session and frees up any resources associated with it.
    ///
    /// The agent must cancel any ongoing work and then free up any resources
    /// associated with the NES session.
    CloseNesRequest(CloseNesRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Exchanges an MCP-over-ACP message.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpRequest(MessageMcpRequest),
    /// Handles extension method requests from the client.
    ///
    /// Extension methods provide a way to add custom functionality while maintaining
    /// protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtMethodRequest(ExtRequest),
}

impl ClientRequest {
    /// Returns the corresponding method name of the request.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::InitializeRequest(_) => AGENT_METHOD_NAMES.initialize,
            Self::AuthenticateRequest(_) => AGENT_METHOD_NAMES.authenticate,
            #[cfg(feature = "unstable_llm_providers")]
            Self::ListProvidersRequest(_) => AGENT_METHOD_NAMES.providers_list,
            #[cfg(feature = "unstable_llm_providers")]
            Self::SetProviderRequest(_) => AGENT_METHOD_NAMES.providers_set,
            #[cfg(feature = "unstable_llm_providers")]
            Self::DisableProviderRequest(_) => AGENT_METHOD_NAMES.providers_disable,
            Self::LogoutRequest(_) => AGENT_METHOD_NAMES.logout,
            Self::NewSessionRequest(_) => AGENT_METHOD_NAMES.session_new,
            Self::LoadSessionRequest(_) => AGENT_METHOD_NAMES.session_load,
            Self::ListSessionsRequest(_) => AGENT_METHOD_NAMES.session_list,
            Self::DeleteSessionRequest(_) => AGENT_METHOD_NAMES.session_delete,
            #[cfg(feature = "unstable_session_fork")]
            Self::ForkSessionRequest(_) => AGENT_METHOD_NAMES.session_fork,
            Self::ResumeSessionRequest(_) => AGENT_METHOD_NAMES.session_resume,
            Self::CloseSessionRequest(_) => AGENT_METHOD_NAMES.session_close,
            Self::SetSessionModeRequest(_) => AGENT_METHOD_NAMES.session_set_mode,
            Self::SetSessionConfigOptionRequest(_) => AGENT_METHOD_NAMES.session_set_config_option,
            Self::PromptRequest(_) => AGENT_METHOD_NAMES.session_prompt,
            #[cfg(feature = "unstable_nes")]
            Self::StartNesRequest(_) => AGENT_METHOD_NAMES.nes_start,
            #[cfg(feature = "unstable_nes")]
            Self::SuggestNesRequest(_) => AGENT_METHOD_NAMES.nes_suggest,
            #[cfg(feature = "unstable_nes")]
            Self::CloseNesRequest(_) => AGENT_METHOD_NAMES.nes_close,
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::MessageMcpRequest(_) => AGENT_METHOD_NAMES.mcp_message,
            Self::ExtMethodRequest(ext_request) => &ext_request.method,
        }
    }
}

/// All possible responses that an agent can send to a client.
///
/// This enum is used internally for routing RPC responses. You typically won't need
/// to use this directly - the responses are handled automatically by the connection.
///
/// These are responses to the corresponding `ClientRequest` variants.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum AgentResponse {
    /// Successful result returned for a `initialize` request.
    InitializeResponse(InitializeResponse),
    /// Successful result returned for a `authenticate` request.
    AuthenticateResponse(#[serde(default)] AuthenticateResponse),
    /// Successful result returned for a `providers/list` request.
    #[cfg(feature = "unstable_llm_providers")]
    ListProvidersResponse(ListProvidersResponse),
    /// Successful result returned for a `providers/set` request.
    #[cfg(feature = "unstable_llm_providers")]
    SetProviderResponse(#[serde(default)] SetProviderResponse),
    /// Successful result returned for a `providers/disable` request.
    #[cfg(feature = "unstable_llm_providers")]
    DisableProviderResponse(#[serde(default)] DisableProviderResponse),
    /// Successful result returned for a `logout` request.
    LogoutResponse(#[serde(default)] LogoutResponse),
    /// Successful result returned for a `session/new` request.
    NewSessionResponse(NewSessionResponse),
    /// Successful result returned for a `session/load` request.
    LoadSessionResponse(#[serde(default)] LoadSessionResponse),
    /// Successful result returned for a `session/list` request.
    ListSessionsResponse(ListSessionsResponse),
    /// Successful result returned for a `session/delete` request.
    DeleteSessionResponse(#[serde(default)] DeleteSessionResponse),
    /// Successful result returned for a `session/fork` request.
    #[cfg(feature = "unstable_session_fork")]
    ForkSessionResponse(ForkSessionResponse),
    /// Successful result returned for a `session/resume` request.
    ResumeSessionResponse(#[serde(default)] ResumeSessionResponse),
    /// Successful result returned for a `session/close` request.
    CloseSessionResponse(#[serde(default)] CloseSessionResponse),
    /// Successful result returned for a `session/set_mode` request.
    SetSessionModeResponse(#[serde(default)] SetSessionModeResponse),
    /// Successful result returned for a `session/set_config_option` request.
    SetSessionConfigOptionResponse(SetSessionConfigOptionResponse),
    /// Successful result returned for a `session/prompt` request.
    PromptResponse(PromptResponse),
    /// Successful result returned for a `nes/start` request.
    #[cfg(feature = "unstable_nes")]
    StartNesResponse(StartNesResponse),
    /// Successful result returned for a `nes/suggest` request.
    #[cfg(feature = "unstable_nes")]
    SuggestNesResponse(SuggestNesResponse),
    /// Successful result returned for a `nes/close` request.
    #[cfg(feature = "unstable_nes")]
    CloseNesResponse(#[serde(default)] CloseNesResponse),
    /// Successful result returned by an extension method outside the core ACP method set.
    ExtMethodResponse(ExtResponse),
    /// Successful result returned by an MCP-over-ACP `mcp/message` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpResponse(MessageMcpResponse),
}

/// All possible notifications that a client can send to an agent.
///
/// This enum is used internally for routing RPC notifications. You typically won't need
/// to use this directly.
///
/// Notifications do not expect a response.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum ClientNotification {
    /// Cancels ongoing operations for a session.
    ///
    /// This is a notification sent by the client to cancel an ongoing prompt turn.
    ///
    /// Upon receiving this notification, the Agent SHOULD:
    /// - Stop all language model requests as soon as possible
    /// - Abort all tool call invocations in progress
    /// - Send any pending `session/update` notifications
    /// - Respond to the original `session/prompt` request with `StopReason::Cancelled`
    ///
    /// See protocol docs: [Cancellation](https://agentclientprotocol.com/protocol/prompt-turn#cancellation)
    CancelNotification(CancelNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is opened in the editor.
    DidOpenDocumentNotification(DidOpenDocumentNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is edited.
    DidChangeDocumentNotification(DidChangeDocumentNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is closed.
    DidCloseDocumentNotification(DidCloseDocumentNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is saved.
    DidSaveDocumentNotification(DidSaveDocumentNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file becomes the active editor tab.
    DidFocusDocumentNotification(DidFocusDocumentNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a suggestion is accepted.
    AcceptNesNotification(AcceptNesNotification),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a suggestion is rejected.
    RejectNesNotification(RejectNesNotification),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Sends an MCP-over-ACP notification.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpNotification(MessageMcpNotification),
    /// Handles extension notifications from the client.
    ///
    /// Extension notifications provide a way to send one-way messages for custom functionality
    /// while maintaining protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtNotification(ExtNotification),
}

impl ClientNotification {
    /// Returns the corresponding method name of the notification.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::CancelNotification(_) => AGENT_METHOD_NAMES.session_cancel,
            #[cfg(feature = "unstable_nes")]
            Self::DidOpenDocumentNotification(_) => AGENT_METHOD_NAMES.document_did_open,
            #[cfg(feature = "unstable_nes")]
            Self::DidChangeDocumentNotification(_) => AGENT_METHOD_NAMES.document_did_change,
            #[cfg(feature = "unstable_nes")]
            Self::DidCloseDocumentNotification(_) => AGENT_METHOD_NAMES.document_did_close,
            #[cfg(feature = "unstable_nes")]
            Self::DidSaveDocumentNotification(_) => AGENT_METHOD_NAMES.document_did_save,
            #[cfg(feature = "unstable_nes")]
            Self::DidFocusDocumentNotification(_) => AGENT_METHOD_NAMES.document_did_focus,
            #[cfg(feature = "unstable_nes")]
            Self::AcceptNesNotification(_) => AGENT_METHOD_NAMES.nes_accept,
            #[cfg(feature = "unstable_nes")]
            Self::RejectNesNotification(_) => AGENT_METHOD_NAMES.nes_reject,
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::MessageMcpNotification(_) => AGENT_METHOD_NAMES.mcp_message,
            Self::ExtNotification(ext_notification) => &ext_notification.method,
        }
    }
}

/// Notification to cancel ongoing operations for a session.
///
/// See protocol docs: [Cancellation](https://agentclientprotocol.com/protocol/prompt-turn#cancellation)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_CANCEL_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelNotification {
    /// The ID of the session to cancel operations for.
    pub session_id: SessionId,
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

impl CancelNotification {
    /// Builds [`CancelNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
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

#[cfg(test)]
mod test_serialization {
    use super::*;
    use serde_json::json;

    fn test_meta() -> Meta {
        json!({ "source": "test" }).as_object().unwrap().clone()
    }

    fn serialized_meta_key_count(value: &impl serde::Serialize) -> usize {
        serde_json::to_string(value)
            .unwrap()
            .matches("\"_meta\"")
            .count()
    }

    #[test]
    fn test_initialize_capabilities_default_on_malformed_values() {
        let request: InitializeRequest = serde_json::from_value(json!({
            "protocolVersion": 1,
            "clientCapabilities": false
        }))
        .unwrap();
        assert_eq!(request.client_capabilities, ClientCapabilities::default());

        let response: InitializeResponse = serde_json::from_value(json!({
            "protocolVersion": 1,
            "agentCapabilities": false
        }))
        .unwrap();
        assert_eq!(response.agent_capabilities, AgentCapabilities::default());
    }

    #[test]
    fn test_agent_capabilities_default_on_malformed_values() {
        let capabilities: AgentCapabilities = serde_json::from_value(json!({
            "loadSession": "yes",
            "promptCapabilities": {
                "image": "yes",
                "audio": true,
                "embeddedContext": {}
            },
            "mcpCapabilities": {
                "http": "yes",
                "sse": true
            },
            "sessionCapabilities": false,
            "auth": false
        }))
        .unwrap();

        assert!(!capabilities.load_session);
        assert!(!capabilities.prompt_capabilities.image);
        assert!(capabilities.prompt_capabilities.audio);
        assert!(!capabilities.prompt_capabilities.embedded_context);
        assert!(!capabilities.mcp_capabilities.http);
        assert!(capabilities.mcp_capabilities.sse);
        assert_eq!(
            capabilities.session_capabilities,
            SessionCapabilities::default()
        );
        assert_eq!(capabilities.auth, AgentAuthCapabilities::default());
    }

    #[test]
    fn test_mcp_server_stdio_serialization() {
        let server = McpServer::Stdio(
            McpServerStdio::new("test-server", "/usr/bin/server")
                .args(vec!["--port".to_string(), "3000".to_string()])
                .env(vec![EnvVariable::new("API_KEY", "secret123")]),
        );

        let json = serde_json::to_value(&server).unwrap();
        assert_eq!(
            json,
            json!({
                "name": "test-server",
                "command": "/usr/bin/server",
                "args": ["--port", "3000"],
                "env": [
                    {
                        "name": "API_KEY",
                        "value": "secret123"
                    }
                ]
            })
        );

        let deserialized: McpServer = serde_json::from_value(json).unwrap();
        match deserialized {
            McpServer::Stdio(McpServerStdio {
                name,
                command,
                args,
                env,
                meta: _,
            }) => {
                assert_eq!(name, "test-server");
                assert_eq!(command, PathBuf::from("/usr/bin/server"));
                assert_eq!(args, vec!["--port", "3000"]);
                assert_eq!(env.len(), 1);
                assert_eq!(env[0].name, "API_KEY");
                assert_eq!(env[0].value, "secret123");
            }
            _ => panic!("Expected Stdio variant"),
        }
    }

    #[test]
    fn test_mcp_server_http_serialization() {
        let server = McpServer::Http(
            McpServerHttp::new("http-server", "https://api.example.com").headers(vec![
                HttpHeader::new("Authorization", "Bearer token123"),
                HttpHeader::new("Content-Type", "application/json"),
            ]),
        );

        let json = serde_json::to_value(&server).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "http",
                "name": "http-server",
                "url": "https://api.example.com",
                "headers": [
                    {
                        "name": "Authorization",
                        "value": "Bearer token123"
                    },
                    {
                        "name": "Content-Type",
                        "value": "application/json"
                    }
                ]
            })
        );

        let deserialized: McpServer = serde_json::from_value(json).unwrap();
        match deserialized {
            McpServer::Http(McpServerHttp {
                name,
                url,
                headers,
                meta: _,
            }) => {
                assert_eq!(name, "http-server");
                assert_eq!(url, "https://api.example.com");
                assert_eq!(headers.len(), 2);
                assert_eq!(headers[0].name, "Authorization");
                assert_eq!(headers[0].value, "Bearer token123");
                assert_eq!(headers[1].name, "Content-Type");
                assert_eq!(headers[1].value, "application/json");
            }
            _ => panic!("Expected Http variant"),
        }
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn test_mcp_server_acp_serialization() {
        let server = McpServer::Acp(McpServerAcp::new("project-tools", "project-tools-id"));

        let json = serde_json::to_value(&server).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "acp",
                "name": "project-tools",
                "serverId": "project-tools-id"
            })
        );

        let deserialized: McpServer = serde_json::from_value(json).unwrap();
        match deserialized {
            McpServer::Acp(McpServerAcp {
                name,
                server_id: id,
                meta: _,
            }) => {
                assert_eq!(name, "project-tools");
                assert_eq!(id, McpServerAcpId::new("project-tools-id"));
            }
            _ => panic!("Expected Acp variant"),
        }
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn test_client_mcp_message_method_names() {
        assert_eq!(AGENT_METHOD_NAMES.mcp_message, "mcp/message");

        assert_eq!(
            ClientRequest::MessageMcpRequest(MessageMcpRequest::new("conn-1", "tools/list"))
                .method(),
            "mcp/message"
        );
        assert_eq!(
            ClientNotification::MessageMcpNotification(MessageMcpNotification::new(
                "conn-1",
                "notifications/progress"
            ))
            .method(),
            "mcp/message"
        );
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn test_mcp_server_acp_schema() {
        let mcp_server_schema = serde_json::to_value(schemars::schema_for!(McpServer)).unwrap();
        assert!(json_contains_entry(
            &mcp_server_schema,
            "const",
            &json!("acp")
        ));
        assert!(json_contains_entry(
            &mcp_server_schema,
            "$ref",
            &json!("#/$defs/McpServerAcp")
        ));

        let capabilities_schema =
            serde_json::to_value(schemars::schema_for!(McpCapabilities)).unwrap();
        assert!(json_contains_key(&capabilities_schema, "acp"));
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    fn json_contains_entry(
        value: &serde_json::Value,
        key: &str,
        expected: &serde_json::Value,
    ) -> bool {
        match value {
            serde_json::Value::Object(map) => {
                map.get(key) == Some(expected)
                    || map
                        .values()
                        .any(|value| json_contains_entry(value, key, expected))
            }
            serde_json::Value::Array(values) => values
                .iter()
                .any(|value| json_contains_entry(value, key, expected)),
            _ => false,
        }
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    fn json_contains_key(value: &serde_json::Value, key: &str) -> bool {
        match value {
            serde_json::Value::Object(map) => {
                map.contains_key(key) || map.values().any(|value| json_contains_key(value, key))
            }
            serde_json::Value::Array(values) => {
                values.iter().any(|value| json_contains_key(value, key))
            }
            _ => false,
        }
    }

    #[test]
    fn test_mcp_server_sse_serialization() {
        let server = McpServer::Sse(
            McpServerSse::new("sse-server", "https://sse.example.com/events")
                .headers(vec![HttpHeader::new("X-API-Key", "apikey456")]),
        );

        let json = serde_json::to_value(&server).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "sse",
                "name": "sse-server",
                "url": "https://sse.example.com/events",
                "headers": [
                    {
                        "name": "X-API-Key",
                        "value": "apikey456"
                    }
                ]
            })
        );

        let deserialized: McpServer = serde_json::from_value(json).unwrap();
        match deserialized {
            McpServer::Sse(McpServerSse {
                name,
                url,
                headers,
                meta: _,
            }) => {
                assert_eq!(name, "sse-server");
                assert_eq!(url, "https://sse.example.com/events");
                assert_eq!(headers.len(), 1);
                assert_eq!(headers[0].name, "X-API-Key");
                assert_eq!(headers[0].value, "apikey456");
            }
            _ => panic!("Expected Sse variant"),
        }
    }

    #[test]
    fn test_session_config_option_category_known_variants() {
        // Test serialization of known variants
        assert_eq!(
            serde_json::to_value(&SessionConfigOptionCategory::Mode).unwrap(),
            json!("mode")
        );
        assert_eq!(
            serde_json::to_value(&SessionConfigOptionCategory::Model).unwrap(),
            json!("model")
        );
        assert_eq!(
            serde_json::to_value(&SessionConfigOptionCategory::ModelConfig).unwrap(),
            json!("model_config")
        );
        assert_eq!(
            serde_json::to_value(&SessionConfigOptionCategory::ThoughtLevel).unwrap(),
            json!("thought_level")
        );

        // Test deserialization of known variants
        assert_eq!(
            serde_json::from_str::<SessionConfigOptionCategory>("\"mode\"").unwrap(),
            SessionConfigOptionCategory::Mode
        );
        assert_eq!(
            serde_json::from_str::<SessionConfigOptionCategory>("\"model\"").unwrap(),
            SessionConfigOptionCategory::Model
        );
        assert_eq!(
            serde_json::from_str::<SessionConfigOptionCategory>("\"model_config\"").unwrap(),
            SessionConfigOptionCategory::ModelConfig
        );
        assert_eq!(
            serde_json::from_str::<SessionConfigOptionCategory>("\"thought_level\"").unwrap(),
            SessionConfigOptionCategory::ThoughtLevel
        );
    }

    #[test]
    fn test_session_config_option_category_unknown_variants() {
        // Test that unknown strings are captured in Other variant
        let unknown: SessionConfigOptionCategory =
            serde_json::from_str("\"some_future_category\"").unwrap();
        assert_eq!(
            unknown,
            SessionConfigOptionCategory::Other("some_future_category".to_string())
        );

        // Test round-trip of unknown category
        let json = serde_json::to_value(&unknown).unwrap();
        assert_eq!(json, json!("some_future_category"));
    }

    #[test]
    fn test_session_config_option_category_custom_categories() {
        // Category names beginning with `_` are free for custom use
        let custom: SessionConfigOptionCategory =
            serde_json::from_str("\"_my_custom_category\"").unwrap();
        assert_eq!(
            custom,
            SessionConfigOptionCategory::Other("_my_custom_category".to_string())
        );

        // Test round-trip preserves the custom category name
        let json = serde_json::to_value(&custom).unwrap();
        assert_eq!(json, json!("_my_custom_category"));

        // Deserialize back and verify
        let deserialized: SessionConfigOptionCategory = serde_json::from_value(json).unwrap();
        assert_eq!(
            deserialized,
            SessionConfigOptionCategory::Other("_my_custom_category".to_string()),
        );
    }

    #[test]
    fn test_auth_method_agent_serialization() {
        let method = AuthMethod::Agent(AuthMethodAgent::new("default-auth", "Default Auth"));

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "default-auth",
                "name": "Default Auth"
            })
        );
        // description should be omitted when None
        assert!(!json.as_object().unwrap().contains_key("description"));
        // Agent variant should not emit a `type` field (backward compat)
        assert!(!json.as_object().unwrap().contains_key("type"));

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::Agent(AuthMethodAgent { id, name, .. }) => {
                assert_eq!(id.0.as_ref(), "default-auth");
                assert_eq!(name, "Default Auth");
            }
            #[cfg(feature = "unstable_auth_methods")]
            _ => panic!("Expected Agent variant"),
        }
    }

    #[test]
    fn test_auth_method_explicit_agent_deserialization() {
        // An explicit `"type": "agent"` should also deserialize to Agent
        let json = json!({
            "id": "agent-auth",
            "name": "Agent Auth",
            "type": "agent"
        });

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        assert!(matches!(deserialized, AuthMethod::Agent(_)));
    }

    #[test]
    fn test_session_delete_serialization() {
        assert_eq!(AGENT_METHOD_NAMES.session_delete, "session/delete");
        assert_eq!(
            ClientRequest::DeleteSessionRequest(DeleteSessionRequest::new("sess_abc123")).method(),
            "session/delete"
        );
        assert_eq!(
            serde_json::to_value(DeleteSessionRequest::new("sess_abc123")).unwrap(),
            json!({
                "sessionId": "sess_abc123"
            })
        );
        assert_eq!(
            serde_json::to_value(DeleteSessionResponse::new()).unwrap(),
            json!({})
        );
        assert_eq!(
            serde_json::to_value(
                SessionCapabilities::new().delete(SessionDeleteCapabilities::new())
            )
            .unwrap(),
            json!({
                "delete": {}
            })
        );
    }
    #[test]
    fn test_session_additional_directories_serialization() {
        assert_eq!(
            serde_json::to_value(NewSessionRequest::new("/home/user/project")).unwrap(),
            json!({
                "cwd": "/home/user/project",
                "mcpServers": []
            })
        );
        assert_eq!(
            serde_json::to_value(
                NewSessionRequest::new("/home/user/project").additional_directories(vec![
                    PathBuf::from("/home/user/shared-lib"),
                    PathBuf::from("/home/user/product-docs"),
                ])
            )
            .unwrap(),
            json!({
                "cwd": "/home/user/project",
                "additionalDirectories": [
                    "/home/user/shared-lib",
                    "/home/user/product-docs"
                ],
                "mcpServers": []
            })
        );
        assert_eq!(
            serde_json::to_value(SessionInfo::new("sess_abc123", "/home/user/project")).unwrap(),
            json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project"
            })
        );
        assert_eq!(
            serde_json::to_value(
                SessionInfo::new("sess_abc123", "/home/user/project").additional_directories(vec![
                    PathBuf::from("/home/user/shared-lib"),
                    PathBuf::from("/home/user/product-docs"),
                ])
            )
            .unwrap(),
            json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project",
                "additionalDirectories": [
                    "/home/user/shared-lib",
                    "/home/user/product-docs"
                ]
            })
        );
        assert_eq!(
            serde_json::from_value::<SessionInfo>(json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project"
            }))
            .unwrap()
            .additional_directories,
            Vec::<PathBuf>::new()
        );
    }
    #[test]
    fn test_session_additional_directories_capabilities_serialization() {
        assert_eq!(
            serde_json::to_value(
                SessionCapabilities::new()
                    .additional_directories(SessionAdditionalDirectoriesCapabilities::new())
            )
            .unwrap(),
            json!({
                "additionalDirectories": {}
            })
        );
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_method_env_var_serialization() {
        let method = AuthMethod::EnvVar(AuthMethodEnvVar::new(
            "api-key",
            "API Key",
            vec![AuthEnvVar::new("API_KEY")],
        ));

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "api-key",
                "name": "API Key",
                "type": "env_var",
                "vars": [{"name": "API_KEY"}]
            })
        );
        // secret defaults to true and should be omitted; optional defaults to false and should be omitted
        assert!(!json["vars"][0].as_object().unwrap().contains_key("secret"));
        assert!(
            !json["vars"][0]
                .as_object()
                .unwrap()
                .contains_key("optional")
        );

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::EnvVar(AuthMethodEnvVar {
                id,
                name: method_name,
                vars,
                link,
                ..
            }) => {
                assert_eq!(id.0.as_ref(), "api-key");
                assert_eq!(method_name, "API Key");
                assert_eq!(vars.len(), 1);
                assert_eq!(vars[0].name, "API_KEY");
                assert!(vars[0].secret);
                assert!(!vars[0].optional);
                assert!(link.is_none());
            }
            _ => panic!("Expected EnvVar variant"),
        }
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_method_env_var_with_link_serialization() {
        let method = AuthMethod::EnvVar(
            AuthMethodEnvVar::new("api-key", "API Key", vec![AuthEnvVar::new("API_KEY")])
                .link("https://example.com/keys"),
        );

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "api-key",
                "name": "API Key",
                "type": "env_var",
                "vars": [{"name": "API_KEY"}],
                "link": "https://example.com/keys"
            })
        );

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::EnvVar(AuthMethodEnvVar { link, .. }) => {
                assert_eq!(link.as_deref(), Some("https://example.com/keys"));
            }
            _ => panic!("Expected EnvVar variant"),
        }
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_method_env_var_multiple_vars() {
        let method = AuthMethod::EnvVar(AuthMethodEnvVar::new(
            "azure-openai",
            "Azure OpenAI",
            vec![
                AuthEnvVar::new("AZURE_OPENAI_API_KEY").label("API Key"),
                AuthEnvVar::new("AZURE_OPENAI_ENDPOINT")
                    .label("Endpoint URL")
                    .secret(false),
                AuthEnvVar::new("AZURE_OPENAI_API_VERSION")
                    .label("API Version")
                    .secret(false)
                    .optional(true),
            ],
        ));

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "azure-openai",
                "name": "Azure OpenAI",
                "type": "env_var",
                "vars": [
                    {"name": "AZURE_OPENAI_API_KEY", "label": "API Key"},
                    {"name": "AZURE_OPENAI_ENDPOINT", "label": "Endpoint URL", "secret": false},
                    {"name": "AZURE_OPENAI_API_VERSION", "label": "API Version", "secret": false, "optional": true}
                ]
            })
        );

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::EnvVar(AuthMethodEnvVar { vars, .. }) => {
                assert_eq!(vars.len(), 3);
                // First var: secret (default true), not optional (default false)
                assert_eq!(vars[0].name, "AZURE_OPENAI_API_KEY");
                assert_eq!(vars[0].label.as_deref(), Some("API Key"));
                assert!(vars[0].secret);
                assert!(!vars[0].optional);
                // Second var: not a secret, not optional
                assert_eq!(vars[1].name, "AZURE_OPENAI_ENDPOINT");
                assert!(!vars[1].secret);
                assert!(!vars[1].optional);
                // Third var: not a secret, optional
                assert_eq!(vars[2].name, "AZURE_OPENAI_API_VERSION");
                assert!(!vars[2].secret);
                assert!(vars[2].optional);
            }
            _ => panic!("Expected EnvVar variant"),
        }
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_method_terminal_serialization() {
        let method = AuthMethod::Terminal(AuthMethodTerminal::new("tui-auth", "Terminal Auth"));

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "tui-auth",
                "name": "Terminal Auth",
                "type": "terminal"
            })
        );
        // args and env should be omitted when empty
        assert!(!json.as_object().unwrap().contains_key("args"));
        assert!(!json.as_object().unwrap().contains_key("env"));

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::Terminal(AuthMethodTerminal { args, env, .. }) => {
                assert!(args.is_empty());
                assert!(env.is_empty());
            }
            _ => panic!("Expected Terminal variant"),
        }
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_method_terminal_with_args_and_env_serialization() {
        use std::collections::HashMap;

        let mut env = HashMap::new();
        env.insert("TERM".to_string(), "xterm-256color".to_string());

        let method = AuthMethod::Terminal(
            AuthMethodTerminal::new("tui-auth", "Terminal Auth")
                .args(vec!["--interactive".to_string(), "--color".to_string()])
                .env(env),
        );

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "tui-auth",
                "name": "Terminal Auth",
                "type": "terminal",
                "args": ["--interactive", "--color"],
                "env": {
                    "TERM": "xterm-256color"
                }
            })
        );

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::Terminal(AuthMethodTerminal { args, env, .. }) => {
                assert_eq!(args, vec!["--interactive", "--color"]);
                assert_eq!(env.len(), 1);
                assert_eq!(env.get("TERM").unwrap(), "xterm-256color");
            }
            _ => panic!("Expected Terminal variant"),
        }
    }

    #[test]
    fn test_session_config_option_value_id_serialize() {
        let val = SessionConfigOptionValue::value_id("model-1");
        let json = serde_json::to_value(&val).unwrap();
        // ValueId omits the "type" field (it's the default)
        assert_eq!(json, json!({ "value": "model-1" }));
        assert!(!json.as_object().unwrap().contains_key("type"));
    }

    #[test]
    fn test_session_config_option_value_boolean_serialize() {
        let val = SessionConfigOptionValue::boolean(true);
        let json = serde_json::to_value(&val).unwrap();
        assert_eq!(json, json!({ "type": "boolean", "value": true }));
    }

    #[test]
    fn test_session_config_option_value_deserialize_no_type() {
        // Missing "type" should default to ValueId
        let json = json!({ "value": "model-1" });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(val, SessionConfigOptionValue::value_id("model-1"));
        assert_eq!(val.as_value_id().unwrap().to_string(), "model-1");
    }

    #[test]
    fn test_session_config_option_value_deserialize_boolean() {
        let json = json!({ "type": "boolean", "value": true });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(val, SessionConfigOptionValue::boolean(true));
        assert_eq!(val.as_bool(), Some(true));
    }

    #[test]
    fn test_session_config_option_value_deserialize_boolean_false() {
        let json = json!({ "type": "boolean", "value": false });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(val, SessionConfigOptionValue::boolean(false));
        assert_eq!(val.as_bool(), Some(false));
    }

    #[test]
    fn test_session_config_option_value_deserialize_unknown_type_with_string_value() {
        // Unknown type with a string value gracefully falls back to ValueId
        let json = json!({ "type": "text", "value": "freeform input" });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(val.as_value_id().unwrap().to_string(), "freeform input");
    }

    #[test]
    fn test_session_config_option_value_roundtrip_value_id() {
        let original = SessionConfigOptionValue::value_id("option-a");
        let json = serde_json::to_value(&original).unwrap();
        let roundtripped: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_session_config_option_value_roundtrip_boolean() {
        let original = SessionConfigOptionValue::boolean(false);
        let json = serde_json::to_value(&original).unwrap();
        let roundtripped: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_session_config_option_value_type_mismatch_boolean_with_string() {
        // type says "boolean" but value is a string — falls to untagged ValueId
        let json = json!({ "type": "boolean", "value": "not a bool" });
        let result = serde_json::from_value::<SessionConfigOptionValue>(json);
        // serde tries Boolean first (fails), then falls to untagged ValueId (succeeds)
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().as_value_id().unwrap().to_string(),
            "not a bool"
        );
    }

    #[test]
    fn test_session_config_option_value_from_impls() {
        let from_str: SessionConfigOptionValue = "model-1".into();
        assert_eq!(from_str.as_value_id().unwrap().to_string(), "model-1");

        let from_id: SessionConfigOptionValue = SessionConfigValueId::new("model-2").into();
        assert_eq!(from_id.as_value_id().unwrap().to_string(), "model-2");

        let from_bool: SessionConfigOptionValue = true.into();
        assert_eq!(from_bool.as_bool(), Some(true));
    }

    #[test]
    fn test_set_session_config_option_request_value_id() {
        let req = SetSessionConfigOptionRequest::new("sess_1", "model", "model-1");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "sess_1",
                "configId": "model",
                "value": "model-1"
            })
        );
        // No "type" field for value_id
        assert!(!json.as_object().unwrap().contains_key("type"));
    }

    #[test]
    fn test_set_session_config_option_request_boolean() {
        let req = SetSessionConfigOptionRequest::new("sess_1", "brave_mode", true);
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "sess_1",
                "configId": "brave_mode",
                "type": "boolean",
                "value": true
            })
        );
    }

    #[test]
    fn test_set_session_config_option_request_deserialize_no_type() {
        // Backwards-compatible: no "type" field → value_id
        let json = json!({
            "sessionId": "sess_1",
            "configId": "model",
            "value": "model-1"
        });
        let req: SetSessionConfigOptionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.session_id.to_string(), "sess_1");
        assert_eq!(req.config_id.to_string(), "model");
        assert_eq!(req.value.as_value_id().unwrap().to_string(), "model-1");
    }

    #[test]
    fn test_set_session_config_option_request_deserialize_boolean() {
        let json = json!({
            "sessionId": "sess_1",
            "configId": "brave_mode",
            "type": "boolean",
            "value": true
        });
        let req: SetSessionConfigOptionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.value.as_bool(), Some(true));
    }

    #[test]
    fn test_set_session_config_option_request_roundtrip_value_id() {
        let original = SetSessionConfigOptionRequest::new("s", "c", "v");
        let json = serde_json::to_value(&original).unwrap();
        let roundtripped: SetSessionConfigOptionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_set_session_config_option_request_roundtrip_boolean() {
        let original = SetSessionConfigOptionRequest::new("s", "c", false);
        let json = serde_json::to_value(&original).unwrap();
        let roundtripped: SetSessionConfigOptionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_session_config_boolean_serialization() {
        let cfg = SessionConfigBoolean::new(true);
        let json = serde_json::to_value(&cfg).unwrap();
        assert_eq!(json, json!({ "currentValue": true }));

        let deserialized: SessionConfigBoolean = serde_json::from_value(json).unwrap();
        assert!(deserialized.current_value);
    }

    #[test]
    fn test_session_config_option_boolean_variant() {
        let opt = SessionConfigOption::boolean("brave_mode", "Brave Mode", false)
            .description("Skip confirmation prompts")
            .meta(test_meta());
        assert_eq!(serialized_meta_key_count(&opt), 1);

        let json = serde_json::to_value(&opt).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "brave_mode",
                "name": "Brave Mode",
                "description": "Skip confirmation prompts",
                "type": "boolean",
                "currentValue": false,
                "_meta": {
                    "source": "test"
                }
            })
        );

        let deserialized: SessionConfigOption = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.id.to_string(), "brave_mode");
        assert_eq!(deserialized.name, "Brave Mode");
        match deserialized.kind {
            SessionConfigKind::Boolean(ref b) => assert!(!b.current_value),
            _ => panic!("Expected Boolean kind"),
        }
    }

    #[test]
    fn test_session_config_option_select_still_works() {
        // Make sure existing select options are unaffected
        let opt = SessionConfigOption::select(
            "model",
            "Model",
            "model-1",
            vec![
                SessionConfigSelectOption::new("model-1", "Model 1"),
                SessionConfigSelectOption::new("model-2", "Model 2"),
            ],
        )
        .meta(test_meta());
        assert_eq!(serialized_meta_key_count(&opt), 1);

        let json = serde_json::to_value(&opt).unwrap();
        assert_eq!(json["type"], "select");
        assert_eq!(json["currentValue"], "model-1");
        assert_eq!(json["options"].as_array().unwrap().len(), 2);
        assert_eq!(json["_meta"]["source"], "test");

        let deserialized: SessionConfigOption = serde_json::from_value(json).unwrap();
        match deserialized.kind {
            SessionConfigKind::Select(ref s) => {
                assert_eq!(s.current_value.to_string(), "model-1");
            }
            _ => panic!("Expected Select kind"),
        }
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_llm_protocol_known_variants() {
        assert_eq!(
            serde_json::to_value(&LlmProtocol::Anthropic).unwrap(),
            json!("anthropic")
        );
        assert_eq!(
            serde_json::to_value(&LlmProtocol::OpenAi).unwrap(),
            json!("openai")
        );
        assert_eq!(
            serde_json::to_value(&LlmProtocol::Azure).unwrap(),
            json!("azure")
        );
        assert_eq!(
            serde_json::to_value(&LlmProtocol::Vertex).unwrap(),
            json!("vertex")
        );
        assert_eq!(
            serde_json::to_value(&LlmProtocol::Bedrock).unwrap(),
            json!("bedrock")
        );

        assert_eq!(
            serde_json::from_str::<LlmProtocol>("\"anthropic\"").unwrap(),
            LlmProtocol::Anthropic
        );
        assert_eq!(
            serde_json::from_str::<LlmProtocol>("\"openai\"").unwrap(),
            LlmProtocol::OpenAi
        );
        assert_eq!(
            serde_json::from_str::<LlmProtocol>("\"azure\"").unwrap(),
            LlmProtocol::Azure
        );
        assert_eq!(
            serde_json::from_str::<LlmProtocol>("\"vertex\"").unwrap(),
            LlmProtocol::Vertex
        );
        assert_eq!(
            serde_json::from_str::<LlmProtocol>("\"bedrock\"").unwrap(),
            LlmProtocol::Bedrock
        );
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_llm_protocol_unknown_variant() {
        let unknown: LlmProtocol = serde_json::from_str("\"cohere\"").unwrap();
        assert_eq!(unknown, LlmProtocol::Other("cohere".to_string()));

        let json = serde_json::to_value(&unknown).unwrap();
        assert_eq!(json, json!("cohere"));
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_provider_current_config_serialization() {
        let config =
            ProviderCurrentConfig::new(LlmProtocol::Anthropic, "https://api.anthropic.com");

        let json = serde_json::to_value(&config).unwrap();
        assert_eq!(
            json,
            json!({
                "apiType": "anthropic",
                "baseUrl": "https://api.anthropic.com"
            })
        );

        let deserialized: ProviderCurrentConfig = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.api_type, LlmProtocol::Anthropic);
        assert_eq!(deserialized.base_url, "https://api.anthropic.com");
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_provider_info_with_current_config() {
        let info = ProviderInfo::new(
            "main",
            vec![LlmProtocol::Anthropic, LlmProtocol::OpenAi],
            true,
            Some(ProviderCurrentConfig::new(
                LlmProtocol::Anthropic,
                "https://api.anthropic.com",
            )),
        );

        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(
            json,
            json!({
                "providerId": "main",
                "supported": ["anthropic", "openai"],
                "required": true,
                "current": {
                    "apiType": "anthropic",
                    "baseUrl": "https://api.anthropic.com"
                }
            })
        );

        let deserialized: ProviderInfo = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.provider_id.to_string(), "main");
        assert_eq!(deserialized.supported.len(), 2);
        assert!(deserialized.required);
        assert!(deserialized.current.is_some());
        assert_eq!(
            deserialized.current.as_ref().unwrap().api_type,
            LlmProtocol::Anthropic
        );
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_provider_info_disabled() {
        let info = ProviderInfo::new(
            "secondary",
            vec![LlmProtocol::OpenAi],
            false,
            None::<ProviderCurrentConfig>,
        );

        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(
            json,
            json!({
                "providerId": "secondary",
                "supported": ["openai"],
                "required": false
            })
        );

        let deserialized: ProviderInfo = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.provider_id.to_string(), "secondary");
        assert!(!deserialized.required);
        assert!(deserialized.current.is_none());
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_provider_info_missing_current_defaults_to_none() {
        // current is optional; omitting it should decode as None
        let json = json!({
            "providerId": "main",
            "supported": ["anthropic"],
            "required": true
        });
        let deserialized: ProviderInfo = serde_json::from_value(json).unwrap();
        assert!(deserialized.current.is_none());
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_provider_info_explicit_null_current_decodes_to_none() {
        // current: null and an omitted current are equivalent on the wire;
        // both must deserialize into None so the disabled state is preserved
        // regardless of which form the peer chose to send.
        let json = json!({
            "providerId": "main",
            "supported": ["anthropic"],
            "required": true,
            "current": null
        });
        let deserialized: ProviderInfo = serde_json::from_value(json).unwrap();
        assert!(deserialized.current.is_none());
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_list_providers_response_serialization() {
        let response = ListProvidersResponse::new(vec![ProviderInfo::new(
            "main",
            vec![LlmProtocol::Anthropic],
            true,
            Some(ProviderCurrentConfig::new(
                LlmProtocol::Anthropic,
                "https://api.anthropic.com",
            )),
        )]);

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["providers"].as_array().unwrap().len(), 1);
        assert_eq!(json["providers"][0]["providerId"], "main");

        let deserialized: ListProvidersResponse = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.providers.len(), 1);
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_set_provider_request_serialization() {
        use std::collections::HashMap;

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer sk-test".to_string());

        let request =
            SetProviderRequest::new("main", LlmProtocol::OpenAi, "https://api.openai.com/v1")
                .headers(headers);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(
            json,
            json!({
                "providerId": "main",
                "apiType": "openai",
                "baseUrl": "https://api.openai.com/v1",
                "headers": {
                    "Authorization": "Bearer sk-test"
                }
            })
        );

        let deserialized: SetProviderRequest = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.provider_id.to_string(), "main");
        assert_eq!(deserialized.api_type, LlmProtocol::OpenAi);
        assert_eq!(deserialized.base_url, "https://api.openai.com/v1");
        assert_eq!(deserialized.headers.len(), 1);
        assert_eq!(
            deserialized.headers.get("Authorization").unwrap(),
            "Bearer sk-test"
        );
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_set_provider_request_omits_empty_headers() {
        let request =
            SetProviderRequest::new("main", LlmProtocol::Anthropic, "https://api.anthropic.com");

        let json = serde_json::to_value(&request).unwrap();
        // headers should be omitted when empty
        assert!(!json.as_object().unwrap().contains_key("headers"));
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_disable_provider_request_serialization() {
        let request = DisableProviderRequest::new("secondary");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json, json!({ "providerId": "secondary" }));

        let deserialized: DisableProviderRequest = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.provider_id.to_string(), "secondary");
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_providers_capabilities_serialization() {
        let caps = ProvidersCapabilities::new();

        let json = serde_json::to_value(&caps).unwrap();
        assert_eq!(json, json!({}));

        let deserialized: ProvidersCapabilities = serde_json::from_value(json).unwrap();
        assert!(deserialized.meta.is_none());
    }

    #[cfg(feature = "unstable_llm_providers")]
    #[test]
    fn test_agent_capabilities_with_providers() {
        let caps = AgentCapabilities::new().providers(ProvidersCapabilities::new());

        let json = serde_json::to_value(&caps).unwrap();
        assert_eq!(json["providers"], json!({}));

        let deserialized: AgentCapabilities = serde_json::from_value(json).unwrap();
        assert!(deserialized.providers.is_some());
    }

    #[test]
    fn prompt_request_rejects_malformed_content_block() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<PromptRequest>(json!({
                "sessionId": "sess-1",
                "prompt": [{"type": "text"}]
            }))
            .is_err()
        );
    }

    #[test]
    fn prompt_request_rejects_non_array_prompt() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<PromptRequest>(json!({
                "sessionId": "sess-1",
                "prompt": "hello"
            }))
            .is_err()
        );
    }
}
