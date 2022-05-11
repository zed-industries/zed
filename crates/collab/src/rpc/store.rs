use crate::db::{self, ChannelId, UserId};
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashMap, HashSet};
use rpc::{proto, ConnectionId};
use std::{collections::hash_map, path::PathBuf};
use tracing::instrument;

#[derive(Default)]
pub struct Store {
    connections: HashMap<ConnectionId, ConnectionState>,
    connections_by_user_id: HashMap<UserId, HashSet<ConnectionId>>,
    projects: HashMap<u64, Project>,
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
    pub language_servers: Vec<proto::LanguageServer>,
}

pub struct Worktree {
    pub root_name: String,
    pub visible: bool,
}

#[derive(Default)]
pub struct ProjectShare {
    pub guests: HashMap<ConnectionId, (ReplicaId, UserId)>,
    pub active_replica_ids: HashSet<ReplicaId>,
    pub worktrees: HashMap<u64, WorktreeShare>,
}

#[derive(Default)]
pub struct WorktreeShare {
    pub entries: HashMap<u64, proto::Entry>,
    pub diagnostic_summaries: BTreeMap<PathBuf, proto::DiagnosticSummary>,
    pub scan_id: u64,
}

#[derive(Default)]
pub struct Channel {
    pub connection_ids: HashSet<ConnectionId>,
}

pub type ReplicaId = u16;

#[derive(Default)]
pub struct RemovedConnectionState {
    pub user_id: UserId,
    pub hosted_projects: HashMap<u64, Project>,
    pub guest_project_ids: HashMap<u64, Vec<ConnectionId>>,
    pub contact_ids: HashSet<UserId>,
}

pub struct JoinedProject<'a> {
    pub replica_id: ReplicaId,
    pub project: &'a Project,
}

pub struct SharedProject {}

pub struct UnsharedProject {
    pub connection_ids: Vec<ConnectionId>,
    pub host_user_id: UserId,
}

pub struct LeftProject {
    pub connection_ids: Vec<ConnectionId>,
    pub host_user_id: UserId,
}

#[derive(Copy, Clone)]
pub struct Metrics {
    pub connections: usize,
    pub registered_projects: usize,
    pub shared_projects: usize,
}

impl Store {
    pub fn metrics(&self) -> Metrics {
        let connections = self.connections.len();
        let mut registered_projects = 0;
        let mut shared_projects = 0;
        for project in self.projects.values() {
            registered_projects += 1;
            if project.share.is_some() {
                shared_projects += 1;
            }
        }

        Metrics {
            connections,
            registered_projects,
            shared_projects,
        }
    }

    #[instrument(skip(self))]
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

    #[instrument(skip(self))]
    pub fn remove_connection(
        &mut self,
        connection_id: ConnectionId,
    ) -> Result<RemovedConnectionState> {
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
        result.user_id = connection.user_id;
        for project_id in connection.projects.clone() {
            if let Ok(project) = self.unregister_project(project_id, connection_id) {
                result.hosted_projects.insert(project_id, project);
            } else if let Ok(project) = self.leave_project(connection_id, project_id) {
                result
                    .guest_project_ids
                    .insert(project_id, project.connection_ids);
            }
        }

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

    pub fn user_id_for_connection(&self, connection_id: ConnectionId) -> Result<UserId> {
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

    pub fn is_user_online(&self, user_id: UserId) -> bool {
        !self
            .connections_by_user_id
            .get(&user_id)
            .unwrap_or(&Default::default())
            .is_empty()
    }

    pub fn build_initial_contacts_update(&self, contacts: db::Contacts) -> proto::UpdateContacts {
        let mut update = proto::UpdateContacts::default();
        for user_id in contacts.current {
            update.contacts.push(self.contact_for_user(user_id));
        }

        for request in contacts.incoming_requests {
            update
                .incoming_requests
                .push(proto::IncomingContactRequest {
                    requester_id: request.requester_id.to_proto(),
                    should_notify: request.should_notify,
                })
        }

        for requested_user_id in contacts.outgoing_requests {
            update.outgoing_requests.push(requested_user_id.to_proto())
        }

        update
    }

    pub fn contact_for_user(&self, user_id: UserId) -> proto::Contact {
        proto::Contact {
            user_id: user_id.to_proto(),
            projects: self.project_metadata_for_user(user_id),
            online: self.is_user_online(user_id),
        }
    }

    pub fn project_metadata_for_user(&self, user_id: UserId) -> Vec<proto::ProjectMetadata> {
        let connection_ids = self.connections_by_user_id.get(&user_id);
        let project_ids = connection_ids.iter().flat_map(|connection_ids| {
            connection_ids
                .iter()
                .filter_map(|connection_id| self.connections.get(connection_id))
                .flat_map(|connection| connection.projects.iter().copied())
        });

        let mut metadata = Vec::new();
        for project_id in project_ids {
            if let Some(project) = self.projects.get(&project_id) {
                if project.host_user_id == user_id {
                    metadata.push(proto::ProjectMetadata {
                        id: project_id,
                        is_shared: project.share.is_some(),
                        worktree_root_names: project
                            .worktrees
                            .values()
                            .map(|worktree| worktree.root_name.clone())
                            .collect(),
                        guests: project
                            .share
                            .iter()
                            .flat_map(|share| {
                                share.guests.values().map(|(_, user_id)| user_id.to_proto())
                            })
                            .collect(),
                    });
                }
            }
        }

        metadata
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
                language_servers: Default::default(),
            },
        );
        if let Some(connection) = self.connections.get_mut(&host_connection_id) {
            connection.projects.insert(project_id);
        }
        self.next_project_id += 1;
        project_id
    }

