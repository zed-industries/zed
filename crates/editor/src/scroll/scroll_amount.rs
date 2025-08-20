use serde::Deserialize;
use ui::{Pixels, px};

#[derive(Debug)]
pub enum ScrollDirection {
    Upwards,
    Downwards,
    Rightwards,
    Leftwards,
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
    // Scroll N columns (positive is towards the right of the document)
    Column(f32),
    // Scroll N page width (positive is towards the right of the document)
    PageWidth(f32),
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
            Self::Column(_count) => 0.0,
            Self::PageWidth(_count) => 0.0,
        }
    }

    pub fn columns(&self, visible_column_count: f32) -> f32 {
        match self {
            Self::Line(_count) => 0.0,
            Self::Page(_count) => 0.0,
            Self::Column(count) => *count,
            Self::PageWidth(count) => (visible_column_count * count).trunc(),
        }
    }

    pub fn pixels(&self, line_height: Pixels, height: Pixels) -> Pixels {
        match self {
            ScrollAmount::Line(x) => px(line_height.0 * x),
            ScrollAmount::Page(x) => px(height.0 * x),
            // This function seems to only be leveraged by the popover that is
            // displayed by the editor when, for example, viewing a function's
            // documentation. Right now that only supports vertical scrolling,
            // so I'm leaving this at 0.0 for now to try and make it clear that
            // this should not have an impact on that?
            ScrollAmount::Column(_) => px(0.0),
            ScrollAmount::PageWidth(_) => px(0.0),
        }
    }

    pub fn is_full_page(&self) -> bool {
        matches!(self, ScrollAmount::Page(count) if count.abs() == 1.0)
    }

    pub fn direction(&self) -> ScrollDirection {
        match self {
            Self::Line(amount) if amount.is_sign_positive() => ScrollDirection::Downwards,
            Self::Page(amount) if amount.is_sign_positive() => ScrollDirection::Downwards,
            Self::Column(amount) if amount.is_sign_positive() => ScrollDirection::Rightwards,
            Self::Column(amount) if amount.is_sign_negative() => ScrollDirection::Leftwards,
            _ => ScrollDirection::Upwards,
        }
    }
}
