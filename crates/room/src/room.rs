mod participant;

use anyhow::{anyhow, Result};
use client::{call::Call, proto, Client, PeerId, TypedEnvelope};
use collections::HashMap;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use participant::{LocalParticipant, ParticipantLocation, RemoteParticipant};
use project::Project;
use std::sync::Arc;

pub enum Event {
    PeerChangedActiveProject,
}

pub enum CallResponse {
    Accepted,
    Rejected,
}

pub struct Room {
    id: u64,
    local_participant: LocalParticipant,
    remote_participants: HashMap<PeerId, RemoteParticipant>,
    pending_user_ids: Vec<u64>,
    client: Arc<Client>,
    _subscriptions: Vec<client::Subscription>,
}

impl Entity for Room {
    type Event = Event;
}

impl Room {
    pub fn create(
        client: Arc<Client>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let room = client.request(proto::CreateRoom {}).await?;
            Ok(cx.add_model(|cx| Self::new(room.id, client, cx)))
        })
    }

    pub fn join(
        call: &Call,
        client: Arc<Client>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        let room_id = call.room_id;
        cx.spawn(|mut cx| async move {
            let response = client.request(proto::JoinRoom { id: room_id }).await?;
            let room_proto = response.room.ok_or_else(|| anyhow!("invalid room"))?;
            let room = cx.add_model(|cx| Self::new(room_id, client, cx));
            room.update(&mut cx, |room, cx| room.apply_room_update(room_proto, cx))?;
            Ok(room)
        })
    }

    fn new(id: u64, client: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        Self {
            id,
            local_participant: LocalParticipant {
                projects: Default::default(),
            },
            remote_participants: Default::default(),
            pending_user_ids: Default::default(),
            _subscriptions: vec![client.add_message_handler(cx.handle(), Self::handle_room_updated)],
            client,
        }
    }

    pub fn remote_participants(&self) -> &HashMap<PeerId, RemoteParticipant> {
        &self.remote_participants
    }

    pub fn pending_user_ids(&self) -> &[u64] {
        &self.pending_user_ids
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
        self.pending_user_ids = room.pending_user_ids;
        cx.notify();
        Ok(())
    }

    pub fn call(&mut self, to_user_id: u64, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let client = self.client.clone();
        let room_id = self.id;
        cx.foreground().spawn(async move {
            client
                .request(proto::Call {
                    room_id,
                    to_user_id,
                })
                .await?;
            Ok(())
        })
    }

    pub async fn publish_project(&mut self, project: ModelHandle<Project>) -> Result<()> {
        todo!()
    }

    pub async fn unpublish_project(&mut self, project: ModelHandle<Project>) -> Result<()> {
        todo!()
    }

    pub async fn set_active_project(
        &mut self,
        project: Option<&ModelHandle<Project>>,
    ) -> Result<()> {
        todo!()
    }

    pub async fn mute(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn unmute(&mut self) -> Result<()> {
        todo!()
    }
}
