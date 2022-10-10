use crate::{
    participant::{ParticipantLocation, RemoteParticipant},
    IncomingCall,
};
use anyhow::{anyhow, Result};
use client::{proto, Client, PeerId, TypedEnvelope, User, UserStore};
use collections::{BTreeMap, HashSet};
use futures::StreamExt;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use project::Project;
use std::sync::Arc;
use util::ResultExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    RemoteProjectShared {
        owner: Arc<User>,
        project_id: u64,
        worktree_root_names: Vec<String>,
    },
}

pub struct Room {
    id: u64,
    status: RoomStatus,
    remote_participants: BTreeMap<PeerId, RemoteParticipant>,
    pending_participants: Vec<Arc<User>>,
    participant_user_ids: HashSet<u64>,
    pending_call_count: usize,
    leave_when_empty: bool,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    subscriptions: Vec<client::Subscription>,
    pending_room_update: Option<Task<()>>,
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
            participant_user_ids: Default::default(),
            remote_participants: Default::default(),
            pending_participants: Default::default(),
            pending_call_count: 0,
            subscriptions: vec![client.add_message_handler(cx.handle(), Self::handle_room_updated)],
            leave_when_empty: false,
            pending_room_update: None,
            client,
            user_store,
        }
    }

    pub(crate) fn create(
        recipient_user_id: u64,
        initial_project: Option<ModelHandle<Project>>,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let response = client.request(proto::CreateRoom {}).await?;
            let room = cx.add_model(|cx| Self::new(response.id, client, user_store, cx));

            let initial_project_id = if let Some(initial_project) = initial_project {
                let initial_project_id = room
                    .update(&mut cx, |room, cx| {
                        room.share_project(initial_project.clone(), cx)
                    })
                    .await?;
                Some(initial_project_id)
            } else {
                None
            };

            match room
                .update(&mut cx, |room, cx| {
                    room.leave_when_empty = true;
                    room.call(recipient_user_id, initial_project_id, cx)
                })
                .await
            {
                Ok(()) => Ok(room),
                Err(error) => Err(anyhow!("room creation failed: {:?}", error)),
            }
        })
    }

    pub(crate) fn join(
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
            room.update(&mut cx, |room, cx| {
                room.leave_when_empty = true;
                room.apply_room_update(room_proto, cx)?;
                anyhow::Ok(())
            })?;
            Ok(room)
        })
    }

    fn should_leave(&self) -> bool {
        self.leave_when_empty
            && self.pending_room_update.is_none()
            && self.pending_participants.is_empty()
            && self.remote_participants.is_empty()
            && self.pending_call_count == 0
    }

    pub(crate) fn leave(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        if self.status.is_offline() {
            return Err(anyhow!("room is offline"));
        }

        cx.notify();
        self.status = RoomStatus::Offline;
        self.remote_participants.clear();
        self.pending_participants.clear();
        self.participant_user_ids.clear();
        self.subscriptions.clear();
        self.client.send(proto::LeaveRoom { id: self.id })?;
        Ok(())
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn status(&self) -> RoomStatus {
        self.status
    }

    pub fn remote_participants(&self) -> &BTreeMap<PeerId, RemoteParticipant> {
        &self.remote_participants
    }

    pub fn pending_participants(&self) -> &[Arc<User>] {
        &self.pending_participants
    }

    pub fn contains_participant(&self, user_id: u64) -> bool {
        self.participant_user_ids.contains(&user_id)
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
        this.update(&mut cx, |this, cx| this.apply_room_update(room, cx))
    }

    fn apply_room_update(
        &mut self,
        mut room: proto::Room,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        // Filter ourselves out from the room's participants.
        room.participants
            .retain(|participant| Some(participant.user_id) != self.client.user_id());

        let remote_participant_user_ids = room
            .participants
            .iter()
            .map(|p| p.user_id)
            .collect::<Vec<_>>();
        let (remote_participants, pending_participants) =
            self.user_store.update(cx, move |user_store, cx| {
                (
                    user_store.get_users(remote_participant_user_ids, cx),
                    user_store.get_users(room.pending_participant_user_ids, cx),
                )
            });
        self.pending_room_update = Some(cx.spawn(|this, mut cx| async move {
            let (remote_participants, pending_participants) =
                futures::join!(remote_participants, pending_participants);

            this.update(&mut cx, |this, cx| {
                this.participant_user_ids.clear();

                if let Some(participants) = remote_participants.log_err() {
                    for (participant, user) in room.participants.into_iter().zip(participants) {
                        let peer_id = PeerId(participant.peer_id);
                        this.participant_user_ids.insert(participant.user_id);

                        let existing_projects = this
                            .remote_participants
                            .get(&peer_id)
                            .into_iter()
                            .flat_map(|existing| &existing.projects)
                            .map(|project| project.id)
                            .collect::<HashSet<_>>();
                        for project in &participant.projects {
                            if !existing_projects.contains(&project.id) {
                                cx.emit(Event::RemoteProjectShared {
                                    owner: user.clone(),
                                    project_id: project.id,
                                    worktree_root_names: project.worktree_root_names.clone(),
                                });
                            }
                        }

                        this.remote_participants.insert(
                            peer_id,
                            RemoteParticipant {
                                user: user.clone(),
                                projects: participant.projects,
                                location: ParticipantLocation::from_proto(participant.location)
                                    .unwrap_or(ParticipantLocation::External),
                            },
                        );
                    }

                    this.remote_participants.retain(|_, participant| {
                        this.participant_user_ids.contains(&participant.user.id)
                    });

                    cx.notify();
                }

                if let Some(pending_participants) = pending_participants.log_err() {
                    this.pending_participants = pending_participants;
                    for participant in &this.pending_participants {
                        this.participant_user_ids.insert(participant.id);
                    }
                    cx.notify();
                }

                this.pending_room_update.take();
                if this.should_leave() {
                    let _ = this.leave(cx);
                }

                this.check_invariants();
            });
        }));

        cx.notify();
        Ok(())
    }

    fn check_invariants(&self) {
        #[cfg(any(test, feature = "test-support"))]
        {
            for participant in self.remote_participants.values() {
                assert!(self.participant_user_ids.contains(&participant.user.id));
            }

            for participant in &self.pending_participants {
                assert!(self.participant_user_ids.contains(&participant.id));
            }

            assert_eq!(
                self.participant_user_ids.len(),
                self.remote_participants.len() + self.pending_participants.len()
            );
        }
    }

    pub(crate) fn call(
        &mut self,
        recipient_user_id: u64,
        initial_project_id: Option<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        cx.notify();
        let client = self.client.clone();
        let room_id = self.id;
        self.pending_call_count += 1;
        cx.spawn(|this, mut cx| async move {
            let result = client
                .request(proto::Call {
                    room_id,
                    recipient_user_id,
                    initial_project_id,
                })
                .await;
            this.update(&mut cx, |this, cx| {
                this.pending_call_count -= 1;
                if this.should_leave() {
                    this.leave(cx)?;
                }
                result
            })?;
            Ok(())
        })
    }

    pub(crate) fn share_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<u64>> {
        if project.read(cx).is_remote() {
            return Task::ready(Err(anyhow!("can't share remote project")));
        } else if let Some(project_id) = project.read(cx).remote_id() {
            return Task::ready(Ok(project_id));
        }

        let request = self.client.request(proto::ShareProject {
            room_id: self.id(),
            worktrees: project
                .read(cx)
                .worktrees(cx)
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    proto::WorktreeMetadata {
                        id: worktree.id().to_proto(),
                        root_name: worktree.root_name().into(),
                        visible: worktree.is_visible(),
                    }
                })
                .collect(),
        });
        cx.spawn_weak(|_, mut cx| async move {
            let response = request.await?;
            project
                .update(&mut cx, |project, cx| {
                    project.shared(response.project_id, cx)
                })
                .await?;
            Ok(response.project_id)
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
