use super::{ChannelId, RoomId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ChannelId,
    pub room_id: Option<RoomId>,
    // pub id_path: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::room::Entity")]
    Room,
    #[sea_orm(has_many = "super::channel_member::Entity")]
    Member,
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

// impl Related<super::follower::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Follower.def()
//     }
// }
