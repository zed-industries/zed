pub use language_core::highlight_map::{HighlightId, HighlightMap};

use gpui::HighlightStyle;
use theme::SyntaxTheme;

/// Extension trait that adds theme-dependent methods to [`HighlightId`].
pub trait HighlightIdExt {
    fn style(&self, theme: &SyntaxTheme) -> Option<HighlightStyle>;
    fn name<'a>(&self, theme: &'a SyntaxTheme) -> Option<&'a str>;
}

impl HighlightIdExt for HighlightId {
    fn style(&self, theme: &SyntaxTheme) -> Option<HighlightStyle> {
        theme
            .highlights
            .get(self.index() as usize)
            .map(|entry| entry.1)
    }

    fn name<'a>(&self, theme: &'a SyntaxTheme) -> Option<&'a str> {
        theme
            .highlights
            .get(self.index() as usize)
            .map(|entry| entry.0.as_str())
    }
}
