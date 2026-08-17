use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Reads the entire contents of a file as raw bytes.
///
/// This is a convenience function for reading entire files. It pre-allocates a buffer based on the
/// file size when available, so it is typically faster than manually opening a file and reading
/// from it.
///
/// If you want to read the contents as a string, use [`read_to_string`] instead.
///
/// This function is an async version of [`std::fs::read`].
///
/// [`read_to_string`]: fn.read_to_string.html
/// [`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file.
/// * The current process lacks permissions to read the file.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let contents = fs::read("a.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::read(&path).context(|| format!("could not read file `{}`", path.display()))
    })
    .await
}
