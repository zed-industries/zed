use crate::{SuppressNotification, Toast, Workspace};
use anyhow::Context as _;
use gpui::{
    AnyView, App, AppContext as _, AsyncWindowContext, ClickEvent, ClipboardItem, Context,
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, PromptLevel, Render, ScrollHandle,
    Task, svg,
};
use parking_lot::Mutex;

use std::ops::Deref;
use std::sync::{Arc, LazyLock};
use std::{any::TypeId, time::Duration};
use ui::{Tooltip, prelude::*};
use util::ResultExt;

#[derive(Default)]
pub struct Notifications {
    notifications: Vec<(NotificationId, AnyView)>,
}

impl Deref for Notifications {
    type Target = Vec<(NotificationId, AnyView)>;

    fn deref(&self) -> &Self::Target {
        &self.notifications
    }
}

impl std::ops::DerefMut for Notifications {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notifications
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum NotificationId {
    Unique(TypeId),
    Composite(TypeId, ElementId),
    Named(SharedString),
}

impl NotificationId {
    /// Returns a unique [`NotificationId`] for the given type.
    pub fn unique<T: 'static>() -> Self {
        Self::Unique(TypeId::of::<T>())
    }

    /// Returns a [`NotificationId`] for the given type that is also identified
    /// by the provided ID.
    pub fn composite<T: 'static>(id: impl Into<ElementId>) -> Self {
        Self::Composite(TypeId::of::<T>(), id.into())
    }

    /// Builds a `NotificationId` out of the given string.
    pub fn named(id: SharedString) -> Self {
        Self::Named(id)
    }
}

pub trait Notification:
    EventEmitter<DismissEvent> + EventEmitter<SuppressEvent> + Focusable + Render
{
}

pub struct SuppressEvent;

impl Workspace {
    #[cfg(any(test, feature = "test-support"))]
    pub fn notification_ids(&self) -> Vec<NotificationId> {
        self.notifications
            .iter()
            .map(|(id, _)| id)
            .cloned()
            .collect()
    }

    pub fn show_notification<V: Notification>(
        &mut self,
        id: NotificationId,
        cx: &mut Context<Self>,
        build_notification: impl FnOnce(&mut Context<Self>) -> Entity<V>,
    ) {
        self.show_notification_without_handling_dismiss_events(&id, cx, |cx| {
            let notification = build_notification(cx);
            cx.subscribe(&notification, {
                let id = id.clone();
                move |this, _, _: &DismissEvent, cx| {
                    this.dismiss_notification(&id, cx);
                }
            })
            .detach();
            cx.subscribe(&notification, {
                let id = id.clone();
                move |workspace: &mut Workspace, _, _: &SuppressEvent, cx| {
                    workspace.suppress_notification(&id, cx);
                }
            })
            .detach();
            notification.into()
        });
    }

    /// Shows a notification in this workspace's window. Caller must handle dismiss.
    ///
    /// This exists so that the `build_notification` closures stored for app notifications can
    /// return `AnyView`. Subscribing to events from an `AnyView` is not supported, so instead that
    /// responsibility is pushed to the caller where the `V` type is known.
    pub(crate) fn show_notification_without_handling_dismiss_events(
        &mut self,
        id: &NotificationId,
        cx: &mut Context<Self>,
        build_notification: impl FnOnce(&mut Context<Self>) -> AnyView,
    ) {
        if self.suppressed_notifications.contains(id) {
            return;
        }
        self.dismiss_notification(id, cx);
        self.notifications
            .push((id.clone(), build_notification(cx)));
        cx.notify();
    }

    pub fn show_error<E>(&mut self, err: &E, cx: &mut Context<Self>)
    where
        E: std::fmt::Debug + std::fmt::Display,
    {
        self.show_notification(workspace_error_notification_id(), cx, |cx| {
            cx.new(|cx| ErrorMessagePrompt::new(format!("Error: {err}"), cx))
        });
    }

    pub fn show_portal_error(&mut self, err: String, cx: &mut Context<Self>) {
        struct PortalError;

        self.show_notification(NotificationId::unique::<PortalError>(), cx, |cx| {
            cx.new(|cx| {
                ErrorMessagePrompt::new(err.to_string(), cx).with_link_button(
                    "See docs",
                    "https://zed.dev/docs/linux#i-cant-open-any-files",
                )
            })
        });
    }

