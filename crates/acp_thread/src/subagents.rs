use agent_client_protocol::schema::{MaybeUndefined, v1 as acp_v1, v2 as acp_v2};
use anyhow::Result;
use gpui::SharedString;

/// What a parent ACP session has reported about a subagent session it owns.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubagentAssociation {
    /// The parent's label for the subagent, which can differ from the subagent's own title.
    pub title: Option<SharedString>,
    pub description: Option<SharedString>,
    /// Whether Zed may cancel the subagent's current work.
    pub can_cancel: bool,
}

impl SubagentAssociation {
    /// Omitted fields stay unchanged and `null` clears them. `state` is not stored here:
    /// it is the subagent's activity, which lives on the subagent's own thread.
    pub fn apply(&mut self, update: &acp_v2::SubagentUpdate) {
        if let Some(title) = update.title.as_opt_ref() {
            self.title = title.map(|title| title.clone().into());
        }
        if let Some(description) = update.description.as_opt_ref() {
            self.description = description.map(|description| description.clone().into());
        }
        if let Some(capabilities) = update.capabilities.as_opt_ref() {
            self.can_cancel =
                capabilities.is_some_and(|capabilities| capabilities.cancel.is_some());
        }
    }
}

/// Association metadata is not carried over: Zed does not use it, and v1 replaces it
/// where v2 patches it by key.
pub fn update_from_v1(update: acp_v1::SubagentUpdate) -> Result<acp_v2::SubagentUpdate> {
    let state = match update.state {
        MaybeUndefined::Undefined => MaybeUndefined::Undefined,
        MaybeUndefined::Null => MaybeUndefined::Null,
        MaybeUndefined::Value(state) => MaybeUndefined::Value(state_from_v1(state)?),
    };
    Ok(acp_v2::SubagentUpdate::new(update.session_id.0)
        .title(update.title)
        .description(update.description)
        .capabilities(update.capabilities.map_value(|capabilities| {
            acp_v2::SubagentSessionCapabilities::new()
                .cancel(
                    capabilities
                        .cancel
                        .map(|cancel| acp_v2::SessionCancelCapabilities::new().meta(cancel.meta)),
                )
                .meta(capabilities.meta)
        }))
        .state(state))
}

/// The subagents RFD gives the v1 snapshot the same wire shape as the v2 `state_update`,
/// so converting through JSON keeps unknown states and stop reasons intact.
fn state_from_v1(state: acp_v1::StateUpdate) -> Result<acp_v2::StateUpdate> {
    Ok(serde_json::from_value(serde_json::to_value(state)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn v2_update(value: serde_json::Value) -> acp_v2::SubagentUpdate {
        serde_json::from_value(value).expect("v2 subagent update")
    }

    #[test]
    fn association_fields_follow_patch_semantics() {
        let mut association = SubagentAssociation::default();
        association.apply(&v2_update(json!({
            "sessionId": "child",
            "title": "Investigator",
            "description": "Looks into failures",
            "capabilities": {"cancel": {}},
        })));
        let announced = SubagentAssociation {
            title: Some("Investigator".into()),
            description: Some("Looks into failures".into()),
            can_cancel: true,
        };
        assert_eq!(association, announced);

        association.apply(&v2_update(json!({"sessionId": "child"})));
        assert_eq!(association, announced, "omitted fields are unchanged");

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "title": null,
            "capabilities": {"cancel": null},
        })));
        assert_eq!(
            association,
            SubagentAssociation {
                title: None,
                description: Some("Looks into failures".into()),
                can_cancel: false,
            },
            "null clears a field, and capabilities are replaced as a whole"
        );

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "capabilities": {"cancel": {}},
        })));
        association.apply(&v2_update(
            json!({"sessionId": "child", "capabilities": null}),
        ));
        assert!(!association.can_cancel, "null capabilities disable cancel");
    }

    #[test]
    fn v1_updates_convert_fields_and_state_snapshots() -> Result<()> {
        for (v1, expected) in [
            (
                json!({
                    "sessionId": "child",
                    "title": "Investigator",
                    "description": null,
                    "capabilities": {"cancel": {}},
                    "state": {"state": "running"},
                    "_meta": {"dropped": true},
                }),
                json!({
                    "sessionId": "child",
                    "title": "Investigator",
                    "description": null,
                    "capabilities": {"cancel": {}},
                    "state": {"state": "running"},
                }),
            ),
            (
                json!({
                    "sessionId": "child",
                    "state": {
                        "state": "idle",
                        "stopReason": "error",
                        "error": {"code": -32000, "message": "Authentication required"},
                    },
                }),
                json!({
                    "sessionId": "child",
                    "state": {
                        "state": "idle",
                        "stopReason": "error",
                        "error": {"code": -32000, "message": "Authentication required"},
                    },
                }),
            ),
            (
                json!({"sessionId": "child", "state": {"state": "unknown"}}),
                json!({"sessionId": "child", "state": {"state": "unknown"}}),
            ),
            (
                json!({"sessionId": "child", "state": {"state": "_paused", "reason": "x"}}),
                json!({"sessionId": "child", "state": {"state": "_paused", "reason": "x"}}),
            ),
            (
                json!({"sessionId": "child", "state": null, "capabilities": null}),
                json!({"sessionId": "child", "state": null, "capabilities": null}),
            ),
        ] {
            let converted = update_from_v1(serde_json::from_value(v1.clone())?)?;
            assert_eq!(serde_json::to_value(&converted)?, expected, "{v1}");
        }
        Ok(())
    }
}
