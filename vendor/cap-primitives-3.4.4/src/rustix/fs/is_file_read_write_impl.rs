use rustix::fd::{AsFd, BorrowedFd};
use rustix::fs::{fcntl_getfl, OFlags};
use std::{fs, io};

#[inline]
pub(crate) fn is_file_read_write_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    Ok(is_file_read_write(file)?)
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`
///
/// Returns a pair of booleans indicating whether the file descriptor is
/// readable and/or writable, respectively. This is only reliable on files; for
/// example, it doesn't reflect whether sockets have been shut down; for
/// general I/O handle support, use [`io::is_read_write`].
#[inline]
fn is_file_read_write<Fd: AsFd>(fd: Fd) -> io::Result<(bool, bool)> {
    _is_file_read_write(fd.as_fd())
}

fn _is_file_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    let mode = fcntl_getfl(fd)?;

    // Check for `O_PATH`.
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "emscripten",
        target_os = "fuchsia"
    ))]
    if mode.contains(OFlags::PATH) {
        return Ok((false, false));
    }

    // Use `RWMODE` rather than `ACCMODE` as `ACCMODE` may include `O_PATH`.
    // We handled `O_PATH` above.
    match mode & OFlags::RWMODE {
        OFlags::RDONLY => Ok((true, false)),
        OFlags::RDWR => Ok((true, true)),
        OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}
