use crate::Room;
use gpui::{Entity, ModelHandle, MutableAppContext};

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
            let active_call = cx.add_model(|_| ActiveCall::default());
            cx.set_global(active_call.clone());
            active_call
        } else {
            cx.global::<ModelHandle<Self>>().clone()
        }
    }

    pub fn observe<F>(cx: &mut MutableAppContext, mut callback: F) -> gpui::Subscription
    where
        F: 'static + FnMut(Option<ModelHandle<Room>>, &mut MutableAppContext),
    {
        cx.observe_default_global::<Option<ModelHandle<Room>>, _>(move |cx| {
            let room = cx.global::<Option<ModelHandle<Room>>>().clone();
            callback(room, cx);
        })
    }

    pub fn get_or_create(
        client: &Arc<Client>,
        user_store: &ModelHandle<UserStore>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Room>>> {
        if let Some(room) = cx.global::<Option<ModelHandle<Room>>>() {
            Task::ready(Ok(room.clone()))
        } else {
            let client = client.clone();
            let user_store = user_store.clone();
            cx.spawn(|mut cx| async move {
                let room = cx.update(|cx| Room::create(client, user_store, cx)).await?;
                cx.update(|cx| cx.set_global(Some(room.clone())));
                Ok(room)
            })
        }
    }

    pub fn clear(cx: &mut MutableAppContext) {
        cx.set_global::<Option<ModelHandle<Room>>>(None);
    }
}