    pub fn dismiss_notification(&mut self, id: &NotificationId, cx: &mut Context<Self>) {
        self.notifications.retain(|(existing_id, _)| {
            if existing_id == id {
                cx.notify();
                false
            } else {
                true
            }
        });
    }

    pub fn show_toast(&mut self, toast: Toast, cx: &mut Context<Self>) {
        self.dismiss_notification(&toast.id, cx);
        self.show_notification(toast.id.clone(), cx, |cx| {
            cx.new(|cx| match toast.on_click.as_ref() {
                Some((click_msg, on_click)) => {
                    let on_click = on_click.clone();
                    simple_message_notification::MessageNotification::new(toast.msg.clone(), cx)
                        .primary_message(click_msg.clone())
                        .primary_on_click(move |window, cx| on_click(window, cx))
                }
                None => {
                    simple_message_notification::MessageNotification::new(toast.msg.clone(), cx)
                }
            })
        });
        if toast.autohide {
            cx.spawn(async move |workspace, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(5000))
                    .await;
                workspace
                    .update(cx, |workspace, cx| workspace.dismiss_toast(&toast.id, cx))
                    .ok();
            })
            .detach();
        }
    }

    pub fn dismiss_toast(&mut self, id: &NotificationId, cx: &mut Context<Self>) {
        self.dismiss_notification(id, cx);
    }

    pub fn clear_all_notifications(&mut self, cx: &mut Context<Self>) {
        self.notifications.clear();
        cx.notify();
    }

    /// Hide all notifications matching the given ID
    pub fn suppress_notification(&mut self, id: &NotificationId, cx: &mut Context<Self>) {
        self.dismiss_notification(id, cx);
        self.suppressed_notifications.insert(id.clone());
    }

    pub fn show_initial_notifications(&mut self, cx: &mut Context<Self>) {
        // Allow absence of the global so that tests don't need to initialize it.
        let app_notifications = GLOBAL_APP_NOTIFICATIONS
            .lock()
            .app_notifications
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for (id, build_notification) in app_notifications {
            self.show_notification_without_handling_dismiss_events(&id, cx, |cx| {
                build_notification(cx)
            });
        }
    }
}

pub struct LanguageServerPrompt {
    focus_handle: FocusHandle,
    request: Option<project::LanguageServerPromptRequest>,
    scroll_handle: ScrollHandle,
}

impl Focusable for LanguageServerPrompt {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Notification for LanguageServerPrompt {}

impl LanguageServerPrompt {
    pub fn new(request: project::LanguageServerPromptRequest, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            request: Some(request),
            scroll_handle: ScrollHandle::new(),
        }
    }

    async fn select_option(this: Entity<Self>, ix: usize, cx: &mut AsyncWindowContext) {
        util::maybe!(async move {
            let potential_future = this.update(cx, |this, _| {
                this.request.take().map(|request| request.respond(ix))
            });

            potential_future? // App Closed
                .context("Response already sent")?
                .await
                .context("Stream already closed")?;

            this.update(cx, |_, cx| cx.emit(DismissEvent))?;

            anyhow::Ok(())
        })
        .await
        .log_err();
    }
}

impl Render for LanguageServerPrompt {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(request) = &self.request else {
            return div().id("language_server_prompt_notification");
        };

        let (icon, color) = match request.level {
            PromptLevel::Info => (IconName::Info, Color::Accent),
            PromptLevel::Warning => (IconName::Warning, Color::Warning),
            PromptLevel::Critical => (IconName::XCircle, Color::Error),
        };

        let suppress = window.modifiers().shift;
        let (close_id, close_icon) = if suppress {
            ("suppress", IconName::Minimize)
        } else {
            ("close", IconName::Close)
        };

