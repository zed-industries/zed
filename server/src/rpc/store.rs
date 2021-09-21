use crate::db::{ChannelId, UserId};
use crate::errors::TideResultExt;
use anyhow::anyhow;
use std::collections::{hash_map, HashMap, HashSet};
use zrpc::{proto, ConnectionId};

#[derive(Default)]
pub struct Store {
    connections: HashMap<ConnectionId, ConnectionState>,
    connections_by_user_id: HashMap<UserId, HashSet<ConnectionId>>,
    worktrees: HashMap<u64, Worktree>,
    visible_worktrees_by_user_id: HashMap<UserId, HashSet<u64>>,
    channels: HashMap<ChannelId, Channel>,
    next_worktree_id: u64,
}

struct ConnectionState {
    user_id: UserId,
    worktrees: HashSet<u64>,
    channels: HashSet<ChannelId>,
}

pub struct Worktree {
    pub host_connection_id: ConnectionId,
    pub collaborator_user_ids: Vec<UserId>,
    pub root_name: String,
    pub share: Option<WorktreeShare>,
}

pub struct WorktreeShare {
    pub guest_connection_ids: HashMap<ConnectionId, ReplicaId>,
    pub active_replica_ids: HashSet<ReplicaId>,
    pub entries: HashMap<u64, proto::Entry>,
}

#[derive(Default)]
pub struct Channel {
    pub connection_ids: HashSet<ConnectionId>,
}

pub type ReplicaId = u16;

#[derive(Default)]
pub struct RemovedConnectionState {
    pub hosted_worktrees: HashMap<u64, Worktree>,
    pub guest_worktree_ids: HashMap<u64, Vec<ConnectionId>>,
    pub collaborator_ids: HashSet<UserId>,
}

