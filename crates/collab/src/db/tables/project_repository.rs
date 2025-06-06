use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project_repositories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub abs_path: String,
    pub legacy_worktree_id: Option<i64>,
    // JSON array containing 1 or more integer project entry ids
    pub entry_ids: String,
    pub scan_id: i64,
    pub is_deleted: bool,
    // JSON array typed string
    pub current_merge_conflicts: Option<String>,
    // A JSON object representing the current Branch values
    pub branch_summary: Option<String>,
    // A JSON object representing the current Head commit values
    pub head_commit_details: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
