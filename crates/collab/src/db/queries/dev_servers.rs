use rpc::proto;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseTransaction, EntityTrait, IntoActiveModel, QueryFilter,
};

use super::{dev_server, remote_project, Database, DevServerId, UserId};

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

    pub async fn get_dev_servers(&self, user_id: UserId) -> crate::Result<Vec<dev_server::Model>> {
        self.transaction(|tx| async move {
            Ok(dev_server::Entity::find()
                .filter(dev_server::Column::UserId.eq(user_id))
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn remote_projects_update(
        &self,
        user_id: UserId,
    ) -> crate::Result<proto::RemoteProjectsUpdate> {
        self.transaction(
            |tx| async move { self.remote_projects_update_internal(user_id, &tx).await },
        )
        .await
    }

    pub async fn remote_projects_update_internal(
        &self,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> crate::Result<proto::RemoteProjectsUpdate> {
        let dev_servers = dev_server::Entity::find()
            .filter(dev_server::Column::UserId.eq(user_id))
            .all(tx)
            .await?;

        let remote_projects = remote_project::Entity::find()
            .filter(
                remote_project::Column::DevServerId
                    .is_in(dev_servers.iter().map(|d| d.id).collect::<Vec<_>>()),
            )
            .find_also_related(super::project::Entity)
            .all(tx)
            .await?;

        Ok(proto::RemoteProjectsUpdate {
            dev_servers: dev_servers
                .into_iter()
                .map(|d| d.to_proto(proto::DevServerStatus::Offline))
                .collect(),
            remote_projects: remote_projects
                .into_iter()
                .map(|(remote_project, project)| remote_project.to_proto(project))
                .collect(),
        })
    }

    pub async fn create_dev_server(
        &self,
        name: &str,
        hashed_access_token: &str,
        user_id: UserId,
    ) -> crate::Result<(dev_server::Model, proto::RemoteProjectsUpdate)> {
        self.transaction(|tx| async move {
            let dev_server = dev_server::Entity::insert(dev_server::ActiveModel {
                id: ActiveValue::NotSet,
                hashed_token: ActiveValue::Set(hashed_access_token.to_string()),
                name: ActiveValue::Set(name.to_string()),
                user_id: ActiveValue::Set(user_id),
            })
            .exec_with_returning(&*tx)
            .await?;

            let remote_projects = self.remote_projects_update_internal(user_id, &tx).await?;

            Ok((dev_server, remote_projects))
        })
        .await
    }

    pub async fn delete_dev_server(
        &self,
        id: DevServerId,
        user_id: UserId,
    ) -> crate::Result<proto::RemoteProjectsUpdate> {
        self.transaction(|tx| async move {
            let Some(dev_server) = dev_server::Entity::find_by_id(id).one(&*tx).await? else {
                return Err(anyhow::anyhow!("no dev server with id {}", id))?;
            };
            if dev_server.user_id != user_id {
                return Err(anyhow::anyhow!(proto::ErrorCode::Forbidden))?;
            }

            remote_project::Entity::delete_many()
                .filter(remote_project::Column::DevServerId.eq(id))
                .exec(&*tx)
                .await?;

            dev_server::Entity::delete(dev_server.into_active_model())
                .exec(&*tx)
                .await?;

            let remote_projects = self.remote_projects_update_internal(user_id, &tx).await?;

            Ok(remote_projects)
        })
        .await
    }
}
