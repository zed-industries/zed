use anyhow::anyhow;
use rpc::{proto, ConnectionId};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter,
};

use super::{
    channel, project, remote_project, worktree, ChannelId, Database, DevServerId, RemoteProjectId,
    ServerId, UserId,
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
                .ok_or_else(|| anyhow!("no remote project with id {}", remote_project_id))?)
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
                path: remote_project.path,
            })
            .collect())
    }

    pub async fn get_remote_projects_for_dev_server(
        &self,
        dev_server_id: DevServerId,
    ) -> crate::Result<Vec<proto::RemoteProject>> {
        self.transaction(|tx| async move {
            let servers = remote_project::Entity::find()
                .filter(remote_project::Column::DevServerId.eq(dev_server_id))
                .find_also_related(project::Entity)
                .all(&*tx)
                .await?;
            Ok(servers
                .into_iter()
                .map(|(remote_project, project)| proto::RemoteProject {
                    id: remote_project.id.to_proto(),
                    project_id: project.map(|p| p.id.to_proto()),
                    channel_id: remote_project.channel_id.to_proto(),
                    name: remote_project.name,
                    dev_server_id: remote_project.dev_server_id.to_proto(),
                    path: remote_project.path,
                })
                .collect())
        })
        .await
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

    pub async fn share_remote_project(
        &self,
        remote_project_id: RemoteProjectId,
        dev_server_id: DevServerId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> crate::Result<proto::RemoteProject> {
        self.transaction(|tx| async move {
            let remote_project = remote_project::Entity::find_by_id(remote_project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no remote project with id {}", remote_project_id))?;

            if remote_project.dev_server_id != dev_server_id {
                return Err(anyhow!("remote project shared from wrong server"))?;
            }

            let project = project::ActiveModel {
                room_id: ActiveValue::Set(None),
                host_user_id: ActiveValue::Set(None),
                host_connection_id: ActiveValue::set(Some(connection.id as i32)),
                host_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                id: ActiveValue::NotSet,
                hosted_project_id: ActiveValue::Set(None),
                remote_project_id: ActiveValue::Set(Some(remote_project_id)),
            }
            .insert(&*tx)
            .await?;

            if !worktrees.is_empty() {
                worktree::Entity::insert_many(worktrees.iter().map(|worktree| {
                    worktree::ActiveModel {
                        id: ActiveValue::set(worktree.id as i64),
                        project_id: ActiveValue::set(project.id),
                        abs_path: ActiveValue::set(worktree.abs_path.clone()),
                        root_name: ActiveValue::set(worktree.root_name.clone()),
                        visible: ActiveValue::set(worktree.visible),
                        scan_id: ActiveValue::set(0),
                        completed_scan_id: ActiveValue::set(0),
                    }
                }))
                .exec(&*tx)
                .await?;
            }

            Ok(remote_project.to_proto(Some(project)))
        })
        .await
    }
}
