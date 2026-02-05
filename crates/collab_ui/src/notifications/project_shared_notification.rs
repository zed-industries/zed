use crate::notification_window_options;
use call::{ActiveCall, room};
use client::User;
use collections::HashMap;
use gpui::{App, Size};
use std::sync::{Arc, Weak};

use ui::{CollabNotification, prelude::*};
use util::ResultExt;
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
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
                let Some(window) = cx
                    .open_window(options, |_, cx| {
                        cx.new(|_| {
                            ProjectSharedNotification::new(
                                owner.clone(),
                                *project_id,
                                worktree_root_names.clone(),
                                app_state.clone(),
                            )
                        })
                    })
                    .log_err()
                else {
                    continue;
                };
                notification_windows
                    .entry(*project_id)
                    .or_insert(Vec::new())
                    .push(window);
            }
        }

        room::Event::RemoteProjectUnshared { project_id }
        | room::Event::RemoteProjectJoined { project_id }
        | room::Event::RemoteProjectInvitationDiscarded { project_id } => {
            if let Some(windows) = notification_windows.remove(project_id) {
                for window in windows {
                    window
                        .update(cx, |_, window, _| {
                            window.remove_window();
                        })
                        .ok();
                }
            }
        }

        room::Event::RoomLeft { .. } => {
            for (_, windows) in notification_windows.drain() {
                for window in windows {
                    window
                        .update(cx, |_, window, _| {
                            window.remove_window();
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

    fn join(&mut self, cx: &mut Context<Self>) {
        if let Some(app_state) = self.app_state.upgrade() {
            workspace::join_in_room_project(self.project_id, self.owner.id, app_state, cx)
                .detach_and_log_err(cx);
        }
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        if let Some(active_room) = ActiveCall::global(cx).read(cx).room().cloned() {
            active_room.update(cx, |_, cx| {
                cx.emit(room::Event::RemoteProjectInvitationDiscarded {
                    project_id: self.project_id,
                });
            });
        }
    }
}

impl Render for ProjectSharedNotification {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);
        let no_worktree_root_names = self.worktree_root_names.is_empty();

        let punctuation = if no_worktree_root_names { "" } else { ":" };
        let main_label = format!(
            "{} is sharing a project with you{}",
            self.owner.github_login.clone(),
            punctuation
        );

        div().size_full().font(ui_font).child(
            CollabNotification::new(
                self.owner.avatar_uri.clone(),
                Button::new("open", "Open").on_click(cx.listener(move |this, _event, _, cx| {
                    this.join(cx);
                })),
                Button::new("dismiss", "Dismiss").on_click(cx.listener(
                    move |this, _event, _, cx| {
                        this.dismiss(cx);
                    },
                )),
            )
            .child(Label::new(main_label))
            .when(!no_worktree_root_names, |this| {
                this.child(Label::new(self.worktree_root_names.join(", ")).color(Color::Muted))
            }),
        )
    }
}
