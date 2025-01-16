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
    /// Old single-code status field, no longer used but kept here to mirror the DB schema.
    pub status: i64,
    pub status_kind: StatusKind,
    /// For unmerged entries, this is the `first_head` status. For tracked entries, this is the `index_status`.
    pub first_status: Option<i32>,
    /// For unmerged entries, this is the `second_head` status. For tracked entries, this is the `worktree_status`.
    pub second_status: Option<i32>,
    pub scan_id: i64,
    pub is_deleted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum StatusKind {
    Untracked = 0,
    Ignored = 1,
    Unmerged = 2,
    Tracked = 3,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
