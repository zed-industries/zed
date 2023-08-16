use super::ChannelId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channel_paths")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_path: String,
    pub channel_id: ChannelId,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
