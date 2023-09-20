use super::ns_string;
use crate::{platform, point, px, size, Bounds, Pixels, PlatformScreen};
use cocoa::{
    appkit::NSScreen,
    base::{id, nil},
    foundation::{NSArray, NSDictionary, NSPoint, NSRect, NSSize},
};
use core_foundation::{
    number::{kCFNumberIntType, CFNumberGetValue, CFNumberRef},
    uuid::{CFUUIDGetUUIDBytes, CFUUIDRef},
};
use core_graphics::display::CGDirectDisplayID;
use std::{any::Any, ffi::c_void};
use uuid::Uuid;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    pub fn CGDisplayCreateUUIDFromDisplayID(display: CGDirectDisplayID) -> CFUUIDRef;
}

#[derive(Debug)]
pub struct MacScreen {
    pub(crate) native_screen: id,
}

impl MacScreen {
    /// Get the screen with the given UUID.
    pub fn find_by_id(uuid: Uuid) -> Option<Self> {
        Self::all().find(|screen| platform::MacScreen::display_uuid(screen) == Some(uuid))
    }

    /// Get the primary screen - the one with the menu bar, and whose bottom left
    /// corner is at the origin of the AppKit coordinate system.
    fn primary() -> Self {
        Self::all().next().unwrap()
    }

    pub fn all() -> impl Iterator<Item = Self> {
        unsafe {
            let native_screens = NSScreen::screens(nil);
            (0..NSArray::count(native_screens)).map(move |ix| MacScreen {
                native_screen: native_screens.objectAtIndex(ix),
            })
        }
    }

    /// Convert the given rectangle in screen coordinates from GPUI's
    /// coordinate system to the AppKit coordinate system.
    ///
    /// In GPUI's coordinates, the origin is at the top left of the primary screen, with
    /// the Y axis pointing downward. In the AppKit coordindate system, the origin is at the
    /// bottom left of the primary screen, with the Y axis pointing upward.
    pub(crate) fn screen_bounds_to_native(bounds: Bounds<Pixels>) -> NSRect {
        let primary_screen_height =
            px(unsafe { Self::primary().native_screen.frame().size.height } as f32);

        NSRect::new(
            NSPoint::new(
                bounds.origin.x.into(),
                (primary_screen_height - bounds.origin.y - bounds.size.height).into(),
            ),
            NSSize::new(bounds.size.width.into(), bounds.size.height.into()),
        )
    }

    /// Convert the given rectangle in screen coordinates from the AppKit
    /// coordinate system to GPUI's coordinate system.
    ///
    /// In GPUI's coordinates, the origin is at the top left of the primary screen, with
    /// the Y axis pointing downward. In the AppKit coordindate system, the origin is at the
    /// bottom left of the primary screen, with the Y axis pointing upward.
    pub(crate) fn screen_bounds_from_native(rect: NSRect) -> Bounds<Pixels> {
        let primary_screen_height = unsafe { Self::primary().native_screen.frame().size.height };
        Bounds {
            origin: point(
                px(rect.origin.x as f32),
                px((primary_screen_height - rect.origin.y - rect.size.height) as f32),
            ),
            size: size(px(rect.size.width as f32), px(rect.size.height as f32)),
        }
    }
}

impl PlatformScreen for MacScreen {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn display_uuid(&self) -> Option<uuid::Uuid> {
        unsafe {
            // Screen ids are not stable. Further, the default device id is also unstable across restarts.
            // CGDisplayCreateUUIDFromDisplayID is stable but not exposed in the bindings we use.
            // This approach is similar to that which winit takes
            // https://github.com/rust-windowing/winit/blob/402cbd55f932e95dbfb4e8b5e8551c49e56ff9ac/src/platform_impl/macos/monitor.rs#L99
            let device_description = self.native_screen.deviceDescription();

            let key = ns_string("NSScreenNumber");
            let device_id_obj = device_description.objectForKey_(key);
            if device_id_obj.is_null() {
                // Under some circumstances, especially display re-arrangements or display locking, we seem to get a null pointer
                // to the device id. See: https://linear.app/zed-industries/issue/Z-257/lock-screen-crash-with-multiple-monitors
                return None;
            }

            let mut device_id: u32 = 0;
            CFNumberGetValue(
                device_id_obj as CFNumberRef,
                kCFNumberIntType,
                (&mut device_id) as *mut _ as *mut c_void,
            );
            let cfuuid = CGDisplayCreateUUIDFromDisplayID(device_id as CGDirectDisplayID);
            if cfuuid.is_null() {
                return None;
            }

            let bytes = CFUUIDGetUUIDBytes(cfuuid);
            Some(Uuid::from_bytes([
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
    }

    fn bounds(&self) -> Bounds<Pixels> {
        unsafe { Self::screen_bounds_from_native(self.native_screen.frame()) }
    }

    fn content_bounds(&self) -> Bounds<Pixels> {
        unsafe { Self::screen_bounds_from_native(self.native_screen.visibleFrame()) }
    }
}