        div()
            .id("language_server_prompt_notification")
            .group("language_server_prompt_notification")
            .occlude()
            .w_full()
            .max_h(vh(0.8, window))
            .elevation_3(cx)
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .on_modifiers_changed(cx.listener(|_, _, _, cx| cx.notify()))
            .child(
                v_flex()
                    .p_3()
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_start()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(icon).color(color))
                                    .child(Label::new(request.lsp_name.clone())),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        IconButton::new("copy", IconName::Copy)
                                            .on_click({
                                                let message = request.message.clone();
                                                move |_, _, cx| {
                                                    cx.write_to_clipboard(
                                                        ClipboardItem::new_string(message.clone()),
                                                    )
                                                }
                                            })
                                            .tooltip(Tooltip::text("Copy Description")),
                                    )
                                    .child(
                                        IconButton::new(close_id, close_icon)
                                            .tooltip(move |window, cx| {
                                                if suppress {
                                                    Tooltip::for_action(
                                                        "Suppress.\nClose with click.",
                                                        &SuppressNotification,
                                                        window,
                                                        cx,
                                                    )
                                                } else {
                                                    Tooltip::for_action(
                                                        "Close.\nSuppress with shift-click.",
                                                        &menu::Cancel,
                                                        window,
                                                        cx,
                                                    )
                                                }
                                            })
                                            .on_click(cx.listener(
                                                move |_, _: &ClickEvent, _, cx| {
                                                    if suppress {
                                                        cx.emit(SuppressEvent);
                                                    } else {
                                                        cx.emit(DismissEvent);
                                                    }
                                                },
                                            )),
                                    ),
                            ),
                    )
                    .child(Label::new(request.message.to_string()).size(LabelSize::Small))
                    .children(request.actions.iter().enumerate().map(|(ix, action)| {
                        let this_handle = cx.entity();
                        Button::new(ix, action.title.clone())
                            .size(ButtonSize::Large)
                            .on_click(move |_, window, cx| {
                                let this_handle = this_handle.clone();
                                window
                                    .spawn(cx, async move |cx| {
                                        LanguageServerPrompt::select_option(this_handle, ix, cx)
                                            .await
                                    })
                                    .detach()
                            })
                    })),
            )
    }
}

impl EventEmitter<DismissEvent> for LanguageServerPrompt {}
impl EventEmitter<SuppressEvent> for LanguageServerPrompt {}

fn workspace_error_notification_id() -> NotificationId {
    struct WorkspaceErrorNotification;
    NotificationId::unique::<WorkspaceErrorNotification>()
}

#[derive(Debug, Clone)]
pub struct ErrorMessagePrompt {
    message: SharedString,
    focus_handle: gpui::FocusHandle,
    label_and_url_button: Option<(SharedString, SharedString)>,
}

impl ErrorMessagePrompt {
    pub fn new<S>(message: S, cx: &mut App) -> Self
    where
        S: Into<SharedString>,
    {
        Self {
            message: message.into(),
            focus_handle: cx.focus_handle(),
            label_and_url_button: None,
        }
    }

    pub fn with_link_button<S>(mut self, label: S, url: S) -> Self
    where
        S: Into<SharedString>,
    {
        self.label_and_url_button = Some((label.into(), url.into()));
        self
    }
}

impl Render for ErrorMessagePrompt {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("error_message_prompt_notification")
            .occlude()
            .elevation_3(cx)
            .items_start()
            .justify_between()
            .p_2()
            .gap_2()
            .w_full()
            .child(
                v_flex()
                    .w_full()
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .child(
                                svg()
                                    .size(window.text_style().font_size)
                                    .flex_none()
                                    .mr_2()
                                    .mt(px(-2.0))
                                    .map(|icon| {
                                        icon.path(IconName::Warning.path())
                                            .text_color(Color::Error.color(cx))
                                    }),
                            )
                            .child(
                                ui::IconButton::new("close", ui::IconName::Close)
                                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                            ),
                    )
                    .child(
                        div()
                            .id("error_message")
                            .max_w_96()
                            .max_h_40()
                            .overflow_y_scroll()
                            .child(Label::new(self.message.clone()).size(LabelSize::Small)),
                    )
                    .when_some(self.label_and_url_button.clone(), |elm, (label, url)| {
                        elm.child(
                            div().mt_2().child(
                                ui::Button::new("error_message_prompt_notification_button", label)
                                    .on_click(move |_, _, cx| cx.open_url(&url)),
                            ),
                        )
                    }),
            )
    }
}

impl Focusable for ErrorMessagePrompt {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ErrorMessagePrompt {}
impl EventEmitter<SuppressEvent> for ErrorMessagePrompt {}

impl Notification for ErrorMessagePrompt {}

#[derive(IntoElement, RegisterComponent)]
pub struct NotificationFrame {
    title: Option<SharedString>,
    show_suppress_button: bool,
    show_close_button: bool,
    close: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    contents: Option<AnyElement>,
    suffix: Option<AnyElement>,
}

impl NotificationFrame {
    pub fn new() -> Self {
        Self {
            title: None,
            contents: None,
            suffix: None,
            show_suppress_button: true,
            show_close_button: true,
            close: None,
        }
    }

