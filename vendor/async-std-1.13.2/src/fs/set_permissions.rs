use crate::fs::Permissions;
use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;

/// Changes the permissions of a file or directory.
///
/// This function is an async version of [`std::fs::set_permissions`].
///
/// [`std::fs::set_permissions`]: https://doc.rust-lang.org/std/fs/fn.set_permissions.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file or directory.
/// * The current process lacks permissions to change attributes on the file or directory.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let mut perm = fs::metadata("a.txt").await?.permissions();
/// perm.set_readonly(true);
/// fs::set_permissions("a.txt", perm).await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || std::fs::set_permissions(path, perm)).await
}
