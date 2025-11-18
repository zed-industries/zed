pub mod participant;
pub mod room;

use anyhow::{Context as _, Result, anyhow};
use audio::Audio;
use client::{ChannelId, Client, TypedEnvelope, User, UserStore, ZED_ALWAYS_ACTIVE, proto};
use collections::HashSet;
use futures::{Future, FutureExt, channel::oneshot, future::Shared};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Global, Subscription, Task,
    WeakEntity,
};
use postage::watch;
use project::Project;
use room::Event;
use std::sync::Arc;

pub use livekit_client::{RemoteVideoTrack, RemoteVideoTrackView, RemoteVideoTrackViewEvent};
pub use participant::ParticipantLocation;
pub use room::Room;

struct GlobalActiveCall(Entity<ActiveCall>);

impl Global for GlobalActiveCall {}

pub fn init(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
    let active_call = cx.new(|cx| ActiveCall::new(client, user_store, cx));
    cx.set_global(GlobalActiveCall(active_call));
}

pub struct OneAtATime {
    cancel: Option<oneshot::Sender<()>>,
}

impl OneAtATime {
    /// spawn a task in the given context.
    /// if another task is spawned before that resolves, or if the OneAtATime itself is dropped, the first task will be cancelled and return Ok(None)
    /// otherwise you'll see the result of the task.
    fn spawn<F, Fut, R>(&mut self, cx: &mut App, f: F) -> Task<Result<Option<R>>>
    where
        F: 'static + FnOnce(AsyncApp) -> Fut,
        Fut: Future<Output = Result<R>>,
        R: 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.cancel.replace(tx);
        cx.spawn(async move |cx| {
            futures::select_biased! {
                _ = rx.fuse() => Ok(None),
                result = f(cx.clone()).fuse() => result.map(Some),
            }
        })
    }

    fn running(&self) -> bool {
        self.cancel
            .as_ref()
            .is_some_and(|cancel| !cancel.is_canceled())
    }
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
    room: Option<(Entity<Room>, Vec<Subscription>)>,
    pending_room_creation: Option<Shared<Task<Result<Entity<Room>, Arc<anyhow::Error>>>>>,
    location: Option<WeakEntity<Project>>,
    _join_debouncer: OneAtATime,
    pending_invites: HashSet<u64>,
    incoming_call: (
        watch::Sender<Option<IncomingCall>>,
        watch::Receiver<Option<IncomingCall>>,
    ),
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    _subscriptions: Vec<client::Subscription>,
}

impl EventEmitter<Event> for ActiveCall {}

