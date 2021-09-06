use gpui::{Element, Entity, View, elements::{Container, Empty}};
use postage::watch;

use crate::Settings;

pub struct ProjectBrowser {
    settings: watch::Receiver<Settings>
}

impl ProjectBrowser {
    pub fn new(settings: watch::Receiver<Settings>) -> Self { 
        Self { settings }
    }
}

pub enum Event {}

impl Entity for ProjectBrowser {
    type Event = Event;
}

impl View for ProjectBrowser {
    fn ui_name() -> &'static str {
        "ProjectBrowser"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme;

        Container::new(
            Empty::new().boxed()
        )
        .with_style(&theme.project_browser.container)
        .boxed()
    }
}
