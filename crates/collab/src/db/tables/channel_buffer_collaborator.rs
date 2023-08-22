use crate::db::{ChannelBufferCollaboratorId, ChannelId, ReplicaId, ServerId, UserId};
use rpc::ConnectionId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channel_buffer_collaborators")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ChannelBufferCollaboratorId,
    pub channel_id: ChannelId,
    pub connection_id: i32,
    pub connection_server_id: ServerId,
    pub connection_lost: bool,
    pub user_id: UserId,
    pub replica_id: ReplicaId,
}

impl Model {
    pub fn connection(&self) -> ConnectionId {
        ConnectionId {
            owner_id: self.connection_server_id.0 as u32,
            id: self.connection_id as u32,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id"
    )]
    Channel,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
