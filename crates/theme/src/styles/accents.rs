use gpui::Hsla;
use serde_derive::Deserialize;

use crate::{AccentContent, default::default_dark_theme, try_parse_color};

/// A collection of colors that are used to color indent aware lines in the editor.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AccentColors(pub Vec<Hsla>);

impl Default for AccentColors {
    /// Don't use this!
    /// We have to have a default to be `[refineable::Refinable]`.
    /// TODO "Find a way to not need this for Refinable"
    fn default() -> Self {
        default_dark_theme().accents().clone()
    }
}

impl AccentColors {
    /// Returns the color for the given index.
    pub fn color_for_index(&self, index: u32) -> Hsla {
        self.0[index as usize % self.0.len()]
    }

    /// Merges the given accent colors into this [`AccentColors`] instance.
    pub fn merge(&mut self, accent_colors: &[AccentContent]) {
        if accent_colors.is_empty() {
            return;
        }

        let colors = accent_colors
            .iter()
            .filter_map(|accent_color| {
                accent_color
                    .0
                    .as_ref()
                    .and_then(|color| try_parse_color(color).ok())
            })
            .collect::<Vec<_>>();

        if !colors.is_empty() {
            self.0 = colors;
        }
    }
}
