use crate::db::{ChannelId, DevServerId};
use rpc::proto;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dev_servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: DevServerId,
    pub name: String,
    pub channel_id: ChannelId,
    pub hashed_token: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Model {
    pub fn to_proto(&self, status: proto::DevServerStatus) -> proto::DevServer {
        proto::DevServer {
            dev_server_id: self.id.to_proto(),
            channel_id: self.channel_id.to_proto(),
            name: self.name.clone(),
            status: status as i32,
        }
    }
}
