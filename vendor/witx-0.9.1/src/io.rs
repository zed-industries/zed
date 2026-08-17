use crate::WitxError;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

pub trait WitxIo {
    /// Read the entire file into a String. Used to resolve `use` declarations.
    fn fgets(&self, path: &Path) -> Result<String, WitxError>;
    /// Read a line of a file into a String. Used for error reporting.
    fn fget_line(&self, path: &Path, line_num: usize) -> Result<String, WitxError>;
    /// Return the canonical (non-symlinked) path of a file. Used to resolve `use` declarations.
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, WitxError>;
}

impl<T: WitxIo + ?Sized> WitxIo for &'_ T {
    fn fgets(&self, path: &Path) -> Result<String, WitxError> {
        T::fgets(self, path)
    }
    fn fget_line(&self, path: &Path, line_num: usize) -> Result<String, WitxError> {
        T::fget_line(self, path, line_num)
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, WitxError> {
        T::canonicalize(self, path)
    }
}

pub struct Filesystem;

impl WitxIo for Filesystem {
    fn fgets(&self, path: &Path) -> Result<String, WitxError> {
        read_to_string(path).map_err(|e| WitxError::Io(path.to_path_buf(), e))
    }
    fn fget_line(&self, path: &Path, line_num: usize) -> Result<String, WitxError> {
        let f = File::open(path).map_err(|e| WitxError::Io(path.into(), e))?;
        let buf = BufReader::new(f);
        let l = buf
            .lines()
            .skip(line_num - 1)
            .next()
            .ok_or_else(|| {
                WitxError::Io(path.into(), Error::new(ErrorKind::Other, "Line not found"))
            })?
            .map_err(|e| WitxError::Io(path.into(), e))?;

        Ok(l)
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, WitxError> {
        path.canonicalize()
            .map_err(|e| WitxError::Io(path.to_path_buf(), e))
    }
}

pub struct MockFs {
    map: HashMap<PathBuf, String>,
}

impl MockFs {
    pub fn new(strings: &[(&str, &str)]) -> Self {
        MockFs {
            map: strings
                .iter()
                .map(|(k, v)| (PathBuf::from(k), v.to_string()))
                .collect(),
        }
    }
}

impl WitxIo for MockFs {
    fn fgets(&self, path: &Path) -> Result<String, WitxError> {
        if let Some(entry) = self.map.get(path) {
            Ok(entry.to_string())
        } else {
            Err(WitxError::Io(
                path.to_path_buf(),
                Error::new(ErrorKind::Other, "mock fs: file not found"),
            ))
        }
    }
    fn fget_line(&self, path: &Path, line: usize) -> Result<String, WitxError> {
        if let Some(entry) = self.map.get(path) {
            entry
                .lines()
                .skip(line - 1)
                .next()
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    WitxError::Io(
                        path.to_path_buf(),
                        Error::new(ErrorKind::Other, "mock fs: file not found"),
                    )
                })
        } else {
            Err(WitxError::Io(
                path.to_path_buf(),
                Error::new(ErrorKind::Other, "mock fs: file not found"),
            ))
        }
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, WitxError> {
        Ok(PathBuf::from(path))
    }
}
