use elements::{Node, NodeStyle};
use gpui::{color::Color, AnyElement, Element, Entity, View, ViewContext};
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

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> AnyElement<PlaygroundView> {
        // Node::with_style(NodeStyle)
        // Node::new().width(100.0).fill(Color::red())
        //
        Node::new()
            .width(100.)
            .height(100.)
            .fill(Color::red())
            .children([
                Node::new().width(20.).height(20.).fill(Color::green()),
                Node::new().width(20.).height(20.).fill(Color::blue()),
            ])
            .into_any()

        // Node::with_style(
        //     NodeStyle::default()
        //         .width(100.)
        //         .height(100.)
        //         .fill(Color::red()),
        // )
        // .into_any()
    }
}
