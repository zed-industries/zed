use rustix::fs::renameat;
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `rename`, but which does not perform
/// sandboxing.
pub(crate) fn rename_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    Ok(renameat(old_start, old_path, new_start, new_path)?)
}
