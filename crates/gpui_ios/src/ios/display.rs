//! iOS display handling using UIScreen.
//!
//! iOS has a simpler display model than macOS - typically just the main screen
//! and possibly an external display via AirPlay or USB-C.

use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, px, size};
use objc2::{MainThreadMarker, rc::Retained};
use objc2_core_foundation::CGRect;
use objc2_ui_kit::UIScreen;
use uuid::Uuid;

/// Represents an iOS display (UIScreen).
#[derive(Debug)]
pub(crate) struct IosDisplay {
    /// The UIScreen object
    screen: Retained<UIScreen>,
}

unsafe impl Send for IosDisplay {}
unsafe impl Sync for IosDisplay {}

impl IosDisplay {
    /// Get the main screen.
    #[allow(deprecated)] // Platform display queries can precede scene creation.
    pub fn main() -> Self {
        let screen =
            UIScreen::mainScreen(MainThreadMarker::new().expect("UIKit requires the main thread"));
        Self { screen }
    }

    /// Get all connected screens.
    #[allow(deprecated)] // Preserve external screens even before they have a GPUI scene.
    pub fn all() -> impl Iterator<Item = Self> {
        let screens =
            UIScreen::screens(MainThreadMarker::new().expect("UIKit requires the main thread"));
        screens.to_vec().into_iter().map(|screen| Self { screen })
    }

    /// Get the screen bounds in points.
    fn bounds_in_points(&self) -> CGRect {
        self.screen.bounds()
    }

    /// Get the native scale factor of this screen.
    pub fn native_scale(&self) -> f32 {
        self.screen.nativeScale() as f32
    }

    /// Get the current scale factor (may differ from native if zoomed).
    pub fn scale(&self) -> f32 {
        self.screen.scale() as f32
    }
}

impl PlatformDisplay for IosDisplay {
    fn id(&self) -> DisplayId {
        // iOS doesn't have display IDs like macOS, so we use the screen pointer as an ID
        DisplayId::new(Retained::as_ptr(&self.screen) as u64)
    }

    fn uuid(&self) -> Result<Uuid> {
        // iOS doesn't provide persistent UUIDs for displays like macOS does.
        // We generate a deterministic UUID based on the screen properties.
        let bounds = self.bounds_in_points();
        let scale = self.native_scale();

        // Create a deterministic UUID from screen properties
        let bytes = format!(
            "ios-screen-{}-{}-{}",
            bounds.size.width as u32,
            bounds.size.height as u32,
            (scale * 100.0) as u32
        );

        Ok(Uuid::new_v5(&Uuid::NAMESPACE_OID, bytes.as_bytes()))
    }

    fn bounds(&self) -> Bounds<Pixels> {
        let bounds = self.bounds_in_points();

        Bounds {
            origin: Default::default(),
            size: size(px(bounds.size.width as f32), px(bounds.size.height as f32)),
        }
    }
}
