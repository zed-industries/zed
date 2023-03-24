use gpui::{
    elements::{Flex, Label, MouseEventHandler, ParentElement, Stack},
    Axis, Element, Entity, View, ViewContext,
};
use settings::Settings;

use crate::{Copilot, PromptingUser};

pub enum Event {
    Dismiss,
}

pub struct AuthModal {}

impl Entity for AuthModal {
    type Event = Event;
}

impl View for AuthModal {
    fn ui_name() -> &'static str {
        "AuthModal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let style = cx.global::<Settings>().theme.copilot.clone();

        let user_code_and_url = Copilot::global(cx).read(cx).prompting_user().cloned();
        let auth_text = style.auth_text.clone();
        MouseEventHandler::<AuthModal>::new(0, cx, move |_state, cx| {
            Stack::new()
                .with_child(match user_code_and_url {
                    Some(PromptingUser {
                        user_code,
                        verification_uri,
                    }) => Flex::new(Axis::Vertical)
                        .with_children([
                            Label::new(user_code, auth_text.clone())
                                .constrained()
                                .with_width(540.)
                                .boxed(),
                            MouseEventHandler::<AuthModal>::new(1, cx, move |_state, _cx| {
                                Label::new("Click here to open github!", auth_text.clone())
                                    .constrained()
                                    .with_width(540.)
                                    .boxed()
                            })
                            .on_click(gpui::MouseButton::Left, move |_click, cx| {
                                cx.platform().open_url(&verification_uri)
                            })
                            .with_cursor_style(gpui::CursorStyle::PointingHand)
                            .boxed(),
                        ])
                        .boxed(),
                    None => Label::new("Not signing in", style.auth_text.clone())
                        .constrained()
                        .with_width(540.)
                        .boxed(),
                })
                .contained()
                .with_style(style.auth_modal)
                .constrained()
                .with_max_width(540.)
                .with_max_height(420.)
                .named("Copilot Authentication status modal")
        })
        .on_hover(|_, _| {})
        .on_click(gpui::MouseButton::Left, |_, _| {})
        .on_click(gpui::MouseButton::Left, |_, _| {})
        .boxed()
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismiss)
    }
}

impl AuthModal {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&Copilot::global(cx), |_, _, cx| cx.notify())
            .detach();

        AuthModal {}
    }
}
