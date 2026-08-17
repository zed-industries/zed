#[cfg(target_os = "linux")]
mod canonicalize_impl;
#[cfg(target_os = "linux")]
mod file_metadata;
mod file_path;
#[cfg(target_os = "linux")]
mod open_entry_impl;
mod open_impl;
mod procfs;
mod set_permissions_impl;
mod set_times_impl;
#[cfg(target_os = "linux")]
mod stat_impl;

#[cfg(target_os = "android")]
pub(crate) use crate::fs::manually::canonicalize as canonicalize_impl;
#[cfg(target_os = "android")]
pub(crate) use crate::fs::manually::open_entry as open_entry_impl;
#[cfg(target_os = "android")]
pub(crate) use crate::fs::manually::stat as stat_impl;
pub(crate) use crate::fs::via_parent::set_times_nofollow as set_times_nofollow_impl;
#[cfg(target_os = "linux")]
pub(crate) use canonicalize_impl::canonicalize_impl;
pub(crate) use file_path::file_path;
#[cfg(target_os = "linux")]
pub(crate) use open_entry_impl::open_entry_impl;
#[cfg(target_os = "linux")]
pub(crate) use open_impl::open_beneath;
pub(crate) use open_impl::open_impl;
pub(crate) use set_permissions_impl::set_permissions_impl;
pub(crate) use set_times_impl::set_times_impl;
#[cfg(target_os = "linux")]
pub(crate) use stat_impl::stat_impl;

// In theory we could optimize `link` using `openat2` with `O_PATH` and
// `linkat` with `AT_EMPTY_PATH`, however that requires `CAP_DAC_READ_SEARCH`,
// so it isn't very widely applicable.
