use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Renames a file or directory to a new location.
///
/// If a file or directory already exists at the target location, it will be overwritten by this
/// operation.
///
/// This function is an async version of [`std::fs::rename`].
///
/// [`std::fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `from` does not point to an existing file or directory.
/// * `from` and `to` are on different filesystems.
/// * The current process lacks permissions to do the rename operation.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::rename("a.txt", "b.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    let from = from.as_ref().to_owned();
    let to = to.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::rename(&from, &to).context(|| {
            format!(
                "could not rename `{}` to `{}`",
                from.display(),
                to.display()
            )
        })
    })
    .await
}
