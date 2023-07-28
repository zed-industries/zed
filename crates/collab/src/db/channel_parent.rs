use super::ChannelId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channel_parents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub child_id: ChannelId,
    #[sea_orm(primary_key)]
    pub parent_id: ChannelId,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
