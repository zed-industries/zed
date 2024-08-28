use crate::db::{ChannelId, ChannelRole, DevServerId, PrincipalId, UserId};
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashMap, HashSet};
use rpc::{proto, ConnectionId};
use semantic_version::SemanticVersion;
use serde::Serialize;
use std::fmt;
use tracing::instrument;

#[derive(Default, Serialize)]
pub struct ConnectionPool {
    connections: BTreeMap<ConnectionId, Connection>,
    connected_users: BTreeMap<UserId, ConnectedPrincipal>,
    connected_dev_servers: BTreeMap<DevServerId, ConnectionId>,
    channels: ChannelPool,
    offline_dev_servers: HashSet<DevServerId>,
}

#[derive(Default, Serialize)]
struct ConnectedPrincipal {
    connection_ids: HashSet<ConnectionId>,
}

#[derive(Copy, Clone, Debug, Serialize, PartialOrd, PartialEq, Eq, Ord)]
pub struct ZedVersion(pub SemanticVersion);

impl fmt::Display for ZedVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ZedVersion {
    pub fn can_collaborate(&self) -> bool {
        self.0 >= SemanticVersion::new(0, 134, 0)
    }

    pub fn with_list_directory() -> ZedVersion {
        ZedVersion(SemanticVersion::new(0, 145, 0))
    }

    pub fn with_search_candidates() -> ZedVersion {
        ZedVersion(SemanticVersion::new(0, 151, 0))
    }
}

#[derive(Serialize)]
pub struct Connection {
    pub principal_id: PrincipalId,
    pub admin: bool,
    pub zed_version: ZedVersion,
}

impl ConnectionPool {
    pub fn reset(&mut self) {
        self.connections.clear();
        self.connected_users.clear();
        self.connected_dev_servers.clear();
        self.channels.clear();
    }

    pub fn connection(&mut self, connection_id: ConnectionId) -> Option<&Connection> {
        self.connections.get(&connection_id)
    }

    #[instrument(skip(self))]
    pub fn add_connection(
        &mut self,
        connection_id: ConnectionId,
        user_id: UserId,
        admin: bool,
        zed_version: ZedVersion,
    ) {
        self.connections.insert(
            connection_id,
            Connection {
                principal_id: PrincipalId::UserId(user_id),
                admin,
                zed_version,
            },
        );
        let connected_user = self.connected_users.entry(user_id).or_default();
        connected_user.connection_ids.insert(connection_id);
    }

    pub fn add_dev_server(
        &mut self,
        connection_id: ConnectionId,
        dev_server_id: DevServerId,
        zed_version: ZedVersion,
    ) {
        self.connections.insert(
            connection_id,
            Connection {
                principal_id: PrincipalId::DevServerId(dev_server_id),
                admin: false,
                zed_version,
            },
        );

        self.connected_dev_servers
            .insert(dev_server_id, connection_id);
    }

    #[instrument(skip(self))]
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> Result<()> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;

