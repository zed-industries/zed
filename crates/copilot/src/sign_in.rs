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

                let window_size = cx
                    .global::<Settings>()
                    .theme
                    .copilot
                    .auth
                    .popup_dimensions
                    .to_vec();

                let (window_id, _) = cx.add_window(
                    WindowOptions {
                        bounds: gpui::WindowBounds::Fixed(RectF::new(
                            Default::default(),
                            window_size,
                        )),
                        titlebar: None,
                        center: true,
                        focus: false,
                        kind: WindowKind::PopUp,
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

        let instruction_text = style.auth.instruction_text;
        let user_code_text = style.auth.user_code;
        let button = style.auth.button;
        let button_width = style.auth.button_width;
        let height = style.auth.popup_dimensions.height;

        let user_code = self.prompt.user_code.replace("-", " - ");

        Flex::column()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, |state, _cx| {
                    let style = style.auth.close_icon.style_for(state, false);
                    theme::ui::icon(style).boxed()
                })
                .on_click(gpui::MouseButton::Left, move |_, cx| {
                    let window_id = cx.window_id();
                    cx.remove_window(window_id);
                })
                .with_cursor_style(gpui::CursorStyle::PointingHand)
                .aligned()
                .right()
                .boxed(),
            )
            .with_child(
                Flex::column()
                    .align_children_center()
                    .with_children([
                        theme::ui::svg(&style.auth.copilot_icon).boxed(),
                        Label::new(
                            "Here is your code to authenticate with github",
                            instruction_text.clone(),
                        )
                        .boxed(),
                        Label::new(user_code, user_code_text.clone()).boxed(),
                        theme::ui::cta_button_with_click("Copy Code", button_width, &button, cx, {
                            let user_code = self.prompt.user_code.clone();
                            move |_, cx| {
                                cx.platform()
                                    .write_to_clipboard(ClipboardItem::new(user_code.clone()))
                            }
                        }),
                        Label::new("Copy it and enter it on GitHub", instruction_text.clone())
                            .boxed(),
                        theme::ui::cta_button_with_click(
                            "Go to Github",
                            button_width,
                            &button,
                            cx,
                            {
                                let verification_uri = self.prompt.verification_uri.clone();
                                move |_, cx| cx.platform().open_url(&verification_uri)
                            },
                        ),
                    ])
                    .aligned()
                    .boxed(),
            )
            .contained()
            .with_style(style.auth.popup_container)
            .constrained()
            .with_height(height)
            .boxed()
    }
}

impl CopilotCodeVerification {
    pub fn new(prompt: PromptUserDeviceFlow) -> Self {
        CopilotCodeVerification { prompt }
    }
}
