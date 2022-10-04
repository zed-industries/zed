use crate::participant::{ParticipantLocation, RemoteParticipant};
use anyhow::{anyhow, Result};
use client::{incoming_call::IncomingCall, proto, Client, PeerId, TypedEnvelope, User, UserStore};
use collections::{HashMap, HashSet};
use futures::StreamExt;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use project::Project;
use std::sync::Arc;
use util::ResultExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    RemoteProjectShared { owner: Arc<User>, project_id: u64 },
}

pub struct Room {
    id: u64,
    status: RoomStatus,
    remote_participants: HashMap<PeerId, RemoteParticipant>,
    pending_users: Vec<Arc<User>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _subscriptions: Vec<client::Subscription>,
    _pending_room_update: Option<Task<()>>,
}

impl Entity for Room {
    type Event = Event;

    fn release(&mut self, _: &mut MutableAppContext) {
        self.client.send(proto::LeaveRoom { id: self.id }).log_err();
    }
}

impl Room {
    fn new(
        id: u64,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut client_status = client.status();
        cx.spawn_weak(|this, mut cx| async move {
            let is_connected = client_status
                .next()
                .await
                .map_or(false, |s| s.is_connected());
            // Even if we're initially connected, any future change of the status means we momentarily disconnected.
            if !is_connected || client_status.next().await.is_some() {
                if let Some(this) = this.upgrade(&cx) {
                    let _ = this.update(&mut cx, |this, cx| this.leave(cx));
                }
            }
        })
        .detach();

        Self {
            id,
            status: RoomStatus::Online,
            remote_participants: Default::default(),
            pending_users: Default::default(),
            _subscriptions: vec![client.add_message_handler(cx.handle(), Self::handle_room_updated)],
            _pending_room_update: None,
            client,
            user_store,
        }
    }

    pub fn create(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let room = client.request(proto::CreateRoom {}).await?;
            Ok(cx.add_model(|cx| Self::new(room.id, client, user_store, cx)))
        })
    }

    pub fn join(
        call: &IncomingCall,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        let room_id = call.room_id;
        cx.spawn(|mut cx| async move {
            let response = client.request(proto::JoinRoom { id: room_id }).await?;
            let room_proto = response.room.ok_or_else(|| anyhow!("invalid room"))?;
            let room = cx.add_model(|cx| Self::new(room_id, client, user_store, cx));
            room.update(&mut cx, |room, cx| room.apply_room_update(room_proto, cx))?;
            Ok(room)
        })
    }

    pub fn leave(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        if self.status.is_offline() {
            return Err(anyhow!("room is offline"));
        }

        cx.notify();
        self.status = RoomStatus::Offline;
        self.remote_participants.clear();
        self.client.send(proto::LeaveRoom { id: self.id })?;
        Ok(())
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn status(&self) -> RoomStatus {
        self.status
    }

    pub fn remote_participants(&self) -> &HashMap<PeerId, RemoteParticipant> {
        &self.remote_participants
    }

    pub fn pending_users(&self) -> &[Arc<User>] {
        &self.pending_users
    }

    async fn handle_room_updated(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::RoomUpdated>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let room = envelope
            .payload
            .room
            .ok_or_else(|| anyhow!("invalid room"))?;
        this.update(&mut cx, |this, cx| this.apply_room_update(room, cx))?;
        Ok(())
    }

    fn apply_room_update(
        &mut self,
        mut room: proto::Room,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        // Filter ourselves out from the room's participants.
        room.participants
            .retain(|participant| Some(participant.user_id) != self.client.user_id());

        let participant_user_ids = room
            .participants
            .iter()
            .map(|p| p.user_id)
            .collect::<Vec<_>>();
        let (participants, pending_users) = self.user_store.update(cx, move |user_store, cx| {
            (
                user_store.get_users(participant_user_ids, cx),
                user_store.get_users(room.pending_user_ids, cx),
            )
        });
        self._pending_room_update = Some(cx.spawn(|this, mut cx| async move {
            let (participants, pending_users) = futures::join!(participants, pending_users);

            this.update(&mut cx, |this, cx| {
                if let Some(participants) = participants.log_err() {
                    let mut seen_participants = HashSet::default();

                    for (participant, user) in room.participants.into_iter().zip(participants) {
                        let peer_id = PeerId(participant.peer_id);
                        seen_participants.insert(peer_id);

                        let existing_project_ids = this
                            .remote_participants
                            .get(&peer_id)
                            .map(|existing| existing.project_ids.clone())
                            .unwrap_or_default();
                        for project_id in &participant.project_ids {
                            if !existing_project_ids.contains(project_id) {
                                cx.emit(Event::RemoteProjectShared {
                                    owner: user.clone(),
                                    project_id: *project_id,
                                });
                            }
                        }

                        this.remote_participants.insert(
                            peer_id,
                            RemoteParticipant {
                                user: user.clone(),
                                project_ids: participant.project_ids,
                                location: ParticipantLocation::from_proto(participant.location)
                                    .unwrap_or(ParticipantLocation::External),
                            },
                        );
                    }

                    for participant_peer_id in
                        this.remote_participants.keys().copied().collect::<Vec<_>>()
                    {
                        if !seen_participants.contains(&participant_peer_id) {
                            this.remote_participants.remove(&participant_peer_id);
                        }
                    }

                    cx.notify();
                }

                if let Some(pending_users) = pending_users.log_err() {
                    this.pending_users = pending_users;
                    cx.notify();
                }
            });
        }));

        cx.notify();
        Ok(())
    }

    pub fn call(
        &mut self,
        recipient_user_id: u64,
        initial_project_id: Option<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        let client = self.client.clone();
        let room_id = self.id;
        cx.foreground().spawn(async move {
            client
                .request(proto::Call {
                    room_id,
                    recipient_user_id,
                    initial_project_id,
                })
                .await?;
            Ok(())
        })
    }

    pub fn set_location(
        &mut self,
        project: Option<&ModelHandle<Project>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        let client = self.client.clone();
        let room_id = self.id;
        let location = if let Some(project) = project {
            if let Some(project_id) = project.read(cx).remote_id() {
                proto::participant_location::Variant::Project(
                    proto::participant_location::Project { id: project_id },
                )
            } else {
                return Task::ready(Err(anyhow!("project is not shared")));
            }
        } else {
            proto::participant_location::Variant::External(proto::participant_location::External {})
        };

        cx.foreground().spawn(async move {
            client
                .request(proto::UpdateParticipantLocation {
                    room_id,
                    location: Some(proto::ParticipantLocation {
                        variant: Some(location),
                    }),
                })
                .await?;
            Ok(())
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RoomStatus {
    Online,
    Offline,
}

impl RoomStatus {
    pub fn is_offline(&self) -> bool {
        matches!(self, RoomStatus::Offline)
    }
}
