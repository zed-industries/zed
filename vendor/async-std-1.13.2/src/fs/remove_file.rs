use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Removes a file.
///
/// This function is an async version of [`std::fs::remove_file`].
///
/// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file.
/// * The current process lacks permissions to remove the file.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::remove_file("a.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::remove_file(&path)
            .context(|| format!("could not remove file `{}`", path.display()))
    })
    .await
}
