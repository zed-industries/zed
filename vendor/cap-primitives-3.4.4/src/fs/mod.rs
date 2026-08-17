//! Filesystem utilities.

#[cfg(racy_asserts)]
#[macro_use]
pub(crate) mod assert_same_file;

mod access;
mod canonicalize;
mod copy;
mod create_dir;
mod dir_builder;
mod dir_entry;
mod dir_options;
mod file;
#[cfg(not(any(target_os = "android", target_os = "linux", windows)))]
mod file_path_by_searching;
mod file_type;
mod follow_symlinks;
mod hard_link;
mod is_file_read_write;
mod maybe_owned_file;
mod metadata;
mod open;
mod open_ambient;
mod open_dir;
mod open_options;
mod open_unchecked_error;
mod permissions;
mod read_dir;
mod read_link;
mod remove_dir;
mod remove_dir_all;
mod remove_file;
mod remove_open_dir;
mod rename;
mod reopen;
#[cfg(not(target_os = "wasi"))]
mod set_permissions;
mod set_times;
mod stat;
mod symlink;
mod system_time_spec;

pub(crate) mod errors;
pub(crate) mod manually;
pub(crate) mod via_parent;

use maybe_owned_file::MaybeOwnedFile;

#[cfg(not(any(target_os = "android", target_os = "linux", windows)))]
pub(crate) use file_path_by_searching::file_path_by_searching;
pub(crate) use open_unchecked_error::*;

#[cfg(not(windows))]
pub(crate) use super::rustix::fs::*;
#[cfg(windows)]
pub(crate) use super::windows::fs::*;

#[cfg(not(windows))]
pub(crate) use read_dir::{read_dir_nofollow, read_dir_unchecked};

pub use access::{access, AccessModes, AccessType};
pub use canonicalize::canonicalize;
pub use copy::copy;
pub use create_dir::create_dir;
pub use dir_builder::*;
pub use dir_entry::DirEntry;
#[cfg(windows)]
pub use dir_entry::_WindowsDirEntryExt;
pub use dir_options::DirOptions;
pub use file::FileExt;
pub use file_type::FileType;
#[cfg(any(unix, target_os = "vxworks", all(windows, windows_file_type_ext)))]
pub use file_type::FileTypeExt;
#[cfg(windows)]
pub use file_type::_WindowsFileTypeExt;
pub use follow_symlinks::FollowSymlinks;
pub use hard_link::hard_link;
pub use is_file_read_write::is_file_read_write;
#[cfg(windows)]
pub use metadata::_WindowsByHandle;
pub use metadata::{Metadata, MetadataExt};
pub use open::open;
pub use open_ambient::open_ambient;
pub use open_dir::*;
pub use open_options::*;
pub use permissions::Permissions;
#[cfg(unix)]
pub use permissions::PermissionsExt;
pub use read_dir::{read_base_dir, read_dir, ReadDir};
pub use read_link::{read_link, read_link_contents};
pub use remove_dir::remove_dir;
pub use remove_dir_all::remove_dir_all;
pub use remove_file::remove_file;
pub use remove_open_dir::{remove_open_dir, remove_open_dir_all};
pub use rename::rename;
pub use reopen::reopen;
#[cfg(not(target_os = "wasi"))]
pub use set_permissions::{set_permissions, set_symlink_permissions};
pub use set_times::{set_times, set_times_nofollow};
pub use stat::stat;
#[cfg(not(windows))]
pub use symlink::{symlink, symlink_contents};
#[cfg(windows)]
pub use symlink::{symlink_dir, symlink_file};
pub use system_time_spec::SystemTimeSpec;

#[cfg(racy_asserts)]
fn map_result<T: Clone>(result: &std::io::Result<T>) -> Result<T, (std::io::ErrorKind, String)> {
    match result {
        Ok(t) => Ok(t.clone()),
        Err(e) => Err((e.kind(), e.to_string())),
    }
}

/// Test that `file_path` works on a few miscellaneous directory paths.
#[test]
fn dir_paths() {
    use crate::ambient_authority;

    for path in [std::env::current_dir().unwrap(), std::env::temp_dir()] {
        let dir = open_ambient_dir(&path, ambient_authority()).unwrap();
        assert_eq!(
            file_path(&dir)
                .as_ref()
                .map(std::fs::canonicalize)
                .map(Result::unwrap),
            Some(std::fs::canonicalize(path).unwrap())
        );
    }
}
