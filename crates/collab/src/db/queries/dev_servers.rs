use sea_orm::EntityTrait;

use super::{dev_server, Database, DevServerId};

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
}
