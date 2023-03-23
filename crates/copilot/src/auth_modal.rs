use gpui::{elements::Label, Element, Entity, View};
use settings::Settings;

pub struct AuthModal {}

impl Entity for AuthModal {
    type Event = ();
}

impl View for AuthModal {
    fn ui_name() -> &'static str {
        "AuthModal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let style = &cx.global::<Settings>().theme.copilot;

        Label::new("[COPILOT AUTH INFO]", style.auth_modal.clone()).boxed()
    }
}
