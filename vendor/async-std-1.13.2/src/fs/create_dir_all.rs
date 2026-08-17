use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Creates a new directory and all of its parents if they are missing.
///
/// This function is an async version of [`std::fs::create_dir_all`].
///
/// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` already points to an existing file or directory.
/// * The current process lacks permissions to create the directory or its missing parents.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::create_dir_all("./some/directory").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::create_dir_all(&path)
            .context(|| format!("could not create directory path `{}`", path.display()))
    })
    .await
}
