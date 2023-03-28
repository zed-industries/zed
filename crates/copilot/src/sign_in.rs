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

pub fn init(cx: &mut MutableAppContext) {
    let copilot = Copilot::global(cx).unwrap();

    let mut code_verification_window_id = None;
    cx.observe(&copilot, move |copilot, cx| {
        match copilot.read(cx).status() {
            crate::Status::SigningIn {
                prompt: Some(prompt),
            } => {
                if let Some(window_id) = code_verification_window_id.take() {
                    cx.remove_window(window_id);
                }

                let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();

                let (window_id, _) = cx.add_window(
                    WindowOptions {
                        bounds: gpui::WindowBounds::Fixed(RectF::new(
                            Default::default(),
                            window_size,
                        )),
                        titlebar: None,
                        center: true,
                        focus: false,
                        kind: WindowKind::Normal,
                        is_movable: true,
                        screen: None,
                    },
                    |_| CopilotCodeVerification::new(prompt),
                );
                code_verification_window_id = Some(window_id);

                cx.activate_window(window_id);
            }
            _ => {
                if let Some(window_id) = code_verification_window_id.take() {
                    cx.remove_window(window_id);
                }
            }
        }
    })
    .detach();

    // let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();

    // let (_window_id, _) = cx.add_window(
    //     WindowOptions {
    //         bounds: gpui::WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
    //         titlebar: None,
    //         center: true,
    //         focus: false,
    //         kind: WindowKind::PopUp,
    //         is_movable: true,
    //         screen: None,
    //     },
    //     |_| {
    //         CopilotCodeVerification::new(PromptUserDeviceFlow {
    //             user_code: "ABCD-1234".to_string(),
    //             verification_uri: "https://github.com/login/device".to_string(),
    //         })
    //     },
    // );
}

pub struct CopilotCodeVerification {
    prompt: PromptUserDeviceFlow,
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

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let style = cx.global::<Settings>().theme.copilot.clone();

        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text() == &self.prompt.user_code)
            .unwrap_or(false);

        theme::ui::modal("Authenticate Copilot", &style.modal, cx, |cx| {
            Flex::column()
                .align_children_center()
                .with_children([
                    Flex::column()
                        .with_children([
                            Flex::row()
                                .with_children([
                                    theme::ui::svg(&style.auth.copilot_icon).boxed(),
                                    theme::ui::svg(&style.auth.plus_icon).boxed(),
                                    theme::ui::svg(&style.auth.zed_icon).boxed(),
                                ])
                                .boxed(),
                            Label::new("Copilot for Zed", style.auth.header_text.clone()).boxed(),
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.header_group)
                        .aligned()
                        .boxed(),
                    Flex::column()
                        .with_children([
                            Label::new(
                                "Here is your code to authenticate with github",
                                style.auth.instruction_text.clone(),
                            )
                            .boxed(),
                            MouseEventHandler::<Self>::new(0, cx, |state, _cx| {
                                Flex::row()
                                    .with_children([
                                        Label::new(
                                            self.prompt.user_code.clone(),
                                            style.auth.device_code.clone(),
                                        )
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
                                            style
                                                .auth
                                                .cta_button
                                                .style_for(state, false)
                                                .text
                                                .clone(),
                                        )
                                        .aligned()
                                        .contained()
                                        .with_style(style.auth.device_code_right_container)
                                        .constrained()
                                        .with_width(style.auth.device_code_right)
                                        .boxed(),
                                    ])
                                    .contained()
                                    .with_style(
                                        style
                                            .auth
                                            .device_code_cta
                                            .style_for(state, false)
                                            .container,
                                    )
                                    .constrained()
                                    .with_width(style.auth.content_width)
                                    .boxed()
                            })
                            .on_click(gpui::MouseButton::Left, {
                                let user_code = self.prompt.user_code.clone();
                                move |_, cx| {
                                    cx.platform()
                                        .write_to_clipboard(ClipboardItem::new(user_code.clone()));
                                    cx.notify();
                                }
                            })
                            .with_cursor_style(gpui::CursorStyle::PointingHand)
                            .boxed(),
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.device_code_group)
                        .aligned()
                        .boxed(),
                    Flex::column()
                        .with_children([
                            Label::new(
                                "Copy it and enter it on GitHub",
                                style.auth.instruction_text.clone(),
                            )
                            .boxed(),
                            theme::ui::cta_button_with_click(
                                "Go to Github",
                                style.auth.content_width,
                                &style.auth.cta_button,
                                cx,
                                {
                                    let verification_uri = self.prompt.verification_uri.clone();
                                    move |_, cx| cx.platform().open_url(&verification_uri)
                                },
                            ),
                        ])
                        .align_children_center()
                        .contained()
                        .with_style(style.auth.github_group)
                        .aligned()
                        .boxed(),
                ])
                .constrained()
                .with_width(style.auth.content_width)
                .aligned()
                .boxed()
        })
    }
}

impl CopilotCodeVerification {
    pub fn new(prompt: PromptUserDeviceFlow) -> Self {
        CopilotCodeVerification { prompt }
    }
}
