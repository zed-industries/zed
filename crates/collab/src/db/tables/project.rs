use crate::db::{ProjectId, Result, RoomId, ServerId, UserId};
use anyhow::Context as _;
use rpc::ConnectionId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ProjectId,
    pub room_id: Option<RoomId>,
    pub host_user_id: Option<UserId>,
    pub host_connection_id: Option<i32>,
    pub host_connection_server_id: Option<ServerId>,
    pub windows_paths: bool,
}

impl Model {
    pub fn host_connection(&self) -> Result<ConnectionId> {
        let host_connection_server_id = self
            .host_connection_server_id
            .context("empty host_connection_server_id")?;
        let host_connection_id = self
            .host_connection_id
            .context("empty host_connection_id")?;
        Ok(ConnectionId {
            owner_id: host_connection_server_id.0 as u32,
            id: host_connection_id as u32,
        })
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::HostUserId",
        to = "super::user::Column::Id"
    )]
    HostUser,
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
    #[sea_orm(has_many = "super::worktree::Entity")]
    Worktrees,
    #[sea_orm(has_many = "super::project_repository::Entity")]
    Repositories,
    #[sea_orm(has_many = "super::project_collaborator::Entity")]
    Collaborators,
    #[sea_orm(has_many = "super::language_server::Entity")]
    LanguageServers,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HostUser.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::worktree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Worktrees.def()
    }
}

impl Related<super::project_repository::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Repositories.def()
    }
}

impl Related<super::project_collaborator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaborators.def()
    }
}

impl Related<super::language_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LanguageServers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
