use crate::{request::PromptUserDeviceFlow, Copilot, Status};
use gpui::{
    elements::*, geometry::rect::RectF, ClipboardItem, Element, Entity, MutableAppContext, View,
    ViewContext, ViewHandle, WindowKind, WindowOptions,
};
use settings::Settings;
use theme::ui::modal;

#[derive(PartialEq, Eq, Debug, Clone)]
struct CopyUserCode;

#[derive(PartialEq, Eq, Debug, Clone)]
struct OpenGithub;

const COPILOT_SIGN_UP_URL: &'static str = "https://github.com/features/copilot";

pub fn init(cx: &mut MutableAppContext) {
    let copilot = Copilot::global(cx).unwrap();

    let mut code_verification: Option<ViewHandle<CopilotCodeVerification>> = None;
    cx.observe(&copilot, move |copilot, cx| {
        let status = copilot.read(cx).status();

        match &status {
            crate::Status::SigningIn { prompt } => {
                if let Some(code_verification_handle) = code_verification.as_mut() {
                    if cx.has_window(code_verification_handle.window_id()) {
                        code_verification_handle.update(cx, |code_verification_view, cx| {
                            code_verification_view.set_status(status, cx)
                        });
                        cx.activate_window(code_verification_handle.window_id());
                    } else {
                        create_copilot_auth_window(cx, &status, &mut code_verification);
                    }
                } else if let Some(_prompt) = prompt {
                    create_copilot_auth_window(cx, &status, &mut code_verification);
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
}

fn create_copilot_auth_window(
    cx: &mut MutableAppContext,
    status: &Status,
    code_verification: &mut Option<ViewHandle<CopilotCodeVerification>>,
) {
    let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();
    let window_options = WindowOptions {
        bounds: gpui::WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
        titlebar: None,
        center: true,
        focus: true,
        kind: WindowKind::Normal,
        is_movable: true,
        screen: None,
    };
    let (_, view) = cx.add_window(window_options, |_cx| {
        CopilotCodeVerification::new(status.clone())
    });
    *code_verification = Some(view);
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

        let device_code_style = &style.auth.prompting.device_code;

        MouseEventHandler::<Self>::new(0, cx, |state, _cx| {
            Flex::row()
                .with_children([
                    Label::new(data.user_code.clone(), device_code_style.text.clone())
                        .aligned()
                        .contained()
                        .with_style(device_code_style.left_container)
                        .constrained()
                        .with_width(device_code_style.left)
                        .boxed(),
                    Label::new(
                        if copied { "Copied!" } else { "Copy" },
                        device_code_style.cta.style_for(state, false).text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(*device_code_style.right_container.style_for(state, false))
                    .constrained()
                    .with_width(device_code_style.right)
                    .boxed(),
                ])
                .contained()
                .with_style(device_code_style.cta.style_for(state, false).container)
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
        .boxed()
    }

    fn render_prompting_modal(
        data: &PromptUserDeviceFlow,
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        Flex::column()
            .with_children([
                Flex::column()
                    .with_children([
                        Label::new(
                            "Enable Copilot by connecting",
                            style.auth.prompting.subheading.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "your existing license.",
                            style.auth.prompting.subheading.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.prompting.subheading.container)
                    .boxed(),
                Self::render_device_code(data, &style, cx),
                Flex::column()
                    .with_children([
                        Label::new(
                            "Paste this code into GitHub after",
                            style.auth.prompting.hint.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "clicking the button below.",
                            style.auth.prompting.hint.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.prompting.hint.container.clone())
                    .boxed(),
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
                .boxed(),
            ])
            .align_children_center()
            .boxed()
    }
    fn render_enabled_modal(
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        let enabled_style = &style.auth.authorized;
        Flex::column()
            .with_children([
                Label::new("Copilot Enabled!", enabled_style.subheading.text.clone())
                    .contained()
                    .with_style(enabled_style.subheading.container)
                    .aligned()
                    .boxed(),
                Flex::column()
                    .with_children([
                        Label::new(
                            "You can update your settings or",
                            enabled_style.hint.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "sign out from the Copilot menu in",
                            enabled_style.hint.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new("the status bar.", enabled_style.hint.text.clone())
                            .aligned()
                            .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(enabled_style.hint.container)
                    .boxed(),
                theme::ui::cta_button_with_click(
                    "Done",
                    style.auth.content_width,
                    &style.auth.cta_button,
                    cx,
                    |_, cx| {
                        let window_id = cx.window_id();
                        cx.remove_window(window_id)
                    },
                )
                .boxed(),
            ])
            .align_children_center()
            .boxed()
    }
    fn render_unauthorized_modal(
        style: &theme::Copilot,
        cx: &mut gpui::RenderContext<Self>,
    ) -> ElementBox {
        let unauthorized_style = &style.auth.not_authorized;

        Flex::column()
            .with_children([
                Flex::column()
                    .with_children([
                        Label::new(
                            "Enable Copilot by connecting",
                            unauthorized_style.subheading.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "your existing license.",
                            unauthorized_style.subheading.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(unauthorized_style.subheading.container)
                    .boxed(),
                Flex::column()
                    .with_children([
                        Label::new(
                            "You must have an active copilot",
                            unauthorized_style.warning.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                        Label::new(
                            "license to use it in Zed.",
                            unauthorized_style.warning.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(unauthorized_style.warning.container)
                    .boxed(),
                theme::ui::cta_button_with_click(
                    "Subscribe on GitHub",
                    style.auth.content_width,
                    &style.auth.cta_button,
                    cx,
                    |_, cx| {
                        let window_id = cx.window_id();
                        cx.remove_window(window_id);
                        cx.platform().open_url(COPILOT_SIGN_UP_URL)
                    },
                )
                .boxed(),
            ])
            .align_children_center()
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
        let style = cx.global::<Settings>().theme.clone();

        modal("Connect Copilot to Zed", &style.copilot.modal, cx, |cx| {
            Flex::column()
                .with_children([
                    theme::ui::icon(&style.copilot.auth.header).boxed(),
                    match &self.status {
                        Status::SigningIn {
                            prompt: Some(prompt),
                        } => Self::render_prompting_modal(&prompt, &style.copilot, cx),
                        Status::Unauthorized => Self::render_unauthorized_modal(&style.copilot, cx),
                        Status::Authorized => Self::render_enabled_modal(&style.copilot, cx),
                        _ => Empty::new().boxed(),
                    },
                ])
                .align_children_center()
                .boxed()
        })
    }
}
