use crate::fs::Permissions;
#[cfg(unix)]
use crate::fs::PermissionsExt;
use rustix::fs::{chmodat, AtFlags, Mode};
use std::path::Path;
use std::{fs, io};

/// This can just use `AT_SYMLINK_NOFOLLOW`.
pub(crate) fn set_symlink_permissions_unchecked(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    let mode = Mode::from_bits_truncate(perm.mode().try_into().unwrap());

    Ok(chmodat(start, path, mode, AtFlags::SYMLINK_NOFOLLOW)?)
}
