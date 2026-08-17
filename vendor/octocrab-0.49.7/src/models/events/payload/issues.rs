use crate::models::{issues::Issue, Author, Label};
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::IssuesEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuesEventPayload {
    /// The action this event represents.
    pub action: IssuesEventAction,
    /// The issue this event corresponds to.
    pub issue: Issue,
    /// The changes to body or title if this event is of type [`IssuesEventAction::Edited`].
    pub changes: Option<IssuesEventChanges>,
    /// The optional user who was assigned or unassigned from the issue.
    ///
    /// Set when they type is [`IssuesEventAction::Assigned`] or
    /// [`IssuesEventAction::Unassigned`].
    pub assignee: Option<Author>,
    /// The optional label added or removed from the issue.
    ///
    /// Set when the type is [`IssuesEventAction::Labeled`] or
    /// [`IssuesEventAction::Unlabeled`].
    pub label: Option<Label>,
}

/// The change which occurred in an event of type [`IssuesEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssuesEventChanges {
    Title(IssuesEventChangesFrom),
    Body(IssuesEventChangesFrom),
}

/// The previous value of the item (either the body or title) of an issue which has changed. Only
/// available in an event of type [`IssuesEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuesEventChangesFrom {
    pub from: String,
}

/// The action on an issue this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssuesEventAction {
    Opened,
    Closed,
    Reopened,
    /// Only available on webhook events.
    Edited,
    /// Only available on webhook events.
    Assigned,
    /// Only available on webhook events.
    Unassigned,
    /// Only available on webhook events.
    Labeled,
    /// Only available on webhook events.
    Unlabeled,
}

#[cfg(test)]
mod test {
    use super::{IssuesEventAction, IssuesEventChanges, IssuesEventChangesFrom};
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""opened""#, IssuesEventAction::Opened),
            (r#""closed""#, IssuesEventAction::Closed),
            (r#""edited""#, IssuesEventAction::Edited),
            (r#""reopened""#, IssuesEventAction::Reopened),
            (r#""assigned""#, IssuesEventAction::Assigned),
            (r#""unassigned""#, IssuesEventAction::Unassigned),
            (r#""labeled""#, IssuesEventAction::Labeled),
            (r#""unlabeled""#, IssuesEventAction::Unlabeled),
        ];
        for (action_str, action) in actions {
            let deserialized = serde_json::from_str(action_str).unwrap();
            assert_eq!(action, deserialized);
        }
    }

    #[test]
    fn should_deserialize_title_changes() {
        let json = json!({
            "title": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<IssuesEventChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            IssuesEventChanges::Title(IssuesEventChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_body_changes() {
        let json = json!({
            "body": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<IssuesEventChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            IssuesEventChanges::Body(IssuesEventChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/issues_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::IssuesEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, IssuesEventAction::Opened);
            assert_eq!(payload.issue.id.0, 786747990);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
