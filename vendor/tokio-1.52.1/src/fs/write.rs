use crate::{fs::asyncify, util::as_ref::OwnedBuf};

use std::{io, path::Path};

/// Creates a future that will open a file for writing and write the entire
/// contents of `contents` to it.
///
/// This is the async equivalent of [`std::fs::write`][std].
///
/// This operation is implemented by running the equivalent blocking operation
/// on a separate thread pool using [`spawn_blocking`].
///
/// [`spawn_blocking`]: crate::task::spawn_blocking
/// [std]: fn@std::fs::write
///
/// # Examples
///
/// ```no_run
/// use tokio::fs;
///
/// # async fn dox() -> std::io::Result<()> {
/// fs::write("foo.txt", b"Hello world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let path = path.as_ref();
    let contents = crate::util::as_ref::upgrade(contents);

    #[cfg(all(
        tokio_unstable,
        feature = "io-uring",
        feature = "rt",
        feature = "fs",
        target_os = "linux"
    ))]
    {
        let handle = crate::runtime::Handle::current();
        let driver_handle = handle.inner.driver().io();
        if driver_handle
            .check_and_init(io_uring::opcode::Write::CODE)
            .await?
        {
            return write_uring(path, contents).await;
        }
    }

    write_spawn_blocking(path, contents).await
}

#[cfg(all(
    tokio_unstable,
    feature = "io-uring",
    feature = "rt",
    feature = "fs",
    target_os = "linux"
))]
async fn write_uring(path: &Path, mut buf: OwnedBuf) -> io::Result<()> {
    use crate::{fs::OpenOptions, runtime::driver::op::Op};
    use std::os::fd::OwnedFd;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .await?;

    let mut fd: OwnedFd = file
        .try_into_std()
        .expect("unexpected in-flight operation detected")
        .into();

    let total: usize = buf.as_ref().len();
    let mut buf_offset: usize = 0;
    let mut file_offset: u64 = 0;
    while buf_offset < total {
        let (res, _buf, _fd) = Op::write_at(fd, buf, buf_offset, file_offset)?.await;

        let n = match res {
            Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => 0,
            Err(e) => return Err(e),
        };

        buf = _buf;
        fd = _fd;
        buf_offset += n as usize;
        file_offset += n as u64;
    }

    Ok(())
}

async fn write_spawn_blocking(path: &Path, contents: OwnedBuf) -> io::Result<()> {
    let path = path.to_owned();
    asyncify(move || std::fs::write(path, contents)).await
}
