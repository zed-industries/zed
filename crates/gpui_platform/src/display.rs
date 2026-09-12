//! Display abstractions shared by `gpui` and its platform backends.

use crate::DEFAULT_WINDOW_SIZE;
use anyhow::Result;
use gpui_types::{Bounds, Pixels, point};
use std::fmt::{self, Debug};
use uuid::Uuid;

/// An opaque identifier for a hardware display
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct DisplayId(pub(crate) u64);

impl DisplayId {
    /// Create a new `DisplayId` from a raw platform display identifier.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl From<u64> for DisplayId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<DisplayId> for u64 {
    fn from(id: DisplayId) -> Self {
        id.0
    }
}

impl Debug for DisplayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DisplayId({})", self.0)
    }
}

/// A handle to a platform's display, e.g. a monitor or laptop screen.
pub trait PlatformDisplay: Debug {
    /// Get the ID for this display
    fn id(&self) -> DisplayId;

    /// Returns a stable identifier for this display that can be persisted and used
    /// across system restarts.
    fn uuid(&self) -> Result<Uuid>;

    /// Get the bounds for this display
    fn bounds(&self) -> Bounds<Pixels>;

    /// Get the visible bounds for this display, excluding taskbar/dock areas.
    /// This is the usable area where windows can be placed without being obscured.
    /// Defaults to the full display bounds if not overridden.
    fn visible_bounds(&self) -> Bounds<Pixels> {
        self.bounds()
    }

    /// Get the default bounds for this display to place a window
    fn default_bounds(&self) -> Bounds<Pixels> {
        let bounds = self.bounds();
        let center = bounds.center();
        let clipped_window_size = DEFAULT_WINDOW_SIZE.min(&bounds.size);

        let offset = clipped_window_size / 2.0;
        let origin = point(center.x - offset.width, center.y - offset.height);
        Bounds::new(origin, clipped_window_size)
    }
}
