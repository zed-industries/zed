//! Decoding for the draft ACP subagent updates, which announce and settle a
//! child session an agent created for delegated work.
//!
//! The released schema has no `SessionUpdate` variant for them, so the typed
//! handler rejects the whole notification. Decode them from the raw message
//! instead; delete this once the schema ships the variants.

use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context as _, Result};
use serde::Deserialize;

/// A child session was created and is now running work for its parent.
pub const SUBAGENT_SPAWNED: &str = "subagent_spawned";
/// A child session reached a terminal state.
pub const SUBAGENT_STATE_UPDATE: &str = "subagent_state_update";

/// Vendor key attributing an update to the tool call that spawned the subagent.
/// The only link between a child session and the call its work belongs to, so
/// reading it is best-effort.
const PARENT_TOOL_CALL_META_PATH: [&str; 2] = ["claudeCode", "parentToolUseId"];

/// What an inbound `session/update` tells us about subagents.
#[derive(Debug, Clone, PartialEq)]
pub enum InboundUpdate {
    /// The typed handlers cannot represent it; the caller claims it.
    Subagent(SubagentNotification),
    /// An ordinary update naming the call it belongs to; still passed on.
    Attribution(SubagentAttribution),
}

/// A child's work, attributed to the call that started it.
#[derive(Debug, Clone, PartialEq)]
pub struct SubagentAttribution {
    pub session_id: acp::SessionId,
    pub parent_tool_call_id: acp::ToolCallId,
}

