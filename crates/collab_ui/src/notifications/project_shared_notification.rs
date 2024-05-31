use crate::notification_window_options;
use crate::notifications::collab_notification::CollabNotification;
use call::{room, ActiveCall};
use client::User;
use collections::HashMap;
use gpui::{AppContext, Size};
use settings::Settings;
use std::sync::{Arc, Weak};
use theme::ThemeSettings;
use ui::{prelude::*, Button, Label};
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
            let window_size = Size {
                width: px(400.),
                height: px(72.),
            };

            for screen in cx.displays() {
                let options = notification_window_options(screen, window_size, cx);
                let window = cx.open_window(options, |cx| {
                    cx.new_view(|_| {
                        ProjectSharedNotification::new(
                            owner.clone(),
                            *project_id,
                            worktree_root_names.clone(),
                            app_state.clone(),
                        )
                    })
                });
                notification_windows
                    .entry(*project_id)
                    .or_insert(Vec::new())
                    .push(window);
            }
        }

        room::Event::RemoteProjectUnshared { project_id }
        | room::Event::RemoteProjectJoined { project_id }
        | room::Event::RemoteProjectInvitationDiscarded { project_id } => {
            if let Some(windows) = notification_windows.remove(&project_id) {
                for window in windows {
                    window
                        .update(cx, |_, cx| {
                            cx.remove_window();
                        })
                        .ok();
                }
            }
        }

        room::Event::RoomLeft { .. } => {
            for (_, windows) in notification_windows.drain() {
                for window in windows {
                    window
                        .update(cx, |_, cx| {
                            cx.remove_window();
                        })
                        .ok();
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
        if let Some(app_state) = self.app_state.upgrade() {
            workspace::join_in_room_project(self.project_id, self.owner.id, app_state, cx)
                .detach_and_log_err(cx);
        }
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_room) =
            ActiveCall::global(cx).read_with(cx, |call, _| call.room().cloned())
        {
            active_room.update(cx, |_, cx| {
                cx.emit(room::Event::RemoteProjectInvitationDiscarded {
                    project_id: self.project_id,
                });
            });
        }
    }
}

impl Render for ProjectSharedNotification {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // TODO: Is there a better place for us to initialize the font?
        let (ui_font, ui_font_size) = {
            let theme_settings = ThemeSettings::get_global(cx);
            (theme_settings.ui_font.clone(), theme_settings.ui_font_size)
        };

        cx.set_rem_size(ui_font_size);

        div().size_full().font(ui_font).child(
            CollabNotification::new(
                self.owner.avatar_uri.clone(),
                Button::new("open", "Open").on_click(cx.listener(move |this, _event, cx| {
                    this.join(cx);
                })),
                Button::new("dismiss", "Dismiss").on_click(cx.listener(move |this, _event, cx| {
                    this.dismiss(cx);
                })),
            )
            .child(Label::new(self.owner.github_login.clone()))
            .child(Label::new(format!(
                "is sharing a project in Zed{}",
                if self.worktree_root_names.is_empty() {
                    ""
                } else {
                    ":"
                }
            )))
            .children(if self.worktree_root_names.is_empty() {
                None
            } else {
                Some(Label::new(self.worktree_root_names.join(", ")))
            }),
        )
    }
}
