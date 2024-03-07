pub mod call_settings;
pub mod participant;
pub mod room;

use anyhow::{anyhow, Result};
use audio::Audio;
use call_settings::CallSettings;
use client::{proto, ChannelId, Client, TypedEnvelope, User, UserStore, ZED_ALWAYS_ACTIVE};
use collections::HashSet;
use futures::{channel::oneshot, future::Shared, Future, FutureExt};
use gpui::{
    AppContext, AsyncAppContext, Context, EventEmitter, Global, Model, ModelContext, Subscription,
    Task, WeakModel,
};
use postage::watch;
use project::Project;
use room::Event;
use settings::Settings;
use std::sync::Arc;

pub use participant::ParticipantLocation;
pub use room::Room;

struct GlobalActiveCall(Model<ActiveCall>);

impl Global for GlobalActiveCall {}

pub fn init(client: Arc<Client>, user_store: Model<UserStore>, cx: &mut AppContext) {
    CallSettings::register(cx);

    let active_call = cx.new_model(|cx| ActiveCall::new(client, user_store, cx));
    cx.set_global(GlobalActiveCall(active_call));
}

pub struct OneAtATime {
    cancel: Option<oneshot::Sender<()>>,
}

impl OneAtATime {
    /// spawn a task in the given context.
    /// if another task is spawned before that resolves, or if the OneAtATime itself is dropped, the first task will be cancelled and return Ok(None)
    /// otherwise you'll see the result of the task.
    fn spawn<F, Fut, R>(&mut self, cx: &mut AppContext, f: F) -> Task<Result<Option<R>>>
    where
        F: 'static + FnOnce(AsyncAppContext) -> Fut,
        Fut: Future<Output = Result<R>>,
        R: 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.cancel.replace(tx);
        cx.spawn(|cx| async move {
            futures::select_biased! {
                _ = rx.fuse() => Ok(None),
                result = f(cx).fuse() => result.map(Some),
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
    room: Option<(Model<Room>, Vec<Subscription>)>,
    pending_room_creation: Option<Shared<Task<Result<Model<Room>, Arc<anyhow::Error>>>>>,
    location: Option<WeakModel<Project>>,
    _join_debouncer: OneAtATime,
    pending_invites: HashSet<u64>,
    incoming_call: (
        watch::Sender<Option<IncomingCall>>,
        watch::Receiver<Option<IncomingCall>>,
    ),
    client: Arc<Client>,
    user_store: Model<UserStore>,
    _subscriptions: Vec<client::Subscription>,
}

impl EventEmitter<Event> for ActiveCall {}

impl ActiveCall {
    fn new(client: Arc<Client>, user_store: Model<UserStore>, cx: &mut ModelContext<Self>) -> Self {
        Self {
            room: None,
            pending_room_creation: None,
            location: None,
            pending_invites: Default::default(),
            incoming_call: watch::channel(),
            _join_debouncer: OneAtATime { cancel: None },
            _subscriptions: vec![
                client.add_request_handler(cx.weak_model(), Self::handle_incoming_call),
                client.add_message_handler(cx.weak_model(), Self::handle_call_canceled),
            ],
            client,
            user_store,
        }
    }

    pub fn channel_id(&self, cx: &AppContext) -> Option<ChannelId> {
        self.room()?.read(cx).channel_id()
    }

    async fn handle_incoming_call(
        this: Model<Self>,
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
        })?;

        Ok(proto::Ack {})
    }

    async fn handle_call_canceled(
        this: Model<Self>,
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
        })?;
        Ok(())
    }

    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalActiveCall>().0.clone()
    }

    pub fn try_global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<GlobalActiveCall>()
            .map(|call| call.0.clone())
    }

    pub fn invite(
        &mut self,
        called_user_id: u64,
        initial_project: Option<Model<Project>>,
        cx: &mut ModelContext<Self>,
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
            cx.spawn(move |_, mut cx| async move {
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
                .spawn(move |this, mut cx| async move {
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
            cx.background_executor().spawn(async move {
                room.await.map_err(|err| anyhow!("{:?}", err))?;
                anyhow::Ok(())
            })
        };

        cx.spawn(move |this, mut cx| async move {
            let result = invite.await;
            if result.is_ok() {
                this.update(&mut cx, |this, cx| this.report_call_event("invite", cx))?;
            } else {
                //TODO: report collaboration error
                log::error!("invite failed: {:?}", result);
            }

            this.update(&mut cx, |this, cx| {
                this.pending_invites.remove(&called_user_id);
                cx.notify();
            })?;
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
        cx.background_executor().spawn(async move {
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

        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| this.set_room(room.clone(), cx))?
                .await?;
            this.update(&mut cx, |this, cx| {
                this.report_call_event("accept incoming", cx)
            })?;
            Ok(())
        })
    }

    pub fn decline_incoming(&mut self, _: &mut ModelContext<Self>) -> Result<()> {
        let call = self
            .incoming_call
            .0
            .borrow_mut()
            .take()
            .ok_or_else(|| anyhow!("no incoming call"))?;
        report_call_event_for_room("decline incoming", call.room_id, None, &self.client);
        self.client.send(proto::DeclineCall {
            room_id: call.room_id,
        })?;
        Ok(())
    }

    pub fn join_channel(
        &mut self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Model<Room>>>> {
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

        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| this.set_room(room.clone(), cx))?
                .await?;
            this.update(&mut cx, |this, cx| {
                this.report_call_event("join channel", cx)
            })?;
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
        project: Model<Project>,
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
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Some((room, _)) = self.room.as_ref() {
            self.report_call_event("unshare project", cx);
            room.update(cx, |room, cx| room.unshare_project(project, cx))
        } else {
            Err(anyhow!("no active call"))
        }
    }

    pub fn location(&self) -> Option<&WeakModel<Project>> {
        self.location.as_ref()
    }

    pub fn set_location(
        &mut self,
        project: Option<&Model<Project>>,
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
        room: Option<Model<Room>>,
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
                    let channel_id = room.read(cx).channel_id();
                    cx.emit(Event::RoomJoined { channel_id });
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

    pub fn room(&self) -> Option<&Model<Room>> {
        self.room.as_ref().map(|(room, _)| room)
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn pending_invites(&self) -> &HashSet<u64> {
        &self.pending_invites
    }

    pub fn report_call_event(&self, operation: &'static str, cx: &mut AppContext) {
        if let Some(room) = self.room() {
            let room = room.read(cx);
            report_call_event_for_room(operation, room.id(), room.channel_id(), &self.client);
        }
    }
}

pub fn report_call_event_for_room(
    operation: &'static str,
    room_id: u64,
    channel_id: Option<ChannelId>,
    client: &Arc<Client>,
) {
    let telemetry = client.telemetry();

    telemetry.report_call_event(operation, Some(room_id), channel_id)
}

pub fn report_call_event_for_channel(
    operation: &'static str,
    channel_id: ChannelId,
    client: &Arc<Client>,
    cx: &AppContext,
) {
    let room = ActiveCall::global(cx).read(cx).room();

    let telemetry = client.telemetry();

    telemetry.report_call_event(operation, room.map(|r| r.read(cx).id()), Some(channel_id))
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
                    assert!(false);
                    Ok(2)
                }),
                one_at_a_time.spawn(cx, |_| async { Ok(3) }),
            )
        });

        assert_eq!(a.await.unwrap(), None);
        assert_eq!(b.await.unwrap(), Some(3));

        let promise = cx.update(|cx| one_at_a_time.spawn(cx, |_| async { Ok(4) }));
        drop(one_at_a_time);

        assert_eq!(promise.await.unwrap(), None);
    }
}
