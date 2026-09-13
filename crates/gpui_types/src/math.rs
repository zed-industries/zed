//! Device-pixel snapping helpers shared by layout and painting.
//!
//! Layout resolves fractional positions and sizes; painting them directly
//! produces blurry edges. These helpers round in device-pixel space so painted
//! edges land on physical pixel boundaries. They live in this leaf crate so the
//! engine (`gpui_backend`) and the facade (`gpui`) snap identically.

/// Rounds to the nearest integer with 0.5 ties toward zero.
#[inline]
pub fn round_half_toward_zero(value: f32) -> f32 {
    (value.abs() - 0.5).ceil().copysign(value)
}

/// Rounds to the nearest integer with 0.5 ties toward zero.
#[inline]
pub fn round_half_toward_zero_f64(value: f64) -> f64 {
    (value.abs() - 0.5).ceil().copysign(value)
}

/// Rounds a logical coordinate to the nearest device pixel.
#[inline]
pub fn round_to_device_pixel(logical: f32, scale_factor: f32) -> f32 {
    round_half_toward_zero(logical * scale_factor)
}

/// Snaps a stroke width to at least one device pixel.
#[inline]
pub fn round_stroke_to_device_pixel(logical: f32, scale_factor: f32) -> f32 {
    if logical == 0.0 {
        0.0
    } else {
        round_to_device_pixel(logical.max(0.0), scale_factor).max(1.0)
    }
}

/// Rounds a logical coordinate down to a device pixel boundary.
#[inline]
pub fn floor_to_device_pixel(logical: f32, scale_factor: f32) -> f32 {
    (logical * scale_factor).floor()
}

/// Rounds a logical coordinate up to a device pixel boundary.
#[inline]
pub fn ceil_to_device_pixel(logical: f32, scale_factor: f32) -> f32 {
    (logical * scale_factor).ceil()
}
