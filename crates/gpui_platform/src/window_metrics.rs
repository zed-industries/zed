//! A sampled snapshot of a window's geometry and state.

use crate::DisplayId;
use gpui_types::{Bounds, Pixels, Size};

/// The state a window is presenting, sampled once from its platform window.
///
/// The getters on [`PlatformWindow`](crate::PlatformWindow) query the operating
/// system, so they can only be called while the window is borrowed. A
/// `WindowMetrics` is a plain value taken from those getters at a moment in
/// time, which makes it cheap to publish and safe to read from another thread.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowMetrics {
    /// The window's bounds in the global coordinate space, which can span displays.
    pub bounds: Bounds<Pixels>,
    /// The visible viewport in window-local logical pixels.
    ///
    /// Unlike [`Self::content_size`], this can shrink or move when a software
    /// keyboard opens.
    pub viewport: Bounds<Pixels>,
    /// The size of the drawable area, which is the window's full layout size.
    pub content_size: Size<Pixels>,
    /// The scale factor of the display the window is on.
    pub scale_factor: f32,
    /// The display the window is on, when the platform reports one.
    pub display_id: Option<DisplayId>,
    /// Whether this is the platform's active, or focused, window.
    pub is_active: bool,
    /// Whether the window is fullscreen.
    pub is_fullscreen: bool,
}
