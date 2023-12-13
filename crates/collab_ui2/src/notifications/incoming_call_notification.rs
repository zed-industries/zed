use crate::notification_window_options;
use call::{ActiveCall, IncomingCall};
use futures::StreamExt;
use gpui::{
    div, px, red, AppContext, Div, Element, ParentElement, Render, RenderOnce, Styled, ViewContext,
    VisualContext as _, WindowHandle,
};
use std::sync::{Arc, Weak};
use ui::prelude::*;
use ui::{h_stack, v_stack, Avatar, Button, Label};
use util::ResultExt;
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    let app_state = Arc::downgrade(app_state);
    let mut incoming_call = ActiveCall::global(cx).read(cx).incoming();
    cx.spawn(|mut cx| async move {
        let mut notification_windows: Vec<WindowHandle<IncomingCallNotification>> = Vec::new();
        while let Some(incoming_call) = incoming_call.next().await {
            for window in notification_windows.drain(..) {
                window
                    .update(&mut cx, |_, cx| {
                        // todo!()
                        cx.remove_window();
                    })
                    .log_err();
            }

            if let Some(incoming_call) = incoming_call {
                let unique_screens = cx.update(|cx| cx.displays()).unwrap();
                let window_size = gpui::Size {
                    width: px(380.),
                    height: px(64.),
                };

                for screen in unique_screens {
                    let options = notification_window_options(screen, window_size);
                    let window = cx
                        .open_window(options, |cx| {
                            cx.build_view(|_| {
                                IncomingCallNotification::new(
                                    incoming_call.clone(),
                                    app_state.clone(),
                                )
                            })
                        })
                        .unwrap();
                    notification_windows.push(window);
                }
            }
        }
    })
    .detach();
}

#[derive(Clone, PartialEq)]
struct RespondToCall {
    accept: bool,
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

    fn respond(&self, accept: bool, cx: &mut AppContext) {
        let active_call = ActiveCall::global(cx);
        if accept {
            let join = active_call.update(cx, |active_call, cx| active_call.accept_incoming(cx));
            let caller_user_id = self.call.calling_user.id;
            let initial_project_id = self.call.initial_project.as_ref().map(|project| project.id);
            let app_state = self.app_state.clone();
            let cx: &mut AppContext = cx;
            cx.spawn(|cx| async move {
                join.await?;
                if let Some(project_id) = initial_project_id {
                    cx.update(|cx| {
                        if let Some(app_state) = app_state.upgrade() {
                            workspace::join_remote_project(
                                project_id,
                                caller_user_id,
                                app_state,
                                cx,
                            )
                            .detach_and_log_err(cx);
                        }
                    })
                    .log_err();
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
    fn render_caller(&self, cx: &mut ViewContext<Self>) -> impl Element {
        h_stack()
            .children(
                self.state
                    .call
                    .calling_user
                    .avatar
                    .as_ref()
                    .map(|avatar| Avatar::new(avatar.clone())),
            )
            .child(
                v_stack()
                    .child(Label::new(format!(
                        "{} is sharing a project in Zed",
                        self.state.call.calling_user.github_login
                    )))
                    .child(self.render_buttons(cx)),
            )
    }

    fn render_buttons(&self, cx: &mut ViewContext<Self>) -> impl Element {
        h_stack()
            .child(Button::new("accept", "Accept").render(cx).on_click({
                let state = self.state.clone();
                move |_, cx| state.respond(true, cx)
            }))
            .child(Button::new("decline", "Decline").render(cx).on_click({
                let state = self.state.clone();
                move |_, cx| state.respond(false, cx)
            }))
    }
}

impl Render for IncomingCallNotification {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div().bg(red()).flex_none().child(self.render_caller(cx))
    }
}
