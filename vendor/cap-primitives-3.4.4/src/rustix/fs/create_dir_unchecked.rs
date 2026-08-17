use crate::fs::DirOptions;
use rustix::fs::{mkdirat, Mode, RawMode};
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `create_dir`, but which does not perform
/// sandboxing.
pub(crate) fn create_dir_unchecked(
    start: &fs::File,
    path: &Path,
    options: &DirOptions,
) -> io::Result<()> {
    #[cfg(not(target_os = "wasi"))]
    let raw_mode = options.ext.mode as RawMode;
    #[cfg(target_os = "wasi")]
    let raw_mode = 0;

    Ok(mkdirat(start, path, Mode::from_bits(raw_mode).unwrap())?)
}
