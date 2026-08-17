use serde::{Deserialize, Serialize};

use crate::models::PushId;

/// The payload in a [`super::EventPayload::PushEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PushEventPayload {
    pub push_id: PushId,
    pub r#ref: String,
    pub head: String,
    pub before: String,
}

#[cfg(test)]
mod test {
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_push_event_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/push_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap().specific.unwrap();
        match payload {
            EventPayload::PushEvent(payload) => {
                assert_eq!(payload.push_id.0, 6080608029);
                assert_eq!(payload.r#ref, "refs/heads/master");
                assert_eq!(payload.head, "eb1a60c03544dcea290f2d57bb66ae188ce25778");
                assert_eq!(payload.before, "9b2afb3a8e03fb30cc09e5efb64823bde802cf59");
            }
            _ => panic!("unexpected event deserialized"),
        }
    }
}
