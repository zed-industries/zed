use gpui2::HighlightStyle;
use std::sync::Arc;

pub struct Theme {
    pub editor: Editor,
}

pub struct Editor {
    pub syntax: Arc<SyntaxTheme>,
}

#[derive(Default)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    pub fn new(highlights: Vec<(String, HighlightStyle)>) -> Self {
        Self { highlights }
    }
}
