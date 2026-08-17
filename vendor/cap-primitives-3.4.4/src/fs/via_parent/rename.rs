use super::open_parent;
#[cfg(unix)]
use crate::fs::{append_dir_suffix, path_has_trailing_slash};
use crate::fs::{rename_unchecked, strip_dir_suffix, MaybeOwnedFile};
use std::path::Path;
use std::{fs, io};

/// Implement `rename` by `open`ing up the parent component of the path and
/// then calling `rename_unchecked` on the last component.
pub(crate) fn rename(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let old_start = MaybeOwnedFile::borrowed(old_start);
    let new_start = MaybeOwnedFile::borrowed(new_start);

    // As a special case, `rename` ignores a trailing slash rather than treating
    // it as equivalent to a trailing slash-dot, so strip any trailing slashes
    // for the purposes of `open_parent`.
    //
    // And on Unix, remember whether the source started with a slash so that we
    // can still fail if it is and the source is a regular file.
    #[cfg(unix)]
    let old_starts_with_slash = path_has_trailing_slash(old_path);
    let old_path = strip_dir_suffix(old_path);
    let new_path = strip_dir_suffix(new_path);

    let (old_dir, old_basename) = open_parent(old_start, &old_path)?;
    let (new_dir, new_basename) = open_parent(new_start, &new_path)?;

    // On Unix, re-append a slash if needed.
    #[cfg(unix)]
    let concat;
    #[cfg(unix)]
    let old_basename = if old_starts_with_slash {
        concat = append_dir_suffix(old_basename.to_owned().into());
        concat.as_os_str()
    } else {
        old_basename
    };

    rename_unchecked(
        &old_dir,
        old_basename.as_ref(),
        &new_dir,
        new_basename.as_ref(),
    )
}
