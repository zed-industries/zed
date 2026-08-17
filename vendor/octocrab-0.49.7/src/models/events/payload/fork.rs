use crate::models::Repository;
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::ForkEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForkEventPayload {
    /// The fork.
    pub forkee: Repository,
}

#[cfg(test)]
mod test {
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/fork_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::ForkEvent(ref payload)) = event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.forkee.id.0, 334843423);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
