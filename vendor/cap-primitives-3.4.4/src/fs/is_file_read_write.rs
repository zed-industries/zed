use crate::fs::is_file_read_write_impl;
use std::{fs, io};

/// Return a pair of booleans indicating whether the given file is opened
/// for reading and writing, respectively.
#[inline]
pub fn is_file_read_write(file: &fs::File) -> io::Result<(bool, bool)> {
    is_file_read_write_impl(file)
}
