use crate::{
    db::UserId,
    llm::db::{BillingEventId, ModelId},
};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_events")]
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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model::Entity",
        from = "Column::ModelId",
        to = "super::model::Column::Id"
    )]
    Model,
}

impl ActiveModelBehavior for ActiveModel {}
