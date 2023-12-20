use crate::notification_window_options;
use call::{ActiveCall, IncomingCall};
use futures::StreamExt;
use gpui::{
    img, px, AppContext, Div, ParentElement, Render, RenderOnce, Styled, ViewContext,
    VisualContext as _, WindowHandle,
};
use settings::Settings;
use std::sync::{Arc, Weak};
use theme::ThemeSettings;
use ui::prelude::*;
use ui::{h_stack, v_stack, Button, Label};
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
}

impl Render for IncomingCallNotification {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        // TODO: Is there a better place for us to initialize the font?
        let (ui_font, ui_font_size) = {
            let theme_settings = ThemeSettings::get_global(cx);
            (
                theme_settings.ui_font.family.clone(),
                theme_settings.ui_font_size.clone(),
            )
        };

        cx.set_rem_size(ui_font_size);

        h_stack()
            .font(ui_font)
            .text_ui()
            .justify_between()
            .size_full()
            .overflow_hidden()
            .elevation_3(cx)
            .p_2()
            .gap_2()
            .child(
                img(self.state.call.calling_user.avatar_uri.clone())
                    .w_12()
                    .h_12()
                    .rounded_full(),
            )
            .child(v_stack().overflow_hidden().child(Label::new(format!(
                "{} is sharing a project in Zed",
                self.state.call.calling_user.github_login
            ))))
            .child(
                v_stack()
                    .child(Button::new("accept", "Accept").render(cx).on_click({
                        let state = self.state.clone();
                        move |_, cx| state.respond(true, cx)
                    }))
                    .child(Button::new("decline", "Decline").render(cx).on_click({
                        let state = self.state.clone();
                        move |_, cx| state.respond(false, cx)
                    })),
            )
    }
}
