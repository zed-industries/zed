use crate::{Toast, Workspace};
use collections::HashMap;
use gpui::{
    AnyView, AppContext, AsyncWindowContext, DismissEvent, Entity, EntityId, EventEmitter, Render,
    View, ViewContext, VisualContext,
};
use std::{any::TypeId, ops::DerefMut};

pub fn init(cx: &mut AppContext) {
    cx.set_global(NotificationTracker::new());
    // todo!()
    // simple_message_notification::init(cx);
}

pub trait Notification: EventEmitter<DismissEvent> + Render {}

impl<V: EventEmitter<DismissEvent> + Render> Notification for V {}

pub trait NotificationHandle: Send {
    fn id(&self) -> EntityId;
    fn to_any(&self) -> AnyView;
}

impl<T: Notification> NotificationHandle for View<T> {
    fn id(&self) -> EntityId {
        self.entity_id()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }
}

impl From<&dyn NotificationHandle> for AnyView {
    fn from(val: &dyn NotificationHandle) -> Self {
        val.to_any()
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
        build_notification: impl FnOnce(&mut ViewContext<Self>) -> View<V>,
    ) {
        if !self.has_shown_notification_once::<V>(id, cx) {
            let tracker = cx.global_mut::<NotificationTracker>();
            let entry = tracker.entry(TypeId::of::<V>()).or_default();
            entry.push(id);
            self.show_notification::<V>(id, cx, build_notification)
        }
    }

