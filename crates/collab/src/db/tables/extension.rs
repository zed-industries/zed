use crate::db::ExtensionId;
use crate::db::ExtensionVersionId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "extensions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ExtensionId,
    pub name: String,
    pub external_id: String,
    pub latest_version: Option<ExtensionVersionId>,
    pub total_download_count: usize,
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