impl ActiveCall {
    fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        Self {
            room: None,
            pending_room_creation: None,
            location: None,
            pending_invites: Default::default(),
            incoming_call: watch::channel(),
            _join_debouncer: OneAtATime { cancel: None },
            _subscriptions: vec![
                client.add_request_handler(cx.weak_entity(), Self::handle_incoming_call),
                client.add_message_handler(cx.weak_entity(), Self::handle_call_canceled),
            ],
            client,
            user_store,
        }
    }

    pub fn channel_id(&self, cx: &App) -> Option<ChannelId> {
        self.room()?.read(cx).channel_id()
    }

    async fn handle_incoming_call(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::IncomingCall>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let user_store = this.read_with(&cx, |this, _| this.user_store.clone())?;
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
        })?;

        Ok(proto::Ack {})
    }

    async fn handle_call_canceled(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CallCanceled>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            let mut incoming_call = this.incoming_call.0.borrow_mut();
            if incoming_call
                .as_ref()
                .is_some_and(|call| call.room_id == envelope.payload.room_id)
            {
                incoming_call.take();
            }
        })?;
        Ok(())
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalActiveCall>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalActiveCall>()
            .map(|call| call.0.clone())
    }

    pub fn invite(
        &mut self,
        called_user_id: u64,
        initial_project: Option<Entity<Project>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if !self.pending_invites.insert(called_user_id) {
            return Task::ready(Err(anyhow!("user was already invited")));
        }
        cx.notify();

        if self._join_debouncer.running() {
            return Task::ready(Ok(()));
        }

        let room = if let Some(room) = self.room().cloned() {
            Some(Task::ready(Ok(room)).shared())
        } else {
            self.pending_room_creation.clone()
        };

        let invite = if let Some(room) = room {
            cx.spawn(async move |_, cx| {
                let room = room.await.map_err(|err| anyhow!("{err:?}"))?;

                let initial_project_id = if let Some(initial_project) = initial_project {
                    Some(
                        room.update(cx, |room, cx| room.share_project(initial_project, cx))?
                            .await?,
                    )
                } else {
                    None
                };

                room.update(cx, move |room, cx| {
                    room.call(called_user_id, initial_project_id, cx)
                })?
                .await?;

                anyhow::Ok(())
            })
        } else {
            let client = self.client.clone();
            let user_store = self.user_store.clone();
            let room = cx
                .spawn(async move |this, cx| {
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

                        this.update(cx, |this, cx| this.set_room(Some(room.clone()), cx))?
                            .await?;

                        anyhow::Ok(room)
                    };

                    let room = create_room.await;
                    this.update(cx, |this, _| this.pending_room_creation = None)?;
                    room.map_err(Arc::new)
                })
                .shared();
            self.pending_room_creation = Some(room.clone());
            cx.background_spawn(async move {
                room.await.map_err(|err| anyhow!("{err:?}"))?;
                anyhow::Ok(())
            })
        };

        cx.spawn(async move |this, cx| {
            let result = invite.await;
            if result.is_ok() {
                this.update(cx, |this, cx| {
                    this.report_call_event("Participant Invited", cx)
                })?;
            } else {
                //TODO: report collaboration error
                log::error!("invite failed: {:?}", result);
            }

            this.update(cx, |this, cx| {
                this.pending_invites.remove(&called_user_id);
                cx.notify();
            })?;
            result
        })
    }

    pub fn cancel_invite(
        &mut self,
        called_user_id: u64,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let room_id = if let Some(room) = self.room() {
            room.read(cx).id()
        } else {
            return Task::ready(Err(anyhow!("no active call")));
        };

        let client = self.client.clone();
        cx.background_spawn(async move {
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

    pub fn accept_incoming(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        if self.room.is_some() {
            return Task::ready(Err(anyhow!("cannot join while on another call")));
        }

        let call = if let Some(call) = self.incoming_call.0.borrow_mut().take() {
            call
        } else {
            return Task::ready(Err(anyhow!("no incoming call")));
        };

        if self.pending_room_creation.is_some() {
            return Task::ready(Ok(()));
        }

        let room_id = call.room_id;
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let join = self
            ._join_debouncer
            .spawn(cx, move |cx| Room::join(room_id, client, user_store, cx));

        cx.spawn(async move |this, cx| {
            let room = join.await?;
            this.update(cx, |this, cx| this.set_room(room.clone(), cx))?
                .await?;
            this.update(cx, |this, cx| {
                this.report_call_event("Incoming Call Accepted", cx)
            })?;
            Ok(())
        })
    }

    pub fn decline_incoming(&mut self, _: &mut Context<Self>) -> Result<()> {
        let call = self
            .incoming_call
            .0
            .borrow_mut()
            .take()
            .context("no incoming call")?;
        telemetry::event!("Incoming Call Declined", room_id = call.room_id);
        self.client.send(proto::DeclineCall {
            room_id: call.room_id,
        })?;
        Ok(())
    }

    pub fn join_channel(
        &mut self,
        channel_id: ChannelId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Entity<Room>>>> {
        if let Some(room) = self.room().cloned() {
            if room.read(cx).channel_id() == Some(channel_id) {
                return Task::ready(Ok(Some(room)));
            } else {
                room.update(cx, |room, cx| room.clear_state(cx));
            }
        }

        if self.pending_room_creation.is_some() {
            return Task::ready(Ok(None));
        }

        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let join = self._join_debouncer.spawn(cx, move |cx| async move {
            Room::join_channel(channel_id, client, user_store, cx).await
        });

        cx.spawn(async move |this, cx| {
            let room = join.await?;
            this.update(cx, |this, cx| this.set_room(room.clone(), cx))?
                .await?;
            this.update(cx, |this, cx| this.report_call_event("Channel Joined", cx))?;
            Ok(room)
        })
    }

    pub fn hang_up(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        cx.notify();
        self.report_call_event("Call Ended", cx);

        Audio::end_call(cx);

        let channel_id = self.channel_id(cx);
        if let Some((room, _)) = self.room.take() {
            cx.emit(Event::RoomLeft { channel_id });
            room.update(cx, |room, cx| room.leave(cx))
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn share_project(
        &mut self,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Result<u64>> {
        if let Some((room, _)) = self.room.as_ref() {
            self.report_call_event("Project Shared", cx);
            room.update(cx, |room, cx| room.share_project(project, cx))
        } else {
            Task::ready(Err(anyhow!("no active call")))
        }
    }

    pub fn unshare_project(
        &mut self,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let (room, _) = self.room.as_ref().context("no active call")?;
        self.report_call_event("Project Unshared", cx);
        room.update(cx, |room, cx| room.unshare_project(project, cx))
    }

    pub fn location(&self) -> Option<&WeakEntity<Project>> {
        self.location.as_ref()
    }

    pub fn set_location(
        &mut self,
        project: Option<&Entity<Project>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if project.is_some() || !*ZED_ALWAYS_ACTIVE {
            self.location = project.map(|project| project.downgrade());
            if let Some((room, _)) = self.room.as_ref() {
                return room.update(cx, |room, cx| room.set_location(project, cx));
            }
        }
        Task::ready(Ok(()))
    }

    fn set_room(&mut self, room: Option<Entity<Room>>, cx: &mut Context<Self>) -> Task<Result<()>> {
        if room.as_ref() == self.room.as_ref().map(|room| &room.0) {
            Task::ready(Ok(()))
        } else {
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
                    let channel_id = room.read(cx).channel_id();
                    cx.emit(Event::RoomJoined { channel_id });
                    room.update(cx, |room, cx| room.set_location(location.as_ref(), cx))
                }
            } else {
                self.room = None;
                Task::ready(Ok(()))
            }
        }
    }

    pub fn room(&self) -> Option<&Entity<Room>> {
        self.room.as_ref().map(|(room, _)| room)
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn pending_invites(&self) -> &HashSet<u64> {
        &self.pending_invites
    }

    pub fn report_call_event(&self, operation: &'static str, cx: &mut App) {
        if let Some(room) = self.room() {
            let room = room.read(cx);
            telemetry::event!(
                operation,
                room_id = room.id(),
                channel_id = room.channel_id()
            )
        }
    }
}

#[cfg(test)]
mod test {
    use gpui::TestAppContext;

    use crate::OneAtATime;

    #[gpui::test]
    async fn test_one_at_a_time(cx: &mut TestAppContext) {
        let mut one_at_a_time = OneAtATime { cancel: None };

        assert_eq!(
            cx.update(|cx| one_at_a_time.spawn(cx, |_| async { Ok(1) }))
                .await
                .unwrap(),
            Some(1)
        );

        let (a, b) = cx.update(|cx| {
            (
                one_at_a_time.spawn(cx, |_| async {
                    panic!("");
                }),
                one_at_a_time.spawn(cx, |_| async { Ok(3) }),
            )
        });

        assert_eq!(a.await.unwrap(), None::<u32>);
        assert_eq!(b.await.unwrap(), Some(3));

        let promise = cx.update(|cx| one_at_a_time.spawn(cx, |_| async { Ok(4) }));
        drop(one_at_a_time);

        assert_eq!(promise.await.unwrap(), None);
    }
}
