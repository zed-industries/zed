pub use language_core::highlight_map::{HighlightId, HighlightMap};

use gpui::HighlightStyle;
use theme::SyntaxTheme;

pub fn highlight_style(id: HighlightId, theme: &SyntaxTheme) -> Option<HighlightStyle> {
    theme.get(id.index() as usize).cloned()
}

pub fn highlight_name(id: HighlightId, theme: &SyntaxTheme) -> Option<&str> {
    theme.get_capture_name(id.index() as usize)
}
