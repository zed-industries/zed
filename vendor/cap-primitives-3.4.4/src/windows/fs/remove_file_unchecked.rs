use super::get_path::concatenate;
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `remove_file`, but which does not perform
/// sandboxing.
pub(crate) fn remove_file_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    let full_path = concatenate(start, path)?;
    fs::remove_file(full_path)
}