    pub fn with_title(mut self, title: Option<impl Into<SharedString>>) -> Self {
        self.title = title.map(Into::into);
        self
    }

    pub fn with_content(self, content: impl IntoElement) -> Self {
        Self {
            contents: Some(content.into_any_element()),
            ..self
        }
    }

    /// Determines whether the given notification ID should be suppressible
    /// Suppressed motifications will not be shown anymore
    pub fn show_suppress_button(mut self, show: bool) -> Self {
        self.show_suppress_button = show;
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    pub fn on_close(self, on_close: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        Self {
            close: Some(Box::new(on_close)),
            ..self
        }
    }

    pub fn with_suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }
}

impl RenderOnce for NotificationFrame {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let entity = window.current_view();
        let show_suppress_button = self.show_suppress_button;
        let suppress = show_suppress_button && window.modifiers().shift;
        let (close_id, close_icon) = if suppress {
            ("suppress", IconName::Minimize)
        } else {
            ("close", IconName::Close)
        };

        v_flex()
            .occlude()
            .p_3()
            .gap_2()
            .elevation_3(cx)
            .child(
                h_flex()
                    .gap_4()
                    .justify_between()
                    .items_start()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .when_some(self.title.clone(), |div, title| {
                                div.child(Label::new(title))
                            })
                            .child(div().max_w_96().children(self.contents)),
                    )
                    .when(self.show_close_button, |this| {
                        this.on_modifiers_changed(move |_, _, cx| cx.notify(entity))
                            .child(
                                IconButton::new(close_id, close_icon)
                                    .tooltip(move |window, cx| {
                                        if suppress {
                                            Tooltip::for_action(
                                                "Suppress.\nClose with click.",
                                                &SuppressNotification,
                                                window,
                                                cx,
                                            )
                                        } else if show_suppress_button {
                                            Tooltip::for_action(
                                                "Close.\nSuppress with shift-click.",
                                                &menu::Cancel,
                                                window,
                                                cx,
                                            )
                                        } else {
                                            Tooltip::for_action("Close", &menu::Cancel, window, cx)
                                        }
                                    })
                                    .on_click({
                                        let close = self.close.take();
                                        move |_, window, cx| {
                                            if let Some(close) = &close {
                                                close(&suppress, window, cx)
                                            }
                                        }
                                    }),
                            )
                    }),
            )
            .children(self.suffix)
    }
}

impl Component for NotificationFrame {}

pub mod simple_message_notification {
    use std::sync::Arc;

    use gpui::{
        AnyElement, DismissEvent, EventEmitter, FocusHandle, Focusable, ParentElement, Render,
        SharedString, Styled,
    };
    use ui::prelude::*;

    use crate::notifications::NotificationFrame;

    use super::{Notification, SuppressEvent};

    pub struct MessageNotification {
        focus_handle: FocusHandle,
        build_content: Box<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement>,
        primary_message: Option<SharedString>,
        primary_icon: Option<IconName>,
        primary_icon_color: Option<Color>,
        primary_on_click: Option<Arc<dyn Fn(&mut Window, &mut Context<Self>)>>,
        secondary_message: Option<SharedString>,
        secondary_icon: Option<IconName>,
        secondary_icon_color: Option<Color>,
        secondary_on_click: Option<Arc<dyn Fn(&mut Window, &mut Context<Self>)>>,
        more_info_message: Option<SharedString>,
        more_info_url: Option<Arc<str>>,
        show_close_button: bool,
        show_suppress_button: bool,
        title: Option<SharedString>,
    }

