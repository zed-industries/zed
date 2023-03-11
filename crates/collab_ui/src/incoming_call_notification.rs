use call::{ActiveCall, IncomingCall};
use client::proto;
use futures::StreamExt;
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    impl_internal_actions, CursorStyle, Entity, MouseButton, MutableAppContext, RenderContext,
    View, ViewContext, WindowBounds, WindowKind, WindowOptions,
};
use settings::Settings;
use util::ResultExt;
use workspace::JoinProject;

impl_internal_actions!(incoming_call_notification, [RespondToCall]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(IncomingCallNotification::respond_to_call);

    let mut incoming_call = ActiveCall::global(cx).read(cx).incoming();
    cx.spawn(|mut cx| async move {
        let mut notification_windows = Vec::new();
        while let Some(incoming_call) = incoming_call.next().await {
            for window_id in notification_windows.drain(..) {
                cx.remove_window(window_id);
            }

            if let Some(incoming_call) = incoming_call {
                const PADDING: f32 = 16.;
                let window_size = cx.read(|cx| {
                    let theme = &cx.global::<Settings>().theme.incoming_call_notification;
                    vec2f(theme.window_width, theme.window_height)
                });

                for screen in cx.platform().screens() {
                    let screen_bounds = screen.bounds();
                    let (window_id, _) = cx.add_window(
                        WindowOptions {
                            bounds: WindowBounds::Fixed(RectF::new(
                                screen_bounds.upper_right()
                                    - vec2f(PADDING + window_size.x(), PADDING),
                                window_size,
                            )),
                            titlebar: None,
                            center: false,
                            focus: false,
                            kind: WindowKind::PopUp,
                            is_movable: false,
                            screen: Some(screen),
                        },
                        |_| IncomingCallNotification::new(incoming_call.clone()),
                    );

                    notification_windows.push(window_id);
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

pub struct IncomingCallNotification {
    call: IncomingCall,
}

impl IncomingCallNotification {
    pub fn new(call: IncomingCall) -> Self {
        Self { call }
    }

    fn respond_to_call(&mut self, action: &RespondToCall, cx: &mut ViewContext<Self>) {
        let active_call = ActiveCall::global(cx);
        if action.accept {
            let join = active_call.update(cx, |active_call, cx| active_call.accept_incoming(cx));
            let caller_user_id = self.call.calling_user.id;
            let initial_project_id = self.call.initial_project.as_ref().map(|project| project.id);
            cx.spawn_weak(|_, mut cx| async move {
                join.await?;
                if let Some(project_id) = initial_project_id {
                    cx.update(|cx| {
                        cx.dispatch_global_action(JoinProject {
                            project_id,
                            follow_user_id: caller_user_id,
                        })
                    });
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            active_call.update(cx, |active_call, _| {
                active_call.decline_incoming().log_err();
            });
        }
    }

    fn render_caller(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.incoming_call_notification;
        let default_project = proto::ParticipantProject::default();
        let initial_project = self
            .call
            .initial_project
            .as_ref()
            .unwrap_or(&default_project);
        Flex::row()
            .with_children(self.call.calling_user.avatar.clone().map(|avatar| {
                Image::from_data(avatar)
                    .with_style(theme.caller_avatar)
                    .aligned()
                    .boxed()
            }))
            .with_child(
                Flex::column()
                    .with_child(
                        Label::new(
                            self.call.calling_user.github_login.clone(),
                            theme.caller_username.text.clone(),
                        )
                        .contained()
                        .with_style(theme.caller_username.container)
                        .boxed(),
                    )
                    .with_child(
                        Label::new(
                            format!(
                                "is sharing a project in Zed{}",
                                if initial_project.worktree_root_names.is_empty() {
                                    ""
                                } else {
                                    ":"
                                }
                            ),
                            theme.caller_message.text.clone(),
                        )
                        .contained()
                        .with_style(theme.caller_message.container)
                        .boxed(),
                    )
                    .with_children(if initial_project.worktree_root_names.is_empty() {
                        None
                    } else {
                        Some(
                            Label::new(
                                initial_project.worktree_root_names.join(", "),
                                theme.worktree_roots.text.clone(),
                            )
                            .contained()
                            .with_style(theme.worktree_roots.container)
                            .boxed(),
                        )
                    })
                    .contained()
                    .with_style(theme.caller_metadata)
                    .aligned()
                    .boxed(),
            )
            .contained()
            .with_style(theme.caller_container)
            .flex(1., true)
            .boxed()
    }

    fn render_buttons(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Accept {}
        enum Decline {}

        Flex::column()
            .with_child(
                MouseEventHandler::<Accept>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme.incoming_call_notification;
                    Label::new("Accept", theme.accept_button.text.clone())
                        .aligned()
                        .contained()
                        .with_style(theme.accept_button.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(RespondToCall { accept: true });
                })
                .flex(1., true)
                .boxed(),
            )
            .with_child(
                MouseEventHandler::<Decline>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme.incoming_call_notification;
                    Label::new("Decline", theme.decline_button.text.clone())
                        .aligned()
                        .contained()
                        .with_style(theme.decline_button.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(RespondToCall { accept: false });
                })
                .flex(1., true)
                .boxed(),
            )
            .constrained()
            .with_width(
                cx.global::<Settings>()
                    .theme
                    .incoming_call_notification
                    .button_width,
            )
            .boxed()
    }
}

impl Entity for IncomingCallNotification {
    type Event = ();
}

impl View for IncomingCallNotification {
    fn ui_name() -> &'static str {
        "IncomingCallNotification"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> gpui::ElementBox {
        let background = cx
            .global::<Settings>()
            .theme
            .incoming_call_notification
            .background;

        Flex::row()
            .with_child(self.render_caller(cx))
            .with_child(self.render_buttons(cx))
            .contained()
            .with_background_color(background)
            .expanded()
            .boxed()
    }
}
