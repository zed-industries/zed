use crate::{
    db::UserId,
    llm::db::{ModelId, UsageId, UsageMeasureId},
};
use sea_orm::entity::prelude::*;

/// An LLM usage record.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "usages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UsageId,
    /// The ID of the Zed user.
    ///
    /// Corresponds to the `users` table in the primary collab database.
    pub user_id: UserId,
    pub model_id: ModelId,
    pub measure_id: UsageMeasureId,
    pub timestamp: DateTime,
    pub buckets: Vec<i64>,
    pub is_staff: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model::Entity",
        from = "Column::ModelId",
        to = "super::model::Column::Id"
    )]
    Model,
    #[sea_orm(
        belongs_to = "super::usage_measure::Entity",
        from = "Column::MeasureId",
        to = "super::usage_measure::Column::Id"
    )]
    UsageMeasure,
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl Related<super::usage_measure::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsageMeasure.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
