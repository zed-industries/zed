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

impl ActiveModelBehavior for ActiveModel {}
