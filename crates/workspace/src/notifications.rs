use crate::{
    MultiWorkspace, SuppressNotification, Toast, Workspace, workspace_error::WorkspaceError,
};
use anyhow::Context as _;
use gpui::{
    AnyEntity, AnyView, App, AppContext as _, AsyncApp, AsyncWindowContext, ClickEvent, Context,
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, PromptLevel, Render, ScrollHandle,
    Task, TextStyleRefinement, UnderlineStyle, WeakEntity,
};
use markdown::{CopyButtonVisibility, Markdown, MarkdownElement, MarkdownStyle};
use parking_lot::Mutex;
use project::project_settings::ProjectSettings;
use settings::Settings;
use theme_settings::ThemeSettings;

use std::ops::Deref;
use std::sync::{Arc, LazyLock};
use std::{any::TypeId, time::Duration};
use ui::{CopyButton, Tooltip, prelude::*};
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
    pub const fn unique<T: 'static>() -> Self {
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

            if let Ok(prompt) =
                AnyEntity::from(notification.clone()).downcast::<LanguageServerPrompt>()
            {
                let is_prompt_without_actions = prompt
                    .read(cx)
                    .request
                    .as_ref()
                    .is_some_and(|request| request.actions.is_empty());

                let dismiss_timeout_ms = ProjectSettings::get_global(cx)
                    .global_lsp_settings
                    .notifications
                    .dismiss_timeout_ms;

                if is_prompt_without_actions {
                    if let Some(dismiss_duration_ms) = dismiss_timeout_ms.filter(|&ms| ms > 0) {
                        let task = cx.spawn({
                            let id = id.clone();
                            async move |this, cx| {
                                cx.background_executor()
                                    .timer(Duration::from_millis(dismiss_duration_ms))
                                    .await;
                                let _ = this.update(cx, |workspace, cx| {
                                    workspace.dismiss_notification(&id, cx);
                                });
                            }
                        });
                        prompt.update(cx, |prompt, _| {
                            prompt.dismiss_task = Some(task);
                        });
                    }
                }
            }
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

    pub fn show_error<E: WorkspaceError + 'static>(&mut self, err: E, cx: &mut Context<Self>) {
        self.show_notification(NotificationId::unique::<E>(), cx, |cx| {
            cx.new(|cx| {
                simple_message_notification::MessageNotification::from_workspace_error(err, cx)
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
            cx.new(|cx| {
                simple_message_notification::MessageNotification::new(toast.message, cx).when_some(
                    toast.on_click,
                    |this, (click_msg, on_click)| {
                        this.primary_message(click_msg)
                            .primary_on_click(move |window, cx| on_click(window, cx))
                    },
                )
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

    pub fn is_notification_suppressed(&self, notification_id: NotificationId) -> bool {
        self.suppressed_notifications.contains(&notification_id)
    }

    pub fn unsuppress(&mut self, notification_id: NotificationId) {
        self.suppressed_notifications.remove(&notification_id);
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
    markdown: Entity<Markdown>,
    dismiss_task: Option<Task<()>>,
}

impl Focusable for LanguageServerPrompt {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Notification for LanguageServerPrompt {}

impl LanguageServerPrompt {
    pub fn new(request: project::LanguageServerPromptRequest, cx: &mut App) -> Self {
        let markdown = cx.new(|cx| Markdown::new(request.message.clone().into(), None, None, cx));

        Self {
            focus_handle: cx.focus_handle(),
            request: Some(request),
            scroll_handle: ScrollHandle::new(),
            markdown,
            dismiss_task: None,
        }
    }

    async fn select_option(this: Entity<Self>, ix: usize, cx: &mut AsyncWindowContext) {
        util::maybe!(async move {
            let potential_future = this.update(cx, |this, _| {
                this.request.take().map(|request| request.respond(ix))
            });

            potential_future
                .context("Response already sent")?
                .await
                .context("Stream already closed")?;

            this.update(cx, |this, cx| {
                this.dismiss_notification(cx);
            });

            anyhow::Ok(())
        })
        .await
        .log_err();
    }

    fn dismiss_notification(&mut self, cx: &mut Context<Self>) {
        self.dismiss_task = None;
        cx.emit(DismissEvent);
    }
}

impl Render for LanguageServerPrompt {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(request) = &self.request else {
            return div().id("language_server_prompt_notification");
        };

        let (icon, color) = match request.level {
            PromptLevel::Info => (IconName::Info, Color::Muted),
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
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(icon).color(color).size(IconSize::Small))
                                    .child(Label::new(request.lsp_name.clone())),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        CopyButton::new(
                                            "copy-description",
                                            request.message.clone(),
                                        )
                                        .tooltip_label("Copy Description"),
                                    )
                                    .child(
                                        IconButton::new(close_id, close_icon)
                                            .tooltip(move |_window, cx| {
                                                if suppress {
                                                    Tooltip::with_meta(
                                                        "Suppress",
                                                        Some(&SuppressNotification),
                                                        "Click to close",
                                                        cx,
                                                    )
                                                } else {
                                                    Tooltip::with_meta(
                                                        "Close",
                                                        Some(&menu::Cancel),
                                                        "Suppress with shift-click",
                                                        cx,
                                                    )
                                                }
                                            })
                                            .on_click(cx.listener(
                                                move |this, _: &ClickEvent, _, cx| {
                                                    if suppress {
                                                        cx.emit(SuppressEvent);
                                                    } else {
                                                        this.dismiss_notification(cx);
                                                    }
                                                },
                                            )),
                                    ),
                            ),
                    )
                    .child(
                        MarkdownElement::new(self.markdown.clone(), markdown_style(window, cx))
                            .text_size(TextSize::Small.rems(cx))
                            .code_block_renderer(markdown::CodeBlockRenderer::Default {
                                copy_button_visibility: CopyButtonVisibility::Hidden,
                                wrap_button_visibility: markdown::WrapButtonVisibility::Hidden,
                                border: false,
                            })
                            .on_url_click(|link, window, cx| {
                                if let Some(workspace) = Workspace::for_window(window, cx) {
                                    workspace.update(cx, |workspace, cx| {
                                        workspace.open_url_or_file(&link, None, window, cx);
                                    });
                                } else {
                                    cx.open_url(&link);
                                }
                            }),
                    )
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

fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let settings = ThemeSettings::get_global(cx);
    let ui_font_family = settings.ui_font.family.clone();
    let ui_font_fallbacks = settings.ui_font.fallbacks.clone();
    let buffer_font_family = settings.buffer_font.family.clone();
    let buffer_font_fallbacks = settings.buffer_font.fallbacks.clone();

    let mut base_text_style = window.text_style();
    base_text_style.refine(&TextStyleRefinement {
        font_family: Some(ui_font_family),
        font_fallbacks: ui_font_fallbacks,
        color: Some(cx.theme().colors().text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style,
        selection_background_color: cx.theme().colors().element_selection_background,
        inline_code: TextStyleRefinement {
            background_color: Some(cx.theme().colors().editor_background.opacity(0.5)),
            font_family: Some(buffer_font_family),
            font_fallbacks: buffer_font_fallbacks,
            ..Default::default()
        },
        link: TextStyleRefinement {
            underline: Some(UnderlineStyle {
                thickness: px(1.),
                color: Some(cx.theme().colors().text_accent),
                wavy: false,
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub mod simple_message_notification {
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use gpui::{
        AnyElement, DismissEvent, EventEmitter, FocusHandle, Focusable, ParentElement, Render,
        ScrollHandle, SharedString, Styled, Task,
    };
    use ui::{CopyButton, Tooltip, WithScrollbar, prelude::*};

    use crate::SuppressNotification;
    use crate::workspace_error::{
        ActionIcon, ErrorAction, ErrorActionHandler, ErrorSeverity, WorkspaceError,
    };

    use super::{Notification, SuppressEvent};

    const FADE_OUT_DURATION: Duration = Duration::from_secs(2);
    const FADE_TO_FULL_OPACITY_DURATION: Duration = Duration::from_millis(200);

    pub(super) struct AutoHideState {
        remaining_dismiss_duration: Duration,
        timer_started: Option<Instant>,
        hovered: bool,
        fade: Option<AutoHideFade>,
        task: Option<Task<()>>,
    }

    enum AutoHideFade {
        FadingOut {
            started_at: Instant,
        },
        FadingIn {
            started_at: Instant,
            start_opacity: f32,
        },
    }

    impl AutoHideState {
        fn new(duration: Duration, cx: &mut Context<MessageNotification>) -> Self {
            let mut this = Self {
                remaining_dismiss_duration: duration,
                timer_started: None,
                hovered: false,
                fade: None,
                task: None,
            };
            this.schedule(cx);
            this
        }

        fn schedule(&mut self, cx: &mut Context<MessageNotification>) {
            if self.task.is_some() || self.hovered {
                return;
            }

            let duration = self.remaining_dismiss_duration;
            self.timer_started = Some(Instant::now());
            self.task = Some(cx.spawn(async move |this, cx| {
                cx.background_executor().timer(duration).await;
                if let Err(error) = this.update(cx, |this, cx| {
                    if let Some(auto_hide) = this.auto_hide.as_mut() {
                        auto_hide.finish_timer();
                        if !auto_hide.hovered {
                            auto_hide.start_fading_out();
                            cx.notify();
                        }
                    }
                }) {
                    log::error!("failed to update auto-hiding notification: {error:?}");
                }
            }));
        }

        fn set_hovered(&mut self, hovered: bool, cx: &mut Context<MessageNotification>) {
            if self.hovered == hovered {
                return;
            }

            self.hovered = hovered;
            if hovered {
                self.remaining_dismiss_duration = self.remaining_dismiss_duration();
                self.timer_started = None;
                self.task.take();

                if matches!(self.fade, Some(AutoHideFade::FadingOut { .. })) {
                    let start_opacity = self.opacity();
                    self.fade = Some(AutoHideFade::FadingIn {
                        started_at: Instant::now(),
                        start_opacity,
                    });
                }
            } else {
                if matches!(self.fade, Some(AutoHideFade::FadingIn { .. })) {
                    self.fade = None;
                }
                self.schedule(cx);
            }
            cx.notify();
        }

        fn refresh_animation(&mut self) -> bool {
            match self.fade {
                Some(AutoHideFade::FadingOut { started_at })
                    if started_at.elapsed() >= FADE_OUT_DURATION =>
                {
                    true
                }
                Some(AutoHideFade::FadingIn { started_at, .. })
                    if started_at.elapsed() >= FADE_TO_FULL_OPACITY_DURATION =>
                {
                    self.fade = None;
                    false
                }
                _ => false,
            }
        }

        fn needs_animation_frame(&self) -> bool {
            self.fade.is_some()
        }

        fn opacity(&self) -> f32 {
            match self.fade {
                Some(AutoHideFade::FadingOut { started_at }) => {
                    1.0 - duration_progress(started_at.elapsed(), FADE_OUT_DURATION)
                }
                Some(AutoHideFade::FadingIn {
                    started_at,
                    start_opacity,
                }) => {
                    let progress =
                        duration_progress(started_at.elapsed(), FADE_TO_FULL_OPACITY_DURATION);
                    start_opacity + (1.0 - start_opacity) * progress
                }
                None => 1.0,
            }
        }

        fn finish_timer(&mut self) {
            self.task.take();
            self.timer_started = None;
            self.remaining_dismiss_duration = Duration::ZERO;
        }

        fn start_fading_out(&mut self) {
            self.fade = Some(AutoHideFade::FadingOut {
                started_at: Instant::now(),
            });
        }

        fn remaining_dismiss_duration(&self) -> Duration {
            self.timer_started
                .map_or(self.remaining_dismiss_duration, |timer_started| {
                    self.remaining_dismiss_duration
                        .saturating_sub(timer_started.elapsed())
                })
        }
    }

    fn duration_progress(elapsed: Duration, duration: Duration) -> f32 {
        if duration.is_zero() {
            1.0
        } else {
            (elapsed.as_secs_f32() / duration.as_secs_f32()).min(1.0)
        }
    }

    #[derive(RegisterComponent)]
    pub struct MessageNotification {
        focus_handle: FocusHandle,
        build_content: Box<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement>,
        button_style: Option<ButtonStyle>,
        content_icon: Option<IconName>,
        content_icon_color: Option<Color>,
        secondary_content: Option<SharedString>,
        copy_text: Option<SharedString>,
        primary_message: Option<SharedString>,
        primary_icon: Option<ActionIcon>,
        primary_icon_color: Option<Color>,
        primary_on_click: Option<Arc<dyn Fn(&mut Window, &mut Context<Self>)>>,
        secondary_message: Option<SharedString>,
        secondary_icon: Option<ActionIcon>,
        secondary_icon_color: Option<Color>,
        secondary_on_click: Option<Arc<dyn Fn(&mut Window, &mut Context<Self>)>>,
        more_info_message: Option<SharedString>,
        more_info_url: Option<Arc<str>>,
        show_close_button: bool,
        show_suppress_button: bool,
        title: Option<SharedString>,
        scroll_handle: ScrollHandle,
        auto_hide: Option<AutoHideState>,
    }

    impl Focusable for MessageNotification {
        fn focus_handle(&self, _: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl EventEmitter<DismissEvent> for MessageNotification {}
    impl EventEmitter<SuppressEvent> for MessageNotification {}

    impl Notification for MessageNotification {}

    impl FluentBuilder for MessageNotification {}

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
                button_style: None,
                content_icon: None,
                content_icon_color: None,
                secondary_content: None,
                copy_text: None,
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
                scroll_handle: ScrollHandle::new(),
                auto_hide: None,
            }
        }

        pub fn button_style(mut self, style: ButtonStyle) -> Self {
            self.button_style = Some(style);
            self
        }

        pub fn primary_message<S>(mut self, message: S) -> Self
        where
            S: Into<SharedString>,
        {
            self.primary_message = Some(message.into());
            self
        }

        /// Show `icon` at the start (left) of the primary action button label.
        pub fn primary_icon(mut self, icon: IconName) -> Self {
            self.primary_icon = Some(ActionIcon::start(icon));
            self
        }

        /// Show `icon` at the end (right) of the primary action button label.
        pub fn primary_end_icon(mut self, icon: IconName) -> Self {
            self.primary_icon = Some(ActionIcon::end(icon));
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

        /// Show `icon` at the start (left) of the secondary action button label.
        pub fn secondary_icon(mut self, icon: IconName) -> Self {
            self.secondary_icon = Some(ActionIcon::start(icon));
            self
        }

        /// Show `icon` at the end (right) of the secondary action button label.
        pub fn secondary_end_icon(mut self, icon: IconName) -> Self {
            self.secondary_icon = Some(ActionIcon::end(icon));
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

        /// Determines whether the given notification ID should be suppressible
        /// Suppressed notifications will not be shown anymor
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

        pub fn content_icon(mut self, icon: IconName, color: Color) -> Self {
            self.content_icon = Some(icon);
            self.content_icon_color = Some(color);
            self
        }

        pub fn secondary_content<S: Into<SharedString>>(mut self, text: S) -> Self {
            self.secondary_content = Some(text.into());
            self
        }

        pub fn copy_text<S: Into<SharedString>>(mut self, text: S) -> Self {
            self.copy_text = Some(text.into());
            self
        }

        fn auto_dismiss(mut self, severity: ErrorSeverity, cx: &mut Context<Self>) -> Self {
            if let Some(delay) = severity.auto_dismiss_delay() {
                self.auto_hide = Some(AutoHideState::new(delay, cx));
            }
            self
        }

        pub fn from_workspace_error<E: WorkspaceError>(error: E, cx: &mut Context<Self>) -> Self {
            let primary_message = error.primary_message();
            let severity = error.severity();
            let primary_action = error.primary_action();
            let secondary_action = error.secondary_action();

            Self::new(primary_message.clone(), cx)
                .content_icon(IconName::Warning, Color::Error)
                .button_style(ButtonStyle::Outlined)
                .copy_text(primary_message)
                .show_suppress_button(false)
                .when_some(error.secondary_message(), |this, text| {
                    this.secondary_content(text)
                })
                .map(|this| {
                    let ErrorAction {
                        label,
                        icon,
                        tooltip: _,
                        handler,
                    } = primary_action;

                    this.primary_message(label)
                        .when_some(icon, |this, icon| match icon.position {
                            IconPosition::Start => this.primary_icon(icon.name),
                            IconPosition::End => this.primary_end_icon(icon.name),
                        })
                        .map(|this| match handler {
                            ErrorActionHandler::Action(action) => {
                                this.primary_on_click(move |window, cx| {
                                    window.dispatch_action(action.boxed_clone(), cx);
                                })
                            }
                            ErrorActionHandler::Dismiss => {
                                this.primary_on_click(move |_, cx| cx.emit(DismissEvent))
                            }
                        })
                })
                .when_some(secondary_action, |this, action| {
                    let ErrorAction {
                        label,
                        icon,
                        tooltip: _,
                        handler,
                    } = action;

                    this.secondary_message(label)
                        .when_some(icon, |this, icon| match icon.position {
                            IconPosition::Start => this.secondary_icon(icon.name),
                            IconPosition::End => this.secondary_end_icon(icon.name),
                        })
                        .map(|this| match handler {
                            ErrorActionHandler::Action(handler) => {
                                this.secondary_on_click(move |window, cx| {
                                    window.dispatch_action(handler.boxed_clone(), cx);
                                })
                            }
                            ErrorActionHandler::Dismiss => {
                                this.secondary_on_click(move |_, cx| cx.emit(DismissEvent))
                            }
                        })
                })
                .auto_dismiss(severity, cx)
        }

        fn on_hover_changed(&mut self, hovering: bool, cx: &mut Context<Self>) {
            if let Some(auto_hide) = self.auto_hide.as_mut() {
                auto_hide.set_hovered(hovering, cx);
            }
        }

        fn opacity(&self) -> f32 {
            self.auto_hide
                .as_ref()
                .map_or(1.0, |auto_hide| auto_hide.opacity())
        }

        fn needs_animation_frame(&self) -> bool {
            self.auto_hide
                .as_ref()
                .is_some_and(|auto_hide| auto_hide.needs_animation_frame())
        }
    }

    impl Render for MessageNotification {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            if self
                .auto_hide
                .as_mut()
                .is_some_and(|auto_hide| auto_hide.refresh_animation())
            {
                cx.emit(DismissEvent);
            }

            if self.needs_animation_frame() {
                window.request_animation_frame();
            }

            let opacity = self.opacity();
            let has_auto_hide = self.auto_hide.is_some();
            let entity = window.current_view();
            let show_suppress_button = self.show_suppress_button;
            let show_close_button = self.show_close_button;
            let suppress = show_suppress_button && window.modifiers().shift;
            let (close_id, close_icon) = if suppress {
                ("suppress", IconName::Minimize)
            } else {
                ("close", IconName::Close)
            };

            let main_content = (self.build_content)(window, cx);
            let line_height = window.line_height();

            let copy_text = self.copy_text.clone();
            let header_actions = h_flex()
                .flex_shrink_0()
                .gap_1()
                .when_some(copy_text, |el, text| {
                    el.child(
                        CopyButton::new("copy-notification-message", text)
                            .tooltip_label("Copy Message"),
                    )
                })
                .when(show_close_button, |el| {
                    el.child(
                        IconButton::new(close_id, close_icon)
                            .tooltip(move |_window, cx| {
                                if suppress {
                                    Tooltip::with_meta(
                                        "Suppress",
                                        Some(&SuppressNotification),
                                        "Click to Close",
                                        cx,
                                    )
                                } else if show_suppress_button {
                                    Tooltip::with_meta(
                                        "Close",
                                        Some(&menu::Cancel),
                                        "Shift-click to Suppress",
                                        cx,
                                    )
                                } else {
                                    Tooltip::for_action("Close", &menu::Cancel, cx)
                                }
                            })
                            .on_click(cx.listener(move |_, _, _, cx| {
                                if suppress {
                                    cx.emit(SuppressEvent);
                                } else {
                                    cx.emit(DismissEvent);
                                }
                            })),
                    )
                });

            let has_suffix = self.primary_message.is_some()
                || self.secondary_message.is_some()
                || self.more_info_message.is_some();

            let suffix = h_flex()
                .gap_1()
                .children(self.primary_message.iter().map(|message| {
                    Button::new(("notification-primary", cx.entity_id()), message.clone())
                        .when_some(self.button_style, |button, style| button.style(style))
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(on_click) = this.primary_on_click.as_ref() {
                                (on_click)(window, cx)
                            };
                            this.dismiss(cx)
                        }))
                        .when_some(self.primary_icon, |button, icon| {
                            let element = Icon::new(icon.name)
                                .size(IconSize::Small)
                                .color(self.primary_icon_color.unwrap_or(Color::Muted));
                            match icon.position {
                                IconPosition::Start => button.start_icon(element),
                                IconPosition::End => button.end_icon(element),
                            }
                        })
                }))
                .children(self.secondary_message.iter().map(|message| {
                    Button::new(("notification-secondary", cx.entity_id()), message.clone())
                        .when_some(self.button_style, |button, style| button.style(style))
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(on_click) = this.secondary_on_click.as_ref() {
                                (on_click)(window, cx)
                            };
                            this.dismiss(cx)
                        }))
                        .when_some(self.secondary_icon, |button, icon| {
                            let element = Icon::new(icon.name)
                                .size(IconSize::Small)
                                .color(self.secondary_icon_color.unwrap_or(Color::Muted));
                            match icon.position {
                                IconPosition::Start => button.start_icon(element),
                                IconPosition::End => button.end_icon(element),
                            }
                        })
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
                                    .end_icon(
                                        Icon::new(IconName::ArrowUpRight)
                                            .size(IconSize::Indicator)
                                            .color(Color::Muted),
                                    )
                                    .on_click(cx.listener(move |_, _, _, cx| {
                                        cx.open_url(&url);
                                    }))
                            }),
                    ),
                );

            // Wrap the icon to vertically align with the first line of the primary
            // message (mirrors `ui::Callout`'s alignment pattern). The body, secondary
            // text and suffix all share a single column to the right of the icon so
            // they line up under one another even when an icon is present.
            let body = h_flex()
                .gap_2()
                .items_start()
                .when_some(
                    self.content_icon.zip(self.content_icon_color),
                    |el, (icon, color)| {
                        el.child(
                            h_flex()
                                .h(line_height)
                                .justify_center()
                                .child(Icon::new(icon).size(IconSize::Small).color(color)),
                        )
                    },
                )
                .child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_2()
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    div()
                                        .child(
                                            div()
                                                .id("message-notification-content")
                                                .max_h(vh(0.6, window))
                                                .overflow_y_scroll()
                                                .track_scroll(&self.scroll_handle.clone())
                                                .child(main_content),
                                        )
                                        .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                                )
                                .when_some(self.secondary_content.clone(), |el, secondary| {
                                    el.child(
                                        Label::new(secondary)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }),
                        )
                        .when(has_suffix, |this| this.child(suffix)),
                );

            div()
                .id("message-notification-wrapper")
                .opacity(opacity)
                .child(
                    v_flex()
                        .id(("notification-frame", entity))
                        .occlude()
                        .when(has_auto_hide, |this| {
                            this.on_hover(cx.listener(|this, hovered: &bool, _window, cx| {
                                this.on_hover_changed(*hovered, cx);
                            }))
                        })
                        .when(show_close_button, |this| {
                            this.on_modifiers_changed(move |_, _, cx| cx.notify(entity))
                        })
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
                                        .flex_1()
                                        .min_w_0()
                                        .gap_0p5()
                                        .when_some(self.title.clone(), |div, title| {
                                            div.child(Label::new(title))
                                        })
                                        .child(div().max_w_96().child(body)),
                                )
                                .child(header_actions),
                        ),
                )
        }
    }

    impl Component for MessageNotification {
        fn scope() -> ComponentScope {
            ComponentScope::Notification
        }

        fn description() -> &'static str {
            "A workspace notification that surfaces a message in a framed container, with an \
            optional title, secondary message, copy button, and primary/secondary action buttons."
        }

        fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
            let normal =
                cx.new(|cx| MessageNotification::new("A regular informational notification.", cx));

            let with_title = cx.new(|cx| {
                MessageNotification::new("Some informational content for the user.", cx)
                    .with_title("Notification Title")
            });

            let with_primary_action = cx.new(|cx| {
                MessageNotification::new("A new version of Zed is available for download.", cx)
                    .with_title("Update Available")
                    .primary_message("Restart Now")
                    .primary_icon(IconName::ArrowCircle)
            });

            let with_end_icon_action = cx.new(|cx| {
                MessageNotification::new("Release notes for this version are available online.", cx)
                    .with_title("What’s New")
                    .primary_message("Read Release Notes")
                    .primary_end_icon(IconName::ArrowUpRight)
            });

            // Mirrors the shape of notifications such as the keymap parse error: a long,
            // multi-line message followed by a primary action button. Useful for catching
            // regressions where the action row overlaps or is clipped by the content above.
            let with_long_content_and_action = cx.new(|cx| {
                let long_message = "Errors in user keymap file. In section with context = \
                    \"Workspace\":\n\
                    • In binding \"ctrl-r\", expected two-element array of [name, input], \
                    found [\"editor::Apply\"].\n\
                    • In binding \"ctrl-shift-r\", action \"editor::Reload\" is not registered.";
                MessageNotification::new(long_message, cx)
                    .primary_message("Open Keymap File")
                    .primary_icon(IconName::Settings)
            });

            struct PreviewError;
            impl WorkspaceError for PreviewError {
                fn primary_message(&self) -> SharedString {
                    "Something went wrong while loading your project.".into()
                }

                fn primary_action(&self) -> ErrorAction {
                    ErrorAction::dismiss()
                }

                fn secondary_message(&self) -> Option<SharedString> {
                    Some("Check your network connection and try again.".into())
                }
                fn severity(&self) -> ErrorSeverity {
                    ErrorSeverity::Error
                }
            }
            let error_state =
                cx.new(|cx| MessageNotification::from_workspace_error(PreviewError, cx));

            let close_only = cx
                .new(|cx| MessageNotification::new("Default header with just a close button.", cx));

            let copy_and_close = cx.new(|cx| {
                let msg: SharedString = "This message can be copied to the clipboard.".into();
                MessageNotification::new(msg.clone(), cx).copy_text(msg)
            });

            let no_close = cx.new(|cx| {
                MessageNotification::new("This notification can't be closed manually.", cx)
                    .show_close_button(false)
            });

            // --- Workspace errors ---
            // These showcase common shapes of [`WorkspaceError`]. They are intentionally
            // [`ErrorSeverity::Critical`] so they never auto-dismiss in the preview, which
            // would otherwise make them disappear mid-inspection.

            struct BasicError;
            impl WorkspaceError for BasicError {
                fn primary_message(&self) -> SharedString {
                    "Failed to save the file.".into()
                }
                fn primary_action(&self) -> ErrorAction {
                    ErrorAction::dismiss()
                }
                fn severity(&self) -> ErrorSeverity {
                    ErrorSeverity::Critical
                }
            }

            struct LanguageServerError;
            impl WorkspaceError for LanguageServerError {
                fn primary_message(&self) -> SharedString {
                    "Error: Prepare rename via rust-analyzer failed: No references found at position"
                        .into()
                }
                fn secondary_message(&self) -> Option<SharedString> {
                    None
                }
                fn primary_action(&self) -> ErrorAction {
                    ErrorAction::dismiss()
                }
                fn severity(&self) -> ErrorSeverity {
                    ErrorSeverity::Critical
                }
            }

            // Mirrors the shape of [`super::super::PortalError`]: a critical error with a
            // documentation link as its primary action.
            struct PortalSetupError;
            impl WorkspaceError for PortalSetupError {
                fn primary_message(&self) -> SharedString {
                    "Linux desktop portal initialization failed.".into()
                }
                fn secondary_message(&self) -> Option<SharedString> {
                    Some("Zed needs an xdg-desktop-portal implementation to open files.".into())
                }
                fn severity(&self) -> ErrorSeverity {
                    ErrorSeverity::Critical
                }
                fn primary_action(&self) -> ErrorAction {
                    ErrorAction::link(
                        "See Docs",
                        "https://zed.dev/docs/linux#i-cant-open-any-files",
                    )
                }
            }

            // Has both a primary action (link) and a secondary action (dismiss), so the
            // preview exercises the full button row.
            struct UpdateRequiredError;
            impl WorkspaceError for UpdateRequiredError {
                fn primary_message(&self) -> SharedString {
                    "An update is required to continue using Zed AI.".into()
                }
                fn severity(&self) -> ErrorSeverity {
                    ErrorSeverity::Critical
                }
                fn primary_action(&self) -> ErrorAction {
                    ErrorAction::link("Update Zed", "https://zed.dev/releases")
                }
                fn secondary_action(&self) -> Option<ErrorAction> {
                    Some(ErrorAction::dismiss())
                }
            }

            let basic_error =
                cx.new(|cx| MessageNotification::from_workspace_error(BasicError, cx));
            let detailed_error =
                cx.new(|cx| MessageNotification::from_workspace_error(LanguageServerError, cx));
            let docs_error =
                cx.new(|cx| MessageNotification::from_workspace_error(PortalSetupError, cx));
            let update_error =
                cx.new(|cx| MessageNotification::from_workspace_error(UpdateRequiredError, cx));

            let container = || div().w(px(440.));

            v_flex()
                .gap_6()
                .p_4()
                .children(vec![
                    example_group_with_title(
                        "States",
                        vec![
                            single_example("Normal", container().child(normal).into_any_element()),
                            single_example(
                                "With Title",
                                container().child(with_title).into_any_element(),
                            ),
                            single_example(
                                "With Primary Action (start icon)",
                                container().child(with_primary_action).into_any_element(),
                            ),
                            single_example(
                                "With Primary Action (end icon)",
                                container().child(with_end_icon_action).into_any_element(),
                            ),
                            single_example(
                                "Long Content + Primary Action",
                                container()
                                    .child(with_long_content_and_action)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Error",
                                container().child(error_state).into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Header Actions (top right)",
                        vec![
                            single_example(
                                "Close Only",
                                container().child(close_only).into_any_element(),
                            ),
                            single_example(
                                "Copy + Close",
                                container().child(copy_and_close).into_any_element(),
                            ),
                            single_example(
                                "No Close",
                                container().child(no_close).into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Workspace Errors",
                        vec![
                            single_example(
                                "Basic",
                                container().child(basic_error).into_any_element(),
                            ),
                            single_example(
                                "With Secondary Message",
                                container().child(detailed_error).into_any_element(),
                            ),
                            single_example(
                                "With Documentation Link",
                                container().child(docs_error).into_any_element(),
                            ),
                            single_example(
                                "With Primary + Secondary Action",
                                container().child(update_error).into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element()
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
            if let Some(multi_workspace) = window.downcast::<MultiWorkspace>() {
                multi_workspace
                    .update(cx, |multi_workspace, _window, cx| {
                        for workspace in multi_workspace.workspaces() {
                            workspace.update(cx, |workspace, cx| {
                                workspace.show_notification_without_handling_dismiss_events(
                                    &id,
                                    cx,
                                    |cx| build_notification(cx),
                                );
                            });
                        }
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
            if let Some(multi_workspace) = window.downcast::<MultiWorkspace>() {
                let id = id.clone();
                multi_workspace
                    .update(cx, |multi_workspace, _window, cx| {
                        for workspace in multi_workspace.workspaces() {
                            workspace.update(cx, |workspace, cx| {
                                workspace.dismiss_notification(&id, cx)
                            });
                        }
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

    fn notify_workspace_async_err(
        self,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncApp,
    ) -> Option<Self::Ok>;

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
                workspace.show_error(format!("Error: {err}"), cx);
                None
            }
        }
    }

    fn notify_workspace_async_err(
        self,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncApp,
    ) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                log::error!("{err:?}");
                let message = format!("Error: {err}");
                workspace
                    .update(cx, |workspace, cx| workspace.show_error(message, cx))
                    .ok();
                None
            }
        }
    }

    fn notify_app_err(self, cx: &mut App) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                let message = format!("Error: {err}");
                log::error!("Showing error notification in app: {message}");
                show_app_notification(workspace_error_notification_id(), cx, {
                    move |cx| {
                        cx.new({
                            let message = message.clone();
                            move |cx| {
                                simple_message_notification::MessageNotification::from_workspace_error(message, cx)
                            }
                        })
                    }
                });

                None
            }
        }
    }
}

pub trait NotifyTaskExt {
    fn detach_and_notify_err(
        self,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    );
}

impl<R, E> NotifyTaskExt for Task<std::result::Result<R, E>>
where
    E: std::fmt::Debug + std::fmt::Display + Sized + 'static,
    R: 'static,
{
    fn detach_and_notify_err(
        self,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        window
            .spawn(cx, async move |mut cx| {
                self.await.notify_workspace_async_err(workspace, &mut cx)
            })
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
                log::error!("{err:#}");
                if let Ok(prompt) = cx.update(|window, cx| {
                    let mut display = format!("{err:#}");
                    if !display.ends_with('\n') {
                        display.push('.');
                    }
                    let detail = f(err, window, cx).unwrap_or(display);
                    window.prompt(PromptLevel::Critical, &msg, Some(&detail), &["OK"], cx)
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

#[cfg(test)]
mod tests {
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::{LanguageServerPromptRequest, Project};

    use crate::tests::init_test;

    use super::*;

    #[gpui::test]
    async fn test_notification_auto_dismiss_with_notifications_from_multiple_language_servers(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let count_notifications = |workspace: &Entity<Workspace>, cx: &mut TestAppContext| {
            workspace.read_with(cx, |workspace, _| workspace.notification_ids().len())
        };

        let show_notification = |workspace: &Entity<Workspace>,
                                 cx: &mut TestAppContext,
                                 lsp_name: &str| {
            workspace.update(cx, |workspace, cx| {
                let request = LanguageServerPromptRequest::test(
                    gpui::PromptLevel::Warning,
                    "Test notification".to_string(),
                    vec![], // Empty actions triggers auto-dismiss
                    lsp_name.to_string(),
                );
                let notification_id = NotificationId::composite::<LanguageServerPrompt>(request.id);
                workspace.show_notification(notification_id, cx, |cx| {
                    cx.new(|cx| LanguageServerPrompt::new(request, cx))
                });
            })
        };

        show_notification(&workspace, cx, "Lsp1");
        assert_eq!(count_notifications(&workspace, cx), 1);

        cx.executor().advance_clock(Duration::from_millis(1000));

        show_notification(&workspace, cx, "Lsp2");
        assert_eq!(count_notifications(&workspace, cx), 2);

        cx.executor().advance_clock(Duration::from_millis(1000));

        show_notification(&workspace, cx, "Lsp3");
        assert_eq!(count_notifications(&workspace, cx), 3);

        cx.executor().advance_clock(Duration::from_millis(3000));
        assert_eq!(count_notifications(&workspace, cx), 2);

        cx.executor().advance_clock(Duration::from_millis(1000));
        assert_eq!(count_notifications(&workspace, cx), 1);

        cx.executor().advance_clock(Duration::from_millis(1000));
        assert_eq!(count_notifications(&workspace, cx), 0);
    }

    #[gpui::test]
    async fn test_notification_auto_dismiss_with_multiple_notifications_from_single_language_server(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let lsp_name = "server1";

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let count_notifications = |workspace: &Entity<Workspace>, cx: &mut TestAppContext| {
            workspace.read_with(cx, |workspace, _| workspace.notification_ids().len())
        };

        let show_notification = |lsp_name: &str,
                                 workspace: &Entity<Workspace>,
                                 cx: &mut TestAppContext| {
            workspace.update(cx, |workspace, cx| {
                let lsp_name = lsp_name.to_string();
                let request = LanguageServerPromptRequest::test(
                    gpui::PromptLevel::Warning,
                    "Test notification".to_string(),
                    vec![], // Empty actions triggers auto-dismiss
                    lsp_name,
                );
                let notification_id = NotificationId::composite::<LanguageServerPrompt>(request.id);

                workspace.show_notification(notification_id, cx, |cx| {
                    cx.new(|cx| LanguageServerPrompt::new(request, cx))
                });
            })
        };

        show_notification(lsp_name, &workspace, cx);
        assert_eq!(count_notifications(&workspace, cx), 1);

        cx.executor().advance_clock(Duration::from_millis(1000));

        show_notification(lsp_name, &workspace, cx);
        assert_eq!(count_notifications(&workspace, cx), 2);

        cx.executor().advance_clock(Duration::from_millis(4000));
        assert_eq!(count_notifications(&workspace, cx), 1);

        cx.executor().advance_clock(Duration::from_millis(1000));
        assert_eq!(count_notifications(&workspace, cx), 0);
    }

    #[gpui::test]
    async fn test_notification_auto_dismiss_turned_off(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| {
            let mut settings = ProjectSettings::get_global(cx).clone();
            settings
                .global_lsp_settings
                .notifications
                .dismiss_timeout_ms = Some(0);
            ProjectSettings::override_global(settings, cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let count_notifications = |workspace: &Entity<Workspace>, cx: &mut TestAppContext| {
            workspace.read_with(cx, |workspace, _| workspace.notification_ids().len())
        };

        workspace.update(cx, |workspace, cx| {
            let request = LanguageServerPromptRequest::test(
                gpui::PromptLevel::Warning,
                "Test notification".to_string(),
                vec![], // Empty actions would trigger auto-dismiss if enabled
                "test_server".to_string(),
            );
            let notification_id = NotificationId::composite::<LanguageServerPrompt>(request.id);
            workspace.show_notification(notification_id, cx, |cx| {
                cx.new(|cx| LanguageServerPrompt::new(request, cx))
            });
        });

        assert_eq!(count_notifications(&workspace, cx), 1);

        // Advance time beyond the default auto-dismiss duration
        cx.executor().advance_clock(Duration::from_millis(10000));
        assert_eq!(count_notifications(&workspace, cx), 1);
    }

    #[gpui::test]
    async fn test_notification_auto_dismiss_with_custom_duration(cx: &mut TestAppContext) {
        init_test(cx);

        let custom_duration_ms: u64 = 2000;
        cx.update(|cx| {
            let mut settings = ProjectSettings::get_global(cx).clone();
            settings
                .global_lsp_settings
                .notifications
                .dismiss_timeout_ms = Some(custom_duration_ms);
            ProjectSettings::override_global(settings, cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let count_notifications = |workspace: &Entity<Workspace>, cx: &mut TestAppContext| {
            workspace.read_with(cx, |workspace, _| workspace.notification_ids().len())
        };

        workspace.update(cx, |workspace, cx| {
            let request = LanguageServerPromptRequest::test(
                gpui::PromptLevel::Warning,
                "Test notification".to_string(),
                vec![], // Empty actions triggers auto-dismiss
                "test_server".to_string(),
            );
            let notification_id = NotificationId::composite::<LanguageServerPrompt>(request.id);
            workspace.show_notification(notification_id, cx, |cx| {
                cx.new(|cx| LanguageServerPrompt::new(request, cx))
            });
        });

        assert_eq!(count_notifications(&workspace, cx), 1);

        // Advance time less than custom duration
        cx.executor()
            .advance_clock(Duration::from_millis(custom_duration_ms - 500));
        assert_eq!(count_notifications(&workspace, cx), 1);

        // Advance time past the custom duration
        cx.executor().advance_clock(Duration::from_millis(1000));
        assert_eq!(count_notifications(&workspace, cx), 0);
    }
}
