use crate::db::{ChannelId, UserId};
use anyhow::anyhow;
use collections::{BTreeMap, HashMap, HashSet};
use rpc::{proto, ConnectionId};
use std::{collections::hash_map, path::PathBuf};

#[derive(Default)]
pub struct Store {
    connections: HashMap<ConnectionId, ConnectionState>,
    connections_by_user_id: HashMap<UserId, HashSet<ConnectionId>>,
    projects: HashMap<u64, Project>,
    visible_projects_by_user_id: HashMap<UserId, HashSet<u64>>,
    channels: HashMap<ChannelId, Channel>,
    next_project_id: u64,
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
    pub worktrees: HashMap<u64, Worktree>,
}

pub struct Worktree {
    pub authorized_user_ids: Vec<UserId>,
    pub root_name: String,
    pub share: Option<WorktreeShare>,
    pub weak: bool,
}

#[derive(Default)]
pub struct ProjectShare {
    pub guests: HashMap<ConnectionId, (ReplicaId, UserId)>,
    pub active_replica_ids: HashSet<ReplicaId>,
}

pub struct WorktreeShare {
    pub entries: HashMap<u64, proto::Entry>,
    pub diagnostic_summaries: BTreeMap<PathBuf, proto::DiagnosticSummary>,
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

pub struct JoinedProject<'a> {
    pub replica_id: ReplicaId,
    pub project: &'a Project,
}

pub struct UnsharedProject {
    pub connection_ids: Vec<ConnectionId>,
    pub authorized_user_ids: Vec<UserId>,
}

pub struct LeftProject {
    pub connection_ids: Vec<ConnectionId>,
    pub authorized_user_ids: Vec<UserId>,
}

