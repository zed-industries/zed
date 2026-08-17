// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(non_camel_case_types, dead_code)]

use libc::{c_int, c_void};

// definitions come from:
// https://github.com/freebsd/freebsd-src/blob/main/lib/libdevstat/devstat.h
// https://github.com/freebsd/freebsd-src/blob/main/sys/sys/devicestat.h

// #[repr(C)]
// pub(crate) struct statinfo {
//     pub(crate) cp_time: [c_long; CPUSTATES as usize],
//     pub(crate) tk_nin: c_long,
//     pub(crate) tk_nout: c_long,
//     pub(crate) dinfo: *mut devinfo,
//     pub(crate) snap_time: c_long_double,
// }

pub(crate) const DEVSTAT_READ: usize = 0x01;
pub(crate) const DEVSTAT_WRITE: usize = 0x02;

// pub(crate) const DSM_NONE: c_int = 0;
// pub(crate) const DSM_TOTAL_BYTES_READ: c_int = 2;
// pub(crate) const DSM_TOTAL_BYTES_WRITE: c_int = 3;

// extern "C" {
//     pub(crate) fn devstat_compute_statistics(current: *mut devstat, previous: *mut devstat, etime: c_long_double, ...) -> c_int;
// }

#[link(name = "geom")]
unsafe extern "C" {
    pub(crate) fn geom_stats_open() -> c_int;
    pub(crate) fn geom_stats_snapshot_get() -> *mut c_void;
    pub(crate) fn geom_stats_snapshot_next(arg: *mut c_void) -> *mut libc::devstat;
    pub(crate) fn geom_stats_snapshot_reset(arg: *mut c_void);
    pub(crate) fn geom_stats_snapshot_free(arg: *mut c_void);
}
