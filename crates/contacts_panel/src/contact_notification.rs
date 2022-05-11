use client::{ContactEvent, ContactEventKind, UserStore};
use gpui::{
    elements::*, impl_internal_actions, platform::CursorStyle, Entity, ModelHandle,
    MutableAppContext, RenderContext, View, ViewContext,
};
use settings::Settings;
use workspace::Notification;

impl_internal_actions!(contact_notifications, [Dismiss, RespondToContactRequest]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ContactNotification::dismiss);
    cx.add_action(ContactNotification::respond_to_contact_request);
}

pub struct ContactNotification {
    user_store: ModelHandle<UserStore>,
    event: ContactEvent,
}

#[derive(Clone)]
struct Dismiss(u64);

#[derive(Clone)]
pub struct RespondToContactRequest {
    pub user_id: u64,
    pub accept: bool,
}

pub enum Event {
    Dismiss,
}

enum Reject {}
enum Accept {}

impl Entity for ContactNotification {
    type Event = Event;
}

impl View for ContactNotification {
    fn ui_name() -> &'static str {
        "ContactNotification"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        match self.event.kind {
            ContactEventKind::Requested => self.render_incoming_request(cx),
            ContactEventKind::Accepted => self.render_acceptance(cx),
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
        event: ContactEvent,
        user_store: ModelHandle<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(&user_store, move |this, _, event, cx| {
            if let client::ContactEvent {
                kind: ContactEventKind::Cancelled,
                user,
            } = event
            {
                if user.id == this.event.user.id {
                    cx.emit(Event::Dismiss);
                }
            }
        })
        .detach();

        Self { event, user_store }
    }

    fn render_incoming_request(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let theme = &theme.contact_notification;
        let user = &self.event.user;
        let user_id = user.id;

        Flex::column()
            .with_child(self.render_header("added you", theme, cx))
            .with_child(
                Label::new(
                    "They won't know if you decline.".to_string(),
                    theme.body_message.text.clone(),
                )
                .contained()
                .with_style(theme.body_message.container)
                .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(
                        MouseEventHandler::new::<Reject, _, _>(
                            self.event.user.id as usize,
                            cx,
                            |_, _| {
                                Label::new("Reject".to_string(), theme.button.text.clone())
                                    .contained()
                                    .with_style(theme.button.container)
                                    .boxed()
                            },
                        )
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(move |_, cx| {
                            cx.dispatch_action(RespondToContactRequest {
                                user_id,
                                accept: false,
                            });
                        })
                        .boxed(),
                    )
                    .with_child(
                        MouseEventHandler::new::<Accept, _, _>(user.id as usize, cx, |_, _| {
                            Label::new("Accept".to_string(), theme.button.text.clone())
                                .contained()
                                .with_style(theme.button.container)
                                .boxed()
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(move |_, cx| {
                            cx.dispatch_action(RespondToContactRequest {
                                user_id,
                                accept: true,
                            });
                        })
                        .boxed(),
                    )
                    .aligned()
                    .right()
                    .boxed(),
            )
            .contained()
            .boxed()
    }

    fn render_acceptance(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let theme = &theme.contact_notification;

        self.render_header("accepted your contact request", theme, cx)
    }

    fn render_header(
        &self,
        message: &'static str,
        theme: &theme::ContactNotification,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let user = &self.event.user;
        let user_id = user.id;
        Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.header_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(
                    format!("{} {}", user.github_login, message),
                    theme.header_message.text.clone(),
                )
                .contained()
                .with_style(theme.header_message.container)
                .aligned()
                .boxed(),
            )
            .with_child(
                MouseEventHandler::new::<Dismiss, _, _>(user.id as usize, cx, |_, _| {
                    Svg::new("icons/reject.svg")
                        .with_color(theme.dismiss_button.color)
                        .constrained()
                        .with_width(theme.dismiss_button.icon_width)
                        .aligned()
                        .contained()
                        .with_style(theme.dismiss_button.container)
                        .constrained()
                        .with_width(theme.dismiss_button.button_width)
                        .with_height(theme.dismiss_button.button_width)
                        .aligned()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, cx| cx.dispatch_action(Dismiss(user_id)))
                .flex_float()
                .boxed(),
            )
            .constrained()
            .with_height(theme.header_height)
            .boxed()
    }

    fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        self.user_store.update(cx, |store, cx| {
            store
                .dismiss_contact_request(self.event.user.id, cx)
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
