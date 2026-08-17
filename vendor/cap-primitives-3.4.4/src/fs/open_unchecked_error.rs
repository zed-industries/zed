use std::io;

#[derive(Debug)]
pub(crate) enum OpenUncheckedError {
    Other(io::Error),
    Symlink(io::Error, SymlinkKind),
    NotFound(io::Error),
}

#[cfg(not(windows))]
pub(crate) type SymlinkKind = ();

#[cfg(windows)]
#[derive(Debug)]
pub(crate) enum SymlinkKind {
    File,
    Dir,
}

impl OpenUncheckedError {
    #[allow(dead_code)]
    pub(crate) fn kind(&self) -> io::ErrorKind {
        match self {
            Self::Other(err) | Self::Symlink(err, _) | Self::NotFound(err) => err.kind(),
        }
    }
}

impl From<OpenUncheckedError> for io::Error {
    fn from(error: OpenUncheckedError) -> Self {
        match error {
            OpenUncheckedError::Other(err)
            | OpenUncheckedError::Symlink(err, _)
            | OpenUncheckedError::NotFound(err) => err,
        }
    }
}
