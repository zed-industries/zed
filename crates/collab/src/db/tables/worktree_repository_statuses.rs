use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_repository_statuses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub worktree_id: i64,
    #[sea_orm(primary_key)]
    pub work_directory_id: i64,
    #[sea_orm(primary_key)]
    pub repo_path: String,
    pub status_kind: StatusKind,
    /// For unmerged entries, this is the `first_head` status. For tracked entries, this is the `index_status`.
    pub first_status: Option<i64>,
    /// For unmerged entries, this is the `second_head` status. For tracked entries, this is the `worktree_status`.
    pub second_status: Option<i64>,
    pub scan_id: i64,
    pub is_deleted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum StatusKind {
    #[sea_orm(num_value = 0)]
    Untracked,
    #[sea_orm(num_value = 1)]
    Ignored,
    #[sea_orm(num_value = 2)]
    Unmerged,
    #[sea_orm(num_value = 3)]
    Tracked,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
