pub mod participant;
pub mod room;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use client::{proto, Client, TypedEnvelope, User, UserStore};
use collections::HashSet;
use futures::{future::Shared, FutureExt};
use postage::watch;

use gpui::{
    AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Subscription, Task, WeakModelHandle,
};
use project::Project;

pub use participant::ParticipantLocation;
pub use room::Room;

pub fn init(client: Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    let active_call = cx.add_model(|cx| ActiveCall::new(client, user_store, cx));
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
    room: Option<(ModelHandle<Room>, Vec<Subscription>)>,
    pending_room_creation: Option<Shared<Task<Result<ModelHandle<Room>, Arc<anyhow::Error>>>>>,
    location: Option<WeakModelHandle<Project>>,
    pending_invites: HashSet<u64>,
    incoming_call: (
        watch::Sender<Option<IncomingCall>>,
        watch::Receiver<Option<IncomingCall>>,
    ),
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _subscriptions: Vec<client::Subscription>,
}

impl Entity for ActiveCall {
    type Event = room::Event;
}

impl ActiveCall {
    fn new(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            room: None,
            pending_room_creation: None,
            location: None,
            pending_invites: Default::default(),
            incoming_call: watch::channel(),
            _subscriptions: vec![
                client.add_request_handler(cx.handle(), Self::handle_incoming_call),
                client.add_message_handler(cx.handle(), Self::handle_call_canceled),
            ],
            client,
            user_store,
        }
    }

    async fn handle_incoming_call(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::IncomingCall>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let user_store = this.read_with(&cx, |this, _| this.user_store.clone());
        let call = IncomingCall {
            room_id: envelope.payload.room_id,
            participants: user_store
                .update(&mut cx, |user_store, cx| {
                    user_store.get_users(envelope.payload.participant_user_ids, cx)
                })
                .await?,
            calling_user: user_store
                .update(&mut cx, |user_store, cx| {
                    user_store.get_user(envelope.payload.calling_user_id, cx)
                })
                .await?,
            initial_project: envelope.payload.initial_project,
        };
        this.update(&mut cx, |this, _| {
            *this.incoming_call.0.borrow_mut() = Some(call);
        });

        Ok(proto::Ack {})
    }

    async fn handle_call_canceled(
        this: ModelHandle<Self>,
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

    pub fn global(cx: &AppContext) -> ModelHandle<Self> {
        cx.global::<ModelHandle<Self>>().clone()
    }

    pub fn invite(
        &mut self,
        called_user_id: u64,
        initial_project: Option<ModelHandle<Project>>,
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
            cx.spawn_weak(|_, mut cx| async move {
                let room = room.await.map_err(|err| anyhow!("{:?}", err))?;

                let initial_project_id = if let Some(initial_project) = initial_project {
                    Some(
                        room.update(&mut cx, |room, cx| room.share_project(initial_project, cx))
                            .await?,
                    )
                } else {
                    None
                };

                room.update(&mut cx, |room, cx| {
                    room.call(called_user_id, initial_project_id, cx)
                })
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
                            })
                            .await?;

                        this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx))
                            .await?;

                        anyhow::Ok(room)
                    };

                    let room = create_room.await;
                    this.update(&mut cx, |this, _| this.pending_room_creation = None);
                    room.map_err(Arc::new)
                })
                .shared();
            self.pending_room_creation = Some(room.clone());
            cx.foreground().spawn(async move {
                room.await.map_err(|err| anyhow!("{:?}", err))?;
                anyhow::Ok(())
            })
        };

        cx.spawn(|this, mut cx| async move {
            let result = invite.await;
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
        cx.foreground().spawn(async move {
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
            this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx))
                .await?;
            Ok(())
        })
    }

    pub fn decline_incoming(&mut self) -> Result<()> {
        let call = self
            .incoming_call
            .0
            .borrow_mut()
            .take()
            .ok_or_else(|| anyhow!("no incoming call"))?;
        self.client.send(proto::DeclineCall {
            room_id: call.room_id,
        })?;
        Ok(())
    }

    pub fn hang_up(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        cx.notify();
        if let Some((room, _)) = self.room.take() {
            room.update(cx, |room, cx| room.leave(cx))
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn share_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<u64>> {
        if let Some((room, _)) = self.room.as_ref() {
            room.update(cx, |room, cx| room.share_project(project, cx))
        } else {
            Task::ready(Err(anyhow!("no active call")))
        }
    }

    pub fn unshare_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Some((room, _)) = self.room.as_ref() {
            room.update(cx, |room, cx| room.unshare_project(project, cx))
        } else {
            Err(anyhow!("no active call"))
        }
    }

    pub fn set_location(
        &mut self,
        project: Option<&ModelHandle<Project>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.location = project.map(|project| project.downgrade());
        if let Some((room, _)) = self.room.as_ref() {
            room.update(cx, |room, cx| room.set_location(project, cx))
        } else {
            Task::ready(Ok(()))
        }
    }

    fn set_room(
        &mut self,
        room: Option<ModelHandle<Room>>,
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
                    let location = self.location.and_then(|location| location.upgrade(cx));
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

    pub fn room(&self) -> Option<&ModelHandle<Room>> {
        self.room.as_ref().map(|(room, _)| room)
    }

    pub fn pending_invites(&self) -> &HashSet<u64> {
        &self.pending_invites
    }
}
