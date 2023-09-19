use crate::db::ChannelId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ChannelId,
    pub name: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::room::Entity")]
    Room,
    #[sea_orm(has_one = "super::buffer::Entity")]
    Buffer,
    #[sea_orm(has_many = "super::channel_member::Entity")]
    Member,
    #[sea_orm(has_many = "super::channel_buffer_collaborator::Entity")]
    BufferCollaborators,
    #[sea_orm(has_many = "super::channel_chat_participant::Entity")]
    ChatParticipants,
}

impl Related<super::channel_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::buffer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buffer.def()
    }
}

impl Related<super::channel_buffer_collaborator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BufferCollaborators.def()
    }
}

impl Related<super::channel_chat_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatParticipants.def()
    }
}
