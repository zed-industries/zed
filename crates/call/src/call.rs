mod participant;
pub mod room;

use anyhow::{anyhow, Result};
use client::{incoming_call::IncomingCall, Client, UserStore};
use gpui::{Entity, ModelContext, ModelHandle, MutableAppContext, Task};
pub use room::Room;
use std::sync::Arc;

pub fn init(client: Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    let active_call = cx.add_model(|_| ActiveCall::new(client, user_store));
    cx.set_global(active_call);
}

pub struct ActiveCall {
    room: Option<ModelHandle<Room>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
}

impl Entity for ActiveCall {
    type Event = ();
}

impl ActiveCall {
    fn new(client: Arc<Client>, user_store: ModelHandle<UserStore>) -> Self {
        Self {
            room: None,
            client,
            user_store,
        }
    }

    pub fn global(cx: &mut MutableAppContext) -> ModelHandle<Self> {
        cx.global::<ModelHandle<Self>>().clone()
    }

    pub fn invite(
        &mut self,
        recipient_user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let room = self.room.clone();

        let client = self.client.clone();
        let user_store = self.user_store.clone();
        cx.spawn(|this, mut cx| async move {
            let room = if let Some(room) = room {
                room
            } else {
                let room = cx.update(|cx| Room::create(client, user_store, cx)).await?;
                this.update(&mut cx, |this, cx| {
                    this.room = Some(room.clone());
                    cx.notify();
                });
                room
            };
            room.update(&mut cx, |room, cx| room.call(recipient_user_id, cx))
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
            this.update(&mut cx, |this, cx| {
                this.room = Some(room);
                cx.notify();
            });
            Ok(())
        })
    }

    pub fn room(&self) -> Option<&ModelHandle<Room>> {
        self.room.as_ref()
    }
}
