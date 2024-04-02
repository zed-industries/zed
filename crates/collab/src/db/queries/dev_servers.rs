use rpc::proto;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

use super::{dev_server, ChannelId, Database, DevServerId};

impl Database {
    pub async fn get_dev_server(
        &self,
        dev_server_id: DevServerId,
    ) -> crate::Result<dev_server::Model> {
        self.transaction(|tx| async move {
            Ok(dev_server::Entity::find_by_id(dev_server_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow::anyhow!("no dev server with id {}", dev_server_id))?)
        })
        .await
    }

    pub async fn get_dev_servers(
        &self,
        channel_ids: &Vec<ChannelId>,
        tx: &DatabaseTransaction,
    ) -> crate::Result<Vec<proto::DevServer>> {
        let servers = dev_server::Entity::find()
            .filter(dev_server::Column::ChannelId.is_in(channel_ids.iter().map(|id| id.0)))
            .all(tx)
            .await?;
        Ok(servers
            .into_iter()
            .map(|s| proto::DevServer {
                channel_id: s.channel_id.to_proto(),
                name: s.name,
                dev_server_id: s.id.to_proto(),
                status: proto::DevServerStatus::Online.into(),
            })
            .collect())
    }
}
