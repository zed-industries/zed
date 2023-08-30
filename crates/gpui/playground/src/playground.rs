#![allow(dead_code, unused_variables)]
use crate::element::Element;
use gpui::{
    geometry::{rect::RectF, vector::vec2f},
    platform::WindowOptions,
};
use log::LevelFilter;
use simplelog::SimpleLogger;
use themes::Theme;
use view::view;
use workspace::workspace;

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
mod workspace;

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
            |_| view(|cx| playground(Theme::default())),
        );
        cx.platform().activate(true);
    });
}

fn playground<V: 'static>(theme: Theme) -> impl Element<V> {
    workspace().themed(theme)
}
