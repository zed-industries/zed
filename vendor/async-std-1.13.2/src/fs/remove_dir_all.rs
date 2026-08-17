use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Removes a directory and all of its contents.
///
/// This function is an async version of [`std::fs::remove_dir_all`].
///
/// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` is not an existing and empty directory.
/// * The current process lacks permissions to remove the directory.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::remove_dir_all("./some/directory").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::remove_dir_all(&path)
            .context(|| format!("could not remove directory `{}`", path.display()))
    })
    .await
}
