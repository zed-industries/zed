use client::{User, UserStore};
use gpui::{color::Color, elements::*, Entity, ModelHandle, View, ViewContext};
use std::sync::Arc;
use workspace::Notification;

pub struct IncomingRequestNotification {
    user: Arc<User>,
    user_store: ModelHandle<UserStore>,
}

pub enum Event {
    Dismiss,
}

impl Entity for IncomingRequestNotification {
    type Event = Event;
}

impl View for IncomingRequestNotification {
    fn ui_name() -> &'static str {
        "IncomingRequestNotification"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        Empty::new()
            .constrained()
            .with_height(200.)
            .contained()
            .with_background_color(Color::red())
            .boxed()
    }
}

impl Notification for IncomingRequestNotification {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool {
        matches!(event, Event::Dismiss)
    }
}

impl IncomingRequestNotification {
    pub fn new(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let user_id = user.id;
        cx.subscribe(&user_store, move |_, _, event, cx| {
            if let client::Event::ContactRequestCancelled(user) = event {
                if user.id == user_id {
                    cx.emit(Event::Dismiss);
                }
            }
        })
        .detach();

        Self { user, user_store }
    }
}
