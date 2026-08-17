use crate::fs::{Metadata, ReadDir};
use std::{fs, io};

pub(crate) fn is_root_dir(dir: &fs::File, parent_iter: &ReadDir) -> io::Result<bool> {
    Ok(Metadata::from_file(dir)?.is_same_file(&parent_iter.inner.self_metadata()?))
}
