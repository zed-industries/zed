use crate::{
    participant::{LocalParticipant, ParticipantLocation, RemoteParticipant, RemoteVideoTrack},
    IncomingCall,
};
use anyhow::{anyhow, Result};
use client::{proto, Client, PeerId, TypedEnvelope, User, UserStore};
use collections::{BTreeMap, HashSet};
use futures::StreamExt;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use live_kit_client::{LocalVideoTrack, RemoteVideoTrackUpdate};
use postage::watch;
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
    RemoteProjectUnshared {
        project_id: u64,
    },
    Left,
}

pub struct Room {
    id: u64,
    live_kit_room: Option<(Arc<live_kit_client::Room>, Task<()>)>,
    status: RoomStatus,
    local_participant: LocalParticipant,
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
        live_kit_connection_info: Option<proto::LiveKitConnectionInfo>,
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

        let live_kit_room = if let Some(connection_info) = live_kit_connection_info {
            let room = live_kit_client::Room::new();
            let mut track_changes = room.remote_video_track_updates();
            let maintain_room = cx.spawn_weak(|this, mut cx| async move {
                while let Some(track_change) = track_changes.next().await {
                    let this = if let Some(this) = this.upgrade(&cx) {
                        this
                    } else {
                        break;
                    };

                    this.update(&mut cx, |this, cx| {
                        this.remote_video_track_updated(track_change, cx).log_err()
                    });
                }
            });
            cx.foreground()
                .spawn(room.connect(&connection_info.server_url, &connection_info.token))
                .detach_and_log_err(cx);
            Some((room, maintain_room))
        } else {
            None
        };

