use rpc::{proto, ErrorCode};

use super::*;

impl Database {
    pub async fn get_hosted_projects(
        &self,
        channel_ids: &Vec<ChannelId>,
        roles: &HashMap<ChannelId, ChannelRole>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<proto::HostedProject>> {
        Ok(hosted_project::Entity::find()
            .filter(hosted_project::Column::ChannelId.is_in(channel_ids.iter().map(|id| id.0)))
            .all(&*tx)
            .await?
            .into_iter()
            .flat_map(|project| {
                if project.deleted_at.is_some() {
                    return None;
                }
                match project.visibility {
                    ChannelVisibility::Public => {}
                    ChannelVisibility::Members => {
                        let is_visible = roles
                            .get(&project.channel_id)
                            .map(|role| role.can_see_all_descendants())
                            .unwrap_or(false);
                        if !is_visible {
                            return None;
                        }
                    }
                };
                Some(proto::HostedProject {
                    id: project.id.to_proto(),
                    channel_id: project.channel_id.to_proto(),
                    name: project.name.clone(),
                    visibility: project.visibility.into(),
                })
            })
            .collect())
    }

    pub async fn get_hosted_project(
        &self,
        hosted_project_id: HostedProjectId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<(hosted_project::Model, ChannelRole)> {
        let project = hosted_project::Entity::find_by_id(hosted_project_id)
            .one(&*tx)
            .await?
            .ok_or_else(|| anyhow!(ErrorCode::NoSuchProject))?;
        let channel = channel::Entity::find_by_id(project.channel_id)
            .one(&*tx)
            .await?
            .ok_or_else(|| anyhow!(ErrorCode::NoSuchChannel))?;

        let role = match project.visibility {
            ChannelVisibility::Public => {
                self.check_user_is_channel_participant(&channel, user_id, tx)
                    .await?
            }
            ChannelVisibility::Members => {
                self.check_user_is_channel_member(&channel, user_id, tx)
                    .await?
            }
        };

        Ok((project, role))
    }

    pub async fn is_hosted_project(&self, project_id: ProjectId) -> Result<bool> {
        self.transaction(|tx| async move {
            Ok(project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .map(|project| project.hosted_project_id.is_some())
                .ok_or_else(|| anyhow!(ErrorCode::NoSuchProject))?)
        })
        .await
    }
}
