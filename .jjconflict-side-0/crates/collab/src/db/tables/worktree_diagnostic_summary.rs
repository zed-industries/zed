use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_diagnostic_summaries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub worktree_id: i64,
    #[sea_orm(primary_key)]
    pub path: String,
    pub language_server_id: i64,
    pub error_count: i32,
    pub warning_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
