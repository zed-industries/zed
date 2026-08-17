//! Methods and notifications the client handles/receives.
//!
//! This module defines the Client trait and all associated types for implementing
//! a client that interacts with AI coding agents via the Agent Client Protocol (ACP).

use std::{collections::BTreeMap, sync::Arc};

use derive_more::{Display, From};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

#[cfg(feature = "unstable_plan_operations")]
use super::PlanRemoved;
#[cfg(feature = "unstable_end_turn_token_usage")]
use super::Usage;
use super::{
    AbsolutePath, ContentBlock, ExtNotification, ExtRequest, ExtResponse, Meta, PlanUpdate,
    SessionConfigOption, SessionId, StopReason, TerminalId, TerminalOutputChunk, TerminalUpdate,
    ToolCallContentChunk, ToolCallId, ToolCallUpdate,
};
#[cfg(feature = "unstable_elicitation")]
use super::{
    CompleteElicitationNotification, CreateElicitationRequest, CreateElicitationResponse,
    ElicitationCapabilities,
};
use crate::{IntoMaybeUndefined, IntoOption, MaybeUndefined, SkipListener};

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
/// Agents can send session updates at any point while the session exists.
///
/// See protocol docs: [Agent Reports Output](https://agentclientprotocol.com/protocol/prompt-lifecycle#3-agent-reports-output)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[schemars(extend("x-side" = "client", "x-method" = SESSION_UPDATE_NOTIFICATION))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateSessionNotification {
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

impl UpdateSessionNotification {
    /// Builds [`UpdateSessionNotification`] with the required notification fields set; optional fields start unset or empty.
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

/// Different types of updates that can be sent while a session exists.
///
/// These updates report messages, progress, and other session activity.
///
/// See protocol docs: [Agent Reports Output](https://agentclientprotocol.com/protocol/prompt-lifecycle#3-agent-reports-output)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "sessionUpdate", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionUpdate {
    /// A chunk of the user's message being streamed.
    UserMessageChunk(ContentChunk),
    /// A user message has been created or updated.
    ///
    /// Agents can send this when they accept or replay a user message. When a
    /// client receives another `user_message` update with the same `messageId`,
    /// fields in the new update patch the previous fields for that message.
    UserMessage(UserMessage),
    /// A chunk of the agent's response being streamed.
    AgentMessageChunk(ContentChunk),
    /// An agent message has been created or updated.
    ///
    /// Agents can send this in addition to streamed chunks. When a client
    /// receives another `agent_message` update with the same `messageId`,
    /// fields in the new update patch the previous fields for that message.
    AgentMessage(AgentMessage),
    /// A chunk of the agent's internal reasoning being streamed.
    AgentThoughtChunk(ContentChunk),
    /// An agent thought or reasoning message has been created or updated.
    ///
    /// Agents can send this in addition to streamed chunks. When a client
    /// receives another `agent_thought` update with the same `messageId`,
    /// fields in the new update patch the previous fields for that message.
    AgentThought(AgentThought),
    /// The state of the agent's foreground work has changed.
    StateUpdate(StateUpdate),
    /// A chunk of tool-call content being streamed.
    ToolCallContentChunk(ToolCallContentChunk),
    /// A tool call has been created or updated.
    ToolCallUpdate(ToolCallUpdate),
    /// An agent-owned terminal has been created or updated.
    TerminalUpdate(TerminalUpdate),
    /// A chunk of bytes appended to an agent-owned terminal's output.
    TerminalOutputChunk(TerminalOutputChunk),
    /// A content update for a plan identified by ID.
    /// See protocol docs: [Agent Plan](https://agentclientprotocol.com/protocol/agent-plan)
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
    /// Session configuration options have been updated.
    ConfigOptionUpdate(ConfigOptionUpdate),
    /// Session metadata has been updated (title, timestamps, custom metadata)
    SessionInfoUpdate(SessionInfoUpdate),
    /// Context window and cost update for the session.
    UsageUpdate(UsageUpdate),
    /// Custom or future session update.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this update type should preserve the
    /// raw payload when storing, replaying, proxying, or forwarding session
    /// history, and otherwise ignore it or display it generically.
    #[serde(untagged)]
    Other(OtherSessionUpdate),
}

/// Custom or future session update payload.
///
/// This preserves the unknown `sessionUpdate` discriminator and the rest of the
/// update object for clients that store, replay, proxy, or forward session
/// history.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq)]
#[schemars(inline)]
#[schemars(transform = other_session_update_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherSessionUpdate {
    /// Custom or future session update type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "sessionUpdate")]
    pub session_update: String,
    /// Additional fields from the unknown update payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherSessionUpdate {
    /// Builds [`OtherSessionUpdate`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        session_update: impl Into<String>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("sessionUpdate");
        Self {
            session_update: session_update.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherSessionUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let session_update = fields
            .remove("sessionUpdate")
            .ok_or_else(|| serde::de::Error::missing_field("sessionUpdate"))?;
        let serde_json::Value::String(session_update) = session_update else {
            return Err(serde::de::Error::custom("`sessionUpdate` must be a string"));
        };

        if is_known_session_update(&session_update) {
            return Err(serde::de::Error::custom(format!(
                "known session update `{session_update}` did not match its schema"
            )));
        }

        Ok(Self {
            session_update,
            fields,
        })
    }
}

fn is_known_session_update(session_update: &str) -> bool {
    #[cfg(feature = "unstable_plan_operations")]
    if session_update == "plan_removed" {
        return true;
    }
    matches!(
        session_update,
        "user_message_chunk"
            | "user_message"
            | "agent_message_chunk"
            | "agent_message"
            | "agent_thought_chunk"
            | "agent_thought"
            | "state_update"
            | "tool_call_content_chunk"
            | "tool_call_update"
            | "terminal_update"
            | "terminal_output_chunk"
            | "plan_update"
            | "available_commands_update"
            | "config_option_update"
            | "session_info_update"
            | "usage_update"
    )
}

