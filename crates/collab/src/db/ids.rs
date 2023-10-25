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
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[allow(unused)]
            pub const MAX: Self = Self(i32::MAX);

            #[allow(unused)]
            pub fn from_proto(value: u64) -> Self {
                Self(value as i32)
            }

            #[allow(unused)]
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
id_type!(NotificationId);
id_type!(NotificationKindId);

#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Default)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum ChannelRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "member")]
    #[default]
    Member,
    #[sea_orm(string_value = "guest")]
    Guest,
    #[sea_orm(string_value = "banned")]
    Banned,
}

impl ChannelRole {
    pub fn should_override(&self, other: Self) -> bool {
        use ChannelRole::*;
        match self {
            Admin => matches!(other, Member | Banned | Guest),
            Member => matches!(other, Banned | Guest),
            Banned => matches!(other, Guest),
            Guest => false,
        }
    }

    pub fn max(&self, other: Self) -> Self {
        if self.should_override(other) {
            *self
        } else {
            other
        }
    }
}

impl From<proto::ChannelRole> for ChannelRole {
    fn from(value: proto::ChannelRole) -> Self {
        match value {
            proto::ChannelRole::Admin => ChannelRole::Admin,
            proto::ChannelRole::Member => ChannelRole::Member,
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

#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Default, Hash)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum ChannelVisibility {
    #[sea_orm(string_value = "public")]
    Public,
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
