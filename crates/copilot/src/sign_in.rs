use crate::{request::PromptUserDeviceFlow, Copilot};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Axis, Element, Entity, MutableAppContext, View, ViewContext, WindowKind, WindowOptions,
};
use settings::Settings;

pub fn init(cx: &mut MutableAppContext) {
    let copilot = Copilot::global(cx);

    let mut code_verification_window_id = None;
    cx.observe(&copilot, move |copilot, cx| {
        match copilot.read(cx).status() {
            crate::Status::SigningIn {
                prompt: Some(prompt),
            } => {
                if let Some(window_id) = code_verification_window_id.take() {
                    cx.remove_window(window_id);
                }

                let screen = cx.platform().screens().pop();
                let (window_id, _) = cx.add_window(
                    WindowOptions {
                        bounds: gpui::WindowBounds::Fixed(RectF::new(
                            vec2f(100., 100.),
                            vec2f(300., 300.),
                        )),
                        titlebar: None,
                        center: false,
                        focus: false,
                        kind: WindowKind::Normal,
                        is_movable: true,
                        screen,
                    },
                    |_| CopilotCodeVerification::new(prompt),
                );
                code_verification_window_id = Some(window_id);
            }
            _ => {
                if let Some(window_id) = code_verification_window_id.take() {
                    cx.remove_window(window_id);
                }
            }
        }
    })
    .detach();
}

pub enum Event {
    Dismiss,
}

pub struct CopilotCodeVerification {
    prompt: PromptUserDeviceFlow,
}

impl Entity for CopilotCodeVerification {
    type Event = Event;
}

impl View for CopilotCodeVerification {
    fn ui_name() -> &'static str {
        "CopilotCodeVerification"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let style = cx.global::<Settings>().theme.copilot.clone();

        let auth_text = style.auth_text.clone();
        let prompt = self.prompt.clone();
        Flex::new(Axis::Vertical)
            .with_child(Label::new(prompt.user_code.clone(), auth_text.clone()).boxed())
            .with_child(
                MouseEventHandler::<Self>::new(1, cx, move |_state, _cx| {
                    Label::new("Click here to open GitHub!", auth_text.clone()).boxed()
                })
                .on_click(gpui::MouseButton::Left, move |_click, cx| {
                    cx.platform().open_url(&prompt.verification_uri)
                })
                .with_cursor_style(gpui::CursorStyle::PointingHand)
                .boxed(),
            )
            .contained()
            .with_style(style.auth_modal)
            .named("Copilot Authentication status modal")
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismiss)
    }
}

impl CopilotCodeVerification {
    pub fn new(prompt: PromptUserDeviceFlow) -> Self {
        CopilotCodeVerification { prompt }
    }
}
