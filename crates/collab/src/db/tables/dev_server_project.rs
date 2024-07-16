use super::project;
use crate::db::{DevServerId, DevServerProjectId};
use rpc::proto;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dev_server_projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: DevServerProjectId,
    pub dev_server_id: DevServerId,
    pub paths: JSONPaths,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct JSONPaths(pub Vec<String>);

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::project::Entity")]
    Project,
    #[sea_orm(
        belongs_to = "super::dev_server::Entity",
        from = "Column::DevServerId",
        to = "super::dev_server::Column::Id"
    )]
    DevServer,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::dev_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DevServer.def()
    }
}

impl Model {
    pub fn to_proto(&self, project: Option<project::Model>) -> proto::DevServerProject {
        proto::DevServerProject {
            id: self.id.to_proto(),
            project_id: project.map(|p| p.id.to_proto()),
            dev_server_id: self.dev_server_id.to_proto(),
            path: self.paths().get(0).cloned().unwrap_or_default(),
            paths: self.paths().clone(),
        }
    }

    pub fn paths(&self) -> &Vec<String> {
        &self.paths.0
    }
}
