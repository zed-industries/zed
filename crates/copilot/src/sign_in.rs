use crate::{request::PromptUserDeviceFlow, Copilot, Status};
use gpui::{
    elements::*,
    geometry::rect::RectF,
    impl_internal_actions,
    platform::{WindowBounds, WindowKind, WindowOptions},
    AnyElement, AnyViewHandle, AppContext, ClipboardItem, Element, Entity, View, ViewContext,
    ViewHandle,
};
use settings::Settings;
use theme::ui::modal;

#[derive(PartialEq, Eq, Debug, Clone)]
struct ClickedConnect;

impl_internal_actions!(copilot_verification, [ClickedConnect]);

#[derive(PartialEq, Eq, Debug, Clone)]
struct CopyUserCode;

#[derive(PartialEq, Eq, Debug, Clone)]
struct OpenGithub;

const COPILOT_SIGN_UP_URL: &'static str = "https://github.com/features/copilot";

pub fn init(cx: &mut AppContext) {
    if let Some(copilot) = Copilot::global(cx) {
        let mut code_verification: Option<ViewHandle<CopilotCodeVerification>> = None;
        cx.observe(&copilot, move |copilot, cx| {
            let status = copilot.read(cx).status();

            match &status {
                crate::Status::SigningIn { prompt } => {
                    if let Some(code_verification_handle) = code_verification.as_mut() {
                        let window_id = code_verification_handle.window_id();
                        let updated = cx.update_window(window_id, |cx| {
                            code_verification_handle.update(cx, |code_verification, cx| {
                                code_verification.set_status(status.clone(), cx)
                            });
                            cx.activate_window();
                        });
                        if updated.is_none() {
                            code_verification = Some(create_copilot_auth_window(cx, &status));
                        }
                    } else if let Some(_prompt) = prompt {
                        code_verification = Some(create_copilot_auth_window(cx, &status));
                    }
                }
                Status::Authorized | Status::Unauthorized => {
                    if let Some(code_verification) = code_verification.as_ref() {
                        let window_id = code_verification.window_id();
                        cx.update_window(window_id, |cx| {
                            code_verification.update(cx, |code_verification, cx| {
                                code_verification.set_status(status, cx)
                            });

                            cx.platform().activate(true);
                            cx.activate_window();
                        });
                    }
                }
                _ => {
                    if let Some(code_verification) = code_verification.take() {
                        cx.update_window(code_verification.window_id(), |cx| cx.remove_window());
                    }
                }
            }
        })
        .detach();

        cx.add_action(
            |code_verification: &mut CopilotCodeVerification, _: &ClickedConnect, _| {
                code_verification.connect_clicked = true;
            },
        );
    }
}

