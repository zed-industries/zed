use crate::db::{ChannelId, ChannelMemberId, ChannelRole, UserId, channel_member};
use rpc::proto;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channel_members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ChannelMemberId,
    pub channel_id: ChannelId,
    pub user_id: UserId,
    pub accepted: bool,
    pub role: ChannelRole,
}

impl From<Model> for proto::ChannelMember {
    fn from(member: Model) -> Self {
        Self {
            role: member.role.into(),
            user_id: member.user_id.to_proto(),
            kind: if member.accepted {
                proto::channel_member::Kind::Member
            } else {
                proto::channel_member::Kind::Invitee
            }
            .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id"
    )]
    Channel,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[derive(Debug)]
pub struct UserToChannel;

impl Linked for UserToChannel {
    type FromEntity = super::user::Entity;

    type ToEntity = super::channel::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            channel_member::Relation::User.def().rev(),
            channel_member::Relation::Channel.def(),
        ]
    }
}
