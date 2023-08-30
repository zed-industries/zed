#![allow(dead_code, unused_variables)]
use crate::element::Element;
use gpui::{
    geometry::{rect::RectF, vector::vec2f},
    platform::WindowOptions,
    serde_json, ViewContext,
};
use log::LevelFilter;
use settings::{default_settings, SettingsStore};
use simplelog::SimpleLogger;
use theme::ThemeSettings;
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
        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        theme::init(Assets, cx);

        cx.add_window(
            WindowOptions {
                bounds: gpui::platform::WindowBounds::Fixed(RectF::new(
                    vec2f(0., 0.),
                    vec2f(400., 300.),
                )),
                center: true,
                ..Default::default()
            },
            |_| view(|cx| playground(cx)),
        );
        cx.platform().activate(true);
    });
}

fn playground<V: 'static>(cx: &mut ViewContext<V>) -> impl Element<V> {
    workspace().themed(current_theme(cx))
}

// Nathan: During the transition, we will include the base theme on the legacy Theme struct.
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
use gpui::AssetSource;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../../assets"]
#[include = "themes/**/*"]
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
