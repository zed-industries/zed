pub mod call_settings;
pub mod participant;
pub mod room;

use anyhow::{anyhow, Result};
use audio2::Audio;
use call_settings::CallSettings;
use client2::{
    proto, ClickhouseEvent, Client, TelemetrySettings, TypedEnvelope, User, UserStore,
    ZED_ALWAYS_ACTIVE,
};
use collections::HashSet;
use futures::{future::Shared, FutureExt};
use gpui2::{
    AppContext, AsyncAppContext, Context, EventEmitter, Handle, ModelContext, Subscription, Task,
    WeakHandle,
};
use postage::watch;
use project2::Project;
use std::sync::Arc;

pub use participant::ParticipantLocation;
pub use room::Room;

pub fn init(client: Arc<Client>, user_store: Handle<UserStore>, cx: &mut AppContext) {
    settings2::register::<CallSettings>(cx);

    let active_call = cx.entity(|cx| ActiveCall::new(client, user_store, cx));
    cx.set_global(active_call);
}

#[derive(Clone)]
pub struct IncomingCall {
    pub room_id: u64,
    pub calling_user: Arc<User>,
    pub participants: Vec<Arc<User>>,
    pub initial_project: Option<proto::ParticipantProject>,
}

/// Singleton global maintaining the user's participation in a room across workspaces.
pub struct ActiveCall {
    room: Option<(Handle<Room>, Vec<Subscription>)>,
    pending_room_creation: Option<Shared<Task<Result<Handle<Room>, Arc<anyhow::Error>>>>>,
    location: Option<WeakHandle<Project>>,
    pending_invites: HashSet<u64>,
    incoming_call: (
        watch::Sender<Option<IncomingCall>>,
        watch::Receiver<Option<IncomingCall>>,
    ),
    client: Arc<Client>,
    user_store: Handle<UserStore>,
    _subscriptions: Vec<client2::Subscription>,
}

impl EventEmitter for ActiveCall {
    type Event = room::Event;
}

