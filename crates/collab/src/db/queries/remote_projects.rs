use anyhow::anyhow;
use rpc::{proto, ConnectionId};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseTransaction, EntityTrait,
    ModelTrait, QueryFilter,
};

use crate::db::ProjectId;

use super::{
    dev_server, project, project_collaborator, remote_project, worktree, Database, DevServerId,
    RejoinedProject, RemoteProjectId, ResharedProject, ServerId, UserId,
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
                    dev_server_id: remote_project.dev_server_id.to_proto(),
                    path: remote_project.path,
                })
                .collect())
        })
        .await
    }

    pub async fn remote_project_ids_for_user(
        &self,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> crate::Result<Vec<RemoteProjectId>> {
        let dev_servers = dev_server::Entity::find()
            .filter(dev_server::Column::UserId.eq(user_id))
            .find_with_related(remote_project::Entity)
            .all(tx)
            .await?;

        Ok(dev_servers
            .into_iter()
            .flat_map(|(_, projects)| projects.into_iter().map(|p| p.id))
            .collect())
    }

    pub async fn owner_for_remote_project(
        &self,
        remote_project_id: RemoteProjectId,
        tx: &DatabaseTransaction,
    ) -> crate::Result<UserId> {
        let dev_server = remote_project::Entity::find_by_id(remote_project_id)
            .find_also_related(dev_server::Entity)
            .one(tx)
            .await?
            .and_then(|(_, dev_server)| dev_server)
            .ok_or_else(|| anyhow!("no remote project"))?;

        Ok(dev_server.user_id)
    }

    pub async fn get_stale_dev_server_projects(
        &self,
        connection: ConnectionId,
    ) -> crate::Result<Vec<ProjectId>> {
        self.transaction(|tx| async move {
            let projects = project::Entity::find()
                .filter(
                    Condition::all()
                        .add(project::Column::HostConnectionId.eq(connection.id))
                        .add(project::Column::HostConnectionServerId.eq(connection.owner_id)),
                )
                .all(&*tx)
                .await?;

            Ok(projects.into_iter().map(|p| p.id).collect())
        })
        .await
    }

    pub async fn create_remote_project(
        &self,
        dev_server_id: DevServerId,
        path: &str,
        user_id: UserId,
    ) -> crate::Result<(remote_project::Model, proto::RemoteProjectsUpdate)> {
        self.transaction(|tx| async move {
            let dev_server = dev_server::Entity::find_by_id(dev_server_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no dev server with id {}", dev_server_id))?;
            if dev_server.user_id != user_id {
                return Err(anyhow!("not your dev server"))?;
            }

            let project = remote_project::Entity::insert(remote_project::ActiveModel {
                id: ActiveValue::NotSet,
                dev_server_id: ActiveValue::Set(dev_server_id),
                path: ActiveValue::Set(path.to_string()),
            })
            .exec_with_returning(&*tx)
            .await?;

            let status = self.remote_projects_update_internal(user_id, &tx).await?;

            Ok((project, status))
        })
        .await
    }

    pub async fn share_remote_project(
        &self,
        remote_project_id: RemoteProjectId,
        dev_server_id: DevServerId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> crate::Result<(proto::RemoteProject, UserId, proto::RemoteProjectsUpdate)> {
        self.transaction(|tx| async move {
            let dev_server = dev_server::Entity::find_by_id(dev_server_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no dev server with id {}", dev_server_id))?;

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

            let status = self
                .remote_projects_update_internal(dev_server.user_id, &tx)
                .await?;

            Ok((
                remote_project.to_proto(Some(project)),
                dev_server.user_id,
                status,
            ))
        })
        .await
    }

    pub async fn reshare_remote_projects(
        &self,
        reshared_projects: &Vec<proto::UpdateProject>,
        dev_server_id: DevServerId,
        connection: ConnectionId,
    ) -> crate::Result<Vec<ResharedProject>> {
        // todo!() project_transaction? (maybe we can make the lock per-dev-server instead of per-project?)
        self.transaction(|tx| async move {
            let mut ret = Vec::new();
            for reshared_project in reshared_projects {
                let project_id = ProjectId::from_proto(reshared_project.project_id);
                let (project, remote_project) = project::Entity::find_by_id(project_id)
                    .find_also_related(remote_project::Entity)
                    .one(&*tx)
                    .await?
                    .ok_or_else(|| anyhow!("project does not exist"))?;

                if remote_project.map(|rp| rp.dev_server_id) != Some(dev_server_id) {
                    return Err(anyhow!("remote project reshared from wrong server"))?;
                }

                let Ok(old_connection_id) = project.host_connection() else {
                    return Err(anyhow!("remote project was not shared"))?;
                };

                project::Entity::update(project::ActiveModel {
                    id: ActiveValue::set(project_id),
                    host_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    host_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

                let collaborators = project
                    .find_related(project_collaborator::Entity)
                    .all(&*tx)
                    .await?;

                self.update_project_worktrees(project_id, &reshared_project.worktrees, &tx)
                    .await?;

                ret.push(super::ResharedProject {
                    id: project_id,
                    old_connection_id,
                    collaborators: collaborators
                        .iter()
                        .map(|collaborator| super::ProjectCollaborator {
                            connection_id: collaborator.connection(),
                            user_id: collaborator.user_id,
                            replica_id: collaborator.replica_id,
                            is_host: collaborator.is_host,
                        })
                        .collect(),
                    worktrees: reshared_project.worktrees.clone(),
                });
            }
            Ok(ret)
        })
        .await
    }

    pub async fn rejoin_remote_projects(
        &self,
        rejoined_projects: &Vec<proto::RejoinProject>,
        user_id: UserId,
        connection_id: ConnectionId,
    ) -> crate::Result<Vec<RejoinedProject>> {
        // todo!() project_transaction? (maybe we can make the lock per-dev-server instead of per-project?)
        self.transaction(|tx| async move {
            let mut ret = Vec::new();
            for rejoined_project in rejoined_projects {
                if let Some(project) = self
                    .rejoin_project_internal(&tx, rejoined_project, user_id, connection_id)
                    .await?
                {
                    ret.push(project);
                }
            }
            Ok(ret)
        })
        .await
    }
}
