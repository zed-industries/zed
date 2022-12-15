use super::{ProjectCollaboratorId, ProjectId, ReplicaId, ServerId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project_collaborators")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ProjectCollaboratorId,
    pub project_id: ProjectId,
    pub connection_id: i32,
    pub connection_server_id: ServerId,
    pub user_id: UserId,
    pub replica_id: ReplicaId,
    pub is_host: bool,
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
