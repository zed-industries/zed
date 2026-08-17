use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Reads the entire contents of a file as a string.
///
/// This is a convenience function for reading entire files. It pre-allocates a string based on the
/// file size when available, so it is typically faster than manually opening a file and reading
/// from it.
///
/// If you want to read the contents as raw bytes, use [`read`] instead.
///
/// This function is an async version of [`std::fs::read_to_string`].
///
/// [`read`]: fn.read.html
/// [`std::fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file.
/// * The current process lacks permissions to read the file.
/// * The contents of the file cannot be read as a UTF-8 string.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let contents = fs::read_to_string("a.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::read_to_string(&path)
            .context(|| format!("could not read file `{}`", path.display()))
    })
    .await
}
