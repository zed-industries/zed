use gpui::{HighlightStyle, Hsla};
use std::sync::Arc;
use theme::{AccentColors, SyntaxTheme};

#[derive(Clone, Debug)]
pub struct BracketMap(Vec<Hsla>);

impl BracketMap {
    pub(crate) fn new(colors: AccentColors) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that matches all of the
        // dot-separated components of the capture name.
        BracketMap(colors.0)
    }

    pub fn get(&self, depth: u32) -> Hsla {
        self.0.get(depth % self.0.len()).copied().unwrap()
    }
}

impl Default for BracketMap {
    fn default() -> Self {
        Self(vec![])
    }
}
