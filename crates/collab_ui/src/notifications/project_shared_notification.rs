use crate::notification_window_options;
use call::{ActiveCall, room};
use client::User;
use collections::HashMap;
use gpui::{App, Size, TaskExt};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Weak},
};

use ui::{CollabNotification, prelude::*};
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    let app_state = Arc::downgrade(app_state);
    let active_call = ActiveCall::global(cx);
    let notification_windows = Rc::new(RefCell::new(HashMap::default()));
    let generations = Rc::new(RefCell::new(HashMap::<u64, u64>::default()));
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

            let project_id = *project_id;
            let generation = *generations.borrow_mut().entry(project_id).or_default();
            for screen in cx.displays() {
                let options = notification_window_options(screen, window_size, cx);
                let owner = owner.clone();
                let worktree_root_names = worktree_root_names.clone();
                let app_state = app_state.clone();
                let open_window = cx.open_window(options, move |_, cx| {
                    cx.new(|_| {
                        ProjectSharedNotification::new(
                            owner,
                            project_id,
                            worktree_root_names,
                            app_state,
                        )
                    })
                });
                let notification_windows = notification_windows.clone();
                let generations = generations.clone();
                cx.spawn(async move |cx| {
                    let window = open_window.await?;
                    if generations.borrow().get(&project_id).copied().unwrap_or(0) == generation {
                        notification_windows
                            .borrow_mut()
                            .entry(project_id)
                            .or_insert_with(Vec::new)
                            .push(window);
                    } else {
                        window.update(cx, |_, window, _| window.remove_window())?;
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        }

        room::Event::RemoteProjectUnshared { project_id }
        | room::Event::RemoteProjectJoined { project_id }
        | room::Event::RemoteProjectInvitationDiscarded { project_id } => {
            *generations.borrow_mut().entry(*project_id).or_default() += 1;
            let windows = notification_windows.borrow_mut().remove(project_id);
            if let Some(windows) = windows {
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
            for generation in generations.borrow_mut().values_mut() {
                *generation += 1;
            }
            let windows = notification_windows
                .borrow_mut()
                .drain()
                .collect::<Vec<_>>();
            for (_, windows) in windows {
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
            workspace::join_in_room_project(self.project_id, self.owner.legacy_id, app_state, cx)
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
        let ui_font = theme_settings::setup_ui_font(window, cx);
        let no_worktree_root_names = self.worktree_root_names.is_empty();

        let punctuation = if no_worktree_root_names { "" } else { ":" };
        let main_label = format!(
            "{} is sharing a project with you{}",
            self.owner.username.clone(),
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
