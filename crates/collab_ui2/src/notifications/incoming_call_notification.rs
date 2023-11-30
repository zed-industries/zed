use crate::notification_window_options;
use futures::StreamExt;
use gpui::{
    div, px, red, AppContext, Div, Element, ParentElement, Render, RenderOnce, Styled, ViewContext,
    VisualContext as _, WindowHandle,
};
use std::sync::{Arc, Weak};
use ui::prelude::*;
use ui::{h_stack, v_stack, Avatar, Button, Label};
use util::ResultExt;
use workspace::{AppState, IncomingCall};

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    let app_state = Arc::downgrade(app_state);
    let mut incoming_call = workspace::call_hub(cx).incoming(cx);
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

                for window in unique_screens {
                    let options = notification_window_options(window, window_size);
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

                // for screen in cx.platform().screens() {
                //     let window = cx
                //         .add_window(notification_window_options(screen, window_size), |_| {
                //             IncomingCallNotification::new(incoming_call.clone(), app_state.clone())
                //         });

                //     notification_windows.push(window);
                // }
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
        let active_call = workspace::call_hub(cx);
        if accept {
            let join = active_call.accept_incoming(cx);
            let initial_project_id = self.call.initial_project.as_ref().map(|project| project.id);
            let app_state = self.app_state.clone();
            let cx: &mut AppContext = cx;
            cx.spawn(|cx| async move {
                join.await?;
                if let Some(_project_id) = initial_project_id {
                    cx.update(|_cx| {
                        if let Some(_app_state) = app_state.upgrade() {
                            // workspace::join_remote_project(
                            //     project_id,
                            //     caller_user_id,
                            //     app_state,
                            //     cx,
                            // )
                            // .detach_and_log_err(cx);
                        }
                    })
                    .log_err();
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            active_call.decline_incoming(cx).log_err();
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
                    .map(|avatar| Avatar::data(avatar.clone())),
            )
            .child(
                v_stack()
                    .child(Label::new(format!(
                        "{} is sharing a project in Zed",
                        self.state.call.calling_user.github_login
                    )))
                    .child(self.render_buttons(cx)),
            )
        // let theme = &theme::current(cx).incoming_call_notification;
        // let default_project = proto::ParticipantProject::default();
        // let initial_project = self
        //     .call
        //     .initial_project
        //     .as_ref()
        //     .unwrap_or(&default_project);
        // Flex::row()
        //     .with_children(self.call.calling_user.avatar.clone().map(|avatar| {
        //         Image::from_data(avatar)
        //             .with_style(theme.caller_avatar)
        //             .aligned()
        //     }))
        //     .with_child(
        //         Flex::column()
        //             .with_child(
        //                 Label::new(
        //                     self.call.calling_user.github_login.clone(),
        //                     theme.caller_username.text.clone(),
        //                 )
        //                 .contained()
        //                 .with_style(theme.caller_username.container),
        //             )
        //             .with_child(
        //                 Label::new(
        //                     format!(
        //                         "is sharing a project in Zed{}",
        //                         if initial_project.worktree_root_names.is_empty() {
        //                             ""
        //                         } else {
        //                             ":"
        //                         }
        //                     ),
        //                     theme.caller_message.text.clone(),
        //                 )
        //                 .contained()
        //                 .with_style(theme.caller_message.container),
        //             )
        //             .with_children(if initial_project.worktree_root_names.is_empty() {
        //                 None
        //             } else {
        //                 Some(
        //                     Label::new(
        //                         initial_project.worktree_root_names.join(", "),
        //                         theme.worktree_roots.text.clone(),
        //                     )
        //                     .contained()
        //                     .with_style(theme.worktree_roots.container),
        //                 )
        //             })
        //             .contained()
        //             .with_style(theme.caller_metadata)
        //             .aligned(),
        //     )
        //     .contained()
        //     .with_style(theme.caller_container)
        //     .flex(1., true)
        //     .into_any()
    }

    fn render_buttons(&self, cx: &mut ViewContext<Self>) -> impl Element {
        h_stack()
            .child(
                Button::new("accept", "Accept")
                    .render(cx)
                    // .bg(green())
                    .on_click({
                        let state = self.state.clone();
                        move |_, cx| state.respond(true, cx)
                    }),
            )
            .child(
                Button::new("decline", "Decline")
                    .render(cx)
                    // .bg(red())
                    .on_click({
                        let state = self.state.clone();
                        move |_, cx| state.respond(false, cx)
                    }),
            )

        // enum Accept {}
        // enum Decline {}

        // let theme = theme::current(cx);
        // Flex::column()
        //     .with_child(
        //         MouseEventHandler::new::<Accept, _>(0, cx, |_, _| {
        //             let theme = &theme.incoming_call_notification;
        //             Label::new("Accept", theme.accept_button.text.clone())
        //                 .aligned()
        //                 .contained()
        //                 .with_style(theme.accept_button.container)
        //         })
        //         .with_cursor_style(CursorStyle::PointingHand)
        //         .on_click(MouseButton::Left, |_, this, cx| {
        //             this.respond(true, cx);
        //         })
        //         .flex(1., true),
        //     )
        //     .with_child(
        //         MouseEventHandler::new::<Decline, _>(0, cx, |_, _| {
        //             let theme = &theme.incoming_call_notification;
        //             Label::new("Decline", theme.decline_button.text.clone())
        //                 .aligned()
        //                 .contained()
        //                 .with_style(theme.decline_button.container)
        //         })
        //         .with_cursor_style(CursorStyle::PointingHand)
        //         .on_click(MouseButton::Left, |_, this, cx| {
        //             this.respond(false, cx);
        //         })
        //         .flex(1., true),
        //     )
        //     .constrained()
        //     .with_width(theme.incoming_call_notification.button_width)
        //     .into_any()
    }
}
impl Render for IncomingCallNotification {
    type Element = Div;
    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div().bg(red()).flex_none().child(self.render_caller(cx))
        // Flex::row()
        //     .with_child()
        //     .with_child(self.render_buttons(cx))
        //     .contained()
        //     .with_background_color(background)
        //     .expanded()
        //     .into_any()
    }
}
