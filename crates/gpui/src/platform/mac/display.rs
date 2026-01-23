use super::ns_string;
use crate::{Bounds, DisplayId, Pixels, PlatformDisplay, point, px, size};
use anyhow::Result;
use cocoa::{
    appkit::NSScreen,
    base::{id, nil},
    foundation::{NSArray, NSDictionary},
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
            let screen_number_key: id = ns_string("NSScreenNumber");
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
unsafe extern "C" {
    fn CGDisplayCreateUUIDFromDisplayID(display: CGDirectDisplayID) -> CFUUIDRef;
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

    fn bounds(&self) -> Bounds<Pixels> {
        unsafe {
            // CGDisplayBounds is in "global display" coordinates, where 0 is
            // the top left of the primary display.
            let bounds = CGDisplayBounds(self.0);

            Bounds {
                origin: Default::default(),
                size: size(px(bounds.size.width as f32), px(bounds.size.height as f32)),
            }
        }
    }

    fn visible_bounds(&self) -> Bounds<Pixels> {
        unsafe {
            let dominated_screen = self.get_nsscreen();

            if dominated_screen == nil {
                return self.bounds();
            }

            let screen_frame = NSScreen::frame(dominated_screen);
            let visible_frame = NSScreen::visibleFrame(dominated_screen);

            // Convert from bottom-left origin (AppKit) to top-left origin
            let origin_y =
                screen_frame.size.height - visible_frame.origin.y - visible_frame.size.height
                    + screen_frame.origin.y;

            Bounds {
                origin: point(
                    px(visible_frame.origin.x as f32 - screen_frame.origin.x as f32),
                    px(origin_y as f32),
                ),
                size: size(
                    px(visible_frame.size.width as f32),
                    px(visible_frame.size.height as f32),
                ),
            }
        }
    }
}

impl MacDisplay {
    /// Find the NSScreen corresponding to this display
    unsafe fn get_nsscreen(&self) -> id {
        let screens = unsafe { NSScreen::screens(nil) };
        let count = unsafe { NSArray::count(screens) };
        let screen_number_key: id = unsafe { ns_string("NSScreenNumber") };

        for i in 0..count {
            let screen = unsafe { NSArray::objectAtIndex(screens, i) };
            let device_description = unsafe { NSScreen::deviceDescription(screen) };
            let screen_number = unsafe { device_description.objectForKey_(screen_number_key) };
            let screen_id: CGDirectDisplayID = msg_send![screen_number, unsignedIntegerValue];
            if screen_id == self.0 {
                return screen;
            }
        }
        nil
    }
}
