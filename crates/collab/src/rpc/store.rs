use crate::db::{self, ProjectId, UserId};
use anyhow::{anyhow, Result};
use collections::{btree_map, BTreeMap, BTreeSet, HashMap, HashSet};
use rpc::{proto, ConnectionId};
use serde::Serialize;
use std::{borrow::Cow, mem, path::PathBuf, str};
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

#[derive(Default)]
pub struct RemovedConnectionState<'a> {
    pub user_id: UserId,
    pub hosted_projects: Vec<Project>,
    pub guest_projects: Vec<LeftProject>,
    pub contact_ids: HashSet<UserId>,
    pub room: Option<Cow<'a, proto::Room>>,
    pub canceled_call_connection_ids: Vec<ConnectionId>,
}

pub struct LeftProject {
    pub id: ProjectId,
    pub host_user_id: UserId,
    pub host_connection_id: ConnectionId,
    pub connection_ids: Vec<ConnectionId>,
    pub remove_collaborator: bool,
}

pub struct LeftRoom<'a> {
    pub room: Cow<'a, proto::Room>,
    pub unshared_projects: Vec<Project>,
    pub left_projects: Vec<LeftProject>,
    pub canceled_call_connection_ids: Vec<ConnectionId>,
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
    pub fn remove_connection(
        &mut self,
        connection_id: ConnectionId,
    ) -> Result<RemovedConnectionState> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;

        let user_id = connection.user_id;

        let mut result = RemovedConnectionState {
            user_id,
            ..Default::default()
        };

        let connected_user = self.connected_users.get(&user_id).unwrap();
        if let Some(active_call) = connected_user.active_call.as_ref() {
            let room_id = active_call.room_id;
            if active_call.connection_id == Some(connection_id) {
                let left_room = self.leave_room(room_id, connection_id)?;
                result.hosted_projects = left_room.unshared_projects;
                result.guest_projects = left_room.left_projects;
                result.room = Some(Cow::Owned(left_room.room.into_owned()));
                result.canceled_call_connection_ids = left_room.canceled_call_connection_ids;
            } else if connected_user.connection_ids.len() == 1 {
                let (room, _) = self.decline_call(room_id, connection_id)?;
                result.room = Some(Cow::Owned(room.clone()));
            }
        }

        let connected_user = self.connected_users.get_mut(&user_id).unwrap();
        connected_user.connection_ids.remove(&connection_id);
        if connected_user.connection_ids.is_empty() {
            self.connected_users.remove(&user_id);
        }
        self.connections.remove(&connection_id).unwrap();

        Ok(result)
    }

    pub fn user_id_for_connection(&self, connection_id: ConnectionId) -> Result<UserId> {
        Ok(self
            .connections
            .get(&connection_id)
            .ok_or_else(|| anyhow!("unknown connection"))?
            .user_id)
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

    fn is_user_busy(&self, user_id: UserId) -> bool {
        self.connected_users
            .get(&user_id)
            .unwrap_or(&Default::default())
            .active_call
            .is_some()
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
                } => {
                    update
                        .contacts
                        .push(self.contact_for_user(user_id, should_notify));
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

    pub fn contact_for_user(&self, user_id: UserId, should_notify: bool) -> proto::Contact {
        proto::Contact {
            user_id: user_id.to_proto(),
            online: self.is_user_online(user_id),
            busy: self.is_user_busy(user_id),
            should_notify,
        }
    }

    pub fn join_room(
        &mut self,
        room_id: RoomId,
        connection_id: ConnectionId,
    ) -> Result<(&proto::Room, Vec<ConnectionId>)> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let user_id = connection.user_id;
        let recipient_connection_ids = self.connection_ids_for_user(user_id).collect::<Vec<_>>();

        let connected_user = self
            .connected_users
            .get_mut(&user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let active_call = connected_user
            .active_call
            .as_mut()
            .ok_or_else(|| anyhow!("not being called"))?;
        anyhow::ensure!(
            active_call.room_id == room_id && active_call.connection_id.is_none(),
            "not being called on this room"
        );

        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        anyhow::ensure!(
            room.pending_participant_user_ids
                .contains(&user_id.to_proto()),
            anyhow!("no such room")
        );
        room.pending_participant_user_ids
            .retain(|pending| *pending != user_id.to_proto());
        room.participants.push(proto::Participant {
            user_id: user_id.to_proto(),
            peer_id: connection_id.0,
            projects: Default::default(),
            location: Some(proto::ParticipantLocation {
                variant: Some(proto::participant_location::Variant::External(
                    proto::participant_location::External {},
                )),
            }),
        });
        active_call.connection_id = Some(connection_id);

        Ok((room, recipient_connection_ids))
    }

    pub fn leave_room(&mut self, room_id: RoomId, connection_id: ConnectionId) -> Result<LeftRoom> {
        let connection = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let user_id = connection.user_id;

        let connected_user = self
            .connected_users
            .get(&user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        anyhow::ensure!(
            connected_user
                .active_call
                .map_or(false, |call| call.room_id == room_id
                    && call.connection_id == Some(connection_id)),
            "cannot leave a room before joining it"
        );

        // Given that users can only join one room at a time, we can safely unshare
        // and leave all projects associated with the connection.
        let mut unshared_projects = Vec::new();
        let mut left_projects = Vec::new();
        for project_id in connection.projects.clone() {
            if let Ok((_, project)) = self.unshare_project(project_id, connection_id) {
                unshared_projects.push(project);
            } else if let Ok(project) = self.leave_project(project_id, connection_id) {
                left_projects.push(project);
            }
        }
        self.connected_users.get_mut(&user_id).unwrap().active_call = None;

        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        room.participants
            .retain(|participant| participant.peer_id != connection_id.0);

        let mut canceled_call_connection_ids = Vec::new();
        room.pending_participant_user_ids
            .retain(|pending_participant_user_id| {
                if let Some(connected_user) = self
                    .connected_users
                    .get_mut(&UserId::from_proto(*pending_participant_user_id))
                {
                    if let Some(call) = connected_user.active_call.as_ref() {
                        if call.calling_user_id == user_id {
                            connected_user.active_call.take();
                            canceled_call_connection_ids
                                .extend(connected_user.connection_ids.iter().copied());
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } else {
                    true
                }
            });

        let room = if room.participants.is_empty() {
            Cow::Owned(self.rooms.remove(&room_id).unwrap())
        } else {
            Cow::Borrowed(self.rooms.get(&room_id).unwrap())
        };

        Ok(LeftRoom {
            room,
            unshared_projects,
            left_projects,
            canceled_call_connection_ids,
        })
    }

    pub fn rooms(&self) -> &BTreeMap<RoomId, proto::Room> {
        &self.rooms
    }

    pub fn cancel_call(
        &mut self,
        room_id: RoomId,
        called_user_id: UserId,
        canceller_connection_id: ConnectionId,
    ) -> Result<(&proto::Room, HashSet<ConnectionId>)> {
        let canceller_user_id = self.user_id_for_connection(canceller_connection_id)?;
        let canceller = self
            .connected_users
            .get(&canceller_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let recipient = self
            .connected_users
            .get(&called_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let canceller_active_call = canceller
            .active_call
            .as_ref()
            .ok_or_else(|| anyhow!("no active call"))?;
        let recipient_active_call = recipient
            .active_call
            .as_ref()
            .ok_or_else(|| anyhow!("no active call for recipient"))?;

        anyhow::ensure!(
            canceller_active_call.room_id == room_id,
            "users are on different calls"
        );
        anyhow::ensure!(
            recipient_active_call.room_id == room_id,
            "users are on different calls"
        );
        anyhow::ensure!(
            recipient_active_call.connection_id.is_none(),
            "recipient has already answered"
        );
        let room_id = recipient_active_call.room_id;
        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        room.pending_participant_user_ids
            .retain(|user_id| UserId::from_proto(*user_id) != called_user_id);

        let recipient = self.connected_users.get_mut(&called_user_id).unwrap();
        recipient.active_call.take();

        Ok((room, recipient.connection_ids.clone()))
    }

    pub fn decline_call(
        &mut self,
        room_id: RoomId,
        recipient_connection_id: ConnectionId,
    ) -> Result<(&proto::Room, Vec<ConnectionId>)> {
        let called_user_id = self.user_id_for_connection(recipient_connection_id)?;
        let recipient = self
            .connected_users
            .get_mut(&called_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        if let Some(active_call) = recipient.active_call {
            anyhow::ensure!(active_call.room_id == room_id, "no such room");
            anyhow::ensure!(
                active_call.connection_id.is_none(),
                "cannot decline a call after joining room"
            );
            recipient.active_call.take();
            let recipient_connection_ids = self
                .connection_ids_for_user(called_user_id)
                .collect::<Vec<_>>();
            let room = self
                .rooms
                .get_mut(&active_call.room_id)
                .ok_or_else(|| anyhow!("no such room"))?;
            room.pending_participant_user_ids
                .retain(|user_id| UserId::from_proto(*user_id) != called_user_id);
            Ok((room, recipient_connection_ids))
        } else {
            Err(anyhow!("user is not being called"))
        }
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

    pub fn update_project(
        &mut self,
        project_id: ProjectId,
        worktrees: &[proto::WorktreeMetadata],
        connection_id: ConnectionId,
    ) -> Result<&proto::Room> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id {
            let mut old_worktrees = mem::take(&mut project.worktrees);
            for worktree in worktrees {
                if let Some(old_worktree) = old_worktrees.remove(&worktree.id) {
                    project.worktrees.insert(worktree.id, old_worktree);
                } else {
                    project.worktrees.insert(
                        worktree.id,
                        Worktree {
                            root_name: worktree.root_name.clone(),
                            visible: worktree.visible,
                            ..Default::default()
                        },
                    );
                }
            }

            let room = self
                .rooms
                .get_mut(&project.room_id)
                .ok_or_else(|| anyhow!("no such room"))?;
            let participant_project = room
                .participants
                .iter_mut()
                .flat_map(|participant| &mut participant.projects)
                .find(|project| project.id == project_id.to_proto())
                .ok_or_else(|| anyhow!("no such project"))?;
            participant_project.worktree_root_names = worktrees
                .iter()
                .filter(|worktree| worktree.visible)
                .map(|worktree| worktree.root_name.clone())
                .collect();

            Ok(room)
        } else {
            Err(anyhow!("no such project"))?
        }
    }

    pub fn update_diagnostic_summary(
        &mut self,
        project_id: ProjectId,
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
        project_id: ProjectId,
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
        requester_connection_id: ConnectionId,
        project_id: ProjectId,
    ) -> Result<(&Project, ReplicaId)> {
        let connection = self
            .connections
            .get_mut(&requester_connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let user = self
            .connected_users
            .get(&connection.user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let active_call = user.active_call.ok_or_else(|| anyhow!("no such project"))?;
        anyhow::ensure!(
            active_call.connection_id == Some(requester_connection_id),
            "no such project"
        );

        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        anyhow::ensure!(project.room_id == active_call.room_id, "no such project");

        connection.projects.insert(project_id);
        let mut replica_id = 1;
        while project.active_replica_ids.contains(&replica_id) {
            replica_id += 1;
        }
        project.active_replica_ids.insert(replica_id);
        project.guests.insert(
            requester_connection_id,
            Collaborator {
                replica_id,
                user_id: connection.user_id,
                admin: connection.admin,
            },
        );

        Ok((project, replica_id))
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

    #[allow(clippy::too_many_arguments)]
    pub fn update_worktree(
        &mut self,
        connection_id: ConnectionId,
        project_id: ProjectId,
        worktree_id: u64,
        worktree_root_name: &str,
        removed_entries: &[u64],
        updated_entries: &[proto::Entry],
        scan_id: u64,
        is_last_update: bool,
    ) -> Result<Vec<ConnectionId>> {
        let project = self.write_project(project_id, connection_id)?;

        let connection_ids = project.connection_ids();
        let mut worktree = project.worktrees.entry(worktree_id).or_default();
        worktree.root_name = worktree_root_name.to_string();

        for entry_id in removed_entries {
            worktree.entries.remove(entry_id);
        }

        for entry in updated_entries {
            worktree.entries.insert(entry.id, entry.clone());
        }

        worktree.scan_id = scan_id;
        worktree.is_complete = is_last_update;
        Ok(connection_ids)
    }

    pub fn project_connection_ids(
        &self,
        project_id: ProjectId,
        acting_connection_id: ConnectionId,
    ) -> Result<Vec<ConnectionId>> {
        Ok(self
            .read_project(project_id, acting_connection_id)?
            .connection_ids())
    }

    pub fn project(&self, project_id: ProjectId) -> Result<&Project> {
        self.projects
            .get(&project_id)
            .ok_or_else(|| anyhow!("no such project"))
    }

    pub fn read_project(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<&Project> {
        let project = self
            .projects
            .get(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id
            || project.guests.contains_key(&connection_id)
        {
            Ok(project)
        } else {
            Err(anyhow!("no such project"))?
        }
    }

    fn write_project(
        &mut self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<&mut Project> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        if project.host_connection_id == connection_id
            || project.guests.contains_key(&connection_id)
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
            for pending_user_id in &room.pending_participant_user_ids {
                assert!(
                    self.connected_users
                        .contains_key(&UserId::from_proto(*pending_user_id)),
                    "call is active on a user that has disconnected"
                );
            }

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

            assert!(
                !room.pending_participant_user_ids.is_empty() || !room.participants.is_empty(),
                "room can't be empty"
            );
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
