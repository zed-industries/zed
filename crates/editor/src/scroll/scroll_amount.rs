use crate::Editor;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub enum ScrollAmount {
    // Scroll N lines (positive is towards the end of the document)
    Line(f32),
    // Scroll N pages (positive is towards the end of the document)
    Page(f32),
}

impl ScrollAmount {
    pub fn lines(&self, editor: &mut Editor) -> f32 {
        match self {
            Self::Line(count) => *count,
            Self::Page(count) => editor
                .visible_line_count()
                // subtract one to leave an anchor line
                // round towards zero (so page-up and page-down are symmetric)
                .map(|l| (l * count).trunc() - count.signum())
                .unwrap_or(0.),
        }
    }
}
