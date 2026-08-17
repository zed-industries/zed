//! Header: `sys/file.h`
//!
//! https://github.com/freebsd/freebsd-src/blob/main/sys/sys/file.h

use crate::prelude::*;

pub const DTYPE_NONE: c_int = 0;
pub const DTYPE_VNODE: c_int = 1;
pub const DTYPE_SOCKET: c_int = 2;
pub const DTYPE_PIPE: c_int = 3;
pub const DTYPE_FIFO: c_int = 4;
pub const DTYPE_KQUEUE: c_int = 5;
pub const DTYPE_CRYPTO: c_int = 6;
pub const DTYPE_MQUEUE: c_int = 7;
pub const DTYPE_SHM: c_int = 8;
pub const DTYPE_SEM: c_int = 9;
pub const DTYPE_PTS: c_int = 10;
pub const DTYPE_DEV: c_int = 11;
pub const DTYPE_PROCDESC: c_int = 12;
pub const DTYPE_EVENTFD: c_int = 13;
pub const DTYPE_TIMERFD: c_int = 14;
pub const DTYPE_INOTIFY: c_int = 15;
pub const DTYPE_JAILDESC: c_int = 16;

s! {
    #[cfg(not(any(freebsd10, freebsd11)))]
    pub struct xfile {
        pub xf_size: crate::ksize_t,
        pub xf_pid: crate::pid_t,
        pub xf_uid: crate::uid_t,
        pub xf_fd: c_int,
        _xf_int_pad1: Padding<c_int>,
        pub xf_file: crate::kvaddr_t,
        pub xf_type: c_short,
        _xf_short_pad1: Padding<c_short>,
        pub xf_count: c_int,
        pub xf_msgcount: c_int,
        _xf_int_pad2: Padding<c_int>,
        pub xf_offset: crate::off_t,
        pub xf_data: crate::kvaddr_t,
        pub xf_vnode: crate::kvaddr_t,
        pub xf_flag: c_uint,
        _xf_int_pad3: Padding<c_int>,
        _xf_int64_pad: Padding<[i64; 6]>,
    }
}
