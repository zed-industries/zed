use crate::io;
use crate::path::{Path, PathBuf};
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Reads a symbolic link and returns the path it points to.
///
/// This function is an async version of [`std::fs::read_link`].
///
/// [`std::fs::read_link`]: https://doc.rust-lang.org/std/fs/fn.read_link.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing link.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let path = fs::read_link("a.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::read_link(&path)
            .map(Into::into)
            .context(|| format!("could not read link `{}`", path.display()))
    })
    .await
}
