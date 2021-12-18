use crate::db::{ChannelId, UserId};
use anyhow::anyhow;
use collections::{HashMap, HashSet};
use rpc::{proto, ConnectionId};
use std::collections::hash_map;

#[derive(Default)]
pub struct Store {
    connections: HashMap<ConnectionId, ConnectionState>,
    connections_by_user_id: HashMap<UserId, HashSet<ConnectionId>>,
    projects: HashMap<u64, Project>,
    visible_projects_by_user_id: HashMap<UserId, HashSet<u64>>,
    channels: HashMap<ChannelId, Channel>,
    next_worktree_id: u64,
}

struct ConnectionState {
    user_id: UserId,
    projects: HashSet<u64>,
    channels: HashSet<ChannelId>,
}

pub struct Project {
    pub host_connection_id: ConnectionId,
    pub host_user_id: UserId,
    pub share: Option<ProjectShare>,
    worktrees: HashMap<u64, Worktree>,
}

pub struct Worktree {
    pub authorized_user_ids: Vec<UserId>,
    pub root_name: String,
}

#[derive(Default)]
pub struct ProjectShare {
    pub guests: HashMap<ConnectionId, (ReplicaId, UserId)>,
    pub active_replica_ids: HashSet<ReplicaId>,
    pub worktrees: HashMap<u64, WorktreeShare>,
}

pub struct WorktreeShare {
    pub entries: HashMap<u64, proto::Entry>,
}

#[derive(Default)]
pub struct Channel {
    pub connection_ids: HashSet<ConnectionId>,
}

pub type ReplicaId = u16;

#[derive(Default)]
pub struct RemovedConnectionState {
    pub hosted_projects: HashMap<u64, Project>,
    pub guest_project_ids: HashMap<u64, Vec<ConnectionId>>,
    pub contact_ids: HashSet<UserId>,
}

pub struct JoinedWorktree<'a> {
    pub replica_id: ReplicaId,
    pub worktree: &'a Worktree,
}

pub struct UnsharedWorktree {
    pub connection_ids: Vec<ConnectionId>,
    pub authorized_user_ids: Vec<UserId>,
}

pub struct LeftWorktree {
    pub connection_ids: Vec<ConnectionId>,
    pub authorized_user_ids: Vec<UserId>,
}

impl Store {
    pub fn add_connection(&mut self, connection_id: ConnectionId, user_id: UserId) {
        self.connections.insert(
            connection_id,
            ConnectionState {
                user_id,
                projects: Default::default(),
                channels: Default::default(),
            },
        );
        self.connections_by_user_id
            .entry(user_id)
            .or_default()
            .insert(connection_id);
    }

    pub fn remove_connection(
        &mut self,
        connection_id: ConnectionId,
    ) -> tide::Result<RemovedConnectionState> {
        let connection = if let Some(connection) = self.connections.remove(&connection_id) {
            connection
        } else {
            return Err(anyhow!("no such connection"))?;
        };

        for channel_id in &connection.channels {
            if let Some(channel) = self.channels.get_mut(&channel_id) {
                channel.connection_ids.remove(&connection_id);
            }
        }

        let user_connections = self
            .connections_by_user_id
            .get_mut(&connection.user_id)
            .unwrap();
        user_connections.remove(&connection_id);
        if user_connections.is_empty() {
            self.connections_by_user_id.remove(&connection.user_id);
        }

        let mut result = RemovedConnectionState::default();
        for worktree_id in connection.worktrees.clone() {
            if let Ok(worktree) = self.unregister_worktree(worktree_id, connection_id) {
                result
                    .contact_ids
                    .extend(worktree.authorized_user_ids.iter().copied());
                result.hosted_worktrees.insert(worktree_id, worktree);
            } else if let Some(worktree) = self.leave_worktree(connection_id, worktree_id) {
                result
                    .guest_worktree_ids
                    .insert(worktree_id, worktree.connection_ids);
                result.contact_ids.extend(worktree.authorized_user_ids);
            }
        }

        #[cfg(test)]
        self.check_invariants();

        Ok(result)
    }

    #[cfg(test)]
    pub fn channel(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.get(&id)
    }