    pub fn register_worktree(
        &mut self,
        project_id: u64,
        worktree_id: u64,
        connection_id: ConnectionId,
        worktree: Worktree,
    ) -> Result<()> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id {
            project.worktrees.insert(worktree_id, worktree);
            if let Ok(share) = project.share_mut() {
                share.worktrees.insert(worktree_id, Default::default());
            }

            Ok(())
        } else {
            Err(anyhow!("no such project"))?
        }
    }

    pub fn unregister_project(
        &mut self,
        project_id: u64,
        connection_id: ConnectionId,
    ) -> Result<Project> {
        match self.projects.entry(project_id) {
            hash_map::Entry::Occupied(e) => {
                if e.get().host_connection_id == connection_id {
                    let project = e.remove();

                    if let Some(host_connection) = self.connections.get_mut(&connection_id) {
                        host_connection.projects.remove(&project_id);
                    }

                    if let Some(share) = &project.share {
                        for guest_connection in share.guests.keys() {
                            if let Some(connection) = self.connections.get_mut(&guest_connection) {
                                connection.projects.remove(&project_id);
                            }
                        }
                    }

                    Ok(project)
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
    ) -> Result<(Worktree, Vec<ConnectionId>)> {
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
        if let Ok(share) = project.share_mut() {
            guest_connection_ids.extend(share.guests.keys());
            share.worktrees.remove(&worktree_id);
        }

        Ok((worktree, guest_connection_ids))
    }

    pub fn share_project(
        &mut self,
        project_id: u64,
        connection_id: ConnectionId,
    ) -> Result<SharedProject> {
        if let Some(project) = self.projects.get_mut(&project_id) {
            if project.host_connection_id == connection_id {
                let mut share = ProjectShare::default();
                for worktree_id in project.worktrees.keys() {
                    share.worktrees.insert(*worktree_id, Default::default());
                }
                project.share = Some(share);
                return Ok(SharedProject {});
            }
        }
        Err(anyhow!("no such project"))?
    }

    pub fn unshare_project(
        &mut self,
        project_id: u64,
        acting_connection_id: ConnectionId,
    ) -> Result<UnsharedProject> {
        let project = if let Some(project) = self.projects.get_mut(&project_id) {
            project
        } else {
            return Err(anyhow!("no such project"))?;
        };

        if project.host_connection_id != acting_connection_id {
            return Err(anyhow!("not your project"))?;
        }

        let connection_ids = project.connection_ids();
        if let Some(share) = project.share.take() {
            for connection_id in share.guests.into_keys() {
                if let Some(connection) = self.connections.get_mut(&connection_id) {
                    connection.projects.remove(&project_id);
                }
            }

            Ok(UnsharedProject {
                connection_ids,
                host_user_id: project.host_user_id,
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
    ) -> Result<Vec<ConnectionId>> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id {
            let worktree = project
                .share_mut()?
                .worktrees
                .get_mut(&worktree_id)
                .ok_or_else(|| anyhow!("no such worktree"))?;
            worktree
                .diagnostic_summaries
                .insert(summary.path.clone().into(), summary);
            return Ok(project.connection_ids());
        }

        Err(anyhow!("no such worktree"))?
    }

    pub fn start_language_server(
        &mut self,
        project_id: u64,
        connection_id: ConnectionId,
        language_server: proto::LanguageServer,
    ) -> Result<Vec<ConnectionId>> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id {
            project.language_servers.push(language_server);
            return Ok(project.connection_ids());
        }

        Err(anyhow!("no such project"))?
    }

    pub fn join_project(
        &mut self,
        connection_id: ConnectionId,
        user_id: UserId,
        project_id: u64,
    ) -> Result<JoinedProject> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;

        let share = project.share_mut()?;
        connection.projects.insert(project_id);

        let mut replica_id = 1;
        while share.active_replica_ids.contains(&replica_id) {
            replica_id += 1;
        }
        share.active_replica_ids.insert(replica_id);
        share.guests.insert(connection_id, (replica_id, user_id));

        Ok(JoinedProject {
            replica_id,
            project: &self.projects[&project_id],
        })
    }

    pub fn leave_project(
        &mut self,
        connection_id: ConnectionId,
        project_id: u64,
    ) -> Result<LeftProject> {
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

        Ok(LeftProject {
            connection_ids: project.connection_ids(),
            host_user_id: project.host_user_id,
        })
    }

    pub fn update_worktree(
        &mut self,
        connection_id: ConnectionId,
        project_id: u64,
        worktree_id: u64,
        removed_entries: &[u64],
        updated_entries: &[proto::Entry],
        scan_id: u64,
    ) -> Result<Vec<ConnectionId>> {
        let project = self.write_project(project_id, connection_id)?;
        let worktree = project
            .share_mut()?
            .worktrees
            .get_mut(&worktree_id)
            .ok_or_else(|| anyhow!("no such worktree"))?;
        for entry_id in removed_entries {
            worktree.entries.remove(&entry_id);
        }
        for entry in updated_entries {
            worktree.entries.insert(entry.id, entry.clone());
        }
        worktree.scan_id = scan_id;
        let connection_ids = project.connection_ids();
        Ok(connection_ids)
    }

    pub fn project_connection_ids(
        &self,
        project_id: u64,
        acting_connection_id: ConnectionId,
    ) -> Result<Vec<ConnectionId>> {
        Ok(self
            .read_project(project_id, acting_connection_id)?
            .connection_ids())
    }

    pub fn channel_connection_ids(&self, channel_id: ChannelId) -> Result<Vec<ConnectionId>> {
        Ok(self
            .channels
            .get(&channel_id)
            .ok_or_else(|| anyhow!("no such channel"))?
            .connection_ids())
    }

    pub fn project(&self, project_id: u64) -> Result<&Project> {
        self.projects
            .get(&project_id)
            .ok_or_else(|| anyhow!("no such project"))
    }

    pub fn read_project(&self, project_id: u64, connection_id: ConnectionId) -> Result<&Project> {
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
    ) -> Result<&mut Project> {
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
    pub fn check_invariants(&self) {
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

                if let Some(share) = project.share.as_ref() {
                    for (worktree_id, worktree) in share.worktrees.iter() {
                        let mut paths = HashMap::default();
                        for entry in worktree.entries.values() {
                            let prev_entry = paths.insert(&entry.path, entry);
                            assert_eq!(
                                prev_entry,
                                None,
                                "worktree {:?}, duplicate path for entries {:?} and {:?}",
                                worktree_id,
                                prev_entry.unwrap(),
                                entry
                            );
                        }
                    }
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

        for (channel_id, channel) in &self.channels {
            for connection_id in &channel.connection_ids {
                let connection = self.connections.get(connection_id).unwrap();
                assert!(connection.channels.contains(channel_id));
            }
        }
    }
}

impl Project {
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

    pub fn share(&self) -> Result<&ProjectShare> {
        Ok(self
            .share
            .as_ref()
            .ok_or_else(|| anyhow!("worktree is not shared"))?)
    }

    fn share_mut(&mut self) -> Result<&mut ProjectShare> {
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
