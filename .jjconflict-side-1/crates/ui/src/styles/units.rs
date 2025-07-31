use gpui::{Length, Rems, Window, rems};

/// The base size of a rem, in pixels.
pub const BASE_REM_SIZE_IN_PX: f32 = 16.;

/// Returns a rem value derived from the provided pixel value and the base rem size (16px).
///
/// This can be used to compute rem values relative to pixel sizes, without
/// needing to hard-code the rem value.
///
/// For instance, instead of writing `rems(0.875)` you can write `rems_from_px(14.)`
#[inline(always)]
pub fn rems_from_px(px: f32) -> Rems {
    rems(px / BASE_REM_SIZE_IN_PX)
}

/// Returns a [`Length`] corresponding to the specified percentage of the viewport's width.
///
/// `percent` should be a value between `0.0` and `1.0`.
pub fn vw(percent: f32, window: &mut Window) -> Length {
    Length::from(window.viewport_size().width * percent)
}

/// Returns a [`Length`] corresponding to the specified percentage of the viewport's height.
///
/// `percent` should be a value between `0.0` and `1.0`.
pub fn vh(percent: f32, window: &mut Window) -> Length {
    Length::from(window.viewport_size().height * percent)
}
