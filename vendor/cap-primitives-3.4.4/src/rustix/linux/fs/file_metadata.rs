use crate::fs::{ImplMetadataExt, Metadata};
use rustix::fs::{statat, AtFlags};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::{fs, io};

/// Like `file.metadata()`, but works with `O_PATH` descriptors on old (pre
/// 3.6) versions of Linux too.
pub(super) fn file_metadata(file: &fs::File) -> io::Result<Metadata> {
    // Record whether we've seen an `EBADF` from an `fstat` on an `O_PATH`
    // file descriptor, meaning we're on a Linux that doesn't support it.
    static FSTAT_PATH_BADF: AtomicBool = AtomicBool::new(false);

    if !FSTAT_PATH_BADF.load(Relaxed) {
        match Metadata::from_file(file) {
            Ok(metadata) => return Ok(metadata),
            Err(err) => match rustix::io::Errno::from_io_error(&err) {
                // Before Linux 3.6, `fstat` with `O_PATH` returned `EBADF`.
                Some(rustix::io::Errno::BADF) => FSTAT_PATH_BADF.store(true, Relaxed),
                _ => return Err(err),
            },
        }
    }

    // If `fstat` with `O_PATH` isn't supported, use `statat` with `AT_EMPTY_PATH`.
    Ok(statat(file, "", AtFlags::EMPTY_PATH).map(ImplMetadataExt::from_rustix)?)
}
