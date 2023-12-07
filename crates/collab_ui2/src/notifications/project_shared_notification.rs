use crate::notification_window_options;
use call::{room, ActiveCall};
use client::User;
use collections::HashMap;
use gpui::{
    px, AppContext, Div, Element, ParentElement, Render, RenderOnce, Size, Styled, ViewContext,
    VisualContext,
};
use std::sync::{Arc, Weak};
use ui::{h_stack, v_stack, Avatar, Button, Clickable, Label};
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
                width: px(380.),
                height: px(64.),
            };

            for screen in cx.displays() {
                let options = notification_window_options(screen, window_size);
                let window = cx.open_window(options, |cx| {
                    cx.build_view(|_| {
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
                            // todo!()
                            cx.remove_window();
                        })
                        .ok();
                }
            }
        }

        room::Event::Left => {
            for (_, windows) in notification_windows.drain() {
                for window in windows {
                    window
                        .update(cx, |_, cx| {
                            // todo!()
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
            workspace::join_remote_project(self.project_id, self.owner.id, app_state, cx)
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

    fn render_owner(&self) -> impl Element {
        h_stack()
            .children(
                self.owner
                    .avatar
                    .clone()
                    .map(|avatar| Avatar::data(avatar.clone())),
            )
            .child(
                v_stack()
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

    fn render_buttons(&self, cx: &mut ViewContext<Self>) -> impl Element {
        let this = cx.view().clone();
        v_stack()
            .child(Button::new("open", "Open").render(cx).on_click({
                let this = this.clone();
                move |_, cx| {
                    this.update(cx, |this, cx| this.join(cx));
                }
            }))
            .child(
                Button::new("dismiss", "Dismiss")
                    .render(cx)
                    .on_click(move |_, cx| {
                        this.update(cx, |this, cx| this.dismiss(cx));
                    }),
            )
    }
}

impl Render for ProjectSharedNotification {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        h_stack()
            .size_full()
            .bg(gpui::red())
            .child(self.render_owner())
            .child(self.render_buttons(cx))
    }
}