fn other_session_update_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "sessionUpdate",
        &[
            "user_message_chunk",
            "user_message",
            "agent_message_chunk",
            "agent_message",
            "agent_thought_chunk",
            "agent_thought",
            "state_update",
            "tool_call_content_chunk",
            "tool_call_update",
            "terminal_update",
            "terminal_output_chunk",
            "plan_update",
            "available_commands_update",
            "config_option_update",
            "session_info_update",
            #[cfg(feature = "unstable_plan_operations")]
            "plan_removed",
            "usage_update",
        ],
    );
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
///
/// Omitted fields leave the existing session info unchanged. `null` clears the
/// corresponding value.
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
    /// RFC 3339 timestamp of last activity. Set to null to clear.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "format" = "date-time"))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub updated_at: MaybeUndefined<String>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Omitted means no metadata update; `null` is an
    /// explicit clear signal. Implementations MUST NOT make assumptions about values at these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<_>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "MaybeUndefined::is_undefined"
    )]
    pub meta: MaybeUndefined<Meta>,
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

    /// RFC 3339 timestamp of last activity. Set to null to clear.
    #[must_use]
    pub fn updated_at(mut self, updated_at: impl IntoMaybeUndefined<String>) -> Self {
        self.updated_at = updated_at.into_maybe_undefined();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Omitted means no metadata update; `null` is an
    /// explicit clear signal. Implementations MUST NOT make assumptions about values at these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoMaybeUndefined<Meta>) -> Self {
        self.meta = meta.into_maybe_undefined();
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

/// The state of the agent's foreground work has changed.
///
/// Background activity can continue and emit other `session/update` notifications
/// while `idle`. Those notifications do not change this state.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "state", rename_all = "snake_case")]
#[non_exhaustive]
pub enum StateUpdate {
    /// Foreground work is in progress.
    Running(RunningStateUpdate),
    /// The agent is ready to process a new prompt.
    Idle(IdleStateUpdate),
    /// Foreground work is blocked on user action.
    RequiresAction(RequiresActionStateUpdate),
    /// Custom or future session state.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(OtherStateUpdate),
}

/// Foreground work is in progress.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RunningStateUpdate {
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

impl RunningStateUpdate {
    /// Builds [`RunningStateUpdate`] with the required fields set; optional fields start unset or empty.
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

/// The agent is ready to process a new prompt.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IdleStateUpdate {
    /// Indicates why foreground work stopped.
    ///
    /// Optional. Omitted or `null` both mean the agent is not reporting a stop reason.
    /// Agents SHOULD include this when the idle transition ends foreground work.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub stop_reason: Option<StopReason>,
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Token usage for completed foreground work.
    ///
    /// Optional. Omitted or `null` both mean the agent is not reporting token
    /// usage for this state update.
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

impl IdleStateUpdate {
    /// Builds [`IdleStateUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Indicates why foreground work stopped.
    #[must_use]
    pub fn stop_reason(mut self, stop_reason: impl IntoOption<StopReason>) -> Self {
        self.stop_reason = stop_reason.into_option();
        self
    }

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Token usage for completed foreground work.
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

/// Foreground work is blocked on user action.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RequiresActionStateUpdate {
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

impl RequiresActionStateUpdate {
    /// Builds [`RequiresActionStateUpdate`] with the required fields set; optional fields start unset or empty.
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

/// Custom or future session state payload.
///
/// This preserves the unknown `state` discriminator and the rest of the state
/// object for clients that store, replay, proxy, or forward session history.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq)]
#[schemars(inline)]
#[schemars(transform = other_state_update_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherStateUpdate {
    /// Custom or future session state.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "state")]
    pub state: String,
    /// Additional fields from the unknown state payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherStateUpdate {
    /// Builds [`OtherStateUpdate`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(state: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("state");
        Self {
            state: state.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherStateUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let state = fields
            .remove("state")
            .ok_or_else(|| serde::de::Error::missing_field("state"))?;
        let serde_json::Value::String(state) = state else {
            return Err(serde::de::Error::custom("`state` must be a string"));
        };

        if is_known_state_update(&state) {
            return Err(serde::de::Error::custom(format!(
                "known state update `{state}` did not match its schema"
            )));
        }

        Ok(Self { state, fields })
    }
}

fn is_known_state_update(state: &str) -> bool {
    matches!(state, "running" | "idle" | "requires_action")
}

fn other_state_update_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "state",
        &["running", "idle", "requires_action"],
    );
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
    #[schemars(pattern(r"^[A-Z]{3}$"))]
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

/// A streamed item of message content.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ContentChunk {
    /// A unique identifier for the message this chunk belongs to.
    ///
    /// All chunks belonging to the same message share the same `messageId`.
    /// A change in `messageId` indicates a new message has started.
    pub message_id: MessageId,
    /// A single item of content
    pub content: ContentBlock,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This field is chunk-scoped.
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
    pub fn new(content: ContentBlock, message_id: impl Into<MessageId>) -> Self {
        Self {
            content,
            message_id: message_id.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This field is chunk-scoped.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// A user message upsert.
///
/// Only [`UserMessage::message_id`] is required. `content` has patch semantics:
/// an omitted field leaves existing message content unchanged, `null` clears the
/// value, and a concrete array replaces the previous value. For a new
/// `messageId`, omitted fields use client defaults. `content` is replaced as a
/// whole array; send `[]` or `null` to clear it.
///
/// Message updates and chunks are applied in the order they are received. When
/// a `user_message` update includes `content`, that array replaces any content
/// previously accumulated for the message, including content from earlier
/// chunks. Later chunks with the same `messageId` append to the current
/// content.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserMessage {
    /// A unique identifier for the message.
    pub message_id: MessageId,
    /// Complete replacement content for this message.
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub content: MaybeUndefined<Vec<ContentBlock>>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. Omitted means no metadata update; `null` is an explicit clear signal.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<_>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "MaybeUndefined::is_undefined"
    )]
    pub meta: MaybeUndefined<Meta>,
}

impl UserMessage {
    /// Builds [`UserMessage`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(message_id: impl Into<MessageId>) -> Self {
        Self {
            message_id: message_id.into(),
            content: MaybeUndefined::Undefined,
            meta: MaybeUndefined::Undefined,
        }
    }

    /// Complete replacement content for this message.
    #[must_use]
    pub fn content(mut self, content: impl IntoMaybeUndefined<Vec<ContentBlock>>) -> Self {
        self.content = content.into_maybe_undefined();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoMaybeUndefined<Meta>) -> Self {
        self.meta = meta.into_maybe_undefined();
        self
    }
}

