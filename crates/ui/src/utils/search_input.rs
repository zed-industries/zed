#![allow(missing_docs)]

use gpui::Pixels;

/// This enum is used to calculate the appropriate width of search input fields
/// based on the container width and fullscreen state.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum SearchInputWidth {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
    Full,
}

impl SearchInputWidth {
    pub const MIN_WIDTH_PX: f32 = 300.0;
    pub const SMALL_WIDTH_PX: f32 = 400.0;
    pub const MEDIUM_WIDTH_PX: f32 = 600.0;
    pub const LARGE_WIDTH_PX: f32 = 800.0;
    pub const XLARGE_WIDTH_PX: f32 = 1000.0;
    pub const FULLSCREEN_MAX_WIDTH_PX: f32 = 1200.0;

    /// Calculates the actual width in pixels based on the container width, considering whether it is fullscreen.
    pub fn calc_width(&self, container_width: Pixels, is_fullscreen: bool) -> Pixels {
        let max_width = if is_fullscreen {
            Self::FULLSCREEN_MAX_WIDTH_PX
        } else {
            match self {
                SearchInputWidth::Small => Self::SMALL_WIDTH_PX,
                SearchInputWidth::Medium => Self::MEDIUM_WIDTH_PX,
                SearchInputWidth::Large => Self::LARGE_WIDTH_PX,
                SearchInputWidth::XLarge => Self::XLARGE_WIDTH_PX,
                SearchInputWidth::Full => container_width.0,
            }
        };

        let width = if *self == SearchInputWidth::Full {
            container_width.0
        } else {
            let percentage = match self {
                SearchInputWidth::Small => 0.3,
                SearchInputWidth::Medium => 0.5,
                SearchInputWidth::Large => 0.7,
                SearchInputWidth::XLarge => 0.9,
                SearchInputWidth::Full => 1.0,
            };
            (container_width.0 * percentage).min(max_width)
        };

        Pixels(width.max(Self::MIN_WIDTH_PX))
    }

    /// Determines the appropriate input width based on the container width.
    pub fn from_container_width(container_width: Pixels) -> Self {
        if container_width.0 < Self::MEDIUM_WIDTH_PX {
            Self::Small
        } else if container_width.0 < Self::LARGE_WIDTH_PX {
            Self::Medium
        } else if container_width.0 < Self::XLARGE_WIDTH_PX {
            Self::Large
        } else {
            Self::XLarge
        }
    }
}
