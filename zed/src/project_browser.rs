use gpui::{elements::Empty, Element, Entity, View};

pub struct ProjectBrowser;

pub enum Event {}

impl Entity for ProjectBrowser {
    type Event = Event;
}

impl View for ProjectBrowser {
    fn ui_name() -> &'static str {
        "ProjectBrowser"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        Empty::new().boxed()
    }
}