impl ActiveCall {
    fn new(
        client: Arc<Client>,
        user_store: Handle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            room: None,
            pending_room_creation: None,
            location: None,
            pending_invites: Default::default(),
            incoming_call: watch::channel(),

            _subscriptions: vec![
                client.add_request_handler(cx.weak_handle(), Self::handle_incoming_call),
                client.add_message_handler(cx.weak_handle(), Self::handle_call_canceled),
            ],
            client,
            user_store,
        }
    }

    pub fn channel_id(&self, cx: &AppContext) -> Option<u64> {
        self.room()?.read(cx).channel_id()
    }

    async fn handle_incoming_call(
        this: Handle<Self>,
        envelope: TypedEnvelope<proto::IncomingCall>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let user_store = this.update(&mut cx, |this, _| this.user_store.clone())?;
        let call = IncomingCall {
            room_id: envelope.payload.room_id,
            participants: user_store
                .update(&mut cx, |user_store, cx| {
                    user_store.get_users(envelope.payload.participant_user_ids, cx)
                })?
                .await?,
            calling_user: user_store
                .update(&mut cx, |user_store, cx| {
                    user_store.get_user(envelope.payload.calling_user_id, cx)
                })?
                .await?,
            initial_project: envelope.payload.initial_project,
        };
        this.update(&mut cx, |this, _| {
            *this.incoming_call.0.borrow_mut() = Some(call);
        });

        Ok(proto::Ack {})
    }

    async fn handle_call_canceled(
        this: Handle<Self>,
        envelope: TypedEnvelope<proto::CallCanceled>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            let mut incoming_call = this.incoming_call.0.borrow_mut();
            if incoming_call
                .as_ref()
                .map_or(false, |call| call.room_id == envelope.payload.room_id)
            {
                incoming_call.take();
            }
        });
        Ok(())
    }

    pub fn global(cx: &AppContext) -> Handle<Self> {
        cx.global::<Handle<Self>>().clone()
    }

    pub fn invite(
        &mut self,
        called_user_id: u64,
        initial_project: Option<Handle<Project>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.pending_invites.insert(called_user_id) {
            return Task::ready(Err(anyhow!("user was already invited")));
        }
        cx.notify();

        let room = if let Some(room) = self.room().cloned() {
            Some(Task::ready(Ok(room)).shared())
        } else {
            self.pending_room_creation.clone()
        };

        let invite = if let Some(room) = room {
            cx.spawn(|_, mut cx| async move {
                let room = room.await.map_err(|err| anyhow!("{:?}", err))?;

                let initial_project_id = if let Some(initial_project) = initial_project {
                    Some(
                        room.update(&mut cx, |room, cx| room.share_project(initial_project, cx))?
                            .await?,
                    )
                } else {
                    None
                };

                room.update(&mut cx, move |room, cx| {
                    room.call(called_user_id, initial_project_id, cx)
                })?
                .await?;

                anyhow::Ok(())
            })
        } else {
            let client = self.client.clone();
            let user_store = self.user_store.clone();
            let room = cx
                .spawn(|this, mut cx| async move {
                    let create_room = async {
                        let room = cx
                            .update(|cx| {
                                Room::create(
                                    called_user_id,
                                    initial_project,
                                    client,
                                    user_store,
                                    cx,
                                )
                            })?
                            .await?;

                        this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx))?
                            .await?;

                        anyhow::Ok(room)
                    };

                    let room = create_room.await;
                    this.update(&mut cx, |this, _| this.pending_room_creation = None)?;
                    room.map_err(Arc::new)
                })
                .shared();
            self.pending_room_creation = Some(room.clone());
            cx.executor().spawn(async move {
                room.await.map_err(|err| anyhow!("{:?}", err))?;
                anyhow::Ok(())
            })
        };

        cx.spawn(|this, mut cx| async move {
            let result = invite.await;
            if result.is_ok() {
                this.update(&mut cx, |this, cx| this.report_call_event("invite", cx));
            } else {
                // TODO: Resport collaboration error
            }

            this.update(&mut cx, |this, cx| {
                this.pending_invites.remove(&called_user_id);
                cx.notify();
            });
            result
        })
    }

    pub fn cancel_invite(
        &mut self,
        called_user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let room_id = if let Some(room) = self.room() {
            room.read(cx).id()
        } else {
            return Task::ready(Err(anyhow!("no active call")));
        };

        let client = self.client.clone();
        cx.executor().spawn(async move {
            client
                .request(proto::CancelCall {
                    room_id,
                    called_user_id,
                })
                .await?;
            anyhow::Ok(())
        })
    }

    pub fn incoming(&self) -> watch::Receiver<Option<IncomingCall>> {
        self.incoming_call.1.clone()
    }

    pub fn accept_incoming(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if self.room.is_some() {
            return Task::ready(Err(anyhow!("cannot join while on another call")));
        }

        let call = if let Some(call) = self.incoming_call.1.borrow().clone() {
            call
        } else {
            return Task::ready(Err(anyhow!("no incoming call")));
        };

        let join = Room::join(&call, self.client.clone(), self.user_store.clone(), cx);

        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx))?
                .await?;
            this.update(&mut cx, |this, cx| {
                this.report_call_event("accept incoming", cx)
            });
            Ok(())
        })
    }

    pub fn decline_incoming(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        let call = self
            .incoming_call
            .0
            .borrow_mut()
            .take()
            .ok_or_else(|| anyhow!("no incoming call"))?;
        report_call_event_for_room("decline incoming", call.room_id, None, &self.client, cx);
        self.client.send(proto::DeclineCall {
            room_id: call.room_id,
        })?;
        Ok(())
    }

    pub fn join_channel(
        &mut self,
        channel_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Handle<Room>>> {
        if let Some(room) = self.room().cloned() {
            if room.read(cx).channel_id() == Some(channel_id) {
                return Task::ready(Ok(room));
            } else {
                room.update(cx, |room, cx| room.clear_state(cx));
            }
        }

        let join = Room::join_channel(channel_id, self.client.clone(), self.user_store.clone(), cx);

        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx))?
                .await?;
            this.update(&mut cx, |this, cx| {
                this.report_call_event("join channel", cx)
            });
            Ok(room)
        })
    }

    pub fn hang_up(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        cx.notify();
        self.report_call_event("hang up", cx);

        Audio::end_call(cx);
        if let Some((room, _)) = self.room.take() {
            room.update(cx, |room, cx| room.leave(cx))
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn share_project(
        &mut self,
        project: Handle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<u64>> {
        if let Some((room, _)) = self.room.as_ref() {
            self.report_call_event("share project", cx);
            room.update(cx, |room, cx| room.share_project(project, cx))
        } else {
            Task::ready(Err(anyhow!("no active call")))
        }
    }

    pub fn unshare_project(
        &mut self,
        project: Handle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Some((room, _)) = self.room.as_ref() {
            self.report_call_event("unshare project", cx);
            room.update(cx, |room, cx| room.unshare_project(project, cx))
        } else {
            Err(anyhow!("no active call"))
        }
    }

    pub fn location(&self) -> Option<&WeakHandle<Project>> {
        self.location.as_ref()
    }

    pub fn set_location(
        &mut self,
        project: Option<&Handle<Project>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if project.is_some() || !*ZED_ALWAYS_ACTIVE {
            self.location = project.map(|project| project.downgrade());
            if let Some((room, _)) = self.room.as_ref() {
                return room.update(cx, |room, cx| room.set_location(project, cx));
            }
        }
        Task::ready(Ok(()))
    }

    fn set_room(
        &mut self,
        room: Option<Handle<Room>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if room.as_ref() != self.room.as_ref().map(|room| &room.0) {
            cx.notify();
            if let Some(room) = room {
                if room.read(cx).status().is_offline() {
                    self.room = None;
                    Task::ready(Ok(()))
                } else {
                    let subscriptions = vec![
                        cx.observe(&room, |this, room, cx| {
                            if room.read(cx).status().is_offline() {
                                this.set_room(None, cx).detach_and_log_err(cx);
                            }

                            cx.notify();
                        }),
                        cx.subscribe(&room, |_, _, event, cx| cx.emit(event.clone())),
                    ];
                    self.room = Some((room.clone(), subscriptions));
                    let location = self
                        .location
                        .as_ref()
                        .and_then(|location| location.upgrade());
                    room.update(cx, |room, cx| room.set_location(location.as_ref(), cx))
                }
            } else {
                self.room = None;
                Task::ready(Ok(()))
            }
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn room(&self) -> Option<&Handle<Room>> {
        self.room.as_ref().map(|(room, _)| room)
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn pending_invites(&self) -> &HashSet<u64> {
        &self.pending_invites
    }

    pub fn report_call_event(&self, operation: &'static str, cx: &AppContext) {
        if let Some(room) = self.room() {
            let room = room.read(cx);
            report_call_event_for_room(operation, room.id(), room.channel_id(), &self.client, cx);
        }
    }
}

pub fn report_call_event_for_room(
    operation: &'static str,
    room_id: u64,
    channel_id: Option<u64>,
    client: &Arc<Client>,
    cx: &AppContext,
) {
    let telemetry = client.telemetry();
    let telemetry_settings = *settings2::get::<TelemetrySettings>(cx);
    let event = ClickhouseEvent::Call {
        operation,
        room_id: Some(room_id),
        channel_id,
    };
    telemetry.report_clickhouse_event(event, telemetry_settings);
}

pub fn report_call_event_for_channel(
    operation: &'static str,
    channel_id: u64,
    client: &Arc<Client>,
    cx: &AppContext,
) {
    let room = ActiveCall::global(cx).read(cx).room();

    let telemetry = client.telemetry();
    let telemetry_settings = *settings2::get::<TelemetrySettings>(cx);

    let event = ClickhouseEvent::Call {
        operation,
        room_id: room.map(|r| r.read(cx).id()),
        channel_id: Some(channel_id),
    };
    telemetry.report_clickhouse_event(event, telemetry_settings);
}
