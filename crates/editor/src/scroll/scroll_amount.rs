use serde::Deserialize;
use ui::{Pixels, px};

#[derive(Debug)]
pub enum ScrollDirection {
    Upwards,
    Downwards,
}

impl ScrollDirection {
    pub fn is_upwards(&self) -> bool {
        matches!(self, ScrollDirection::Upwards)
    }
}

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
                if self.is_full_page() {
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

    pub fn is_full_page(&self) -> bool {
        match self {
            ScrollAmount::Page(count) if count.abs() == 1.0 => true,
            _ => false,
        }
    }

    pub fn direction(&self) -> ScrollDirection {
        match self {
            Self::Line(amount) if amount.is_sign_positive() => ScrollDirection::Downwards,
            Self::Page(amount) if amount.is_sign_positive() => ScrollDirection::Downwards,
            _ => ScrollDirection::Upwards,
        }
    }
}