    impl Focusable for MessageNotification {
        fn focus_handle(&self, _: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl EventEmitter<DismissEvent> for MessageNotification {}
    impl EventEmitter<SuppressEvent> for MessageNotification {}

    impl Notification for MessageNotification {}

    impl MessageNotification {
        pub fn new<S>(message: S, cx: &mut App) -> MessageNotification
        where
            S: Into<SharedString>,
        {
            let message = message.into();
            Self::new_from_builder(cx, move |_, _| {
                Label::new(message.clone()).into_any_element()
            })
        }

        pub fn new_from_builder<F>(cx: &mut App, content: F) -> MessageNotification
        where
            F: 'static + Fn(&mut Window, &mut Context<Self>) -> AnyElement,
        {
            Self {
                build_content: Box::new(content),
                primary_message: None,
                primary_icon: None,
                primary_icon_color: None,
                primary_on_click: None,
                secondary_message: None,
                secondary_icon: None,
                secondary_icon_color: None,
                secondary_on_click: None,
                more_info_message: None,
                more_info_url: None,
                show_close_button: true,
                show_suppress_button: true,
                title: None,
                focus_handle: cx.focus_handle(),
            }
        }

        pub fn primary_message<S>(mut self, message: S) -> Self
        where
            S: Into<SharedString>,
        {
            self.primary_message = Some(message.into());
            self
        }

        pub fn primary_icon(mut self, icon: IconName) -> Self {
            self.primary_icon = Some(icon);
            self
        }

        pub fn primary_icon_color(mut self, color: Color) -> Self {
            self.primary_icon_color = Some(color);
            self
        }

        pub fn primary_on_click<F>(mut self, on_click: F) -> Self
        where
            F: 'static + Fn(&mut Window, &mut Context<Self>),
        {
            self.primary_on_click = Some(Arc::new(on_click));
            self
        }

        pub fn primary_on_click_arc<F>(mut self, on_click: Arc<F>) -> Self
        where
            F: 'static + Fn(&mut Window, &mut Context<Self>),
        {
            self.primary_on_click = Some(on_click);
            self
        }

        pub fn secondary_message<S>(mut self, message: S) -> Self
        where
            S: Into<SharedString>,
        {
            self.secondary_message = Some(message.into());
            self
        }

        pub fn secondary_icon(mut self, icon: IconName) -> Self {
            self.secondary_icon = Some(icon);
            self
        }

        pub fn secondary_icon_color(mut self, color: Color) -> Self {
            self.secondary_icon_color = Some(color);
            self
        }

        pub fn secondary_on_click<F>(mut self, on_click: F) -> Self
        where
            F: 'static + Fn(&mut Window, &mut Context<Self>),
        {
            self.secondary_on_click = Some(Arc::new(on_click));
            self
        }

        pub fn secondary_on_click_arc<F>(mut self, on_click: Arc<F>) -> Self
        where
            F: 'static + Fn(&mut Window, &mut Context<Self>),
        {
            self.secondary_on_click = Some(on_click);
            self
        }

        pub fn more_info_message<S>(mut self, message: S) -> Self
        where
            S: Into<SharedString>,
        {
            self.more_info_message = Some(message.into());
            self
        }

        pub fn more_info_url<S>(mut self, url: S) -> Self
        where
            S: Into<Arc<str>>,
        {
            self.more_info_url = Some(url.into());
            self
        }

        pub fn dismiss(&mut self, cx: &mut Context<Self>) {
            cx.emit(DismissEvent);
        }

        pub fn show_close_button(mut self, show: bool) -> Self {
            self.show_close_button = show;
            self
        }

        /// Determines whether the given notification ID should be supressable
        /// Suppressed motifications will not be shown anymor
        pub fn show_suppress_button(mut self, show: bool) -> Self {
            self.show_suppress_button = show;
            self
        }

        pub fn with_title<S>(mut self, title: S) -> Self
        where
            S: Into<SharedString>,
        {
            self.title = Some(title.into());
            self
        }
    }

    impl Render for MessageNotification {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            NotificationFrame::new()
                .with_title(self.title.clone())
                .with_content((self.build_content)(window, cx))
                .show_close_button(self.show_close_button)
                .show_suppress_button(self.show_suppress_button)
                .on_close(cx.listener(|_, suppress, _, cx| {
                    if *suppress {
                        cx.emit(SuppressEvent);
                    } else {
                        cx.emit(DismissEvent);
                    }
                }))
                .with_suffix(
                    h_flex()
                        .gap_1()
                        .children(self.primary_message.iter().map(|message| {
                            let mut button = Button::new(message.clone(), message.clone())
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    if let Some(on_click) = this.primary_on_click.as_ref() {
                                        (on_click)(window, cx)
                                    };
                                    this.dismiss(cx)
                                }));

                            if let Some(icon) = self.primary_icon {
                                button = button
                                    .icon(icon)
                                    .icon_color(self.primary_icon_color.unwrap_or(Color::Muted))
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small);
                            }

                            button
                        }))
                        .children(self.secondary_message.iter().map(|message| {
                            let mut button = Button::new(message.clone(), message.clone())
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    if let Some(on_click) = this.secondary_on_click.as_ref() {
                                        (on_click)(window, cx)
                                    };
                                    this.dismiss(cx)
                                }));

