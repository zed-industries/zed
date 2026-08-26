use gpui::{Pixels, px};

pub struct SearchInputWidth;

impl SearchInputWidth {
    /// The container size in which the input stops filling the whole width.
    pub const THRESHOLD_WIDTH: Pixels = px(1200.0);

    /// The maximum width for the search input when the container is larger than the threshold.
    pub const MAX_WIDTH: Pixels = px(1200.0);

    /// Calculates the actual width in pixels based on the container width.
    pub fn calc_width(container_width: Pixels) -> Pixels {
        if container_width < Self::THRESHOLD_WIDTH {
            container_width
        } else {
            container_width.min(Self::MAX_WIDTH)
        }
    }
}
