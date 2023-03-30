use crate::{request::PromptUserDeviceFlow, Copilot, Status};
use gpui::{
    elements::*, geometry::rect::RectF, ClipboardItem, Element, Entity, MutableAppContext, View,
    ViewContext, ViewHandle, WindowKind, WindowOptions,
};
use settings::Settings;

#[derive(PartialEq, Eq, Debug, Clone)]
struct CopyUserCode;

#[derive(PartialEq, Eq, Debug, Clone)]
struct OpenGithub;

const _COPILOT_SIGN_UP_URL: &'static str = "https://github.com/features/copilot";

pub fn init(cx: &mut MutableAppContext) {
    let copilot = Copilot::global(cx).unwrap();

    let mut code_verification: Option<ViewHandle<CopilotCodeVerification>> = None;
    cx.observe(&copilot, move |copilot, cx| {
        let status = copilot.read(cx).status();

        match &status {
            crate::Status::SigningIn { prompt } => {
                if let Some(code_verification) = code_verification.as_ref() {
                    code_verification.update(cx, |code_verification, cx| {
                        code_verification.set_status(status, cx)
                    });
                    cx.activate_window(code_verification.window_id());
                } else if let Some(_prompt) = prompt {
                    let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();
                    let window_options = WindowOptions {
                        bounds: gpui::WindowBounds::Fixed(RectF::new(
                            Default::default(),
                            window_size,
                        )),
                        titlebar: None,
                        center: true,
                        focus: true,
                        kind: WindowKind::Normal,
                        is_movable: true,
                        screen: None,
                    };
                    let (_, view) =
                        cx.add_window(window_options, |_cx| CopilotCodeVerification::new(status));
                    code_verification = Some(view);
                }
            }
            Status::Authorized | Status::Unauthorized => {
                if let Some(code_verification) = code_verification.as_ref() {
                    code_verification.update(cx, |code_verification, cx| {
                        code_verification.set_status(status, cx)
                    });

                    cx.platform().activate(true);
                    cx.activate_window(code_verification.window_id());
                }
            }
            _ => {
                if let Some(code_verification) = code_verification.take() {
                    cx.remove_window(code_verification.window_id());
                }
            }
        }
    })
    .detach();

    // Modal theming test:
    // use gpui::geometry::vector::vec2f;
    // let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();
    // let window_options = WindowOptions {
    //     bounds: gpui::WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
    //     titlebar: None,
    //     center: false,
    //     focus: false,
    //     kind: WindowKind::PopUp,
    //     is_movable: true,
    //     screen: None,
    // };
    // let (_, _view) = cx.add_window(window_options, |_cx| {
    //     CopilotCodeVerification::new(Status::SigningIn {
    //         prompt: Some(PromptUserDeviceFlow {
    //             user_code: "ABCD-1234".to_string(),
    //             verification_uri: "https://github.com/login/device".to_string(),
    //         }),
    //     })
    // });

    // let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();
    // let window_options = WindowOptions {
    //     bounds: gpui::WindowBounds::Fixed(RectF::new(vec2f(window_size.x(), 0.), window_size)),
    //     titlebar: None,
    //     center: false,
    //     focus: false,
    //     kind: WindowKind::PopUp,
    //     is_movable: true,
    //     screen: None,
    // };
    // let (_, _view) = cx.add_window(window_options, |_cx| {
    //     CopilotCodeVerification::new(Status::Authorized)
    // });

    // let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();
    // let window_options = WindowOptions {
    //     bounds: gpui::WindowBounds::Fixed(RectF::new(vec2f(0., window_size.y()), window_size)),
    //     titlebar: None,
    //     center: false,
    //     focus: false,
    //     kind: WindowKind::PopUp,
    //     is_movable: true,
    //     screen: None,
    // };
    // let (_, _view) = cx.add_window(window_options, |_cx| {
    //     CopilotCodeVerification::new(Status::Unauthorized)
    // });
}

pub struct CopilotCodeVerification {
    status: Status,
}

impl CopilotCodeVerification {
    pub fn new(status: Status) -> Self {
        Self { status }
    }

    pub fn set_status(&mut self, status: Status, cx: &mut ViewContext<Self>) {
        self.status = status;
        cx.notify();
    }

    fn render_device_code(
        data: &PromptUserDeviceFlow,
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text() == &data.user_code)
            .unwrap_or(false);

