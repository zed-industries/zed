use crate::db::ProjectId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worktree_settings_files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub worktree_id: i64,
    #[sea_orm(primary_key)]
    pub path: String,
    pub content: String,
    pub kind: LocalSettingsKind,
    pub outside_worktree: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Default, Hash, serde::Serialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum LocalSettingsKind {
    #[default]
    #[sea_orm(string_value = "settings")]
    Settings,
    #[sea_orm(string_value = "tasks")]
    Tasks,
    #[sea_orm(string_value = "editorconfig")]
    Editorconfig,
    #[sea_orm(string_value = "debug")]
    Debug,
}
