//! Extension traits for `Dir`

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(all(windows, windows_by_handle), feature(windows_by_handle))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

mod dir_entry_ext;
mod dir_ext;
mod file_type_ext;
mod is_file_read_write;
mod metadata_ext;
mod open_options_follow_ext;
mod open_options_maybe_dir_ext;
mod open_options_sync_ext;
mod reopen;

pub use dir_entry_ext::DirEntryExt;
#[cfg(feature = "async_std")]
pub use dir_ext::AsyncDirExt;
#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
pub use dir_ext::AsyncDirExtUtf8;
#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
pub use dir_ext::DirExtUtf8;
pub use dir_ext::{AccessType, DirExt, SystemTimeSpec};
pub use file_type_ext::FileTypeExt;
pub use is_file_read_write::IsFileReadWrite;
pub use metadata_ext::MetadataExt;
pub use open_options_follow_ext::OpenOptionsFollowExt;
pub use open_options_maybe_dir_ext::OpenOptionsMaybeDirExt;
pub use open_options_sync_ext::OpenOptionsSyncExt;
pub use reopen::Reopen;

/// Re-export these to allow them to be used with `Reuse`.
pub use cap_primitives::fs::{
    FollowSymlinks, Metadata, MetadataExt as OsMetadataExt, OpenOptions, OpenOptionsExt,
};

#[doc(hidden)]
pub use cap_primitives::ambient_authority_known_at_compile_time;
pub use cap_primitives::{ambient_authority, AmbientAuthority};
