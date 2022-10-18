use crate::db::{self, ChannelId, ProjectId, UserId};
use anyhow::{anyhow, Result};
use collections::{btree_map, BTreeMap, BTreeSet, HashMap, HashSet};
use nanoid::nanoid;
use rpc::{proto, ConnectionId};
use serde::Serialize;
use std::{borrow::Cow, mem, path::PathBuf, str, time::Duration};
use time::OffsetDateTime;
use tracing::instrument;
use util::post_inc;

pub type RoomId = u64;

#[derive(Default, Serialize)]
pub struct Store {
    connections: BTreeMap<ConnectionId, ConnectionState>,
    connected_users: BTreeMap<UserId, ConnectedUser>,
    next_room_id: RoomId,
    rooms: BTreeMap<RoomId, proto::Room>,
    projects: BTreeMap<ProjectId, Project>,
    #[serde(skip)]
    channels: BTreeMap<ChannelId, Channel>,
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
    channels: HashSet<ChannelId>,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize)]
pub struct Call {
    pub caller_user_id: UserId,
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
    #[serde(skip)]
    pub last_activity: Option<OffsetDateTime>,
    pub admin: bool,
}

#[derive(Default, Serialize)]
pub struct Worktree {
    pub root_name: String,
    pub visible: bool,
    #[serde(skip)]
    pub entries: BTreeMap<u64, proto::Entry>,
    #[serde(skip)]
    pub diagnostic_summaries: BTreeMap<PathBuf, proto::DiagnosticSummary>,
    pub scan_id: u64,
    pub is_complete: bool,
}

#[derive(Default)]
pub struct Channel {
    pub connection_ids: HashSet<ConnectionId>,
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
    pub registered_projects: usize,
    pub active_projects: usize,
    pub shared_projects: usize,
}

impl Store {
    pub fn metrics(&self) -> Metrics {
        const ACTIVE_PROJECT_TIMEOUT: Duration = Duration::from_secs(60);
        let active_window_start = OffsetDateTime::now_utc() - ACTIVE_PROJECT_TIMEOUT;

        let connections = self.connections.values().filter(|c| !c.admin).count();
        let mut registered_projects = 0;
        let mut active_projects = 0;
        let mut shared_projects = 0;
        for project in self.projects.values() {
            if let Some(connection) = self.connections.get(&project.host_connection_id) {
                if !connection.admin {
                    registered_projects += 1;
                    if project.is_active_since(active_window_start) {
                        active_projects += 1;
                        if !project.guests.is_empty() {
                            shared_projects += 1;
                        }
                    }
                }
            }
        }

        Metrics {
            connections,
            registered_projects,
            active_projects,
            shared_projects,
        }
    }

