use crate::fs::{to_timespec, via_parent, SystemTimeSpec};
use rustix::fs::{utimensat, AtFlags, Timestamps};
use std::path::Path;
use std::{fs, io};

pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    if !super::beneath_supported() {
        return super::super::super::fs::set_times_manually(start, path, atime, mtime);
    }

    let times = Timestamps {
        last_access: to_timespec(atime)?,
        last_modification: to_timespec(mtime)?,
    };

    Ok(utimensat(start, path, &times, AtFlags::RESOLVE_BENEATH)?)
}

pub(crate) fn set_times_nofollow_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    if !super::beneath_supported() {
        return via_parent::set_times_nofollow(start, path, atime, mtime);
    }

    let times = Timestamps {
        last_access: to_timespec(atime)?,
        last_modification: to_timespec(mtime)?,
    };

    Ok(utimensat(
        start,
        path,
        &times,
        AtFlags::RESOLVE_BENEATH | AtFlags::SYMLINK_NOFOLLOW,
    )?)
}
