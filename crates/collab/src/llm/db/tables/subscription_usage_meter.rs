use sea_orm::entity::prelude::*;

use crate::llm::db::ModelId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subscription_usage_meters")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub subscription_usage_id: i32,
    pub model_id: ModelId,
    pub requests: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::subscription_usage::Entity",
        from = "Column::SubscriptionUsageId",
        to = "super::subscription_usage::Column::Id"
    )]
    SubscriptionUsage,
    #[sea_orm(
        belongs_to = "super::model::Entity",
        from = "Column::ModelId",
        to = "super::model::Column::Id"
    )]
    Model,
}

impl Related<super::subscription_usage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubscriptionUsage.def()
    }
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
