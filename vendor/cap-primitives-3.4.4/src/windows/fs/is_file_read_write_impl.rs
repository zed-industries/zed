use io_lifetimes::AsHandle;
use std::{fs, io};

pub(crate) fn is_file_read_write_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    let handle = file.as_handle();
    let access_mode = winx::file::query_access_information(handle)?;
    let read = access_mode.contains(winx::file::AccessMode::FILE_READ_DATA);
    let write = access_mode.contains(winx::file::AccessMode::FILE_WRITE_DATA)
        || access_mode.contains(winx::file::AccessMode::FILE_APPEND_DATA);
    Ok((read, write))
}
