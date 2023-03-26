use crate::{request::PromptUserDeviceFlow, Copilot};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Axis, Element, Entity, MutableAppContext, View, WindowKind, WindowOptions,
};
use settings::Settings;

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

                let (window_id, _) = cx.add_window(
                    WindowOptions {
                        bounds: gpui::WindowBounds::Fixed(RectF::new(
                            Default::default(),
                            vec2f(600., 400.),
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
}

impl CopilotCodeVerification {
    pub fn new(prompt: PromptUserDeviceFlow) -> Self {
        CopilotCodeVerification { prompt }
    }
}
