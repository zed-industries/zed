use std::fs::File;
use std::io::{BufReader, Error as IOError, ErrorKind, Read};
use std::path::PathBuf;

pub(crate) trait Source {
    type Item;
    type Error;

    fn load(&self) -> Result<Self::Item, Self::Error>;
}

pub(crate) struct FileSource {
    path: PathBuf,
}

impl FileSource {
    pub(crate) fn new(path: PathBuf) -> FileSource {
        FileSource { path }
    }
}

impl Source for FileSource {
    type Item = String;
    type Error = IOError;

    fn load(&self) -> Result<Self::Item, Self::Error> {
        let mut reader = BufReader::new(File::open(&self.path)?);

        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        Ok(buf)
    }
}

pub(crate) struct LazySource<F: Fn() -> Option<String>> {
    loader: F,
}

impl<F: Fn() -> Option<String>> LazySource<F> {
    pub(crate) fn new(loader: F) -> LazySource<F> {
        LazySource { loader }
    }
}

impl<F: Fn() -> Option<String>> Source for LazySource<F> {
    type Item = String;
    type Error = IOError;

    fn load(&self) -> Result<Self::Item, Self::Error> {
        (self.loader)().ok_or(IOError::new(ErrorKind::Other, "Source load error"))
    }
}