                            if let Some(icon) = self.secondary_icon {
                                button = button
                                    .icon(icon)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
                                    .icon_color(self.secondary_icon_color.unwrap_or(Color::Muted));
                            }

                            button
                        }))
                        .child(
                            h_flex().w_full().justify_end().children(
                                self.more_info_message
                                    .iter()
                                    .zip(self.more_info_url.iter())
                                    .map(|(message, url)| {
                                        let url = url.clone();
                                        Button::new(message.clone(), message.clone())
                                            .label_size(LabelSize::Small)
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::Indicator)
                                            .icon_color(Color::Muted)
                                            .on_click(cx.listener(move |_, _, _, cx| {
                                                cx.open_url(&url);
                                            }))
                                    }),
                            ),
                        ),
                )
        }
    }
}

static GLOBAL_APP_NOTIFICATIONS: LazyLock<Mutex<AppNotifications>> = LazyLock::new(|| {
    Mutex::new(AppNotifications {
        app_notifications: Vec::new(),
    })
});

/// Stores app notifications so that they can be shown in new workspaces.
struct AppNotifications {
    app_notifications: Vec<(
        NotificationId,
        Arc<dyn Fn(&mut Context<Workspace>) -> AnyView + Send + Sync>,
    )>,
}

impl AppNotifications {
    pub fn insert(
        &mut self,
        id: NotificationId,
        build_notification: Arc<dyn Fn(&mut Context<Workspace>) -> AnyView + Send + Sync>,
    ) {
        self.remove(&id);
        self.app_notifications.push((id, build_notification))
    }

    pub fn remove(&mut self, id: &NotificationId) {
        self.app_notifications
            .retain(|(existing_id, _)| existing_id != id);
    }
}

/// Shows a notification in all workspaces. New workspaces will also receive the notification - this
/// is particularly to handle notifications that occur on initialization before any workspaces
/// exist. If the notification is dismissed within any workspace, it will be removed from all.
pub fn show_app_notification<V: Notification + 'static>(
    id: NotificationId,
    cx: &mut App,
    build_notification: impl Fn(&mut Context<Workspace>) -> Entity<V> + 'static + Send + Sync,
) {
    // Defer notification creation so that windows on the stack can be returned to GPUI
    cx.defer(move |cx| {
        // Handle dismiss events by removing the notification from all workspaces.
        let build_notification: Arc<dyn Fn(&mut Context<Workspace>) -> AnyView + Send + Sync> =
            Arc::new({
                let id = id.clone();
                move |cx| {
                    let notification = build_notification(cx);
                    cx.subscribe(&notification, {
                        let id = id.clone();
                        move |_, _, _: &DismissEvent, cx| {
                            dismiss_app_notification(&id, cx);
                        }
                    })
                    .detach();
                    cx.subscribe(&notification, {
                        let id = id.clone();
                        move |workspace: &mut Workspace, _, _: &SuppressEvent, cx| {
                            workspace.suppress_notification(&id, cx);
                        }
                    })
                    .detach();
                    notification.into()
                }
            });

        // Store the notification so that new workspaces also receive it.
        GLOBAL_APP_NOTIFICATIONS
            .lock()
            .insert(id.clone(), build_notification.clone());

        for window in cx.windows() {
            if let Some(workspace_window) = window.downcast::<Workspace>() {
                workspace_window
                    .update(cx, |workspace, _window, cx| {
                        workspace.show_notification_without_handling_dismiss_events(
                            &id,
                            cx,
                            |cx| build_notification(cx),
                        );
                    })
                    .ok(); // Doesn't matter if the windows are dropped
            }
        }
    });
}

