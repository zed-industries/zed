//! This module corresponds to `mach/port.h`

use vm_types::{integer_t, natural_t};

pub type mach_port_name_t = natural_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct ipc_port;

pub type ipc_port_t = *mut ipc_port;

pub type mach_port_t = ::libc::c_uint;
pub type mach_port_array_t = *mut mach_port_t;

pub const MACH_PORT_NULL: mach_port_t = 0;
pub const MACH_PORT_DEAD: mach_port_t = !0;

pub type mach_port_right_t = natural_t;

pub const MACH_PORT_RIGHT_SEND: mach_port_right_t = 0;
pub const MACH_PORT_RIGHT_RECEIVE: mach_port_right_t = 1;
pub const MACH_PORT_RIGHT_SEND_ONCE: mach_port_right_t = 2;
pub const MACH_PORT_RIGHT_PORT_SET: mach_port_right_t = 3;
pub const MACH_PORT_RIGHT_DEAD_NAME: mach_port_right_t = 4;
pub const MACH_PORT_RIGHT_LABELH: mach_port_right_t = 5;
pub const MACH_PORT_RIGHT_NUMBER: mach_port_right_t = 6;

pub type mach_port_urefs_t = natural_t;
pub type mach_port_delta_t = integer_t;

pub type mach_port_seqno_t = natural_t;
pub type mach_port_mscount_t = natural_t;
pub type mach_port_msgcount_t = natural_t;
pub type mach_port_rights_t = natural_t;

pub const MACH_PORT_QLIMIT_ZERO: mach_port_msgcount_t = 0;
pub const MACH_PORT_QLIMIT_BASIC: mach_port_msgcount_t = 5;
pub const MACH_PORT_QLIMIT_SMALL: mach_port_msgcount_t = 16;
pub const MACH_PORT_QLIMIT_LARGE: mach_port_msgcount_t = 1024;
pub const MACH_PORT_QLIMIT_KERNEL: mach_port_msgcount_t = 65534;
pub const MACH_PORT_QLIMIT_MIN: mach_port_msgcount_t = MACH_PORT_QLIMIT_ZERO;
pub const MACH_PORT_QLIMIT_DEFAULT: mach_port_msgcount_t = MACH_PORT_QLIMIT_BASIC;
pub const MACH_PORT_QLIMIT_MAX: mach_port_msgcount_t = MACH_PORT_QLIMIT_LARGE;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_port_limits_t {
    pub mpl_qlimit: mach_port_msgcount_t,
}

pub const MPO_CONTEXT_AS_GUARD: u32 = 1;
pub const MPO_QLIMIT: u32 = 2;
pub const MPO_TEMPOWNER: u32 = 4;
pub const MPO_IMPORTANCE_RECEIVER: u32 = 8;
pub const MPO_INSERT_SEND_RIGHT: u32 = 0x10;
pub const MPO_STRICT: u32 = 0x20;
pub const MPO_DENAP_RECEIVER: u32 = 0x40;
pub const MPO_IMMOVABLE_RECEIVE: u32 = 0x80;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_port_options_t {
    pub flags: u32,
    pub mpl: mach_port_limits_t,
    pub reserved: [u64; 2],
}
