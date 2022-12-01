use super::{ProjectCollaboratorId, ProjectId, ReplicaId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project_collaborators")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ProjectCollaboratorId,
    pub project_id: ProjectId,
    pub connection_id: u32,
    pub user_id: UserId,
    pub replica_id: ReplicaId,
    pub is_host: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
