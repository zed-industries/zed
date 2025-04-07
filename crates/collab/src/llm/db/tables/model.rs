use sea_orm::entity::prelude::*;

use crate::llm::db::{ModelId, ProviderId};

/// An LLM model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "models")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ModelId,
    pub provider_id: ProviderId,
    pub name: String,
    pub max_requests_per_minute: i64,
    pub max_tokens_per_minute: i64,
    pub max_input_tokens_per_minute: i64,
    pub max_output_tokens_per_minute: i64,
    pub max_tokens_per_day: i64,
    pub price_per_million_input_tokens: i32,
    pub price_per_million_cache_creation_input_tokens: i32,
    pub price_per_million_cache_read_input_tokens: i32,
    pub price_per_million_output_tokens: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::provider::Entity",
        from = "Column::ProviderId",
        to = "super::provider::Column::Id"
    )]
    Provider,
    #[sea_orm(has_many = "super::usage::Entity")]
    Usages,
    #[sea_orm(has_many = "super::billing_event::Entity")]
    BillingEvents,
}

impl Related<super::provider::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Provider.def()
    }
}

impl Related<super::usage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usages.def()
    }
}

impl Related<super::billing_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BillingEvents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
