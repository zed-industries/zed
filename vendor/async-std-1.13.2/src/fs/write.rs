use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;
use crate::utils::Context as _;

/// Writes a slice of bytes as the new contents of a file.
///
/// This function will create a file if it does not exist, and will entirely replace its contents
/// if it does.
///
/// This function is an async version of [`std::fs::write`].
///
/// [`std::fs::write`]: https://doc.rust-lang.org/std/fs/fn.write.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * The file's parent directory does not exist.
/// * The current process lacks permissions to write to the file.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::write("a.txt", b"Hello world!").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    let contents = contents.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::write(&path, contents)
            .context(|| format!("could not write to file `{}`", path.display()))
    })
    .await
}
