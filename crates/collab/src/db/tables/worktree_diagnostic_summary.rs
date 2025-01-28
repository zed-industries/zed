use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_diagnostic_summaries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub worktree_id: i64,
    // TODO:
    // The path here should be a Vec<String> like others, but it's a String for now.
    // Because the sea-orm doesn't support Vec<String> as primary key.
    // This is used in the database, so the path spearator of this path string should
    // be `/`.
    #[sea_orm(primary_key)]
    pub path: String,
    pub language_server_id: i64,
    pub error_count: i32,
    pub warning_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
