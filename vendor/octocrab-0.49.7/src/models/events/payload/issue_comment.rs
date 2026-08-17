use crate::models::issues::{Comment, Issue};
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::IssueCommentEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueCommentEventPayload {
    /// The action this event represents.
    pub action: IssueCommentEventAction,
    /// The issue this event corresponds to.
    pub issue: Issue,
    /// The comment this event corresponds to.
    pub comment: Comment,
    /// The changes to the body of the issue comment if
    /// this event is of type [`IssueCommentEventAction::Edited`].
    pub changes: Option<IssueCommentEventChanges>,
}

/// The change which occurred in an event of type [`IssueCommentEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssueCommentEventChanges {
    Body(IssueCommentEventChangesFrom),
}

/// The previous value of the body of the issue comment which has changed. Only
/// available in an event of type [`IssueCommentEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueCommentEventChangesFrom {
    pub from: String,
}

/// The action on an issue comment this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssueCommentEventAction {
    Created,
    /// Only available on webhooks.
    Deleted,
    /// Only available on webhooks.
    Edited,
}

#[cfg(test)]
mod test {
    use super::{IssueCommentEventAction, IssueCommentEventChanges, IssueCommentEventChangesFrom};
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""created""#, IssueCommentEventAction::Created),
            (r#""edited""#, IssueCommentEventAction::Edited),
            (r#""deleted""#, IssueCommentEventAction::Deleted),
        ];
        for (action_str, action) in actions {
            let deserialized = serde_json::from_str(action_str).unwrap();
            assert_eq!(action, deserialized);
        }
    }

    #[test]
    fn should_deserialize_body_changes() {
        let json = json!({
            "body": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<IssueCommentEventChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            IssueCommentEventChanges::Body(IssueCommentEventChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/issue_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::IssueCommentEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, IssueCommentEventAction::Created);
            assert_eq!(payload.issue.id.0, 785981862);
            assert_eq!(payload.comment.id.0, 760203693);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
