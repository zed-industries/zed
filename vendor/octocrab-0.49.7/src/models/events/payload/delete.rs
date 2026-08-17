use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::DeleteEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeleteEventPayload {
    /// The ref which was deleted.
    pub r#ref: String,
    /// The type of the ref which was deleted.
    pub ref_type: String,
}

#[cfg(test)]
mod test {
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/delete_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::DeleteEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.r#ref, "test2");
            assert_eq!(payload.ref_type, "branch");
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
