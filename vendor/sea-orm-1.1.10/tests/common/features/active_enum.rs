use super::sea_orm_active_enums::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[cfg_attr(feature = "sqlx-postgres", sea_orm(schema_name = "public"))]
#[sea_orm(table_name = "active_enum")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub category: Option<Category>,
    pub color: Option<Color>,
    pub tea: Option<Tea>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::active_enum_child::Entity")]
    ActiveEnumChild,
}

impl Related<super::active_enum_child::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActiveEnumChild.def()
    }
}

pub struct ActiveEnumChildLink;

impl Linked for ActiveEnumChildLink {
    type FromEntity = Entity;

    type ToEntity = super::active_enum_child::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::ActiveEnumChild.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
