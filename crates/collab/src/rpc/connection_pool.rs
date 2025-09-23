use crate::db::{ChannelId, ChannelRole, UserId};
use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, HashSet};
use rpc::ConnectionId;
use semantic_version::SemanticVersion;
use serde::Serialize;
use std::fmt;
use tracing::instrument;

#[derive(Default, Serialize)]
pub struct ConnectionPool {
    connections: BTreeMap<ConnectionId, Connection>,
    connected_users: BTreeMap<UserId, ConnectedPrincipal>,
    channels: ChannelPool,
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
        // v0.198.4 is the first version where we no longer connect to Collab automatically.
        // We reject any clients older than that to prevent them from connecting to Collab just for authentication.
        if self.0 < SemanticVersion::new(0, 198, 4) {
            return false;
        }

        // Since we hotfixed the changes to no longer connect to Collab automatically to Preview, we also need to reject
        // versions in the range [v0.199.0, v0.199.1].
        if self.0 >= SemanticVersion::new(0, 199, 0) && self.0 < SemanticVersion::new(0, 199, 2) {
            return false;
        }

        true
    }
}

#[derive(Serialize)]
pub struct Connection {
    pub user_id: UserId,
    pub admin: bool,
    pub zed_version: ZedVersion,
}

impl ConnectionPool {
    pub fn reset(&mut self) {
        self.connections.clear();
        self.connected_users.clear();
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
                user_id,
                admin,
                zed_version,
            },
        );
        let connected_user = self.connected_users.entry(user_id).or_default();
        connected_user.connection_ids.insert(connection_id);
    }

    #[instrument(skip(self))]
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> Result<()> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .context("no such connection")?;

        let user_id = connection.user_id;

        let connected_user = self.connected_users.get_mut(&user_id).unwrap();
        connected_user.connection_ids.remove(&connection_id);
        if connected_user.connection_ids.is_empty() {
            self.connected_users.remove(&user_id);
            self.channels.remove_user(&user_id);
        };
        self.connections.remove(&connection_id).unwrap();
        Ok(())
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
            assert!(
                self.connected_users
                    .get(&connection.user_id)
                    .unwrap()
                    .connection_ids
                    .contains(connection_id)
            );
        }

        for (user_id, state) in &self.connected_users {
            for connection_id in &state.connection_ids {
                assert_eq!(
                    self.connections.get(connection_id).unwrap().user_id,
                    *user_id
                );
            }
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
        if let Some(channels) = self.by_user.remove(user_id) {
            for channel_id in channels.keys() {
                self.unsubscribe(user_id, channel_id)
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
