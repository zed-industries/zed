use crate::db::{BufferId, ChannelId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "buffers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BufferId,
    pub epoch: i32,
    pub channel_id: ChannelId,
    pub latest_operation_epoch: Option<i32>,
    pub latest_operation_lamport_timestamp: Option<i32>,
    pub latest_operation_replica_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::buffer_operation::Entity")]
    Operations,
    #[sea_orm(has_many = "super::buffer_snapshot::Entity")]
    Snapshots,
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id"
    )]
    Channel,
}

impl Related<super::buffer_operation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operations.def()
    }
}

impl Related<super::buffer_snapshot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Snapshots.def()
    }
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
