use crate::db::{ChannelId, RoomId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RoomId,
    pub live_kit_room: String,
    pub channel_id: Option<ChannelId>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::room_participant::Entity")]
    RoomParticipant,
    #[sea_orm(has_many = "super::project::Entity")]
    Project,
    #[sea_orm(has_many = "super::follower::Entity")]
    Follower,
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id"
    )]
    Channel,
}

impl Related<super::room_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoomParticipant.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::follower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Follower.def()
    }
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
