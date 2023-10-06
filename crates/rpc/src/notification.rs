use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

// An integer indicating a type of notification. The variants' numerical
// values are stored in the database, so they should never be removed
// or changed.
#[repr(i32)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString, Display)]
pub enum NotificationKind {
    ContactRequest = 0,
    ContactRequestAccepted = 1,
    ChannelInvitation = 2,
    ChannelMessageMention = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Notification {
    ContactRequest {
        requester_id: u64,
    },
    ContactRequestAccepted {
        contact_id: u64,
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

impl Notification {
    /// Load this notification from its generic representation, which is
    /// used to represent it in the database, and in the wire protocol.
    ///
    /// The order in which a given notification type's fields are listed must
    /// match the order they're listed in the `to_parts` method, and it must
    /// not change, because they're stored in that order in the database.
    pub fn from_parts(kind: NotificationKind, entity_ids: [Option<u64>; 3]) -> Option<Self> {
        use NotificationKind::*;
        Some(match kind {
            ContactRequest => Self::ContactRequest {
                requester_id: entity_ids[0]?,
            },

            ContactRequestAccepted => Self::ContactRequest {
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

    /// Convert this notification into its generic representation, which is
    /// used to represent it in the database, and in the wire protocol.
    ///
    /// The order in which a given notification type's fields are listed must
    /// match the order they're listed in the `from_parts` method, and it must
    /// not change, because they're stored in that order in the database.
    pub fn to_parts(&self) -> (NotificationKind, [Option<u64>; 3]) {
        use NotificationKind::*;
        match self {
            Self::ContactRequest { requester_id } => {
                (ContactRequest, [Some(*requester_id), None, None])
            }

            Self::ContactRequestAccepted { contact_id } => {
                (ContactRequest, [Some(*contact_id), None, None])
            }

            Self::ChannelInvitation {
                inviter_id,
                channel_id,
            } => (
                ChannelInvitation,
                [Some(*inviter_id), Some(*channel_id), None],
            ),

            Self::ChannelMessageMention {
                sender_id,
                channel_id,
                message_id,
            } => (
                ChannelMessageMention,
                [Some(*sender_id), Some(*channel_id), Some(*message_id)],
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
