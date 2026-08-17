use crate::fs::via_parent;
use rustix::fs::{unlinkat, AtFlags};
use std::path::Path;
use std::{fs, io};

pub(crate) fn remove_dir_impl(start: &fs::File, path: &Path) -> io::Result<()> {
    if !super::beneath_supported() {
        return via_parent::remove_dir(start, path);
    }

    Ok(unlinkat(
        start,
        path,
        AtFlags::RESOLVE_BENEATH | AtFlags::REMOVEDIR,
    )?)
}
