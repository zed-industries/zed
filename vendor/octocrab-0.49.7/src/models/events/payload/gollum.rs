use serde::{Deserialize, Serialize};
use url::Url;

/// The payload in a [`super::EventPayload::GollumEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GollumEventPayload {
    /// The pages that were updated.
    pub pages: Vec<GollumEventPage>,
}

/// A page in a [`GollumEventPayload`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GollumEventPage {
    /// The name of the page.
    pub page_name: String,
    /// The title of the page.
    pub title: String,
    /// The action performed on the page.
    pub action: GollumEventPageAction,
    /// The latest commit SHA of the page.
    pub sha: String,
    /// Url to the page.
    pub html_url: Url,
}

/// The action performed on a given page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum GollumEventPageAction {
    Created,
    Edited,
}

#[cfg(test)]
mod test {
    use super::GollumEventPageAction;
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""created""#, GollumEventPageAction::Created),
            (r#""edited""#, GollumEventPageAction::Edited),
        ];
        for (action_str, action) in actions {
            let deserialized = serde_json::from_str(action_str).unwrap();
            assert_eq!(action, deserialized);
        }
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/gollum_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::GollumEvent(ref payload)) =
            event.payload.as_ref().unwrap().specific
        {
            assert_eq!(payload.pages[0].page_name, "Home");
            assert_eq!(payload.pages[0].title, "Home");
            assert_eq!(payload.pages[0].action, GollumEventPageAction::Created);
            assert_eq!(
                payload.pages[0].sha,
                "738b45139cbf06c11f3013e4b2b1a1ad370696ca"
            );
            assert_eq!(
                payload.pages[0].html_url.to_string(),
                "https://github.com/wayofthepie/test-events/wiki/Home"
            );
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
