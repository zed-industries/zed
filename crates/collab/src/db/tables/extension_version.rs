use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

use crate::db::ExtensionId;
use crate::db::ExtensionVersionId;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "extension_versions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ExtensionVersionId,
    pub extension_id: ExtensionId,
    pub published_at: PrimitiveDateTime,
    pub version: String,
    pub authors: String,
    pub repository: String,
    pub description: String,
    pub download_count: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::extension::Entity")]
    Extension,
}

impl Related<super::extension::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Extension.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
