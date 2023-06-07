use crate::{Toast, Workspace};
use collections::HashMap;
use gpui::{AnyViewHandle, AppContext, Entity, View, ViewContext, ViewHandle};
use std::{any::TypeId, ops::DerefMut};

pub fn init(cx: &mut AppContext) {
    cx.set_global(NotificationTracker::new());
    simple_message_notification::init(cx);
}

pub trait Notification: View {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool;
}

pub trait NotificationHandle {
    fn id(&self) -> usize;
    fn as_any(&self) -> &AnyViewHandle;
}

impl<T: Notification> NotificationHandle for ViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }
}

impl From<&dyn NotificationHandle> for AnyViewHandle {
    fn from(val: &dyn NotificationHandle) -> Self {
        val.as_any().clone()
    }
}

pub(crate) struct NotificationTracker {
    notifications_sent: HashMap<TypeId, Vec<usize>>,
}

impl std::ops::Deref for NotificationTracker {
    type Target = HashMap<TypeId, Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.notifications_sent
    }
}

impl DerefMut for NotificationTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notifications_sent
    }
}

impl NotificationTracker {
    fn new() -> Self {
        Self {
            notifications_sent: Default::default(),
        }
    }
}

impl Workspace {
    pub fn has_shown_notification_once<V: Notification>(
        &self,
        id: usize,
        cx: &ViewContext<Self>,
    ) -> bool {
        cx.global::<NotificationTracker>()
            .get(&TypeId::of::<V>())
            .map(|ids| ids.contains(&id))
            .unwrap_or(false)
    }

    pub fn show_notification_once<V: Notification>(
        &mut self,
        id: usize,
        cx: &mut ViewContext<Self>,
        build_notification: impl FnOnce(&mut ViewContext<Self>) -> ViewHandle<V>,
    ) {
        if !self.has_shown_notification_once::<V>(id, cx) {
            cx.update_global::<NotificationTracker, _, _>(|tracker, _| {
                let entry = tracker.entry(TypeId::of::<V>()).or_default();
                entry.push(id);
            });

            self.show_notification::<V>(id, cx, build_notification)
        }
    }

    pub fn show_notification<V: Notification>(
        &mut self,
        id: usize,
        cx: &mut ViewContext<Self>,
        build_notification: impl FnOnce(&mut ViewContext<Self>) -> ViewHandle<V>,
    ) {
        let type_id = TypeId::of::<V>();
        if self
            .notifications
            .iter()
            .all(|(existing_type_id, existing_id, _)| {
                (*existing_type_id, *existing_id) != (type_id, id)
            })
        {
            let notification = build_notification(cx);
            cx.subscribe(&notification, move |this, handle, event, cx| {
                if handle.read(cx).should_dismiss_notification_on_event(event) {
                    this.dismiss_notification_internal(type_id, id, cx);
                }
            })
            .detach();
            self.notifications
                .push((type_id, id, Box::new(notification)));
            cx.notify();
        }
    }

    pub fn dismiss_notification<V: Notification>(&mut self, id: usize, cx: &mut ViewContext<Self>) {
        let type_id = TypeId::of::<V>();

        self.dismiss_notification_internal(type_id, id, cx)
    }

    pub fn show_toast(&mut self, toast: Toast, cx: &mut ViewContext<Self>) {
        self.dismiss_notification::<simple_message_notification::MessageNotification>(toast.id, cx);
        self.show_notification(toast.id, cx, |cx| {
            cx.add_view(|_cx| match toast.on_click.as_ref() {
                Some((click_msg, on_click)) => {
                    let on_click = on_click.clone();
                    simple_message_notification::MessageNotification::new(toast.msg.clone())
                        .with_click_message(click_msg.clone())
                        .on_click(move |cx| on_click(cx))
                }
                None => simple_message_notification::MessageNotification::new(toast.msg.clone()),
            })
        })
    }

    pub fn dismiss_toast(&mut self, id: usize, cx: &mut ViewContext<Self>) {
        self.dismiss_notification::<simple_message_notification::MessageNotification>(id, cx);
    }

    fn dismiss_notification_internal(
        &mut self,
        type_id: TypeId,
        id: usize,
        cx: &mut ViewContext<Self>,
    ) {
        self.notifications
            .retain(|(existing_type_id, existing_id, _)| {
                if (*existing_type_id, *existing_id) == (type_id, id) {
                    cx.notify();
                    false
                } else {
                    true
                }
            });
    }
}

pub mod simple_message_notification {
    use super::Notification;
    use crate::Workspace;
    use gpui::{
        actions,
        elements::{Flex, MouseEventHandler, Padding, ParentElement, Svg, Text},
        fonts::TextStyle,
        impl_actions,
        platform::{CursorStyle, MouseButton},
        AnyElement, AppContext, Element, Entity, View, ViewContext,
    };
    use menu::Cancel;
    use serde::Deserialize;
    use std::{borrow::Cow, sync::Arc};

    actions!(message_notifications, [CancelMessageNotification]);

    #[derive(Clone, Default, Deserialize, PartialEq)]
    pub struct OsOpen(pub Cow<'static, str>);

    impl OsOpen {
        pub fn new<I: Into<Cow<'static, str>>>(url: I) -> Self {
            OsOpen(url.into())
        }
    }

    impl_actions!(message_notifications, [OsOpen]);

