use std::io;

#[cfg(not(windows))]
pub(crate) use crate::rustix::fs::errors::*;
#[cfg(windows)]
pub(crate) use crate::windows::fs::errors::*;

#[cold]
pub(crate) fn escape_attempt() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    )
}
