//! Input modality and viewport helpers for adaptive interfaces.
//!
//! Touch input and narrow viewports are independent: tablets need larger hit
//! targets, while narrow desktop windows can retain mouse-oriented controls.

use gpui::{App, Global, Pixels, Window, px};

const NARROW_VIEWPORT_MAX_WIDTH: Pixels = px(500.);

#[derive(Default)]
struct InputModality {
    touch_primary: bool,
}

impl Global for InputModality {}

/// Records whether the device's primary pointer is touch.
///
/// Hosts set this during startup. Hosts that never set it default to non-touch.
pub fn set_touch_input(touch_primary: bool, cx: &mut App) {
    cx.set_global(InputModality { touch_primary });
}

/// Whether the primary pointer is coarse touch input without hover.
pub fn is_touch_input(cx: &App) -> bool {
    cx.try_global::<InputModality>()
        .is_some_and(|modality| modality.touch_primary)
}

/// Whether the viewport is phone-sized, at most 500 logical pixels wide.
pub fn is_narrow_viewport(window: &Window) -> bool {
    window.viewport_size().width <= NARROW_VIEWPORT_MAX_WIDTH
}

/// Whether a touch-first, phone-sized layout is appropriate.
pub fn uses_mobile_layout(window: &Window, cx: &App) -> bool {
    is_narrow_viewport(window) && is_touch_input(cx)
}

#[cfg(test)]
mod tests;
