#![allow(dead_code, unused_variables)]
use element::{AnyElement, Element};
use frame::frame;
use log::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.add_window(Default::default(), |_| {
            view(|_| workspace(&rose_pine::moon()))
        });
        cx.platform().activate(true);
    });
}

use themes::{rose_pine, ThemeColors};
use view::view;

mod adapter;
mod color;
mod element;
mod frame;
mod style;
mod themes;
mod view;

pub struct Playground<V: 'static>(AnyElement<V>);

impl<V> Playground<V> {
    pub fn new() -> Self {
        Self(workspace(&rose_pine::moon()).into_any())
    }
}

fn workspace<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    frame()
}
//     todo!()
//     // column()
//     // .size(auto())
//     // .fill(theme.base(0.5))
//     // .text_color(theme.text(0.5))
//     // .child(title_bar(theme))
//     // .child(stage(theme))
//     // .child(status_bar(theme))
// }

// fn title_bar<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
//     row()
//         .fill(theme.base(0.2))
//         .justify(0.)
//         .width(auto())
//         .child(text("Zed Playground"))
// }

// fn stage<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
//     row().fill(theme.surface(0.9))
// }

// fn status_bar<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
//     row().fill(theme.surface(0.1))
// }
