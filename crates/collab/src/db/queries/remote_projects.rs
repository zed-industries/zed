use rpc::proto;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

use super::{
    channel, project, remote_project, ChannelId, Database, DevServerId, RemoteProjectId, UserId,
};

impl Database {
    pub async fn get_remote_project(
        &self,
        remote_project_id: RemoteProjectId,
    ) -> crate::Result<remote_project::Model> {
        self.transaction(|tx| async move {
            Ok(remote_project::Entity::find_by_id(remote_project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| {
                    anyhow::anyhow!("no remote project with id {}", remote_project_id)
                })?)
        })
        .await
    }

    pub async fn get_remote_projects(
        &self,
        channel_ids: &Vec<ChannelId>,
        tx: &DatabaseTransaction,
    ) -> crate::Result<Vec<proto::RemoteProject>> {
        let servers = remote_project::Entity::find()
            .filter(remote_project::Column::ChannelId.is_in(channel_ids.iter().map(|id| id.0)))
            .find_also_related(project::Entity)
            .all(tx)
            .await?;
        Ok(servers
            .into_iter()
            .map(|(remote_project, project)| proto::RemoteProject {
                id: remote_project.id.to_proto(),
                project_id: project.map(|p| p.id.to_proto()),
                channel_id: remote_project.channel_id.to_proto(),
                name: remote_project.name,
                dev_server_id: remote_project.dev_server_id.to_proto(),
            })
            .collect())
    }

    pub async fn create_remote_project(
        &self,
        channel_id: ChannelId,
        dev_server_id: DevServerId,
        name: &str,
        path: &str,
        user_id: UserId,
    ) -> crate::Result<(channel::Model, remote_project::Model)> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, user_id, &*tx)
                .await?;

            let project = remote_project::Entity::insert(remote_project::ActiveModel {
                name: ActiveValue::Set(name.to_string()),
                id: ActiveValue::NotSet,
                channel_id: ActiveValue::Set(channel_id),
                dev_server_id: ActiveValue::Set(dev_server_id),
                path: ActiveValue::Set(path.to_string()),
            })
            .exec_with_returning(&*tx)
            .await?;

            Ok((channel, project))
        })
        .await
    }
}
