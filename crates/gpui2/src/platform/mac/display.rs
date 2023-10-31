use crate::{point, size, Bounds, DisplayId, GlobalPixels, PlatformDisplay};
use core_graphics::{
    display::{CGDirectDisplayID, CGDisplayBounds, CGGetActiveDisplayList},
    geometry::{CGPoint, CGRect, CGSize},
};
use std::any::Any;

#[derive(Debug)]
pub struct MacDisplay(pub(crate) CGDirectDisplayID);

unsafe impl Send for MacDisplay {}

impl MacDisplay {
    /// Get the screen with the given UUID.
    pub fn find_by_id(id: DisplayId) -> Option<Self> {
        Self::all().find(|screen| screen.id() == id)
    }

    /// Get the primary screen - the one with the menu bar, and whose bottom left
    /// corner is at the origin of the AppKit coordinate system.
    pub fn primary() -> Self {
        Self::all().next().unwrap()
    }

    pub fn all() -> impl Iterator<Item = Self> {
        unsafe {
            let mut display_count: u32 = 0;
            let result = CGGetActiveDisplayList(0, std::ptr::null_mut(), &mut display_count);

            if result == 0 {
                let mut displays = Vec::with_capacity(display_count as usize);
                CGGetActiveDisplayList(display_count, displays.as_mut_ptr(), &mut display_count);
                displays.set_len(display_count as usize);

                displays.into_iter().map(|display| MacDisplay(display))
            } else {
                panic!("Failed to get active display list");
            }
        }
    }
}

/// Convert the given rectangle from CoreGraphics' native coordinate space to GPUI's coordinate space.
///
/// CoreGraphics' coordinate space has its origin at the bottom left of the primary screen,
/// with the Y axis pointing upwards.
///
/// Conversely, in GPUI's coordinate system, the origin is placed at the top left of the primary
/// screen, with the Y axis pointing downwards.
pub(crate) fn display_bounds_from_native(rect: CGRect) -> Bounds<GlobalPixels> {
    let primary_screen_size = unsafe { CGDisplayBounds(MacDisplay::primary().id().0) }.size;

    Bounds {
        origin: point(
            GlobalPixels(rect.origin.x as f32),
            GlobalPixels(
                primary_screen_size.height as f32 - rect.origin.y as f32 - rect.size.height as f32,
            ),
        ),
        size: size(
            GlobalPixels(rect.size.width as f32),
            GlobalPixels(rect.size.height as f32),
        ),
    }
}

/// Convert the given rectangle from GPUI's coordinate system to CoreGraphics' native coordinate space.
///
/// CoreGraphics' coordinate space has its origin at the bottom left of the primary screen,
/// with the Y axis pointing upwards.
///
/// Conversely, in GPUI's coordinate system, the origin is placed at the top left of the primary
/// screen, with the Y axis pointing downwards.
pub(crate) fn display_bounds_to_native(bounds: Bounds<GlobalPixels>) -> CGRect {
    let primary_screen_height = MacDisplay::primary().bounds().size.height;

    CGRect::new(
        &CGPoint::new(
            bounds.origin.x.into(),
            (primary_screen_height - bounds.origin.y - bounds.size.height).into(),
        ),
        &CGSize::new(bounds.size.width.into(), bounds.size.height.into()),
    )
}

impl PlatformDisplay for MacDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(self.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        unsafe {
            let native_bounds = CGDisplayBounds(self.0);
            display_bounds_from_native(native_bounds)
        }
    }
}