impl Store {
    pub fn add_connection(&mut self, connection_id: ConnectionId, user_id: UserId) {
        self.connections.insert(
            connection_id,
            ConnectionState {
                user_id,
                worktrees: Default::default(),
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
        let connection = if let Some(connection) = self.connections.get(&connection_id) {
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
            if let Ok(worktree) = self.remove_worktree(worktree_id, connection_id) {
                result
                    .collaborator_ids
                    .extend(worktree.collaborator_user_ids.iter().copied());
                result.hosted_worktrees.insert(worktree_id, worktree);
            } else {
                if let Some(worktree) = self.worktrees.get(&worktree_id) {
                    result
                        .guest_worktree_ids
                        .insert(worktree_id, worktree.connection_ids());
                    result
                        .collaborator_ids
                        .extend(worktree.collaborator_user_ids.iter().copied());
                }
            }
        }

        Ok(result)
    }

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

    pub fn collaborators_for_user(&self, user_id: UserId) -> Vec<proto::Collaborator> {
        let mut collaborators = HashMap::new();
        for worktree_id in self
            .visible_worktrees_by_user_id
            .get(&user_id)
            .unwrap_or(&HashSet::new())
        {
            let worktree = &self.worktrees[worktree_id];

            let mut guests = HashSet::new();
            if let Ok(share) = worktree.share() {
                for guest_connection_id in share.guest_connection_ids.keys() {
                    if let Ok(user_id) = self.user_id_for_connection(*guest_connection_id) {
                        guests.insert(user_id.to_proto());
                    }
                }
            }

            if let Ok(host_user_id) = self
                .user_id_for_connection(worktree.host_connection_id)
                .context("stale worktree host connection")
            {
                let host =
                    collaborators
                        .entry(host_user_id)
                        .or_insert_with(|| proto::Collaborator {
                            user_id: host_user_id.to_proto(),
                            worktrees: Vec::new(),
                        });
                host.worktrees.push(proto::WorktreeMetadata {
                    root_name: worktree.root_name.clone(),
                    is_shared: worktree.share().is_ok(),
                    participants: guests.into_iter().collect(),
                });
            }
        }

        collaborators.into_values().collect()
    }

    pub fn add_worktree(&mut self, worktree: Worktree) -> u64 {
        let worktree_id = self.next_worktree_id;
        for collaborator_user_id in &worktree.collaborator_user_ids {
            self.visible_worktrees_by_user_id
                .entry(*collaborator_user_id)
                .or_default()
                .insert(worktree_id);
        }
        self.next_worktree_id += 1;
        if let Some(connection) = self.connections.get_mut(&worktree.host_connection_id) {
            connection.worktrees.insert(worktree_id);
        }
        self.worktrees.insert(worktree_id, worktree);

        #[cfg(test)]
        self.check_invariants();

        worktree_id
    }

    pub fn remove_worktree(
        &mut self,
        worktree_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<Worktree> {
        let worktree = if let hash_map::Entry::Occupied(e) = self.worktrees.entry(worktree_id) {
            if e.get().host_connection_id != acting_connection_id {
                Err(anyhow!("not your worktree"))?;
            }
            e.remove()
        } else {
            return Err(anyhow!("no such worktree"))?;
        };

        if let Some(connection) = self.connections.get_mut(&worktree.host_connection_id) {
            connection.worktrees.remove(&worktree_id);
        }

        if let Some(share) = &worktree.share {
            for connection_id in share.guest_connection_ids.keys() {
                if let Some(connection) = self.connections.get_mut(connection_id) {
                    connection.worktrees.remove(&worktree_id);
                }
            }
        }

        for collaborator_user_id in &worktree.collaborator_user_ids {
            if let Some(visible_worktrees) = self
                .visible_worktrees_by_user_id
                .get_mut(&collaborator_user_id)
            {
                visible_worktrees.remove(&worktree_id);
            }
        }

        #[cfg(test)]
        self.check_invariants();

        Ok(worktree)
    }

    pub fn share_worktree(
        &mut self,
        worktree_id: u64,
        connection_id: ConnectionId,
        entries: HashMap<u64, proto::Entry>,
    ) -> Option<Vec<UserId>> {
        if let Some(worktree) = self.worktrees.get_mut(&worktree_id) {
            if worktree.host_connection_id == connection_id {
                worktree.share = Some(WorktreeShare {
                    guest_connection_ids: Default::default(),
                    active_replica_ids: Default::default(),
                    entries,
                });
                return Some(worktree.collaborator_user_ids.clone());
            }
        }
        None
    }

    pub fn unshare_worktree(
        &mut self,
        worktree_id: u64,
        acting_connection_id: ConnectionId,
    ) -> tide::Result<(Vec<ConnectionId>, Vec<UserId>)> {
        let worktree = if let Some(worktree) = self.worktrees.get_mut(&worktree_id) {
            worktree
        } else {
            return Err(anyhow!("no such worktree"))?;
        };

        if worktree.host_connection_id != acting_connection_id {
            return Err(anyhow!("not your worktree"))?;
        }

        let connection_ids = worktree.connection_ids();

        if let Some(_) = worktree.share.take() {
            for connection_id in &connection_ids {
                if let Some(connection) = self.connections.get_mut(connection_id) {
                    connection.worktrees.remove(&worktree_id);
                }
            }
            Ok((connection_ids, worktree.collaborator_user_ids.clone()))
        } else {
            Err(anyhow!("worktree is not shared"))?
        }
    }

    pub fn join_worktree(
        &mut self,
        connection_id: ConnectionId,
        user_id: UserId,
        worktree_id: u64,
    ) -> tide::Result<(ReplicaId, &Worktree)> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let worktree = self
            .worktrees
            .get_mut(&worktree_id)
            .and_then(|worktree| {
                if worktree.collaborator_user_ids.contains(&user_id) {
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
        share.guest_connection_ids.insert(connection_id, replica_id);
        return Ok((replica_id, worktree));
    }

    pub fn leave_worktree(
        &mut self,
        connection_id: ConnectionId,
        worktree_id: u64,
    ) -> Option<(Vec<ConnectionId>, Vec<UserId>)> {
        let worktree = self.worktrees.get_mut(&worktree_id)?;
        let share = worktree.share.as_mut()?;
        let replica_id = share.guest_connection_ids.remove(&connection_id)?;
        share.active_replica_ids.remove(&replica_id);
        Some((
            worktree.connection_ids(),
            worktree.collaborator_user_ids.clone(),
        ))
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
            .guest_connection_ids
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
            || worktree
                .share()?
                .guest_connection_ids
                .contains_key(&connection_id)
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
            || worktree.share.as_ref().map_or(false, |share| {
                share.guest_connection_ids.contains_key(&connection_id)
            })
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
                    assert!(worktree
                        .share()
                        .unwrap()
                        .guest_connection_ids
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

        for (worktree_id, worktree) in &self.worktrees {
            let host_connection = self.connections.get(&worktree.host_connection_id).unwrap();
            assert!(host_connection.worktrees.contains(worktree_id));

            for collaborator_id in &worktree.collaborator_user_ids {
                let visible_worktree_ids = self
                    .visible_worktrees_by_user_id
                    .get(collaborator_id)
                    .unwrap();
                assert!(visible_worktree_ids.contains(worktree_id));
            }

            if let Some(share) = &worktree.share {
                for guest_connection_id in share.guest_connection_ids.keys() {
                    let guest_connection = self.connections.get(guest_connection_id).unwrap();
                    assert!(guest_connection.worktrees.contains(worktree_id));
                }
                assert_eq!(
                    share.active_replica_ids.len(),
                    share.guest_connection_ids.len(),
                );
                assert_eq!(
                    share.active_replica_ids,
                    share
                        .guest_connection_ids
                        .values()
                        .copied()
                        .collect::<HashSet<_>>(),
                );
            }
        }

        for (user_id, visible_worktree_ids) in &self.visible_worktrees_by_user_id {
            for worktree_id in visible_worktree_ids {
                let worktree = self.worktrees.get(worktree_id).unwrap();
                assert!(worktree.collaborator_user_ids.contains(user_id));
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
                .guest_connection_ids
                .keys()
                .copied()
                .chain(Some(self.host_connection_id))
                .collect()
        } else {
            vec![self.host_connection_id]
        }
    }

    pub fn share(&self) -> tide::Result<&WorktreeShare> {
        Ok(self
            .share
            .as_ref()
            .ok_or_else(|| anyhow!("worktree is not shared"))?)
    }

    fn share_mut(&mut self) -> tide::Result<&mut WorktreeShare> {
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
