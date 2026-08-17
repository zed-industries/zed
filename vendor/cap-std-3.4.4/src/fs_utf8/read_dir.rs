use crate::fs_utf8::DirEntry;
use std::{fmt, io};

/// Iterator over the entries in a directory.
///
/// This corresponds to [`std::fs::ReadDir`].
///
/// There is no `from_std` method, as `std::fs::ReadDir` doesn't provide a way
/// to construct a `ReadDir` without opening directories by ambient paths.
pub struct ReadDir {
    cap_std: crate::fs::ReadDir,
}

impl ReadDir {
    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::ReadDir) -> Self {
        Self { cap_std }
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.cap_std
            .next()
            .map(|result| result.map(DirEntry::from_cap_std))
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
