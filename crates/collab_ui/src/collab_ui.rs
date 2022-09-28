mod collab_titlebar_item;
mod contacts_popover;

use client::{call::Call, UserStore};
pub use collab_titlebar_item::CollabTitlebarItem;
use futures::StreamExt;
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Entity, ModelHandle, MutableAppContext, View, WindowBounds, WindowKind, WindowOptions,
};
use settings::Settings;

pub fn init(user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    contacts_popover::init(cx);
    collab_titlebar_item::init(cx);

    let mut incoming_call = user_store.read(cx).incoming_call();
    cx.spawn(|mut cx| async move {
        let mut notification_window = None;
        while let Some(incoming_call) = incoming_call.next().await {
            if let Some(window_id) = notification_window.take() {
                cx.remove_window(window_id);
            }

            if let Some(incoming_call) = incoming_call {
                let (window_id, _) = cx.add_window(
                    WindowOptions {
                        bounds: WindowBounds::Fixed(RectF::new(vec2f(0., 0.), vec2f(300., 400.))),
                        titlebar: None,
                        center: true,
                        kind: WindowKind::PopUp,
                        is_movable: false,
                    },
                    |_| IncomingCallNotification::new(incoming_call),
                );
                notification_window = Some(window_id);
            }
        }
    })
    .detach();
}

struct IncomingCallNotification {
    call: Call,
}

impl IncomingCallNotification {
    fn new(call: Call) -> Self {
        Self { call }
    }
}

impl Entity for IncomingCallNotification {
    type Event = ();
}

impl View for IncomingCallNotification {
    fn ui_name() -> &'static str {
        "IncomingCallNotification"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let theme = &cx.global::<Settings>().theme.contacts_panel;
        Flex::row()
            .with_children(self.call.from.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(
                    self.call.from.github_login.clone(),
                    theme.contact_username.text.clone(),
                )
                .contained()
                .aligned()
                .left()
                .flex(1., true)
                .boxed(),
            )
            .boxed()
    }
}