/// An agent message upsert.
///
/// Only [`AgentMessage::message_id`] is required. `content` has patch semantics:
/// an omitted field leaves existing message content unchanged, `null` clears the
/// value, and a concrete array replaces the previous value. For a new
/// `messageId`, omitted fields use client defaults. `content` is replaced as a
/// whole array; send `[]` or `null` to clear it.
///
/// Message updates and chunks are applied in the order they are received. When
/// an `agent_message` update includes `content`, that array replaces any
/// content previously accumulated for the message, including content from
/// earlier chunks. Later chunks with the same `messageId` append to the current
/// content.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AgentMessage {
    /// A unique identifier for the message.
    pub message_id: MessageId,
    /// Complete replacement content for this message.
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub content: MaybeUndefined<Vec<ContentBlock>>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. Omitted means no metadata update; `null` is an explicit clear signal.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<_>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "MaybeUndefined::is_undefined"
    )]
    pub meta: MaybeUndefined<Meta>,
}

impl AgentMessage {
    /// Builds [`AgentMessage`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(message_id: impl Into<MessageId>) -> Self {
        Self {
            message_id: message_id.into(),
            content: MaybeUndefined::Undefined,
            meta: MaybeUndefined::Undefined,
        }
    }

    /// Complete replacement content for this message.
    #[must_use]
    pub fn content(mut self, content: impl IntoMaybeUndefined<Vec<ContentBlock>>) -> Self {
        self.content = content.into_maybe_undefined();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoMaybeUndefined<Meta>) -> Self {
        self.meta = meta.into_maybe_undefined();
        self
    }
}

/// An agent thought or reasoning message upsert.
///
/// Only [`AgentThought::message_id`] is required. `content` has patch semantics:
/// an omitted field leaves existing thought content unchanged, `null` clears the
/// value, and a concrete array replaces the previous value. For a new
/// `messageId`, omitted fields use client defaults. `content` is replaced as a
/// whole array; send `[]` or `null` to clear it.
///
/// Message updates and chunks are applied in the order they are received. When
/// an `agent_thought` update includes `content`, that array replaces any
/// content previously accumulated for the thought, including content from
/// earlier chunks. Later chunks with the same `messageId` append to the current
/// content.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AgentThought {
    /// A unique identifier for the thought message.
    pub message_id: MessageId,
    /// Complete replacement content for this thought message.
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub content: MaybeUndefined<Vec<ContentBlock>>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. Omitted means no metadata update; `null` is an explicit clear signal.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<_>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "MaybeUndefined::is_undefined"
    )]
    pub meta: MaybeUndefined<Meta>,
}

impl AgentThought {
    /// Builds [`AgentThought`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(message_id: impl Into<MessageId>) -> Self {
        Self {
            message_id: message_id.into(),
            content: MaybeUndefined::Undefined,
            meta: MaybeUndefined::Undefined,
        }
    }

    /// Complete replacement content for this thought message.
    #[must_use]
    pub fn content(mut self, content: impl IntoMaybeUndefined<Vec<ContentBlock>>) -> Self {
        self.content = content.into_maybe_undefined();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoMaybeUndefined<Meta>) -> Self {
        self.meta = meta.into_maybe_undefined();
        self
    }
}

/// Unique identifier for a message within a session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct MessageId(pub Arc<str>);

