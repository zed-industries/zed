use crate::db::UserId;
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashSet};
use rpc::ConnectionId;
use serde::Serialize;
use tracing::instrument;
use util::SemanticVersion;

#[derive(Default, Serialize)]
pub struct ConnectionPool {
    connections: BTreeMap<ConnectionId, Connection>,
    connected_users: BTreeMap<UserId, ConnectedUser>,
}

#[derive(Default, Serialize)]
struct ConnectedUser {
    connection_ids: HashSet<ConnectionId>,
}

#[derive(Debug, Serialize)]
pub struct ZedVersion(pub SemanticVersion);
use std::fmt;

impl fmt::Display for ZedVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ZedVersion {
    pub fn is_supported(&self) -> bool {
        self.0 != SemanticVersion::new(0, 123, 0)
    }
    pub fn supports_talker_role(&self) -> bool {
        self.0 >= SemanticVersion::new(0, 125, 0)
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
            .ok_or_else(|| anyhow!("no such connection"))?;

        let user_id = connection.user_id;
        let connected_user = self.connected_users.get_mut(&user_id).unwrap();
        connected_user.connection_ids.remove(&connection_id);
        if connected_user.connection_ids.is_empty() {
            self.connected_users.remove(&user_id);
        }
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
            assert!(self
                .connected_users
                .get(&connection.user_id)
                .unwrap()
                .connection_ids
                .contains(connection_id));
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
