use super::get_path::concatenate;
use crate::fs::Permissions;
use std::path::Path;
use std::{fs, io};

/// This can just use `AT_SYMLINK_NOFOLLOW`.
pub(crate) fn set_symlink_permissions_unchecked(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    // According to [Rust's documentation], `fs::set_permissions` uses
    // `SetFileAttributes`, and according to [Windows' documentation]
    // `SetFileAttributes` does not follow symbolic links.
    //
    // [Windows' documentation]: https://docs.microsoft.com/en-us/windows/win32/fileio/symbolic-link-effects-on-file-systems-functions#setfileattributes
    // [Rust's documentation]: https://doc.rust-lang.org/std/fs/fn.set_permissions.html#platform-specific-behavior
    let out_path = concatenate(start, path)?;
    fs::set_permissions(out_path, perm.into_std(start)?)
}