pub fn dismiss_app_notification(id: &NotificationId, cx: &mut App) {
    let id = id.clone();
    // Defer notification dismissal so that windows on the stack can be returned to GPUI
    cx.defer(move |cx| {
        GLOBAL_APP_NOTIFICATIONS.lock().remove(&id);
        for window in cx.windows() {
            if let Some(workspace_window) = window.downcast::<Workspace>() {
                let id = id.clone();
                workspace_window
                    .update(cx, |workspace, _window, cx| {
                        workspace.dismiss_notification(&id, cx)
                    })
                    .ok();
            }
        }
    });
}

pub trait NotifyResultExt {
    type Ok;

    fn notify_err(self, workspace: &mut Workspace, cx: &mut Context<Workspace>)
    -> Option<Self::Ok>;

    fn notify_async_err(self, cx: &mut AsyncWindowContext) -> Option<Self::Ok>;

    /// Notifies the active workspace if there is one, otherwise notifies all workspaces.
    fn notify_app_err(self, cx: &mut App) -> Option<Self::Ok>;
}

impl<T, E> NotifyResultExt for std::result::Result<T, E>
where
    E: std::fmt::Debug + std::fmt::Display,
{
    type Ok = T;

    fn notify_err(self, workspace: &mut Workspace, cx: &mut Context<Workspace>) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                log::error!("Showing error notification in workspace: {err:?}");
                workspace.show_error(&err, cx);
                None
            }
        }
    }

    fn notify_async_err(self, cx: &mut AsyncWindowContext) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                log::error!("{err:?}");
                cx.update_root(|view, _, cx| {
                    if let Ok(workspace) = view.downcast::<Workspace>() {
                        workspace.update(cx, |workspace, cx| workspace.show_error(&err, cx))
                    }
                })
                .ok();
                None
            }
        }
    }

    fn notify_app_err(self, cx: &mut App) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                let message: SharedString = format!("Error: {err}").into();
                log::error!("Showing error notification in app: {message}");
                show_app_notification(workspace_error_notification_id(), cx, {
                    move |cx| {
                        cx.new({
                            let message = message.clone();
                            move |cx| ErrorMessagePrompt::new(message, cx)
                        })
                    }
                });

                None
            }
        }
    }
}

pub trait NotifyTaskExt {
    fn detach_and_notify_err(self, window: &mut Window, cx: &mut App);
}

impl<R, E> NotifyTaskExt for Task<std::result::Result<R, E>>
where
    E: std::fmt::Debug + std::fmt::Display + Sized + 'static,
    R: 'static,
{
    fn detach_and_notify_err(self, window: &mut Window, cx: &mut App) {
        window
            .spawn(cx, async move |cx| self.await.notify_async_err(cx))
            .detach();
    }
}

pub trait DetachAndPromptErr<R> {
    fn prompt_err(
        self,
        msg: &str,
        window: &Window,
        cx: &App,
        f: impl FnOnce(&anyhow::Error, &mut Window, &mut App) -> Option<String> + 'static,
    ) -> Task<Option<R>>;

    fn detach_and_prompt_err(
        self,
        msg: &str,
        window: &Window,
        cx: &App,
        f: impl FnOnce(&anyhow::Error, &mut Window, &mut App) -> Option<String> + 'static,
    );
}

impl<R> DetachAndPromptErr<R> for Task<anyhow::Result<R>>
where
    R: 'static,
{
    fn prompt_err(
        self,
        msg: &str,
        window: &Window,
        cx: &App,
        f: impl FnOnce(&anyhow::Error, &mut Window, &mut App) -> Option<String> + 'static,
    ) -> Task<Option<R>> {
        let msg = msg.to_owned();
        window.spawn(cx, async move |cx| {
            let result = self.await;
            if let Err(err) = result.as_ref() {
                log::error!("{err:?}");
                if let Ok(prompt) = cx.update(|window, cx| {
                    let detail =
                        f(err, window, cx).unwrap_or_else(|| format!("{err}. Please try again."));
                    window.prompt(PromptLevel::Critical, &msg, Some(&detail), &["Ok"], cx)
                }) {
                    prompt.await.ok();
                }
                return None;
            }
            Some(result.unwrap())
        })
    }

    fn detach_and_prompt_err(
        self,
        msg: &str,
        window: &Window,
        cx: &App,
        f: impl FnOnce(&anyhow::Error, &mut Window, &mut App) -> Option<String> + 'static,
    ) {
        self.prompt_err(msg, window, cx, f).detach();
    }
}
