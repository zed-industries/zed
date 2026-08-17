use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Copies the contents and permissions of a file to a new location.
///
/// On success, the total number of bytes copied is returned and equals the length of the `to` file
/// after this operation.
///
/// The old contents of `to` will be overwritten. If `from` and `to` both point to the same file,
/// then the file will likely get truncated as a result of this operation.
///
/// If you're working with open [`File`]s and want to copy contents through those types, use the
/// [`io::copy`] function.
///
/// This function is an async version of [`std::fs::copy`].
///
/// [`File`]: struct.File.html
/// [`io::copy`]: ../io/fn.copy.html
/// [`std::fs::copy`]: https://doc.rust-lang.org/std/fs/fn.copy.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `from` does not point to an existing file.
/// * The current process lacks permissions to read `from` or write `to`.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let num_bytes = fs::copy("a.txt", "b.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    let from = from.as_ref().to_owned();
    let to = to.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::copy(&from, &to)
            .context(|| format!("could not copy `{}` to `{}`", from.display(), to.display()))
    })
    .await
}
