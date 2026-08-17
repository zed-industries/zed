use super::super::super::fs::compute_oflags;
use crate::fs::{errors, manually, OpenOptions};
use io_lifetimes::FromFd;
use rustix::fs::{openat, Mode, OFlags, RawMode};
use std::path::Path;
use std::{fs, io};

pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    if !super::beneath_supported() {
        return manually::open(start, path, options);
    }

    let oflags = compute_oflags(options)? | OFlags::RESOLVE_BENEATH;

    let mode = if oflags.contains(OFlags::CREATE) {
        Mode::from_bits((options.ext.mode & 0o7777) as RawMode).unwrap()
    } else {
        Mode::empty()
    };

    match openat(start, path, oflags, mode) {
        Ok(file) => Ok(fs::File::from_into_fd(file)),
        Err(rustix::io::Errno::NOTCAPABLE) => Err(errors::escape_attempt()),
        Err(err) => Err(err.into()),
    }
}
