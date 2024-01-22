use crate::{point, size, Bounds, DisplayId, GlobalPixels, PlatformDisplay};
use anyhow::Result;
use cocoa::{
    appkit::NSScreen,
    base::{id, nil},
    foundation::{NSDictionary, NSPoint, NSRect, NSSize, NSString},
};
use core_foundation::uuid::{CFUUIDGetUUIDBytes, CFUUIDRef};
use core_graphics::display::{CGDirectDisplayID, CGDisplayBounds, CGGetActiveDisplayList};
use objc::{msg_send, sel, sel_impl};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct MacDisplay(pub(crate) CGDirectDisplayID);

unsafe impl Send for MacDisplay {}

impl MacDisplay {
    /// Get the screen with the given [`DisplayId`].
    pub fn find_by_id(id: DisplayId) -> Option<Self> {
        Self::all().find(|screen| screen.id() == id)
    }

    /// Get the primary screen - the one with the menu bar, and whose bottom left
    /// corner is at the origin of the AppKit coordinate system.
    pub fn primary() -> Self {
        // Instead of iterating through all active systems displays via `all()` we use the first
        // NSScreen and gets its CGDirectDisplayID, because we can't be sure that `CGGetActiveDisplayList`
        // will always return a list of active displays (machine might be sleeping).
        //
        // The following is what Chromium does too:
        //
        // https://chromium.googlesource.com/chromium/src/+/66.0.3359.158/ui/display/mac/screen_mac.mm#56
        unsafe {
            let screens = NSScreen::screens(nil);
            let screen = cocoa::foundation::NSArray::objectAtIndex(screens, 0);
            let device_description = NSScreen::deviceDescription(screen);
            let screen_number_key: id = NSString::alloc(nil).init_str("NSScreenNumber");
            let screen_number = device_description.objectForKey_(screen_number_key);
            let screen_number: CGDirectDisplayID = msg_send![screen_number, unsignedIntegerValue];
            Self(screen_number)
        }
    }

    /// Obtains an iterator over all currently active system displays.
    pub fn all() -> impl Iterator<Item = Self> {
        unsafe {
            // We're assuming there aren't more than 32 displays connected to the system.
            let mut displays = Vec::with_capacity(32);
            let mut display_count = 0;
            let result = CGGetActiveDisplayList(
                displays.capacity() as u32,
                displays.as_mut_ptr(),
                &mut display_count,
            );

            if result == 0 {
                displays.set_len(display_count as usize);
                displays.into_iter().map(MacDisplay)
            } else {
                panic!("Failed to get active display list. Result: {result}");
            }
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGDisplayCreateUUIDFromDisplayID(display: CGDirectDisplayID) -> CFUUIDRef;
}

/// Convert the given rectangle from Cocoa's coordinate space to GPUI's coordinate space.
///
/// Cocoa's coordinate space has its origin at the bottom left of the primary screen,
/// with the Y axis pointing upwards.
///
/// Conversely, in GPUI's coordinate system, the origin is placed at the top left of the primary
/// screen, with the Y axis pointing downwards (matching CoreGraphics)
pub(crate) fn global_bounds_from_ns_rect(rect: NSRect) -> Bounds<GlobalPixels> {
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

/// Convert the given rectangle from GPUI's coordinate system to Cocoa's native coordinate space.
///
/// Cocoa's coordinate space has its origin at the bottom left of the primary screen,
/// with the Y axis pointing upwards.
///
/// Conversely, in GPUI's coordinate system, the origin is placed at the top left of the primary
/// screen, with the Y axis pointing downwards (matching CoreGraphics)
pub(crate) fn global_bounds_to_ns_rect(bounds: Bounds<GlobalPixels>) -> NSRect {
    let primary_screen_height = MacDisplay::primary().bounds().size.height;

    NSRect::new(
        NSPoint::new(
            bounds.origin.x.into(),
            (primary_screen_height - bounds.origin.y - bounds.size.height).into(),
        ),
        NSSize::new(bounds.size.width.into(), bounds.size.height.into()),
    )
}

impl PlatformDisplay for MacDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(self.0)
    }

    fn uuid(&self) -> Result<Uuid> {
        let cfuuid = unsafe { CGDisplayCreateUUIDFromDisplayID(self.0 as CGDirectDisplayID) };
        anyhow::ensure!(
            !cfuuid.is_null(),
            "AppKit returned a null from CGDisplayCreateUUIDFromDisplayID"
        );

        let bytes = unsafe { CFUUIDGetUUIDBytes(cfuuid) };
        Ok(Uuid::from_bytes([
            bytes.byte0,
            bytes.byte1,
            bytes.byte2,
            bytes.byte3,
            bytes.byte4,
            bytes.byte5,
            bytes.byte6,
            bytes.byte7,
            bytes.byte8,
            bytes.byte9,
            bytes.byte10,
            bytes.byte11,
            bytes.byte12,
            bytes.byte13,
            bytes.byte14,
            bytes.byte15,
        ]))
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        unsafe {
            // CGDisplayBounds is in "global display" coordinates, where 0 is
            // the top left of the primary display.
            let bounds = CGDisplayBounds(self.0);

            Bounds {
                origin: point(
                    GlobalPixels(bounds.origin.x as f32),
                    GlobalPixels(bounds.origin.y as f32),
                ),
                size: size(
                    GlobalPixels(bounds.size.width as f32),
                    GlobalPixels(bounds.size.height as f32),
                ),
            }
        }
    }
}
