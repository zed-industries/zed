use crate::db::{ChannelId, DevServerId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dev_servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: DevServerId,
    pub name: String,
    pub channel_id: ChannelId,
    pub hashed_token: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