        Flex::column()
            .with_children([
                MouseEventHandler::<Self>::new(0, cx, |state, _cx| {
                    Flex::row()
                        .with_children([
                            Label::new(data.user_code.clone(), style.auth.device_code.clone())
                                .aligned()
                                .contained()
                                .with_style(style.auth.device_code_left_container)
                                .constrained()
                                .with_width(style.auth.device_code_left)
                                .boxed(),
                            Empty::new()
                                .constrained()
                                .with_width(1.)
                                .with_height(style.auth.device_code_seperator_height)
                                .contained()
                                .with_background_color(
                                    style
                                        .auth
                                        .cta_button
                                        .style_for(state, false)
                                        .container
                                        .border
                                        .color,
                                )
                                .boxed(),
                            Label::new(
                                if copied { "Copied!" } else { "Copy" },
                                style.auth.cta_button.style_for(state, false).text.clone(),
                            )
                            .aligned()
                            .contained()
                            .with_style(style.auth.device_code_right_container)
                            .constrained()
                            .with_width(style.auth.device_code_right)
                            .boxed(),
                        ])
                        .contained()
                        .with_style(style.auth.device_code_cta.style_for(state, false).container)
                        .constrained()
                        .with_width(style.auth.content_width)
                        .boxed()
                })
                .on_click(gpui::MouseButton::Left, {
                    let user_code = data.user_code.clone();
                    move |_, cx| {
                        cx.platform()
                            .write_to_clipboard(ClipboardItem::new(user_code.clone()));
                        cx.notify();
                    }
                })
                .with_cursor_style(gpui::CursorStyle::PointingHand)
                .boxed(),
                Flex::column()
                    .with_children([
                        Label::new(
                            "Paste this code into GitHub after",
                            style.auth.hint.text.clone(),
                        )
                        .boxed(),
                        Label::new("clicking the button below.", style.auth.hint.text.clone())
                            .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.hint.container.clone())
                    .boxed(),
            ])
            .align_children_center()
            .contained()
            .with_style(style.auth.device_code_group)
            .aligned()
            .boxed()
    }

