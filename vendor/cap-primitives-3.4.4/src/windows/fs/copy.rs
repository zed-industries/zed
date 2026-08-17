use super::get_path::get_path;
use crate::fs::{open, OpenOptions};
use std::path::Path;
use std::{fs, io};

pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    let from = open(from_start, from_path, OpenOptions::new().read(true))?;
    let to = open(
        to_start,
        to_path,
        OpenOptions::new().create(true).truncate(true).write(true),
    )?;
    fs::copy(get_path(&from)?, get_path(&to)?)
}
