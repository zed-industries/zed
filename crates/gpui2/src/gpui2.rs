#![allow(dead_code, unused_variables)]
use gpui::{serde_json, ViewContext};
use theme::ThemeSettings;
use themes::Theme;

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
