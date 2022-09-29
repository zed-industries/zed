use crate::Room;
use anyhow::{anyhow, Result};
use client::{call::Call, Client, UserStore};
use gpui::{Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use std::sync::Arc;
use util::ResultExt;

#[derive(Default)]
pub struct ActiveCall {
    room: Option<ModelHandle<Room>>,
}

impl Entity for ActiveCall {
    type Event = ();
}

impl ActiveCall {
    pub fn global(cx: &mut MutableAppContext) -> ModelHandle<Self> {
        if cx.has_global::<ModelHandle<Self>>() {
            cx.global::<ModelHandle<Self>>().clone()
        } else {
            let active_call = cx.add_model(|_| ActiveCall::default());
            cx.set_global(active_call.clone());
            active_call
        }
    }

    pub fn get_or_create(
        &mut self,
        client: &Arc<Client>,
        user_store: &ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Room>>> {
        if let Some(room) = self.room.clone() {
            Task::ready(Ok(room))
        } else {
            let client = client.clone();
            let user_store = user_store.clone();
            cx.spawn(|this, mut cx| async move {
                let room = cx.update(|cx| Room::create(client, user_store, cx)).await?;
                this.update(&mut cx, |this, cx| {
                    this.room = Some(room.clone());
                    cx.notify();
                });
                Ok(room)
            })
        }
    }

    pub fn join(
        &mut self,
        call: &Call,
        client: &Arc<Client>,
        user_store: &ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Room>>> {
        if self.room.is_some() {
            return Task::ready(Err(anyhow!("cannot join while on another call")));
        }

        let join = Room::join(call, client.clone(), user_store.clone(), cx);
        cx.spawn(|this, mut cx| async move {
            let room = join.await?;
            this.update(&mut cx, |this, cx| {
                this.room = Some(room.clone());
                cx.notify();
            });
            Ok(room)
        })
    }

    pub fn room(&self) -> Option<&ModelHandle<Room>> {
        self.room.as_ref()
    }

    pub fn clear(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(room) = self.room.take() {
            room.update(cx, |room, cx| room.leave(cx)).log_err();
            cx.notify();
        }
    }
}
