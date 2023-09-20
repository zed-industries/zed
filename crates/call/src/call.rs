pub mod call_settings;
pub mod participant;
pub mod room;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use audio::Audio;
use call_settings::CallSettings;
use channel::ChannelId;
use client::{
    proto::{self, PeerId},
    ClickhouseEvent, Client, TelemetrySettings, TypedEnvelope, User, UserStore,
};
use collections::HashSet;
use futures::{future::Shared, FutureExt};
use postage::watch;

use gpui::{
    AnyViewHandle, AnyWeakViewHandle, AppContext, AsyncAppContext, Entity, ModelContext,
    ModelHandle, Subscription, Task, ViewContext, WeakModelHandle,
};
use project::Project;

pub use participant::ParticipantLocation;
pub use room::Room;
use util::ResultExt;

pub fn init(client: Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut AppContext) {
    settings::register::<CallSettings>(cx);

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
    follow_handlers: Vec<FollowHandler>,
    followers: Vec<Follower>,
    _subscriptions: Vec<client::Subscription>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Follower {
    project_id: Option<u64>,
    peer_id: PeerId,
}

struct FollowHandler {
    project_id: Option<u64>,
    root_view: AnyWeakViewHandle,
    get_views:
        Box<dyn Fn(&AnyViewHandle, Option<u64>, &mut AppContext) -> Option<proto::FollowResponse>>,
    update_view: Box<dyn Fn(&AnyViewHandle, PeerId, proto::UpdateFollowers, &mut AppContext)>,
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
            follow_handlers: Default::default(),
            followers: Default::default(),
            _subscriptions: vec![
                client.add_request_handler(cx.handle(), Self::handle_incoming_call),
                client.add_message_handler(cx.handle(), Self::handle_call_canceled),
                client.add_request_handler(cx.handle(), Self::handle_follow),
                client.add_message_handler(cx.handle(), Self::handle_unfollow),
                client.add_message_handler(cx.handle(), Self::handle_update_followers),
            ],
            client,
            user_store,
        }
    }

    pub fn channel_id(&self, cx: &AppContext) -> Option<ChannelId> {
        self.room()?.read(cx).channel_id()
    }

    pub fn add_follow_handler<V: gpui::View, GetViews, UpdateView>(
        &mut self,
        root_view: gpui::ViewHandle<V>,
        project_id: Option<u64>,
        get_views: GetViews,
        update_view: UpdateView,
        _cx: &mut ModelContext<Self>,
    ) where
        GetViews: 'static
            + Fn(&mut V, Option<u64>, &mut gpui::ViewContext<V>) -> Result<proto::FollowResponse>,
        UpdateView:
            'static + Fn(&mut V, PeerId, proto::UpdateFollowers, &mut ViewContext<V>) -> Result<()>,
    {
        self.follow_handlers
            .retain(|h| h.root_view.id() != root_view.id());
        if let Err(ix) = self
            .follow_handlers
            .binary_search_by_key(&(project_id, root_view.id()), |f| {
                (f.project_id, f.root_view.id())
            })
        {
            self.follow_handlers.insert(
                ix,
                FollowHandler {
                    project_id,
                    root_view: root_view.into_any().downgrade(),
                    get_views: Box::new(move |view, project_id, cx| {
                        let view = view.clone().downcast::<V>().unwrap();
                        view.update(cx, |view, cx| get_views(view, project_id, cx).log_err())
                            .flatten()
                    }),
                    update_view: Box::new(move |view, leader_id, message, cx| {
                        let view = view.clone().downcast::<V>().unwrap();
                        view.update(cx, |view, cx| {
                            update_view(view, leader_id, message, cx).log_err()
                        });
                    }),
                },
            );
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

    async fn handle_follow(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::Follow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FollowResponse> {
        this.update(&mut cx, |this, cx| {
            let follower = Follower {
                project_id: envelope.payload.project_id,
                peer_id: envelope.original_sender_id()?,
            };
            let active_project_id = this
                .location
                .as_ref()
                .and_then(|project| project.upgrade(cx)?.read(cx).remote_id());

            let mut response = proto::FollowResponse::default();
            for handler in &this.follow_handlers {
                if follower.project_id != handler.project_id && follower.project_id.is_some() {
                    continue;
                }

                let Some(root_view) = handler.root_view.upgrade(cx) else {
                    continue;
                };

                let Some(handler_response) =
                    (handler.get_views)(&root_view, follower.project_id, cx)
                else {
                    continue;
                };

                if response.views.is_empty() {
                    response.views = handler_response.views;
                } else {
                    response.views.extend_from_slice(&handler_response.views);
                }

                if let Some(active_view_id) = handler_response.active_view_id.clone() {
                    if response.active_view_id.is_none() || handler.project_id == active_project_id
                    {
                        response.active_view_id = Some(active_view_id);
                    }
                }
            }

            if let Err(ix) = this.followers.binary_search(&follower) {
                this.followers.insert(ix, follower);
            }

            Ok(response)
        })
    }

    async fn handle_unfollow(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::Unfollow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            let follower = Follower {
                project_id: envelope.payload.project_id,
                peer_id: envelope.original_sender_id()?,
            };
            if let Err(ix) = this.followers.binary_search(&follower) {
                this.followers.remove(ix);
            }
            Ok(())
        })
    }

    async fn handle_update_followers(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateFollowers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let leader_id = envelope.original_sender_id()?;
        let update = envelope.payload;
        this.update(&mut cx, |this, cx| {
            for handler in &this.follow_handlers {
                if update.project_id != handler.project_id && update.project_id.is_some() {
                    continue;
                }
                let Some(root_view) = handler.root_view.upgrade(cx) else {
                    continue;
                };
                (handler.update_view)(&root_view, leader_id, update.clone(), cx);
            }
            Ok(())
        })
    }

    pub fn update_followers(
        &self,
        project_id: Option<u64>,
        update: proto::update_followers::Variant,
        cx: &AppContext,
    ) -> Option<()> {
        let room_id = self.room()?.read(cx).id();
        let follower_ids: Vec<_> = self
            .followers
            .iter()
            .filter_map(|follower| {
                (follower.project_id == project_id).then_some(follower.peer_id.into())
            })
            .collect();
        if follower_ids.is_empty() {
            return None;
        }
        self.client
            .send(proto::UpdateFollowers {
                room_id,
                project_id,
                follower_ids,
                variant: Some(update),
            })
            .log_err()
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
                this.report_call_event("invite", cx);
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
        Self::report_call_event_for_room(
            "decline incoming",
            Some(call.room_id),
            None,
            &self.client,
            cx,
        );
        self.client.send(proto::DeclineCall {
            room_id: call.room_id,
        })?;
        Ok(())
    }

    pub fn join_channel(
        &mut self,
        channel_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if let Some(room) = self.room().cloned() {
            if room.read(cx).channel_id() == Some(channel_id) {
                return Task::ready(Ok(()));
            } else {
                room.update(cx, |room, cx| room.clear_state(cx));
            }
        }

        let join = Room::join_channel(channel_id, self.client.clone(), self.user_store.clone(), cx);

        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx))
                .await?;
            this.update(&mut cx, |this, cx| {
                this.report_call_event("join channel", cx)
            });
            Ok(())
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
        project: ModelHandle<Project>,
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
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Some((room, _)) = self.room.as_ref() {
            self.report_call_event("unshare project", cx);
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

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn pending_invites(&self) -> &HashSet<u64> {
        &self.pending_invites
    }

    fn report_call_event(&self, operation: &'static str, cx: &AppContext) {
        let (room_id, channel_id) = match self.room() {
            Some(room) => {
                let room = room.read(cx);
                (Some(room.id()), room.channel_id())
            }
            None => (None, None),
        };
        Self::report_call_event_for_room(operation, room_id, channel_id, &self.client, cx)
    }

    pub fn report_call_event_for_room(
        operation: &'static str,
        room_id: Option<u64>,
        channel_id: Option<u64>,
        client: &Arc<Client>,
        cx: &AppContext,
    ) {
        let telemetry = client.telemetry();
        let telemetry_settings = *settings::get::<TelemetrySettings>(cx);
        let event = ClickhouseEvent::Call {
            operation,
            room_id,
            channel_id,
        };
        telemetry.report_clickhouse_event(event, telemetry_settings);
    }
}
