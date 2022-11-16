use crate::db::{self, ProjectId, UserId};
use anyhow::{anyhow, Result};
use collections::{btree_map, BTreeMap, BTreeSet, HashMap, HashSet};
use rpc::{proto, ConnectionId};
use serde::Serialize;
use std::path::PathBuf;
use tracing::instrument;

pub type RoomId = u64;

#[derive(Default, Serialize)]
pub struct Store {
    connections: BTreeMap<ConnectionId, ConnectionState>,
    connected_users: BTreeMap<UserId, ConnectedUser>,
    next_room_id: RoomId,
    rooms: BTreeMap<RoomId, proto::Room>,
    projects: BTreeMap<ProjectId, Project>,
}

#[derive(Default, Serialize)]
struct ConnectedUser {
    connection_ids: HashSet<ConnectionId>,
    active_call: Option<Call>,
}

#[derive(Serialize)]
struct ConnectionState {
    user_id: UserId,
    admin: bool,
    projects: BTreeSet<ProjectId>,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize)]
pub struct Call {
    pub calling_user_id: UserId,
    pub room_id: RoomId,
    pub connection_id: Option<ConnectionId>,
    pub initial_project_id: Option<ProjectId>,
}

#[derive(Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub room_id: RoomId,
    pub host_connection_id: ConnectionId,
    pub host: Collaborator,
    pub guests: HashMap<ConnectionId, Collaborator>,
    pub active_replica_ids: HashSet<ReplicaId>,
    pub worktrees: BTreeMap<u64, Worktree>,
    pub language_servers: Vec<proto::LanguageServer>,
}

#[derive(Serialize)]
pub struct Collaborator {
    pub replica_id: ReplicaId,
    pub user_id: UserId,
    pub admin: bool,
}

#[derive(Default, Serialize)]
pub struct Worktree {
    pub abs_path: PathBuf,
    pub root_name: String,
    pub visible: bool,
    #[serde(skip)]
    pub entries: BTreeMap<u64, proto::Entry>,
    #[serde(skip)]
    pub diagnostic_summaries: BTreeMap<PathBuf, proto::DiagnosticSummary>,
    pub scan_id: u64,
    pub is_complete: bool,
}

pub type ReplicaId = u16;

pub struct LeftProject {
    pub id: ProjectId,
    pub host_user_id: UserId,
    pub host_connection_id: ConnectionId,
    pub connection_ids: Vec<ConnectionId>,
    pub remove_collaborator: bool,
}

#[derive(Copy, Clone)]
pub struct Metrics {
    pub connections: usize,
    pub shared_projects: usize,
}

impl Store {
    pub fn metrics(&self) -> Metrics {
        let connections = self.connections.values().filter(|c| !c.admin).count();
        let mut shared_projects = 0;
        for project in self.projects.values() {
            if let Some(connection) = self.connections.get(&project.host_connection_id) {
                if !connection.admin {
                    shared_projects += 1;
                }
            }
        }

        Metrics {
            connections,
            shared_projects,
        }
    }

