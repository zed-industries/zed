use crate::db::ExtensionId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "extensions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ExtensionId,
    pub external_id: String,
    pub name: String,
    pub latest_version: String,
    pub total_download_count: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::extension_version::Entity")]
    LatestVersion,
}

impl Related<super::extension_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LatestVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