    #[instrument(skip(self))]
    pub fn add_connection(
        &mut self,
        connection_id: ConnectionId,
        user_id: UserId,
        admin: bool,
    ) -> Option<proto::IncomingCall> {
        self.connections.insert(
            connection_id,
            ConnectionState {
                user_id,
                admin,
                projects: Default::default(),
                channels: Default::default(),
            },
        );
        let connected_user = self.connected_users.entry(user_id).or_default();
        connected_user.connection_ids.insert(connection_id);
        if let Some(active_call) = connected_user.active_call {
            if active_call.connection_id.is_some() {
                None
            } else {
                let room = self.room(active_call.room_id)?;
                Some(proto::IncomingCall {
                    room_id: active_call.room_id,
                    caller_user_id: active_call.caller_user_id.to_proto(),
                    participant_user_ids: room
                        .participants
                        .iter()
                        .map(|participant| participant.user_id)
                        .collect(),
                    initial_project: active_call
                        .initial_project_id
                        .and_then(|id| Self::build_participant_project(id, &self.projects)),
                })
            }
        } else {
            None
        }
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
        let connection_channels = mem::take(&mut connection.channels);

        let mut result = RemovedConnectionState {
            user_id,
            ..Default::default()
        };

        // Leave all channels.
        for channel_id in connection_channels {
            self.leave_channel(connection_id, channel_id);
        }

        let connected_user = self.connected_users.get(&user_id).unwrap();
        if let Some(active_call) = connected_user.active_call.as_ref() {
            let room_id = active_call.room_id;
            let left_room = self.leave_room(room_id, connection_id)?;
            result.hosted_projects = left_room.unshared_projects;
            result.guest_projects = left_room.left_projects;
            result.room = Some(Cow::Owned(left_room.room.into_owned()));
            result.canceled_call_connection_ids = left_room.canceled_call_connection_ids;
        }

        let connected_user = self.connected_users.get_mut(&user_id).unwrap();
        connected_user.connection_ids.remove(&connection_id);
        if connected_user.connection_ids.is_empty() {
            self.connected_users.remove(&user_id);
        }
        self.connections.remove(&connection_id).unwrap();

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
            if let btree_map::Entry::Occupied(mut entry) = self.channels.entry(channel_id) {
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

    pub fn create_room(&mut self, creator_connection_id: ConnectionId) -> Result<&proto::Room> {
        let connection = self
            .connections
            .get_mut(&creator_connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let connected_user = self
            .connected_users
            .get_mut(&connection.user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        anyhow::ensure!(
            connected_user.active_call.is_none(),
            "can't create a room with an active call"
        );

        let room_id = post_inc(&mut self.next_room_id);
        let room = proto::Room {
            id: room_id,
            participants: vec![proto::Participant {
                user_id: connection.user_id.to_proto(),
                peer_id: creator_connection_id.0,
                projects: Default::default(),
                location: Some(proto::ParticipantLocation {
                    variant: Some(proto::participant_location::Variant::External(
                        proto::participant_location::External {},
                    )),
                }),
            }],
            pending_participant_user_ids: Default::default(),
            live_kit_room: nanoid!(30),
        };

        self.rooms.insert(room_id, room);
        connected_user.active_call = Some(Call {
            caller_user_id: connection.user_id,
            room_id,
            connection_id: Some(creator_connection_id),
            initial_project_id: None,
        });
        Ok(self.rooms.get(&room_id).unwrap())
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
                        if call.caller_user_id == user_id {
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

    pub fn room(&self, room_id: RoomId) -> Option<&proto::Room> {
        self.rooms.get(&room_id)
    }

    pub fn call(
        &mut self,
        room_id: RoomId,
        recipient_user_id: UserId,
        initial_project_id: Option<ProjectId>,
        from_connection_id: ConnectionId,
    ) -> Result<(&proto::Room, Vec<ConnectionId>, proto::IncomingCall)> {
        let caller_user_id = self.user_id_for_connection(from_connection_id)?;

        let recipient_connection_ids = self
            .connection_ids_for_user(recipient_user_id)
            .collect::<Vec<_>>();
        let mut recipient = self
            .connected_users
            .get_mut(&recipient_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        anyhow::ensure!(
            recipient.active_call.is_none(),
            "recipient is already on another call"
        );

        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        anyhow::ensure!(
            room.participants
                .iter()
                .any(|participant| participant.peer_id == from_connection_id.0),
            "no such room"
        );
        anyhow::ensure!(
            room.pending_participant_user_ids
                .iter()
                .all(|user_id| UserId::from_proto(*user_id) != recipient_user_id),
            "cannot call the same user more than once"
        );
        room.pending_participant_user_ids
            .push(recipient_user_id.to_proto());

        if let Some(initial_project_id) = initial_project_id {
            let project = self
                .projects
                .get(&initial_project_id)
                .ok_or_else(|| anyhow!("no such project"))?;
            anyhow::ensure!(project.room_id == room_id, "no such project");
        }

        recipient.active_call = Some(Call {
            caller_user_id,
            room_id,
            connection_id: None,
            initial_project_id,
        });

        Ok((
            room,
            recipient_connection_ids,
            proto::IncomingCall {
                room_id,
                caller_user_id: caller_user_id.to_proto(),
                participant_user_ids: room
                    .participants
                    .iter()
                    .map(|participant| participant.user_id)
                    .collect(),
                initial_project: initial_project_id
                    .and_then(|id| Self::build_participant_project(id, &self.projects)),
            },
        ))
    }

    pub fn call_failed(&mut self, room_id: RoomId, to_user_id: UserId) -> Result<&proto::Room> {
        let mut recipient = self
            .connected_users
            .get_mut(&to_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        anyhow::ensure!(recipient
            .active_call
            .map_or(false, |call| call.room_id == room_id
                && call.connection_id.is_none()));
        recipient.active_call = None;
        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        room.pending_participant_user_ids
            .retain(|user_id| UserId::from_proto(*user_id) != to_user_id);
        Ok(room)
    }

    pub fn cancel_call(
        &mut self,
        room_id: RoomId,
        recipient_user_id: UserId,
        canceller_connection_id: ConnectionId,
    ) -> Result<(&proto::Room, HashSet<ConnectionId>)> {
        let canceller_user_id = self.user_id_for_connection(canceller_connection_id)?;
        let canceller = self
            .connected_users
            .get(&canceller_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        let recipient = self
            .connected_users
            .get(&recipient_user_id)
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
            .retain(|user_id| UserId::from_proto(*user_id) != recipient_user_id);

        let recipient = self.connected_users.get_mut(&recipient_user_id).unwrap();
        recipient.active_call.take();

        Ok((room, recipient.connection_ids.clone()))
    }

    pub fn decline_call(
        &mut self,
        room_id: RoomId,
        recipient_connection_id: ConnectionId,
    ) -> Result<(&proto::Room, Vec<ConnectionId>)> {
        let recipient_user_id = self.user_id_for_connection(recipient_connection_id)?;
        let recipient = self
            .connected_users
            .get_mut(&recipient_user_id)
            .ok_or_else(|| anyhow!("no such connection"))?;
        if let Some(active_call) = recipient.active_call.take() {
            anyhow::ensure!(active_call.room_id == room_id, "no such room");
            let recipient_connection_ids = self
                .connection_ids_for_user(recipient_user_id)
                .collect::<Vec<_>>();
            let room = self
                .rooms
                .get_mut(&active_call.room_id)
                .ok_or_else(|| anyhow!("no such room"))?;
            room.pending_participant_user_ids
                .retain(|user_id| UserId::from_proto(*user_id) != recipient_user_id);
            Ok((room, recipient_connection_ids))
        } else {
            Err(anyhow!("user is not being called"))
        }
    }

    pub fn update_participant_location(
        &mut self,
        room_id: RoomId,
        location: proto::ParticipantLocation,
        connection_id: ConnectionId,
    ) -> Result<&proto::Room> {
        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        if let Some(proto::participant_location::Variant::SharedProject(project)) =
            location.variant.as_ref()
        {
            anyhow::ensure!(
                room.participants
                    .iter()
                    .flat_map(|participant| &participant.projects)
                    .any(|participant_project| participant_project.id == project.id),
                "no such project"
            );
        }

        let participant = room
            .participants
            .iter_mut()
            .find(|participant| participant.peer_id == connection_id.0)
            .ok_or_else(|| anyhow!("no such room"))?;
        participant.location = Some(location);

        Ok(room)
    }

    pub fn share_project(
        &mut self,
        room_id: RoomId,
        project_id: ProjectId,
        worktrees: Vec<proto::WorktreeMetadata>,
        host_connection_id: ConnectionId,
    ) -> Result<&proto::Room> {
        let connection = self
            .connections
            .get_mut(&host_connection_id)
            .ok_or_else(|| anyhow!("no such connection"))?;

        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| anyhow!("no such room"))?;
        let participant = room
            .participants
            .iter_mut()
            .find(|participant| participant.peer_id == host_connection_id.0)
            .ok_or_else(|| anyhow!("no such room"))?;

        connection.projects.insert(project_id);
        self.projects.insert(
            project_id,
            Project {
                id: project_id,
                room_id,
                host_connection_id,
                host: Collaborator {
                    user_id: connection.user_id,
                    replica_id: 0,
                    last_activity: None,
                    admin: connection.admin,
                },
                guests: Default::default(),
                active_replica_ids: Default::default(),
                worktrees: worktrees
                    .into_iter()
                    .map(|worktree| {
                        (
                            worktree.id,
                            Worktree {
                                root_name: worktree.root_name,
                                visible: worktree.visible,
                                ..Default::default()
                            },
                        )
                    })
                    .collect(),
                language_servers: Default::default(),
            },
        );

        participant
            .projects
            .extend(Self::build_participant_project(project_id, &self.projects));

        Ok(room)
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
                last_activity: Some(OffsetDateTime::now_utc()),
                admin: connection.admin,
            },
        );

        project.host.last_activity = Some(OffsetDateTime::now_utc());
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

    fn build_participant_project(
        project_id: ProjectId,
        projects: &BTreeMap<ProjectId, Project>,
    ) -> Option<proto::ParticipantProject> {
        Some(proto::ParticipantProject {
            id: project_id.to_proto(),
            worktree_root_names: projects
                .get(&project_id)?
                .worktrees
                .values()
                .filter(|worktree| worktree.visible)
                .map(|worktree| worktree.root_name.clone())
                .collect(),
        })
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

    pub fn channel_connection_ids(&self, channel_id: ChannelId) -> Result<Vec<ConnectionId>> {
        Ok(self
            .channels
            .get(&channel_id)
            .ok_or_else(|| anyhow!("no such channel"))?
            .connection_ids())
    }

    pub fn project(&self, project_id: ProjectId) -> Result<&Project> {
        self.projects
            .get(&project_id)
            .ok_or_else(|| anyhow!("no such project"))
    }

    pub fn register_project_activity(
        &mut self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<()> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("no such project"))?;
        let collaborator = if connection_id == project.host_connection_id {
            &mut project.host
        } else if let Some(guest) = project.guests.get_mut(&connection_id) {
            guest
        } else {
            return Err(anyhow!("no such project"))?;
        };
        collaborator.last_activity = Some(OffsetDateTime::now_utc());
        Ok(())
    }

    pub fn projects(&self) -> impl Iterator<Item = (&ProjectId, &Project)> {
        self.projects.iter()
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
            for channel_id in &connection.channels {
                let channel = self.channels.get(channel_id).unwrap();
                assert!(channel.connection_ids.contains(connection_id));
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
                    "room contains participant that has disconnected"
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

        for (channel_id, channel) in &self.channels {
            for connection_id in &channel.connection_ids {
                let connection = self.connections.get(connection_id).unwrap();
                assert!(connection.channels.contains(channel_id));
            }
        }
    }
}

impl Project {
    fn is_active_since(&self, start_time: OffsetDateTime) -> bool {
        self.guests
            .values()
            .chain([&self.host])
            .any(|collaborator| {
                collaborator
                    .last_activity
                    .map_or(false, |active_time| active_time > start_time)
            })
    }

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

impl Channel {
    fn connection_ids(&self) -> Vec<ConnectionId> {
        self.connection_ids.iter().copied().collect()
    }
}