        match connection.principal_id {
            PrincipalId::UserId(user_id) => {
                let connected_user = self.connected_users.get_mut(&user_id).unwrap();
                connected_user.connection_ids.remove(&connection_id);
                if connected_user.connection_ids.is_empty() {
                    self.connected_users.remove(&user_id);
                    self.channels.remove_user(&user_id);
                }
            }
            PrincipalId::DevServerId(dev_server_id) => {
                self.connected_dev_servers.remove(&dev_server_id);
                self.offline_dev_servers.remove(&dev_server_id);
            }
        }
        self.connections.remove(&connection_id).unwrap();
        Ok(())
    }

    pub fn set_dev_server_offline(&mut self, dev_server_id: DevServerId) {
        self.offline_dev_servers.insert(dev_server_id);
    }

    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.values()
    }

    pub fn user_connections(&self, user_id: UserId) -> impl Iterator<Item = &Connection> + '_ {
        self.connected_users
            .get(&user_id)
            .into_iter()
            .flat_map(|state| {
                state
                    .connection_ids
                    .iter()
                    .flat_map(|cid| self.connections.get(cid))
            })
    }

    pub fn user_connection_ids(&self, user_id: UserId) -> impl Iterator<Item = ConnectionId> + '_ {
        self.connected_users
            .get(&user_id)
            .into_iter()
            .flat_map(|state| &state.connection_ids)
            .copied()
    }

    pub fn dev_server_status(&self, dev_server_id: DevServerId) -> proto::DevServerStatus {
        if self.dev_server_connection_id(dev_server_id).is_some()
            && !self.offline_dev_servers.contains(&dev_server_id)
        {
            proto::DevServerStatus::Online
        } else {
            proto::DevServerStatus::Offline
        }
    }

    pub fn dev_server_connection_id(&self, dev_server_id: DevServerId) -> Option<ConnectionId> {
        self.connected_dev_servers.get(&dev_server_id).copied()
    }

    pub fn dev_server_connection_id_supporting(
        &self,
        dev_server_id: DevServerId,
        required: ZedVersion,
    ) -> Result<ConnectionId> {
        match self.connected_dev_servers.get(&dev_server_id) {
            Some(cid) if self.connections[cid].zed_version >= required => Ok(*cid),
            Some(_) => Err(anyhow!(proto::ErrorCode::RemoteUpgradeRequired)),
            None => Err(anyhow!(proto::ErrorCode::DevServerOffline)),
        }
    }

    pub fn channel_user_ids(
        &self,
        channel_id: ChannelId,
    ) -> impl Iterator<Item = (UserId, ChannelRole)> + '_ {
        self.channels.users_to_notify(channel_id)
    }

    pub fn channel_connection_ids(
        &self,
        channel_id: ChannelId,
    ) -> impl Iterator<Item = (ConnectionId, ChannelRole)> + '_ {
        self.channels
            .users_to_notify(channel_id)
            .flat_map(|(user_id, role)| {
                self.user_connection_ids(user_id)
                    .map(move |connection_id| (connection_id, role))
            })
    }

    pub fn subscribe_to_channel(
        &mut self,
        user_id: UserId,
        channel_id: ChannelId,
        role: ChannelRole,
    ) {
        self.channels.subscribe(user_id, channel_id, role);
    }

    pub fn unsubscribe_from_channel(&mut self, user_id: &UserId, channel_id: &ChannelId) {
        self.channels.unsubscribe(user_id, channel_id);
    }

    pub fn is_user_online(&self, user_id: UserId) -> bool {
        !self
            .connected_users
            .get(&user_id)
            .unwrap_or(&Default::default())
            .connection_ids
            .is_empty()
    }

    #[cfg(test)]
    pub fn check_invariants(&self) {
        for (connection_id, connection) in &self.connections {
            match &connection.principal_id {
                PrincipalId::UserId(user_id) => {
                    assert!(self
                        .connected_users
                        .get(user_id)
                        .unwrap()
                        .connection_ids
                        .contains(connection_id));
                }
                PrincipalId::DevServerId(dev_server_id) => {
                    assert_eq!(
                        self.connected_dev_servers.get(&dev_server_id).unwrap(),
                        connection_id
                    );
                }
            }
        }

        for (user_id, state) in &self.connected_users {
            for connection_id in &state.connection_ids {
                assert_eq!(
                    self.connections.get(connection_id).unwrap().principal_id,
                    PrincipalId::UserId(*user_id)
                );
            }
        }

        for (dev_server_id, connection_id) in &self.connected_dev_servers {
            assert_eq!(
                self.connections.get(connection_id).unwrap().principal_id,
                PrincipalId::DevServerId(*dev_server_id)
            );
        }
    }
}

#[derive(Default, Serialize)]
pub struct ChannelPool {
    by_user: HashMap<UserId, HashMap<ChannelId, ChannelRole>>,
    by_channel: HashMap<ChannelId, HashSet<UserId>>,
}

impl ChannelPool {
    pub fn clear(&mut self) {
        self.by_user.clear();
        self.by_channel.clear();
    }

    pub fn subscribe(&mut self, user_id: UserId, channel_id: ChannelId, role: ChannelRole) {
        self.by_user
            .entry(user_id)
            .or_default()
            .insert(channel_id, role);
        self.by_channel
            .entry(channel_id)
            .or_default()
            .insert(user_id);
    }

    pub fn unsubscribe(&mut self, user_id: &UserId, channel_id: &ChannelId) {
        if let Some(channels) = self.by_user.get_mut(user_id) {
            channels.remove(channel_id);
            if channels.is_empty() {
                self.by_user.remove(user_id);
            }
        }
        if let Some(users) = self.by_channel.get_mut(channel_id) {
            users.remove(user_id);
            if users.is_empty() {
                self.by_channel.remove(channel_id);
            }
        }
    }

    pub fn remove_user(&mut self, user_id: &UserId) {
        if let Some(channels) = self.by_user.remove(&user_id) {
            for channel_id in channels.keys() {
                self.unsubscribe(user_id, &channel_id)
            }
        }
    }

    pub fn users_to_notify(
        &self,
        channel_id: ChannelId,
    ) -> impl '_ + Iterator<Item = (UserId, ChannelRole)> {
        self.by_channel
            .get(&channel_id)
            .into_iter()
            .flat_map(move |users| {
                users.iter().flat_map(move |user_id| {
                    Some((
                        *user_id,
                        self.by_user
                            .get(user_id)
                            .and_then(|channels| channels.get(&channel_id))
                            .copied()?,
                    ))
                })
            })
    }
}
