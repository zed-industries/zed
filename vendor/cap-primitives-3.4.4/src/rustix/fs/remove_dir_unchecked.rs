use rustix::fs::{unlinkat, AtFlags};
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `remove_dir`, but which does not perform
/// sandboxing.
pub(crate) fn remove_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    Ok(unlinkat(start, path, AtFlags::REMOVEDIR)?)
}
