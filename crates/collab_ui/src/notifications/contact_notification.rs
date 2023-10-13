use crate::notifications::render_user_notification;
use client::{ContactEventKind, User, UserStore};
use gpui::{elements::*, Entity, ModelHandle, View, ViewContext};
use std::sync::Arc;
use workspace::notifications::Notification;

pub struct ContactNotification {
    user_store: ModelHandle<UserStore>,
    user: Arc<User>,
    notification: rpc::Notification,
}

#[derive(Clone, PartialEq)]
struct Dismiss(u64);

#[derive(Clone, PartialEq)]
pub struct RespondToContactRequest {
    pub user_id: u64,
    pub accept: bool,
}

pub enum Event {
    Dismiss,
}

impl Entity for ContactNotification {
    type Event = Event;
}

impl View for ContactNotification {
    fn ui_name() -> &'static str {
        "ContactNotification"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        match self.notification {
            rpc::Notification::ContactRequest { .. } => render_user_notification(
                self.user.clone(),
                "wants to add you as a contact",
                Some("They won't be alerted if you decline."),
                |notification, cx| notification.dismiss(cx),
                vec![
                    (
                        "Decline",
                        Box::new(|notification, cx| {
                            notification.respond_to_contact_request(false, cx)
                        }),
                    ),
                    (
                        "Accept",
                        Box::new(|notification, cx| {
                            notification.respond_to_contact_request(true, cx)
                        }),
                    ),
                ],
                cx,
            ),
            rpc::Notification::ContactRequestAccepted { .. } => render_user_notification(
                self.user.clone(),
                "accepted your contact request",
                None,
                |notification, cx| notification.dismiss(cx),
                vec![],
                cx,
            ),
            _ => unreachable!(),
        }
    }
}

impl Notification for ContactNotification {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool {
        matches!(event, Event::Dismiss)
    }
}

impl ContactNotification {
    pub fn new(
        user: Arc<User>,
        notification: rpc::Notification,
        user_store: ModelHandle<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(&user_store, move |this, _, event, cx| {
            if let client::Event::Contact {
                kind: ContactEventKind::Cancelled,
                user,
            } = event
            {
                if user.id == this.user.id {
                    cx.emit(Event::Dismiss);
                }
            }
        })
        .detach();

        Self {
            user,
            notification,
            user_store,
        }
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        self.user_store.update(cx, |store, cx| {
            store
                .dismiss_contact_request(self.user.id, cx)
                .detach_and_log_err(cx);
        });
        cx.emit(Event::Dismiss);
    }

    fn respond_to_contact_request(&mut self, accept: bool, cx: &mut ViewContext<Self>) {
        self.user_store
            .update(cx, |store, cx| {
                store.respond_to_contact_request(self.user.id, accept, cx)
            })
            .detach();
    }
}
