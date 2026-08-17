//! Path canonicalization using `/proc/self/fd`.

use super::procfs::get_path_from_proc_self_fd;
use crate::fs::OpenOptionsExt;
use crate::fs::{manually, open_beneath, FollowSymlinks, OpenOptions};
use rustix::fs::OFlags;
use std::path::{Component, Path, PathBuf};
use std::{fs, io};

/// Implement `canonicalize` by using readlink on `/proc/self/fd/*`.
pub(crate) fn canonicalize_impl(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Open the path with `O_PATH`. Use `read(true)` even though we don't need
    // `read` permissions, because Rust's libstd requires an access mode, and
    // Linux ignores `O_RDONLY` with `O_PATH`.
    let result = open_beneath(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .follow(FollowSymlinks::Yes)
            .custom_flags(OFlags::PATH.bits() as i32),
    );

    // If that worked, call `readlink`.
    match result {
        Ok(file) => {
            if let Ok(start_path) = get_path_from_proc_self_fd(start) {
                if let Ok(file_path) = get_path_from_proc_self_fd(&file) {
                    if let Ok(canonical_path) = file_path.strip_prefix(start_path) {
                        #[cfg(racy_asserts)]
                        if canonical_path.as_os_str().is_empty() {
                            assert_eq!(
                                Component::CurDir.as_os_str(),
                                manually::canonicalize(start, path).unwrap()
                            );
                        } else {
                            assert_eq!(
                                canonical_path,
                                manually::canonicalize(start, path).unwrap()
                            );
                        }

                        let mut path_buf = canonical_path.to_path_buf();

                        // Replace "" with ".", since "" as a relative path is
                        // interpreted as an error.
                        if path_buf.as_os_str().is_empty() {
                            path_buf.push(Component::CurDir);
                        }

                        return Ok(path_buf);
                    }
                }
            }
        }
        Err(err) => match rustix::io::Errno::from_io_error(&err) {
            // `ENOSYS` from `open_beneath` means `openat2` is unavailable
            // and we should use a fallback.
            Some(rustix::io::Errno::NOSYS) => (),
            _ => return Err(err),
        },
    }

    // Use a fallback.
    manually::canonicalize(start, path)
}
