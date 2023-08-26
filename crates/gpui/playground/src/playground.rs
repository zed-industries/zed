#![allow(dead_code, unused_variables)]
use crate::{
    color::black, element::ParentElement, style::StyleHelpers, themes::rose_pine::RosePinePalette,
};
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
mod interactive;
mod layout_context;
mod paint_context;
mod pressable;
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
    let p = RosePinePalette::dawn();

    div()
        .text_color(black())
        .h_full()
        .w_full()
        .fill(p.rose)
        .block()
        .child(
            div()
                .block()
                .fill(p.pine)
                .child(div().block().fill(p.love).w_6().h_3()),
        )
        .child(
            div()
                .block()
                .fill(p.gold)
                .child(div().block().fill(p.iris).w_3().h_3()),
        )
}
