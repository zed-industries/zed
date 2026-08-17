use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::CreateEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateEventPayload {
    // a null ref will occur on the initial create event
    pub r#ref: Option<String>,
    pub ref_type: String,
    pub master_branch: String,
    pub description: Option<String>,
    pub pusher_type: String,
}

#[cfg(test)]
mod test {
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_create_event_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap().specific.unwrap();
        match payload {
            EventPayload::CreateEvent(payload) => {
                assert_eq!(payload.r#ref, Some("url-normalisation".to_string()));
                assert_eq!(payload.ref_type, "branch");
                assert_eq!(payload.master_branch, "main");
                assert_eq!(
                    payload.description,
                    Some("Your friendly URL vetting service".to_string())
                );
                assert_eq!(payload.pusher_type, "user");
            }
            _ => panic!("unexpected event deserialized"),
        }
    }

    #[test]
    fn should_deserialize_null_description_as_none() {
        let json =
            include_str!("../../../../tests/resources/create_event_with_null_description.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap().specific.unwrap();
        match payload {
            EventPayload::CreateEvent(payload) => assert_eq!(payload.description, None),
            _ => panic!("unexpected event deserialized"),
        }
    }
}
