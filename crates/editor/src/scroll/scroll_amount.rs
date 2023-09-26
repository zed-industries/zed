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
                .map(|mut l| {
                    // for full pages subtract one to leave an anchor line
                    if count.abs() == 1.0 {
                        l -= 1.0
                    }
                    (l * count).trunc()
                })
                .unwrap_or(0.),
        }
    }
}
