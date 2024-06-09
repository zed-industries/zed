use rpc::proto;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseTransaction, EntityTrait, IntoActiveModel, QueryFilter,
};

use super::{dev_server, dev_server_project, Database, DevServerId, UserId};

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

    pub async fn dev_server_projects_update(
        &self,
        user_id: UserId,
    ) -> crate::Result<proto::DevServerProjectsUpdate> {
        self.transaction(|tx| async move {
            self.dev_server_projects_update_internal(user_id, &tx).await
        })
        .await
    }

    pub async fn dev_server_projects_update_internal(
        &self,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> crate::Result<proto::DevServerProjectsUpdate> {
        let dev_servers = dev_server::Entity::find()
            .filter(dev_server::Column::UserId.eq(user_id))
            .all(tx)
            .await?;

        let dev_server_projects = dev_server_project::Entity::find()
            .filter(
                dev_server_project::Column::DevServerId
                    .is_in(dev_servers.iter().map(|d| d.id).collect::<Vec<_>>()),
            )
            .find_also_related(super::project::Entity)
            .all(tx)
            .await?;

        Ok(proto::DevServerProjectsUpdate {
            dev_servers: dev_servers
                .into_iter()
                .map(|d| d.to_proto(proto::DevServerStatus::Offline))
                .collect(),
            dev_server_projects: dev_server_projects
                .into_iter()
                .map(|(dev_server_project, project)| dev_server_project.to_proto(project))
                .collect(),
        })
    }

    pub async fn create_dev_server(
        &self,
        name: &str,
        ssh_connection_string: Option<&str>,
        hashed_access_token: &str,
        user_id: UserId,
    ) -> crate::Result<(dev_server::Model, proto::DevServerProjectsUpdate)> {
        self.transaction(|tx| async move {
            if name.trim().is_empty() {
                return Err(anyhow::anyhow!(proto::ErrorCode::Forbidden))?;
            }

            let dev_server = dev_server::Entity::insert(dev_server::ActiveModel {
                id: ActiveValue::NotSet,
                hashed_token: ActiveValue::Set(hashed_access_token.to_string()),
                name: ActiveValue::Set(name.trim().to_string()),
                user_id: ActiveValue::Set(user_id),
                ssh_connection_string: ActiveValue::Set(
                    ssh_connection_string.map(ToOwned::to_owned),
                ),
            })
            .exec_with_returning(&*tx)
            .await?;

            let dev_server_projects = self
                .dev_server_projects_update_internal(user_id, &tx)
                .await?;

            Ok((dev_server, dev_server_projects))
        })
        .await
    }

    pub async fn update_dev_server_token(
        &self,
        id: DevServerId,
        hashed_token: &str,
        user_id: UserId,
    ) -> crate::Result<proto::DevServerProjectsUpdate> {
        self.transaction(|tx| async move {
            let Some(dev_server) = dev_server::Entity::find_by_id(id).one(&*tx).await? else {
                return Err(anyhow::anyhow!("no dev server with id {}", id))?;
            };
            if dev_server.user_id != user_id {
                return Err(anyhow::anyhow!(proto::ErrorCode::Forbidden))?;
            }

            dev_server::Entity::update(dev_server::ActiveModel {
                hashed_token: ActiveValue::Set(hashed_token.to_string()),
                ..dev_server.clone().into_active_model()
            })
            .exec(&*tx)
            .await?;

            let dev_server_projects = self
                .dev_server_projects_update_internal(user_id, &tx)
                .await?;

            Ok(dev_server_projects)
        })
        .await
    }

    pub async fn rename_dev_server(
        &self,
        id: DevServerId,
        name: &str,
        ssh_connection_string: Option<&str>,
        user_id: UserId,
    ) -> crate::Result<proto::DevServerProjectsUpdate> {
        self.transaction(|tx| async move {
            let Some(dev_server) = dev_server::Entity::find_by_id(id).one(&*tx).await? else {
                return Err(anyhow::anyhow!("no dev server with id {}", id))?;
            };
            if dev_server.user_id != user_id || name.trim().is_empty() {
                return Err(anyhow::anyhow!(proto::ErrorCode::Forbidden))?;
            }

            dev_server::Entity::update(dev_server::ActiveModel {
                name: ActiveValue::Set(name.trim().to_string()),
                ssh_connection_string: ActiveValue::Set(
                    ssh_connection_string.map(ToOwned::to_owned),
                ),
                ..dev_server.clone().into_active_model()
            })
            .exec(&*tx)
            .await?;

            let dev_server_projects = self
                .dev_server_projects_update_internal(user_id, &tx)
                .await?;

            Ok(dev_server_projects)
        })
        .await
    }

    pub async fn delete_dev_server(
        &self,
        id: DevServerId,
        user_id: UserId,
    ) -> crate::Result<proto::DevServerProjectsUpdate> {
        self.transaction(|tx| async move {
            let Some(dev_server) = dev_server::Entity::find_by_id(id).one(&*tx).await? else {
                return Err(anyhow::anyhow!("no dev server with id {}", id))?;
            };
            if dev_server.user_id != user_id {
                return Err(anyhow::anyhow!(proto::ErrorCode::Forbidden))?;
            }

            dev_server_project::Entity::delete_many()
                .filter(dev_server_project::Column::DevServerId.eq(id))
                .exec(&*tx)
                .await?;

            dev_server::Entity::delete(dev_server.into_active_model())
                .exec(&*tx)
                .await?;

            let dev_server_projects = self
                .dev_server_projects_update_internal(user_id, &tx)
                .await?;

            Ok(dev_server_projects)
        })
        .await
    }
}
