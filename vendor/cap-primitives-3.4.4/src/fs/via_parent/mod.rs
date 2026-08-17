//! In many operations, the last component of a path is special. For example,
//! in `create_dir`, the last component names the path to be created, while the
//! rest of the components just name the place to create it in.

mod access;
mod create_dir;
mod hard_link;
mod open_parent;
#[cfg(not(windows))] // doesn't work on windows; use a windows-specific impl
mod read_link;
mod remove_dir;
mod remove_file;
mod rename;
#[cfg(windows)]
mod set_permissions;
#[cfg(not(target_os = "wasi"))]
mod set_symlink_permissions;
#[cfg(not(windows))]
mod set_times_nofollow;
mod symlink;

use open_parent::open_parent;

pub(crate) use access::access;
pub(crate) use create_dir::create_dir;
pub(crate) use hard_link::hard_link;
#[cfg(not(windows))] // doesn't work on windows; use a windows-specific impl
pub(crate) use read_link::read_link;
pub(crate) use remove_dir::remove_dir;
pub(crate) use remove_file::remove_file;
pub(crate) use rename::rename;
#[cfg(windows)]
pub(crate) use set_permissions::set_permissions;
#[cfg(not(target_os = "wasi"))]
pub(crate) use set_symlink_permissions::set_symlink_permissions;
#[cfg(not(windows))]
pub(crate) use set_times_nofollow::set_times_nofollow;
#[cfg(not(windows))]
pub(crate) use symlink::symlink;
#[cfg(windows)]
pub(crate) use symlink::{symlink_dir, symlink_file};
