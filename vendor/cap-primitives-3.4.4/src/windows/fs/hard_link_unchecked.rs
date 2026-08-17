use super::get_path::concatenate;
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `hard_link`, but which does not perform
/// sandboxing.
pub(crate) fn hard_link_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let old_full_path = concatenate(old_start, old_path)?;
    let new_full_path = concatenate(new_start, new_path)?;
    fs::hard_link(old_full_path, new_full_path)
}
