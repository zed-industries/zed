//! This defines `open_ambient`, for unsandboxed file opening.

use crate::fs::{open_ambient_impl, OpenOptions};
use crate::AmbientAuthority;
use std::path::Path;
use std::{fs, io};

/// Open a file named by a bare path, using the host process' ambient
/// authority.
///
/// # Ambient Authority
///
/// This function is not sandboxed and may trivially access any path that the
/// host process has access to.
#[inline]
pub fn open_ambient(
    path: &Path,
    options: &OpenOptions,
    ambient_authority: AmbientAuthority,
) -> io::Result<fs::File> {
    Ok(open_ambient_impl(path, options, ambient_authority)?)
}
