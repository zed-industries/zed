use super::procfs::set_permissions_through_proc_self_fd;
use crate::fs::{open, OpenOptions, Permissions};
use rustix::fs::{fchmod, Mode, RawMode};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

pub(crate) fn set_permissions_impl(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    let std_perm = perm.into_std(start)?;

    // First try using `O_PATH` and doing a chmod on `/proc/self/fd/{}`
    // (`fchmod` doesn't work on `O_PATH` file descriptors).
    //
    // This may fail, due to older Linux versions not supporting `O_PATH`, or
    // due to procfs being unavailable, but if it does work, go with it, as
    // `O_PATH` tells Linux that we don't actually need to read or write the
    // file, which may avoid side effects associated with opening device files.
    let result = set_permissions_through_proc_self_fd(start, path, std_perm.clone());
    if let Ok(()) = result {
        return Ok(());
    }

    // Then try `fchmod` with a normal handle. Normal handles need some kind of
    // access, so first try read.
    match open(start, path, OpenOptions::new().read(true)) {
        Ok(file) => return set_file_permissions(&file, std_perm),
        Err(err) => match rustix::io::Errno::from_io_error(&err) {
            Some(rustix::io::Errno::ACCESS) => (),
            _ => return Err(err),
        },
    }

    // Next try write.
    match open(start, path, OpenOptions::new().write(true)) {
        Ok(file) => return set_file_permissions(&file, std_perm),
        Err(err) => match rustix::io::Errno::from_io_error(&err) {
            Some(rustix::io::Errno::ACCESS) | Some(rustix::io::Errno::ISDIR) => (),
            _ => return Err(err),
        },
    }

    // Nothing worked, so just return the original error.
    result
}

fn set_file_permissions(file: &fs::File, perm: fs::Permissions) -> io::Result<()> {
    // Use `from_bits_truncate` for compatibility with std, which allows
    // non-permission bits to propagate through.
    let mode = Mode::from_bits_truncate(perm.mode() as RawMode);
    Ok(fchmod(file, mode)?)
}
