mod active_call;
mod participant;

use anyhow::{anyhow, Result};
use client::{call::Call, proto, Client, PeerId, TypedEnvelope, User, UserStore};
use collections::HashMap;
use futures::StreamExt;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use participant::{LocalParticipant, ParticipantLocation, RemoteParticipant};
use project::Project;
use std::sync::Arc;
use util::ResultExt;

pub enum Event {
    PeerChangedActiveProject,
}

pub struct Room {
    id: u64,
    status: RoomStatus,
    local_participant: LocalParticipant,
    remote_participants: HashMap<PeerId, RemoteParticipant>,
    pending_users: Vec<Arc<User>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _subscriptions: Vec<client::Subscription>,
    _load_pending_users: Option<Task<()>>,
}

impl Entity for Room {
    type Event = Event;
}

impl Room {
    pub fn observe<F>(cx: &mut MutableAppContext, mut callback: F) -> gpui::Subscription
    where
        F: 'static + FnMut(Option<ModelHandle<Self>>, &mut MutableAppContext),
    {
        cx.observe_default_global::<Option<ModelHandle<Self>>, _>(move |cx| {
            let room = cx.global::<Option<ModelHandle<Self>>>().clone();
            callback(room, cx);
        })
    }

    pub fn get_or_create(
        client: &Arc<Client>,
        user_store: &ModelHandle<UserStore>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        if let Some(room) = cx.global::<Option<ModelHandle<Self>>>() {
            Task::ready(Ok(room.clone()))
        } else {
            let client = client.clone();
            let user_store = user_store.clone();
            cx.spawn(|mut cx| async move {
                let room = cx.update(|cx| Room::create(client, user_store, cx)).await?;
                cx.update(|cx| cx.set_global(Some(room.clone())));
                Ok(room)
            })
        }
    }

    pub fn clear(cx: &mut MutableAppContext) {
        cx.set_global::<Option<ModelHandle<Self>>>(None);
    }

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
            local_participant: LocalParticipant {
                projects: Default::default(),
            },
            remote_participants: Default::default(),
            pending_users: Default::default(),
            _subscriptions: vec![client.add_message_handler(cx.handle(), Self::handle_room_updated)],
            _load_pending_users: None,
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
        call: &Call,
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

    fn apply_room_update(&mut self, room: proto::Room, cx: &mut ModelContext<Self>) -> Result<()> {
        // TODO: compute diff instead of clearing participants
        self.remote_participants.clear();
        for participant in room.participants {
            if Some(participant.user_id) != self.client.user_id() {
                self.remote_participants.insert(
                    PeerId(participant.peer_id),
                    RemoteParticipant {
                        user_id: participant.user_id,
                        projects: Default::default(), // TODO: populate projects
                        location: ParticipantLocation::from_proto(participant.location)?,
                    },
                );
            }
        }

        let pending_users = self.user_store.update(cx, move |user_store, cx| {
            user_store.get_users(room.pending_user_ids, cx)
        });
        self._load_pending_users = Some(cx.spawn(|this, mut cx| async move {
            if let Some(pending_users) = pending_users.await.log_err() {
                this.update(&mut cx, |this, cx| {
                    this.pending_users = pending_users;
                    cx.notify();
                });
            }
        }));

        cx.notify();
        Ok(())
    }

    pub fn call(
        &mut self,
        recipient_user_id: u64,
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
                })
                .await?;
            Ok(())
        })
    }

    pub fn publish_project(&mut self, project: ModelHandle<Project>) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        todo!()
    }

    pub fn unpublish_project(&mut self, project: ModelHandle<Project>) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        todo!()
    }

    pub fn set_active_project(
        &mut self,
        project: Option<&ModelHandle<Project>>,
    ) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        todo!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RoomStatus {
    Online,
    Offline,
}

impl RoomStatus {
    fn is_offline(&self) -> bool {
        matches!(self, RoomStatus::Offline)
    }
}
