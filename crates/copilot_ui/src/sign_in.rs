use copilot::{request::PromptUserDeviceFlow, Copilot, Status};
use gpui::{
    div, svg, AppContext, ClipboardItem, DismissEvent, Element, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, IntoElement, Model, ParentElement, Render, Styled,
    Subscription, ViewContext,
};
use ui::{prelude::*, Button, Icon, Label};
use workspace::ModalView;

const COPILOT_SIGN_UP_URL: &'static str = "https://github.com/features/copilot";

pub struct CopilotCodeVerification {
    status: Status,
    connect_clicked: bool,
    focus_handle: FocusHandle,
    _subscription: Subscription,
}

impl FocusableView for CopilotCodeVerification {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for CopilotCodeVerification {}
impl ModalView for CopilotCodeVerification {}

impl CopilotCodeVerification {
    pub(crate) fn new(copilot: &Model<Copilot>, cx: &mut ViewContext<Self>) -> Self {
        let status = copilot.read(cx).status();
        Self {
            status,
            connect_clicked: false,
            focus_handle: cx.focus_handle(),
            _subscription: cx.observe(copilot, |this, copilot, cx| {
                let status = copilot.read(cx).status();
                match status {
                    Status::Authorized | Status::Unauthorized | Status::SigningIn { .. } => {
                        this.set_status(status, cx)
                    }
                    _ => cx.emit(DismissEvent),
                }
            }),
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
            .gap_2()
            .items_center()
            .child(Headline::new("Use Github Copilot in Zed.").size(HeadlineSize::Large))
            .child(
                Label::new("Using Copilot requres an active subscription on Github.")
                    .color(Color::Muted),
            )
            .child(Self::render_device_code(data, cx))
            .child(
                Label::new("Paste this code into GitHub after clicking the button below.")
                    .size(ui::LabelSize::Small),
            )
            .child(
                Button::new("connect-button", connect_button_label)
                    .on_click({
                        let verification_uri = data.verification_uri.clone();
                        cx.listener(move |this, _, cx| {
                            cx.open_url(&verification_uri);
                            this.connect_clicked = true;
                        })
                    })
                    .full_width()
                    .style(ButtonStyle::Filled),
            )
    }
    fn render_enabled_modal(cx: &mut ViewContext<Self>) -> impl Element {
        v_stack()
            .child(Label::new("Copilot Enabled!"))
            .child(Label::new(
                "You can update your settings or sign out from the Copilot menu in the status bar.",
            ))
            .child(
                Button::new("copilot-enabled-done-button", "Done")
                    .on_click(cx.listener(|_, _, cx| cx.emit(DismissEvent))),
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
                Button::new("copilot-subscribe-button", "Subscibe on Github")
                    .on_click(|_, cx| cx.open_url(COPILOT_SIGN_UP_URL)),
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
                Self::render_enabled_modal(cx).into_any_element()
            }
            _ => div().into_any_element(),
        };

        v_stack()
            .id("copilot code verification")
            .elevation_3(cx)
            .w_96()
            .items_center()
            .p_4()
            .gap_2()
            .child(
                svg()
                    .w_32()
                    .h_16()
                    .flex_none()
                    .path(Icon::ZedXCopilot.path())
                    .text_color(cx.theme().colors().icon),
            )
            .child(prompt)
    }
}