    pub fn join_channel(&mut self, connection_id: ConnectionId, channel_id: ChannelId) {
        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.channels.insert(channel_id);
            self.channels
                .entry(channel_id)
                .or_default()
                .connection_ids
                .insert(connection_id);
        }
    }

    pub fn leave_channel(&mut self, connection_id: ConnectionId, channel_id: ChannelId) {
        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.channels.remove(&channel_id);
            if let hash_map::Entry::Occupied(mut entry) = self.channels.entry(channel_id) {
                entry.get_mut().connection_ids.remove(&connection_id);
                if entry.get_mut().connection_ids.is_empty() {
                    entry.remove();
                }
            }
        }
    }

    pub fn user_id_for_connection(&self, connection_id: ConnectionId) -> tide::Result<UserId> {
        Ok(self
            .connections
            .get(&connection_id)
            .ok_or_else(|| anyhow!("unknown connection"))?
            .user_id)
    }

    pub fn connection_ids_for_user<'a>(
        &'a self,
        user_id: UserId,
    ) -> impl 'a + Iterator<Item = ConnectionId> {
        self.connections_by_user_id
            .get(&user_id)
            .into_iter()
            .flatten()
            .copied()
    }

    pub fn contacts_for_user(&self, user_id: UserId) -> Vec<proto::Contact> {
        let mut contacts = HashMap::default();
        for project_id in self
            .visible_projects_by_user_id
            .get(&user_id)
            .unwrap_or(&HashSet::default())
        {
            let project = &self.projects[project_id];

            let mut guests = HashSet::default();
            if let Ok(share) = worktree.share() {
                for guest_connection_id in share.guests.keys() {
                    if let Ok(user_id) = self.user_id_for_connection(*guest_connection_id) {
                        guests.insert(user_id.to_proto());
                    }
                }
            }

            if let Ok(host_user_id) = self.user_id_for_connection(project.host_connection_id) {
                contacts
                    .entry(host_user_id)
                    .or_insert_with(|| proto::Contact {
                        user_id: host_user_id.to_proto(),
                        projects: Vec::new(),
                    })
                    .projects
                    .push(proto::ProjectMetadata {
                        id: *project_id,
                        worktree_root_names: project
                            .worktrees
                            .iter()
                            .map(|worktree| worktree.root_name.clone())
                            .collect(),
                        is_shared: project.share.is_some(),
                        guests: guests.into_iter().collect(),
                    });
            }
        }

        contacts.into_values().collect()
    }

    pub fn register_project(
        &mut self,
        host_connection_id: ConnectionId,
        host_user_id: UserId,
    ) -> u64 {
        let project_id = self.next_project_id;
        self.projects.insert(
            project_id,
            Project {
                host_connection_id,
                host_user_id,
                share: None,
                worktrees: Default::default(),
            },
        );
        self.next_project_id += 1;
        project_id
    }

    pub fn register_worktree(
        &mut self,
        project_id: u64,
        worktree_id: u64,
        worktree: Worktree,
    ) -> bool {
        if let Some(project) = self.projects.get_mut(&project_id) {
            for authorized_user_id in &worktree.authorized_user_ids {
                self.visible_projects_by_user_id
                    .entry(*authorized_user_id)
                    .or_default()
                    .insert(project_id);
            }
            if let Some(connection) = self.connections.get_mut(&project.host_connection_id) {
                connection.projects.insert(project_id);
            }
            project.worktrees.insert(worktree_id, worktree);

            #[cfg(test)]
            self.check_invariants();
            true
        } else {
            false
        }
    }

    pub fn unregister_project(&mut self, project_id: u64) {
        todo!()
    }

    pub fn unregister_worktree(
        &mut self,
        project_id: u64,
        worktree_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<Worktree> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id != acting_connection_id {
            Err(anyhow!("not your worktree"))?;
        }

        let worktree = project
            .worktrees
            .remove(&worktree_id)
            .ok_or_else(|| anyhow!("no such worktree"))?;

        if let Some(connection) = self.connections.get_mut(&project.host_connection_id) {
            connection.worktrees.remove(&worktree_id);
        }

        if let Some(share) = &worktree.share {
            for connection_id in share.guests.keys() {
                if let Some(connection) = self.connections.get_mut(connection_id) {
                    connection.worktrees.remove(&worktree_id);
                }
            }
        }

        for authorized_user_id in &worktree.authorized_user_ids {
            if let Some(visible_worktrees) = self
                .visible_worktrees_by_user_id
                .get_mut(&authorized_user_id)
            {
                visible_worktrees.remove(&worktree_id);
            }
        }

        #[cfg(test)]
        self.check_invariants();

        Ok(worktree)
    }

    pub fn share_project(&mut self, project_id: u64, connection_id: ConnectionId) -> bool {
        if let Some(project) = self.projects.get_mut(&project_id) {
            if project.host_connection_id == connection_id {
                project.share = Some(ProjectShare::default());
                return true;
            }
        }
        false
    }

    pub fn share_worktree(
        &mut self,
        project_id: u64,
        worktree_id: u64,
        connection_id: ConnectionId,
        entries: HashMap<u64, proto::Entry>,
    ) -> Option<Vec<UserId>> {
        if let Some(project) = self.projects.get_mut(&project_id) {
            if project.host_connection_id == connection_id {
                if let Some(share) = project.share.as_mut() {
                    share
                        .worktrees
                        .insert(worktree_id, WorktreeShare { entries });
                    return Some(project.authorized_user_ids());
                }
            }
        }
        None
    }

    pub fn unshare_worktree(
        &mut self,
        worktree_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<UnsharedWorktree> {
        let worktree = if let Some(worktree) = self.worktrees.get_mut(&worktree_id) {
            worktree
        } else {
            return Err(anyhow!("no such worktree"))?;
        };

        if worktree.host_connection_id != acting_connection_id {
            return Err(anyhow!("not your worktree"))?;
        }

        let connection_ids = worktree.connection_ids();
        let authorized_user_ids = worktree.authorized_user_ids.clone();
        if let Some(share) = worktree.share.take() {
            for connection_id in share.guests.into_keys() {
                if let Some(connection) = self.connections.get_mut(&connection_id) {
                    connection.worktrees.remove(&worktree_id);
                }
            }

            #[cfg(test)]
            self.check_invariants();

            Ok(UnsharedWorktree {
                connection_ids,
                authorized_user_ids,
            })
        } else {
            Err(anyhow!("worktree is not shared"))?
        }
    }

    pub fn join_worktree(
        &mut self,
        connection_id: ConnectionId,
        user_id: UserId,
        worktree_id: u64,
    ) -> tide::Result<JoinedWorktree> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let worktree = self
            .worktrees
            .get_mut(&worktree_id)
            .and_then(|worktree| {
                if worktree.authorized_user_ids.contains(&user_id) {
                    Some(worktree)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("no such worktree"))?;

        let share = worktree.share_mut()?;
        connection.worktrees.insert(worktree_id);

        let mut replica_id = 1;
        while share.active_replica_ids.contains(&replica_id) {
            replica_id += 1;
        }
        share.active_replica_ids.insert(replica_id);
        share.guests.insert(connection_id, (replica_id, user_id));

        #[cfg(test)]
        self.check_invariants();

        Ok(JoinedWorktree {
            replica_id,
            worktree: &self.worktrees[&worktree_id],
        })
    }

    pub fn leave_worktree(
        &mut self,
        connection_id: ConnectionId,
        worktree_id: u64,
    ) -> Option<LeftWorktree> {
        let worktree = self.worktrees.get_mut(&worktree_id)?;
        let share = worktree.share.as_mut()?;
        let (replica_id, _) = share.guests.remove(&connection_id)?;
        share.active_replica_ids.remove(&replica_id);

        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.worktrees.remove(&worktree_id);
        }

        let connection_ids = worktree.connection_ids();
        let authorized_user_ids = worktree.authorized_user_ids.clone();

        #[cfg(test)]
        self.check_invariants();

        Some(LeftWorktree {
            connection_ids,
            authorized_user_ids,
        })
    }

    pub fn update_worktree(
        &mut self,
        connection_id: ConnectionId,
        worktree_id: u64,
        removed_entries: &[u64],
        updated_entries: &[proto::Entry],
    ) -> tide::Result<Vec<ConnectionId>> {
        let worktree = self.write_worktree(worktree_id, connection_id)?;
        let share = worktree.share_mut()?;
        for entry_id in removed_entries {
            share.entries.remove(&entry_id);
        }
        for entry in updated_entries {
            share.entries.insert(entry.id, entry.clone());
        }
        Ok(worktree.connection_ids())
    }

    pub fn worktree_host_connection_id(
        &self,
        connection_id: ConnectionId,
        worktree_id: u64,
    ) -> tide::Result<ConnectionId> {
        Ok(self
            .read_worktree(worktree_id, connection_id)?
            .host_connection_id)
    }

    pub fn worktree_guest_connection_ids(
        &self,
        connection_id: ConnectionId,
        worktree_id: u64,
    ) -> tide::Result<Vec<ConnectionId>> {
        Ok(self
            .read_worktree(worktree_id, connection_id)?
            .share()?
            .guests
            .keys()
            .copied()
            .collect())
    }

    pub fn worktree_connection_ids(
        &self,
        connection_id: ConnectionId,
        worktree_id: u64,
    ) -> tide::Result<Vec<ConnectionId>> {
        Ok(self
            .read_worktree(worktree_id, connection_id)?
            .connection_ids())
    }

    pub fn channel_connection_ids(&self, channel_id: ChannelId) -> Option<Vec<ConnectionId>> {
        Some(self.channels.get(&channel_id)?.connection_ids())
    }

    fn read_worktree(
        &self,
        worktree_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<&Worktree> {
        let worktree = self
            .worktrees
            .get(&worktree_id)
            .ok_or_else(|| anyhow!("worktree not found"))?;

        if worktree.host_connection_id == connection_id
            || worktree.share()?.guests.contains_key(&connection_id)
        {
            Ok(worktree)
        } else {
            Err(anyhow!(
                "{} is not a member of worktree {}",
                connection_id,
                worktree_id
            ))?
        }
    }

    fn write_worktree(
        &mut self,
        worktree_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<&mut Worktree> {
        let worktree = self
            .worktrees
            .get_mut(&worktree_id)
            .ok_or_else(|| anyhow!("worktree not found"))?;

        if worktree.host_connection_id == connection_id
            || worktree
                .share
                .as_ref()
                .map_or(false, |share| share.guests.contains_key(&connection_id))
        {
            Ok(worktree)
        } else {
            Err(anyhow!(
                "{} is not a member of worktree {}",
                connection_id,
                worktree_id
            ))?
        }
    }

    #[cfg(test)]
    fn check_invariants(&self) {
        for (connection_id, connection) in &self.connections {
            for worktree_id in &connection.worktrees {
                let worktree = &self.worktrees.get(&worktree_id).unwrap();
                if worktree.host_connection_id != *connection_id {
                    assert!(worktree.share().unwrap().guests.contains_key(connection_id));
                }
            }
            for channel_id in &connection.channels {
                let channel = self.channels.get(channel_id).unwrap();
                assert!(channel.connection_ids.contains(connection_id));
            }
            assert!(self
                .connections_by_user_id
                .get(&connection.user_id)
                .unwrap()
                .contains(connection_id));
        }

        for (user_id, connection_ids) in &self.connections_by_user_id {
            for connection_id in connection_ids {
                assert_eq!(
                    self.connections.get(connection_id).unwrap().user_id,
                    *user_id
                );
            }
        }

        for (worktree_id, worktree) in &self.worktrees {
            let host_connection = self.connections.get(&worktree.host_connection_id).unwrap();
            assert!(host_connection.worktrees.contains(worktree_id));

            for authorized_user_ids in &worktree.authorized_user_ids {
                let visible_worktree_ids = self
                    .visible_worktrees_by_user_id
                    .get(authorized_user_ids)
                    .unwrap();
                assert!(visible_worktree_ids.contains(worktree_id));
            }

            if let Some(share) = &worktree.share {
                for guest_connection_id in share.guests.keys() {
                    let guest_connection = self.connections.get(guest_connection_id).unwrap();
                    assert!(guest_connection.worktrees.contains(worktree_id));
                }
                assert_eq!(share.active_replica_ids.len(), share.guests.len(),);
                assert_eq!(
                    share.active_replica_ids,
                    share
                        .guests
                        .values()
                        .map(|(replica_id, _)| *replica_id)
                        .collect::<HashSet<_>>(),
                );
            }
        }

        for (user_id, visible_worktree_ids) in &self.visible_worktrees_by_user_id {
            for worktree_id in visible_worktree_ids {
                let worktree = self.worktrees.get(worktree_id).unwrap();
                assert!(worktree.authorized_user_ids.contains(user_id));
            }
        }

        for (channel_id, channel) in &self.channels {
            for connection_id in &channel.connection_ids {
                let connection = self.connections.get(connection_id).unwrap();
                assert!(connection.channels.contains(channel_id));
            }
        }
    }
}

impl Worktree {
    pub fn connection_ids(&self) -> Vec<ConnectionId> {
        if let Some(share) = &self.share {
            share
                .guests
                .keys()
                .copied()
                .chain(Some(self.host_connection_id))
                .collect()
        } else {
            vec![self.host_connection_id]
        }
    }

    pub fn share(&self) -> tide::Result<&ProjectShare> {
        Ok(self
            .share
            .as_ref()
            .ok_or_else(|| anyhow!("worktree is not shared"))?)
    }

    fn share_mut(&mut self) -> tide::Result<&mut ProjectShare> {
        Ok(self
            .share
            .as_mut()
            .ok_or_else(|| anyhow!("worktree is not shared"))?)
    }
}

impl Channel {
    fn connection_ids(&self) -> Vec<ConnectionId> {
        self.connection_ids.iter().copied().collect()
    }
}
