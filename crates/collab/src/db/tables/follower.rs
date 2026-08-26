use crate::db::{FollowerId, ProjectId, RoomId, ServerId};
use rpc::ConnectionId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "followers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: FollowerId,
    pub room_id: RoomId,
    pub project_id: ProjectId,
    pub leader_connection_server_id: ServerId,
    pub leader_connection_id: i32,
    pub follower_connection_server_id: ServerId,
    pub follower_connection_id: i32,
}

impl Model {
    pub fn leader_connection(&self) -> ConnectionId {
        ConnectionId {
            owner_id: self.leader_connection_server_id.0 as u32,
            id: self.leader_connection_id as u32,
        }
    }

    pub fn follower_connection(&self) -> ConnectionId {
        ConnectionId {
            owner_id: self.follower_connection_server_id.0 as u32,
            id: self.follower_connection_id as u32,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
