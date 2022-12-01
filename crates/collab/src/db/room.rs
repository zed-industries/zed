use super::RoomId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RoomId,
    pub live_kit_room: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::room_participant::Entity")]
    RoomParticipant,
    #[sea_orm(has_many = "super::project::Entity")]
    Project,
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

impl ActiveModelBehavior for ActiveModel {}
