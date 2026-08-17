//! Header: `sys/socket.h`
//!
//! <https://github.com/NetBSD/src/blob/trunk/sys/sys/socket.h>
//!

use crate::prelude::*;

s_no_extra_traits! {
    pub union __c_anonymous_pcb_sockaddr_src {
        pub _kis_src: crate::sockaddr,
        _kis_pad: Padding<[c_char; 256 + 8]>,
    }

    pub union __c_anonymous_pcb_sockaddr_dst {
        pub _kid_dst: crate::sockaddr,
        _kid_pad: Padding<[c_char; 256 + 8]>,
    }

    pub struct kinfo_pcb {
        pub ki_pcbaddr: u64,
        pub ki_ppcbaddr: u64,
        pub ki_sockaddr: u64,
        pub ki_family: u32,
        pub ki_type: u32,
        pub ki_protocol: u32,
        pub ki_pflags: u32,
        pub ki_sostate: u32,
        pub ki_prstate: u32,
        pub ki_tstate: i32,
        pub ki_tflags: u32,
        pub ki_rcvq: u64,
        pub ki_sndq: u64,
        pub ki_s: __c_anonymous_pcb_sockaddr_src,
        pub ki_d: __c_anonymous_pcb_sockaddr_dst,
        pub ki_inode: u64,
        pub ki_vnode: u64,
        pub ki_conn: u64,
        pub ki_refs: u64,
        pub ki_nextref: u64,
    }
}

pub const PCB_SLOP: c_int = 20;
pub const PCB_ALL: c_int = 0;
