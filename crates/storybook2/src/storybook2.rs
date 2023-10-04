#![allow(dead_code, unused_variables)]

use assets::Assets;
use gpui3::{px, size, Bounds, WindowBounds, WindowOptions};
use log::LevelFilter;
use simplelog::SimpleLogger;
use std::sync::Arc;
use workspace::workspace;

mod assets;
mod collab_panel;
mod theme;
mod themes;
mod ui;
mod workspace;

// gpui2::actions! {
//     storybook,
//     [ToggleInspector]
// }

fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() };

    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let asset_source = Arc::new(Assets);
    gpui3::App::production(asset_source).run(|cx| {
        let window = cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(800.), px(600.)),
                }),
                ..Default::default()
            },
            |cx| workspace(cx),
        );

        cx.activate(true);
    });
}

// fn load_embedded_fonts(platform: &dyn gpui2::Platform) {
//     let font_paths = Assets.list("fonts");
//     let mut embedded_fonts = Vec::new();
//     for font_path in &font_paths {
//         if font_path.ends_with(".ttf") {
//             let font_path = &*font_path;
//             let font_bytes = Assets.load(font_path).unwrap().to_vec();
//             embedded_fonts.push(Arc::from(font_bytes));
//         }
//     }
//     platform.fonts().add_fonts(&embedded_fonts).unwrap();
// }
