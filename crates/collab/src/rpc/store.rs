use crate::db::{self, UserId};
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashSet};
use rpc::{proto, ConnectionId};
use serde::Serialize;
use tracing::instrument;

#[derive(Default, Serialize)]
pub struct Store {
    connections: BTreeMap<ConnectionId, Connection>,
    connected_users: BTreeMap<UserId, ConnectedUser>,
}

#[derive(Default, Serialize)]
struct ConnectedUser {
    connection_ids: HashSet<ConnectionId>,
}

#[derive(Serialize)]
pub struct Connection {
    pub user_id: UserId,
    pub admin: bool,
}

impl Store {
    #[instrument(skip(self))]
    pub fn add_connection(&mut self, connection_id: ConnectionId, user_id: UserId, admin: bool) {
        self.connections
            .insert(connection_id, Connection { user_id, admin });
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

    pub fn user_connection_ids(&self, user_id: UserId) -> impl Iterator<Item = ConnectionId> + '_ {
        self.connected_users
            .get(&user_id)
            .into_iter()
            .map(|state| &state.connection_ids)
            .flatten()
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

    pub fn build_initial_contacts_update(
        &self,
        contacts: Vec<db::Contact>,
    ) -> proto::UpdateContacts {
        let mut update = proto::UpdateContacts::default();

        for contact in contacts {
            match contact {
                db::Contact::Accepted {
                    user_id,
                    should_notify,
                    busy,
                } => {
                    update
                        .contacts
                        .push(self.contact_for_user(user_id, should_notify, busy));
                }
                db::Contact::Outgoing { user_id } => {
                    update.outgoing_requests.push(user_id.to_proto())
                }
                db::Contact::Incoming {
                    user_id,
                    should_notify,
                } => update
                    .incoming_requests
                    .push(proto::IncomingContactRequest {
                        requester_id: user_id.to_proto(),
                        should_notify,
                    }),
            }
        }

        update
    }

    pub fn contact_for_user(
        &self,
        user_id: UserId,
        should_notify: bool,
        busy: bool,
    ) -> proto::Contact {
        proto::Contact {
            user_id: user_id.to_proto(),
            online: self.is_user_online(user_id),
            busy,
            should_notify,
        }
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
