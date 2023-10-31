mod colors;
mod default_colors;
mod default_theme;
mod registry;
mod scale;
mod settings;
mod syntax;
mod theme3;
mod themes;
mod utils;

pub use default_colors::*;
pub use default_theme::*;
pub use registry::*;
pub use scale::*;
pub use settings::*;
pub use syntax::*;

use gpui2::{AppContext, HighlightStyle, Hsla, SharedString};
use settings2::Settings;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Appearance {
    Light,
    Dark,
}

pub fn init(cx: &mut AppContext) {
    cx.set_global(ThemeRegistry::default());
    ThemeSettings::register(cx);
}

pub fn active_theme<'a>(cx: &'a AppContext) -> &'a Arc<Theme> {
    &ThemeSettings::get_global(cx).active_theme
}

pub fn theme(cx: &AppContext) -> Arc<Theme> {
    active_theme(cx).clone()
}

pub struct Theme {
    pub metadata: ThemeMetadata,

    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    /// The background color of an elevated surface, like a modal, tooltip or toast.
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    /// Window background color of the base app
    pub background: Hsla,
    /// Default background for elements like filled buttons,
    /// text fields, checkboxes, radio buttons, etc.
    /// - TODO: Map to step 3.
    pub filled_element: Hsla,
    /// The background color of a hovered element, like a button being hovered
    /// with a mouse, or hovered on a touch screen.
    /// - TODO: Map to step 4.
    pub filled_element_hover: Hsla,
    /// The background color of an active element, like a button being pressed,
    /// or tapped on a touch screen.
    /// - TODO: Map to step 5.
    pub filled_element_active: Hsla,
    /// The background color of a selected element, like a selected tab,
    /// a button toggled on, or a checkbox that is checked.
    pub filled_element_selected: Hsla,
    pub filled_element_disabled: Hsla,
    pub ghost_element: Hsla,
    /// The background color of a hovered element with no default background,
    /// like a ghost-style button or an interactable list item.
    /// - TODO: Map to step 3.
    pub ghost_element_hover: Hsla,
    /// - TODO: Map to step 4.
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,
    pub icon_muted: Hsla,
    pub syntax: SyntaxTheme,

    pub status_bar: Hsla,
    pub title_bar: Hsla,
    pub toolbar: Hsla,
    pub tab_bar: Hsla,
    /// The background of the editor
    pub editor: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
    pub terminal: Hsla,
    pub image_fallback_background: Hsla,

    pub git_created: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
    pub git_conflict: Hsla,
    pub git_ignored: Hsla,
    pub git_renamed: Hsla,

    pub players: [PlayerTheme; 8],
}

#[derive(Clone)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    // TOOD: Get this working with `#[cfg(test)]`. Why isn't it?
    pub fn new_test(colors: impl IntoIterator<Item = (&'static str, Hsla)>) -> Self {
        SyntaxTheme {
            highlights: colors
                .into_iter()
                .map(|(key, color)| {
                    (
                        key.to_owned(),
                        HighlightStyle {
                            color: Some(color),
                            ..Default::default()
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn get(&self, name: &str) -> HighlightStyle {
        self.highlights
            .iter()
            .find_map(|entry| if entry.0 == name { Some(entry.1) } else { None })
            .unwrap_or_default()
    }

    pub fn color(&self, name: &str) -> Hsla {
        self.get(name).color.unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
pub struct PlayerTheme {
    pub cursor: Hsla,
    pub selection: Hsla,
}

#[derive(Clone)]
pub struct ThemeMetadata {
    pub name: SharedString,
    pub is_light: bool,
}

pub struct Editor {
    pub syntax: Arc<SyntaxTheme>,
}
