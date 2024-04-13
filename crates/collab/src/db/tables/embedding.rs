use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "embeddings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub model: String,
    #[sea_orm(primary_key)]
    pub digest: Vec<u8>,
    pub dimensions: Vec<f32>,
    pub retrieved_at: PrimitiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
