use rpc::proto;

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
}
