use crate::{db::UserId, llm::db::ModelId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "lifetime_usages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: UserId,
    pub model_id: ModelId,
    pub input_tokens: i64,
    pub output_tokens: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
