pub use language_core::highlight_map::{HighlightId, HighlightMap};

use gpui::HighlightStyle;
use theme::SyntaxTheme;

pub fn highlight_style(id: HighlightId, theme: &SyntaxTheme) -> Option<HighlightStyle> {
    theme
        .highlights
        .get(id.index() as usize)
        .map(|entry| entry.1)
}

pub fn highlight_name(id: HighlightId, theme: &SyntaxTheme) -> Option<&str> {
    theme
        .highlights
        .get(id.index() as usize)
        .map(|entry| entry.0.as_str())
}
