use crate::fs::{open_impl, OpenOptions};
use std::ffi::OsStr;
use std::{fs, io};

#[inline(always)]
pub(crate) fn open_entry_impl(
    start: &fs::File,
    path: &OsStr,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    open_impl(start, path.as_ref(), options)
}
