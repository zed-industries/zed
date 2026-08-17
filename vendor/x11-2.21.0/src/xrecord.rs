// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_int, c_uchar, c_ulong, c_ushort};

use super::xlib::{Bool, Display, Time, XID};

//
// functions
//

x11_link! { Xf86vmode, xtst, ["libXtst.so.6", "libXtst.so"], 14,
  pub fn XRecordAllocRange () -> *mut XRecordRange,
  pub fn XRecordCreateContext (_6: *mut Display, _5: c_int, _4: *mut c_ulong, _3: c_int, _2: *mut *mut XRecordRange, _1: c_int) -> c_ulong,
  pub fn XRecordDisableContext (_2: *mut Display, _1: c_ulong) -> c_int,
  pub fn XRecordEnableContext (_4: *mut Display, _3: c_ulong, _2: Option<unsafe extern "C" fn (*mut c_char, *mut XRecordInterceptData)>, _1: *mut c_char) -> c_int,
  pub fn XRecordEnableContextAsync (_4: *mut Display, _3: c_ulong, _2: Option<unsafe extern "C" fn (*mut c_char, *mut XRecordInterceptData)>, _1: *mut c_char) -> c_int,
  pub fn XRecordFreeContext (_2: *mut Display, _1: c_ulong) -> c_int,
  pub fn XRecordFreeData (_1: *mut XRecordInterceptData) -> (),
  pub fn XRecordFreeState (_1: *mut XRecordState) -> (),
  pub fn XRecordGetContext (_3: *mut Display, _2: c_ulong, _1: *mut *mut XRecordState) -> c_int,
  pub fn XRecordIdBaseMask (_1: *mut Display) -> c_ulong,
  pub fn XRecordProcessReplies (_1: *mut Display) -> (),
  pub fn XRecordQueryVersion (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XRecordRegisterClients (_7: *mut Display, _6: c_ulong, _5: c_int, _4: *mut c_ulong, _3: c_int, _2: *mut *mut XRecordRange, _1: c_int) -> c_int,
  pub fn XRecordUnregisterClients (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int,
variadic:
globals:
}

//
// constants
//

pub const XRecordFromServerTime: c_int = 0x01;
pub const XRecordFromClientTime: c_int = 0x02;
pub const XRecordFromClientSequence: c_int = 0x04;

pub const XRecordCurrentClients: c_ulong = 1;
pub const XRecordFutureClients: c_ulong = 2;
pub const XRecordAllClients: c_ulong = 3;

pub const XRecordFromServer: c_int = 0;
pub const XRecordFromClient: c_int = 1;
pub const XRecordClientStarted: c_int = 2;
pub const XRecordClientDied: c_int = 3;
pub const XRecordStartOfData: c_int = 4;
pub const XRecordEndOfData: c_int = 5;

//
// types
//

pub type XRecordClientSpec = c_ulong;
pub type XRecordContext = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordClientInfo {
    pub client: XRecordClientSpec,
    pub nranges: c_ulong,
    pub ranges: *mut *mut XRecordRange,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordExtRange {
    pub ext_major: XRecordRange8,
    pub ext_minor: XRecordRange16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordInterceptData {
    pub id_base: XID,
    pub server_time: Time,
    pub client_seq: c_ulong,
    pub category: c_int,
    pub client_swapped: Bool,
    pub data: *mut c_uchar,
    pub data_len: c_ulong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordRange {
    pub core_requests: XRecordRange8,
    pub core_replies: XRecordRange8,
    pub ext_requests: XRecordExtRange,
    pub ext_replies: XRecordExtRange,
    pub delivered_events: XRecordRange8,
    pub device_events: XRecordRange8,
    pub errors: XRecordRange8,
    pub client_started: Bool,
    pub client_died: Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordRange8 {
    pub first: c_uchar,
    pub last: c_uchar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordRange16 {
    pub first: c_ushort,
    pub last: c_ushort,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordState {
    pub enabled: Bool,
    pub datum_flags: c_int,
    pub nclients: c_ulong,
    pub client_info: *mut *mut XRecordClientInfo,
}
