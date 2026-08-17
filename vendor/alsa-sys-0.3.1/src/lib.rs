#![allow(non_camel_case_types)]

use libc::{FILE, pid_t, timeval, timespec, pollfd};

pub const SND_PCM_NONBLOCK: ::std::os::raw::c_int = 0x1;
pub const SND_PCM_ASYNC: ::std::os::raw::c_int = 0x2;

pub const SND_SEQ_OPEN_OUTPUT: i32 = 1;
pub const SND_SEQ_OPEN_INPUT: i32 = 2;
pub const SND_SEQ_OPEN_DUPLEX: i32 = SND_SEQ_OPEN_OUTPUT | SND_SEQ_OPEN_INPUT;
pub const SND_SEQ_NONBLOCK: i32 = 0x0001;
pub const SND_SEQ_ADDRESS_BROADCAST: u8 = 255;
pub const SND_SEQ_ADDRESS_SUBSCRIBERS: u8 = 254;
pub const SND_SEQ_ADDRESS_UNKNOWN: u8 = 253;
pub const SND_SEQ_QUEUE_DIRECT: u8 = 253;
pub const SND_SEQ_TIME_MODE_MASK: u8 = 1<<1;
pub const SND_SEQ_TIME_STAMP_MASK: u8 = 1<<0;
pub const SND_SEQ_TIME_MODE_REL: u8 = 1<<1;
pub const SND_SEQ_TIME_STAMP_REAL: u8 = 1<<0;
pub const SND_SEQ_TIME_STAMP_TICK: u8 = 0<<0;
pub const SND_SEQ_TIME_MODE_ABS: u8 = 0<<1;
pub const SND_SEQ_CLIENT_SYSTEM: u8 = 0;
pub const SND_SEQ_PORT_SYSTEM_TIMER: u8 = 0;
pub const SND_SEQ_PORT_SYSTEM_ANNOUNCE: u8 = 1;
pub const SND_SEQ_PRIORITY_HIGH: u8 = 1<<4;
pub const SND_SEQ_EVENT_LENGTH_FIXED: u8 = 0<<2;
pub const SND_SEQ_EVENT_LENGTH_MASK: u8 = 3<<2;
pub const SND_SEQ_EVENT_LENGTH_VARIABLE: u8 = 1<<2;
pub const SND_SEQ_EVENT_LENGTH_VARUSR: u8 = 2<<2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}

include!("generated.rs");
