#![allow(dead_code, unused_variables)]

use crate::theme::Theme;
use ::theme as legacy_theme;
use components::icon_button;
use element_ext::ElementExt;
use gpui2::{
    elements::div, serde_json, style::StyleHelpers, vec2f, view, Element, ParentElement, RectF,
    ViewContext, WindowBounds,
};
use legacy_theme::ThemeSettings;
use log::LevelFilter;
use modules::title_bar;
use settings::{default_settings, SettingsStore};
use simplelog::SimpleLogger;

mod collab_panel;
mod components;
mod element_ext;
mod modules;
mod prelude;
mod theme;
mod workspace;

gpui2::actions! {
    storybook,
    [ToggleInspector]
}

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui2::App::new(Assets).unwrap().run(|cx| {
        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        legacy_theme::init(Assets, cx);
        // load_embedded_fonts(cx.platform().as_ref());

        cx.add_window(
            gpui2::WindowOptions {
                bounds: WindowBounds::Fixed(RectF::new(vec2f(0., 0.), vec2f(1600., 900.))),
                center: true,
                ..Default::default()
            },
            |cx| {
                view(|cx| {
                    // cx.enable_inspector();
                    storybook(&mut ViewContext::new(cx))
                })
            },
        );
        cx.platform().activate(true);
    });
}

fn storybook<V: 'static>(cx: &mut ViewContext<V>) -> impl Element<V> {
    workspace().themed(current_theme(cx))
}

// Nathan: During the transition to gpui2, we will include the base theme on the legacy Theme struct.
fn current_theme<V: 'static>(cx: &mut ViewContext<V>) -> Theme {
    settings::get::<ThemeSettings>(cx)
        .theme
        .deserialized_base_theme
        .lock()
        .get_or_insert_with(|| {
            let theme: Theme =
                serde_json::from_value(settings::get::<ThemeSettings>(cx).theme.base_theme.clone())
                    .unwrap();
            Box::new(theme)
        })
        .downcast_ref::<Theme>()
        .unwrap()
        .clone()
}

use anyhow::{anyhow, Result};
use gpui2::AssetSource;
use rust_embed::RustEmbed;
use workspace::workspace;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "themes/**/*"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Vec<std::borrow::Cow<'static, str>> {
        Self::iter().filter(|p| p.starts_with(path)).collect()
    }
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
