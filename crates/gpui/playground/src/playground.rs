#![allow(dead_code, unused_variables)]
use crate::{color::black, style::StyleHelpers};
use element::Element;
use gpui::{
    geometry::{rect::RectF, vector::vec2f},
    platform::WindowOptions,
};
use log::LevelFilter;
use simplelog::SimpleLogger;
use themes::{rose_pine, ThemeColors};
use view::view;

mod adapter;
mod color;
mod components;
mod div;
mod element;
mod hoverable;
mod layout_context;
mod paint_context;
mod style;
mod text;
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
            |_| view(|_| playground(&rose_pine::moon())),
        );
        cx.platform().activate(true);
    });
}

fn playground<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    use div::div;

    div()
        .text_color(black())
        .h_full()
        .w_1_2()
        .fill(theme.success(0.5))
    // .hover()
    // .fill(theme.error(0.5))
    // .child(button().label("Hello").click(|_, _, _| println!("click!")))
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
