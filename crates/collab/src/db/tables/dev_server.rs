use crate::db::{DevServerId, UserId};
use rpc::proto;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dev_servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: DevServerId,
    pub name: String,
    pub user_id: UserId,
    pub hashed_token: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dev_server_project::Entity")]
    RemoteProject,
}

impl Related<super::dev_server_project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RemoteProject.def()
    }
}

impl Model {
    pub fn to_proto(&self, status: proto::DevServerStatus) -> proto::DevServer {
        proto::DevServer {
            dev_server_id: self.id.to_proto(),
            name: self.name.clone(),
            status: status as i32,
        }
    }
}
