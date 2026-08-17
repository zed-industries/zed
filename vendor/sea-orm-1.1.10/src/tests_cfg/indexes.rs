//! An entity definition for testing table index creation.
use crate as sea_orm;
use crate::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn schema_name(&self) -> Option<&str> {
        Some("public")
    }

    fn table_name(&self) -> &str {
        "indexes"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub indexes_id: i32,
    pub unique_attr: i32,
    pub index1_attr: i32,
    pub index2_attr: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    IndexesId,
    UniqueAttr,
    Index1Attr,
    Index2Attr,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    IndexesId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::IndexesId => ColumnType::Integer.def(),
            Self::UniqueAttr => ColumnType::Integer.def().unique(),
            Self::Index1Attr => ColumnType::Integer.def().indexed(),
            Self::Index2Attr => ColumnType::Integer.def().indexed().unique(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
