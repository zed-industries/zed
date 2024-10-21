use crate::llm::db::ProviderId;
use sea_orm::entity::prelude::*;

/// An LLM provider.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "providers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ProviderId,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::model::Entity")]
    Models,
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Models.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
