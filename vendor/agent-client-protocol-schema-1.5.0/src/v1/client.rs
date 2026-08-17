//! Methods and notifications the client handles/receives.
//!
//! This module defines the Client trait and all associated types for implementing
//! a client that interacts with AI coding agents via the Agent Client Protocol (ACP).

use std::{path::PathBuf, sync::Arc};

use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

#[cfg(feature = "unstable_elicitation")]
use super::{
    CompleteElicitationNotification, CreateElicitationRequest, CreateElicitationResponse,
    ElicitationCapabilities,
};
use crate::{IntoMaybeUndefined, IntoOption, MaybeUndefined, SkipListener};

use super::{
    ContentBlock, EnvVariable, ExtNotification, ExtRequest, ExtResponse, Meta, Plan,
    SessionConfigOption, SessionId, SessionModeId, ToolCall, ToolCallUpdate,
};
#[cfg(feature = "unstable_plan_operations")]
use super::{PlanCapabilities, PlanRemoved, PlanUpdate};

#[cfg(feature = "unstable_mcp_over_acp")]
use super::mcp::{
    ConnectMcpRequest, ConnectMcpResponse, DisconnectMcpRequest, DisconnectMcpResponse,
    MCP_CONNECT_METHOD_NAME, MCP_DISCONNECT_METHOD_NAME, MCP_MESSAGE_METHOD_NAME,
    MessageMcpNotification, MessageMcpRequest, MessageMcpResponse,
};

#[cfg(feature = "unstable_nes")]
use super::{ClientNesCapabilities, PositionEncodingKind};

// Session updates

/// Notification containing a session update from the agent.
///
/// Used to stream real-time progress and results during prompt processing.
///
/// See protocol docs: [Agent Reports Output](https://agentclientprotocol.com/protocol/prompt-turn#3-agent-reports-output)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[schemars(extend("x-side" = "client", "x-method" = SESSION_UPDATE_NOTIFICATION))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionNotification {
    /// The ID of the session this update pertains to.
    pub session_id: SessionId,
    /// The actual update content.
    pub update: SessionUpdate,
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

