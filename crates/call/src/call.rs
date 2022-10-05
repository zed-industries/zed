mod participant;
pub mod room;

use anyhow::{anyhow, Result};
use client::{incoming_call::IncomingCall, Client, UserStore};
use gpui::{AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Subscription, Task};
pub use participant::ParticipantLocation;
use project::Project;
pub use room::Room;
use std::sync::Arc;

pub fn init(client: Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    let active_call = cx.add_model(|_| ActiveCall::new(client, user_store));
    cx.set_global(active_call);
}

pub struct ActiveCall {
    room: Option<(ModelHandle<Room>, Vec<Subscription>)>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
}

impl Entity for ActiveCall {
    type Event = room::Event;
}

impl ActiveCall {
    fn new(client: Arc<Client>, user_store: ModelHandle<UserStore>) -> Self {
        Self {
            room: None,
            client,
            user_store,
        }
    }

    pub fn global(cx: &AppContext) -> ModelHandle<Self> {
        cx.global::<ModelHandle<Self>>().clone()
    }

    pub fn invite(
        &mut self,
        recipient_user_id: u64,
        initial_project: Option<ModelHandle<Project>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let room = self.room.as_ref().map(|(room, _)| room.clone());
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        cx.spawn(|this, mut cx| async move {
            let room = if let Some(room) = room {
                room
            } else {
                cx.update(|cx| Room::create(client, user_store, cx)).await?
            };

            let initial_project_id = if let Some(initial_project) = initial_project {
                let room_id = room.read_with(&cx, |room, _| room.id());
                Some(
                    initial_project
                        .update(&mut cx, |project, cx| project.share(room_id, cx))
                        .await?,
                )
            } else {
                None
            };

            this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx));
            room.update(&mut cx, |room, cx| {
                room.call(recipient_user_id, initial_project_id, cx)
            })
            .await?;

            Ok(())
        })
    }

    pub fn join(&mut self, call: &IncomingCall, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if self.room.is_some() {
            return Task::ready(Err(anyhow!("cannot join while on another call")));
        }

        let join = Room::join(call, self.client.clone(), self.user_store.clone(), cx);
        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| this.set_room(Some(room.clone()), cx));
            Ok(())
        })
    }

    fn set_room(&mut self, room: Option<ModelHandle<Room>>, cx: &mut ModelContext<Self>) {
        if room.as_ref() != self.room.as_ref().map(|room| &room.0) {
            if let Some(room) = room {
                let subscriptions = vec![
                    cx.observe(&room, |_, _, cx| cx.notify()),
                    cx.subscribe(&room, |_, _, event, cx| cx.emit(event.clone())),
                ];
                self.room = Some((room, subscriptions));
            } else {
                self.room = None;
            }
            cx.notify();
        }
    }

    pub fn room(&self) -> Option<&ModelHandle<Room>> {
        self.room.as_ref().map(|(room, _)| room)
    }
}
