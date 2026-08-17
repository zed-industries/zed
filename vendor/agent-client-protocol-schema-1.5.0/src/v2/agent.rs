//! Methods and notifications the agent handles/receives.
//!
//! This module defines the Agent trait and all associated types for implementing
//! an AI coding agent that follows the Agent Client Protocol (ACP).

use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

#[cfg(feature = "unstable_llm_providers")]
use std::collections::HashMap;

use derive_more::{Display, From};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use super::{
    AbsolutePath, ClientCapabilities, ContentBlock, ExtNotification, ExtRequest, ExtResponse, Meta,
    SessionId,
};
#[cfg(feature = "unstable_auth_methods")]
use crate::DefaultTrueOnError;
use crate::{IntoOption, ProtocolVersion, SkipListener};

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
    /// Information about the implementation sending this initialize request.
    pub info: Implementation,
    /// Capabilities supported by the client.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub capabilities: ClientCapabilities,
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
    pub fn new(protocol_version: ProtocolVersion, info: Implementation) -> Self {
        Self {
            protocol_version,
            capabilities: ClientCapabilities::default(),
            info,
            meta: None,
        }
    }

    /// Capabilities supported by the client.
    #[must_use]
    pub fn capabilities(mut self, capabilities: ClientCapabilities) -> Self {
        self.capabilities = capabilities;
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
    /// Information about the implementation sending this initialize response.
    pub info: Implementation,
    /// Capabilities supported by the agent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub capabilities: AgentCapabilities,
    /// Authentication methods supported by the agent.
    ///
    /// Optional. Omitted or empty means the agent does not advertise the
    /// authentication method surface. Supplying one or more valid methods means
    /// the agent MUST support both `auth/login` and `auth/logout`.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_methods: Vec<AuthMethod>,
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
    pub fn new(protocol_version: ProtocolVersion, info: Implementation) -> Self {
        Self {
            protocol_version,
            capabilities: AgentCapabilities::default(),
            auth_methods: vec![],
            info,
            meta: None,
        }
    }

    /// Capabilities supported by the agent.
    #[must_use]
    pub fn capabilities(mut self, capabilities: AgentCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Authentication methods supported by the agent.
    ///
    /// Supplying one or more valid methods means the agent MUST support both
    /// `auth/login` and `auth/logout`.
    #[must_use]
    pub fn auth_methods(mut self, auth_methods: Vec<AuthMethod>) -> Self {
        self.auth_methods = auth_methods;
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

/// Request parameters for the `auth/login` method.
///
/// Specifies which authentication method to use.
///
/// Agents MUST support this method when their `initialize` response advertised
/// at least one valid authentication method. Clients MUST NOT call this method
/// when `authMethods` was omitted or empty.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = AUTH_LOGIN_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoginAuthRequest {
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

impl LoginAuthRequest {
    /// Builds [`LoginAuthRequest`] with the required request fields set; optional fields start unset or empty.
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

/// Response to the `auth/login` method.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = AUTH_LOGIN_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoginAuthResponse {
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

impl LoginAuthResponse {
    /// Builds [`LoginAuthResponse`] with the required response fields set; optional fields start unset or empty.
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

/// Request parameters for the `auth/logout` method.
///
/// Terminates the current authenticated session.
///
/// Agents MUST support this method when their `initialize` response advertised
/// at least one valid authentication method. Clients MUST NOT call this method
/// when `authMethods` was omitted or empty.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = AUTH_LOGOUT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LogoutAuthRequest {
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

impl LogoutAuthRequest {
    /// Builds [`LogoutAuthRequest`] with the required request fields set; optional fields start unset or empty.
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

/// Response to the `auth/logout` method.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = AUTH_LOGOUT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LogoutAuthResponse {
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

impl LogoutAuthResponse {
    /// Builds [`LogoutAuthResponse`] with the required response fields set; optional fields start unset or empty.
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

/// Authentication-related extension capabilities supported by the agent.
///
/// This object does not advertise support for `auth/login` or `auth/logout`.
/// Those methods are advertised by a non-empty `authMethods` list in the
/// `initialize` response.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AgentAuthCapabilities {
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
    /// Builds an empty [`AgentAuthCapabilities`].
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
#[from(forward)]
#[non_exhaustive]
pub struct AuthMethodId(pub Arc<str>);

impl AuthMethodId {
    /// Wraps a protocol string as a typed [`AuthMethodId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// Describes an available authentication method.
///
/// The `type` field acts as the discriminator in the serialized JSON form.
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
    /// The `type` discriminator value is `agent`.
    Agent(AuthMethodAgent),
    /// Custom or future authentication method.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Clients that do not understand this method type should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding initialization
    /// data, and otherwise ignore the method or display it generically.
    #[serde(untagged)]
    Other(OtherAuthMethod),
}

impl AuthMethod {
    /// The unique identifier for this authentication method.
    #[must_use]
    pub fn method_id(&self) -> &AuthMethodId {
        match self {
            Self::Agent(a) => &a.method_id,
            Self::Other(a) => &a.method_id,
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(e) => &e.method_id,
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(t) => &t.method_id,
        }
    }

    /// The human-readable name of this authentication method.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Agent(a) => &a.name,
            Self::Other(a) => &a.name,
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
            Self::Other(a) => a.description.as_deref(),
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
            Self::Other(a) => a.meta.as_ref(),
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(e) => e.meta.as_ref(),
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(t) => t.meta.as_ref(),
        }
    }
}

/// Custom or future authentication method payload.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_auth_method_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherAuthMethod {
    /// Custom or future authentication method type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Unique identifier for this authentication method.
    pub method_id: AuthMethodId,
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
    /// Additional fields from the unknown authentication method payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherAuthMethod {
    /// Builds [`OtherAuthMethod`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        type_: impl Into<String>,
        method_id: impl Into<AuthMethodId>,
        name: impl Into<String>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("type");
        fields.remove("methodId");
        fields.remove("name");
        fields.remove("description");
        fields.remove("_meta");
        Self {
            type_: type_.into(),
            method_id: method_id.into(),
            name: name.into(),
            description: None,
            meta: None,
            fields,
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

impl<'de> Deserialize<'de> for OtherAuthMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawOtherAuthMethod {
            #[serde(rename = "type")]
            type_: String,
            method_id: AuthMethodId,
            name: String,
            description: Option<String>,
            #[serde(rename = "_meta")]
            meta: Option<Meta>,
            #[serde(flatten)]
            fields: BTreeMap<String, serde_json::Value>,
        }

        let raw = RawOtherAuthMethod::deserialize(deserializer)?;
        if is_known_auth_method_type(&raw.type_) {
            return Err(serde::de::Error::custom(format!(
                "known authentication method `{}` did not match its schema",
                raw.type_
            )));
        }

        Ok(Self {
            type_: raw.type_,
            method_id: raw.method_id,
            name: raw.name,
            description: raw.description,
            meta: raw.meta,
            fields: raw.fields,
        })
    }
}

fn is_known_auth_method_type(type_: &str) -> bool {
    match type_ {
        "agent" => true,
        #[cfg(feature = "unstable_auth_methods")]
        "env_var" | "terminal" => true,
        _ => false,
    }
}

fn other_auth_method_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        &[
            "agent",
            #[cfg(feature = "unstable_auth_methods")]
            "env_var",
            #[cfg(feature = "unstable_auth_methods")]
            "terminal",
        ],
    );
}

/// Agent handles authentication itself.
///
/// The `type` discriminator value is `agent`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthMethodAgent {
    /// Unique identifier for this authentication method.
    pub method_id: AuthMethodId,
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
    pub fn new(method_id: impl Into<AuthMethodId>, name: impl Into<String>) -> Self {
        Self {
            method_id: method_id.into(),
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
    pub method_id: AuthMethodId,
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
    #[schemars(url)]
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
        method_id: impl Into<AuthMethodId>,
        name: impl Into<String>,
        vars: Vec<AuthEnvVar>,
    ) -> Self {
        Self {
            method_id: method_id.into(),
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
    pub method_id: AuthMethodId,
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
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

#[cfg(feature = "unstable_auth_methods")]
impl AuthMethodTerminal {
    /// Builds [`AuthMethodTerminal`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(method_id: impl Into<AuthMethodId>, name: impl Into<String>) -> Self {
        Self {
            method_id: method_id.into(),
            name: name.into(),
            description: None,
            args: Vec::new(),
            env: Vec::new(),
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
    pub fn env(mut self, env: Vec<EnvVariable>) -> Self {
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
    pub cwd: AbsolutePath,
    /// Additional workspace roots for this session. Each path must be absolute.
    ///
    /// These expand the session's workspace scope without changing `cwd`, which
    /// remains the base for relative paths. When omitted or empty, no
    /// additional roots are activated for the new session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<AbsolutePath>,
    /// List of MCP (Model Context Protocol) servers the agent should connect to.
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

impl NewSessionRequest {
    /// Builds [`NewSessionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(cwd: impl Into<AbsolutePath>) -> Self {
        Self {
            cwd: cwd.into(),
            additional_directories: vec![],
            mcp_servers: vec![],
            meta: None,
        }
    }

    /// Additional workspace roots for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories<I, P>(mut self, additional_directories: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<AbsolutePath>,
    {
        self.additional_directories = additional_directories.into_iter().map(Into::into).collect();
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
    /// Initial session configuration options.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

impl NewSessionResponse {
    /// Builds [`NewSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
            config_options: Vec::new(),
            meta: None,
        }
    }

    /// Initial session configuration options.
    #[must_use]
    pub fn config_options(mut self, config_options: Vec<SessionConfigOption>) -> Self {
        self.config_options = config_options;
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
    pub cwd: AbsolutePath,
    /// Additional workspace roots to activate for this session. Each path must be absolute.
    ///
    /// When omitted or empty, no additional roots are activated. When non-empty,
    /// this is the complete resulting additional-root list for the forked
    /// session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<AbsolutePath>,
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
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<AbsolutePath>) -> Self {
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
    pub fn additional_directories<I, P>(mut self, additional_directories: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<AbsolutePath>,
    {
        self.additional_directories = additional_directories.into_iter().map(Into::into).collect();
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
    /// Initial session configuration options.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

#[cfg(feature = "unstable_session_fork")]
impl ForkSessionResponse {
    /// Builds [`ForkSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
            config_options: Vec::new(),
            meta: None,
        }
    }

    /// Initial session configuration options.
    #[must_use]
    pub fn config_options(mut self, config_options: Vec<SessionConfigOption>) -> Self {
        self.config_options = config_options;
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
/// Resumes an existing session and optionally replays prior conversation
/// history according to `replayFrom`.
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
    pub cwd: AbsolutePath,
    /// Additional workspace roots to activate for this session. Each path must be absolute.
    ///
    /// When omitted or empty, no additional roots are activated. When non-empty,
    /// this is the complete resulting additional-root list for the resumed
    /// session. It may differ from any previously used or reported list as long as
    /// the request `cwd` matches the session's `cwd`.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<AbsolutePath>,
    /// List of MCP servers to connect to for this session.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp_servers: Vec<McpServer>,
    /// Inclusive cursor describing where conversation replay should begin.
    ///
    /// Optional. Omitted or `null` both mean the Agent should resume without
    /// replaying previous conversation history. Replay cursors are inclusive:
    /// replay includes the position identified by the cursor. Supplying
    /// `{ "type": "start" }` means the Agent should replay the whole
    /// conversation before responding.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub replay_from: Option<ReplayFrom>,
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
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<AbsolutePath>) -> Self {
        Self {
            session_id: session_id.into(),
            cwd: cwd.into(),
            additional_directories: vec![],
            mcp_servers: vec![],
            replay_from: None,
            meta: None,
        }
    }

    /// Additional workspace roots to activate for this session. Each path must be absolute.
    #[must_use]
    pub fn additional_directories<I, P>(mut self, additional_directories: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<AbsolutePath>,
    {
        self.additional_directories = additional_directories.into_iter().map(Into::into).collect();
        self
    }

    /// List of MCP servers to connect to for this session.
    #[must_use]
    pub fn mcp_servers(mut self, mcp_servers: Vec<McpServer>) -> Self {
        self.mcp_servers = mcp_servers;
        self
    }

    /// Inclusive cursor describing where conversation replay should begin.
    ///
    /// Omitted or `null` both mean the Agent should resume without replaying
    /// previous conversation history. Replay cursors are inclusive: replay
    /// includes the position identified by the cursor. Supplying
    /// `{ "type": "start" }` means the Agent should replay the whole
    /// conversation before responding.
    #[must_use]
    pub fn replay_from(mut self, replay_from: impl IntoOption<ReplayFrom>) -> Self {
        self.replay_from = replay_from.into_option();
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

/// Inclusive cursor describing where replayed session history should begin.
///
/// Replay includes the position identified by the cursor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReplayFrom {
    /// Replay the whole conversation from its first replayable entry.
    Start(ReplayFromStart),
    /// Custom or future replay cursor.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this cursor should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding requests, and
    /// otherwise reject the request rather than guessing where to replay from.
    #[serde(untagged)]
    Other(OtherReplayFrom),
}

impl From<ReplayFromStart> for ReplayFrom {
    fn from(replay_from: ReplayFromStart) -> Self {
        Self::Start(replay_from)
    }
}

/// Inclusive replay cursor requesting replay from the start of the conversation.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReplayFromStart {
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

impl ReplayFromStart {
    /// Builds [`ReplayFromStart`].
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

/// Custom or future replay cursor payload.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_replay_from_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherReplayFrom {
    /// Custom or future replay cursor type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
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
    /// Additional fields from the unknown replay cursor payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherReplayFrom {
    /// Builds [`OtherReplayFrom`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        fields.remove("_meta");
        Self {
            type_: type_.into(),
            meta: None,
            fields,
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

impl<'de> Deserialize<'de> for OtherReplayFrom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        if is_known_replay_from_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known replay cursor `{type_}` did not match its schema"
            )));
        }

        let meta = fields
            .remove("_meta")
            .and_then(|value| serde_json::from_value(value).ok());

        Ok(Self {
            type_,
            meta,
            fields,
        })
    }
}

fn is_known_replay_from_type(type_: &str) -> bool {
    matches!(type_, "start")
}

fn other_replay_from_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(schema, "type", &["start"]);
}

/// Response from resuming an existing session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_RESUME_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResumeSessionResponse {
    /// Initial session configuration options.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

impl ResumeSessionResponse {
    /// Builds [`ResumeSessionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Initial session configuration options.
    #[must_use]
    pub fn config_options(mut self, config_options: Vec<SessionConfigOption>) -> Self {
        self.config_options = config_options;
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
/// The agent **must** cancel any ongoing work related to the session (treat it
/// as if `session/cancel` was called) and then free up any resources associated
/// with the session.
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

/// An opaque cursor used to paginate `session/list` results.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &str, &mut str, Box<str>, Cow<'_, str>)]
#[non_exhaustive]
pub struct SessionListCursor(pub Arc<str>);

impl SessionListCursor {
    /// Wraps a protocol string as a typed [`SessionListCursor`].
    #[must_use]
    pub fn new(cursor: impl Into<Self>) -> Self {
        cursor.into()
    }
}

impl AsRef<str> for SessionListCursor {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&String> for SessionListCursor {
    fn from(cursor: &String) -> Self {
        Self(cursor.as_str().into())
    }
}

macro_rules! impl_session_list_cursor_option_conversion {
    ($source:ty) => {
        impl IntoOption<SessionListCursor> for $source {
            fn into_option(self) -> Option<SessionListCursor> {
                Some(self.into())
            }
        }
    };
}

impl_session_list_cursor_option_conversion!(Arc<str>);
impl_session_list_cursor_option_conversion!(String);
impl_session_list_cursor_option_conversion!(&str);
impl_session_list_cursor_option_conversion!(&mut str);
impl_session_list_cursor_option_conversion!(&String);
impl_session_list_cursor_option_conversion!(Box<str>);
impl_session_list_cursor_option_conversion!(Cow<'_, str>);

/// Request parameters for listing existing sessions.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_LIST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSessionsRequest {
    /// Filter sessions by working directory. Must be an absolute path.
    #[serde(default)]
    pub cwd: Option<AbsolutePath>,
    /// Opaque cursor token from a previous response's nextCursor field for cursor-based pagination
    #[serde(default)]
    pub cursor: Option<SessionListCursor>,
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
    pub fn cwd(mut self, cwd: impl IntoOption<AbsolutePath>) -> Self {
        self.cwd = cwd.into_option();
        self
    }

    /// Opaque cursor token from a previous response's nextCursor field for cursor-based pagination
    #[must_use]
    pub fn cursor(mut self, cursor: impl IntoOption<SessionListCursor>) -> Self {
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
    /// Array of session information objects.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub sessions: Vec<SessionInfo>,
    /// Opaque cursor token. If present, pass this in the next request's cursor parameter
    /// to fetch the next page. If absent, there are no more results.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub next_cursor: Option<SessionListCursor>,
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
    pub fn next_cursor(mut self, next_cursor: impl IntoOption<SessionListCursor>) -> Self {
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
/// Only available if the Agent supports the `session.delete` capability.
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
    pub cwd: AbsolutePath,
    /// Additional workspace roots reported for this session. Each path must be absolute.
    ///
    /// When present, this is the complete ordered additional-root list reported
    /// by the Agent. Omitted and empty values are equivalent: the response
    /// reports no additional roots.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_directories: Vec<AbsolutePath>,

    /// Human-readable title for the session
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// RFC 3339 timestamp of last activity.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "format" = "date-time"))]
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
    pub fn new(session_id: impl Into<SessionId>, cwd: impl Into<AbsolutePath>) -> Self {
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
    pub fn additional_directories<I, P>(mut self, additional_directories: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<AbsolutePath>,
    {
        self.additional_directories = additional_directories.into_iter().map(Into::into).collect();
        self
    }

    /// Human-readable title for the session
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// RFC 3339 timestamp of last activity.
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

// Session config options

/// Unique identifier for a session configuration option.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct SessionConfigId(pub Arc<str>);

impl SessionConfigId {
    /// Wraps a protocol string as a typed [`SessionConfigId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// Unique identifier for a session configuration option value.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct SessionConfigValueId(pub Arc<str>);

impl SessionConfigValueId {
    /// Wraps a protocol string as a typed [`SessionConfigValueId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// Unique identifier for a session configuration option value group.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From, Display)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct SessionConfigGroupId(pub Arc<str>);

impl SessionConfigGroupId {
    /// Wraps a protocol string as a typed [`SessionConfigGroupId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
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
    pub group_id: SessionConfigGroupId,
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
        group_id: impl Into<SessionConfigGroupId>,
        name: impl Into<String>,
        options: Vec<SessionConfigSelectOption>,
    ) -> Self {
        Self {
            group_id: group_id.into(),
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
    /// Custom or future category.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// Type-specific session configuration option payload.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionConfigKind {
    /// Single-value selector (dropdown).
    Select(SessionConfigSelect),
    /// Boolean on/off toggle.
    Boolean(SessionConfigBoolean),
    /// Custom or future session configuration option payload.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Clients that do not understand this option type should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding configuration
    /// data, and otherwise ignore the option or display it generically.
    #[serde(untagged)]
    Other(OtherSessionConfigKind),
}

/// Custom or future session configuration option payload.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_session_config_kind_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherSessionConfigKind {
    /// Custom or future session configuration option type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown session configuration option payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherSessionConfigKind {
    /// Builds [`OtherSessionConfigKind`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        fields.remove("_meta");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherSessionConfigKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        if is_known_session_config_kind_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known session configuration option `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

fn is_known_session_config_kind_type(type_: &str) -> bool {
    matches!(type_, "select" | "boolean")
}

fn other_session_config_kind_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(schema, "type", &["select", "boolean"]);
}

/// A session configuration option selector and its current state.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigOption {
    /// Unique identifier for the configuration option.
    pub config_id: SessionConfigId,
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
        config_id: impl Into<SessionConfigId>,
        name: impl Into<String>,
        kind: SessionConfigKind,
    ) -> Self {
        Self {
            config_id: config_id.into(),
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
        config_id: impl Into<SessionConfigId>,
        name: impl Into<String>,
        current_value: impl Into<SessionConfigValueId>,
        options: impl Into<SessionConfigSelectOptions>,
    ) -> Self {
        Self::new(
            config_id,
            name,
            SessionConfigKind::Select(SessionConfigSelect::new(current_value, options)),
        )
    }

    /// Builds a boolean-style session configuration option with its current value.
    #[must_use]
    pub fn boolean(
        config_id: impl Into<SessionConfigId>,
        name: impl Into<String>,
        current_value: bool,
    ) -> Self {
        Self::new(
            config_id,
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
///
/// The `type` discriminator describes the *shape* of the value, not the option
/// kind. For example every option kind that picks from a list of ids
/// (`select`, `radio`, …) would use [`Id`](Self::Id), while a future freeform
/// text option would get its own variant.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionConfigOptionValue {
    /// A [`SessionConfigValueId`] string value (`type: "id"`).
    Id {
        /// The value ID.
        value: SessionConfigValueId,
    },
    /// A boolean value (`type: "boolean"`).
    Boolean {
        /// The boolean value.
        value: bool,
    },
    /// Custom or future session configuration option value payload.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(OtherSessionConfigOptionValue),
}

/// Custom or future session configuration option value payload.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_session_config_option_value_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherSessionConfigOptionValue {
    /// Custom or future session configuration option value type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Raw value payload for the custom or future value type.
    pub value: serde_json::Value,
    /// Additional fields from the unknown session configuration option value payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherSessionConfigOptionValue {
    /// Builds [`OtherSessionConfigOptionValue`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        type_: impl Into<String>,
        value: serde_json::Value,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("type");
        fields.remove("value");
        fields.remove("_meta");
        Self {
            type_: type_.into(),
            value,
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherSessionConfigOptionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        if is_known_session_config_option_value_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known session configuration option value `{type_}` did not match its schema"
            )));
        }

        let value = fields
            .remove("value")
            .ok_or_else(|| serde::de::Error::missing_field("value"))?;

        Ok(Self {
            type_,
            value,
            fields,
        })
    }
}

impl<'de> Deserialize<'de> for SessionConfigOptionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields.remove("type");
        let value = fields
            .remove("value")
            .ok_or_else(|| serde::de::Error::missing_field("value"))?;

        let type_ = type_.ok_or_else(|| serde::de::Error::missing_field("type"))?;

        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        match type_.as_str() {
            "id" => {
                let value = serde_json::from_value(value).map_err(|error| {
                    serde::de::Error::custom(format!(
                        "`value` must be a string for `type: id`: {error}"
                    ))
                })?;
                Ok(Self::Id { value })
            }
            "boolean" => {
                let value = serde_json::from_value(value).map_err(|error| {
                    serde::de::Error::custom(format!(
                        "`value` must be a boolean for `type: boolean`: {error}"
                    ))
                })?;
                Ok(Self::Boolean { value })
            }
            _ => Ok(Self::Other(OtherSessionConfigOptionValue {
                type_,
                value,
                fields,
            })),
        }
    }
}

fn is_known_session_config_option_value_type(type_: &str) -> bool {
    matches!(type_, "id" | "boolean")
}

fn other_session_config_option_value_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(schema, "type", &["id", "boolean"]);
}

impl SessionConfigOptionValue {
    /// Create an id option value (used by `select` and other id-based option types).
    #[must_use]
    pub fn id(id: impl Into<SessionConfigValueId>) -> Self {
        Self::Id { value: id.into() }
    }

    /// Create a boolean option value.
    #[must_use]
    pub fn boolean(val: bool) -> Self {
        Self::Boolean { value: val }
    }

    /// Return the inner [`SessionConfigValueId`] if this is a
    /// [`Id`](Self::Id) value.
    #[must_use]
    pub fn as_id(&self) -> Option<&SessionConfigValueId> {
        match self {
            Self::Id { value } => Some(value),
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
        Self::Id { value }
    }
}

impl From<bool> for SessionConfigOptionValue {
    fn from(value: bool) -> Self {
        Self::Boolean { value }
    }
}

impl From<&str> for SessionConfigOptionValue {
    fn from(value: &str) -> Self {
        Self::Id {
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
    /// Payloads must send `type: "id"` for id-based options.
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
    /// Only available when the Agent capabilities include `session.mcp.http`.
    Http(McpServerHttp),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// ACP transport configuration
    ///
    /// Only available when the Agent capabilities include `session.mcp.acp`.
    /// The MCP server is provided by an ACP component and communicates over the ACP channel.
    #[cfg(feature = "unstable_mcp_over_acp")]
    Acp(McpServerAcp),
    /// Stdio transport configuration
    ///
    /// Only available when the Agent capabilities include `session.mcp.stdio`.
    Stdio(McpServerStdio),
    /// Custom or future MCP server transport configuration.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this transport should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding session setup
    /// data, and otherwise ignore it or reject the server configuration.
    #[serde(untagged)]
    Other(OtherMcpServer),
}

/// Custom or future MCP server transport payload.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_mcp_server_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherMcpServer {
    /// Custom or future MCP server transport type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown MCP server transport payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherMcpServer {
    /// Builds [`OtherMcpServer`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherMcpServer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        if is_known_mcp_server_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known MCP server transport `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

fn is_known_mcp_server_type(type_: &str) -> bool {
    match type_ {
        "http" | "stdio" => true,
        #[cfg(feature = "unstable_mcp_over_acp")]
        "acp" => true,
        _ => false,
    }
}

fn other_mcp_server_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        &[
            "http",
            "stdio",
            #[cfg(feature = "unstable_mcp_over_acp")]
            "acp",
        ],
    );
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
    #[schemars(url)]
    pub url: String,
    /// HTTP headers to set when making requests to the MCP server.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[from(forward)]
#[non_exhaustive]
pub struct McpServerAcpId(pub Arc<str>);

#[cfg(feature = "unstable_mcp_over_acp")]
impl McpServerAcpId {
    /// Wraps a protocol string as a typed [`McpServerAcpId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
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
    pub fn new(name: impl Into<String>, server_id: impl Into<McpServerAcpId>) -> Self {
        Self {
            name: name.into(),
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
    pub command: AbsolutePath,
    /// Command-line arguments to pass to the MCP server.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Environment variables to set when launching the MCP server.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub fn new(name: impl Into<String>, command: impl Into<AbsolutePath>) -> Self {
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
/// See protocol docs: [User Message](https://agentclientprotocol.com/protocol/prompt-lifecycle#1-user-message)
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

/// Response acknowledging that a user prompt was accepted.
///
/// This response does not indicate that the agent has finished processing.
/// Processing and completion are reported through `state_update` session updates.
///
/// See protocol docs: [Prompt Accepted](https://agentclientprotocol.com/protocol/prompt-lifecycle#2-prompt-accepted)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_PROMPT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PromptResponse {
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

/// Reasons why an agent stops active session work.
///
/// See protocol docs: [Stop Reasons](https://agentclientprotocol.com/protocol/prompt-lifecycle#stop-reasons)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StopReason {
    /// The active work ended successfully.
    EndTurn,
    /// The active work ended because the agent reached the maximum number of tokens.
    MaxTokens,
    /// The active work ended because the agent reached the maximum number of
    /// allowed agent requests before returning idle.
    MaxTurnRequests,
    /// The active work ended because the agent refused to continue. The user
    /// prompt and everything that comes after it won't be included in the next
    /// prompt, so this should be reflected in the UI.
    Refusal,
    /// Active session work was cancelled by the client via `session/cancel`.
    ///
    /// Agents should report this stop reason on an idle `state_update` session update
    /// when cancellation succeeds, even if cancellation causes exceptions in
    /// underlying operations.
    Cancelled,
    /// Custom or future stop reason.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Token usage information for completed session work.
#[cfg(feature = "unstable_end_turn_token_usage")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Usage {
    /// Sum of all token types across session.
    pub total_tokens: u64,
    /// Total input tokens.
    pub input_tokens: u64,
    /// Total output tokens.
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
    /// Custom or future protocol.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
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
    #[schemars(url)]
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
#[from(forward)]
#[non_exhaustive]
pub struct ProviderId(pub Arc<str>);

#[cfg(feature = "unstable_llm_providers")]
impl ProviderId {
    /// Wraps a protocol string as a typed [`ProviderId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
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
    #[schemars(url)]
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
    /// Session capabilities supported by the agent.
    ///
    /// Optional. Omitted or `null` both mean the agent does not support the
    /// `session/*` method surface. Supplying `{}` means the agent supports the
    /// baseline session methods: `session/new`, `session/prompt`,
    /// `session/cancel`, and `session/update`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub session: Option<SessionCapabilities>,
    /// Authentication-related extension capabilities supported by the agent.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise any
    /// authentication-related extensions. This field does not advertise support
    /// for `auth/login` or `auth/logout`; those methods are advertised by a
    /// non-empty `authMethods` list in the `initialize` response.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub auth: Option<AgentAuthCapabilities>,
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

    /// Session capabilities supported by the agent.
    ///
    /// Omitted or `null` both mean the agent does not support the `session/*`
    /// method surface. Supplying `{}` means the agent supports the baseline
    /// session methods: `session/new`, `session/prompt`, `session/cancel`, and
    /// `session/update`.
    #[must_use]
    pub fn session(mut self, session: impl IntoOption<SessionCapabilities>) -> Self {
        self.session = session.into_option();
        self
    }

    /// Authentication-related extension capabilities supported by the agent.
    ///
    /// This field does not advertise support for `auth/login` or `auth/logout`.
    #[must_use]
    pub fn auth(mut self, auth: impl IntoOption<AgentAuthCapabilities>) -> Self {
        self.auth = auth.into_option();
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
/// Supplying `{}` means the agent supports the baseline session methods:
/// `session/new`, `session/list`, `session/resume`, `session/close`,
/// `session/prompt`, `session/cancel`, and `session/update`.
///
/// Agents that support sessions **MAY** support additional session methods,
/// prompt content types, and MCP transports by specifying additional
/// capabilities.
///
/// See protocol docs: [Session Capabilities](https://agentclientprotocol.com/protocol/initialization#session-capabilities)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionCapabilities {
    /// Prompt capabilities supported by the agent in `session/prompt` requests.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise any
    /// prompt extensions beyond the baseline text and resource-link content
    /// required by `session/prompt`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub prompt: Option<PromptCapabilities>,
    /// MCP capabilities supported by the agent for session lifecycle requests.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise MCP
    /// server transport support for sessions.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mcp: Option<McpCapabilities>,
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
    /// Agents may return `SessionInfo.additionalDirectories` to report the
    /// complete ordered additional-root list associated with a listed session.
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

    /// Prompt capabilities supported by the agent in `session/prompt` requests.
    ///
    /// Omitted or `null` both mean the agent does not advertise any prompt
    /// extensions beyond the baseline text and resource-link content required by
    /// `session/prompt`.
    #[must_use]
    pub fn prompt(mut self, prompt: impl IntoOption<PromptCapabilities>) -> Self {
        self.prompt = prompt.into_option();
        self
    }

    /// MCP capabilities supported by the agent for session lifecycle requests.
    ///
    /// Omitted or `null` both mean the agent does not advertise MCP server
    /// transport support for sessions.
    #[must_use]
    pub fn mcp(mut self, mcp: impl IntoOption<McpCapabilities>) -> Self {
        self.mcp = mcp.into_option();
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
    /// Agents may return `SessionInfo.additionalDirectories` to report the
    /// complete ordered additional-root list associated with a listed session.
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
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports image content in prompts.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub image: Option<PromptImageCapabilities>,
    /// Agent supports [`ContentBlock::Audio`].
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports audio content in prompts.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub audio: Option<PromptAudioCapabilities>,
    /// Agent supports embedded context in `session/prompt` requests.
    ///
    /// When enabled, the Client is allowed to include [`ContentBlock::Resource`]
    /// in prompt requests for pieces of context that are referenced in the message.
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports embedded context in prompts.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub embedded_context: Option<PromptEmbeddedContextCapabilities>,
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
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports image content in prompts.
    #[must_use]
    pub fn image(mut self, image: impl IntoOption<PromptImageCapabilities>) -> Self {
        self.image = image.into_option();
        self
    }

    /// Agent supports [`ContentBlock::Audio`].
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports audio content in prompts.
    #[must_use]
    pub fn audio(mut self, audio: impl IntoOption<PromptAudioCapabilities>) -> Self {
        self.audio = audio.into_option();
        self
    }

    /// Agent supports embedded context in `session/prompt` requests.
    ///
    /// When enabled, the Client is allowed to include [`ContentBlock::Resource`]
    /// in prompt requests for pieces of context that are referenced in the message.
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports embedded context in prompts.
    #[must_use]
    pub fn embedded_context(
        mut self,
        embedded_context: impl IntoOption<PromptEmbeddedContextCapabilities>,
    ) -> Self {
        self.embedded_context = embedded_context.into_option();
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

/// Capabilities for image content in prompt requests.
///
/// Supplying `{}` means the agent supports image content in prompts.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct PromptImageCapabilities {
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

impl PromptImageCapabilities {
    /// Builds an empty [`PromptImageCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for audio content in prompt requests.
///
/// Supplying `{}` means the agent supports audio content in prompts.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct PromptAudioCapabilities {
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

impl PromptAudioCapabilities {
    /// Builds an empty [`PromptAudioCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for embedded context in prompt requests.
///
/// Supplying `{}` means the agent supports embedded context in prompts.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct PromptEmbeddedContextCapabilities {
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

impl PromptEmbeddedContextCapabilities {
    /// Builds an empty [`PromptEmbeddedContextCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// MCP capabilities supported by the agent for session lifecycle requests.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct McpCapabilities {
    /// Agent supports [`McpServer::Stdio`].
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports stdio MCP server transports.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub stdio: Option<McpStdioCapabilities>,
    /// Agent supports [`McpServer::Http`].
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports HTTP MCP server transports.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub http: Option<McpHttpCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Agent supports [`McpServer::Acp`].
    ///
    /// Optional. Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports ACP MCP server transports.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub acp: Option<McpAcpCapabilities>,
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

    /// Agent supports [`McpServer::Stdio`].
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports stdio MCP server transports.
    #[must_use]
    pub fn stdio(mut self, stdio: impl IntoOption<McpStdioCapabilities>) -> Self {
        self.stdio = stdio.into_option();
        self
    }

    /// Agent supports [`McpServer::Http`].
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports HTTP MCP server transports.
    #[must_use]
    pub fn http(mut self, http: impl IntoOption<McpHttpCapabilities>) -> Self {
        self.http = http.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Agent supports [`McpServer::Acp`].
    #[cfg(feature = "unstable_mcp_over_acp")]
    ///
    /// Omitted or `null` both mean the agent does not advertise support.
    /// Supplying `{}` means the agent supports ACP MCP server transports.
    #[must_use]
    pub fn acp(mut self, acp: impl IntoOption<McpAcpCapabilities>) -> Self {
        self.acp = acp.into_option();
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

/// Capabilities for stdio MCP server transports.
///
/// Supplying `{}` means the agent supports stdio MCP server transports.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct McpStdioCapabilities {
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

impl McpStdioCapabilities {
    /// Builds an empty [`McpStdioCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Capabilities for HTTP MCP server transports.
///
/// Supplying `{}` means the agent supports HTTP MCP server transports.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct McpHttpCapabilities {
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

impl McpHttpCapabilities {
    /// Builds an empty [`McpHttpCapabilities`]; use builder methods to advertise supported sub-capabilities.
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
/// Capabilities for ACP MCP server transports.
///
/// Supplying `{}` means the agent supports ACP MCP server transports.
#[cfg(feature = "unstable_mcp_over_acp")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct McpAcpCapabilities {
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
impl McpAcpCapabilities {
    /// Builds an empty [`McpAcpCapabilities`]; use builder methods to advertise supported sub-capabilities.
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

/// Notification to cancel ongoing operations for a session.
///
/// See protocol docs: [Cancellation](https://agentclientprotocol.com/protocol/prompt-lifecycle#cancellation)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = SESSION_CANCEL_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelSessionNotification {
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

impl CancelSessionNotification {
    /// Builds [`CancelSessionNotification`] with the required notification fields set; optional fields start unset or empty.
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
    pub auth_login: &'static str,
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
    pub auth_logout: &'static str,
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
    auth_login: AUTH_LOGIN_METHOD_NAME,
    #[cfg(feature = "unstable_llm_providers")]
    providers_list: PROVIDERS_LIST_METHOD_NAME,
    #[cfg(feature = "unstable_llm_providers")]
    providers_set: PROVIDERS_SET_METHOD_NAME,
    #[cfg(feature = "unstable_llm_providers")]
    providers_disable: PROVIDERS_DISABLE_METHOD_NAME,
    session_new: SESSION_NEW_METHOD_NAME,
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
    auth_logout: AUTH_LOGOUT_METHOD_NAME,
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
/// Method name for the `auth/login` request.
pub(crate) const AUTH_LOGIN_METHOD_NAME: &str = "auth/login";
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
/// Method name for the `auth/logout` request.
pub(crate) const AUTH_LOGOUT_METHOD_NAME: &str = "auth/logout";

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
    InitializeRequest(Box<InitializeRequest>),
    /// Authenticates the client using the specified authentication method.
    ///
    /// Agents MUST support this method when their `initialize` response advertised
    /// at least one valid authentication method. Clients MUST NOT call this method
    /// when `authMethods` was omitted or empty.
    ///
    /// Called when the agent requires authentication before allowing session creation.
    /// The client provides the authentication method ID that was advertised during initialization.
    ///
    /// After successful authentication, the client can proceed to create sessions with
    /// `new_session` without receiving an `auth_required` error.
    ///
    /// See protocol docs: [Initialization](https://agentclientprotocol.com/protocol/initialization)
    LoginAuthRequest(Box<LoginAuthRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Lists providers that can be configured by the client.
    #[cfg(feature = "unstable_llm_providers")]
    ListProvidersRequest(Box<ListProvidersRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Replaces the configuration for a provider.
    #[cfg(feature = "unstable_llm_providers")]
    SetProviderRequest(Box<SetProviderRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Disables a provider.
    #[cfg(feature = "unstable_llm_providers")]
    DisableProviderRequest(Box<DisableProviderRequest>),
    /// Logs out of the current authenticated state.
    ///
    /// Agents MUST support this method when their `initialize` response advertised
    /// at least one valid authentication method. Clients MUST NOT call this method
    /// when `authMethods` was omitted or empty.
    ///
    /// After a successful logout, authentication-gated requests require the client
    /// to authenticate again. There is no guarantee about the behavior of already
    /// running sessions.
    LogoutAuthRequest(Box<LogoutAuthRequest>),
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
    NewSessionRequest(Box<NewSessionRequest>),
    /// Lists existing sessions known to the agent.
    ///
    /// The agent should return metadata about sessions with optional filtering and pagination support.
    ListSessionsRequest(Box<ListSessionsRequest>),
    /// Deletes an existing session from `session/list`.
    ///
    /// This method is only available if the agent advertises the `session.delete` capability.
    DeleteSessionRequest(Box<DeleteSessionRequest>),
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
    ForkSessionRequest(Box<ForkSessionRequest>),
    /// Resumes an existing session.
    ///
    /// The agent should resume the session context, allowing the conversation
    /// to continue. If `replayFrom` is set, the agent should replay
    /// conversation history before responding.
    ResumeSessionRequest(Box<ResumeSessionRequest>),
    /// Closes an active session and frees up any resources associated with it.
    ///
    /// The agent must cancel any ongoing work (as if `session/cancel` was called)
    /// and then free up any resources associated with the session.
    CloseSessionRequest(Box<CloseSessionRequest>),
    /// Sets the current value for a session configuration option.
    SetSessionConfigOptionRequest(Box<SetSessionConfigOptionRequest>),
    /// Processes a user prompt within a session.
    ///
    /// This request accepts the prompt:
    /// - Receives user messages with optional context (files, images, etc.)
    /// - Returns once the prompt is accepted
    ///
    /// After acceptance, the Agent reports the accepted user message,
    /// processing state, output, tool calls, and completion through
    /// `session/update` notifications.
    ///
    /// See protocol docs: [Prompt Lifecycle](https://agentclientprotocol.com/protocol/prompt-lifecycle)
    PromptRequest(Box<PromptRequest>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Starts an NES session.
    StartNesRequest(Box<StartNesRequest>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Requests a code suggestion.
    SuggestNesRequest(Box<SuggestNesRequest>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Closes an active NES session and frees up any resources associated with it.
    ///
    /// The agent must cancel any ongoing work and then free up any resources
    /// associated with the NES session.
    CloseNesRequest(Box<CloseNesRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Exchanges an MCP-over-ACP message.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpRequest(Box<MessageMcpRequest>),
    /// Handles extension method requests from the client.
    ///
    /// Extension methods provide a way to add custom functionality while maintaining
    /// protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtMethodRequest(Box<ExtRequest>),
}

impl ClientRequest {
    /// Returns the corresponding method name of the request.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::InitializeRequest(_) => AGENT_METHOD_NAMES.initialize,
            Self::LoginAuthRequest(_) => AGENT_METHOD_NAMES.auth_login,
            #[cfg(feature = "unstable_llm_providers")]
            Self::ListProvidersRequest(_) => AGENT_METHOD_NAMES.providers_list,
            #[cfg(feature = "unstable_llm_providers")]
            Self::SetProviderRequest(_) => AGENT_METHOD_NAMES.providers_set,
            #[cfg(feature = "unstable_llm_providers")]
            Self::DisableProviderRequest(_) => AGENT_METHOD_NAMES.providers_disable,
            Self::LogoutAuthRequest(_) => AGENT_METHOD_NAMES.auth_logout,
            Self::NewSessionRequest(_) => AGENT_METHOD_NAMES.session_new,
            Self::ListSessionsRequest(_) => AGENT_METHOD_NAMES.session_list,
            Self::DeleteSessionRequest(_) => AGENT_METHOD_NAMES.session_delete,
            #[cfg(feature = "unstable_session_fork")]
            Self::ForkSessionRequest(_) => AGENT_METHOD_NAMES.session_fork,
            Self::ResumeSessionRequest(_) => AGENT_METHOD_NAMES.session_resume,
            Self::CloseSessionRequest(_) => AGENT_METHOD_NAMES.session_close,
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
pub enum AgentResponse {
    /// Successful result returned for a `initialize` request.
    InitializeResponse(Box<InitializeResponse>),
    /// Successful result returned for an `auth/login` request.
    LoginAuthResponse(#[serde(default)] Box<LoginAuthResponse>),
    /// Successful result returned for a `providers/list` request.
    #[cfg(feature = "unstable_llm_providers")]
    ListProvidersResponse(Box<ListProvidersResponse>),
    /// Successful result returned for a `providers/set` request.
    #[cfg(feature = "unstable_llm_providers")]
    SetProviderResponse(#[serde(default)] Box<SetProviderResponse>),
    /// Successful result returned for a `providers/disable` request.
    #[cfg(feature = "unstable_llm_providers")]
    DisableProviderResponse(#[serde(default)] Box<DisableProviderResponse>),
    /// Successful result returned for an `auth/logout` request.
    LogoutAuthResponse(#[serde(default)] Box<LogoutAuthResponse>),
    /// Successful result returned for a `session/new` request.
    NewSessionResponse(Box<NewSessionResponse>),
    /// Successful result returned for a `session/list` request.
    ListSessionsResponse(Box<ListSessionsResponse>),
    /// Successful result returned for a `session/delete` request.
    DeleteSessionResponse(#[serde(default)] Box<DeleteSessionResponse>),
    /// Successful result returned for a `session/fork` request.
    #[cfg(feature = "unstable_session_fork")]
    ForkSessionResponse(Box<ForkSessionResponse>),
    /// Successful result returned for a `session/resume` request.
    ResumeSessionResponse(#[serde(default)] Box<ResumeSessionResponse>),
    /// Successful result returned for a `session/close` request.
    CloseSessionResponse(#[serde(default)] Box<CloseSessionResponse>),
    /// Successful result returned for a `session/set_config_option` request.
    SetSessionConfigOptionResponse(Box<SetSessionConfigOptionResponse>),
    /// Successful result returned for a `session/prompt` request.
    PromptResponse(Box<PromptResponse>),
    /// Successful result returned for a `nes/start` request.
    #[cfg(feature = "unstable_nes")]
    StartNesResponse(Box<StartNesResponse>),
    /// Successful result returned for a `nes/suggest` request.
    #[cfg(feature = "unstable_nes")]
    SuggestNesResponse(Box<SuggestNesResponse>),
    /// Successful result returned for a `nes/close` request.
    #[cfg(feature = "unstable_nes")]
    CloseNesResponse(#[serde(default)] Box<CloseNesResponse>),
    /// Successful result returned by an extension method outside the core ACP method set.
    ExtMethodResponse(Box<ExtResponse>),
    /// Successful result returned by an MCP-over-ACP `mcp/message` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpResponse(Box<MessageMcpResponse>),
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
pub enum ClientNotification {
    /// Cancels ongoing operations for a session.
    ///
    /// This is a notification sent by the client to cancel active work in a
    /// session.
    ///
    /// Upon receiving this notification, the Agent SHOULD:
    /// - Stop all language model requests as soon as possible
    /// - Abort all tool call invocations in progress
    /// - Send any pending `session/update` notifications
    /// - Report an idle `state_update` with `StopReason::Cancelled` after
    ///   cancellation succeeds
    ///
    /// See protocol docs: [Cancellation](https://agentclientprotocol.com/protocol/prompt-lifecycle#cancellation)
    CancelSessionNotification(Box<CancelSessionNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is opened in the editor.
    DidOpenDocumentNotification(Box<DidOpenDocumentNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is edited.
    DidChangeDocumentNotification(Box<DidChangeDocumentNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is closed.
    DidCloseDocumentNotification(Box<DidCloseDocumentNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file is saved.
    DidSaveDocumentNotification(Box<DidSaveDocumentNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a file becomes the active editor tab.
    DidFocusDocumentNotification(Box<DidFocusDocumentNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a suggestion is accepted.
    AcceptNesNotification(Box<AcceptNesNotification>),
    #[cfg(feature = "unstable_nes")]
    /// **UNSTABLE**
    ///
    /// Notification sent when a suggestion is rejected.
    RejectNesNotification(Box<RejectNesNotification>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Sends an MCP-over-ACP notification.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpNotification(Box<MessageMcpNotification>),
    /// Handles extension notifications from the client.
    ///
    /// Extension notifications provide a way to send one-way messages for custom functionality
    /// while maintaining protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtNotification(Box<ExtNotification>),
}

impl ClientNotification {
    /// Returns the corresponding method name of the notification.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::CancelSessionNotification(_) => AGENT_METHOD_NAMES.session_cancel,
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

#[cfg(test)]
mod test_serialization {
    use std::path::PathBuf;

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
            "protocolVersion": 2,
            "capabilities": false,
            "info": {
                "name": "client",
                "version": "1.0.0"
            }
        }))
        .unwrap();
        assert_eq!(request.capabilities, ClientCapabilities::default());

        let response: InitializeResponse = serde_json::from_value(json!({
            "protocolVersion": 2,
            "capabilities": false,
            "info": {
                "name": "agent",
                "version": "1.0.0"
            }
        }))
        .unwrap();
        assert_eq!(response.capabilities, AgentCapabilities::default());
    }

    #[test]
    fn test_agent_capabilities_default_on_malformed_values() {
        let capabilities: AgentCapabilities = serde_json::from_value(json!({
            "session": false,
            "auth": false
        }))
        .unwrap();

        assert!(capabilities.session.is_none());
        assert_eq!(capabilities.auth, None);
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
                "type": "stdio",
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
                assert_eq!(command, AbsolutePath::new("/usr/bin/server"));
                assert_eq!(args, vec!["--port", "3000"]);
                assert_eq!(env.len(), 1);
                assert_eq!(env[0].name, "API_KEY");
                assert_eq!(env[0].value, "secret123");
            }
            _ => panic!("Expected Stdio variant"),
        }
    }

    #[test]
    fn test_mcp_server_empty_arrays_are_optional() {
        let stdio = McpServer::Stdio(McpServerStdio::new("test-server", "/usr/bin/server"));
        assert_eq!(
            serde_json::to_value(&stdio).unwrap(),
            json!({
                "type": "stdio",
                "name": "test-server",
                "command": "/usr/bin/server"
            })
        );

        let McpServer::Stdio(McpServerStdio { args, env, .. }) =
            serde_json::from_value::<McpServer>(json!({
                "type": "stdio",
                "name": "test-server",
                "command": "/usr/bin/server"
            }))
            .unwrap()
        else {
            panic!("Expected Stdio variant");
        };
        assert!(args.is_empty());
        assert!(env.is_empty());

        let http = McpServer::Http(McpServerHttp::new("http-server", "https://api.example.com"));
        assert_eq!(
            serde_json::to_value(&http).unwrap(),
            json!({
                "type": "http",
                "name": "http-server",
                "url": "https://api.example.com"
            })
        );

        let McpServer::Http(McpServerHttp { headers, .. }) =
            serde_json::from_value::<McpServer>(json!({
                "type": "http",
                "name": "http-server",
                "url": "https://api.example.com"
            }))
            .unwrap()
        else {
            panic!("Expected Http variant");
        };
        assert!(headers.is_empty());
    }

    #[test]
    fn test_mcp_server_unknown_transport_serialization() {
        let json = json!({
            "type": "websocket",
            "name": "future-server",
            "url": "wss://example.com/mcp",
            "protocolVersion": "2026-01-01"
        });

        let deserialized: McpServer = serde_json::from_value(json.clone()).unwrap();
        let McpServer::Other(OtherMcpServer { type_, fields }) = &deserialized else {
            panic!("Expected Other variant");
        };

        assert_eq!(type_, "websocket");
        assert_eq!(fields["name"], "future-server");
        assert_eq!(fields["url"], "wss://example.com/mcp");
        assert_eq!(fields["protocolVersion"], "2026-01-01");
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), json);
    }

    #[test]
    fn test_mcp_server_stdio_requires_type() {
        let result = serde_json::from_value::<McpServer>(json!({
            "name": "test-server",
            "command": "/usr/bin/server",
            "args": [],
            "env": []
        }));

        assert!(result.is_err());
    }

    #[test]
    fn test_mcp_server_unknown_does_not_hide_malformed_known_transport() {
        let result = serde_json::from_value::<McpServer>(json!({
            "type": "stdio",
            "name": "test-server",
            "args": [],
            "env": []
        }));

        assert!(result.is_err());
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

    #[test]
    fn mcp_server_http_schema_marks_url_as_uri() {
        let schema = serde_json::to_value(schemars::schema_for!(McpServerHttp)).unwrap();

        assert_eq!(schema["properties"]["url"]["format"], "uri");
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn test_client_mcp_message_method_names() {
        assert_eq!(AGENT_METHOD_NAMES.mcp_message, "mcp/message");

        assert_eq!(
            ClientRequest::MessageMcpRequest(Box::new(MessageMcpRequest::new(
                "conn-1",
                "tools/list"
            )))
            .method(),
            "mcp/message"
        );
        assert_eq!(
            ClientNotification::MessageMcpNotification(Box::new(MessageMcpNotification::new(
                "conn-1",
                "notifications/progress"
            )))
            .method(),
            "mcp/message"
        );
    }

    #[test]
    fn test_auth_method_names() {
        assert_eq!(AGENT_METHOD_NAMES.auth_login, "auth/login");
        assert_eq!(AGENT_METHOD_NAMES.auth_logout, "auth/logout");

        assert_eq!(
            ClientRequest::LoginAuthRequest(Box::new(LoginAuthRequest::new("agent-login")))
                .method(),
            "auth/login"
        );
        assert_eq!(
            ClientRequest::LogoutAuthRequest(Box::new(LogoutAuthRequest::new())).method(),
            "auth/logout"
        );
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

    fn test_config_option() -> SessionConfigOption {
        SessionConfigOption::select(
            "mode",
            "Mode",
            "ask",
            vec![SessionConfigSelectOption::new("ask", "Ask")],
        )
    }

    #[test]
    fn test_session_response_config_options_default_empty_and_skip_serializing() {
        assert_eq!(
            serde_json::to_value(NewSessionResponse::new("sess")).unwrap(),
            json!({ "sessionId": "sess" })
        );
        assert_eq!(
            serde_json::to_value(ResumeSessionResponse::new()).unwrap(),
            json!({})
        );
        #[cfg(feature = "unstable_session_fork")]
        assert_eq!(
            serde_json::to_value(ForkSessionResponse::new("fork")).unwrap(),
            json!({ "sessionId": "fork" })
        );

        let json = serde_json::to_value(
            NewSessionResponse::new("sess").config_options(vec![test_config_option()]),
        )
        .unwrap();
        assert_eq!(json["configOptions"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_session_response_config_options_deserialize_missing_null_and_invalid() {
        let missing: NewSessionResponse =
            serde_json::from_value(json!({ "sessionId": "sess" })).unwrap();
        assert!(missing.config_options.is_empty());

        let null: NewSessionResponse = serde_json::from_value(json!({
            "sessionId": "sess",
            "configOptions": null
        }))
        .unwrap();
        assert!(null.config_options.is_empty());

        let wrong_shape: NewSessionResponse = serde_json::from_value(json!({
            "sessionId": "sess",
            "configOptions": "oops"
        }))
        .unwrap();
        assert!(wrong_shape.config_options.is_empty());

        let valid_option = serde_json::to_value(test_config_option()).unwrap();
        let mixed: NewSessionResponse = serde_json::from_value(json!({
            "sessionId": "sess",
            "configOptions": ["oops", valid_option]
        }))
        .unwrap();
        assert_eq!(mixed.config_options.len(), 1);

        let resume: ResumeSessionResponse = serde_json::from_value(json!({})).unwrap();
        assert!(resume.config_options.is_empty());
        #[cfg(feature = "unstable_session_fork")]
        {
            let fork: ForkSessionResponse =
                serde_json::from_value(json!({ "sessionId": "fork" })).unwrap();
            assert!(fork.config_options.is_empty());
        }
    }

    #[test]
    fn test_resume_session_replay_from_serialization() {
        assert_eq!(
            serde_json::to_value(ResumeSessionRequest::new(
                "sess_abc123",
                "/home/user/project"
            ))
            .unwrap(),
            json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project"
            })
        );
        assert_eq!(
            serde_json::to_value(
                ResumeSessionRequest::new("sess_abc123", "/home/user/project")
                    .replay_from(ReplayFrom::from(ReplayFromStart::new()))
            )
            .unwrap(),
            json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project",
                "replayFrom": {
                    "type": "start"
                }
            })
        );

        let replay: ResumeSessionRequest = serde_json::from_value(json!({
            "sessionId": "sess_abc123",
            "cwd": "/home/user/project",
            "replayFrom": {
                "type": "start"
            }
        }))
        .unwrap();
        assert!(matches!(replay.replay_from, Some(ReplayFrom::Start(_))));

        let none: ResumeSessionRequest = serde_json::from_value(json!({
            "sessionId": "sess_abc123",
            "cwd": "/home/user/project",
            "replayFrom": null
        }))
        .unwrap();
        assert!(none.replay_from.is_none());
    }

    #[test]
    fn test_auth_method_agent_serialization() {
        let method = AuthMethod::Agent(AuthMethodAgent::new("default-auth", "Default Auth"));

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "methodId": "default-auth",
                "name": "Default Auth",
                "type": "agent"
            })
        );
        // description should be omitted when None
        assert!(!json.as_object().unwrap().contains_key("description"));

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::Agent(AuthMethodAgent {
                method_id, name, ..
            }) => {
                assert_eq!(method_id.0.as_ref(), "default-auth");
                assert_eq!(name, "Default Auth");
            }
            _ => panic!("Expected Agent variant"),
        }
    }

    #[test]
    fn test_auth_method_agent_deserialization() {
        let json = json!({
            "methodId": "agent-auth",
            "name": "Agent Auth",
            "type": "agent"
        });

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        assert!(matches!(deserialized, AuthMethod::Agent(_)));
    }

    #[test]
    fn test_auth_method_agent_requires_type() {
        assert!(
            serde_json::from_value::<AuthMethod>(json!({
                "methodId": "agent-auth",
                "name": "Agent Auth"
            }))
            .is_err()
        );
    }

    #[test]
    fn test_auth_method_agent_rejects_null_type() {
        assert!(
            serde_json::from_value::<AuthMethod>(json!({
                "methodId": "agent-auth",
                "name": "Agent Auth",
                "type": null
            }))
            .is_err()
        );
    }

    #[test]
    fn test_auth_method_unknown_does_not_hide_malformed_agent() {
        assert!(
            serde_json::from_value::<AuthMethod>(json!({
                "methodId": "agent-auth",
                "type": "agent"
            }))
            .is_err()
        );
    }

    #[test]
    fn test_auth_method_unknown_variant_roundtrip() {
        let method: AuthMethod = serde_json::from_value(json!({
            "methodId": "oauth",
            "name": "OAuth",
            "type": "_oauth",
            "authorizationUrl": "https://example.com/auth"
        }))
        .unwrap();

        assert_eq!(method.method_id().0.as_ref(), "oauth");
        assert_eq!(method.name(), "OAuth");
        let AuthMethod::Other(unknown) = method else {
            panic!("expected unknown auth method");
        };
        assert_eq!(unknown.type_, "_oauth");
        assert_eq!(
            unknown.fields.get("authorizationUrl"),
            Some(&json!("https://example.com/auth"))
        );

        assert_eq!(
            serde_json::to_value(AuthMethod::Other(unknown)).unwrap(),
            json!({
                "methodId": "oauth",
                "name": "OAuth",
                "type": "_oauth",
                "authorizationUrl": "https://example.com/auth"
            })
        );
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_method_unknown_does_not_hide_malformed_known_variant() {
        assert!(
            serde_json::from_value::<AuthMethod>(json!({
                "methodId": "api-key",
                "name": "API Key",
                "type": "env_var"
            }))
            .is_err()
        );
    }

    #[test]
    fn test_session_delete_serialization() {
        assert_eq!(AGENT_METHOD_NAMES.session_delete, "session/delete");
        assert_eq!(
            ClientRequest::DeleteSessionRequest(Box::new(DeleteSessionRequest::new("sess_abc123")))
                .method(),
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
            })
        );
        assert_eq!(
            serde_json::to_value(ResumeSessionRequest::new(
                "sess_abc123",
                "/home/user/project"
            ))
            .unwrap(),
            json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project",
            })
        );
        assert_eq!(
            serde_json::from_value::<ResumeSessionRequest>(json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project"
            }))
            .unwrap()
            .mcp_servers,
            Vec::<McpServer>::new()
        );
        assert_eq!(
            serde_json::from_value::<ResumeSessionRequest>(json!({
                "sessionId": "sess_abc123",
                "cwd": "/home/user/project",
                "mcpServers": null
            }))
            .unwrap()
            .mcp_servers,
            Vec::<McpServer>::new()
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
            Vec::<AbsolutePath>::new()
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
                "methodId": "api-key",
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
                method_id,
                name: method_name,
                vars,
                link,
                ..
            }) => {
                assert_eq!(method_id.0.as_ref(), "api-key");
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
                "methodId": "api-key",
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
                "methodId": "azure-openai",
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
                "methodId": "tui-auth",
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
        let method = AuthMethod::Terminal(
            AuthMethodTerminal::new("tui-auth", "Terminal Auth")
                .args(vec!["--interactive".to_string(), "--color".to_string()])
                .env(vec![EnvVariable::new("TERM", "xterm-256color")]),
        );

        let json = serde_json::to_value(&method).unwrap();
        assert_eq!(
            json,
            json!({
                "methodId": "tui-auth",
                "name": "Terminal Auth",
                "type": "terminal",
                "args": ["--interactive", "--color"],
                "env": [
                    {
                        "name": "TERM",
                        "value": "xterm-256color"
                    }
                ]
            })
        );

        let deserialized: AuthMethod = serde_json::from_value(json).unwrap();
        match deserialized {
            AuthMethod::Terminal(AuthMethodTerminal { args, env, .. }) => {
                assert_eq!(args, vec!["--interactive", "--color"]);
                assert_eq!(env.len(), 1);
                assert_eq!(env[0].name, "TERM");
                assert_eq!(env[0].value, "xterm-256color");
            }
            _ => panic!("Expected Terminal variant"),
        }
    }

    #[test]
    fn test_session_config_option_id_serialize() {
        let val = SessionConfigOptionValue::id("model-1");
        let json = serde_json::to_value(&val).unwrap();
        assert_eq!(json, json!({ "type": "id", "value": "model-1" }));
    }

    #[test]
    fn test_session_config_option_value_boolean_serialize() {
        let val = SessionConfigOptionValue::boolean(true);
        let json = serde_json::to_value(&val).unwrap();
        assert_eq!(json, json!({ "type": "boolean", "value": true }));
    }

    #[test]
    fn test_session_config_option_value_deserialize_id() {
        let json = json!({ "type": "id", "value": "model-1" });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(val, SessionConfigOptionValue::id("model-1"));
        assert_eq!(val.as_id().unwrap().to_string(), "model-1");
    }

    #[test]
    fn test_session_config_option_value_deserialize_requires_type() {
        let json = json!({ "value": "model-1" });
        let result = serde_json::from_value::<SessionConfigOptionValue>(json);
        assert!(result.is_err());
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
        let json = json!({
            "type": "text",
            "value": "freeform input",
            "maxLength": 200
        });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        let SessionConfigOptionValue::Other(unknown) = val else {
            panic!("Expected Other variant");
        };
        assert_eq!(unknown.type_, "text");
        assert_eq!(unknown.value, json!("freeform input"));
        assert_eq!(unknown.fields["maxLength"], json!(200));
    }

    #[test]
    fn test_session_config_option_value_deserialize_unknown_type_with_object_value() {
        let json = json!({
            "type": "range",
            "value": { "min": 1, "max": 5 }
        });
        let val: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        let SessionConfigOptionValue::Other(unknown) = val else {
            panic!("Expected Other variant");
        };
        assert_eq!(unknown.type_, "range");
        assert_eq!(unknown.value, json!({ "min": 1, "max": 5 }));
    }

    #[test]
    fn test_session_config_option_value_roundtrip_id() {
        let original = SessionConfigOptionValue::id("option-a");
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
    fn test_session_config_option_value_roundtrip_other() {
        let mut fields = BTreeMap::new();
        fields.insert("maxLength".to_string(), json!(200));
        let original = SessionConfigOptionValue::Other(OtherSessionConfigOptionValue::new(
            "text",
            json!("freeform input"),
            fields,
        ));
        let json = serde_json::to_value(&original).unwrap();
        let roundtripped: SessionConfigOptionValue = serde_json::from_value(json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_session_config_option_value_type_mismatch_boolean_with_string() {
        let json = json!({ "type": "boolean", "value": "not a bool" });
        let result = serde_json::from_value::<SessionConfigOptionValue>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_session_config_option_value_from_impls() {
        let from_str: SessionConfigOptionValue = "model-1".into();
        assert_eq!(from_str.as_id().unwrap().to_string(), "model-1");

        let from_id: SessionConfigOptionValue = SessionConfigValueId::new("model-2").into();
        assert_eq!(from_id.as_id().unwrap().to_string(), "model-2");

        let from_bool: SessionConfigOptionValue = true.into();
        assert_eq!(from_bool.as_bool(), Some(true));
    }

    #[test]
    fn test_set_session_config_option_request_id() {
        let req = SetSessionConfigOptionRequest::new("sess_1", "model", "model-1");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "sess_1",
                "configId": "model",
                "type": "id",
                "value": "model-1"
            })
        );
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
    fn test_set_session_config_option_request_deserialize_requires_type() {
        let json = json!({
            "sessionId": "sess_1",
            "configId": "model",
            "value": "model-1"
        });
        let result = serde_json::from_value::<SetSessionConfigOptionRequest>(json);
        assert!(result.is_err());
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
    fn test_set_session_config_option_request_roundtrip_id() {
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
                "configId": "brave_mode",
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
        assert_eq!(deserialized.config_id.to_string(), "brave_mode");
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

    #[test]
    fn test_session_config_option_unknown_kind_roundtrip() {
        let option: SessionConfigOption = serde_json::from_value(json!({
            "configId": "verbosity",
            "name": "Verbosity",
            "type": "_slider",
            "currentValue": 3,
            "min": 0,
            "max": 5,
            "_meta": {
                "source": "test"
            }
        }))
        .unwrap();

        assert_eq!(option.config_id.to_string(), "verbosity");
        assert_eq!(option.meta.as_ref().unwrap()["source"], "test");
        let SessionConfigKind::Other(unknown) = &option.kind else {
            panic!("expected unknown config kind");
        };
        assert_eq!(unknown.type_, "_slider");
        assert_eq!(unknown.fields.get("currentValue"), Some(&json!(3)));
        assert!(!unknown.fields.contains_key("_meta"));
        assert_eq!(serialized_meta_key_count(&option), 1);

        let json = serde_json::to_value(&option).unwrap();
        assert_eq!(json["type"], "_slider");
        assert_eq!(json["currentValue"], 3);
        assert_eq!(json["min"], 0);
        assert_eq!(json["max"], 5);
        assert_eq!(json["_meta"]["source"], "test");
    }

    #[test]
    fn test_session_config_option_unknown_kind_does_not_duplicate_flattened_meta() {
        let mut fields = std::collections::BTreeMap::new();
        fields.insert("currentValue".to_string(), json!(3));
        fields.insert("_meta".to_string(), json!({ "inner": "ignored" }));

        let option = SessionConfigOption::new(
            "verbosity",
            "Verbosity",
            SessionConfigKind::Other(OtherSessionConfigKind::new("_slider", fields)),
        )
        .meta(test_meta());

        let SessionConfigKind::Other(unknown) = &option.kind else {
            panic!("expected unknown config kind");
        };
        assert!(!unknown.fields.contains_key("_meta"));
        assert_eq!(serialized_meta_key_count(&option), 1);

        let json = serde_json::to_value(&option).unwrap();
        assert_eq!(json["type"], "_slider");
        assert_eq!(json["currentValue"], 3);
        assert_eq!(json["_meta"]["source"], "test");
    }

    #[test]
    fn test_session_config_option_unknown_does_not_hide_malformed_known_kind() {
        assert!(
            serde_json::from_value::<SessionConfigOption>(json!({
                "configId": "model",
                "name": "Model",
                "type": "select"
            }))
            .is_err()
        );
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
    fn test_agent_capabilities_session_is_explicit() {
        let json = serde_json::to_value(AgentCapabilities::new()).unwrap();
        assert!(json.get("session").is_none());

        let caps = AgentCapabilities::new().session(
            SessionCapabilities::new()
                .prompt(PromptCapabilities::new().image(PromptImageCapabilities::new()))
                .mcp(McpCapabilities::new().stdio(McpStdioCapabilities::new())),
        );

        assert_eq!(
            serde_json::to_value(&caps).unwrap(),
            json!({
                "session": {
                    "prompt": {
                        "image": {}
                    },
                    "mcp": {
                        "stdio": {}
                    }
                }
            })
        );

        let deserialized: AgentCapabilities = serde_json::from_value(json!({
            "session": false
        }))
        .unwrap();
        assert!(deserialized.session.is_none());
    }

    #[test]
    fn test_prompt_capabilities_serialize_supported_content_as_objects() {
        let caps = PromptCapabilities::new()
            .image(PromptImageCapabilities::new())
            .audio(PromptAudioCapabilities::new())
            .embedded_context(PromptEmbeddedContextCapabilities::new());

        assert_eq!(
            serde_json::to_value(&caps).unwrap(),
            json!({
                "image": {},
                "audio": {},
                "embeddedContext": {}
            })
        );

        let deserialized: PromptCapabilities = serde_json::from_value(json!({
            "image": null,
            "audio": false,
            "embeddedContext": {}
        }))
        .unwrap();
        assert!(deserialized.image.is_none());
        assert!(deserialized.audio.is_none());
        assert!(deserialized.embedded_context.is_some());
    }

    #[test]
    fn test_mcp_capabilities_serialize_supported_transports_as_objects() {
        let caps = McpCapabilities::new()
            .stdio(McpStdioCapabilities::new())
            .http(McpHttpCapabilities::new());

        assert_eq!(
            serde_json::to_value(&caps).unwrap(),
            json!({
                "stdio": {},
                "http": {}
            })
        );

        let deserialized: McpCapabilities = serde_json::from_value(json!({
            "stdio": null,
            "http": false
        }))
        .unwrap();
        assert!(deserialized.stdio.is_none());
        assert!(deserialized.http.is_none());
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn test_mcp_capabilities_serialize_acp_support_as_object() {
        let caps = McpCapabilities::new().acp(McpAcpCapabilities::new());

        assert_eq!(
            serde_json::to_value(&caps).unwrap(),
            json!({
                "acp": {}
            })
        );
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
