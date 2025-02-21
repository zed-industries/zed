use gpui::Pixels;

pub struct SearchInputWidth;

impl SearchInputWidth {
    /// The containzer size in which the input stops filling the whole width.
    pub const THRESHOLD_WIDTH: f32 = 1200.0;

    /// The maximum width for the search input when the container is larger than the threshold.
    pub const MAX_WIDTH: f32 = 1200.0;

    /// Calculates the actual width in pixels based on the container width.
    pub fn calc_width(container_width: Pixels) -> Pixels {
        if container_width.0 < Self::THRESHOLD_WIDTH {
            container_width
        } else {
            Pixels(container_width.0.min(Self::MAX_WIDTH))
        }
    }
}
