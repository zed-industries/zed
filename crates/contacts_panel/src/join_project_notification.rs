use client::User;
use gpui::{
    actions, ElementBox, Entity, ModelHandle, MutableAppContext, RenderContext, View, ViewContext,
};
use project::Project;
use std::sync::Arc;
use workspace::Notification;

use crate::notifications::render_user_notification;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(JoinProjectNotification::decline);
    cx.add_action(JoinProjectNotification::accept);
}

pub enum Event {
    Dismiss,
}

actions!(contacts_panel, [Accept, Decline]);

pub struct JoinProjectNotification {
    project: ModelHandle<Project>,
    user: Arc<User>,
}

impl JoinProjectNotification {
    pub fn new(project: ModelHandle<Project>, user: Arc<User>) -> Self {
        Self { project, user }
    }

    fn decline(&mut self, _: &Decline, cx: &mut ViewContext<Self>) {
        self.project.update(cx, |project, cx| {
            project.respond_to_join_request(self.user.id, false, cx)
        });
        cx.emit(Event::Dismiss)
    }

    fn accept(&mut self, _: &Accept, cx: &mut ViewContext<Self>) {
        self.project.update(cx, |project, cx| {
            project.respond_to_join_request(self.user.id, true, cx)
        });
        cx.emit(Event::Dismiss)
    }
}

impl Entity for JoinProjectNotification {
    type Event = Event;
}

impl View for JoinProjectNotification {
    fn ui_name() -> &'static str {
        "JoinProjectNotification"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        render_user_notification(
            self.user.clone(),
            "wants to join your project",
            Decline,
            vec![("Decline", Box::new(Decline)), ("Accept", Box::new(Accept))],
            cx,
        )
    }
}

impl Notification for JoinProjectNotification {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool {
        matches!(event, Event::Dismiss)
    }
}
