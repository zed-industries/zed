use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Creates a new directory.
///
/// Note that this function will only create the final directory in `path`. If you want to create
/// all of its missing parent directories too, use the [`create_dir_all`] function instead.
///
/// This function is an async version of [`std::fs::create_dir`].
///
/// [`create_dir_all`]: fn.create_dir_all.html
/// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` already points to an existing file or directory.
/// * A parent directory in `path` does not exist.
/// * The current process lacks permissions to create the directory.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::create_dir("./some/directory").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::create_dir(&path)
            .context(|| format!("could not create directory `{}`", path.display()))
    })
    .await
}
