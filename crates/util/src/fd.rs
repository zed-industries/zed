use anyhow::{Context as _, Result};
use std::os::fd::RawFd;

/// Marks every currently open file descriptor (except stdin/stdout/stderr)
/// as `FD_CLOEXEC`. Use before spawning a child process to prevent fd leaks
/// from GPU drivers, Wayland compositor, or third-party libraries.
pub fn mark_open_fds_close_on_exec() -> Result<()> {
    let entries = std::fs::read_dir("/proc/self/fd").context("read /proc/self/fd")?;

    for entry in entries {
        let entry = entry?;
        let Some(fd) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<RawFd>().ok())
        else {
            continue;
        };

        if fd <= 2 {
            continue;
        }

        set_fd_close_on_exec(fd)?;
    }

    Ok(())
}

fn set_fd_close_on_exec(fd: RawFd) -> Result<()> {
    // SAFETY: `fd` comes from `/proc/self/fd` and is only used for the duration
    // of these calls. A concurrent close is handled as `EBADF` below.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags == -1 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EBADF) {
            return Ok(());
        }
        return Err(error.into());
    }

    if flags & libc::FD_CLOEXEC != 0 {
        return Ok(());
    }

    // SAFETY: The same descriptor and flags are passed to the libc operation.
    if unsafe { libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC) } == -1 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EBADF) {
            return Ok(());
        }
        return Err(error.into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::os::fd::{AsRawFd, RawFd};

    fn open_without_close_on_exec(path: &std::path::Path) -> anyhow::Result<std::fs::File> {
        use std::ffi::CString;
        use std::os::fd::FromRawFd;
        use std::os::unix::ffi::OsStrExt;

        let path = CString::new(path.as_os_str().as_bytes())?;
        // SAFETY: `path` is a valid NUL-terminated path and no output
        // pointers are passed to `open`.
        let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY) };
        if fd == -1 {
            return Err(std::io::Error::last_os_error().into());
        }

        // SAFETY: `fd` is a valid, uniquely owned descriptor returned by
        // `open`, so ownership can be transferred to `File`.
        Ok(unsafe { std::fs::File::from_raw_fd(fd) })
    }

    fn has_close_on_exec(fd: RawFd) -> anyhow::Result<bool> {
        // SAFETY: The caller keeps the owned file descriptor alive.
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
        if flags == -1 {
            return Err(std::io::Error::last_os_error().into());
        }

        Ok(flags & libc::FD_CLOEXEC != 0)
    }

    #[test]
    fn mark_open_fds_prevents_existing_fd_inheritance() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().join("fd-test-all");
        std::fs::write(&path, b"test")?;
        let file = open_without_close_on_exec(&path)?;
        assert!(!has_close_on_exec(file.as_raw_fd())?);

        super::mark_open_fds_close_on_exec()?;

        assert!(has_close_on_exec(file.as_raw_fd())?);

        Ok(())
    }
}
