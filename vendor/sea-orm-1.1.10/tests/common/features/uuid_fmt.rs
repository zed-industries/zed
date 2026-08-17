use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "uuid_fmt")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_braced: uuid::fmt::Braced,
    pub uuid_hyphenated: uuid::fmt::Hyphenated,
    pub uuid_simple: uuid::fmt::Simple,
    pub uuid_urn: uuid::fmt::Urn,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
