use crate::models::pulls::{PullRequest, Review};
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::PullRequestReviewEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewEventPayload {
    /// The action this event represents.
    pub action: PullRequestReviewEventAction,
    /// The pull request this event corresponds to.
    pub pull_request: PullRequest,
    /// The review that was affected.
    pub review: Review,
}

/// The action on a pull request review this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PullRequestReviewEventAction {
    Created,
}

/// The change which occurred in an event of type [`PullRequestReviewEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PullRequestReviewChanges {
    Body(PullRequestReviewChangesFrom),
}

/// The previous value of the body of a review which has changed. Only
/// available in an event of type [`PullRequestReviewEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub struct PullRequestReviewChangesFrom {
    pub from: String,
}

#[cfg(test)]
mod test {
    use super::{
        PullRequestReviewChanges, PullRequestReviewChangesFrom, PullRequestReviewEventAction,
    };
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![(r#""created""#, PullRequestReviewEventAction::Created)];
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
        let deserialized = serde_json::from_value::<PullRequestReviewChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            PullRequestReviewChanges::Body(PullRequestReviewChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/pull_request_review_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::PullRequestReviewEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.pull_request.id.0, 1237933052);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
