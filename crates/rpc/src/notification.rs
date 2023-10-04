use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

// An integer indicating a type of notification. The variants' numerical
// values are stored in the database, so they should never be removed
// or changed.
#[repr(i32)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString, Display)]
pub enum NotificationKind {
    ContactRequest = 0,
    ChannelInvitation = 1,
    ChannelMessageMention = 2,
}

pub enum Notification {
    ContactRequest {
        requester_id: u64,
    },
    ChannelInvitation {
        inviter_id: u64,
        channel_id: u64,
    },
    ChannelMessageMention {
        sender_id: u64,
        channel_id: u64,
        message_id: u64,
    },
}

#[derive(Copy, Clone)]
pub enum NotificationEntityKind {
    User,
    Channel,
    ChannelMessage,
}

impl Notification {
    pub fn from_fields(kind: NotificationKind, entity_ids: [Option<u64>; 3]) -> Option<Self> {
        use NotificationKind::*;

        Some(match kind {
            ContactRequest => Self::ContactRequest {
                requester_id: entity_ids[0]?,
            },
            ChannelInvitation => Self::ChannelInvitation {
                inviter_id: entity_ids[0]?,
                channel_id: entity_ids[1]?,
            },
            ChannelMessageMention => Self::ChannelMessageMention {
                sender_id: entity_ids[0]?,
                channel_id: entity_ids[1]?,
                message_id: entity_ids[2]?,
            },
        })
    }

    pub fn to_fields(&self) -> (NotificationKind, [Option<(u64, NotificationEntityKind)>; 3]) {
        use NotificationKind::*;

        match self {
            Self::ContactRequest { requester_id } => (
                ContactRequest,
                [
                    Some((*requester_id, NotificationEntityKind::User)),
                    None,
                    None,
                ],
            ),

            Self::ChannelInvitation {
                inviter_id,
                channel_id,
            } => (
                ChannelInvitation,
                [
                    Some((*inviter_id, NotificationEntityKind::User)),
                    Some((*channel_id, NotificationEntityKind::User)),
                    None,
                ],
            ),

            Self::ChannelMessageMention {
                sender_id,
                channel_id,
                message_id,
            } => (
                ChannelMessageMention,
                [
                    Some((*sender_id, NotificationEntityKind::User)),
                    Some((*channel_id, NotificationEntityKind::ChannelMessage)),
                    Some((*message_id, NotificationEntityKind::Channel)),
                ],
            ),
        }
    }
}

impl NotificationKind {
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    pub fn from_i32(i: i32) -> Option<Self> {
        Self::iter().find(|kind| *kind as i32 == i)
    }
}
