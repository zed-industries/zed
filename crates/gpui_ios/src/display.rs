use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use uuid::Uuid;

/// Represents the iPad's screen. On iPadOS there is always exactly one display
/// per UIScene (Stage Manager allows multiple scenes but each maps to its own
/// UIWindowScene with its own coordinate space).
#[derive(Debug)]
pub(crate) struct IosDisplay {
    uuid: Uuid,
    /// Logical-pixel bounds of the display. Retrieved from UIScreen at
    /// platform construction time; updated on scene geometry change
    /// notifications in Phase 1.3.
    bounds: Bounds<Pixels>,
}

impl IosDisplay {
    pub fn new(bounds: Bounds<Pixels>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            bounds,
        }
    }
}

impl PlatformDisplay for IosDisplay {
    fn id(&self) -> DisplayId {
        DisplayId::new(1)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}

/// Returns the main screen's logical bounds using UIScreen.main.bounds.
/// Called once at platform init; Stage Manager scene geometry changes will
/// be handled separately in Phase 1.3.
pub(crate) fn main_screen_bounds() -> Bounds<Pixels> {
    // UIScreen.main.bounds is accessed through the C shim defined in
    // crates/zed-ios/src/ios_shims.h and exposed via FFI in Phase 1.3.
    // For now, return a reasonable iPad Pro 13" logical resolution.
    // This placeholder is replaced when the real UIKit bridge is wired up.
    Bounds {
        origin: Point::default(),
        size: Size {
            width: px(1366.0),
            height: px(1024.0),
        },
    }
}
