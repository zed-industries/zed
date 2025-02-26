use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_repositories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub worktree_id: i64,
    #[sea_orm(primary_key)]
    pub work_directory_id: i64,
    pub scan_id: i64,
    pub branch: Option<String>,
    pub is_deleted: bool,
    // JSON array typed string
    pub current_merge_conflicts: Option<String>,
    // A JSON object representing the current Branch values
    pub branch_summary: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
