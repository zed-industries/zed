pub use language_core::highlight_map::{HighlightId, HighlightMap};

use gpui::HighlightStyle;
use theme::SyntaxTheme;

pub fn highlight_style(id: HighlightId, theme: &SyntaxTheme) -> Option<HighlightStyle> {
    theme
        .highlights
        .get(id.index() as usize)
        .map(|entry| entry.1)
}

pub fn highlight_name<'a>(id: HighlightId, theme: &'a SyntaxTheme) -> Option<&'a str> {
    theme
        .highlights
        .get(id.index() as usize)
        .map(|entry| entry.0.as_str())
}
