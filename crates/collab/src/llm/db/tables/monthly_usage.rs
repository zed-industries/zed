use crate::{db::UserId, llm::db::ModelId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "monthly_usages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: UserId,
    pub model_id: ModelId,
    pub month: i32,
    pub year: i32,
    pub input_tokens: i64,
    pub cache_creation_input_tokens: i64,
    pub cache_read_input_tokens: i64,
    pub output_tokens: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
