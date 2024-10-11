use crate::{
    db::UserId,
    llm::db::{BillingEventId, ModelId},
};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "monthly_usages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingEventId,
    pub user_id: UserId,
    pub model_id: ModelId,
    pub input_tokens: i64,
    pub cache_creation_input_tokens: i64,
    pub cache_read_input_tokens: i64,
    pub output_tokens: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
