use super::ServerEpoch;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub epoch: ServerEpoch,
    pub environment: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
