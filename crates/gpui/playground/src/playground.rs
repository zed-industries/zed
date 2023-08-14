#![allow(dead_code, unused_variables)]
use element::Element;
use frame::frame;
use gpui::{
    geometry::{rect::RectF, vector::vec2f},
    platform::WindowOptions,
};
use log::LevelFilter;
use simplelog::SimpleLogger;

use style::percent;
use themes::{rose_pine, ThemeColors};
use view::view;

mod adapter;
mod color;
mod element;
mod frame;
mod style;
mod themes;
mod view;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.add_window(
            WindowOptions {
                bounds: gpui::platform::WindowBounds::Fixed(RectF::new(
                    vec2f(0., 0.),
                    vec2f(400., 300.),
                )),
                center: true,
                ..Default::default()
            },
            |_| view(|_| workspace(&rose_pine::moon())),
        );
        cx.platform().activate(true);
    });
}

fn workspace<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    // frame().w_full().h_half().fill(theme.success(0.5))
    frame().h_full().w(percent(50.)).fill(theme.success(0.5))
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
