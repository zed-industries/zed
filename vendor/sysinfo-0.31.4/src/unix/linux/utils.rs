// Take a look at the license at the top of the repository in the LICENSE file.

use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::Path;

#[cfg(feature = "system")]
pub(crate) fn get_all_data_from_file(file: &mut File, size: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(size);
    file.rewind()?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub(crate) fn get_all_utf8_data_from_file(file: &mut File, size: usize) -> io::Result<String> {
    let mut buf = String::with_capacity(size);
    file.rewind()?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub(crate) fn get_all_utf8_data<P: AsRef<Path>>(file_path: P, size: usize) -> io::Result<String> {
    let mut file = File::open(file_path.as_ref())?;
    get_all_utf8_data_from_file(&mut file, size)
}

#[cfg(feature = "system")]
#[allow(clippy::useless_conversion)]
pub(crate) fn realpath(path: &Path) -> Option<std::path::PathBuf> {
    match std::fs::read_link(path) {
        Ok(path) => Some(path),
        Err(_e) => {
            sysinfo_debug!("failed to get real path for {:?}: {:?}", path, _e);
            None
        }
    }
}

/// This type is used in `retrieve_all_new_process_info` because we have a "parent" path and
/// from it, we `pop`/`join` every time because it's more memory efficient than using `Path::join`.
#[cfg(feature = "system")]
pub(crate) struct PathHandler(std::path::PathBuf);

#[cfg(feature = "system")]
impl PathHandler {
    pub(crate) fn new(path: &Path) -> Self {
        // `path` is the "parent" for all paths which will follow so we add a fake element at
        // the end since every `PathHandler::join` call will first call `pop` internally.
        Self(path.join("a"))
    }
}

#[cfg(feature = "system")]
pub(crate) trait PathPush {
    fn join(&mut self, p: &str) -> &Path;
}

#[cfg(feature = "system")]
impl PathPush for PathHandler {
    fn join(&mut self, p: &str) -> &Path {
        self.0.pop();
        self.0.push(p);
        self.0.as_path()
    }
}

// This implementation allows to skip one allocation that is done in `PathHandler`.
#[cfg(feature = "system")]
impl PathPush for std::path::PathBuf {
    fn join(&mut self, p: &str) -> &Path {
        self.push(p);
        self.as_path()
    }
}

#[cfg(feature = "system")]
pub(crate) fn to_u64(v: &[u8]) -> u64 {
    let mut x = 0;

    for c in v {
        x *= 10;
        x += u64::from(c - b'0');
    }
    x
}

/// Converts a path to a NUL-terminated `Vec<u8>` suitable for use with C functions.
pub(crate) fn to_cpath(path: &std::path::Path) -> Vec<u8> {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

    let path_os: &OsStr = path.as_ref();
    let mut cpath = path_os.as_bytes().to_vec();
    cpath.push(0);
    cpath
}
