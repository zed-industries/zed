use client::{User, UserStore};
use gpui::{color::Color, elements::*, Entity, ModelHandle, View};
use std::sync::Arc;
use workspace::Notification;

pub struct IncomingRequestNotification {
    user: Arc<User>,
    user_store: ModelHandle<UserStore>,
}

impl Entity for IncomingRequestNotification {
    type Event = ();
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

impl Notification for IncomingRequestNotification {}

impl IncomingRequestNotification {
    pub fn new(user: Arc<User>, user_store: ModelHandle<UserStore>) -> Self {
        Self { user, user_store }
    }
}
