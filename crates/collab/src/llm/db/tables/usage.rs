use sea_orm::entity::prelude::*;

use crate::llm::db::ModelId;

/// An LLM usage record.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "usages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// The ID of the Zed user.
    ///
    /// Corresponds to the `users` table in the primary collab database.
    pub user_id: i32,
    pub model_id: ModelId,
    pub requests_this_minute: i32,
    pub tokens_this_minute: i64,
    pub requests_this_day: i32,
    pub tokens_this_day: i64,
    pub requests_this_month: i32,
    pub tokens_this_month: i64,
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

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
