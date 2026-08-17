use crate::AuthInfo;

use super::ext::*;
use libc::{c_char, c_int, c_uint, c_void};

/// Current protocol version
pub const X_PROTOCOL: u32 = 11;

/// Current minor version
pub const X_PROTOCOL_REVISION: u32 = 0;

/// X_TCP_PORT + display number = server port for TCP transport
pub const X_TCP_PORT: u32 = 6000;

/// xcb connection errors because of socket, pipe and other stream errors.
pub const XCB_CONN_ERROR: i32 = 1;

/// xcb connection shutdown because of extension not supported
pub const XCB_CONN_CLOSED_EXT_NOTSUPPORTED: i32 = 2;

/// malloc(), calloc() and realloc() error upon failure, for eg ENOMEM
pub const XCB_CONN_CLOSED_MEM_INSUFFICIENT: i32 = 3;

/// Connection closed, exceeding request length that server accepts.
pub const XCB_CONN_CLOSED_REQ_LEN_EXCEED: i32 = 4;

/// Connection closed, error during parsing display string.
pub const XCB_CONN_CLOSED_PARSE_ERR: i32 = 5;

/// Connection closed because the server does not have a screen matching the display.
pub const XCB_CONN_CLOSED_INVALID_SCREEN: i32 = 6;

/// Connection closed because some FD passing operation failed
pub const XCB_CONN_CLOSED_FDPASSING_FAILED: i32 = 7;

/// XCB Connection structure.
///
/// A structure that contain all data that  XCB needs to communicate with an X server.
pub enum xcb_connection_t {}

pub(crate) enum xcb_special_event_t {}

/// Generic event.
///
/// A generic event structure.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_generic_event_t {
    /// Type of the response
    pub response_type: u8,
    /// Padding
    pub pad0: u8,
    /// Sequence number
    pub sequence: u16,
    /// Padding
    pub pad: [u32; 7],
    /// full sequence
    pub full_sequence: u32,
}

// GE_GENERIC stuff is actually generated in X-proto, but we need it here in FFI form,
// so it is just copy pasted from xproto.h

/// `response_type` number corresponding to a [xcb_ge_generic_event_t].
pub const XCB_GE_GENERIC: u8 = 35;

/// FFI type for the Generic Event Extension.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ge_generic_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 22],
    pub full_sequence: u32,
}

/// Generic error
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5],
    pub full_sequence: u32,
}

/// FFI type for a void request cookie.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct xcb_void_cookie_t {
    pub(crate) seq: u32,
}

/// Container for authorization information.
/// A container for authorization information to be sent to the X server
#[repr(C)]
pub(crate) struct xcb_auth_info_t {
    /// length of the string name (as returned by strlen)
    pub namelen: c_int,
    /// String containing the authentication protocol name,
    /// such as "MIT-MAGIC-COOKIE-1" or "XDM-AUTHORIZATION-1".
    pub name: *mut c_char,
    /// length of the data member
    pub datalen: c_int,
    /// data interpreted in a protocol specific manner
    pub data: *mut c_char,
}

#[link(name = "xcb")]
extern "C" {
    pub(crate) fn xcb_flush(c: *mut xcb_connection_t) -> c_int;

    pub(crate) fn xcb_get_maximum_request_length(c: *mut xcb_connection_t) -> u32;

    pub(crate) fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t) -> c_void;

    pub(crate) fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;

    pub(crate) fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;

    pub(crate) fn xcb_poll_for_queued_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;

    pub(crate) fn xcb_poll_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t;

    pub(crate) fn xcb_wait_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t;

    pub(crate) fn xcb_register_for_special_xge(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
        eid: u32,
        stamp: *mut u32,
    ) -> *mut xcb_special_event_t;

    pub(crate) fn xcb_unregister_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    );

    pub(crate) fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;

    pub(crate) fn xcb_discard_reply(c: *mut xcb_connection_t, sequence: c_uint);

    pub(crate) fn xcb_discard_reply64(c: *mut xcb_connection_t, sequence: u64);

    // We trick the result from `*const xcb_query_extension_reply_t` to `*const u8` to be compatible with
    // `x::QueryExtensionReply::from_raw`
    pub(crate) fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const u8;

    pub(crate) fn xcb_prefetch_extension_data(c: *mut xcb_connection_t, ext: *mut xcb_extension_t);

    // We trick the result from `*const xcb_setup_t` to `*const u8` to be compatible with
    // `x::Setup::from_data`
    pub(crate) fn xcb_get_setup(c: *mut xcb_connection_t) -> *const u8;

    pub(crate) fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> c_int;

    pub(crate) fn xcb_connection_has_error(c: *mut xcb_connection_t) -> c_int;

    pub(crate) fn xcb_connect_to_fd(
        fd: c_int,
        auth_info: *mut xcb_auth_info_t,
    ) -> *mut xcb_connection_t;

    pub(crate) fn xcb_disconnect(c: *mut xcb_connection_t);

    pub(crate) fn xcb_parse_display(
        name: *const c_char,
        host: *mut *mut c_char,
        display: *mut c_int,
        screen: *mut c_int,
    ) -> c_int;

    pub(crate) fn xcb_connect(
        displayname: *const c_char,
        screenp: *mut c_int,
    ) -> *mut xcb_connection_t;

    pub(crate) fn xcb_connect_to_display_with_auth_info(
        display: *const c_char,
        auth: *mut xcb_auth_info_t,
        screen: *mut c_int,
    ) -> *mut xcb_connection_t;

    pub(crate) fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;

    pub(crate) fn xcb_total_read(c: *mut xcb_connection_t) -> u64;

    pub(crate) fn xcb_total_written(c: *mut xcb_connection_t) -> u64;
}
