use call::{room, ActiveCall};
use client::User;
use collections::HashMap;
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    platform::{CursorStyle, MouseButton, WindowBounds, WindowKind, WindowOptions},
    AppContext, Entity, View, ViewContext,
};
use std::sync::{Arc, Weak};
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    let app_state = Arc::downgrade(app_state);
    let active_call = ActiveCall::global(cx);
    let mut notification_windows = HashMap::default();
    cx.subscribe(&active_call, move |_, event, cx| match event {
        room::Event::RemoteProjectShared {
            owner,
            project_id,
            worktree_root_names,
        } => {
            const PADDING: f32 = 16.;
            let theme = &theme::current(cx).project_shared_notification;
            let window_size = vec2f(theme.window_width, theme.window_height);

            for screen in cx.platform().screens() {
                let screen_bounds = screen.bounds();
                let window = cx.add_window(
                    WindowOptions {
                        bounds: WindowBounds::Fixed(RectF::new(
                            screen_bounds.upper_right() - vec2f(PADDING + window_size.x(), PADDING),
                            window_size,
                        )),
                        titlebar: None,
                        center: false,
                        focus: false,
                        show: true,
                        kind: WindowKind::PopUp,
                        is_movable: false,
                        screen: Some(screen),
                    },
                    |_| {
                        ProjectSharedNotification::new(
                            owner.clone(),
                            *project_id,
                            worktree_root_names.clone(),
                            app_state.clone(),
                        )
                    },
                );
                notification_windows
                    .entry(*project_id)
                    .or_insert(Vec::new())
                    .push(window);
            }
        }
        room::Event::RemoteProjectUnshared { project_id } => {
            if let Some(windows) = notification_windows.remove(&project_id) {
                for window in windows {
                    window.remove(cx);
                }
            }
        }
        room::Event::Left => {
            for (_, windows) in notification_windows.drain() {
                for window in windows {
                    window.remove(cx);
                }
            }
        }
        _ => {}
    })
    .detach();
}

pub struct ProjectSharedNotification {
    project_id: u64,
    worktree_root_names: Vec<String>,
    owner: Arc<User>,
    app_state: Weak<AppState>,
}

impl ProjectSharedNotification {
    fn new(
        owner: Arc<User>,
        project_id: u64,
        worktree_root_names: Vec<String>,
        app_state: Weak<AppState>,
    ) -> Self {
        Self {
            project_id,
            worktree_root_names,
            owner,
            app_state,
        }
    }

    fn join(&mut self, cx: &mut ViewContext<Self>) {
        cx.remove_window();
        if let Some(app_state) = self.app_state.upgrade() {
            workspace::join_remote_project(self.project_id, self.owner.id, app_state, cx)
                .detach_and_log_err(cx);
        }
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.remove_window();
    }

    fn render_owner(&self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &theme::current(cx).project_shared_notification;
        Flex::row()
            .with_children(self.owner.avatar.clone().map(|avatar| {
                Image::from_data(avatar)
                    .with_style(theme.owner_avatar)
                    .aligned()
            }))
            .with_child(
                Flex::column()
                    .with_child(
                        Label::new(
                            self.owner.github_login.clone(),
                            theme.owner_username.text.clone(),
                        )
                        .contained()
                        .with_style(theme.owner_username.container),
                    )
                    .with_child(
                        Label::new(
                            format!(
                                "is sharing a project in Zed{}",
                                if self.worktree_root_names.is_empty() {
                                    ""
                                } else {
                                    ":"
                                }
                            ),
                            theme.message.text.clone(),
                        )
                        .contained()
                        .with_style(theme.message.container),
                    )
                    .with_children(if self.worktree_root_names.is_empty() {
                        None
                    } else {
                        Some(
                            Label::new(
                                self.worktree_root_names.join(", "),
                                theme.worktree_roots.text.clone(),
                            )
                            .contained()
                            .with_style(theme.worktree_roots.container),
                        )
                    })
                    .contained()
                    .with_style(theme.owner_metadata)
                    .aligned(),
            )
            .contained()
            .with_style(theme.owner_container)
            .flex(1., true)
            .into_any()
    }

    fn render_buttons(&self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum Open {}
        enum Dismiss {}

        let theme = theme::current(cx);
        Flex::column()
            .with_child(
                MouseEventHandler::new::<Open, _>(0, cx, |_, _| {
                    let theme = &theme.project_shared_notification;
                    Label::new("Open", theme.open_button.text.clone())
                        .aligned()
                        .contained()
                        .with_style(theme.open_button.container)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| this.join(cx))
                .flex(1., true),
            )
            .with_child(
                MouseEventHandler::new::<Dismiss, _>(0, cx, |_, _| {
                    let theme = &theme.project_shared_notification;
                    Label::new("Dismiss", theme.dismiss_button.text.clone())
                        .aligned()
                        .contained()
                        .with_style(theme.dismiss_button.container)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, this, cx| {
                    this.dismiss(cx);
                })
                .flex(1., true),
            )
            .constrained()
            .with_width(theme.project_shared_notification.button_width)
            .into_any()
    }
}

impl Entity for ProjectSharedNotification {
    type Event = ();
}

impl View for ProjectSharedNotification {
    fn ui_name() -> &'static str {
        "ProjectSharedNotification"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
        let background = theme::current(cx).project_shared_notification.background;
        Flex::row()
            .with_child(self.render_owner(cx))
            .with_child(self.render_buttons(cx))
            .contained()
            .with_background_color(background)
            .expanded()
            .into_any()
    }
}
