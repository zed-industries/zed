use crate::models::{issues::Comment, pulls::PullRequest};
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::PullRequestReviewCommentEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewCommentEventPayload {
    /// The action this event represents.
    pub action: PullRequestReviewCommentEventAction,
    /// The pull request this event corresponds to.
    pub pull_request: PullRequest,
    /// The comment this event corresponds to.
    pub comment: Comment,
    /// The changes to body or title if this event is of type [`PullRequestReviewCommentEventAction::Edited`].
    pub changes: Option<PullRequestReviewCommentChanges>,
}

/// The action on a pull request review comment this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PullRequestReviewCommentEventAction {
    Created,
    /// Only available on webhook events.
    Edited,
}

/// The change which occurred in an event of type [`PullRequestReviewCommentEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PullRequestReviewCommentChanges {
    Body(PullRequestReviewCommentChangesFrom),
}

/// The previous value of the body of a review comment which has changed. Only
/// available in an event of type [`PullRequestReviewCommentEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub struct PullRequestReviewCommentChangesFrom {
    pub from: String,
}

#[cfg(test)]
mod test {
    use super::{
        PullRequestReviewCommentChanges, PullRequestReviewCommentChangesFrom,
        PullRequestReviewCommentEventAction,
    };
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""created""#, PullRequestReviewCommentEventAction::Created),
            (r#""edited""#, PullRequestReviewCommentEventAction::Edited),
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
        let deserialized = serde_json::from_value::<PullRequestReviewCommentChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            PullRequestReviewCommentChanges::Body(PullRequestReviewCommentChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json =
            include_str!("../../../../tests/resources/pull_request_review_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::PullRequestReviewCommentEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, PullRequestReviewCommentEventAction::Created);
            assert_eq!(payload.pull_request.id.0, 558121796);
            assert_eq!(payload.comment.id.0, 560976245);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
