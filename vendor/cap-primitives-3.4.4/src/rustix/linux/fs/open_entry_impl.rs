use crate::fs::{manually, open_beneath, OpenOptions};
use std::ffi::OsStr;
use std::{fs, io};

pub(crate) fn open_entry_impl(
    start: &fs::File,
    path: &OsStr,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let result = open_beneath(start, path.as_ref(), options);

    match result {
        Ok(file) => Ok(file),
        Err(err) => match rustix::io::Errno::from_io_error(&err) {
            Some(rustix::io::Errno::NOSYS) => manually::open_entry(start, path, options),
            _ => Err(err),
        },
    }
}
