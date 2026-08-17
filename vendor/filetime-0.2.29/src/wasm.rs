use crate::FileTime;
use std::fs;
use std::io;
use std::path::Path;

pub fn set_symlink_file_times(_p: &Path, _atime: FileTime, _mtime: FileTime) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, "Wasm not implemented"))
}

pub fn from_last_modification_time(_meta: &fs::Metadata) -> FileTime {
    unimplemented!()
}

pub fn from_last_access_time(_meta: &fs::Metadata) -> FileTime {
    unimplemented!()
}

pub fn from_creation_time(_meta: &fs::Metadata) -> Option<FileTime> {
    unimplemented!()
}

pub fn open(path: &Path) -> io::Result<fs::File> {
    fs::File::open(path)
}
