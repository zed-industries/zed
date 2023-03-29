use crate::{request::PromptUserDeviceFlow, Copilot};
use gpui::{
    elements::*, geometry::rect::RectF, impl_internal_actions, ClipboardItem, Element, Entity,
    MutableAppContext, View, WindowKind, WindowOptions,
};
use settings::Settings;

#[derive(PartialEq, Eq, Debug, Clone)]
struct CopyUserCode;

#[derive(PartialEq, Eq, Debug, Clone)]
struct OpenGithub;

impl_internal_actions!(copilot_sign_in, [CopyUserCode, OpenGithub]);

const _COPILOT_SIGN_UP_URL: &'static str = "https://github.com/features/copilot";

enum SignInContents {
    PromptingUser(PromptUserDeviceFlow),
    Unauthorized,
    Enabled,
}

pub fn init(cx: &mut MutableAppContext) {
    let copilot = Copilot::global(cx).unwrap();

    let mut code_verification_window_id: Option<(usize, SignInContents)> = None;
    cx.observe(&copilot, move |copilot, cx| {
        match copilot.read(cx).status() {
            crate::Status::SigningIn {
                prompt: Some(prompt),
            } => {
                let window_id = match code_verification_window_id.take() {
                    Some((window_id, SignInContents::PromptingUser(current_prompt)))
                        if current_prompt == prompt =>
                    {
                        if cx.window_ids().find(|item| item == &window_id).is_some() {
                            window_id
                        } else {
                            CopilotCodeVerification::prompting(prompt.clone(), cx)
                        }
                    }
                    Some((window_id, _)) => {
                        cx.remove_window(window_id);
                        CopilotCodeVerification::prompting(prompt.clone(), cx)
                    }
                    None => CopilotCodeVerification::prompting(prompt.clone(), cx),
                };

                code_verification_window_id =
                    Some((window_id, SignInContents::PromptingUser(prompt)));

                cx.activate_window(window_id);
            }
            crate::Status::Authorized => match code_verification_window_id.take() {
                Some((window_id, sign_in_contents)) => {
                    match sign_in_contents {
                        SignInContents::PromptingUser(_) => cx.remove_window(window_id),
                        SignInContents::Unauthorized => cx.remove_window(window_id),
                        SignInContents::Enabled => {
                            if cx.has_window(window_id) {
                                code_verification_window_id =
                                    Some((window_id, SignInContents::Enabled))
                            }
                            return;
                        }
                    }
                    let window_id = CopilotCodeVerification::enabled(cx);
                    code_verification_window_id = Some((window_id, SignInContents::Enabled));
                    cx.activate_window(window_id);
                }
                None => return,
            },
            crate::Status::Unauthorized => match code_verification_window_id.take() {
                Some((window_id, sign_in_contents)) => {
                    match sign_in_contents {
                        SignInContents::PromptingUser(_) => cx.remove_window(window_id), // Show prompt
                        SignInContents::Unauthorized => {
                            if cx.has_window(window_id) {
                                code_verification_window_id =
                                    Some((window_id, SignInContents::Unauthorized))
                            }
                            return;
                        } //Do nothing
                        SignInContents::Enabled => cx.remove_window(window_id),          //
                    }

                    let window_id = CopilotCodeVerification::unauthorized(cx);
                    code_verification_window_id = Some((window_id, SignInContents::Unauthorized));
                    cx.activate_window(window_id);
                }
                None => return,
            },
            _ => {
                if let Some((window_id, _)) = code_verification_window_id.take() {
                    cx.remove_window(window_id);
                }
            }
        }
    })
    .detach();
}

pub struct CopilotCodeVerification {
    prompt: SignInContents,
}

impl CopilotCodeVerification {
    pub fn prompting(prompt: PromptUserDeviceFlow, cx: &mut MutableAppContext) -> usize {
        let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();

        let (window_id, _) = cx.add_window(
            WindowOptions {
                bounds: gpui::WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
                titlebar: None,
                center: true,
                focus: false,
                kind: WindowKind::Normal,
                is_movable: true,
                screen: None,
            },
            |_| CopilotCodeVerification {
                prompt: SignInContents::PromptingUser(prompt),
            },
        );

        window_id
    }

    pub fn unauthorized(cx: &mut MutableAppContext) -> usize {
        let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();

        let (window_id, _) = cx.add_window(
            WindowOptions {
                bounds: gpui::WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
                titlebar: None,
                center: true,
                focus: false,
                kind: WindowKind::Normal,
                is_movable: true,
                screen: None,
            },
            |_| CopilotCodeVerification {
                prompt: SignInContents::Unauthorized,
            },
        );

        window_id
    }

    pub fn enabled(cx: &mut MutableAppContext) -> usize {
        let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();

        let (window_id, _) = cx.add_window(
            WindowOptions {
                bounds: gpui::WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
                titlebar: None,
                center: true,
                focus: false,
                kind: WindowKind::Normal,
                is_movable: true,
                screen: None,
            },
            |_| CopilotCodeVerification {
                prompt: SignInContents::Enabled,
            },
        );

        window_id
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
                            match self.prompt {
                                SignInContents::PromptingUser(_) | SignInContents::Unauthorized => {
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
                                        .boxed()
                                }
                                SignInContents::Enabled => {
                                    Label::new("Copilot Enabled!", style.auth.enable_text.clone())
                                        .boxed()
                                }
                            },
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.header_group)
                        .aligned()
                        .boxed(),
                    match &self.prompt {
                        SignInContents::PromptingUser(data) => {
                            Self::render_device_code(data, &style, cx)
                        }
                        SignInContents::Unauthorized => Self::render_not_authorized_warning(&style),
                        SignInContents::Enabled => Self::render_copilot_enabled(&style),
                    },
                    Flex::column()
                        .with_child({
                            match &self.prompt {
                                SignInContents::PromptingUser(data) => {
                                    theme::ui::cta_button_with_click(
                                        "Connect to GitHub",
                                        style.auth.content_width,
                                        &style.auth.cta_button,
                                        cx,
                                        {
                                            let verification_uri = data.verification_uri.clone();
                                            move |_, cx| cx.platform().open_url(&verification_uri)
                                        },
                                    )
                                }
                                SignInContents::Unauthorized => theme::ui::cta_button_with_click(
                                    "Close",
                                    style.auth.content_width,
                                    &style.auth.cta_button,
                                    cx,
                                    |_, cx| {
                                        let window_id = cx.window_id();
                                        cx.remove_window(window_id)
                                    },
                                ),
                                SignInContents::Enabled => theme::ui::cta_button_with_click(
                                    "Done",
                                    style.auth.content_width,
                                    &style.auth.cta_button,
                                    cx,
                                    |_, cx| {
                                        let window_id = cx.window_id();
                                        cx.remove_window(window_id)
                                    },
                                ),
                            }
                        })
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