fn create_copilot_auth_window(
    cx: &mut AppContext,
    status: &Status,
) -> ViewHandle<CopilotCodeVerification> {
    let window_size = cx.global::<Settings>().theme.copilot.modal.dimensions();
    let window_options = WindowOptions {
        bounds: WindowBounds::Fixed(RectF::new(Default::default(), window_size)),
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
    view
}

pub struct CopilotCodeVerification {
    status: Status,
    connect_clicked: bool,
}

impl CopilotCodeVerification {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            connect_clicked: false,
        }
    }

    pub fn set_status(&mut self, status: Status, cx: &mut ViewContext<Self>) {
        self.status = status;
        cx.notify();
    }

    fn render_device_code(
        data: &PromptUserDeviceFlow,
        style: &theme::Copilot,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text() == &data.user_code)
            .unwrap_or(false);

        let device_code_style = &style.auth.prompting.device_code;

        MouseEventHandler::<Self, _>::new(0, cx, |state, _cx| {
            Flex::row()
                .with_child(
                    Label::new(data.user_code.clone(), device_code_style.text.clone())
                        .aligned()
                        .contained()
                        .with_style(device_code_style.left_container)
                        .constrained()
                        .with_width(device_code_style.left),
                )
                .with_child(
                    Label::new(
                        if copied { "Copied!" } else { "Copy" },
                        device_code_style.cta.style_for(state, false).text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(*device_code_style.right_container.style_for(state, false))
                    .constrained()
                    .with_width(device_code_style.right),
                )
                .contained()
                .with_style(device_code_style.cta.style_for(state, false).container)
        })
        .on_click(gpui::platform::MouseButton::Left, {
            let user_code = data.user_code.clone();
            move |_, _, cx| {
                cx.platform()
                    .write_to_clipboard(ClipboardItem::new(user_code.clone()));
                cx.notify();
            }
        })
        .with_cursor_style(gpui::platform::CursorStyle::PointingHand)
    }

    fn render_prompting_modal(
        connect_clicked: bool,
        data: &PromptUserDeviceFlow,
        style: &theme::Copilot,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum ConnectButton {}

        Flex::column()
            .with_child(
                Flex::column()
                    .with_children([
                        Label::new(
                            "Enable Copilot by connecting",
                            style.auth.prompting.subheading.text.clone(),
                        )
                        .aligned(),
                        Label::new(
                            "your existing license.",
                            style.auth.prompting.subheading.text.clone(),
                        )
                        .aligned(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.prompting.subheading.container),
            )
            .with_child(Self::render_device_code(data, &style, cx))
            .with_child(
                Flex::column()
                    .with_children([
                        Label::new(
                            "Paste this code into GitHub after",
                            style.auth.prompting.hint.text.clone(),
                        )
                        .aligned(),
                        Label::new(
                            "clicking the button below.",
                            style.auth.prompting.hint.text.clone(),
                        )
                        .aligned(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(style.auth.prompting.hint.container.clone()),
            )
            .with_child(theme::ui::cta_button_with_click::<ConnectButton, _, _, _>(
                if connect_clicked {
                    "Waiting for connection..."
                } else {
                    "Connect to GitHub"
                },
                style.auth.content_width,
                &style.auth.cta_button,
                cx,
                {
                    let verification_uri = data.verification_uri.clone();
                    move |_, _, cx| {
                        cx.platform().open_url(&verification_uri);
                        cx.dispatch_action(ClickedConnect)
                    }
                },
            ))
            .align_children_center()
            .into_any()
    }

    fn render_enabled_modal(
        style: &theme::Copilot,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum DoneButton {}

        let enabled_style = &style.auth.authorized;
        Flex::column()
            .with_child(
                Label::new("Copilot Enabled!", enabled_style.subheading.text.clone())
                    .contained()
                    .with_style(enabled_style.subheading.container)
                    .aligned(),
            )
            .with_child(
                Flex::column()
                    .with_children([
                        Label::new(
                            "You can update your settings or",
                            enabled_style.hint.text.clone(),
                        )
                        .aligned(),
                        Label::new(
                            "sign out from the Copilot menu in",
                            enabled_style.hint.text.clone(),
                        )
                        .aligned(),
                        Label::new("the status bar.", enabled_style.hint.text.clone()).aligned(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(enabled_style.hint.container),
            )
            .with_child(theme::ui::cta_button_with_click::<DoneButton, _, _, _>(
                "Done",
                style.auth.content_width,
                &style.auth.cta_button,
                cx,
                |_, _, cx| cx.remove_window(),
            ))
            .align_children_center()
            .into_any()
    }

    fn render_unauthorized_modal(
        style: &theme::Copilot,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let unauthorized_style = &style.auth.not_authorized;

        Flex::column()
            .with_child(
                Flex::column()
                    .with_children([
                        Label::new(
                            "Enable Copilot by connecting",
                            unauthorized_style.subheading.text.clone(),
                        )
                        .aligned(),
                        Label::new(
                            "your existing license.",
                            unauthorized_style.subheading.text.clone(),
                        )
                        .aligned(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(unauthorized_style.subheading.container),
            )
            .with_child(
                Flex::column()
                    .with_children([
                        Label::new(
                            "You must have an active copilot",
                            unauthorized_style.warning.text.clone(),
                        )
                        .aligned(),
                        Label::new(
                            "license to use it in Zed.",
                            unauthorized_style.warning.text.clone(),
                        )
                        .aligned(),
                    ])
                    .align_children_center()
                    .contained()
                    .with_style(unauthorized_style.warning.container),
            )
            .with_child(theme::ui::cta_button_with_click::<Self, _, _, _>(
                "Subscribe on GitHub",
                style.auth.content_width,
                &style.auth.cta_button,
                cx,
                |_, _, cx| {
                    cx.remove_window();
                    cx.platform().open_url(COPILOT_SIGN_UP_URL)
                },
            ))
            .align_children_center()
            .into_any()
    }
}

impl Entity for CopilotCodeVerification {
    type Event = ();
}

impl View for CopilotCodeVerification {
    fn ui_name() -> &'static str {
        "CopilotCodeVerification"
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        cx.notify()
    }

    fn focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        cx.notify()
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum ConnectModal {}

        let style = cx.global::<Settings>().theme.clone();

        modal::<ConnectModal, _, _, _, _>(
            "Connect Copilot to Zed",
            &style.copilot.modal,
            cx,
            |cx| {
                Flex::column()
                    .with_children([
                        theme::ui::icon(&style.copilot.auth.header).into_any(),
                        match &self.status {
                            Status::SigningIn {
                                prompt: Some(prompt),
                            } => Self::render_prompting_modal(
                                self.connect_clicked,
                                &prompt,
                                &style.copilot,
                                cx,
                            ),
                            Status::Unauthorized => {
                                self.connect_clicked = false;
                                Self::render_unauthorized_modal(&style.copilot, cx)
                            }
                            Status::Authorized => {
                                self.connect_clicked = false;
                                Self::render_enabled_modal(&style.copilot, cx)
                            }
                            _ => Empty::new().into_any(),
                        },
                    ])
                    .align_children_center()
            },
        )
        .into_any()
    }
}
