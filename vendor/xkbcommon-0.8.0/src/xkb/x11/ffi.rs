use crate::xkb::ffi::{xkb_context, xkb_keymap, xkb_keymap_compile_flags, xkb_state};

use as_raw_xcb_connection::xcb_connection_t;
use std::os::raw::c_int;

pub const XKB_X11_MIN_MAJOR_XKB_VERSION: u16 = 1;
pub const XKB_X11_MIN_MINOR_XKB_VERSION: u16 = 0;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum xkb_x11_setup_xkb_extension_flags {
    /** Do not apply any flags. */
    NO_FLAGS = 0,
}

#[link(name = "xkbcommon-x11")]
extern "C" {

    pub fn xkb_x11_setup_xkb_extension(
        connection: *mut xcb_connection_t,
        major_xkb_version: u16,
        minor_xkb_version: u16,
        flags: xkb_x11_setup_xkb_extension_flags,
        major_xkb_version_out: *mut u16,
        minor_xkb_version_out: *mut u16,
        base_event_out: *mut u8,
        base_error_out: *mut u8,
    ) -> c_int;

    pub fn xkb_x11_get_core_keyboard_device_id(connection: *mut xcb_connection_t) -> i32;

    pub fn xkb_x11_keymap_new_from_device(
        context: *mut xkb_context,
        connection: *mut xcb_connection_t,
        device_id: i32,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;

    pub fn xkb_x11_state_new_from_device(
        keymap: *mut xkb_keymap,
        connection: *mut xcb_connection_t,
        device_id: i32,
    ) -> *mut xkb_state;

}
