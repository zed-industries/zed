use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use strum::{EnumVariantNames, IntoStaticStr, VariantNames as _};

const KIND: &'static str = "kind";
const ACTOR_ID: &'static str = "actor_id";

/// A notification that can be stored, associated with a given user.
///
/// This struct is stored in the collab database as JSON, so it shouldn't be
/// changed in a backward-incompatible way.
///
/// For example, when renaming a variant, add a serde alias for the old name.
#[derive(Debug, Clone, PartialEq, Eq, EnumVariantNames, IntoStaticStr, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Notification {
    ContactRequest {
        actor_id: u64,
    },
    ContactRequestAccepted {
        actor_id: u64,
    },
    ChannelInvitation {
        actor_id: u64,
        channel_id: u64,
    },
    ChannelMessageMention {
        actor_id: u64,
        channel_id: u64,
        message_id: u64,
    },
}

/// The representation of a notification that is stored in the database and
/// sent over the wire.
#[derive(Debug)]
pub struct AnyNotification {
    pub kind: Cow<'static, str>,
    pub actor_id: Option<u64>,
    pub content: String,
}

impl Notification {
    pub fn to_any(&self) -> AnyNotification {
        let kind: &'static str = self.into();
        let mut value = serde_json::to_value(self).unwrap();
        let mut actor_id = None;
        if let Some(value) = value.as_object_mut() {
            value.remove("kind");
            actor_id = value
                .remove("actor_id")
                .and_then(|value| Some(value.as_i64()? as u64));
        }
        AnyNotification {
            kind: Cow::Borrowed(kind),
            actor_id,
            content: serde_json::to_string(&value).unwrap(),
        }
    }

    pub fn from_any(notification: &AnyNotification) -> Option<Self> {
        let mut value = serde_json::from_str::<Value>(&notification.content).ok()?;
        let object = value.as_object_mut()?;
        object.insert(KIND.into(), notification.kind.to_string().into());
        if let Some(actor_id) = notification.actor_id {
            object.insert(ACTOR_ID.into(), actor_id.into());
        }
        serde_json::from_value(value).ok()
    }

    pub fn all_kinds() -> &'static [&'static str] {
        Self::VARIANTS
    }
}

#[test]
fn test_notification() {
    // Notifications can be serialized and deserialized.
    for notification in [
        Notification::ContactRequest { actor_id: 1 },
        Notification::ContactRequestAccepted { actor_id: 2 },
        Notification::ChannelInvitation {
            actor_id: 0,
            channel_id: 100,
        },
        Notification::ChannelMessageMention {
            actor_id: 200,
            channel_id: 30,
            message_id: 1,
        },
    ] {
        let serialized = notification.to_any();
        let deserialized = Notification::from_any(&serialized).unwrap();
        assert_eq!(deserialized, notification);
    }

    // When notifications are serialized, the `kind` and `actor_id` fields are
    // stored separately, and do not appear redundantly in the JSON.
    let notification = Notification::ContactRequest { actor_id: 1 };
    assert_eq!(notification.to_any().content, "{}");
}
