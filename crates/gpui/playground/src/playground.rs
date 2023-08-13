#![allow(dead_code, unused_variables)]

use gpui::{elements::Empty, Element};
use log::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        // cx.add_window(
        //     WindowOptions {
        //         titlebar: Some(TitlebarOptions {
        //             appears_transparent: true,
        //             ..Default::default()
        //         }),
        //         ..Default::default()
        //     },
        //     |_| view(|_| Playground::new()),
        // );
    });
}

use std::marker::PhantomData;
use themes::ThemeColors;

mod color;
mod frame;
mod style;
mod themes;
mod tokens;

#[derive(Element, Clone)]
pub struct Playground<V: 'static>(PhantomData<V>);

impl<V> Playground<V> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> impl Element<V> {
        Empty::new()
        // workspace(&rose_pine::dawn())
    }
}

// fn workspace<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
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
