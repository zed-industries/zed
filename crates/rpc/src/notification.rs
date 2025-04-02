use crate::proto;
use serde::{Deserialize, Serialize};
use serde_json::{Value, map};
use strum::VariantNames;

const KIND: &str = "kind";
const ENTITY_ID: &str = "entity_id";

/// A notification that can be stored, associated with a given recipient.
///
/// This struct is stored in the collab database as JSON, so it shouldn't be
/// changed in a backward-incompatible way. For example, when renaming a
/// variant, add a serde alias for the old name.
///
/// Most notification types have a special field which is aliased to
/// `entity_id`. This field is stored in its own database column, and can
/// be used to query the notification.
#[derive(Debug, Clone, PartialEq, Eq, VariantNames, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Notification {
    ContactRequest {
        #[serde(rename = "entity_id")]
        sender_id: u64,
    },
    ContactRequestAccepted {
        #[serde(rename = "entity_id")]
        responder_id: u64,
    },
    ChannelInvitation {
        #[serde(rename = "entity_id")]
        channel_id: u64,
        channel_name: String,
        inviter_id: u64,
    },
    ChannelMessageMention {
        #[serde(rename = "entity_id")]
        message_id: u64,
        sender_id: u64,
        channel_id: u64,
    },
}

impl Notification {
    pub fn to_proto(&self) -> proto::Notification {
        let mut value = serde_json::to_value(self).unwrap();
        let mut entity_id = None;
        let value = value.as_object_mut().unwrap();
        let Some(Value::String(kind)) = value.remove(KIND) else {
            unreachable!("kind is the enum tag")
        };
        if let map::Entry::Occupied(e) = value.entry(ENTITY_ID) {
            if e.get().is_u64() {
                entity_id = e.remove().as_u64();
            }
        }
        proto::Notification {
            kind,
            entity_id,
            content: serde_json::to_string(&value).unwrap(),
            ..Default::default()
        }
    }

    pub fn from_proto(notification: &proto::Notification) -> Option<Self> {
        let mut value = serde_json::from_str::<Value>(&notification.content).ok()?;
        let object = value.as_object_mut()?;
        object.insert(KIND.into(), notification.kind.to_string().into());
        if let Some(entity_id) = notification.entity_id {
            object.insert(ENTITY_ID.into(), entity_id.into());
        }
        serde_json::from_value(value).ok()
    }

    pub fn all_variant_names() -> &'static [&'static str] {
        Self::VARIANTS
    }
}

#[cfg(test)]
mod tests {
    use crate::Notification;

    #[test]
    fn test_notification() {
        // Notifications can be serialized and deserialized.
        for notification in [
            Notification::ContactRequest { sender_id: 1 },
            Notification::ContactRequestAccepted { responder_id: 2 },
            Notification::ChannelInvitation {
                channel_id: 100,
                channel_name: "the-channel".into(),
                inviter_id: 50,
            },
            Notification::ChannelMessageMention {
                sender_id: 200,
                channel_id: 30,
                message_id: 1,
            },
        ] {
            let message = notification.to_proto();
            let deserialized = Notification::from_proto(&message).unwrap();
            assert_eq!(deserialized, notification);
        }

        // When notifications are serialized, the `kind` and `actor_id` fields are
        // stored separately, and do not appear redundantly in the JSON.
        let notification = Notification::ContactRequest { sender_id: 1 };
        assert_eq!(notification.to_proto().content, "{}");
    }
}
