use elements::{Atom, AtomStyle};
use gpui::{color::Color, AnyElement, Element, Entity, View};
use log::LevelFilter;
use simplelog::SimpleLogger;

mod elements;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(Default::default(), |_| PlaygroundView);
    });
}

struct PlaygroundView;

impl Entity for PlaygroundView {
    type Event = ();
}

impl View for PlaygroundView {
    fn ui_name() -> &'static str {
        "PlaygroundView"
    }

    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> AnyElement<PlaygroundView> {
        Atom::new(
            AtomStyle::default()
                .width(100.)
                .height(100.)
                .fill(Color::red()),
        )
        .into_any()
    }
}
