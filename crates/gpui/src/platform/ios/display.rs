//! iOS display handling using UIScreen.
//!
//! iOS has a simpler display model than macOS - typically just the main screen
//! and possibly an external display via AirPlay or USB-C.

use crate::{Bounds, DisplayId, Pixels, PlatformDisplay, px, size};
use anyhow::Result;
use core_graphics::geometry::CGRect;
use objc::{class, msg_send, sel, sel_impl};
use uuid::Uuid;

/// Represents an iOS display (UIScreen).
#[derive(Debug)]
pub(crate) struct IosDisplay {
    /// The UIScreen object
    screen: *mut objc::runtime::Object,
}

unsafe impl Send for IosDisplay {}
unsafe impl Sync for IosDisplay {}

impl IosDisplay {
    /// Get the main screen.
    pub fn main() -> Self {
        unsafe {
            let screen: *mut objc::runtime::Object = msg_send![class!(UIScreen), mainScreen];
            Self { screen }
        }
    }

    /// Get all connected screens.
    pub fn all() -> impl Iterator<Item = Self> {
        unsafe {
            let screens: *mut objc::runtime::Object = msg_send![class!(UIScreen), screens];
            let count: usize = msg_send![screens, count];

            (0..count).map(move |i| {
                let screen: *mut objc::runtime::Object = msg_send![screens, objectAtIndex: i];
                Self { screen }
            })
        }
    }

    /// Get the screen bounds in points.
    fn bounds_in_points(&self) -> CGRect {
        unsafe { msg_send![self.screen, bounds] }
    }

    /// Get the native scale factor of this screen.
    pub fn native_scale(&self) -> f32 {
        unsafe {
            let scale: f64 = msg_send![self.screen, nativeScale];
            scale as f32
        }
    }

    /// Get the current scale factor (may differ from native if zoomed).
    pub fn scale(&self) -> f32 {
        unsafe {
            let scale: f64 = msg_send![self.screen, scale];
            scale as f32
        }
    }
}

impl PlatformDisplay for IosDisplay {
    fn id(&self) -> DisplayId {
        // iOS doesn't have display IDs like macOS, so we use the screen pointer as an ID
        DisplayId(self.screen as u32)
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