    pub fn show_notification<V: Notification>(
        &mut self,
        id: usize,
        cx: &mut ViewContext<Self>,
        build_notification: impl FnOnce(&mut ViewContext<Self>) -> View<V>,
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
            cx.subscribe(
                &notification,
                move |this, handle, event: &DismissEvent, cx| {
                    this.dismiss_notification_internal(type_id, id, cx);
                },
            )
            .detach();
            self.notifications
                .push((type_id, id, Box::new(notification)));
            cx.notify();
        }
    }

    pub fn show_error<E>(&mut self, err: &E, cx: &mut ViewContext<Self>)
    where
        E: std::fmt::Debug,
    {
        self.show_notification(0, cx, |cx| {
            cx.build_view(|_cx| {
                simple_message_notification::MessageNotification::new(format!("Error: {err:?}"))
            })
        });
    }

    pub fn dismiss_notification<V: Notification>(&mut self, id: usize, cx: &mut ViewContext<Self>) {
        let type_id = TypeId::of::<V>();

        self.dismiss_notification_internal(type_id, id, cx)
    }

    pub fn show_toast(&mut self, toast: Toast, cx: &mut ViewContext<Self>) {
        todo!()
        // self.dismiss_notification::<simple_message_notification::MessageNotification>(toast.id, cx);
        // self.show_notification(toast.id, cx, |cx| {
        //     cx.add_view(|_cx| match toast.on_click.as_ref() {
        //         Some((click_msg, on_click)) => {
        //             let on_click = on_click.clone();
        //             simple_message_notification::MessageNotification::new(toast.msg.clone())
        //                 .with_click_message(click_msg.clone())
        //                 .on_click(move |cx| on_click(cx))
        //         }
        //         None => simple_message_notification::MessageNotification::new(toast.msg.clone()),
        //     })
        // })
    }

    pub fn dismiss_toast(&mut self, id: usize, cx: &mut ViewContext<Self>) {
        todo!()
        // self.dismiss_notification::<simple_message_notification::MessageNotification>(id, cx);
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
    use gpui::{
        div, AnyElement, AppContext, DismissEvent, Div, EventEmitter, InteractiveElement,
        ParentElement, Render, SharedString, StatefulInteractiveElement, Styled, TextStyle,
        ViewContext,
    };
    use serde::Deserialize;
    use std::{borrow::Cow, sync::Arc};
    use ui::{h_stack, v_stack, Button, Icon, IconElement, Label, StyledExt};

    #[derive(Clone, Default, Deserialize, PartialEq)]
    pub struct OsOpen(pub Cow<'static, str>);

    impl OsOpen {
        pub fn new<I: Into<Cow<'static, str>>>(url: I) -> Self {
            OsOpen(url.into())
        }
    }

    // todo!()
    //     impl_actions!(message_notifications, [OsOpen]);
    //
    // todo!()
    //     pub fn init(cx: &mut AppContext) {
    //         cx.add_action(MessageNotification::dismiss);
    //         cx.add_action(
    //             |_workspace: &mut Workspace, open_action: &OsOpen, cx: &mut ViewContext<Workspace>| {
    //                 cx.platform().open_url(open_action.0.as_ref());
    //             },
    //         )
    //     }

    enum NotificationMessage {
        Text(SharedString),
        Element(fn(TextStyle, &AppContext) -> AnyElement),
    }

    pub struct MessageNotification {
        message: NotificationMessage,
        on_click: Option<Arc<dyn Fn(&mut ViewContext<Self>) + Send + Sync>>,
        click_message: Option<SharedString>,
    }

    impl EventEmitter<DismissEvent> for MessageNotification {}

    impl MessageNotification {
        pub fn new<S>(message: S) -> MessageNotification
        where
            S: Into<SharedString>,
        {
            Self {
                message: NotificationMessage::Text(message.into()),
                on_click: None,
                click_message: None,
            }
        }

        // not needed I think (only for the "new panel" toast, which is outdated now)
        // pub fn new_element(
        //     message: fn(TextStyle, &AppContext) -> AnyElement,
        // ) -> MessageNotification {
        //     Self {
        //         message: NotificationMessage::Element(message),
        //         on_click: None,
        //         click_message: None,
        //     }
        // }

        pub fn with_click_message<S>(mut self, message: S) -> Self
        where
            S: Into<SharedString>,
        {
            self.click_message = Some(message.into());
            self
        }

        pub fn on_click<F>(mut self, on_click: F) -> Self
        where
            F: 'static + Send + Sync + Fn(&mut ViewContext<Self>),
        {
            self.on_click = Some(Arc::new(on_click));
            self
        }

        pub fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
            cx.emit(DismissEvent);
        }
    }

    impl Render for MessageNotification {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            v_stack()
                .elevation_3(cx)
                .p_4()
                .child(
                    h_stack()
                        .justify_between()
                        .child(div().max_w_80().child(match &self.message {
                            NotificationMessage::Text(text) => Label::new(text.clone()),
                            NotificationMessage::Element(element) => {
                                todo!()
                            }
                        }))
                        .child(
                            div()
                                .id("cancel")
                                .child(IconElement::new(Icon::Close))
                                .cursor_pointer()
                                .on_click(cx.listener(|this, event, cx| this.dismiss(cx))),
                        ),
                )
                .children(self.click_message.iter().map(|message| {
                    Button::new(message.clone()).on_click(cx.listener(|this, _, cx| {
                        if let Some(on_click) = this.on_click.as_ref() {
                            (on_click)(cx)
                        };
                        this.dismiss(cx)
                    }))
                }))
        }
    }
    // todo!()
    //     impl View for MessageNotification {
    //         fn ui_name() -> &'static str {
    //             "MessageNotification"
    //         }

    //         fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> gpui::AnyElement<Self> {
    //             let theme = theme2::current(cx).clone();
    //             let theme = &theme.simple_message_notification;

    //             enum MessageNotificationTag {}

    //             let click_message = self.click_message.clone();
    //             let message = match &self.message {
    //                 NotificationMessage::Text(text) => {
    //                     Text::new(text.to_owned(), theme.message.text.clone()).into_any()
    //                 }
    //                 NotificationMessage::Element(e) => e(theme.message.text.clone(), cx),
    //             };
    //             let on_click = self.on_click.clone();
    //             let has_click_action = on_click.is_some();

    //             Flex::column()
    //                 .with_child(
    //                     Flex::row()
    //                         .with_child(
    //                             message
    //                                 .contained()
    //                                 .with_style(theme.message.container)
    //                                 .aligned()
    //                                 .top()
    //                                 .left()
    //                                 .flex(1., true),
    //                         )
    //                         .with_child(
    //                             MouseEventHandler::new::<Cancel, _>(0, cx, |state, _| {
    //                                 let style = theme.dismiss_button.style_for(state);
    //                                 Svg::new("icons/x.svg")
    //                                     .with_color(style.color)
    //                                     .constrained()
    //                                     .with_width(style.icon_width)
    //                                     .aligned()
    //                                     .contained()
    //                                     .with_style(style.container)
    //                                     .constrained()
    //                                     .with_width(style.button_width)
    //                                     .with_height(style.button_width)
    //                             })
    //                             .with_padding(Padding::uniform(5.))
    //                             .on_click(MouseButton::Left, move |_, this, cx| {
    //                                 this.dismiss(&Default::default(), cx);
    //                             })
    //                             .with_cursor_style(CursorStyle::PointingHand)
    //                             .aligned()
    //                             .constrained()
    //                             .with_height(cx.font_cache().line_height(theme.message.text.font_size))
    //                             .aligned()
    //                             .top()
    //                             .flex_float(),
    //                         ),
    //                 )
    //                 .with_children({
    //                     click_message
    //                         .map(|click_message| {
    //                             MouseEventHandler::new::<MessageNotificationTag, _>(
    //                                 0,
    //                                 cx,
    //                                 |state, _| {
    //                                     let style = theme.action_message.style_for(state);

    //                                     Flex::row()
    //                                         .with_child(
    //                                             Text::new(click_message, style.text.clone())
    //                                                 .contained()
    //                                                 .with_style(style.container),
    //                                         )
    //                                         .contained()
    //                                 },
    //                             )
    //                             .on_click(MouseButton::Left, move |_, this, cx| {
    //                                 if let Some(on_click) = on_click.as_ref() {
    //                                     on_click(cx);
    //                                     this.dismiss(&Default::default(), cx);
    //                                 }
    //                             })
    //                             // Since we're not using a proper overlay, we have to capture these extra events
    //                             .on_down(MouseButton::Left, |_, _, _| {})
    //                             .on_up(MouseButton::Left, |_, _, _| {})
    //                             .with_cursor_style(if has_click_action {
    //                                 CursorStyle::PointingHand
    //                             } else {
    //                                 CursorStyle::Arrow
    //                             })
    //                         })
    //                         .into_iter()
    //                 })
    //                 .into_any()
    //         }
    //     }
}

pub trait NotifyResultExt {
    type Ok;

    fn notify_err(
        self,
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Self::Ok>;

    fn notify_async_err(self, cx: &mut AsyncWindowContext) -> Option<Self::Ok>;
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
                log::error!("TODO {err:?}");
                workspace.show_error(&err, cx);
                None
            }
        }
    }

    fn notify_async_err(self, cx: &mut AsyncWindowContext) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                log::error!("TODO {err:?}");
                cx.update(|view, cx| {
                    if let Ok(workspace) = view.downcast::<Workspace>() {
                        workspace.update(cx, |workspace, cx| workspace.show_error(&err, cx))
                    }
                })
                .ok();
                None
            }
        }
    }
}
