#![no_std]
#[allow(unused_imports)]
use libc::{c_int, c_short, c_uint, c_ushort, c_void, intptr_t, size_t, timespec, uintptr_t};

#[cfg(not(target_os = "netbsd"))]
use core::ptr;

pub mod constants;

pub use self::constants::*;

#[cfg(not(target_os = "netbsd"))]
pub type EventListSize = c_int;

#[cfg(target_os = "netbsd")]
pub type EventListSize = size_t;

#[cfg(all(not(target_os = "netbsd"), not(target_os = "freebsd")))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    pub ident: uintptr_t,
    pub filter: EventFilter,
    pub flags: EventFlag,
    pub fflags: FilterFlag,
    pub data: i64,
    pub udata: *mut c_void,
}

#[cfg(target_os = "netbsd")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    pub ident: uintptr_t,
    pub filter: EventFilter,
    pub flags: EventFlag,
    pub fflags: FilterFlag,
    pub data: i64,
    pub udata: intptr_t,
}

#[cfg(target_os = "freebsd")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    pub ident: uintptr_t,
    pub filter: EventFilter,
    pub flags: EventFlag,
    pub fflags: FilterFlag,
    pub data: i64,
    pub udata: *mut c_void,
    pub ext: [i64; 4],
}

impl kevent {
    #[cfg(all(not(target_os = "netbsd"), not(target_os = "freebsd")))]
    pub fn new(
        ident: uintptr_t,
        filter: EventFilter,
        flags: EventFlag,
        fflags: FilterFlag,
    ) -> kevent {
        kevent {
            ident,
            filter,
            flags,
            fflags,
            data: 0,
            udata: ptr::null_mut(),
        }
    }

    #[cfg(target_os = "netbsd")]
    pub fn new(
        ident: uintptr_t,
        filter: EventFilter,
        flags: EventFlag,
        fflags: FilterFlag,
    ) -> kevent {
        kevent {
            ident,
            filter,
            flags,
            fflags,
            data: 0,
            udata: 0,
        }
    }

    #[cfg(target_os = "freebsd")]
    pub fn new(
        ident: uintptr_t,
        filter: EventFilter,
        flags: EventFlag,
        fflags: FilterFlag,
    ) -> kevent {
        kevent {
            ident,
            filter,
            flags,
            fflags,
            data: 0,
            udata: ptr::null_mut(),
            ext: [0; 4],
        }
    }
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn kqueue() -> c_int;

    pub fn kevent(
        kq: c_int,
        changelist: *const kevent,
        nchanges: EventListSize,
        eventlist: *mut kevent,
        nevents: EventListSize,
        timeout: *const timespec,
    ) -> c_int;

    #[cfg(target_os = "netbsd")]
    pub fn kqueue1(flags: c_int) -> c_int;
}

#[cfg(test)]
mod test {
    use super::kqueue;

    #[test]
    fn test_kqueue() {
        unsafe {
            assert!(kqueue() > 0);
        }
    }
}
