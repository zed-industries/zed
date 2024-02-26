use crate::Result;
use rpc::proto;
use sea_orm::{entity::prelude::*, DbErr};
use serde::{Deserialize, Serialize};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
            DeriveValueType,
        )]
        #[allow(missing_docs)]
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[allow(unused)]
            #[allow(missing_docs)]
            pub const MAX: Self = Self(i32::MAX);

            #[allow(unused)]
            #[allow(missing_docs)]
            pub fn from_proto(value: u64) -> Self {
                Self(value as i32)
            }

            #[allow(unused)]
            #[allow(missing_docs)]
            pub fn to_proto(self) -> u64 {
                self.0 as u64
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl sea_orm::TryFromU64 for $name {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                Ok(Self(n.try_into().map_err(|_| {
                    DbErr::ConvertFromU64(concat!(
                        "error converting ",
                        stringify!($name),
                        " to u64"
                    ))
                })?))
            }
        }

        impl sea_orm::sea_query::Nullable for $name {
            fn null() -> Value {
                Value::Int(None)
            }
        }
    };
}

id_type!(BufferId);
id_type!(AccessTokenId);
id_type!(ChannelChatParticipantId);
id_type!(ChannelId);
id_type!(ChannelMemberId);
id_type!(MessageId);
id_type!(ContactId);
id_type!(FollowerId);
id_type!(RoomId);
id_type!(RoomParticipantId);
id_type!(ProjectId);
id_type!(ProjectCollaboratorId);
id_type!(ReplicaId);
id_type!(ServerId);
id_type!(SignupId);
id_type!(UserId);
id_type!(ChannelBufferCollaboratorId);
id_type!(FlagId);
id_type!(ExtensionId);
id_type!(NotificationId);
id_type!(NotificationKindId);
id_type!(HostedProjectId);

/// ChannelRole gives you permissions for both channels and calls.
#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Default, Hash)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum ChannelRole {
    /// Admin can read/write and change permissions.
    #[sea_orm(string_value = "admin")]
    Admin,
    /// Member can read/write, but not change pemissions.
    #[sea_orm(string_value = "member")]
    #[default]
    Member,
    /// Talker can read, but not write.
    /// They can use microphones and the channel chat
    #[sea_orm(string_value = "talker")]
    Talker,
    /// Guest can read, but not write.
    /// They can not use microphones but can use the chat.
    #[sea_orm(string_value = "guest")]
    Guest,
    /// Banned may not read.
    #[sea_orm(string_value = "banned")]
    Banned,
}

impl ChannelRole {
    /// Returns true if this role is more powerful than the other role.
    pub fn should_override(&self, other: Self) -> bool {
        use ChannelRole::*;
        match self {
            Admin => matches!(other, Member | Banned | Talker | Guest),
            Member => matches!(other, Banned | Talker | Guest),
            Talker => matches!(other, Guest),
            Banned => matches!(other, Guest),
            Guest => false,
        }
    }

    /// Returns the maximal role between the two
    pub fn max(&self, other: Self) -> Self {
        if self.should_override(other) {
            *self
        } else {
            other
        }
    }

    pub fn can_see_channel(&self, visibility: ChannelVisibility) -> bool {
        use ChannelRole::*;
        match self {
            Admin | Member => true,
            Guest | Talker => visibility == ChannelVisibility::Public,
            Banned => false,
        }
    }

    /// True if the role allows access to all descendant channels
    pub fn can_see_all_descendants(&self) -> bool {
        use ChannelRole::*;
        match self {
            Admin | Member => true,
            Guest | Talker | Banned => false,
        }
    }

    /// True if the role only allows access to public descendant channels
    pub fn can_only_see_public_descendants(&self) -> bool {
        use ChannelRole::*;
        match self {
            Guest | Talker => true,
            Admin | Member | Banned => false,
        }
    }

    /// True if the role can share screen/microphone/projects into rooms.
    pub fn can_use_microphone(&self) -> bool {
        use ChannelRole::*;
        match self {
            Admin | Member | Talker => true,
            Guest | Banned => false,
        }
    }

    /// True if the role can edit shared projects.
    pub fn can_edit_projects(&self) -> bool {
        use ChannelRole::*;
        match self {
            Admin | Member => true,
            Talker | Guest | Banned => false,
        }
    }

    /// True if the role can read shared projects.
    pub fn can_read_projects(&self) -> bool {
        use ChannelRole::*;
        match self {
            Admin | Member | Guest | Talker => true,
            Banned => false,
        }
    }

    pub fn requires_cla(&self) -> bool {
        use ChannelRole::*;
        match self {
            Admin | Member => true,
            Banned | Guest | Talker => false,
        }
    }
}

impl From<proto::ChannelRole> for ChannelRole {
    fn from(value: proto::ChannelRole) -> Self {
        match value {
            proto::ChannelRole::Admin => ChannelRole::Admin,
            proto::ChannelRole::Member => ChannelRole::Member,
            proto::ChannelRole::Talker => ChannelRole::Talker,
            proto::ChannelRole::Guest => ChannelRole::Guest,
            proto::ChannelRole::Banned => ChannelRole::Banned,
        }
    }
}

impl Into<proto::ChannelRole> for ChannelRole {
    fn into(self) -> proto::ChannelRole {
        match self {
            ChannelRole::Admin => proto::ChannelRole::Admin,
            ChannelRole::Member => proto::ChannelRole::Member,
            ChannelRole::Talker => proto::ChannelRole::Talker,
            ChannelRole::Guest => proto::ChannelRole::Guest,
            ChannelRole::Banned => proto::ChannelRole::Banned,
        }
    }
}

impl Into<i32> for ChannelRole {
    fn into(self) -> i32 {
        let proto: proto::ChannelRole = self.into();
        proto.into()
    }
}

/// ChannelVisibility controls whether channels are public or private.
#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Default, Hash)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum ChannelVisibility {
    /// Public channels are visible to anyone with the link. People join with the Guest role by default.
    #[sea_orm(string_value = "public")]
    Public,
    /// Members channels are only visible to members of this channel or its parents.
    #[sea_orm(string_value = "members")]
    #[default]
    Members,
}

impl From<proto::ChannelVisibility> for ChannelVisibility {
    fn from(value: proto::ChannelVisibility) -> Self {
        match value {
            proto::ChannelVisibility::Public => ChannelVisibility::Public,
            proto::ChannelVisibility::Members => ChannelVisibility::Members,
        }
    }
}

impl Into<proto::ChannelVisibility> for ChannelVisibility {
    fn into(self) -> proto::ChannelVisibility {
        match self {
            ChannelVisibility::Public => proto::ChannelVisibility::Public,
            ChannelVisibility::Members => proto::ChannelVisibility::Members,
        }
    }
}

impl Into<i32> for ChannelVisibility {
    fn into(self) -> i32 {
        let proto: proto::ChannelVisibility = self.into();
        proto.into()
    }
}
