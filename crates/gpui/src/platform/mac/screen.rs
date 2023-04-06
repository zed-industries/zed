use std::{any::Any, ffi::c_void};

use crate::platform;
use cocoa::{
    appkit::NSScreen,
    base::{id, nil},
    foundation::{NSArray, NSDictionary},
};
use core_foundation::{
    number::{kCFNumberIntType, CFNumberGetValue, CFNumberRef},
    uuid::{CFUUIDGetUUIDBytes, CFUUIDRef},
};
use core_graphics::display::CGDirectDisplayID;
use pathfinder_geometry::rect::RectF;
use uuid::Uuid;

use super::{geometry::NSRectExt, ns_string};

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    pub fn CGDisplayCreateUUIDFromDisplayID(display: CGDirectDisplayID) -> CFUUIDRef;
}

#[derive(Debug)]
pub struct Screen {
    pub(crate) native_screen: id,
}

impl Screen {
    pub fn find_by_id(uuid: Uuid) -> Option<Self> {
        unsafe {
            let native_screens = NSScreen::screens(nil);
            (0..NSArray::count(native_screens))
                .into_iter()
                .map(|ix| Screen {
                    native_screen: native_screens.objectAtIndex(ix),
                })
                .find(|screen| platform::Screen::display_uuid(screen) == Some(uuid))
        }
    }

    pub fn all() -> Vec<Self> {
        let mut screens = Vec::new();
        unsafe {
            let native_screens = NSScreen::screens(nil);
            for ix in 0..NSArray::count(native_screens) {
                screens.push(Screen {
                    native_screen: native_screens.objectAtIndex(ix),
                });
            }
        }
        screens
    }
}

impl platform::Screen for Screen {
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

    fn bounds(&self) -> RectF {
        unsafe {
            let frame = self.native_screen.frame();
            frame.to_rectf()
        }
    }
}
