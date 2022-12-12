use super::{ProjectId, RoomId, RoomParticipantId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "room_participants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RoomParticipantId,
    pub room_id: RoomId,
    pub user_id: UserId,
    pub answering_connection_id: Option<i32>,
    pub answering_connection_epoch: Option<Uuid>,
    pub answering_connection_lost: bool,
    pub location_kind: Option<i32>,
    pub location_project_id: Option<ProjectId>,
    pub initial_project_id: Option<ProjectId>,
    pub calling_user_id: UserId,
    pub calling_connection_id: i32,
    pub calling_connection_epoch: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
