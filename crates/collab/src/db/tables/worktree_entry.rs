use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub worktree_id: i64,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub is_dir: bool,
    pub path: String,
    pub inode: i64,
    pub mtime_seconds: i64,
    pub mtime_nanos: i32,
    pub git_status: Option<i64>,
    pub is_ignored: bool,
    pub is_external: bool,
    pub is_deleted: bool,
    pub is_hidden: bool,
    pub scan_id: i64,
    pub is_fifo: bool,
    pub canonical_path: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
