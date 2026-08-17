use super::sea_orm_active_enums::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[cfg_attr(feature = "sqlx-postgres", sea_orm(schema_name = "public"))]
#[sea_orm(table_name = "active_enum_child")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_id: i32,
    pub category: Option<Category>,
    pub color: Option<Color>,
    pub tea: Option<Tea>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        fk_name = "fk-active_enum_child-active_enum",
        belongs_to = "super::active_enum::Entity",
        from = "Column::ParentId",
        to = "super::active_enum::Column::Id"
    )]
    ActiveEnum,
}

impl Related<super::active_enum::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActiveEnum.def()
    }
}

pub struct ActiveEnumLink;

impl Linked for ActiveEnumLink {
    type FromEntity = Entity;

    type ToEntity = super::active_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::ActiveEnum.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
