use crate::models::Author;
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::MemberEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MemberEventPayload {
    /// The action this event represents.
    pub action: MemberEventAction,
    /// The user this event corresponds to.
    pub member: Author,
    /// Only available on webhooks.
    ///
    /// Changes to the collaborator permissions.
    pub changes: Option<MemberEventChanges>,
}

/// The change which occurred.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum MemberEventChanges {
    Permission(MemberEventChangesTo),
}

/// The new permission given to the collaborator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MemberEventChangesTo {
    /// The optional previous permission.
    pub from: Option<String>,
    /// The new permission.
    pub to: String,
}

/// The action this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum MemberEventAction {
    Added,
    /// Only available on webhooks.
    Edited,
}

#[cfg(test)]
mod test {
    use super::{MemberEventAction, MemberEventChanges, MemberEventChangesTo};
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""added""#, MemberEventAction::Added),
            (r#""edited""#, MemberEventAction::Edited),
        ];
        for (action_str, action) in actions {
            let deserialized = serde_json::from_str(action_str).unwrap();
            assert_eq!(action, deserialized);
        }
    }

    #[test]
    fn should_deserialize_permission_changes() {
        let json = json!({
            "permission": {
                "from": "triage",
                "to": "write"
            }
        });
        let deserialized = serde_json::from_value::<MemberEventChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            MemberEventChanges::Permission(MemberEventChangesTo {
                from: Some("triage".to_owned()),
                to: "write".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/member_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::MemberEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, MemberEventAction::Added);
            assert_eq!(payload.member.id.0, 58522265);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
