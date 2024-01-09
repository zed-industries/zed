use crate::{request::PromptUserDeviceFlow, Copilot, Status};
use gpui::{
    div, size, AppContext, Bounds, ClipboardItem, Element, GlobalPixels, InteractiveElement,
    IntoElement, ParentElement, Point, Render, Styled, ViewContext, VisualContext, WindowBounds,
    WindowHandle, WindowKind, WindowOptions,
};
use theme::ActiveTheme;
use ui::{prelude::*, Button, Icon, IconPath, Label};

const COPILOT_SIGN_UP_URL: &'static str = "https://github.com/features/copilot";

pub fn init(cx: &mut AppContext) {
    if let Some(copilot) = Copilot::global(cx) {
        let mut verification_window: Option<WindowHandle<CopilotCodeVerification>> = None;
        cx.observe(&copilot, move |copilot, cx| {
            let status = copilot.read(cx).status();

            match &status {
                crate::Status::SigningIn { prompt } => {
                    if let Some(window) = verification_window.as_mut() {
                        let updated = window
                            .update(cx, |verification, cx| {
                                verification.set_status(status.clone(), cx);
                                cx.activate_window();
                            })
                            .is_ok();
                        if !updated {
                            verification_window = Some(create_copilot_auth_window(cx, &status));
                        }
                    } else if let Some(_prompt) = prompt {
                        verification_window = Some(create_copilot_auth_window(cx, &status));
                    }
                }
                Status::Authorized | Status::Unauthorized => {
                    if let Some(window) = verification_window.as_ref() {
                        window
                            .update(cx, |verification, cx| {
                                verification.set_status(status, cx);
                                cx.activate(true);
                                cx.activate_window();
                            })
                            .ok();
                    }
                }
                _ => {
                    if let Some(code_verification) = verification_window.take() {
                        code_verification
                            .update(cx, |_, cx| cx.remove_window())
                            .ok();
                    }
                }
            }
        })
        .detach();
    }
}

fn create_copilot_auth_window(
    cx: &mut AppContext,
    status: &Status,
) -> WindowHandle<CopilotCodeVerification> {
    let window_size = size(GlobalPixels::from(280.), GlobalPixels::from(280.));
    let window_options = WindowOptions {
        bounds: WindowBounds::Fixed(Bounds::new(Point::default(), window_size)),
        titlebar: None,
        center: true,
        focus: true,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: true,
        display_id: None,
    };
    let window = cx.open_window(window_options, |cx| {
        cx.new_view(|_| CopilotCodeVerification::new(status.clone()))
    });
    window
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
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text() == &data.user_code)
            .unwrap_or(false);
        h_stack()
            .cursor_pointer()
            .justify_between()
            .on_mouse_down(gpui::MouseButton::Left, {
                let user_code = data.user_code.clone();
                move |_, cx| {
                    cx.write_to_clipboard(ClipboardItem::new(user_code.clone()));
                    cx.notify();
                }
            })
            .child(Label::new(data.user_code.clone()))
            .child(div())
            .child(Label::new(if copied { "Copied!" } else { "Copy" }))
    }

    fn render_prompting_modal(
        connect_clicked: bool,
        data: &PromptUserDeviceFlow,
        cx: &mut ViewContext<Self>,
    ) -> impl Element {
        let connect_button_label = if connect_clicked {
            "Waiting for connection..."
        } else {
            "Connect to Github"
        };
        v_stack()
            .flex_1()
            .items_center()
            .justify_between()
            .w_full()
            .child(Label::new(
                "Enable Copilot by connecting your existing license",
            ))
            .child(Self::render_device_code(data, cx))
            .child(
                Label::new("Paste this code into GitHub after clicking the button below.")
                    .size(ui::LabelSize::Small),
            )
            .child(
                Button::new("connect-button", connect_button_label).on_click({
                    let verification_uri = data.verification_uri.clone();
                    cx.listener(move |this, _, cx| {
                        cx.open_url(&verification_uri);
                        this.connect_clicked = true;
                    })
                }),
            )
    }
    fn render_enabled_modal() -> impl Element {
        v_stack()
            .child(Label::new("Copilot Enabled!"))
            .child(Label::new(
                "You can update your settings or sign out from the Copilot menu in the status bar.",
            ))
            .child(
                Button::new("copilot-enabled-done-button", "Done")
                    .on_click(|_, cx| cx.remove_window()),
            )
    }

    fn render_unauthorized_modal() -> impl Element {
        v_stack()
            .child(Label::new(
                "Enable Copilot by connecting your existing license.",
            ))
            .child(
                Label::new("You must have an active Copilot license to use it in Zed.")
                    .color(Color::Warning),
            )
            .child(
                Button::new("copilot-subscribe-button", "Subscibe on Github").on_click(|_, cx| {
                    cx.remove_window();
                    cx.open_url(COPILOT_SIGN_UP_URL)
                }),
            )
    }
}

impl Render for CopilotCodeVerification {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let prompt = match &self.status {
            Status::SigningIn {
                prompt: Some(prompt),
            } => Self::render_prompting_modal(self.connect_clicked, &prompt, cx).into_any_element(),
            Status::Unauthorized => {
                self.connect_clicked = false;
                Self::render_unauthorized_modal().into_any_element()
            }
            Status::Authorized => {
                self.connect_clicked = false;
                Self::render_enabled_modal().into_any_element()
            }
            _ => div().into_any_element(),
        };
        div()
            .id("copilot code verification")
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .p_10()
            .bg(cx.theme().colors().element_background)
            .child(ui::Label::new("Connect Copilot to Zed"))
            .child(Icon::new(IconPath::ZedXCopilot))
            .child(prompt)
    }
}
