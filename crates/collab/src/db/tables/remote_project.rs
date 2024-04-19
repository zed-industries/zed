use super::project;
use crate::db::{ChannelId, DevServerId, RemoteProjectId};
use rpc::proto;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "remote_projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RemoteProjectId,
    pub channel_id: ChannelId,
    pub dev_server_id: DevServerId,
    pub name: String,
    pub path: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::project::Entity")]
    Project,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Model {
    pub fn to_proto(&self, project: Option<project::Model>) -> proto::RemoteProject {
        proto::RemoteProject {
            id: self.id.to_proto(),
            project_id: project.map(|p| p.id.to_proto()),
            channel_id: self.channel_id.to_proto(),
            dev_server_id: self.dev_server_id.to_proto(),
            name: self.name.clone(),
            path: self.path.clone(),
        }
    }
}
