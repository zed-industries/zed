use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::WatchEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WatchEventPayload {
    /// The action that was performed.
    pub action: WatchEventAction,
}

/// The action that was performed as part of the [`super::EventPayload::WatchEvent`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WatchEventAction {
    Started,
}

#[cfg(test)]
mod test {
    use crate::models::events::{
        payload::{EventPayload, WatchEventAction},
        Event,
    };

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/watch_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::WatchEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.action, WatchEventAction::Started);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
