use std::future::Future;
use std::pin::Pin;

use crate::fs::DirEntry;
use crate::io;
use crate::path::Path;
use crate::stream::Stream;
use crate::task::{spawn_blocking, Context, JoinHandle, Poll};
use crate::utils::Context as _;

/// Returns a stream of entries in a directory.
///
/// The stream yields items of type [`io::Result`]`<`[`DirEntry`]`>`. Note that I/O errors can
/// occur while reading from the stream.
///
/// This function is an async version of [`std::fs::read_dir`].
///
/// [`io::Result`]: ../io/type.Result.html
/// [`DirEntry`]: struct.DirEntry.html
/// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing directory.
/// * The current process lacks permissions to read the contents of the directory.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
/// use async_std::prelude::*;
///
/// let mut entries = fs::read_dir(".").await?;
///
/// while let Some(res) = entries.next().await {
///     let entry = res?;
///     println!("{}", entry.file_name().to_string_lossy());
/// }
/// #
/// # Ok(()) }) }
/// ```
pub async fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || {
        std::fs::read_dir(&path)
            .context(|| format!("could not read directory `{}`", path.display()))
    })
    .await
    .map(ReadDir::new)
}

/// A stream of entries in a directory.
///
/// This stream is returned by [`read_dir`] and yields items of type
/// [`io::Result`]`<`[`DirEntry`]`>`. Each [`DirEntry`] can then retrieve information like entry's
/// path or metadata.
///
/// This type is an async version of [`std::fs::ReadDir`].
///
/// [`read_dir`]: fn.read_dir.html
/// [`io::Result`]: ../io/type.Result.html
/// [`DirEntry`]: struct.DirEntry.html
/// [`std::fs::ReadDir`]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
#[derive(Debug)]
pub struct ReadDir(State);

/// The state of an asynchronous `ReadDir`.
///
/// The `ReadDir` can be either idle or busy performing an asynchronous operation.
#[derive(Debug)]
enum State {
    Idle(Option<std::fs::ReadDir>),
    Busy(JoinHandle<(std::fs::ReadDir, Option<io::Result<std::fs::DirEntry>>)>),
}

impl ReadDir {
    /// Creates an asynchronous `ReadDir` from a synchronous handle.
    pub(crate) fn new(inner: std::fs::ReadDir) -> ReadDir {
        ReadDir(State::Idle(Some(inner)))
    }
}

impl Stream for ReadDir {
    type Item = io::Result<DirEntry>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match &mut self.0 {
                State::Idle(opt) => {
                    let mut inner = opt.take().unwrap();

                    // Start the operation asynchronously.
                    self.0 = State::Busy(spawn_blocking(move || {
                        let next = inner.next();
                        (inner, next)
                    }));
                }
                // Poll the asynchronous operation the file is currently blocked on.
                State::Busy(task) => {
                    let (inner, opt) = futures_core::ready!(Pin::new(task).poll(cx));
                    self.0 = State::Idle(Some(inner));
                    return Poll::Ready(opt.map(|res| res.map(DirEntry::new)));
                }
            }
        }
    }
}
