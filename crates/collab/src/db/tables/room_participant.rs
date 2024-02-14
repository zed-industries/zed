use crate::db::{ChannelRole, ProjectId, RoomId, RoomParticipantId, ServerId, UserId};
use rpc::ConnectionId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "room_participants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RoomParticipantId,
    pub room_id: RoomId,
    pub user_id: UserId,
    pub answering_connection_id: Option<i32>,
    pub answering_connection_server_id: Option<ServerId>,
    pub answering_connection_lost: bool,
    pub location_kind: Option<i32>,
    pub location_project_id: Option<ProjectId>,
    pub initial_project_id: Option<ProjectId>,
    pub calling_user_id: UserId,
    pub calling_connection_id: i32,
    pub calling_connection_server_id: Option<ServerId>,
    pub participant_index: Option<i32>,
    pub role: Option<ChannelRole>,
}

impl Model {
    pub fn answering_connection(&self) -> Option<ConnectionId> {
        Some(ConnectionId {
            owner_id: self.answering_connection_server_id?.0 as u32,
            id: self.answering_connection_id? as u32,
        })
    }
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