    pub fn init(cx: &mut AppContext) {
        cx.add_action(MessageNotification::dismiss);
        cx.add_action(
            |_workspace: &mut Workspace, open_action: &OsOpen, cx: &mut ViewContext<Workspace>| {
                cx.platform().open_url(open_action.0.as_ref());
            },
        )
    }

    enum NotificationMessage {
        Text(Cow<'static, str>),
        Element(fn(TextStyle, &AppContext) -> AnyElement<MessageNotification>),
    }

    pub struct MessageNotification {
        message: NotificationMessage,
        on_click: Option<Arc<dyn Fn(&mut ViewContext<Self>)>>,
        click_message: Option<Cow<'static, str>>,
    }

    pub enum MessageNotificationEvent {
        Dismiss,
    }

    impl Entity for MessageNotification {
        type Event = MessageNotificationEvent;
    }

    impl MessageNotification {
        pub fn new<S>(message: S) -> MessageNotification
        where
            S: Into<Cow<'static, str>>,
        {
            Self {
                message: NotificationMessage::Text(message.into()),
                on_click: None,
                click_message: None,
            }
        }

        pub fn new_element(
            message: fn(TextStyle, &AppContext) -> AnyElement<MessageNotification>,
        ) -> MessageNotification {
            Self {
                message: NotificationMessage::Element(message),
                on_click: None,
                click_message: None,
            }
        }

        pub fn with_click_message<S>(mut self, message: S) -> Self
        where
            S: Into<Cow<'static, str>>,
        {
            self.click_message = Some(message.into());
            self
        }

        pub fn on_click<F>(mut self, on_click: F) -> Self
        where
            F: 'static + Fn(&mut ViewContext<Self>),
        {
            self.on_click = Some(Arc::new(on_click));
            self
        }

        pub fn dismiss(&mut self, _: &CancelMessageNotification, cx: &mut ViewContext<Self>) {
            cx.emit(MessageNotificationEvent::Dismiss);
        }
    }

    impl View for MessageNotification {
        fn ui_name() -> &'static str {
            "MessageNotification"
        }

        fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> gpui::AnyElement<Self> {
            let theme = theme::current(cx).clone();
            let theme = &theme.simple_message_notification;

            enum MessageNotificationTag {}

            let click_message = self.click_message.clone();
            let message = match &self.message {
                NotificationMessage::Text(text) => {
                    Text::new(text.to_owned(), theme.message.text.clone()).into_any()
                }
                NotificationMessage::Element(e) => e(theme.message.text.clone(), cx),
            };
            let on_click = self.on_click.clone();
            let has_click_action = on_click.is_some();

            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(
                            message
                                .contained()
                                .with_style(theme.message.container)
                                .aligned()
                                .top()
                                .left()
                                .flex(1., true),
                        )
                        .with_child(
                            MouseEventHandler::<Cancel, _>::new(0, cx, |state, _| {
                                let style = theme.dismiss_button.style_for(state, false);
                                Svg::new("icons/x_mark_8.svg")
                                    .with_color(style.color)
                                    .constrained()
                                    .with_width(style.icon_width)
                                    .aligned()
                                    .contained()
                                    .with_style(style.container)
                                    .constrained()
                                    .with_width(style.button_width)
                                    .with_height(style.button_width)
                            })
                            .with_padding(Padding::uniform(5.))
                            .on_click(MouseButton::Left, move |_, this, cx| {
                                this.dismiss(&Default::default(), cx);
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .aligned()
                            .constrained()
                            .with_height(cx.font_cache().line_height(theme.message.text.font_size))
                            .aligned()
                            .top()
                            .flex_float(),
                        ),
                )
                .with_children({
                    click_message
                        .map(|click_message| {
                            MouseEventHandler::<MessageNotificationTag, _>::new(
                                0,
                                cx,
                                |state, _| {
                                    let style = theme.action_message.style_for(state, false);

                                    Flex::row()
                                        .with_child(
                                            Text::new(click_message, style.text.clone())
                                                .contained()
                                                .with_style(style.container),
                                        )
                                        .contained()
                                },
                            )
                            .on_click(MouseButton::Left, move |_, this, cx| {
                                if let Some(on_click) = on_click.as_ref() {
                                    on_click(cx);
                                    this.dismiss(&Default::default(), cx);
                                }
                            })
                            // Since we're not using a proper overlay, we have to capture these extra events
                            .on_down(MouseButton::Left, |_, _, _| {})
                            .on_up(MouseButton::Left, |_, _, _| {})
                            .with_cursor_style(if has_click_action {
                                CursorStyle::PointingHand
                            } else {
                                CursorStyle::Arrow
                            })
                        })
                        .into_iter()
                })
                .into_any()
        }
    }

    impl Notification for MessageNotification {
        fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool {
            match event {
                MessageNotificationEvent::Dismiss => true,
            }
        }
    }
}

pub trait NotifyResultExt {
    type Ok;

    fn notify_err(
        self,
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Self::Ok>;
}

impl<T, E> NotifyResultExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    fn notify_err(self, workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                workspace.show_notification(0, cx, |cx| {
                    cx.add_view(|_cx| {
                        simple_message_notification::MessageNotification::new(format!(
                            "Error: {:?}",
                            err,
                        ))
                    })
                });

                None
            }
        }
    }
}
