//! A fully UTF-8 filesystem API modeled after [`cap_std::fs`].
//!
//! Where `cap_std::fs` would use `Path` and `PathBuf`, this `fs_utf8` module
//! uses [`Utf8Path`] and [`Utf8PathBuf`], meaning that all paths are valid
//! UTF-8.
//!
//! To use this module, enable the `fs_utf8` cargo feature.
//!
//! If you don't want to restrict paths to UTF-8, use the regular
//! [`cap_std::fs`] module instead.
//!
//! [`cap_std::fs`]: ../fs/

mod dir;
mod dir_entry;
mod file;
mod read_dir;

pub use dir::Dir;
pub use dir_entry::DirEntry;
pub use file::File;
pub use read_dir::ReadDir;

// Re-export things from `cap_std::fs` that we can use as-is.
pub use crate::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};

// Re-export conditional types from `cap_primitives`.
#[cfg(any(unix, target_os = "vxworks", all(windows, windows_file_type_ext)))]
pub use cap_primitives::fs::FileTypeExt;
#[cfg(unix)]
pub use cap_primitives::fs::{DirBuilderExt, PermissionsExt};
pub use cap_primitives::fs::{FileExt, MetadataExt, OpenOptionsExt};

// Re-export `camino` to make it easy for users to depend on the same
// version we do, because we use its types in our public API.
pub use camino;

use camino::{Utf8Path, Utf8PathBuf};

#[cfg(not(feature = "arf_strings"))]
pub(crate) fn from_utf8<'a>(path: &'a Utf8Path) -> std::io::Result<&'a std::path::Path> {
    Ok(path.as_std_path())
}

#[cfg(feature = "arf_strings")]
pub(crate) fn from_utf8<'a>(path: &'a Utf8Path) -> std::io::Result<std::path::PathBuf> {
    #[cfg(not(windows))]
    let path = {
        #[cfg(unix)]
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        #[cfg(target_os = "wasi")]
        use std::{ffi::OsString, os::wasi::ffi::OsStringExt};

        let string = arf_strings::str_to_host(path.as_str())?;
        OsString::from_vec(string.into_bytes())
    };

    #[cfg(windows)]
    let path = arf_strings::str_to_host(path.as_str())?;

    Ok(path.into())
}

fn to_utf8<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Utf8PathBuf> {
    #[cfg(not(feature = "arf_strings"))]
    #[cfg(not(windows))]
    {
        Ok(Utf8Path::from_path(path.as_ref())
            .ok_or(::rustix::io::Errno::ILSEQ)?
            .to_path_buf())
    }

    #[cfg(not(feature = "arf_strings"))]
    #[cfg(windows)]
    {
        Ok(Utf8Path::from_path(path.as_ref())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "filesystem path is not valid UTF-8",
                )
            })?
            .to_path_buf())
    }

    #[cfg(feature = "arf_strings")]
    {
        // For now, for WASI use the same logic as other OS's, but
        // in the future, the idea is we could avoid this.
        let osstr = path.as_ref().as_os_str();

        #[cfg(not(windows))]
        {
            arf_strings::host_os_str_to_str(osstr)
                .map(std::borrow::Cow::into_owned)
                .map(Into::into)
        }

        #[cfg(windows)]
        {
            arf_strings::host_to_str(osstr).map(Into::into)
        }
    }
}
