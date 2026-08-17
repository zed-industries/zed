use std::borrow::Cow;
use std::io;
use std::path::Path;

use crate::BasePathBuf;

#[cfg(feature = "localization")]
pub(super) mod localize;

#[inline(always)]
pub(crate) fn is_base(_: &Path) -> bool {
    true
}

#[inline(always)]
pub(crate) fn to_base(_: &Path) -> io::Result<BasePathBuf> {
    unreachable!();
}

pub(crate) fn normalize(path: &Path) -> io::Result<BasePathBuf> {
    // This method rejects null bytes and empty paths, which is consistent with
    // [GetFullPathNameW] on Windows.
    path.canonicalize().and_then(BasePathBuf::new)
}

pub(crate) fn expand(path: &Path) -> io::Result<Cow<'_, Path>> {
    path.metadata().map(|_| Cow::Borrowed(path))
}

pub(crate) fn shorten(path: &Path) -> io::Result<Cow<'_, Path>> {
    expand(path)
}

pub(crate) fn push(base: &mut BasePathBuf, path: &Path) {
    if !path.as_os_str().is_empty() {
        base.0.push(path);
    }
}
