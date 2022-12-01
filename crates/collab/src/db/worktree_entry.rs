use super::{ProjectId, WorktreeEntryId, WorktreeId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    project_id: ProjectId,
    #[sea_orm(primary_key)]
    worktree_id: WorktreeId,
    #[sea_orm(primary_key)]
    id: WorktreeEntryId,
    is_dir: bool,
    path: String,
    inode: u64,
    mtime_seconds: u64,
    mtime_nanos: u32,
    is_symlink: bool,
    is_ignored: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
