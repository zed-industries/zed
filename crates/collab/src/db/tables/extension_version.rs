use crate::db::ExtensionId;
use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "extension_versions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub extension_id: ExtensionId,
    #[sea_orm(primary_key)]
    pub version: String,
    pub published_at: PrimitiveDateTime,
    pub authors: String,
    pub repository: String,
    pub description: String,
    pub schema_version: i32,
    pub wasm_api_version: Option<String>,
    pub download_count: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::extension::Entity",
        from = "Column::ExtensionId",
        to = "super::extension::Column::Id"
        on_condition = r#"super::extension::Column::LatestVersion.into_expr().eq(Column::Version.into_expr())"#
    )]
    Extension,
}

impl Related<super::extension::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Extension.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
