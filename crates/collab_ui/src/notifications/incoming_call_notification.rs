use crate::notification_window_options;
use crate::notifications::collab_notification::CollabNotification;
use call::{ActiveCall, IncomingCall};
use futures::StreamExt;
use gpui::{prelude::*, AppContext, WindowHandle};
use settings::Settings;
use std::sync::{Arc, Weak};
use theme::ThemeSettings;
use ui::{prelude::*, Button, Label};
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
                        cx.remove_window();
                    })
                    .log_err();
            }

            if let Some(incoming_call) = incoming_call {
                let unique_screens = cx.update(|cx| cx.displays()).unwrap();
                let window_size = gpui::Size {
                    width: px(400.),
                    height: px(72.),
                };

                for screen in unique_screens {
                    if let Some(options) = cx
                        .update(|cx| notification_window_options(screen, window_size, cx))
                        .log_err()
                    {
                        let window = cx
                            .open_window(options, |cx| {
                                cx.new_view(|_| {
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
                            workspace::join_in_room_project(
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // TODO: Is there a better place for us to initialize the font?
        let (ui_font, ui_font_size) = {
            let theme_settings = ThemeSettings::get_global(cx);
            (theme_settings.ui_font.clone(), theme_settings.ui_font_size)
        };

        cx.set_rem_size(ui_font_size);

        div().size_full().font(ui_font).child(
            CollabNotification::new(
                self.state.call.calling_user.avatar_uri.clone(),
                Button::new("accept", "Accept").on_click({
                    let state = self.state.clone();
                    move |_, cx| state.respond(true, cx)
                }),
                Button::new("decline", "Decline").on_click({
                    let state = self.state.clone();
                    move |_, cx| state.respond(false, cx)
                }),
            )
            .child(v_flex().overflow_hidden().child(Label::new(format!(
                "{} is sharing a project in Zed",
                self.state.call.calling_user.github_login
            )))),
        )
    }
}
