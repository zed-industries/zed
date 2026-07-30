use anyhow::Result;

#[cfg(target_os = "linux")]
use std::os::fd::RawFd;

/// Wraps an operation. Any file descriptors opened or transferred during it
/// are marked `FD_CLOEXEC` so they are not inherited by child processes
#[cfg(target_os = "linux")]
pub fn with_new_fds_close_on_exec<T>(operation: impl FnOnce() -> T) -> T {
    let before = open_fds_snapshot();
    let output = operation();

    if let Ok(before) = before {
        if let Err(error) = mark_fds_changed_since(&before) {
            log::debug!("failed to set new fds close-on-exec: {error}");
        }
    }

    output
}

#[cfg(not(target_os = "linux"))]
pub fn with_new_fds_close_on_exec<T>(operation: impl FnOnce() -> T) -> T {
    operation()
}

/// Marks every currently open file descriptor (except stdin/stdout/stderr)
/// as `FD_CLOEXEC`. Use before spawning a child process to prevent fd leaks
/// from GPU drivers, Wayland compositor, or third-party libraries.
#[cfg(target_os = "linux")]
pub fn mark_open_fds_close_on_exec() -> Result<usize> {
    let Ok(entries) = std::fs::read_dir("/proc/self/fd") else {
        return Ok(0);
    };

    let mut updated = 0;
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

        if set_fd_close_on_exec(fd)? {
            updated += 1;
        }
    }

    Ok(updated)
}

#[cfg(not(target_os = "linux"))]
pub fn mark_open_fds_close_on_exec() -> Result<usize> {
    Ok(0)
}

#[cfg(target_os = "linux")]
fn set_fd_close_on_exec(fd: RawFd) -> Result<bool> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags == -1 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EBADF) {
            return Ok(false);
        }
        return Err(error.into());
    }

    if flags & libc::FD_CLOEXEC != 0 {
        return Ok(false);
    }

    if unsafe { libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC) } == -1 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EBADF) {
            return Ok(false);
        }
        return Err(error.into());
    }

    Ok(true)
}

#[cfg(target_os = "linux")]
fn open_fds_snapshot() -> Result<std::collections::HashMap<RawFd, std::path::PathBuf>> {
    let Ok(entries) = std::fs::read_dir("/proc/self/fd") else {
        return Ok(std::collections::HashMap::default());
    };

    let mut snapshot = std::collections::HashMap::default();
    for entry in entries {
        let entry = entry?;
        let Some(fd) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<RawFd>().ok())
        else {
            continue;
        };
        let Ok(target) = std::fs::read_link(entry.path()) else {
            continue;
        };

        snapshot.insert(fd, target);
    }

    Ok(snapshot)
}

#[cfg(target_os = "linux")]
fn mark_fds_changed_since(
    before: &std::collections::HashMap<RawFd, std::path::PathBuf>,
) -> Result<usize> {
    let after = open_fds_snapshot()?;
    let mut updated = 0;

    for (fd, target) in after {
        if before.get(&fd) == Some(&target) {
            continue;
        }

        if set_fd_close_on_exec(fd)? {
            updated += 1;
        }
    }

    Ok(updated)
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    #[test]
    fn with_new_fds_marks_new_fd() {
        use std::ffi::CString;
        use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
        use std::os::unix::ffi::OsStrExt;

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("fd-test");
        std::fs::write(&path, b"test").unwrap();
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();

        let file = super::with_new_fds_close_on_exec(|| {
            let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY) };
            assert_ne!(fd, -1, "failed to open test file");
            unsafe { OwnedFd::from_raw_fd(fd) }
        });

        let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFD) };
        assert_ne!(flags, -1, "failed to get fd flags");
        assert_ne!(flags & libc::FD_CLOEXEC, 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn mark_open_fds_covers_existing_fd() {
        use std::ffi::CString;
        use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
        use std::os::unix::ffi::OsStrExt;

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("fd-test-all");
        std::fs::write(&path, b"test").unwrap();
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();

        let file = unsafe {
            let fd = libc::open(path.as_ptr(), libc::O_RDONLY);
            assert_ne!(fd, -1, "failed to open test file");
            OwnedFd::from_raw_fd(fd)
        };

        let before = super::mark_open_fds_close_on_exec().unwrap();
        assert!(before >= 1);

        let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFD) };
        assert_ne!(flags, -1, "failed to get fd flags");
        assert_ne!(flags & libc::FD_CLOEXEC, 0);
    }
}
