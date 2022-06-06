use std::sync::Arc;

use crate::notifications::render_user_notification;
use client::{ContactEventKind, User, UserStore};
use gpui::{
    elements::*, impl_internal_actions, Entity, ModelHandle, MutableAppContext, RenderContext,
    View, ViewContext,
};
use workspace::Notification;

impl_internal_actions!(contact_notifications, [Dismiss, RespondToContactRequest]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ContactNotification::dismiss);
    cx.add_action(ContactNotification::respond_to_contact_request);
}

pub struct ContactNotification {
    user_store: ModelHandle<UserStore>,
    user: Arc<User>,
    kind: client::ContactEventKind,
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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        match self.kind {
            ContactEventKind::Requested => render_user_notification(
                self.user.clone(),
                "wants to add you as a contact",
                Some("They won't know if you decline."),
                RespondToContactRequest {
                    user_id: self.user.id,
                    accept: false,
                },
                vec![
                    (
                        "Decline",
                        Box::new(RespondToContactRequest {
                            user_id: self.user.id,
                            accept: false,
                        }),
                    ),
                    (
                        "Accept",
                        Box::new(RespondToContactRequest {
                            user_id: self.user.id,
                            accept: true,
                        }),
                    ),
                ],
                cx,
            ),
            ContactEventKind::Accepted => render_user_notification(
                self.user.clone(),
                "accepted your contact request",
                None,
                Dismiss(self.user.id),
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
        kind: client::ContactEventKind,
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
            kind,
            user_store,
        }
    }

    fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        self.user_store.update(cx, |store, cx| {
            store
                .dismiss_contact_request(self.user.id, cx)
                .detach_and_log_err(cx);
        });
        cx.emit(Event::Dismiss);
    }

    fn respond_to_contact_request(
        &mut self,
        action: &RespondToContactRequest,
        cx: &mut ViewContext<Self>,
    ) {
        self.user_store
            .update(cx, |store, cx| {
                store.respond_to_contact_request(action.user_id, action.accept, cx)
            })
            .detach();
    }
}
