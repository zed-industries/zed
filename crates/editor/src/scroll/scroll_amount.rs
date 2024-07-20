use serde::Deserialize;
use ui::{px, Pixels};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ScrollAmount {
    // Scroll N lines (positive is towards the end of the document)
    Line(f32),
    // Scroll N pages (positive is towards the end of the document)
    Page(f32),
}

impl ScrollAmount {
    pub fn lines(&self, mut visible_line_count: f32) -> f32 {
        match self {
            Self::Line(count) => *count,
            Self::Page(count) => {
                // for full pages subtract one to leave an anchor line
                if count.abs() == 1.0 {
                    visible_line_count -= 1.0
                }
                (visible_line_count * count).trunc()
            }
        }
    }

    pub fn pixels(&self, line_height: Pixels, height: Pixels) -> Pixels {
        match self {
            ScrollAmount::Line(x) => px(line_height.0 * x),
            ScrollAmount::Page(x) => px(height.0 * x),
        }
    }
}