        Self {
            id,
            live_kit_room,
            status: RoomStatus::Online,
            participant_user_ids: Default::default(),
            local_participant: Default::default(),
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
            let room_proto = response.room.ok_or_else(|| anyhow!("invalid room"))?;
            let room = cx.add_model(|cx| {
                Self::new(
                    room_proto.id,
                    response.live_kit_connection_info,
                    client,
                    user_store,
                    cx,
                )
            });

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
            let room = cx.add_model(|cx| {
                Self::new(
                    room_id,
                    response.live_kit_connection_info,
                    client,
                    user_store,
                    cx,
                )
            });
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
        cx.emit(Event::Left);
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

    pub fn local_participant(&self) -> &LocalParticipant {
        &self.local_participant
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
        let local_participant_ix = room
            .participants
            .iter()
            .position(|participant| Some(participant.user_id) == self.client.user_id());
        let local_participant = local_participant_ix.map(|ix| room.participants.swap_remove(ix));

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

                if let Some(participant) = local_participant {
                    this.local_participant.projects = participant.projects;
                } else {
                    this.local_participant.projects.clear();
                }

                if let Some(participants) = remote_participants.log_err() {
                    for (participant, user) in room.participants.into_iter().zip(participants) {
                        let peer_id = PeerId(participant.peer_id);
                        this.participant_user_ids.insert(participant.user_id);

                        let old_projects = this
                            .remote_participants
                            .get(&peer_id)
                            .into_iter()
                            .flat_map(|existing| &existing.projects)
                            .map(|project| project.id)
                            .collect::<HashSet<_>>();
                        let new_projects = participant
                            .projects
                            .iter()
                            .map(|project| project.id)
                            .collect::<HashSet<_>>();

                        for project in &participant.projects {
                            if !old_projects.contains(&project.id) {
                                cx.emit(Event::RemoteProjectShared {
                                    owner: user.clone(),
                                    project_id: project.id,
                                    worktree_root_names: project.worktree_root_names.clone(),
                                });
                            }
                        }

                        for unshared_project_id in old_projects.difference(&new_projects) {
                            cx.emit(Event::RemoteProjectUnshared {
                                project_id: *unshared_project_id,
                            });
                        }

                        this.remote_participants.insert(
                            peer_id,
                            RemoteParticipant {
                                user: user.clone(),
                                projects: participant.projects,
                                location: ParticipantLocation::from_proto(participant.location)
                                    .unwrap_or(ParticipantLocation::External),
                                tracks: Default::default(),
                            },
                        );

                        if let Some((room, _)) = this.live_kit_room.as_ref() {
                            println!("getting video tracks for peer id {}", peer_id.0.to_string());
                            let tracks = room.remote_video_tracks(&peer_id.0.to_string());
                            dbg!(tracks.len());
                            for track in tracks {
                                dbg!(track.id(), track.publisher_id());
                                this.remote_video_track_updated(
                                    RemoteVideoTrackUpdate::Subscribed(track),
                                    cx,
                                )
                                .log_err();
                            }
                        }
                    }

                    this.remote_participants.retain(|_, participant| {
                        if this.participant_user_ids.contains(&participant.user.id) {
                            true
                        } else {
                            for project in &participant.projects {
                                cx.emit(Event::RemoteProjectUnshared {
                                    project_id: project.id,
                                });
                            }
                            false
                        }
                    });
                }

                if let Some(pending_participants) = pending_participants.log_err() {
                    this.pending_participants = pending_participants;
                    for participant in &this.pending_participants {
                        this.participant_user_ids.insert(participant.id);
                    }
                }

                this.pending_room_update.take();
                if this.should_leave() {
                    let _ = this.leave(cx);
                }

                this.check_invariants();
                cx.notify();
            });
        }));

        cx.notify();
        Ok(())
    }

    fn remote_video_track_updated(
        &mut self,
        change: RemoteVideoTrackUpdate,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        match change {
            RemoteVideoTrackUpdate::Subscribed(track) => {
                dbg!(track.publisher_id(), track.id());
                let peer_id = PeerId(track.publisher_id().parse()?);
                let track_id = track.id().to_string();
                let participant = self
                    .remote_participants
                    .get_mut(&peer_id)
                    .ok_or_else(|| anyhow!("subscribed to track by unknown participant"))?;
                let (mut tx, mut rx) = watch::channel();
                track.add_renderer(move |frame| *tx.borrow_mut() = Some(frame));
                participant.tracks.insert(
                    track_id.clone(),
                    RemoteVideoTrack {
                        frame: None,
                        _live_kit_track: track,
                        _maintain_frame: Arc::new(cx.spawn_weak(|this, mut cx| async move {
                            while let Some(frame) = rx.next().await {
                                let this = if let Some(this) = this.upgrade(&cx) {
                                    this
                                } else {
                                    break;
                                };

                                let done = this.update(&mut cx, |this, cx| {
                                    // TODO: replace this with an emit.
                                    cx.notify();
                                    if let Some(track) =
                                        this.remote_participants.get_mut(&peer_id).and_then(
                                            |participant| participant.tracks.get_mut(&track_id),
                                        )
                                    {
                                        track.frame = frame;
                                        false
                                    } else {
                                        true
                                    }
                                });

                                if done {
                                    break;
                                }
                            }
                        })),
                    },
                );
            }
            RemoteVideoTrackUpdate::Unsubscribed {
                publisher_id,
                track_id,
            } => {
                let peer_id = PeerId(publisher_id.parse()?);
                let participant = self
                    .remote_participants
                    .get_mut(&peer_id)
                    .ok_or_else(|| anyhow!("unsubscribed from track by unknown participant"))?;
                participant.tracks.remove(&track_id);
            }
        }

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
        if let Some(project_id) = project.read(cx).remote_id() {
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
        cx.spawn(|this, mut cx| async move {
            let response = request.await?;

            project.update(&mut cx, |project, cx| {
                project
                    .shared(response.project_id, cx)
                    .detach_and_log_err(cx)
            });

            // If the user's location is in this project, it changes from UnsharedProject to SharedProject.
            this.update(&mut cx, |this, cx| {
                let active_project = this.local_participant.active_project.as_ref();
                if active_project.map_or(false, |location| *location == project) {
                    this.set_location(Some(&project), cx)
                } else {
                    Task::ready(Ok(()))
                }
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
            self.local_participant.active_project = Some(project.downgrade());
            if let Some(project_id) = project.read(cx).remote_id() {
                proto::participant_location::Variant::SharedProject(
                    proto::participant_location::SharedProject { id: project_id },
                )
            } else {
                proto::participant_location::Variant::UnsharedProject(
                    proto::participant_location::UnsharedProject {},
                )
            }
        } else {
            self.local_participant.active_project = None;
            proto::participant_location::Variant::External(proto::participant_location::External {})
        };

        cx.notify();
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

    pub fn share_screen(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if self.status.is_offline() {
            return Task::ready(Err(anyhow!("room is offline")));
        }

        let room = if let Some((room, _)) = self.live_kit_room.as_ref() {
            room.clone()
        } else {
            return Task::ready(Err(anyhow!("not connected to LiveKit")));
        };

        cx.foreground().spawn(async move {
            let display = live_kit_client::display_sources().await?;
            // let display = displays
            //     .first()
            //     .ok_or_else(|| anyhow!("no display found"))?;
            let track = LocalVideoTrack::screen_share_for_display(&display);
            room.publish_video_track(&track).await?;
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
