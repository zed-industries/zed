use crate::db::{ChannelId, DevServerId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "devservers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: DevServerId,
    pub name: String,
    pub channel_id: ChannelId,
    pub latest_operation_epoch: Option<i32>,
    pub latest_operation_lamport_timestamp: Option<i32>,
    pub latest_operation_replica_id: Option<i32>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
