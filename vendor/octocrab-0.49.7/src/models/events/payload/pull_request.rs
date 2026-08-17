use crate::models::pulls::PullRequest;
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::PullRequestEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestEventPayload {
    /// The action this event represents.
    pub action: PullRequestEventAction,
    /// The pull request number this event corresponds to.
    pub number: u64,
    /// The pull request this event corresponds to.
    pub pull_request: PullRequest,
    /// The changes to body or title if this event is of type [`PullRequestEventAction::Edited`].
    pub changes: Option<PullRequestChanges>,
}

/// The action on a pull request this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestEventAction {
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
    ReviewRequested,
    /// Only available on webhook events.
    ReviewRequestRemoved,
    /// Only available on webhook events.
    Labeled,
    /// Only available on webhook events.
    Unlabeled,
    /// Only available on webhook events.
    ///
    /// This event is fired when the HEAD ref in a pull request is changed.
    Synchronize,
}

/// The change which occurred in an event of type [`PullRequestEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestChanges {
    pub title: Option<PullRequestEventChangesFrom>,
    pub body: Option<PullRequestEventChangesFrom>,
}

/// The previous value of the item (either the body or title) of a pull request which has changed. Only
/// available in an event of type [`PullRequestEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub struct PullRequestEventChangesFrom {
    pub from: String,
}

#[cfg(test)]
mod test {
    use super::{PullRequestChanges, PullRequestEventAction, PullRequestEventChangesFrom};
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_snake_case() {
        let actions = vec![
            (r#""opened""#, PullRequestEventAction::Opened),
            (r#""closed""#, PullRequestEventAction::Closed),
            (r#""reopened""#, PullRequestEventAction::Reopened),
            (r#""edited""#, PullRequestEventAction::Edited),
            (r#""assigned""#, PullRequestEventAction::Assigned),
            (r#""unassigned""#, PullRequestEventAction::Unassigned),
            (
                r#""review_requested""#,
                PullRequestEventAction::ReviewRequested,
            ),
            (
                r#""review_request_removed""#,
                PullRequestEventAction::ReviewRequestRemoved,
            ),
            (r#""labeled""#, PullRequestEventAction::Labeled),
            (r#""unlabeled""#, PullRequestEventAction::Unlabeled),
            (r#""synchronize""#, PullRequestEventAction::Synchronize),
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
        let deserialized = serde_json::from_value::<PullRequestChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            PullRequestChanges {
                title: Some(PullRequestEventChangesFrom {
                    from: "test".to_owned()
                }),
                body: None,
            },
        );
    }

    #[test]
    fn should_deserialize_body_changes() {
        let json = json!({
            "body": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<PullRequestChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            PullRequestChanges {
                title: None,
                body: Some(PullRequestEventChangesFrom {
                    from: "test".to_owned()
                }),
            },
        );
    }

    #[test]
    fn should_deserialize_empty_changes() {
        let json = json!({});
        let deserialized = serde_json::from_value::<PullRequestChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            PullRequestChanges {
                title: None,
                body: None,
            },
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/pull_request_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::PullRequestEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, PullRequestEventAction::Opened);
            assert_eq!(payload.number, 8);
            assert_eq!(payload.pull_request.id.0, 558121796);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
