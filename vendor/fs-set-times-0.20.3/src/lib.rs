//! `fs-set-times` provides functions to set timestamps on files, directories,
//! and other filesystem objects.
//!
//! On Windows, modifying a file's timestamp requires write access to the file.

#![deny(missing_docs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(not(windows), forbid(unsafe_code))]

mod set_times;
mod system_time_spec;

pub use set_times::{set_atime, set_mtime, set_symlink_times, set_times, SetTimes};
pub use system_time_spec::SystemTimeSpec;
