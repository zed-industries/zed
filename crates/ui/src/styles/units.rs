use gpui::{Length, WindowContext};

/// Returns a [`Length`] corresponding to the specified percentage of the viewport's width.
///
/// `percent` should be a value between `0.0` and `1.0`.
pub fn vw(percent: f32, cx: &mut WindowContext) -> Length {
    Length::from(cx.viewport_size().width * percent)
}

/// Returns a [`Length`] corresponding to the specified percentage of the viewport's height.
///
/// `percent` should be a value between `0.0` and `1.0`.
pub fn vh(percent: f32, cx: &mut WindowContext) -> Length {
    Length::from(cx.viewport_size().height * percent)
}
