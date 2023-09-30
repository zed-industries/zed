#![allow(dead_code, unused_variables)]

use gpui3::{Bounds, WindowBounds, WindowOptions};
use log::LevelFilter;
use simplelog::SimpleLogger;

mod collab_panel;
mod theme;
mod themes;
mod workspace;

// gpui2::actions! {
//     storybook,
//     [ToggleInspector]
// }

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };

    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui3::App::production().run(|cx| {
        let window = cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    size: gpui3::Size {
                        width: 800_f32.into(),
                        height: 600_f32.into(),
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| workspace(cx),
        );
        cx.activate(true);
    });
}

use rust_embed::RustEmbed;
use workspace::workspace;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "themes/**/*"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

// impl AssetSource for Assets {
//     fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
//         Self::get(path)
//             .map(|f| f.data)
//             .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
//     }

//     fn list(&self, path: &str) -> Vec<std::borrow::Cow<'static, str>> {
//         Self::iter().filter(|p| p.starts_with(path)).collect()
//     }
// }

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