impl SessionNotification {
    /// Builds [`SessionNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, update: SessionUpdate) -> Self {
        Self {
            session_id: session_id.into(),
            update,
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

/// Different types of updates that can be sent during session processing.
///
/// These updates provide real-time feedback about the agent's progress.
///
/// See protocol docs: [Agent Reports Output](https://agentclientprotocol.com/protocol/prompt-turn#3-agent-reports-output)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "sessionUpdate", rename_all = "snake_case")]
#[schemars(extend("discriminator" = {"propertyName": "sessionUpdate"}))]
#[non_exhaustive]
pub enum SessionUpdate {
    /// A chunk of the user's message being streamed.
    UserMessageChunk(ContentChunk),
    /// A chunk of the agent's response being streamed.
    AgentMessageChunk(ContentChunk),
    /// A chunk of the agent's internal reasoning being streamed.
    AgentThoughtChunk(ContentChunk),
    /// Notification that a new tool call has been initiated.
    ToolCall(ToolCall),
    /// Update on the status or results of a tool call.
    ToolCallUpdate(ToolCallUpdate),
    /// The agent's execution plan for complex tasks.
    /// See protocol docs: [Agent Plan](https://agentclientprotocol.com/protocol/agent-plan)
    Plan(Plan),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// A content update for a plan identified by ID.
    #[cfg(feature = "unstable_plan_operations")]
    PlanUpdate(PlanUpdate),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Removal notice for a plan identified by ID.
    #[cfg(feature = "unstable_plan_operations")]
    PlanRemoved(PlanRemoved),
    /// Available commands are ready or have changed
    AvailableCommandsUpdate(AvailableCommandsUpdate),
    /// The current mode of the session has changed
    ///
    /// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
    CurrentModeUpdate(CurrentModeUpdate),
    /// Session configuration options have been updated.
    ConfigOptionUpdate(ConfigOptionUpdate),
    /// Session metadata has been updated (title, timestamps, custom metadata)
    SessionInfoUpdate(SessionInfoUpdate),
    /// Context window and cost update for the session.
    UsageUpdate(UsageUpdate),
}

/// The current mode of the session has changed
///
/// See protocol docs: [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CurrentModeUpdate {
    /// The ID of the current mode
    pub current_mode_id: SessionModeId,
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

impl CurrentModeUpdate {
    /// Builds [`CurrentModeUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(current_mode_id: impl Into<SessionModeId>) -> Self {
        Self {
            current_mode_id: current_mode_id.into(),
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

/// Session configuration options have been updated.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConfigOptionUpdate {
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

impl ConfigOptionUpdate {
    /// Builds [`ConfigOptionUpdate`] with the required fields set; optional fields start unset or empty.
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

/// Update to session metadata. All fields are optional to support partial updates.
///
/// Agents send this notification to update session information like title or custom metadata.
/// This allows clients to display dynamic session names and track session state changes.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionInfoUpdate {
    /// Human-readable title for the session. Set to null to clear.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub title: MaybeUndefined<String>,
    /// ISO 8601 timestamp of last activity. Set to null to clear.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub updated_at: MaybeUndefined<String>,
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

impl SessionInfoUpdate {
    /// Builds [`SessionInfoUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Human-readable title for the session. Set to null to clear.
    #[must_use]
    pub fn title(mut self, title: impl IntoMaybeUndefined<String>) -> Self {
        self.title = title.into_maybe_undefined();
        self
    }

    /// ISO 8601 timestamp of last activity. Set to null to clear.
    #[must_use]
    pub fn updated_at(mut self, updated_at: impl IntoMaybeUndefined<String>) -> Self {
        self.updated_at = updated_at.into_maybe_undefined();
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

/// Context window and cost update for a session.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UsageUpdate {
    /// Tokens currently in context.
    pub used: u64,
    /// Total context window size in tokens.
    pub size: u64,
    /// Cumulative session cost (optional).
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub cost: Option<Cost>,
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

impl UsageUpdate {
    /// Builds [`UsageUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(used: u64, size: u64) -> Self {
        Self {
            used,
            size,
            cost: None,
            meta: None,
        }
    }

    /// Cumulative session cost (optional).
    #[must_use]
    pub fn cost(mut self, cost: impl IntoOption<Cost>) -> Self {
        self.cost = cost.into_option();
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

/// Cost information for a session.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Cost {
    /// Total cumulative cost for session.
    pub amount: f64,
    /// ISO 4217 currency code (e.g., "USD", "EUR").
    pub currency: String,
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

impl Cost {
    /// Builds [`Cost`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(amount: f64, currency: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into(),
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

/// A streamed item of content
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ContentChunk {
    /// A single item of content
    pub content: ContentBlock,
    /// A unique identifier for the message this chunk belongs to.
    ///
    /// All chunks belonging to the same message share the same `messageId`.
    /// A change in `messageId` indicates a new message has started.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub message_id: Option<MessageId>,
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

impl ContentChunk {
    /// Builds [`ContentChunk`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(content: ContentBlock) -> Self {
        Self {
            content,
            message_id: None,
            meta: None,
        }
    }

    /// A unique identifier for the message this chunk belongs to.
    ///
    /// All chunks belonging to the same message share the same `messageId`.
    /// A change in `messageId` indicates a new message has started.
    #[must_use]
    pub fn message_id(mut self, message_id: impl IntoOption<MessageId>) -> Self {
        self.message_id = message_id.into_option();
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

/// Unique identifier for a message within a session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct MessageId(pub Arc<str>);

impl MessageId {
    /// Wraps a protocol string as a typed [`MessageId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

impl IntoOption<MessageId> for &str {
    fn into_option(self) -> Option<MessageId> {
        Some(MessageId::new(self))
    }
}

/// Available commands are ready or have changed
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AvailableCommandsUpdate {
    /// Commands the agent can execute
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub available_commands: Vec<AvailableCommand>,
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

impl AvailableCommandsUpdate {
    /// Builds [`AvailableCommandsUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(available_commands: Vec<AvailableCommand>) -> Self {
        Self {
            available_commands,
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

/// Information about a command.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AvailableCommand {
    /// Command name (e.g., `create_plan`, `research_codebase`).
    pub name: String,
    /// Human-readable description of what the command does.
    pub description: String,
    /// Input for the command if required
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub input: Option<AvailableCommandInput>,
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

impl AvailableCommand {
    /// Builds [`AvailableCommand`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input: None,
            meta: None,
        }
    }

    /// Input for the command if required
    #[must_use]
    pub fn input(mut self, input: impl IntoOption<AvailableCommandInput>) -> Self {
        self.input = input.into_option();
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

/// The input specification for a command.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase")]
#[non_exhaustive]
pub enum AvailableCommandInput {
    /// All text that was typed after the command name is provided as input.
    Unstructured(UnstructuredCommandInput),
}

/// All text that was typed after the command name is provided as input.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UnstructuredCommandInput {
    /// A hint to display when the input hasn't been provided yet
    pub hint: String,
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

impl UnstructuredCommandInput {
    /// Builds [`UnstructuredCommandInput`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            hint: hint.into(),
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

// Permission

/// Request for user permission to execute a tool call.
///
/// Sent when the agent needs authorization before performing a sensitive operation.
///
/// See protocol docs: [Requesting Permission](https://agentclientprotocol.com/protocol/tool-calls#requesting-permission)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[schemars(extend("x-side" = "client", "x-method" = SESSION_REQUEST_PERMISSION_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RequestPermissionRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// Details about the tool call requiring permission.
    pub tool_call: ToolCallUpdate,
    /// Available permission options for the user to choose from.
    pub options: Vec<PermissionOption>,
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

impl RequestPermissionRequest {
    /// Builds [`RequestPermissionRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        tool_call: ToolCallUpdate,
        options: Vec<PermissionOption>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            tool_call,
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

/// An option presented to the user when requesting permission.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PermissionOption {
    /// Unique identifier for this permission option.
    pub option_id: PermissionOptionId,
    /// Human-readable label to display to the user.
    pub name: String,
    /// Hint about the nature of this permission option.
    pub kind: PermissionOptionKind,
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

impl PermissionOption {
    /// Builds [`PermissionOption`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        option_id: impl Into<PermissionOptionId>,
        name: impl Into<String>,
        kind: PermissionOptionKind,
    ) -> Self {
        Self {
            option_id: option_id.into(),
            name: name.into(),
            kind,
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

/// Unique identifier for a permission option.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct PermissionOptionId(pub Arc<str>);

impl PermissionOptionId {
    /// Wraps a protocol string as a typed [`PermissionOptionId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// The type of permission option being presented to the user.
///
/// Helps clients choose appropriate icons and UI treatment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PermissionOptionKind {
    /// Allow this operation only this time.
    AllowOnce,
    /// Allow this operation and remember the choice.
    AllowAlways,
    /// Reject this operation only this time.
    RejectOnce,
    /// Reject this operation and remember the choice.
    RejectAlways,
}

/// Response to a permission request.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "client", "x-method" = SESSION_REQUEST_PERMISSION_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RequestPermissionResponse {
    /// The user's decision on the permission request.
    // This extra-level is unfortunately needed because the output must be an object
    pub outcome: RequestPermissionOutcome,
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

impl RequestPermissionResponse {
    /// Builds [`RequestPermissionResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(outcome: RequestPermissionOutcome) -> Self {
        Self {
            outcome,
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

/// The outcome of a permission request.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
#[schemars(extend("discriminator" = {"propertyName": "outcome"}))]
#[non_exhaustive]
pub enum RequestPermissionOutcome {
    /// The prompt turn was cancelled before the user responded.
    ///
    /// When a client sends a `session/cancel` notification to cancel an ongoing
    /// prompt turn, it MUST respond to all pending `session/request_permission`
    /// requests with this `Cancelled` outcome.
    ///
    /// See protocol docs: [Cancellation](https://agentclientprotocol.com/protocol/prompt-turn#cancellation)
    Cancelled,
    /// The user selected one of the provided options.
    #[serde(rename_all = "camelCase")]
    Selected(SelectedPermissionOutcome),
}

/// The user selected one of the provided options.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SelectedPermissionOutcome {
    /// The ID of the option the user selected.
    pub option_id: PermissionOptionId,
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

impl SelectedPermissionOutcome {
    /// Builds [`SelectedPermissionOutcome`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(option_id: impl Into<PermissionOptionId>) -> Self {
        Self {
            option_id: option_id.into(),
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

// Write text file

/// Request to write content to a text file.
///
/// Only available if the client supports the `fs.writeTextFile` capability.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "client", "x-method" = FS_WRITE_TEXT_FILE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WriteTextFileRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// Absolute path to the file to write.
    pub path: PathBuf,
    /// The text content to write to the file.
    pub content: String,
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

impl WriteTextFileRequest {
    /// Builds [`WriteTextFileRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        path: impl Into<PathBuf>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            path: path.into(),
            content: content.into(),
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

/// Response to `fs/write_text_file`
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = FS_WRITE_TEXT_FILE_METHOD_NAME))]
#[non_exhaustive]
pub struct WriteTextFileResponse {
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

impl WriteTextFileResponse {
    /// Builds [`WriteTextFileResponse`] with the required response fields set; optional fields start unset or empty.
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

// Read text file

/// Request to read content from a text file.
///
/// Only available if the client supports the `fs.readTextFile` capability.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "client", "x-method" = FS_READ_TEXT_FILE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReadTextFileRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// Absolute path to the file to read.
    pub path: PathBuf,
    /// Line number to start reading from (1-based).
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub line: Option<u32>,
    /// Maximum number of lines to read.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub limit: Option<u32>,
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

impl ReadTextFileRequest {
    /// Builds [`ReadTextFileRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, path: impl Into<PathBuf>) -> Self {
        Self {
            session_id: session_id.into(),
            path: path.into(),
            line: None,
            limit: None,
            meta: None,
        }
    }

    /// Line number to start reading from (1-based).
    #[must_use]
    pub fn line(mut self, line: impl IntoOption<u32>) -> Self {
        self.line = line.into_option();
        self
    }

    /// Maximum number of lines to read.
    #[must_use]
    pub fn limit(mut self, limit: impl IntoOption<u32>) -> Self {
        self.limit = limit.into_option();
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

/// Response containing the contents of a text file.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "client", "x-method" = FS_READ_TEXT_FILE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReadTextFileResponse {
    /// Content payload returned by this response.
    pub content: String,
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

impl ReadTextFileResponse {
    /// Builds [`ReadTextFileResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
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

// Terminals

/// Typed identifier used for terminal values on the wire.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &'static str)]
#[non_exhaustive]
pub struct TerminalId(pub Arc<str>);

impl TerminalId {
    /// Wraps a protocol string as a typed [`TerminalId`].
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

/// Request to create a new terminal and execute a command.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_CREATE_METHOD_NAME))]
#[non_exhaustive]
pub struct CreateTerminalRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// The command to execute.
    pub command: String,
    /// Array of command arguments.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Environment variables for the command.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVariable>,
    /// Working directory for the command. Must be an absolute path.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Maximum number of output bytes to retain.
    ///
    /// When the limit is exceeded, the Client truncates from the beginning of the output
    /// to stay within the limit.
    ///
    /// The Client MUST ensure truncation happens at a character boundary to maintain valid
    /// string output, even if this means the retained output is slightly less than the
    /// specified limit.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub output_byte_limit: Option<u64>,
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

impl CreateTerminalRequest {
    /// Builds [`CreateTerminalRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, command: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            command: command.into(),
            args: Vec::new(),
            env: Vec::new(),
            cwd: None,
            output_byte_limit: None,
            meta: None,
        }
    }

    /// Array of command arguments.
    #[must_use]
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Environment variables for the command.
    #[must_use]
    pub fn env(mut self, env: Vec<EnvVariable>) -> Self {
        self.env = env;
        self
    }

    /// Working directory for the command. Must be an absolute path.
    #[must_use]
    pub fn cwd(mut self, cwd: impl IntoOption<PathBuf>) -> Self {
        self.cwd = cwd.into_option();
        self
    }

    /// Maximum number of output bytes to retain.
    ///
    /// When the limit is exceeded, the Client truncates from the beginning of the output
    /// to stay within the limit.
    ///
    /// The Client MUST ensure truncation happens at a character boundary to maintain valid
    /// string output, even if this means the retained output is slightly less than the
    /// specified limit.
    #[must_use]
    pub fn output_byte_limit(mut self, output_byte_limit: impl IntoOption<u64>) -> Self {
        self.output_byte_limit = output_byte_limit.into_option();
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

/// Response containing the ID of the created terminal.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_CREATE_METHOD_NAME))]
#[non_exhaustive]
pub struct CreateTerminalResponse {
    /// The unique identifier for the created terminal.
    pub terminal_id: TerminalId,
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

impl CreateTerminalResponse {
    /// Builds [`CreateTerminalResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            terminal_id: terminal_id.into(),
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

/// Request to get the current output and status of a terminal.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_OUTPUT_METHOD_NAME))]
#[non_exhaustive]
pub struct TerminalOutputRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// The ID of the terminal to get output from.
    pub terminal_id: TerminalId,
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

impl TerminalOutputRequest {
    /// Builds [`TerminalOutputRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            session_id: session_id.into(),
            terminal_id: terminal_id.into(),
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

/// Response containing the terminal output and exit status.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_OUTPUT_METHOD_NAME))]
#[non_exhaustive]
pub struct TerminalOutputResponse {
    /// The terminal output captured so far.
    pub output: String,
    /// Whether the output was truncated due to byte limits.
    pub truncated: bool,
    /// Exit status if the command has completed.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub exit_status: Option<TerminalExitStatus>,
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

impl TerminalOutputResponse {
    /// Builds [`TerminalOutputResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(output: impl Into<String>, truncated: bool) -> Self {
        Self {
            output: output.into(),
            truncated,
            exit_status: None,
            meta: None,
        }
    }

    /// Exit status if the command has completed.
    #[must_use]
    pub fn exit_status(mut self, exit_status: impl IntoOption<TerminalExitStatus>) -> Self {
        self.exit_status = exit_status.into_option();
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

/// Request to release a terminal and free its resources.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_RELEASE_METHOD_NAME))]
#[non_exhaustive]
pub struct ReleaseTerminalRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// The ID of the terminal to release.
    pub terminal_id: TerminalId,
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

impl ReleaseTerminalRequest {
    /// Builds [`ReleaseTerminalRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            session_id: session_id.into(),
            terminal_id: terminal_id.into(),
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

/// Response to terminal/release method
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_RELEASE_METHOD_NAME))]
#[non_exhaustive]
pub struct ReleaseTerminalResponse {
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

impl ReleaseTerminalResponse {
    /// Builds [`ReleaseTerminalResponse`] with the required response fields set; optional fields start unset or empty.
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

/// Request to kill a terminal without releasing it.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_KILL_METHOD_NAME))]
#[non_exhaustive]
pub struct KillTerminalRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// The ID of the terminal to kill.
    pub terminal_id: TerminalId,
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

impl KillTerminalRequest {
    /// Builds [`KillTerminalRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            session_id: session_id.into(),
            terminal_id: terminal_id.into(),
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

/// Response to `terminal/kill` method
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_KILL_METHOD_NAME))]
#[non_exhaustive]
pub struct KillTerminalResponse {
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

impl KillTerminalResponse {
    /// Builds [`KillTerminalResponse`] with the required response fields set; optional fields start unset or empty.
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

/// Request to wait for a terminal command to exit.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_WAIT_FOR_EXIT_METHOD_NAME))]
#[non_exhaustive]
pub struct WaitForTerminalExitRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// The ID of the terminal to wait for.
    pub terminal_id: TerminalId,
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

impl WaitForTerminalExitRequest {
    /// Builds [`WaitForTerminalExitRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            session_id: session_id.into(),
            terminal_id: terminal_id.into(),
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

/// Response containing the exit status of a terminal command.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[schemars(extend("x-side" = "client", "x-method" = TERMINAL_WAIT_FOR_EXIT_METHOD_NAME))]
#[non_exhaustive]
pub struct WaitForTerminalExitResponse {
    /// The exit status of the terminal command.
    #[serde(flatten)]
    pub exit_status: TerminalExitStatus,
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

impl WaitForTerminalExitResponse {
    /// Builds [`WaitForTerminalExitResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(exit_status: TerminalExitStatus) -> Self {
        Self {
            exit_status,
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

/// Exit status of a terminal command.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalExitStatus {
    /// The process exit code (may be null if terminated by signal).
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub exit_code: Option<u32>,
    /// The signal that terminated the process (may be null if exited normally).
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub signal: Option<String>,
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

impl TerminalExitStatus {
    /// Builds [`TerminalExitStatus`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The process exit code (may be null if terminated by signal).
    #[must_use]
    pub fn exit_code(mut self, exit_code: impl IntoOption<u32>) -> Self {
        self.exit_code = exit_code.into_option();
        self
    }

    /// The signal that terminated the process (may be null if exited normally).
    #[must_use]
    pub fn signal(mut self, signal: impl IntoOption<String>) -> Self {
        self.signal = signal.into_option();
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

// Capabilities

/// Capabilities supported by the client.
///
/// Advertised during initialization to inform the agent about
/// available features and methods.
///
/// See protocol docs: [Client Capabilities](https://agentclientprotocol.com/protocol/initialization#client-capabilities)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ClientCapabilities {
    /// File system capabilities supported by the client.
    /// Determines which file operations the agent can request.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub fs: FileSystemCapabilities,
    /// Whether the Client support all `terminal/*` methods.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub terminal: bool,
    /// Session-related capabilities supported by the client.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise any
    /// session-related extensions.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub session: Option<ClientSessionCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Whether the client supports `plan_update` and `plan_removed` session updates.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the client can receive both update types.
    #[cfg(feature = "unstable_plan_operations")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub plan: Option<PlanCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Authentication capabilities supported by the client.
    /// Determines which authentication method types the agent may include
    /// in its `InitializeResponse`.
    #[cfg(feature = "unstable_auth_methods")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub auth: AuthCapabilities,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Elicitation capabilities supported by the client.
    /// Determines which elicitation modes the agent may use.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise
    /// elicitation support.
    #[cfg(feature = "unstable_elicitation")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub elicitation: Option<ElicitationCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// NES (Next Edit Suggestions) capabilities supported by the client.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise any
    /// NES suggestion-kind extensions.
    #[cfg(feature = "unstable_nes")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub nes: Option<ClientNesCapabilities>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// The position encodings supported by the client, in order of preference.
    #[cfg(feature = "unstable_nes")]
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub position_encodings: Vec<PositionEncodingKind>,

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

impl ClientCapabilities {
    /// Builds an empty [`ClientCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// File system capabilities supported by the client.
    /// Determines which file operations the agent can request.
    #[must_use]
    pub fn fs(mut self, fs: FileSystemCapabilities) -> Self {
        self.fs = fs;
        self
    }

    /// Whether the Client support all `terminal/*` methods.
    #[must_use]
    pub fn terminal(mut self, terminal: bool) -> Self {
        self.terminal = terminal;
        self
    }

    /// Session-related capabilities supported by the client.
    #[must_use]
    pub fn session(mut self, session: impl IntoOption<ClientSessionCapabilities>) -> Self {
        self.session = session.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Whether the client supports `plan_update` and `plan_removed` session updates.
    ///
    /// Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the client can receive both update types.
    #[cfg(feature = "unstable_plan_operations")]
    #[must_use]
    pub fn plan(mut self, plan: impl IntoOption<PlanCapabilities>) -> Self {
        self.plan = plan.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Authentication capabilities supported by the client.
    /// Determines which authentication method types the agent may include
    /// in its `InitializeResponse`.
    #[cfg(feature = "unstable_auth_methods")]
    #[must_use]
    pub fn auth(mut self, auth: AuthCapabilities) -> Self {
        self.auth = auth;
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Elicitation capabilities supported by the client.
    /// Determines which elicitation modes the agent may use.
    #[cfg(feature = "unstable_elicitation")]
    #[must_use]
    pub fn elicitation(mut self, elicitation: impl IntoOption<ElicitationCapabilities>) -> Self {
        self.elicitation = elicitation.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// NES (Next Edit Suggestions) capabilities supported by the client.
    #[cfg(feature = "unstable_nes")]
    #[must_use]
    pub fn nes(mut self, nes: impl IntoOption<ClientNesCapabilities>) -> Self {
        self.nes = nes.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// The position encodings supported by the client, in order of preference.
    #[cfg(feature = "unstable_nes")]
    #[must_use]
    pub fn position_encodings(mut self, position_encodings: Vec<PositionEncodingKind>) -> Self {
        self.position_encodings = position_encodings;
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

/// Session-related capabilities supported by the client.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ClientSessionCapabilities {
    /// Config option capabilities supported by the client.
    ///
    /// Omitted or `null` both mean the client does not advertise support for any
    /// config option extensions.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub config_options: Option<SessionConfigOptionsCapabilities>,
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

impl ClientSessionCapabilities {
    /// Builds an empty [`ClientSessionCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Config option capabilities supported by the client.
    ///
    /// Omitted or `null` both mean the client does not advertise support for any
    /// config option extensions.
    #[must_use]
    pub fn config_options(
        mut self,
        config_options: impl IntoOption<SessionConfigOptionsCapabilities>,
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

/// Session configuration option capabilities supported by the client.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SessionConfigOptionsCapabilities {
    /// Whether the client supports boolean session configuration options.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means agents may include `type: "boolean"` entries in
    /// `configOptions`, and the client may send `session/set_config_option`
    /// requests with `type: "boolean"` and a boolean `value`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub boolean: Option<BooleanConfigOptionCapabilities>,
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

impl SessionConfigOptionsCapabilities {
    /// Builds an empty [`SessionConfigOptionsCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the client supports boolean session configuration options.
    ///
    /// Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means agents may include `type: "boolean"` entries in
    /// `configOptions`, and the client may send `session/set_config_option`
    /// requests with `type: "boolean"` and a boolean `value`.
    #[must_use]
    pub fn boolean(mut self, boolean: impl IntoOption<BooleanConfigOptionCapabilities>) -> Self {
        self.boolean = boolean.into_option();
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

/// Capabilities for boolean session configuration options.
///
/// Supplying `{}` means the client supports boolean session configuration options.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct BooleanConfigOptionCapabilities {
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

impl BooleanConfigOptionCapabilities {
    /// Builds an empty [`BooleanConfigOptionCapabilities`]; use builder methods to advertise supported sub-capabilities.
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
/// Authentication capabilities supported by the client.
///
/// Advertised during initialization to inform the agent which authentication
/// method types the client can handle. This governs opt-in types that require
/// additional client-side support.
#[cfg(feature = "unstable_auth_methods")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthCapabilities {
    /// Whether the client supports `terminal` authentication methods.
    ///
    /// When `true`, the agent may include `terminal` entries in its authentication methods.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub terminal: bool,
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
impl AuthCapabilities {
    /// Builds an empty [`AuthCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the client supports `terminal` authentication methods.
    ///
    /// When `true`, the agent may include `AuthMethod::Terminal`
    /// entries in its authentication methods.
    #[must_use]
    pub fn terminal(mut self, terminal: bool) -> Self {
        self.terminal = terminal;
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

/// File system capabilities that a client may support.
///
/// See protocol docs: [FileSystem](https://agentclientprotocol.com/protocol/initialization#filesystem)
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FileSystemCapabilities {
    /// Whether the Client supports `fs/read_text_file` requests.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub read_text_file: bool,
    /// Whether the Client supports `fs/write_text_file` requests.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub write_text_file: bool,
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

impl FileSystemCapabilities {
    /// Builds an empty [`FileSystemCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the Client supports `fs/read_text_file` requests.
    #[must_use]
    pub fn read_text_file(mut self, read_text_file: bool) -> Self {
        self.read_text_file = read_text_file;
        self
    }

    /// Whether the Client supports `fs/write_text_file` requests.
    #[must_use]
    pub fn write_text_file(mut self, write_text_file: bool) -> Self {
        self.write_text_file = write_text_file;
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

/// Names of all methods that clients handle.
///
/// Provides a centralized definition of method names used in the protocol.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct ClientMethodNames {
    /// Method for requesting permission from the user.
    pub session_request_permission: &'static str,
    /// Notification for session updates.
    pub session_update: &'static str,
    /// Method for writing text files.
    pub fs_write_text_file: &'static str,
    /// Method for reading text files.
    pub fs_read_text_file: &'static str,
    /// Method for creating new terminals.
    pub terminal_create: &'static str,
    /// Method for getting terminals output.
    pub terminal_output: &'static str,
    /// Method for releasing a terminal.
    pub terminal_release: &'static str,
    /// Method for waiting for a terminal to finish.
    pub terminal_wait_for_exit: &'static str,
    /// Method for killing a terminal.
    pub terminal_kill: &'static str,
    /// Method for opening an MCP-over-ACP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    pub mcp_connect: &'static str,
    /// Method for exchanging MCP-over-ACP messages.
    #[cfg(feature = "unstable_mcp_over_acp")]
    pub mcp_message: &'static str,
    /// Method for closing an MCP-over-ACP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    pub mcp_disconnect: &'static str,
    /// Method for elicitation.
    #[cfg(feature = "unstable_elicitation")]
    pub elicitation_create: &'static str,
    /// Notification for elicitation completion.
    #[cfg(feature = "unstable_elicitation")]
    pub elicitation_complete: &'static str,
}

/// Constant containing all client method names.
pub const CLIENT_METHOD_NAMES: ClientMethodNames = ClientMethodNames {
    session_update: SESSION_UPDATE_NOTIFICATION,
    session_request_permission: SESSION_REQUEST_PERMISSION_METHOD_NAME,
    fs_write_text_file: FS_WRITE_TEXT_FILE_METHOD_NAME,
    fs_read_text_file: FS_READ_TEXT_FILE_METHOD_NAME,
    terminal_create: TERMINAL_CREATE_METHOD_NAME,
    terminal_output: TERMINAL_OUTPUT_METHOD_NAME,
    terminal_release: TERMINAL_RELEASE_METHOD_NAME,
    terminal_wait_for_exit: TERMINAL_WAIT_FOR_EXIT_METHOD_NAME,
    terminal_kill: TERMINAL_KILL_METHOD_NAME,
    #[cfg(feature = "unstable_mcp_over_acp")]
    mcp_connect: MCP_CONNECT_METHOD_NAME,
    #[cfg(feature = "unstable_mcp_over_acp")]
    mcp_message: MCP_MESSAGE_METHOD_NAME,
    #[cfg(feature = "unstable_mcp_over_acp")]
    mcp_disconnect: MCP_DISCONNECT_METHOD_NAME,
    #[cfg(feature = "unstable_elicitation")]
    elicitation_create: ELICITATION_CREATE_METHOD_NAME,
    #[cfg(feature = "unstable_elicitation")]
    elicitation_complete: ELICITATION_COMPLETE_NOTIFICATION,
};

/// Notification name for session updates.
pub(crate) const SESSION_UPDATE_NOTIFICATION: &str = "session/update";
/// Method name for requesting user permission.
pub(crate) const SESSION_REQUEST_PERMISSION_METHOD_NAME: &str = "session/request_permission";
/// Method name for writing text files.
pub(crate) const FS_WRITE_TEXT_FILE_METHOD_NAME: &str = "fs/write_text_file";
/// Method name for reading text files.
pub(crate) const FS_READ_TEXT_FILE_METHOD_NAME: &str = "fs/read_text_file";
/// Method name for creating a new terminal.
pub(crate) const TERMINAL_CREATE_METHOD_NAME: &str = "terminal/create";
/// Method for getting terminals output.
pub(crate) const TERMINAL_OUTPUT_METHOD_NAME: &str = "terminal/output";
/// Method for releasing a terminal.
pub(crate) const TERMINAL_RELEASE_METHOD_NAME: &str = "terminal/release";
/// Method for waiting for a terminal to finish.
pub(crate) const TERMINAL_WAIT_FOR_EXIT_METHOD_NAME: &str = "terminal/wait_for_exit";
/// Method for killing a terminal.
pub(crate) const TERMINAL_KILL_METHOD_NAME: &str = "terminal/kill";
/// Method name for elicitation.
#[cfg(feature = "unstable_elicitation")]
pub(crate) const ELICITATION_CREATE_METHOD_NAME: &str = "elicitation/create";
/// Notification name for elicitation completion.
#[cfg(feature = "unstable_elicitation")]
pub(crate) const ELICITATION_COMPLETE_NOTIFICATION: &str = "elicitation/complete";

/// All possible requests that an agent can send to a client.
///
/// This enum is used internally for routing RPC requests. You typically won't need
/// to use this directly.
///
/// This enum encompasses all method calls from agent to client.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum AgentRequest {
    /// Writes content to a text file in the client's file system.
    ///
    /// Only available if the client advertises the `fs.writeTextFile` capability.
    /// Allows the agent to create or modify files within the client's environment.
    ///
    /// See protocol docs: [Client](https://agentclientprotocol.com/protocol/overview#client)
    WriteTextFileRequest(WriteTextFileRequest),
    /// Reads content from a text file in the client's file system.
    ///
    /// Only available if the client advertises the `fs.readTextFile` capability.
    /// Allows the agent to access file contents within the client's environment.
    ///
    /// See protocol docs: [Client](https://agentclientprotocol.com/protocol/overview#client)
    ReadTextFileRequest(ReadTextFileRequest),
    /// Requests permission from the user for a tool call operation.
    ///
    /// Called by the agent when it needs user authorization before executing
    /// a potentially sensitive operation. The client should present the options
    /// to the user and return their decision.
    ///
    /// If the client cancels the prompt turn via `session/cancel`, it MUST
    /// respond to this request with `RequestPermissionOutcome::Cancelled`.
    ///
    /// See protocol docs: [Requesting Permission](https://agentclientprotocol.com/protocol/tool-calls#requesting-permission)
    RequestPermissionRequest(RequestPermissionRequest),
    /// Executes a command in a new terminal
    ///
    /// Only available if the `terminal` Client capability is set to `true`.
    ///
    /// Returns a `TerminalId` that can be used with other terminal methods
    /// to get the current output, wait for exit, and kill the command.
    ///
    /// The `TerminalId` can also be used to embed the terminal in a tool call
    /// by using the `ToolCallContent::Terminal` variant.
    ///
    /// The Agent is responsible for releasing the terminal by using the `terminal/release`
    /// method.
    ///
    /// See protocol docs: [Terminals](https://agentclientprotocol.com/protocol/terminals)
    CreateTerminalRequest(CreateTerminalRequest),
    /// Gets the terminal output and exit status
    ///
    /// Returns the current content in the terminal without waiting for the command to exit.
    /// If the command has already exited, the exit status is included.
    ///
    /// See protocol docs: [Terminals](https://agentclientprotocol.com/protocol/terminals)
    TerminalOutputRequest(TerminalOutputRequest),
    /// Releases a terminal
    ///
    /// The command is killed if it hasn't exited yet. Use `terminal/wait_for_exit`
    /// to wait for the command to exit before releasing the terminal.
    ///
    /// After release, the `TerminalId` can no longer be used with other `terminal/*` methods,
    /// but tool calls that already contain it, continue to display its output.
    ///
    /// The `terminal/kill` method can be used to terminate the command without releasing
    /// the terminal, allowing the Agent to call `terminal/output` and other methods.
    ///
    /// See protocol docs: [Terminals](https://agentclientprotocol.com/protocol/terminals)
    ReleaseTerminalRequest(ReleaseTerminalRequest),
    /// Waits for the terminal command to exit and return its exit status
    ///
    /// See protocol docs: [Terminals](https://agentclientprotocol.com/protocol/terminals)
    WaitForTerminalExitRequest(WaitForTerminalExitRequest),
    /// Kills the terminal command without releasing the terminal
    ///
    /// While `terminal/release` will also kill the command, this method will keep
    /// the `TerminalId` valid so it can be used with other methods.
    ///
    /// This method can be helpful when implementing command timeouts which terminate
    /// the command as soon as elapsed, and then get the final output so it can be sent
    /// to the model.
    ///
    /// Note: Call `terminal/release` when `TerminalId` is no longer needed.
    ///
    /// See protocol docs: [Terminals](https://agentclientprotocol.com/protocol/terminals)
    KillTerminalRequest(KillTerminalRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Requests structured user input via a form or URL.
    #[cfg(feature = "unstable_elicitation")]
    CreateElicitationRequest(CreateElicitationRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Opens an MCP-over-ACP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    ConnectMcpRequest(ConnectMcpRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Exchanges an MCP-over-ACP message.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpRequest(MessageMcpRequest),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Closes an MCP-over-ACP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    DisconnectMcpRequest(DisconnectMcpRequest),
    /// Handles extension method requests from the agent.
    ///
    /// Allows the Agent to send an arbitrary request that is not part of the ACP spec.
    /// Extension methods provide a way to add custom functionality while maintaining
    /// protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtMethodRequest(ExtRequest),
}

impl AgentRequest {
    /// Returns the corresponding method name of the request.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::WriteTextFileRequest(_) => CLIENT_METHOD_NAMES.fs_write_text_file,
            Self::ReadTextFileRequest(_) => CLIENT_METHOD_NAMES.fs_read_text_file,
            Self::RequestPermissionRequest(_) => CLIENT_METHOD_NAMES.session_request_permission,
            Self::CreateTerminalRequest(_) => CLIENT_METHOD_NAMES.terminal_create,
            Self::TerminalOutputRequest(_) => CLIENT_METHOD_NAMES.terminal_output,
            Self::ReleaseTerminalRequest(_) => CLIENT_METHOD_NAMES.terminal_release,
            Self::WaitForTerminalExitRequest(_) => CLIENT_METHOD_NAMES.terminal_wait_for_exit,
            Self::KillTerminalRequest(_) => CLIENT_METHOD_NAMES.terminal_kill,
            #[cfg(feature = "unstable_elicitation")]
            Self::CreateElicitationRequest(_) => CLIENT_METHOD_NAMES.elicitation_create,
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::ConnectMcpRequest(_) => CLIENT_METHOD_NAMES.mcp_connect,
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::MessageMcpRequest(_) => CLIENT_METHOD_NAMES.mcp_message,
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::DisconnectMcpRequest(_) => CLIENT_METHOD_NAMES.mcp_disconnect,
            Self::ExtMethodRequest(ext_request) => &ext_request.method,
        }
    }
}

/// All possible responses that a client can send to an agent.
///
/// This enum is used internally for routing RPC responses. You typically won't need
/// to use this directly - the responses are handled automatically by the connection.
///
/// These are responses to the corresponding `AgentRequest` variants.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
#[non_exhaustive]
pub enum ClientResponse {
    /// Successful result returned for a `fs/write_text_file` request.
    WriteTextFileResponse(#[serde(default)] WriteTextFileResponse),
    /// Successful result returned for a `fs/read_text_file` request.
    ReadTextFileResponse(ReadTextFileResponse),
    /// Successful result returned for a `session/request_permission` request.
    RequestPermissionResponse(RequestPermissionResponse),
    /// Successful result returned for a `terminal/create` request.
    CreateTerminalResponse(CreateTerminalResponse),
    /// Successful result returned for a `terminal/output` request.
    TerminalOutputResponse(TerminalOutputResponse),
    /// Successful result returned for a `terminal/release` request.
    ReleaseTerminalResponse(#[serde(default)] ReleaseTerminalResponse),
    /// Successful result returned for a `terminal/wait_for_exit` request.
    WaitForTerminalExitResponse(WaitForTerminalExitResponse),
    /// Successful result returned for a `terminal/kill` request.
    KillTerminalResponse(#[serde(default)] KillTerminalResponse),
    /// Successful result returned for a `elicitation/create` request.
    #[cfg(feature = "unstable_elicitation")]
    CreateElicitationResponse(CreateElicitationResponse),
    /// Successful result returned for a `mcp/connect` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    ConnectMcpResponse(ConnectMcpResponse),
    /// Successful result returned for a `mcp/disconnect` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    DisconnectMcpResponse(#[serde(default)] DisconnectMcpResponse),
    /// Successful result returned by an MCP-over-ACP `mcp/message` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpResponse(MessageMcpResponse),
    /// Successful result returned by an extension method outside the core ACP method set.
    ExtMethodResponse(ExtResponse),
}

/// All possible notifications that an agent can send to a client.
///
/// This enum is used internally for routing RPC notifications. You typically won't need
/// to use this directly.
///
/// Notifications do not expect a response.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[expect(clippy::large_enum_variant)]
#[schemars(inline)]
#[non_exhaustive]
pub enum AgentNotification {
    /// Handles session update notifications from the agent.
    ///
    /// This is a notification endpoint (no response expected) that receives
    /// real-time updates about session progress, including message chunks,
    /// tool calls, and execution plans.
    ///
    /// Note: Clients SHOULD continue accepting tool call updates even after
    /// sending a `session/cancel` notification, as the agent may send final
    /// updates before responding with the cancelled stop reason.
    ///
    /// See protocol docs: [Agent Reports Output](https://agentclientprotocol.com/protocol/prompt-turn#3-agent-reports-output)
    SessionNotification(SessionNotification),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Notification that a URL-based elicitation has completed.
    #[cfg(feature = "unstable_elicitation")]
    CompleteElicitationNotification(CompleteElicitationNotification),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Receives an MCP-over-ACP notification.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpNotification(MessageMcpNotification),
    /// Handles extension notifications from the agent.
    ///
    /// Allows the Agent to send an arbitrary notification that is not part of the ACP spec.
    /// Extension notifications provide a way to send one-way messages for custom functionality
    /// while maintaining protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtNotification(ExtNotification),
}

impl AgentNotification {
    /// Returns the corresponding method name of the notification.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::SessionNotification(_) => CLIENT_METHOD_NAMES.session_update,
            #[cfg(feature = "unstable_elicitation")]
            Self::CompleteElicitationNotification(_) => CLIENT_METHOD_NAMES.elicitation_complete,
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::MessageMcpNotification(_) => CLIENT_METHOD_NAMES.mcp_message,
            Self::ExtNotification(ext_notification) => &ext_notification.method,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_capabilities_default_on_malformed_values() {
        use serde_json::json;

        let capabilities: ClientCapabilities = serde_json::from_value(json!({
            "fs": {
                "readTextFile": "yes",
                "writeTextFile": true
            },
            "terminal": {}
        }))
        .unwrap();

        assert!(!capabilities.fs.read_text_file);
        assert!(capabilities.fs.write_text_file);
        assert!(!capabilities.terminal);

        let capabilities: ClientCapabilities = serde_json::from_value(json!({
            "fs": false
        }))
        .unwrap();
        assert_eq!(capabilities.fs, FileSystemCapabilities::default());

        #[cfg(feature = "unstable_auth_methods")]
        {
            let capabilities: ClientCapabilities = serde_json::from_value(json!({
                "auth": false
            }))
            .unwrap();
            assert_eq!(capabilities.auth, AuthCapabilities::default());

            let capabilities: AuthCapabilities = serde_json::from_value(json!({
                "terminal": {}
            }))
            .unwrap();
            assert!(!capabilities.terminal);
        }
    }

    #[test]
    fn test_serialization_behavior() {
        use serde_json::json;

        assert_eq!(
            serde_json::from_value::<SessionInfoUpdate>(json!({})).unwrap(),
            SessionInfoUpdate {
                title: MaybeUndefined::Undefined,
                updated_at: MaybeUndefined::Undefined,
                meta: None
            }
        );
        assert_eq!(
            serde_json::from_value::<SessionInfoUpdate>(json!({"title": null, "updatedAt": null}))
                .unwrap(),
            SessionInfoUpdate {
                title: MaybeUndefined::Null,
                updated_at: MaybeUndefined::Null,
                meta: None
            }
        );
        assert_eq!(
            serde_json::from_value::<SessionInfoUpdate>(
                json!({"title": "title", "updatedAt": "timestamp"})
            )
            .unwrap(),
            SessionInfoUpdate {
                title: MaybeUndefined::Value("title".to_string()),
                updated_at: MaybeUndefined::Value("timestamp".to_string()),
                meta: None
            }
        );

        assert_eq!(
            serde_json::to_value(SessionInfoUpdate::new()).unwrap(),
            json!({})
        );
        assert_eq!(
            serde_json::to_value(SessionInfoUpdate::new().title("title")).unwrap(),
            json!({"title": "title"})
        );
        assert_eq!(
            serde_json::to_value(SessionInfoUpdate::new().title(None)).unwrap(),
            json!({"title": null})
        );
        assert_eq!(
            serde_json::to_value(
                SessionInfoUpdate::new()
                    .title("title")
                    .title(MaybeUndefined::Undefined)
            )
            .unwrap(),
            json!({})
        );
    }

    #[test]
    fn test_content_chunk_message_id_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::AgentMessageChunk(ContentChunk::new(
                ContentBlock::Text(crate::v1::TextContent::new("Hello"))
            )))
            .unwrap(),
            json!({
                "sessionUpdate": "agent_message_chunk",
                "content": {
                    "type": "text",
                    "text": "Hello"
                }
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::AgentMessageChunk(
                ContentChunk::new(ContentBlock::Text(crate::v1::TextContent::new("Hello")))
                    .message_id("msg_agent_c42b9")
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "agent_message_chunk",
                "messageId": "msg_agent_c42b9",
                "content": {
                    "type": "text",
                    "text": "Hello"
                }
            })
        );

        let SessionUpdate::AgentMessageChunk(chunk) = serde_json::from_value(json!({
            "sessionUpdate": "agent_message_chunk",
            "messageId": null,
            "content": {
                "type": "text",
                "text": "Hello"
            }
        }))
        .unwrap() else {
            panic!("expected agent message chunk");
        };

        assert_eq!(chunk.message_id, None);
    }

    #[test]
    fn test_usage_update_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::UsageUpdate(UsageUpdate::new(
                53_000, 200_000
            )))
            .unwrap(),
            json!({
                "sessionUpdate": "usage_update",
                "used": 53000,
                "size": 200_000
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::UsageUpdate(
                UsageUpdate::new(53_000, 200_000).cost(Cost::new(0.045, "USD"))
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "usage_update",
                "used": 53000,
                "size": 200_000,
                "cost": {
                    "amount": 0.045,
                    "currency": "USD"
                }
            })
        );

        let SessionUpdate::UsageUpdate(update) = serde_json::from_value(json!({
            "sessionUpdate": "usage_update",
            "used": 53000,
            "size": 200_000,
            "cost": null
        }))
        .unwrap() else {
            panic!("expected usage update");
        };

        assert_eq!(update.cost, None);
    }

    #[cfg(feature = "unstable_nes")]
    #[test]
    fn test_client_capabilities_position_encodings_serialization() {
        use serde_json::json;

        let capabilities = ClientCapabilities::new().position_encodings(vec![
            PositionEncodingKind::Utf32,
            PositionEncodingKind::Utf16,
        ]);
        let json = serde_json::to_value(&capabilities).unwrap();

        assert_eq!(json["positionEncodings"], json!(["utf-32", "utf-16"]));
    }

    #[test]
    fn test_client_capabilities_boolean_config_options_serialization() {
        use serde_json::json;

        let capabilities = ClientCapabilities::new().session(
            ClientSessionCapabilities::new().config_options(
                SessionConfigOptionsCapabilities::new()
                    .boolean(BooleanConfigOptionCapabilities::new()),
            ),
        );
        let json = serde_json::to_value(&capabilities).unwrap();

        assert_eq!(json["session"]["configOptions"]["boolean"], json!({}));

        let omitted: ClientCapabilities = serde_json::from_value(json!({})).unwrap();
        assert!(omitted.session.is_none());

        let null_session: ClientCapabilities = serde_json::from_value(json!({
            "session": null
        }))
        .unwrap();
        assert!(null_session.session.is_none());

        let null_config_options: ClientCapabilities = serde_json::from_value(json!({
            "session": {
                "configOptions": null
            }
        }))
        .unwrap();
        assert!(
            null_config_options
                .session
                .and_then(|session| session.config_options)
                .is_none()
        );

        let null_boolean: ClientCapabilities = serde_json::from_value(json!({
            "session": {
                "configOptions": {
                    "boolean": null
                }
            }
        }))
        .unwrap();
        assert!(
            null_boolean
                .session
                .and_then(|session| session.config_options)
                .and_then(|config_options| config_options.boolean)
                .is_none()
        );
    }

    #[cfg(feature = "unstable_plan_operations")]
    #[test]
    fn test_plan_operations_serialization() {
        use serde_json::json;

        use crate::v1::{PlanEntry, PlanEntryPriority, PlanEntryStatus, PlanUpdateContent};

        let plan_update = SessionUpdate::PlanUpdate(PlanUpdate::new(PlanUpdateContent::items(
            "plan-1",
            vec![PlanEntry::new(
                "Step 1",
                PlanEntryPriority::High,
                PlanEntryStatus::Pending,
            )],
        )));

        assert_eq!(
            serde_json::to_value(plan_update).unwrap(),
            json!({
                "sessionUpdate": "plan_update",
                "plan": {
                    "type": "items",
                    "planId": "plan-1",
                    "entries": [
                        {
                            "content": "Step 1",
                            "priority": "high",
                            "status": "pending"
                        }
                    ]
                }
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::PlanRemoved(PlanRemoved::new("plan-1"))).unwrap(),
            json!({
                "sessionUpdate": "plan_removed",
                "planId": "plan-1"
            })
        );

        let capabilities = ClientCapabilities::new().plan(PlanCapabilities::new());
        let json = serde_json::to_value(&capabilities).unwrap();
        assert_eq!(json["plan"], json!({}));

        assert_eq!(
            serde_json::from_value::<ClientCapabilities>(json!({ "plan": null }))
                .unwrap()
                .plan,
            None
        );
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn test_agent_mcp_request_method_names() {
        use serde_json::json;

        let params: serde_json::Map<String, serde_json::Value> =
            [("cursor".to_string(), json!("abc"))].into_iter().collect();

        assert_eq!(CLIENT_METHOD_NAMES.mcp_connect, "mcp/connect");
        assert_eq!(CLIENT_METHOD_NAMES.mcp_message, "mcp/message");
        assert_eq!(CLIENT_METHOD_NAMES.mcp_disconnect, "mcp/disconnect");

        assert_eq!(
            AgentRequest::ConnectMcpRequest(ConnectMcpRequest::new("server-1")).method(),
            "mcp/connect"
        );
        assert_eq!(
            AgentRequest::MessageMcpRequest(MessageMcpRequest::new("conn-1", "tools/list"))
                .method(),
            "mcp/message"
        );
        assert_eq!(
            AgentRequest::DisconnectMcpRequest(DisconnectMcpRequest::new("conn-1")).method(),
            "mcp/disconnect"
        );
        assert_eq!(
            AgentNotification::MessageMcpNotification(MessageMcpNotification::new(
                "conn-1",
                "notifications/progress"
            ))
            .method(),
            "mcp/message"
        );

        assert_eq!(
            serde_json::to_value(ConnectMcpRequest::new("server-1")).unwrap(),
            json!({ "serverId": "server-1" })
        );
        assert_eq!(
            serde_json::to_value(ConnectMcpResponse::new("conn-1")).unwrap(),
            json!({ "connectionId": "conn-1" })
        );
        assert_eq!(
            serde_json::to_value(MessageMcpRequest::new("conn-1", "tools/list").params(params))
                .unwrap(),
            json!({
                "connectionId": "conn-1",
                "method": "tools/list",
                "params": { "cursor": "abc" }
            })
        );
        assert_eq!(
            serde_json::to_value(DisconnectMcpRequest::new("conn-1")).unwrap(),
            json!({ "connectionId": "conn-1" })
        );
        assert_eq!(
            serde_json::to_value(MessageMcpNotification::new(
                "conn-1",
                "notifications/progress"
            ))
            .unwrap(),
            json!({
                "connectionId": "conn-1",
                "method": "notifications/progress"
            })
        );

        let request_with_null_params: MessageMcpRequest = serde_json::from_value(json!({
            "connectionId": "conn-1",
            "method": "tools/list",
            "params": null
        }))
        .unwrap();
        assert_eq!(request_with_null_params.params, None);
    }

    #[test]
    fn request_permission_request_rejects_malformed_options() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<RequestPermissionRequest>(json!({
                "sessionId": "sess-1",
                "toolCall": {"toolCallId": "tc-1"},
                "options": "not-an-array"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionRequest>(json!({
                "sessionId": "sess-1",
                "toolCall": {"toolCallId": "tc-1"},
                "options": [{"optionId": "allow"}]
            }))
            .is_err()
        );
    }
}