/// A decoded child-session update and the parent session it arrived on.
#[derive(Debug, Clone, PartialEq)]
pub struct SubagentNotification {
    /// The session the agent sent the update on: the child's parent.
    pub parent_session_id: acp::SessionId,
    pub update: SubagentUpdate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubagentUpdate {
    Spawned(SubagentSpawned),
    StateChanged(SubagentStateChanged),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubagentSpawned {
    pub session_id: acp::SessionId,
    /// Display name, e.g. the agent type it was spawned as.
    pub name: String,
    /// Short description of what the parent asked for.
    pub task: String,
    /// The prompt the parent sent, when the agent reports it.
    pub prompt: Option<String>,
    /// Whether the client may cancel this child's work on its own.
    pub can_cancel: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubagentStateChanged {
    pub session_id: acp::SessionId,
    pub state: SubagentState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubagentState {
    Completed,
    Failed,
    Cancelled,
    /// Lost track of the child without a result.
    Disconnected,
}

/// Returns `None` when the notification says nothing about subagents, which is
/// the common case, and `Some(Err(..))` when one announced itself as a
/// child-session update but did not decode.
pub fn inspect(method: &str, params: &serde_json::Value) -> Option<Result<InboundUpdate>> {
    if !<acp::SessionNotification as agent_client_protocol::JsonRpcMessage>::matches_method(method)
    {
        return None;
    }

    let update = params.get("update")?;
    let session_id = params
        .get("sessionId")
        .and_then(serde_json::Value::as_str)?;

    match update
        .get("sessionUpdate")
        .and_then(serde_json::Value::as_str)
    {
        Some(kind @ (SUBAGENT_SPAWNED | SUBAGENT_STATE_UPDATE)) => {
            Some(decode_matched(kind, session_id, update).map(InboundUpdate::Subagent))
        }
        _ => {
            let parent_tool_call_id = parent_tool_call_id(update)?;
            Some(Ok(InboundUpdate::Attribution(SubagentAttribution {
                session_id: acp::SessionId::new(session_id),
                parent_tool_call_id,
            })))
        }
    }
}

fn parent_tool_call_id(update: &serde_json::Value) -> Option<acp::ToolCallId> {
    let [namespace, key] = PARENT_TOOL_CALL_META_PATH;
    let id = update
        .get("_meta")?
        .get(namespace)?
        .get(key)?
        .as_str()
        .filter(|id| !id.is_empty())?;
    Some(acp::ToolCallId::new(id))
}

fn decode_matched(
    kind: &str,
    parent_session_id: &str,
    update: &serde_json::Value,
) -> Result<SubagentNotification> {
    let update = match kind {
        SUBAGENT_SPAWNED => {
            let spawned: RawSubagentSpawned = serde_json::from_value(update.clone())
                .context("failed to decode a `subagent_spawned` update")?;
            SubagentUpdate::Spawned(SubagentSpawned {
                session_id: acp::SessionId::new(spawned.subagent_session_id),
                name: spawned.name,
                task: spawned.task,
                prompt: spawned.prompt,
                can_cancel: spawned.capabilities.cancel,
            })
        }
        SUBAGENT_STATE_UPDATE => {
            let changed: RawSubagentStateUpdate = serde_json::from_value(update.clone())
                .context("failed to decode a `subagent_state_update` update")?;
            SubagentUpdate::StateChanged(SubagentStateChanged {
                session_id: acp::SessionId::new(changed.subagent_session_id),
                state: changed.state,
            })
        }
        _ => unreachable!("decode only matches the two subagent discriminants"),
    };

    Ok(SubagentNotification {
        parent_session_id: acp::SessionId::new(parent_session_id),
        update,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSubagentSpawned {
    subagent_session_id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    task: String,
    #[serde(default)]
    prompt: Option<String>,
    #[serde(default)]
    capabilities: RawSubagentCapabilities,
}

/// Mutations the agent permits on the child session; absent means denied.
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSubagentCapabilities {
    #[serde(default)]
    cancel: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSubagentStateUpdate {
    subagent_session_id: String,
    state: SubagentState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const METHOD: &str = "session/update";

    fn subagent(method: &str, params: serde_json::Value) -> Option<Result<SubagentNotification>> {
        match inspect(method, &params)? {
            Ok(InboundUpdate::Subagent(notification)) => Some(Ok(notification)),
            Ok(InboundUpdate::Attribution(_)) => None,
            Err(error) => Some(Err(error)),
        }
    }

    #[test]
    fn decodes_a_spawn() {
        let decoded = subagent(
            METHOD,
            json!({
                "sessionId": "parent",
                "update": {
                    "sessionUpdate": "subagent_spawned",
                    "subagentSessionId": "child",
                    "name": "Explore",
                    "task": "Find the callers of `foo`",
                    "prompt": "Find every caller of `foo` and report the files.",
                    "capabilities": { "cancel": true },
                },
            }),
        )
        .expect("the discriminant should match")
        .expect("the payload should decode");

        assert_eq!(decoded.parent_session_id, acp::SessionId::new("parent"));
        assert_eq!(
            decoded.update,
            SubagentUpdate::Spawned(SubagentSpawned {
                session_id: acp::SessionId::new("child"),
                name: "Explore".into(),
                task: "Find the callers of `foo`".into(),
                prompt: Some("Find every caller of `foo` and report the files.".into()),
                can_cancel: true,
            })
        );
    }

    #[test]
    fn decodes_a_spawn_without_optional_fields() {
        let decoded = subagent(
            METHOD,
            json!({
                "sessionId": "parent",
                "update": {
                    "sessionUpdate": "subagent_spawned",
                    "subagentSessionId": "child",
                    "capabilities": {},
                },
            }),
        )
        .expect("the discriminant should match")
        .expect("a spawn without optional fields should still decode");

        assert_eq!(
            decoded.update,
            SubagentUpdate::Spawned(SubagentSpawned {
                session_id: acp::SessionId::new("child"),
                name: String::new(),
                task: String::new(),
                prompt: None,
                can_cancel: false,
            })
        );
    }

    #[test]
    fn decodes_a_terminal_state() {
        let decoded = subagent(
            METHOD,
            json!({
                "sessionId": "parent",
                "update": {
                    "sessionUpdate": "subagent_state_update",
                    "subagentSessionId": "child",
                    "state": "cancelled",
                },
            }),
        )
        .expect("the discriminant should match")
        .expect("the payload should decode");

        assert_eq!(
            decoded.update,
            SubagentUpdate::StateChanged(SubagentStateChanged {
                session_id: acp::SessionId::new("child"),
                state: SubagentState::Cancelled,
            })
        );
    }

    #[test]
    fn reads_the_tool_call_an_update_belongs_to() {
        let decoded = inspect(
            METHOD,
            &json!({
                "sessionId": "child",
                "update": {
                    "sessionUpdate": "agent_message_chunk",
                    "content": { "type": "text", "text": "looking" },
                    "_meta": { "claudeCode": { "parentToolUseId": "toolu_1" } },
                },
            }),
        )
        .expect("an attributed update should be reported")
        .expect("the attribution should decode");

        assert_eq!(
            decoded,
            InboundUpdate::Attribution(SubagentAttribution {
                session_id: acp::SessionId::new("child"),
                parent_tool_call_id: acp::ToolCallId::new("toolu_1"),
            })
        );
    }

    #[test]
    fn ignores_unattributed_updates() {
        assert!(
            inspect(
                METHOD,
                &json!({
                    "sessionId": "parent",
                    "update": {
                        "sessionUpdate": "agent_message_chunk",
                        "content": { "type": "text", "text": "hi" },
                    },
                }),
            )
            .is_none(),
            "an ordinary update must fall through to the typed handlers untouched"
        );
    }

    #[test]
    fn ignores_other_methods() {
        assert!(
            inspect(
                "session/request_permission",
                &json!({ "update": { "sessionUpdate": "subagent_spawned" } }),
            )
            .is_none(),
            "only session notifications carry subagent updates"
        );
    }

    #[test]
    fn reports_a_malformed_payload() {
        let decoded = inspect(
            METHOD,
            &json!({
                "sessionId": "parent",
                "update": {
                    "sessionUpdate": "subagent_spawned",
                    "name": "Explore",
                },
            }),
        )
        .expect("the discriminant should match");

        assert!(
            decoded.is_err(),
            "a spawn without a child session id is malformed, not unrelated"
        );
    }
}
