use std::ops::{Deref, DerefMut};

use gpui::{AnyElement, Element, Entity, View};
use log::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(Default::default(), |_| Playground::default());
    });
}

#[derive(Clone, Default)]
struct Playground(playground_ui::Playground<Self>);

impl Deref for Playground {
    type Target = playground_ui::Playground<Self>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Playground {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Entity for Playground {
    type Event = ();
}

impl View for Playground {
    fn ui_name() -> &'static str {
        "PlaygroundView"
    }

    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> AnyElement<Playground> {
        self.0.clone().into_any()
    }
}