    fn render_not_authorized_warning(style: &theme::Copilot) -> ElementBox {
        Flex::column()
            .with_children([
                Flex::column()
                    .with_children([
                        Label::new(
                            "You must have an active copilot",
                            style.auth.warning.text.to_owned(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "license to use it in Zed.",
                            style.auth.warning.text.to_owned(),
                        )
                        .aligned()
                        .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.warning.container)
                    .boxed(),
                Flex::column()
                    .with_children([
                        Label::new(
                            "Try connecting again once you",
                            style.auth.hint.text.to_owned(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "have activated a Copilot license.",
                            style.auth.hint.text.to_owned(),
                        )
                        .aligned()
                        .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.not_authorized_hint)
                    .boxed(),
            ])
            .align_children_center()
            .boxed()
    }

    fn render_copilot_enabled(style: &theme::Copilot) -> ElementBox {
        Flex::column()
            .with_children([
                Label::new(
                    "You can update your settings or",
                    style.auth.hint.text.clone(),
                )
                .aligned()
                .boxed(),
                Label::new(
                    "sign out from the Copilot menu in",
                    style.auth.hint.text.clone(),
                )
                .aligned()
                .boxed(),
                Label::new("the status bar.", style.auth.hint.text.clone())
                    .aligned()
                    .boxed(),
            ])
            .align_children_center()
            .contained()
            .with_style(style.auth.enabled_hint)
            .boxed()
    }

    fn render_prompting_modal(
        data: &PromptUserDeviceFlow,
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        theme::ui::modal("Connect Copilot to Zed", &style.modal, cx, |cx| {
            Flex::column()
                .with_children([
                    Flex::column()
                        .with_children([
                            Flex::row()
                                .with_children([
                                    theme::ui::svg(&style.auth.copilot_icon).boxed(),
                                    theme::ui::icon(&style.auth.plus_icon).boxed(),
                                    theme::ui::svg(&style.auth.zed_icon).boxed(),
                                ])
                                .boxed(),
                            Flex::column()
                                .with_children([
                                    Label::new(
                                        "Enable Copilot by connecting",
                                        style.auth.enable_text.clone(),
                                    )
                                    .boxed(),
                                    Label::new(
                                        "your existing license.",
                                        style.auth.enable_text.clone(),
                                    )
                                    .boxed(),
                                ])
                                .align_children_center()
                                .contained()
                                .with_style(style.auth.enable_group.clone())
                                .boxed(),
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.header_group)
                        .aligned()
                        .boxed(),
                    Self::render_device_code(data, &style, cx),
                    Flex::column()
                        .with_child(theme::ui::cta_button_with_click(
                            "Connect to GitHub",
                            style.auth.content_width,
                            &style.auth.cta_button,
                            cx,
                            {
                                let verification_uri = data.verification_uri.clone();
                                move |_, cx| cx.platform().open_url(&verification_uri)
                            },
                        ))
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.github_group)
                        .aligned()
                        .boxed(),
                ])
                .align_children_center()
                .constrained()
                .with_width(style.auth.content_width)
                .aligned()
                .boxed()
        })
    }
    fn render_enabled_modal(
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        theme::ui::modal("Connect Copilot to Zed", &style.modal, cx, |cx| {
            Flex::column()
                .with_children([
                    Flex::column()
                        .with_children([
                            Flex::row()
                                .with_children([
                                    theme::ui::svg(&style.auth.copilot_icon).boxed(),
                                    theme::ui::icon(&style.auth.plus_icon).boxed(),
                                    theme::ui::svg(&style.auth.zed_icon).boxed(),
                                ])
                                .boxed(),
                            Label::new("Copilot Enabled!", style.auth.enable_text.clone()).boxed(),
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.header_group)
                        .aligned()
                        .boxed(),
                    Self::render_copilot_enabled(&style),
                    Flex::column()
                        .with_child(theme::ui::cta_button_with_click(
                            "Close",
                            style.auth.content_width,
                            &style.auth.cta_button,
                            cx,
                            |_, cx| {
                                let window_id = cx.window_id();
                                cx.remove_window(window_id)
                            },
                        ))
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.github_group)
                        .aligned()
                        .boxed(),
                ])
                .align_children_center()
                .constrained()
                .with_width(style.auth.content_width)
                .aligned()
                .boxed()
        })
    }
    fn render_unauthorized_modal(
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        theme::ui::modal("Connect Copilot to Zed", &style.modal, cx, |cx| {
            Flex::column()
                .with_children([
                    Flex::column()
                        .with_children([
                            Flex::row()
                                .with_children([
                                    theme::ui::svg(&style.auth.copilot_icon).boxed(),
                                    theme::ui::icon(&style.auth.plus_icon).boxed(),
                                    theme::ui::svg(&style.auth.zed_icon).boxed(),
                                ])
                                .boxed(),
                            Flex::column()
                                .with_children([
                                    Label::new(
                                        "Enable Copilot by connecting",
                                        style.auth.enable_text.clone(),
                                    )
                                    .boxed(),
                                    Label::new(
                                        "your existing license.",
                                        style.auth.enable_text.clone(),
                                    )
                                    .boxed(),
                                ])
                                .align_children_center()
                                .contained()
                                .with_style(style.auth.enable_group.clone())
                                .boxed(),
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.header_group)
                        .aligned()
                        .boxed(),
                    Self::render_not_authorized_warning(&style),
                    Flex::column()
                        .with_child(theme::ui::cta_button_with_click(
                            "Close",
                            style.auth.content_width,
                            &style.auth.cta_button,
                            cx,
                            |_, cx| {
                                let window_id = cx.window_id();
                                cx.remove_window(window_id)
                            },
                        ))
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.github_group)
                        .aligned()
                        .boxed(),
                ])
                .align_children_center()
                .constrained()
                .with_width(style.auth.content_width)
                .aligned()
                .boxed()
        })
    }
}

impl Entity for CopilotCodeVerification {
    type Event = ();
}

impl View for CopilotCodeVerification {
    fn ui_name() -> &'static str {
        "CopilotCodeVerification"
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut gpui::ViewContext<Self>) {
        cx.notify()
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut gpui::ViewContext<Self>) {
        cx.notify()
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let style = cx.global::<Settings>().theme.copilot.clone();
        match &self.status {
            Status::SigningIn {
                prompt: Some(prompt),
            } => Self::render_prompting_modal(&prompt, &style, cx),
            Status::Unauthorized => Self::render_unauthorized_modal(&style, cx),
            Status::Authorized => Self::render_enabled_modal(&style, cx),
            _ => Empty::new().boxed(),
        }
    }
}
