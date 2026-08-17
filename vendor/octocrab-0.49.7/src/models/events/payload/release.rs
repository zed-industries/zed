use crate::models::repos::Release;
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::ReleaseEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReleaseEventPayload {
    /// The action this event represents.
    pub action: ReleaseEventAction,
    /// The release this event corresponds to.
    pub release: Release,
    /// The changes to body or name if this event is of type [`ReleaseEventAction::Edited`].
    pub changes: Option<ReleaseEventChanges>,
}

/// The change which occurred in an event of type [`ReleaseEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReleaseEventChanges {
    Body(ReleaseEventChangesFrom),
    Name(ReleaseEventChangesFrom),
}

/// The previous value of the item (either the body or title) of a release which has changed. Only
/// available in an event of type [`ReleaseEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReleaseEventChangesFrom {
    pub from: String,
}

/// The action on a release this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReleaseEventAction {
    Published,
    Edited,
}

#[cfg(test)]
mod test {
    use crate::models::events::{
        payload::{EventPayload, ReleaseEventAction},
        Event,
    };

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/release_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::ReleaseEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, ReleaseEventAction::Published);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