pub struct SharedWorktree {
    pub authorized_user_ids: Vec<UserId>,
    pub connection_ids: Vec<ConnectionId>,
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
        for project_id in connection.projects.clone() {
            if let Ok(project) = self.unregister_project(project_id, connection_id) {
                result.contact_ids.extend(project.authorized_user_ids());
                result.hosted_projects.insert(project_id, project);
            } else if let Ok(project) = self.leave_project(connection_id, project_id) {
                result
                    .guest_project_ids
                    .insert(project_id, project.connection_ids);
                result.contact_ids.extend(project.authorized_user_ids);
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
            if let Ok(share) = project.share() {
                for guest_connection_id in share.guests.keys() {
                    if let Ok(user_id) = self.user_id_for_connection(*guest_connection_id) {
                        guests.insert(user_id.to_proto());
                    }
                }
            }

            if let Ok(host_user_id) = self.user_id_for_connection(project.host_connection_id) {
                let mut worktree_root_names = project
                    .worktrees
                    .values()
                    .filter(|worktree| !worktree.weak)
                    .map(|worktree| worktree.root_name.clone())
                    .collect::<Vec<_>>();
                worktree_root_names.sort_unstable();
                contacts
                    .entry(host_user_id)
                    .or_insert_with(|| proto::Contact {
                        user_id: host_user_id.to_proto(),
                        projects: Vec::new(),
                    })
                    .projects
                    .push(proto::ProjectMetadata {
                        id: *project_id,
                        worktree_root_names,
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
        connection_id: ConnectionId,
        worktree: Worktree,
    ) -> tide::Result<()> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id {
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
            Ok(())
        } else {
            Err(anyhow!("no such project"))?
        }
    }

    pub fn unregister_project(
        &mut self,
        project_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<Project> {
        match self.projects.entry(project_id) {
            hash_map::Entry::Occupied(e) => {
                if e.get().host_connection_id == connection_id {
                    for user_id in e.get().authorized_user_ids() {
                        if let hash_map::Entry::Occupied(mut projects) =
                            self.visible_projects_by_user_id.entry(user_id)
                        {
                            projects.get_mut().remove(&project_id);
                        }
                    }

                    Ok(e.remove())
                } else {
                    Err(anyhow!("no such project"))?
                }
            }
            hash_map::Entry::Vacant(_) => Err(anyhow!("no such project"))?,
        }
    }

    pub fn unregister_worktree(
        &mut self,
        project_id: u64,
        worktree_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<(Worktree, Vec<ConnectionId>)> {
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

        let mut guest_connection_ids = Vec::new();
        if let Some(share) = &project.share {
            guest_connection_ids.extend(share.guests.keys());
        }

        for authorized_user_id in &worktree.authorized_user_ids {
            if let Some(visible_projects) =
                self.visible_projects_by_user_id.get_mut(authorized_user_id)
            {
                if !project.has_authorized_user_id(*authorized_user_id) {
                    visible_projects.remove(&project_id);
                }
            }
        }

        #[cfg(test)]
        self.check_invariants();

        Ok((worktree, guest_connection_ids))
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

    pub fn unshare_project(
        &mut self,
        project_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<UnsharedProject> {
        let project = if let Some(project) = self.projects.get_mut(&project_id) {
            project
        } else {
            return Err(anyhow!("no such project"))?;
        };

        if project.host_connection_id != acting_connection_id {
            return Err(anyhow!("not your project"))?;
        }

        let connection_ids = project.connection_ids();
        let authorized_user_ids = project.authorized_user_ids();
        if let Some(share) = project.share.take() {
            for connection_id in share.guests.into_keys() {
                if let Some(connection) = self.connections.get_mut(&connection_id) {
                    connection.projects.remove(&project_id);
                }
            }

            for worktree in project.worktrees.values_mut() {
                worktree.share.take();
            }

            #[cfg(test)]
            self.check_invariants();

            Ok(UnsharedProject {
                connection_ids,
                authorized_user_ids,
            })
        } else {
            Err(anyhow!("project is not shared"))?
        }
    }

    pub fn update_diagnostic_summary(
        &mut self,
        project_id: u64,
        worktree_id: u64,
        connection_id: ConnectionId,
        summary: proto::DiagnosticSummary,
    ) -> tide::Result<Vec<ConnectionId>> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        let worktree = project
            .worktrees
            .get_mut(&worktree_id)
            .ok_or_else(|| anyhow!("no such worktree"))?;
        if project.host_connection_id == connection_id {
            if let Some(share) = worktree.share.as_mut() {
                share
                    .diagnostic_summaries
                    .insert(summary.path.clone().into(), summary);
                return Ok(project.connection_ids());
            }
        }

        Err(anyhow!("no such worktree"))?
    }

    pub fn join_project(
        &mut self,
        connection_id: ConnectionId,
        user_id: UserId,
        project_id: u64,
    ) -> tide::Result<JoinedProject> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let project = self
            .projects
            .get_mut(&project_id)
            .and_then(|project| {
                if project.has_authorized_user_id(user_id) {
                    Some(project)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("no such project"))?;

        let share = project.share_mut()?;
        connection.projects.insert(project_id);

        let mut replica_id = 1;
        while share.active_replica_ids.contains(&replica_id) {
            replica_id += 1;
        }
        share.active_replica_ids.insert(replica_id);
        share.guests.insert(connection_id, (replica_id, user_id));

        #[cfg(test)]
        self.check_invariants();

        Ok(JoinedProject {
            replica_id,
            project: &self.projects[&project_id],
        })
    }

    pub fn leave_project(
        &mut self,
        connection_id: ConnectionId,
        project_id: u64,
    ) -> tide::Result<LeftProject> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        let share = project
            .share
            .as_mut()
            .ok_or_else(|| anyhow!("project is not shared"))?;
        let (replica_id, _) = share
            .guests
            .remove(&connection_id)
            .ok_or_else(|| anyhow!("cannot leave a project before joining it"))?;
        share.active_replica_ids.remove(&replica_id);

        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.projects.remove(&project_id);
        }

        let connection_ids = project.connection_ids();
        let authorized_user_ids = project.authorized_user_ids();

        #[cfg(test)]
        self.check_invariants();

        Ok(LeftProject {
            connection_ids,
            authorized_user_ids,
        })
    }

    pub fn update_worktree(
        &mut self,
        connection_id: ConnectionId,
        project_id: u64,
        worktree_id: u64,
        removed_entries: &[u64],
        updated_entries: &[proto::Entry],
    ) -> tide::Result<Vec<ConnectionId>> {
        let project = self.write_project(project_id, connection_id)?;
        let share = project
            .worktrees
            .get_mut(&worktree_id)
            .ok_or_else(|| anyhow!("no such worktree"))?
            .share
            .as_mut()
            .ok_or_else(|| anyhow!("worktree is not shared"))?;
        for entry_id in removed_entries {
            share.entries.remove(&entry_id);
        }
        for entry in updated_entries {
            share.entries.insert(entry.id, entry.clone());
        }
        Ok(project.connection_ids())
    }

    pub fn project_connection_ids(
        &self,
        project_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<Vec<ConnectionId>> {
        Ok(self
            .read_project(project_id, acting_connection_id)?
            .connection_ids())
    }

    pub fn channel_connection_ids(&self, channel_id: ChannelId) -> tide::Result<Vec<ConnectionId>> {
        Ok(self
            .channels
            .get(&channel_id)
            .ok_or_else(|| anyhow!("no such channel"))?
            .connection_ids())
    }

    #[cfg(test)]
    pub fn project(&self, project_id: u64) -> Option<&Project> {
        self.projects.get(&project_id)
    }

    pub fn read_project(
        &self,
        project_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<&Project> {
        let project = self
            .projects
            .get(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id
            || project
                .share
                .as_ref()
                .ok_or_else(|| anyhow!("project is not shared"))?
                .guests
                .contains_key(&connection_id)
        {
            Ok(project)
        } else {
            Err(anyhow!("no such project"))?
        }
    }

    fn write_project(
        &mut self,
        project_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<&mut Project> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id
            || project
                .share
                .as_ref()
                .ok_or_else(|| anyhow!("project is not shared"))?
                .guests
                .contains_key(&connection_id)
        {
            Ok(project)
        } else {
            Err(anyhow!("no such project"))?
        }
    }

    #[cfg(test)]
    fn check_invariants(&self) {
        for (connection_id, connection) in &self.connections {
            for project_id in &connection.projects {
                let project = &self.projects.get(&project_id).unwrap();
                if project.host_connection_id != *connection_id {
                    assert!(project
                        .share
                        .as_ref()
                        .unwrap()
                        .guests
                        .contains_key(connection_id));
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

        for (project_id, project) in &self.projects {
            let host_connection = self.connections.get(&project.host_connection_id).unwrap();
            assert!(host_connection.projects.contains(project_id));

            for authorized_user_ids in project.authorized_user_ids() {
                let visible_project_ids = self
                    .visible_projects_by_user_id
                    .get(&authorized_user_ids)
                    .unwrap();
                assert!(visible_project_ids.contains(project_id));
            }

            if let Some(share) = &project.share {
                for guest_connection_id in share.guests.keys() {
                    let guest_connection = self.connections.get(guest_connection_id).unwrap();
                    assert!(guest_connection.projects.contains(project_id));
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

        for (user_id, visible_project_ids) in &self.visible_projects_by_user_id {
            for project_id in visible_project_ids {
                let project = self.projects.get(project_id).unwrap();
                assert!(project.authorized_user_ids().contains(user_id));
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

impl Project {
    pub fn has_authorized_user_id(&self, user_id: UserId) -> bool {
        self.worktrees
            .values()
            .any(|worktree| worktree.authorized_user_ids.contains(&user_id))
    }

    pub fn authorized_user_ids(&self) -> Vec<UserId> {
        let mut ids = self
            .worktrees
            .values()
            .flat_map(|worktree| worktree.authorized_user_ids.iter())
            .copied()
            .collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    pub fn guest_connection_ids(&self) -> Vec<ConnectionId> {
        if let Some(share) = &self.share {
            share.guests.keys().copied().collect()
        } else {
            Vec::new()
        }
    }

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
