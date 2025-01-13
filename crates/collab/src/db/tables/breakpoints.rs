use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "breakpoints")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    pub worktree_id: i64,
    pub path: String,
    pub kind: BreakpointKind,
    pub log_message: Option<String>,
    pub position: i32,
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Default, Hash, serde::Serialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum BreakpointKind {
    #[default]
    #[sea_orm(string_value = "standard")]
    Standard,
    #[sea_orm(string_value = "log")]
    Log,
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
