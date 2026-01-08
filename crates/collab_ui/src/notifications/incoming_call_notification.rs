use crate::notification_window_options;
use crate::notifications::collab_notification::CollabNotification;
use call::{ActiveCall, IncomingCall};
use futures::StreamExt;
use gpui::{App, WindowHandle, prelude::*};

use std::sync::{Arc, Weak};
use ui::{Button, Label, prelude::*};
use util::ResultExt;
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    let app_state = Arc::downgrade(app_state);
    let mut incoming_call = ActiveCall::global(cx).read(cx).incoming();
    cx.spawn(async move |cx| {
        let mut notification_windows: Vec<WindowHandle<IncomingCallNotification>> = Vec::new();
        while let Some(incoming_call) = incoming_call.next().await {
            for window in notification_windows.drain(..) {
                window
                    .update(cx, |_, window, _| {
                        window.remove_window();
                    })
                    .log_err();
            }

            if let Some(incoming_call) = incoming_call {
                let unique_screens = cx.update(|cx| cx.displays());
                let window_size = gpui::Size {
                    width: px(400.),
                    height: px(72.),
                };

                for screen in unique_screens {
                    let options =
                        cx.update(|cx| notification_window_options(screen, window_size, cx));
                    if let Ok(window) = cx.open_window(options, |_, cx| {
                        cx.new(|_| {
                            IncomingCallNotification::new(incoming_call.clone(), app_state.clone())
                        })
                    }) {
                        notification_windows.push(window);
                    }
                }
            }
        }
    })
    .detach();
}

struct IncomingCallNotificationState {
    call: IncomingCall,
    app_state: Weak<AppState>,
}

pub struct IncomingCallNotification {
    state: Arc<IncomingCallNotificationState>,
}
impl IncomingCallNotificationState {
    pub fn new(call: IncomingCall, app_state: Weak<AppState>) -> Self {
        Self { call, app_state }
    }

    fn respond(&self, accept: bool, cx: &mut App) {
        let active_call = ActiveCall::global(cx);
        if accept {
            let join = active_call.update(cx, |active_call, cx| active_call.accept_incoming(cx));
            let caller_user_id = self.call.calling_user.id;
            let initial_project_id = self.call.initial_project.as_ref().map(|project| project.id);
            let app_state = self.app_state.clone();
            let cx: &mut App = cx;
            cx.spawn(async move |cx| {
                join.await?;
                if let Some(project_id) = initial_project_id {
                    cx.update(|cx| {
                        if let Some(app_state) = app_state.upgrade() {
                            workspace::join_in_room_project(
                                project_id,
                                caller_user_id,
                                app_state,
                                cx,
                            )
                            .detach_and_log_err(cx);
                        }
                    });
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            active_call.update(cx, |active_call, cx| {
                active_call.decline_incoming(cx).log_err();
            });
        }
    }
}

impl IncomingCallNotification {
    pub fn new(call: IncomingCall, app_state: Weak<AppState>) -> Self {
        Self {
            state: Arc::new(IncomingCallNotificationState::new(call, app_state)),
        }
    }
}

impl Render for IncomingCallNotification {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);

        div().size_full().font(ui_font).child(
            CollabNotification::new(
                self.state.call.calling_user.avatar_uri.clone(),
                Button::new("accept", "Accept").on_click({
                    let state = self.state.clone();
                    move |_, _, cx| state.respond(true, cx)
                }),
                Button::new("decline", "Decline").on_click({
                    let state = self.state.clone();
                    move |_, _, cx| state.respond(false, cx)
                }),
            )
            .child(v_flex().overflow_hidden().child(Label::new(format!(
                "{} is sharing a project in Zed",
                self.state.call.calling_user.github_login
            )))),
        )
    }
}
