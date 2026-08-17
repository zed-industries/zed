use crate::fs::{open_unchecked, OpenOptions};
use crate::rustix::fs::file_path;
use io_lifetimes::AsFilelike;
use rustix::fs::CWD;
use std::{fs, io};

/// Implementation of `reopen`.
pub(crate) fn reopen_impl(file: &fs::File, options: &OpenOptions) -> io::Result<fs::File> {
    if let Some(path) = file_path(file) {
        Ok(open_unchecked(
            &CWD.as_filelike_view::<fs::File>(),
            &path,
            options,
        )?)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Couldn't reopen file"))
    }
}
