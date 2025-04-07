use crate::llm::db::UsageMeasureId;
use sea_orm::entity::prelude::*;

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, strum::EnumString, strum::Display, strum::EnumIter,
)]
#[strum(serialize_all = "snake_case")]
pub enum UsageMeasure {
    RequestsPerMinute,
    TokensPerMinute,
    InputTokensPerMinute,
    OutputTokensPerMinute,
    TokensPerDay,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "usage_measures")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UsageMeasureId,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::usage::Entity")]
    Usages,
}

impl Related<super::usage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
