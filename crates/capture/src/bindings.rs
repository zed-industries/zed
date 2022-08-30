#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use objc::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn dispatch_get_main_queue() -> dispatch_queue_t {
    unsafe { std::mem::transmute(&_dispatch_main_q) }
}