impl MessageId {
    /// Wraps a protocol string as a typed [`MessageId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// Available commands are ready or have changed
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AvailableCommandsUpdate {
    /// Commands the agent can execute.
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
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum AvailableCommandInput {
    /// All text that was typed after the command name is provided as input.
    #[serde(rename = "text")]
    Text(TextCommandInput),
    /// Custom or future command input specification.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Clients that do not understand this input type should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding command
    /// metadata, and otherwise ignore the input specification or display the
    /// command without structured input.
    #[serde(untagged)]
    Other(OtherAvailableCommandInput),
}

/// Custom or future command input specification.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_available_command_input_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherAvailableCommandInput {
    /// Custom or future command input type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown command input payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherAvailableCommandInput {
    /// Builds [`OtherAvailableCommandInput`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherAvailableCommandInput {
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

        if is_known_available_command_input_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known available command input type `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

const KNOWN_AVAILABLE_COMMAND_INPUT_TYPES: &[&str] = &["text"];

fn is_known_available_command_input_type(type_: &str) -> bool {
    KNOWN_AVAILABLE_COMMAND_INPUT_TYPES.contains(&type_)
}

fn other_available_command_input_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        KNOWN_AVAILABLE_COMMAND_INPUT_TYPES,
    );
}

/// All text that was typed after the command name is provided as input.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TextCommandInput {
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

impl TextCommandInput {
    /// Builds [`TextCommandInput`] with the required fields set; optional fields start unset or empty.
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

/// Request for user permission to proceed with an operation.
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
    /// Human-readable title for the permission prompt.
    ///
    /// This title is specific to the permission prompt and does not update any
    /// subject's displayed title.
    pub title: String,
    /// Optional human-readable explanation of why permission is needed.
    ///
    /// This text is specific to the permission prompt and does not update any
    /// subject's displayed content. Omitted or `null` both mean no separate
    /// permission description was provided.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Optional structured context about the operation requiring permission.
    ///
    /// Omitted or `null` both mean no structured subject was provided.
    #[serde(default)]
    pub subject: Option<RequestPermissionSubject>,
    /// Available permission options for the user to choose from.
    /// Must contain at least one option.
    #[schemars(length(min = 1))]
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
        title: impl Into<String>,
        options: Vec<PermissionOption>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            title: title.into(),
            description: None,
            subject: None,
            options,
            meta: None,
        }
    }

    /// Sets or clears the optional `description` field.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Sets or clears the optional `subject` field.
    #[must_use]
    pub fn subject(mut self, subject: impl IntoOption<RequestPermissionSubject>) -> Self {
        self.subject = subject.into_option();
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

/// The operation requiring permission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RequestPermissionSubject {
    /// Permission is requested before executing a tool call.
    ToolCall(Box<ToolCallPermissionSubject>),
    /// Permission is requested before running a command.
    Command(CommandPermissionSubject),
    /// Custom or future permission subject.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Clients that do not understand this subject type should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding permission
    /// requests, and otherwise display a generic permission prompt or decline it
    /// according to policy.
    #[serde(untagged)]
    Other(OtherRequestPermissionSubject),
}

impl From<ToolCallPermissionSubject> for RequestPermissionSubject {
    fn from(subject: ToolCallPermissionSubject) -> Self {
        Self::ToolCall(Box::new(subject))
    }
}

impl From<ToolCallUpdate> for RequestPermissionSubject {
    fn from(tool_call: ToolCallUpdate) -> Self {
        ToolCallPermissionSubject::new(tool_call).into()
    }
}

impl From<CommandPermissionSubject> for RequestPermissionSubject {
    fn from(subject: CommandPermissionSubject) -> Self {
        Self::Command(subject)
    }
}

/// Permission request details for a tool call.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ToolCallPermissionSubject {
    /// Details about the tool call requiring permission.
    pub tool_call: ToolCallUpdate,
}

impl ToolCallPermissionSubject {
    /// Builds [`ToolCallPermissionSubject`] with the required fields set.
    #[must_use]
    pub fn new(tool_call: ToolCallUpdate) -> Self {
        Self { tool_call }
    }
}

/// Permission request details for a command.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CommandPermissionSubject {
    /// The command that would be run if permission is granted.
    pub command: String,
    /// The absolute working directory for the command.
    pub cwd: AbsolutePath,
    /// The associated tool call, when known. Omitted and `null` are equivalent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub tool_call_id: Option<ToolCallId>,
    /// The associated terminal, when already known. Omitted and `null` are equivalent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub terminal_id: Option<TerminalId>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. Omitted and `null` are equivalent and mean no subject metadata was provided.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl CommandPermissionSubject {
    /// Builds command permission details with the required command and working directory.
    #[must_use]
    pub fn new(command: impl Into<String>, cwd: impl Into<AbsolutePath>) -> Self {
        Self {
            command: command.into(),
            cwd: cwd.into(),
            tool_call_id: None,
            terminal_id: None,
            meta: None,
        }
    }

    /// Sets or clears the associated tool-call ID.
    #[must_use]
    pub fn tool_call_id(mut self, tool_call_id: impl IntoOption<ToolCallId>) -> Self {
        self.tool_call_id = tool_call_id.into_option();
        self
    }

    /// Sets or clears the associated terminal ID.
    #[must_use]
    pub fn terminal_id(mut self, terminal_id: impl IntoOption<TerminalId>) -> Self {
        self.terminal_id = terminal_id.into_option();
        self
    }

    /// Sets or clears subject-scoped metadata.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Custom or future permission subject payload.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq)]
#[schemars(inline)]
#[schemars(transform = other_request_permission_subject_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherRequestPermissionSubject {
    /// Custom or future permission subject type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown permission subject payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherRequestPermissionSubject {
    /// Builds [`OtherRequestPermissionSubject`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherRequestPermissionSubject {
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

        if is_known_request_permission_subject_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known request permission subject `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

fn is_known_request_permission_subject_type(type_: &str) -> bool {
    matches!(type_, "tool_call" | "command")
}

fn other_request_permission_subject_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        &["tool_call", "command"],
    );
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
#[from(forward)]
#[non_exhaustive]
pub struct PermissionOptionId(pub Arc<str>);

impl PermissionOptionId {
    /// Wraps a protocol string as a typed [`PermissionOptionId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// The type of permission option being presented to the user.
///
/// Helps clients choose appropriate icons and UI treatment.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
    /// Custom or future permission option kind.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
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
#[non_exhaustive]
pub enum RequestPermissionOutcome {
    /// Active session work was cancelled before the user responded.
    ///
    /// When a client sends a `session/cancel` notification to cancel active
    /// session work, it MUST respond to all pending `session/request_permission`
    /// requests with this `Cancelled` outcome.
    ///
    /// See protocol docs: [Cancellation](https://agentclientprotocol.com/protocol/prompt-lifecycle#cancellation)
    Cancelled,
    /// The user selected one of the provided options.
    #[serde(rename_all = "camelCase")]
    Selected(SelectedPermissionOutcome),
    /// Custom or future permission outcome.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Agents that do not understand this outcome MUST NOT treat it as approval.
    /// They should preserve the raw payload when storing, replaying, proxying, or
    /// forwarding permission responses, and otherwise fail or decline the
    /// permission request according to policy.
    #[serde(untagged)]
    Other(OtherRequestPermissionOutcome),
}

/// Custom or future permission outcome payload.
///
/// This preserves the unknown `outcome` discriminator and the rest of the
/// outcome object for agents that store, replay, proxy, or forward permission
/// responses.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_request_permission_outcome_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherRequestPermissionOutcome {
    /// Custom or future permission outcome.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    pub outcome: String,
    /// Additional fields from the unknown permission outcome payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherRequestPermissionOutcome {
    /// Builds [`OtherRequestPermissionOutcome`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        outcome: impl Into<String>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("outcome");
        Self {
            outcome: outcome.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherRequestPermissionOutcome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let outcome = fields
            .remove("outcome")
            .ok_or_else(|| serde::de::Error::missing_field("outcome"))?;
        let serde_json::Value::String(outcome) = outcome else {
            return Err(serde::de::Error::custom("`outcome` must be a string"));
        };

        if is_known_request_permission_outcome(&outcome) {
            return Err(serde::de::Error::custom(format!(
                "known request permission outcome `{outcome}` did not match its schema"
            )));
        }

        Ok(Self { outcome, fields })
    }
}

fn is_known_request_permission_outcome(outcome: &str) -> bool {
    matches!(outcome, "cancelled" | "selected")
}

fn other_request_permission_outcome_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "outcome",
        &["cancelled", "selected"],
    );
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
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Authentication capabilities supported by the client.
    /// Determines which authentication method types the agent may include
    /// in its `InitializeResponse`.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise any
    /// authentication-method extensions.
    #[cfg(feature = "unstable_auth_methods")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub auth: Option<AuthCapabilities>,
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

    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Authentication capabilities supported by the client.
    /// Determines which authentication method types the agent may include
    /// in its `InitializeResponse`.
    #[cfg(feature = "unstable_auth_methods")]
    #[must_use]
    pub fn auth(mut self, auth: impl IntoOption<AuthCapabilities>) -> Self {
        self.auth = auth.into_option();
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
    /// Optional. Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the agent may include `terminal` entries in its authentication methods.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub terminal: Option<TerminalAuthCapabilities>,
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
    /// Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the agent may include `AuthMethod::Terminal` entries in its authentication methods.
    #[must_use]
    pub fn terminal(mut self, terminal: impl IntoOption<TerminalAuthCapabilities>) -> Self {
        self.terminal = terminal.into_option();
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
/// Capabilities for terminal authentication methods.
///
/// Supplying `{}` means the client supports terminal authentication methods.
#[cfg(feature = "unstable_auth_methods")]
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub struct TerminalAuthCapabilities {
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
impl TerminalAuthCapabilities {
    /// Builds an empty [`TerminalAuthCapabilities`]; use builder methods to advertise supported sub-capabilities.
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
pub enum AgentRequest {
    /// Requests permission from the user for an operation.
    ///
    /// Called by the agent when it needs user authorization before executing
    /// a potentially sensitive operation. The client should present the options
    /// to the user and return their decision.
    ///
    /// If the client cancels active session work via `session/cancel`, it MUST
    /// respond to this request with `RequestPermissionOutcome::Cancelled`.
    ///
    /// See protocol docs: [Requesting Permission](https://agentclientprotocol.com/protocol/tool-calls#requesting-permission)
    RequestPermissionRequest(Box<RequestPermissionRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Requests structured user input via a form or URL.
    #[cfg(feature = "unstable_elicitation")]
    CreateElicitationRequest(Box<CreateElicitationRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Opens an MCP-over-ACP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    ConnectMcpRequest(Box<ConnectMcpRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Exchanges an MCP-over-ACP message.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpRequest(Box<MessageMcpRequest>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Closes an MCP-over-ACP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    DisconnectMcpRequest(Box<DisconnectMcpRequest>),
    /// Handles extension method requests from the agent.
    ///
    /// Allows the Agent to send an arbitrary request that is not part of the ACP spec.
    /// Extension methods provide a way to add custom functionality while maintaining
    /// protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtMethodRequest(Box<ExtRequest>),
}

impl AgentRequest {
    /// Returns the corresponding method name of the request.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::RequestPermissionRequest(_) => CLIENT_METHOD_NAMES.session_request_permission,
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
    /// Successful result returned for a `session/request_permission` request.
    RequestPermissionResponse(Box<RequestPermissionResponse>),
    /// Successful result returned for a `elicitation/create` request.
    #[cfg(feature = "unstable_elicitation")]
    CreateElicitationResponse(Box<CreateElicitationResponse>),
    /// Successful result returned for a `mcp/connect` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    ConnectMcpResponse(Box<ConnectMcpResponse>),
    /// Successful result returned for a `mcp/disconnect` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    DisconnectMcpResponse(#[serde(default)] Box<DisconnectMcpResponse>),
    /// Successful result returned by an MCP-over-ACP `mcp/message` request.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpResponse(Box<MessageMcpResponse>),
    /// Successful result returned by an extension method outside the core ACP method set.
    ExtMethodResponse(Box<ExtResponse>),
}

/// All possible notifications that an agent can send to a client.
///
/// This enum is used internally for routing RPC notifications. You typically won't need
/// to use this directly.
///
/// Notifications do not expect a response.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
#[non_exhaustive]
pub enum AgentNotification {
    /// Handles session update notifications from the agent.
    ///
    /// This is a notification endpoint (no response expected) that receives
    /// updates about session activity, including message updates, message chunks,
    /// tool calls, and execution plans.
    ///
    /// Note: Clients SHOULD continue accepting tool call updates even after
    /// sending a `session/cancel` notification, as the agent may send final
    /// updates before reporting an idle `state_update` with the cancelled
    /// stop reason.
    ///
    /// See protocol docs: [Agent Reports Output](https://agentclientprotocol.com/protocol/prompt-lifecycle#3-agent-reports-output)
    UpdateSessionNotification(Box<UpdateSessionNotification>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Notification that a URL-based elicitation has completed.
    #[cfg(feature = "unstable_elicitation")]
    CompleteElicitationNotification(Box<CompleteElicitationNotification>),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Receives an MCP-over-ACP notification.
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpNotification(Box<MessageMcpNotification>),
    /// Handles extension notifications from the agent.
    ///
    /// Allows the Agent to send an arbitrary notification that is not part of the ACP spec.
    /// Extension notifications provide a way to send one-way messages for custom functionality
    /// while maintaining protocol compatibility.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    ExtNotification(Box<ExtNotification>),
}

impl AgentNotification {
    /// Returns the corresponding method name of the notification.
    #[must_use]
    pub fn method(&self) -> &str {
        match self {
            Self::UpdateSessionNotification(_) => CLIENT_METHOD_NAMES.session_update,
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

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_client_capabilities_auth_defaults_on_malformed_value() {
        use serde_json::json;

        let capabilities: ClientCapabilities = serde_json::from_value(json!({
            "auth": false
        }))
        .unwrap();

        assert_eq!(capabilities.auth, None);
    }

    #[test]
    fn test_serialization_behavior() {
        use serde_json::json;

        assert_eq!(
            serde_json::from_value::<SessionInfoUpdate>(json!({})).unwrap(),
            SessionInfoUpdate {
                title: MaybeUndefined::Undefined,
                updated_at: MaybeUndefined::Undefined,
                meta: MaybeUndefined::Undefined
            }
        );
        assert_eq!(
            serde_json::from_value::<SessionInfoUpdate>(json!({"title": null, "updatedAt": null}))
                .unwrap(),
            SessionInfoUpdate {
                title: MaybeUndefined::Null,
                updated_at: MaybeUndefined::Null,
                meta: MaybeUndefined::Undefined
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
                meta: MaybeUndefined::Undefined
            }
        );

        let clear_meta =
            serde_json::from_value::<SessionInfoUpdate>(json!({"_meta": null})).unwrap();
        assert_eq!(clear_meta.meta, MaybeUndefined::Null);

        let mut meta = Meta::new();
        meta.insert("source".to_string(), json!("session-info"));

        assert_eq!(
            serde_json::from_value::<SessionInfoUpdate>(json!({"_meta": {
                "source": "session-info"
            }}))
            .unwrap()
            .meta,
            MaybeUndefined::Value(meta.clone())
        );

        assert_eq!(
            serde_json::to_value(SessionInfoUpdate::new()).unwrap(),
            json!({})
        );

        assert_eq!(
            serde_json::to_value(SessionInfoUpdate::new().meta(None::<Meta>)).unwrap(),
            json!({"_meta": null})
        );

        assert_eq!(
            serde_json::to_value(SessionInfoUpdate::new().meta(meta)).unwrap(),
            json!({"_meta": {
                "source": "session-info"
            }})
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
                ContentBlock::Text(crate::v2::TextContent::new("Hello")),
                "msg_agent_c42b9",
            )))
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

        let err = serde_json::from_value::<ContentChunk>(json!({
            "content": {
                "type": "text",
                "text": "Hello"
            }
        }))
        .unwrap_err();

        assert!(err.to_string().contains("messageId"), "{err}");
    }

    #[test]
    fn test_tool_call_content_chunk_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::ToolCallContentChunk(
                ToolCallContentChunk::new(
                    "call_001",
                    crate::v2::ContentBlock::Text(crate::v2::TextContent::new("partial output")),
                )
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "tool_call_content_chunk",
                "toolCallId": "call_001",
                "content": {
                    "type": "content",
                    "content": {
                        "type": "text",
                        "text": "partial output"
                    }
                }
            })
        );

        let err = serde_json::from_value::<ToolCallContentChunk>(json!({
            "content": {
                "type": "content",
                "content": {
                    "type": "text",
                    "text": "partial output"
                }
            }
        }))
        .unwrap_err();

        assert!(err.to_string().contains("toolCallId"), "{err}");
    }

    #[test]
    fn test_full_message_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::UserMessage(
                UserMessage::new("msg_user_8f7a1").content(vec![ContentBlock::Text(
                    crate::v2::TextContent::new("Hello")
                )])
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "user_message",
                "messageId": "msg_user_8f7a1",
                "content": [
                    {
                        "type": "text",
                        "text": "Hello"
                    }
                ]
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::AgentMessage(
                AgentMessage::new("msg_agent_c42b9").content(vec![ContentBlock::Text(
                    crate::v2::TextContent::new("Hello")
                )])
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "agent_message",
                "messageId": "msg_agent_c42b9",
                "content": [
                    {
                        "type": "text",
                        "text": "Hello"
                    }
                ]
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::AgentThought(
                AgentThought::new("msg_thought_a12").content(vec![ContentBlock::Text(
                    crate::v2::TextContent::new("Need to inspect the call sites first.")
                )])
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "agent_thought",
                "messageId": "msg_thought_a12",
                "content": [
                    {
                        "type": "text",
                        "text": "Need to inspect the call sites first."
                    }
                ]
            })
        );
    }

    #[test]
    fn test_message_upsert_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::UserMessage(
                UserMessage::new("msg_empty").content(Vec::<ContentBlock>::new())
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "user_message",
                "messageId": "msg_empty",
                "content": []
            })
        );

        let empty = serde_json::from_value::<UserMessage>(json!({
            "messageId": "msg_empty",
            "content": []
        }))
        .unwrap();
        assert!(matches!(
            empty.content,
            MaybeUndefined::Value(ref content) if content.is_empty()
        ));

        let patch = serde_json::from_value::<AgentMessage>(json!({
            "messageId": "msg_agent_c42b9"
        }))
        .unwrap();
        assert_eq!(patch.content, MaybeUndefined::Undefined);
        assert_eq!(patch.meta, MaybeUndefined::Undefined);

        let malformed_meta = serde_json::from_value::<AgentMessage>(json!({
            "messageId": "msg_agent_c42b9",
            "_meta": false
        }))
        .unwrap();
        assert_eq!(malformed_meta.meta, MaybeUndefined::Undefined);

        let patch = serde_json::from_value::<AgentThought>(json!({
            "messageId": "msg_thought_a12"
        }))
        .unwrap();
        assert_eq!(patch.content, MaybeUndefined::Undefined);

        let clear = serde_json::from_value::<UserMessage>(json!({
            "messageId": "msg_user_8f7a1",
            "content": null
        }))
        .unwrap();
        assert_eq!(clear.content, MaybeUndefined::Null);

        let clear_meta = serde_json::from_value::<UserMessage>(json!({
            "messageId": "msg_user_8f7a1",
            "_meta": null
        }))
        .unwrap();
        assert_eq!(clear_meta.meta, MaybeUndefined::Null);

        let mut meta = Meta::new();
        meta.insert("source".to_string(), json!("replay"));

        assert_eq!(
            serde_json::to_value(SessionUpdate::UserMessage(
                UserMessage::new("msg_user_8f7a1").meta(meta)
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "user_message",
                "messageId": "msg_user_8f7a1",
                "_meta": {
                    "source": "replay"
                }
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::UserMessage(
                UserMessage::new("msg_user_8f7a1").meta(None::<Meta>)
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "user_message",
                "messageId": "msg_user_8f7a1",
                "_meta": null
            })
        );
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

    #[test]
    fn test_state_update_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::StateUpdate(StateUpdate::Running(
                RunningStateUpdate::new()
            )))
            .unwrap(),
            json!({
                "sessionUpdate": "state_update",
                "state": "running"
            })
        );

        assert_eq!(
            serde_json::to_value(SessionUpdate::StateUpdate(StateUpdate::Idle(
                IdleStateUpdate::new().stop_reason(StopReason::EndTurn)
            )))
            .unwrap(),
            json!({
                "sessionUpdate": "state_update",
                "state": "idle",
                "stopReason": "end_turn"
            })
        );

        let SessionUpdate::StateUpdate(update) = serde_json::from_value(json!({
            "sessionUpdate": "state_update",
            "state": "requires_action"
        }))
        .unwrap() else {
            panic!("expected state update");
        };

        assert!(matches!(update, StateUpdate::RequiresAction(_)));

        let SessionUpdate::StateUpdate(StateUpdate::Idle(update)) = serde_json::from_value(json!({
            "sessionUpdate": "state_update",
            "state": "idle",
            "stopReason": null
        }))
        .unwrap() else {
            panic!("expected idle state update");
        };

        assert_eq!(update.stop_reason, None);

        let SessionUpdate::StateUpdate(StateUpdate::Other(update)) =
            serde_json::from_value(json!({
                "sessionUpdate": "state_update",
                "state": "_paused",
                "label": "Paused"
            }))
            .unwrap()
        else {
            panic!("expected unknown state update");
        };

        assert_eq!(update.state, "_paused");
        assert_eq!(update.fields["label"], json!("Paused"));
    }

    #[test]
    fn session_update_preserves_unknown_variant() {
        use serde_json::json;

        let update: SessionUpdate = serde_json::from_value(json!({
            "sessionUpdate": "_status_badge",
            "label": "Indexing",
            "progress": 0.5
        }))
        .unwrap();

        let SessionUpdate::Other(unknown) = update else {
            panic!("expected unknown session update");
        };

        assert_eq!(unknown.session_update, "_status_badge");
        assert_eq!(unknown.fields.get("label"), Some(&json!("Indexing")));
        assert_eq!(unknown.fields.get("progress"), Some(&json!(0.5)));

        assert_eq!(
            serde_json::to_value(SessionUpdate::Other(unknown)).unwrap(),
            json!({
                "sessionUpdate": "_status_badge",
                "label": "Indexing",
                "progress": 0.5
            })
        );
    }

    #[test]
    fn terminal_session_updates_use_known_discriminators() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::TerminalUpdate(
                TerminalUpdate::new("term_1").command("cargo test")
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "terminal_update",
                "terminalId": "term_1",
                "command": "cargo test"
            })
        );
        assert_eq!(
            serde_json::to_value(SessionUpdate::TerminalOutputChunk(
                TerminalOutputChunk::new("term_1", "dGVzdAo=")
            ))
            .unwrap(),
            json!({
                "sessionUpdate": "terminal_output_chunk",
                "terminalId": "term_1",
                "data": "dGVzdAo="
            })
        );
    }

    #[test]
    fn session_update_does_not_hide_malformed_known_terminal_variants() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<SessionUpdate>(json!({
                "sessionUpdate": "terminal_update"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<SessionUpdate>(json!({
                "sessionUpdate": "terminal_output_chunk",
                "terminalId": "term_1"
            }))
            .is_err()
        );
    }

    #[test]
    fn test_plan_update_serialization() {
        use serde_json::json;

        let plan_update =
            SessionUpdate::PlanUpdate(PlanUpdate::new(crate::v2::PlanUpdateContent::items(
                "plan-1",
                vec![crate::v2::PlanEntry::new(
                    "Step 1",
                    crate::v2::PlanEntryPriority::High,
                    crate::v2::PlanEntryStatus::Pending,
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
    }

    #[cfg(feature = "unstable_plan_operations")]
    #[test]
    fn test_plan_removed_serialization() {
        use serde_json::json;

        assert_eq!(
            serde_json::to_value(SessionUpdate::PlanRemoved(PlanRemoved::new("plan-1"))).unwrap(),
            json!({
                "sessionUpdate": "plan_removed",
                "planId": "plan-1"
            })
        );
    }

    #[test]
    fn available_command_input_preserves_unknown_typed_variant() {
        use serde_json::json;

        let input: AvailableCommandInput = serde_json::from_value(json!({
            "type": "_choices",
            "hint": "Pick one",
            "options": ["fast", "careful"]
        }))
        .unwrap();

        let AvailableCommandInput::Other(unknown) = input else {
            panic!("expected unknown command input");
        };

        assert_eq!(unknown.type_, "_choices");
        assert_eq!(unknown.fields.get("hint"), Some(&json!("Pick one")));
        assert_eq!(
            unknown.fields.get("options"),
            Some(&json!(["fast", "careful"]))
        );
        assert_eq!(
            serde_json::to_value(AvailableCommandInput::Other(unknown)).unwrap(),
            json!({
                "type": "_choices",
                "hint": "Pick one",
                "options": ["fast", "careful"]
            })
        );
    }

    #[test]
    fn available_command_input_text_uses_type_discriminator() {
        use serde_json::json;

        let input = AvailableCommandInput::Text(TextCommandInput::new("Describe changes"));

        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "text",
                "hint": "Describe changes"
            })
        );

        let roundtripped: AvailableCommandInput = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped, AvailableCommandInput::Text(_)));
    }

    #[test]
    fn request_permission_subject_tool_call_uses_type_discriminator() {
        use serde_json::json;

        let subject = RequestPermissionSubject::from(ToolCallUpdate::new("call_001"));

        let json = serde_json::to_value(&subject).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "tool_call",
                "toolCall": {
                    "toolCallId": "call_001"
                }
            })
        );

        let roundtripped: RequestPermissionSubject = serde_json::from_value(json).unwrap();
        assert!(matches!(
            roundtripped,
            RequestPermissionSubject::ToolCall(_)
        ));
    }

    #[test]
    fn request_permission_subject_command_uses_type_discriminator() {
        use serde_json::json;

        let mut meta = Meta::new();
        meta.insert("source".to_string(), json!("shell"));
        let subject = RequestPermissionSubject::from(
            CommandPermissionSubject::new("cargo test", "/workspace/project")
                .tool_call_id("call_001")
                .terminal_id("term_1")
                .meta(meta),
        );

        let json = serde_json::to_value(&subject).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "command",
                "command": "cargo test",
                "cwd": "/workspace/project",
                "toolCallId": "call_001",
                "terminalId": "term_1",
                "_meta": {
                    "source": "shell"
                }
            })
        );

        let roundtripped: RequestPermissionSubject = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped, RequestPermissionSubject::Command(_)));
    }

    #[test]
    fn command_permission_subject_treats_optional_association_nulls_as_omitted() {
        use serde_json::json;

        let subject: RequestPermissionSubject = serde_json::from_value(json!({
            "type": "command",
            "command": "cargo test",
            "cwd": "/workspace/project",
            "toolCallId": null,
            "terminalId": null,
            "_meta": null
        }))
        .unwrap();

        let RequestPermissionSubject::Command(subject) = subject else {
            panic!("expected command permission subject");
        };
        assert_eq!(subject.cwd, AbsolutePath::new("/workspace/project"));
        assert_eq!(subject.tool_call_id, None);
        assert_eq!(subject.terminal_id, None);
        assert_eq!(subject.meta, None);
        assert_eq!(
            serde_json::to_value(RequestPermissionSubject::Command(subject)).unwrap(),
            json!({
                "type": "command",
                "command": "cargo test",
                "cwd": "/workspace/project"
            })
        );
    }

    #[test]
    fn request_permission_subject_preserves_unknown_variant() {
        use serde_json::json;

        let subject: RequestPermissionSubject = serde_json::from_value(json!({
            "type": "_review",
            "reason": "needs-review",
            "retryAfterSeconds": 30
        }))
        .unwrap();

        let RequestPermissionSubject::Other(unknown) = subject else {
            panic!("expected unknown permission subject");
        };

        assert_eq!(unknown.type_, "_review");
        assert_eq!(unknown.fields.get("reason"), Some(&json!("needs-review")));
        assert_eq!(unknown.fields.get("retryAfterSeconds"), Some(&json!(30)));
        assert_eq!(
            serde_json::to_value(RequestPermissionSubject::Other(unknown)).unwrap(),
            json!({
                "type": "_review",
                "reason": "needs-review",
                "retryAfterSeconds": 30
            })
        );
    }

    #[test]
    fn request_permission_subject_unknown_does_not_hide_malformed_known_variant() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<RequestPermissionSubject>(json!({
                "type": "tool_call"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionSubject>(json!({
                "type": 1
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionSubject>(json!({
                "type": "command",
                "cwd": "/workspace/project"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionSubject>(json!({
                "type": "command",
                "command": "cargo test"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionSubject>(json!({
                "type": "command",
                "command": "cargo test",
                "cwd": null
            }))
            .is_err()
        );
    }

    #[test]
    fn request_permission_title_and_description_are_separate_from_tool_call_content() {
        use serde_json::json;

        let request =
            RequestPermissionRequest::new("sess_abc123def456", "Approve file edit?", Vec::new())
                .description("Allow this tool to edit src/main.rs?")
                .subject(RequestPermissionSubject::from(ToolCallUpdate::new(
                    "call_001",
                )));

        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({
                "sessionId": "sess_abc123def456",
                "title": "Approve file edit?",
                "description": "Allow this tool to edit src/main.rs?",
                "subject": {
                    "type": "tool_call",
                    "toolCall": {
                        "toolCallId": "call_001"
                    }
                },
                "options": []
            })
        );
    }

    #[test]
    fn request_permission_requires_title_and_allows_missing_subject() {
        use serde_json::json;

        let request = RequestPermissionRequest::new(
            "sess_abc123def456",
            "Approve elevated permissions?",
            Vec::new(),
        );

        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({
                "sessionId": "sess_abc123def456",
                "title": "Approve elevated permissions?",
                "options": []
            })
        );

        let missing_subject: RequestPermissionRequest = serde_json::from_value(json!({
            "sessionId": "sess_abc123def456",
            "title": "Approve elevated permissions?",
            "options": []
        }))
        .unwrap();
        assert!(missing_subject.subject.is_none());

        let null_subject: RequestPermissionRequest = serde_json::from_value(json!({
            "sessionId": "sess_abc123def456",
            "title": "Approve elevated permissions?",
            "subject": null,
            "options": []
        }))
        .unwrap();
        assert!(null_subject.subject.is_none());

        assert!(
            serde_json::from_value::<RequestPermissionRequest>(json!({
                "sessionId": "sess_abc123def456",
                "options": []
            }))
            .is_err()
        );
    }

    #[test]
    fn request_permission_outcome_preserves_unknown_variant() {
        use serde_json::json;

        let outcome: RequestPermissionOutcome = serde_json::from_value(json!({
            "outcome": "_defer",
            "reason": "needs-review",
            "retryAfterSeconds": 30
        }))
        .unwrap();

        let RequestPermissionOutcome::Other(unknown) = outcome else {
            panic!("expected unknown permission outcome");
        };

        assert_eq!(unknown.outcome, "_defer");
        assert_eq!(unknown.fields.get("reason"), Some(&json!("needs-review")));
        assert_eq!(unknown.fields.get("retryAfterSeconds"), Some(&json!(30)));
        assert_eq!(
            serde_json::to_value(RequestPermissionOutcome::Other(unknown)).unwrap(),
            json!({
                "outcome": "_defer",
                "reason": "needs-review",
                "retryAfterSeconds": 30
            })
        );
    }

    #[test]
    fn request_permission_outcome_unknown_does_not_hide_malformed_known_variant() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<RequestPermissionOutcome>(json!({
                "outcome": "selected"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionOutcome>(json!({
                "outcome": 1
            }))
            .is_err()
        );
    }

    #[test]
    fn available_command_input_unknown_does_not_hide_malformed_text_variant() {
        use serde_json::json;

        assert!(serde_json::from_value::<AvailableCommandInput>(json!({})).is_err());
        assert!(
            serde_json::from_value::<AvailableCommandInput>(json!({
                "hint": "Pick one"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<AvailableCommandInput>(json!({
                "type": 1,
                "hint": "Pick one"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<OtherAvailableCommandInput>(json!({
                "type": "text",
                "hint": "Pick one"
            }))
            .is_err()
        );
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
            AgentRequest::ConnectMcpRequest(Box::new(ConnectMcpRequest::new("server-1"))).method(),
            "mcp/connect"
        );
        assert_eq!(
            AgentRequest::MessageMcpRequest(Box::new(MessageMcpRequest::new(
                "conn-1",
                "tools/list"
            )))
            .method(),
            "mcp/message"
        );
        assert_eq!(
            AgentRequest::DisconnectMcpRequest(Box::new(DisconnectMcpRequest::new("conn-1")))
                .method(),
            "mcp/disconnect"
        );
        assert_eq!(
            AgentNotification::MessageMcpNotification(Box::new(MessageMcpNotification::new(
                "conn-1",
                "notifications/progress"
            )))
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

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn test_auth_capabilities_serialize_terminal_support_as_object() {
        use serde_json::json;

        let capabilities = AuthCapabilities::new().terminal(TerminalAuthCapabilities::new());

        assert_eq!(
            serde_json::to_value(&capabilities).unwrap(),
            json!({
                "terminal": {}
            })
        );

        let deserialized: AuthCapabilities = serde_json::from_value(json!({
            "terminal": false
        }))
        .unwrap();
        assert!(deserialized.terminal.is_none());
    }

    #[test]
    fn request_permission_request_rejects_malformed_options() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<RequestPermissionRequest>(json!({
                "sessionId": "sess-1",
                "title": "Run tool?",
                "options": "not-an-array"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<RequestPermissionRequest>(json!({
                "sessionId": "sess-1",
                "title": "Run tool?",
                "options": [{"optionId": "allow"}]
            }))
            .is_err()
        );
    }

    #[cfg(feature = "unstable_plan_operations")]
    #[test]
    fn malformed_plan_removed_is_not_hidden_as_unknown_update() {
        use serde_json::json;

        assert!(
            serde_json::from_value::<SessionUpdate>(json!({
                "sessionUpdate": "plan_removed"
            }))
            .is_err()
        );
    }
}
