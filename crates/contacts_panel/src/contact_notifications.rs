use client::{User, UserStore};
use gpui::{
    elements::*, impl_internal_actions, platform::CursorStyle, Entity, ModelHandle,
    MutableAppContext, RenderContext, View, ViewContext,
};
use settings::Settings;
use std::sync::Arc;
use workspace::Notification;

impl_internal_actions!(contact_notifications, [Dismiss]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(IncomingRequestNotification::dismiss);
}

pub struct IncomingRequestNotification {
    user: Arc<User>,
    user_store: ModelHandle<UserStore>,
}

#[derive(Clone)]
struct Dismiss(u64);

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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Dismiss {}
        enum Reject {}
        enum Accept {}

        let theme = cx.global::<Settings>().theme.clone();
        let theme = &theme.incoming_request_notification;
        let user_id = self.user.id;

        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(self.user.avatar.clone().map(|avatar| {
                        Image::new(avatar)
                            .with_style(theme.header_avatar)
                            .aligned()
                            .left()
                            .boxed()
                    }))
                    .with_child(
                        Label::new(
                            format!("{} added you", self.user.github_login),
                            theme.header_message.text.clone(),
                        )
                        .contained()
                        .with_style(theme.header_message.container)
                        .aligned()
                        .boxed(),
                    )
                    .with_child(
                        MouseEventHandler::new::<Dismiss, _, _>(
                            self.user.id as usize,
                            cx,
                            |_, _| {
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
                            },
                        )
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(move |_, cx| cx.dispatch_action(Dismiss(user_id)))
                        .flex_float()
                        .boxed(),
                    )
                    .constrained()
                    .with_height(theme.header_height)
                    .boxed(),
            )
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
                        Label::new("Decline".to_string(), theme.button.text.clone())
                            .contained()
                            .with_style(theme.button.container)
                            .boxed(),
                    )
                    .with_child(
                        Label::new("Accept".to_string(), theme.button.text.clone())
                            .contained()
                            .with_style(theme.button.container)
                            .boxed(),
                    )
                    .aligned()
                    .right()
                    .boxed(),
            )
            .contained()
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

    fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        self.user_store.update(cx, |store, cx| {
            store
                .dismiss_contact_request(self.user.id, cx)
                .detach_and_log_err(cx);
        });
        cx.emit(Event::Dismiss);
    }
}
