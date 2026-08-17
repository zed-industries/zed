//! Re-open a `fs::File` to produce an independent handle.

use crate::fs::{is_file_read_write, is_same_file, reopen_impl, OpenOptions};
use std::{fs, io};

/// Re-open an `fs::File` to produce an independent handle.
///
/// This operation isn't supported by all operating systems in all
/// circumstances, or in some operating systems in any circumstances,
/// so it may return an `io::ErrorKind::Other` error if the file
/// cannot be reopened.
#[inline]
pub fn reopen(file: &fs::File, options: &OpenOptions) -> io::Result<fs::File> {
    let (read, write) = is_file_read_write(file)?;

    // Don't grant more rights than the original file had. And don't allow
    // it to create a file.
    if options.create
        || options.create_new
        || (!read && options.read)
        || (!write && (options.write || options.append || options.truncate))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Couldn't reopen file",
        ));
    }

    let new = reopen_impl(file, options)?;

    if !is_same_file(file, &new)? {
        return Err(io::Error::new(io::ErrorKind::Other, "Couldn't reopen file"));
    }

    Ok(new)
}
