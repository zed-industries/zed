use sea_orm::{ActiveValue, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

use super::{channel, dev_server, ChannelId, Database, DevServerId, UserId};

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
    ) -> crate::Result<Vec<dev_server::Model>> {
        let servers = dev_server::Entity::find()
            .filter(dev_server::Column::ChannelId.is_in(channel_ids.iter().map(|id| id.0)))
            .all(tx)
            .await?;
        Ok(servers)
    }

    pub async fn create_dev_server(
        &self,
        channel_id: ChannelId,
        name: &str,
        hashed_access_token: &str,
        user_id: UserId,
    ) -> crate::Result<(channel::Model, dev_server::Model)> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &tx).await?;
            self.check_user_is_channel_admin(&channel, user_id, &tx)
                .await?;

            let dev_server = dev_server::Entity::insert(dev_server::ActiveModel {
                id: ActiveValue::NotSet,
                hashed_token: ActiveValue::Set(hashed_access_token.to_string()),
                channel_id: ActiveValue::Set(channel_id),
                name: ActiveValue::Set(name.to_string()),
            })
            .exec_with_returning(&*tx)
            .await?;

            Ok((channel, dev_server))
        })
        .await
    }
}
