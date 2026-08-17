pub mod ffi;

use self::ffi::*;
use super::{Context, Keymap, KeymapCompileFlags, State};
use as_raw_xcb_connection::AsRawXcbConnection;
use std::mem;

pub const MIN_MAJOR_XKB_VERSION: u16 = 1;
pub const MIN_MINOR_XKB_VERSION: u16 = 0;

#[repr(C)]
pub enum SetupXkbExtensionFlags {
    /** Do not apply any flags. */
    NoFlags = 0,
}

pub fn setup_xkb_extension(
    connection: impl AsRawXcbConnection,
    major_xkb_version: u16,
    minor_xkb_version: u16,
    flags: SetupXkbExtensionFlags,
    major_xkb_version_out: &mut u16,
    minor_xkb_version_out: &mut u16,
    base_event_out: &mut u8,
    base_error_out: &mut u8,
) -> bool {
    unsafe {
        xkb_x11_setup_xkb_extension(
            connection.as_raw_xcb_connection(),
            major_xkb_version,
            minor_xkb_version,
            mem::transmute(flags),
            major_xkb_version_out,
            minor_xkb_version_out,
            base_event_out,
            base_error_out,
        ) != 0
    }
}

#[must_use]
pub fn get_core_keyboard_device_id(connection: impl AsRawXcbConnection) -> i32 {
    unsafe { xkb_x11_get_core_keyboard_device_id(connection.as_raw_xcb_connection()) as i32 }
}

#[must_use]
pub fn keymap_new_from_device(
    context: &Context,
    connection: impl AsRawXcbConnection,
    device_id: i32,
    flags: KeymapCompileFlags,
) -> Keymap {
    unsafe {
        Keymap::from_raw_ptr(xkb_x11_keymap_new_from_device(
            context.get_raw_ptr(),
            connection.as_raw_xcb_connection(),
            device_id,
            flags,
        ))
    }
}

#[must_use]
pub fn state_new_from_device(
    keymap: &Keymap,
    connection: impl AsRawXcbConnection,
    device_id: i32,
) -> State {
    unsafe {
        State::from_raw_ptr(xkb_x11_state_new_from_device(
            keymap.get_raw_ptr(),
            connection.as_raw_xcb_connection(),
            device_id,
        ))
    }
}