    #[instrument(skip(self))]
    pub fn add_connection(&mut self, connection_id: ConnectionId, user_id: UserId, admin: bool) {
        self.connections.insert(
            connection_id,
            ConnectionState {
                user_id,
                admin,
                projects: Default::default(),
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

    pub fn connection_ids_for_user(
        &self,
        user_id: UserId,
    ) -> impl Iterator<Item = ConnectionId> + '_ {
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

    pub fn rooms(&self) -> &BTreeMap<RoomId, proto::Room> {
        &self.rooms
    }

    pub fn unshare_project(
        &mut self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<(&proto::Room, Project)> {
        match self.projects.entry(project_id) {
            btree_map::Entry::Occupied(e) => {
                if e.get().host_connection_id == connection_id {
                    let project = e.remove();

                    if let Some(host_connection) = self.connections.get_mut(&connection_id) {
                        host_connection.projects.remove(&project_id);
                    }

                    for guest_connection in project.guests.keys() {
                        if let Some(connection) = self.connections.get_mut(guest_connection) {
                            connection.projects.remove(&project_id);
                        }
                    }

                    let room = self
                        .rooms
                        .get_mut(&project.room_id)
                        .ok_or_else(|| anyhow!("no such room"))?;
                    let participant = room
                        .participants
                        .iter_mut()
                        .find(|participant| participant.peer_id == connection_id.0)
                        .ok_or_else(|| anyhow!("no such room"))?;
                    participant
                        .projects
                        .retain(|project| project.id != project_id.to_proto());

                    Ok((room, project))
                } else {
                    Err(anyhow!("no such project"))?
                }
            }
            btree_map::Entry::Vacant(_) => Err(anyhow!("no such project"))?,
        }
    }

    pub fn leave_project(
        &mut self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<LeftProject> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;

        // If the connection leaving the project is a collaborator, remove it.
        let remove_collaborator = if let Some(guest) = project.guests.remove(&connection_id) {
            project.active_replica_ids.remove(&guest.replica_id);
            true
        } else {
            false
        };

        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.projects.remove(&project_id);
        }

        Ok(LeftProject {
            id: project.id,
            host_connection_id: project.host_connection_id,
            host_user_id: project.host.user_id,
            connection_ids: project.connection_ids(),
            remove_collaborator,
        })
    }

    #[cfg(test)]
    pub fn check_invariants(&self) {
        for (connection_id, connection) in &self.connections {
            for project_id in &connection.projects {
                let project = &self.projects.get(project_id).unwrap();
                if project.host_connection_id != *connection_id {
                    assert!(project.guests.contains_key(connection_id));
                }

                for (worktree_id, worktree) in project.worktrees.iter() {
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

            if let Some(active_call) = state.active_call.as_ref() {
                if let Some(active_call_connection_id) = active_call.connection_id {
                    assert!(
                        state.connection_ids.contains(&active_call_connection_id),
                        "call is active on a dead connection"
                    );
                    assert!(
                        state.connection_ids.contains(&active_call_connection_id),
                        "call is active on a dead connection"
                    );
                }
            }
        }

        for (room_id, room) in &self.rooms {
            // for pending_user_id in &room.pending_participant_user_ids {
            //     assert!(
            //         self.connected_users
            //             .contains_key(&UserId::from_proto(*pending_user_id)),
            //         "call is active on a user that has disconnected"
            //     );
            // }

            for participant in &room.participants {
                assert!(
                    self.connections
                        .contains_key(&ConnectionId(participant.peer_id)),
                    "room {} contains participant {:?} that has disconnected",
                    room_id,
                    participant
                );

                for participant_project in &participant.projects {
                    let project = &self.projects[&ProjectId::from_proto(participant_project.id)];
                    assert_eq!(
                        project.room_id, *room_id,
                        "project was shared on a different room"
                    );
                }
            }

            // assert!(
            //     !room.pending_participant_user_ids.is_empty() || !room.participants.is_empty(),
            //     "room can't be empty"
            // );
        }

        for (project_id, project) in &self.projects {
            let host_connection = self.connections.get(&project.host_connection_id).unwrap();
            assert!(host_connection.projects.contains(project_id));

            for guest_connection_id in project.guests.keys() {
                let guest_connection = self.connections.get(guest_connection_id).unwrap();
                assert!(guest_connection.projects.contains(project_id));
            }
            assert_eq!(project.active_replica_ids.len(), project.guests.len());
            assert_eq!(
                project.active_replica_ids,
                project
                    .guests
                    .values()
                    .map(|guest| guest.replica_id)
                    .collect::<HashSet<_>>(),
            );

            let room = &self.rooms[&project.room_id];
            let room_participant = room
                .participants
                .iter()
                .find(|participant| participant.peer_id == project.host_connection_id.0)
                .unwrap();
            assert!(
                room_participant
                    .projects
                    .iter()
                    .any(|project| project.id == project_id.to_proto()),
                "project was not shared in room"
            );
        }
    }
}

impl Project {
    pub fn guest_connection_ids(&self) -> Vec<ConnectionId> {
        self.guests.keys().copied().collect()
    }

    pub fn connection_ids(&self) -> Vec<ConnectionId> {
        self.guests
            .keys()
            .copied()
            .chain(Some(self.host_connection_id))
            .collect()
    }
}
