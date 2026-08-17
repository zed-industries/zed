use crate::io;
use crate::path::{Path, PathBuf};
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Returns the canonical form of a path.
///
/// The returned path is in absolute form with all intermediate components normalized and symbolic
/// links resolved.
///
/// This function is an async version of [`std::fs::canonicalize`].
///
/// [`std::fs::canonicalize`]: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file or directory.
/// * A non-final component in `path` is not a directory.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let path = fs::canonicalize(".").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn canonicalize<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::canonicalize(&path)
            .map(Into::into)
            .context(|| format!("could not canonicalize `{}`", path.display()))
    })
    .await
}
