use std::borrow::Cow;

#[cfg(feature = "runtime-async-std")]
use async_std::fs::Metadata;
#[cfg(feature = "runtime-async-std")]
use async_std::{
    fs,
    io::{self, Read, Write},
    path::Path,
    prelude::*,
};
#[cfg(feature = "runtime-tokio")]
use std::fs::Metadata;
#[cfg(feature = "runtime-tokio")]
use std::path::Path;
#[cfg(feature = "runtime-tokio")]
use tokio::{
    fs,
    io::{self, AsyncRead as Read, AsyncReadExt, AsyncWrite as Write, AsyncWriteExt},
};
#[cfg(feature = "runtime-tokio")]
use tokio_stream::StreamExt;

use crate::{
    EntryType, Header,
    header::{HeaderMode, bytes2path, path2bytes},
    metadata, other, symlink_metadata,
};

/// A structure for building archives
///
/// This structure has methods for building up an archive from scratch into any
/// arbitrary writer.
///
/// You **must** call [`finish`] or [`into_inner`] to finalize the archive.
/// The `runtime-tokio` feature will panic on drop if not finalized.
///
/// [`into_inner`]: Builder::into_inner
/// [`finish`]: Builder::finish
pub struct Builder<W: Write + Unpin + Send + Sync> {
    mode: HeaderMode,
    follow: bool,
    finished: bool,
    obj: Option<W>,
}

impl<W: Write + Unpin + Send + Sync> Builder<W> {
    /// Create a new archive builder with the underlying object as the
    /// destination of all data written. The builder will use
    /// `HeaderMode::Complete` by default.
    pub fn new(obj: W) -> Builder<W> {
        Builder {
            mode: HeaderMode::Complete,
            follow: true,
            finished: false,
            obj: Some(obj),
        }
    }

    /// Changes the HeaderMode that will be used when reading fs Metadata for
    /// methods that implicitly read metadata for an input Path. Notably, this
    /// does _not_ apply to `append(Header)`.
    pub fn mode(&mut self, mode: HeaderMode) {
        self.mode = mode;
    }

    /// Follow symlinks, archiving the contents of the file they point to rather
    /// than adding a symlink to the archive. Defaults to true.
    pub fn follow_symlinks(&mut self, follow: bool) {
        self.follow = follow;
    }

    /// Gets shared reference to the underlying object.
    pub fn get_ref(&self) -> &W {
        self.obj.as_ref().unwrap()
    }

    /// Gets mutable reference to the underlying object.
    ///
    /// Note that care must be taken while writing to the underlying
    /// object. But, e.g. `get_mut().flush()` is claimed to be safe and
    /// useful in the situations when one needs to be ensured that
    /// tar entry was flushed to the disk.
    pub fn get_mut(&mut self) -> &mut W {
        self.obj.as_mut().unwrap()
    }

    /// Unwrap this archive, returning the underlying object.
    ///
    /// This function will finish writing the archive if the `finish` function
    /// hasn't yet been called, returning any I/O error which happens during
    /// that operation.
    pub async fn into_inner(mut self) -> io::Result<W> {
        if !self.finished {
            self.finish().await?;
        }
        Ok(self.obj.take().unwrap())
    }

    /// Adds a new entry to this archive.
    ///
    /// This function will append the header specified, followed by contents of
    /// the stream specified by `data`. To produce a valid archive the `size`
    /// field of `header` must be the same as the length of the stream that's
    /// being written. Additionally the checksum for the header should have been
    /// set via the `set_cksum` method.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all entries have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Errors
    ///
    /// This function will return an error for any intermittent I/O error which
    /// occurs when either reading or writing.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "runtime-async-std", doc = "```")]
    #[cfg_attr(feature = "runtime-tokio", doc = "```ignore")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_tar::{Builder, Header};
    ///
    /// let mut header = Header::new_gnu();
    /// header.set_path("foo")?;
    /// header.set_size(4);
    /// header.set_cksum();
    ///
    /// let mut data: &[u8] = &[1, 2, 3, 4];
    ///
    /// let mut ar = Builder::new(Vec::new());
    /// ar.append(&header, data).await?;
    /// let data = ar.into_inner().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn append<R: Read + Unpin + Send>(
        &mut self,
        header: &Header,
        mut data: R,
    ) -> io::Result<()> {
        append(self.get_mut(), header, &mut data).await?;

        Ok(())
    }

    /// Adds a new entry to this archive with the specified path.
    ///
    /// This function will set the specified path in the given header, which may
    /// require appending a GNU long-name extension entry to the archive first.
    /// The checksum for the header will be automatically updated via the
    /// `set_cksum` method after setting the path. No other metadata in the
    /// header will be modified.
    ///
    /// Then it will append the header, followed by contents of the stream
    /// specified by `data`. To produce a valid archive the `size` field of
    /// `header` must be the same as the length of the stream that's being
    /// written.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all entries have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Errors
    ///
    /// This function will return an error for any intermittent I/O error which
    /// occurs when either reading or writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_tar::{Builder, Header};
    ///
    /// let mut header = Header::new_gnu();
    /// header.set_size(4);
    /// header.set_cksum();
    ///
    /// let mut data: &[u8] = &[1, 2, 3, 4];
    ///
    /// let mut ar = Builder::new(Vec::new());
    /// ar.append_data(&mut header, "really/long/path/to/foo", data).await?;
    /// let data = ar.into_inner().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn append_data<P: AsRef<Path>, R: Read + Unpin + Send>(
        &mut self,
        header: &mut Header,
        path: P,
        data: R,
    ) -> io::Result<()> {
        prepare_header_path(self.get_mut(), header, path.as_ref()).await?;
        header.set_cksum();
        self.append(header, data).await?;

        Ok(())
    }

    /// Adds a file on the local filesystem to this archive.
    ///
    /// This function will open the file specified by `path` and insert the file
    /// into the archive with the appropriate metadata set, returning any I/O
    /// error which occurs while writing. The path name for the file inside of
    /// this archive will be the same as `path`, and it is required that the
    /// path is a relative path.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all files have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_tar::Builder;
    ///
    /// let mut ar = Builder::new(Vec::new());
    ///
    /// ar.append_path("foo/bar.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn append_path<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mode = self.mode;
        let follow = self.follow;
        append_path_with_name(self.get_mut(), path.as_ref(), None, mode, follow).await?;
        Ok(())
    }

    /// Adds a file on the local filesystem to this archive under another name.
    ///
    /// This function will open the file specified by `path` and insert the file
    /// into the archive as `name` with appropriate metadata set, returning any
    /// I/O error which occurs while writing. The path name for the file inside
    /// of this archive will be `name` is required to be a relative path.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all files have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_tar::Builder;
    ///
    /// let mut ar = Builder::new(Vec::new());
    ///
    /// // Insert the local file "foo/bar.txt" in the archive but with the name
    /// // "bar/foo.txt".
    /// ar.append_path_with_name("foo/bar.txt", "bar/foo.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn append_path_with_name<P: AsRef<Path>, N: AsRef<Path>>(
        &mut self,
        path: P,
        name: N,
    ) -> io::Result<()> {
        let mode = self.mode;
        let follow = self.follow;
        append_path_with_name(
            self.get_mut(),
            path.as_ref(),
            Some(name.as_ref()),
            mode,
            follow,
        )
        .await?;
        Ok(())
    }

    /// Adds a file to this archive with the given path as the name of the file
    /// in the archive.
    ///
    /// This will use the metadata of `file` to populate a `Header`, and it will
    /// then append the file to the archive with the name `path`.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all files have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "runtime-async-std", doc = "```no_run")]
    #[cfg_attr(feature = "runtime-tokio", doc = "```ignore")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_tar::Builder;
    ///
    /// let mut ar = Builder::new(Vec::new());
    ///
    /// // Open the file at one location, but insert it into the archive with a
    /// // different name.
    /// let mut f = File::open("foo/bar/baz.txt").await?;
    /// ar.append_file("bar/baz.txt", &mut f).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn append_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        file: &mut fs::File,
    ) -> io::Result<()> {
        let mode = self.mode;
        append_file(self.get_mut(), path.as_ref(), file, mode).await?;
        Ok(())
    }

    /// Adds a directory to this archive with the given path as the name of the
    /// directory in the archive.
    ///
    /// This will use `stat` to populate a `Header`, and it will then append the
    /// directory to the archive with the name `path`.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all files have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "runtime-async-std", doc = "```")]
    #[cfg_attr(feature = "runtime-tokio", doc = "```ignore")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_tar::Builder;
    ///
    /// let mut ar = Builder::new(Vec::new());
    ///
    /// // Use the directory at one location, but insert it into the archive
    /// // with a different name.
    /// ar.append_dir("bardir", ".").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn append_dir<P, Q>(&mut self, path: P, src_path: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let mode = self.mode;
        append_dir(self.get_mut(), path.as_ref(), src_path.as_ref(), mode).await?;
        Ok(())
    }

    /// Adds a directory and all of its contents (recursively) to this archive
    /// with the given path as the name of the directory in the archive.
    ///
    /// Note that this will not attempt to seek the archive to a valid position,
    /// so if the archive is in the middle of a read or some other similar
    /// operation then this may corrupt the archive.
    ///
    /// Also note that after all files have been written to an archive the
    /// `finish` function needs to be called to finish writing the archive.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "runtime-async-std", doc = "```")]
    #[cfg_attr(feature = "runtime-tokio", doc = "```ignore")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_tar::Builder;
    ///
    /// let mut ar = Builder::new(Vec::new());
    ///
    /// // Use the directory at one location, but insert it into the archive
    /// // with a different name.
    /// ar.append_dir_all("bardir", ".").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    pub async fn append_dir_all<P, Q>(&mut self, path: P, src_path: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let mode = self.mode;
        let follow = self.follow;
        append_dir_all(
            self.get_mut(),
            path.as_ref(),
            src_path.as_ref(),
            mode,
            follow,
        )
        .await?;
        Ok(())
    }

    /// Finish writing this archive, emitting the termination sections.
    ///
    /// This function should only be called when the archive has been written
    /// entirely and if an I/O error happens the underlying object still needs
    /// to be acquired.
    ///
    /// In most situations the `into_inner` method should be preferred.
    pub async fn finish(&mut self) -> io::Result<()> {
        if self.finished {
            return Ok(());
        }
        self.finished = true;
        self.get_mut().write_all(&[0; 1024]).await?;
        Ok(())
    }
}

async fn append(
    mut dst: &mut (dyn Write + Unpin + Send),
    header: &Header,
    mut data: &mut (dyn Read + Unpin + Send),
) -> io::Result<()> {
    dst.write_all(header.as_bytes()).await?;
    let len = io::copy(&mut data, &mut dst).await?;

    // Pad with zeros if necessary.
    let buf = [0; 512];
    let remaining = 512 - (len % 512);
    if remaining < 512 {
        dst.write_all(&buf[..remaining as usize]).await?;
    }

    Ok(())
}

async fn append_path_with_name(
    dst: &mut (dyn Write + Unpin + Sync + Send),
    path: &Path,
    name: Option<&Path>,
    mode: HeaderMode,
    follow: bool,
) -> io::Result<()> {
    let stat = if follow {
        metadata(path).await.map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting metadata for {}", err, path.display()),
            )
        })?
    } else {
        symlink_metadata(path).await.map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting metadata for {}", err, path.display()),
            )
        })?
    };
    let ar_name = name.unwrap_or(path);
    if stat.is_file() {
        append_fs(
            dst,
            ar_name,
            &stat,
            &mut fs::File::open(path).await?,
            mode,
            None,
        )
        .await?;
        Ok(())
    } else if stat.is_dir() {
        append_fs(dst, ar_name, &stat, &mut io::empty(), mode, None).await?;
        Ok(())
    } else if stat.file_type().is_symlink() {
        let link_name = fs::read_link(path).await?;
        append_fs(
            dst,
            ar_name,
            &stat,
            &mut io::empty(),
            mode,
            Some(&link_name),
        )
        .await?;
        Ok(())
    } else {
        Err(other(&format!("{} has unknown file type", path.display())))
    }
}

async fn append_file(
    dst: &mut (dyn Write + Unpin + Send + Sync),
    path: &Path,
    file: &mut fs::File,
    mode: HeaderMode,
) -> io::Result<()> {
    let stat = file.metadata().await?;
    append_fs(dst, path, &stat, file, mode, None).await?;
    Ok(())
}

async fn append_dir(
    dst: &mut (dyn Write + Unpin + Send + Sync),
    path: &Path,
    src_path: &Path,
    mode: HeaderMode,
) -> io::Result<()> {
    let stat = fs::metadata(src_path).await?;
    append_fs(dst, path, &stat, &mut io::empty(), mode, None).await?;
    Ok(())
}

fn prepare_header(size: u64, entry_type: EntryType) -> Header {
    let mut header = Header::new_gnu();
    let name = b"././@LongLink";
    header.as_gnu_mut().unwrap().name[..name.len()].clone_from_slice(&name[..]);
    header.set_mode(0o644);
    header.set_uid(0);
    header.set_gid(0);
    header.set_mtime(0);
    // + 1 to be compliant with GNU tar
    header.set_size(size + 1);
    header.set_entry_type(entry_type);
    header.set_cksum();
    header
}

async fn prepare_header_path(
    dst: &mut (dyn Write + Unpin + Send + Sync),
    header: &mut Header,
    path: &Path,
) -> io::Result<()> {
    // Try to encode the path directly in the header, but if it ends up not
    // working (probably because it's too long) then try to use the GNU-specific
    // long name extension by emitting an entry which indicates that it's the
    // filename.
    if let Err(e) = header.set_path(path) {
        let data = path2bytes(path)?;
        let max = header.as_old().name.len();
        //  Since e isn't specific enough to let us know the path is indeed too
        //  long, verify it first before using the extension.
        if data.len() < max {
            return Err(e);
        }
        let header2 = prepare_header(data.len() as u64, EntryType::GNULongName);
        // null-terminated string
        let mut data2 = data.chain(io::repeat(0).take(1));
        append(dst, &header2, &mut data2).await?;
        // Truncate the path to store in the header we're about to emit to
        // ensure we've got something at least mentioned.
        let path = bytes2path(Cow::Borrowed(&data[..max]))?;
        header.set_truncated_path_for_gnu_header(&path)?;
    }
    Ok(())
}

async fn prepare_header_link(
    dst: &mut (dyn Write + Unpin + Send + Sync),
    header: &mut Header,
    link_name: &Path,
) -> io::Result<()> {
    // Same as previous function but for linkname
    if let Err(e) = header.set_link_name(link_name) {
        let data = path2bytes(link_name)?;
        if data.len() < header.as_old().linkname.len() {
            return Err(e);
        }
        let header2 = prepare_header(data.len() as u64, EntryType::GNULongLink);
        let mut data2 = data.chain(io::repeat(0).take(1));
        append(dst, &header2, &mut data2).await?;
    }
    Ok(())
}

async fn append_fs(
    dst: &mut (dyn Write + Unpin + Send + Sync),
    path: &Path,
    meta: &Metadata,
    read: &mut (dyn Read + Unpin + Sync + Send),
    mode: HeaderMode,
    link_name: Option<&Path>,
) -> io::Result<()> {
    let mut header = Header::new_gnu();

    prepare_header_path(dst, &mut header, path).await?;
    header.set_metadata_in_mode(meta, mode);
    if let Some(link_name) = link_name {
        prepare_header_link(dst, &mut header, link_name).await?;
    }
    header.set_cksum();
    append(dst, &header, read).await?;

    Ok(())
}

async fn append_dir_all(
    dst: &mut (dyn Write + Unpin + Send + Sync),
    path: &Path,
    src_path: &Path,
    mode: HeaderMode,
    follow: bool,
) -> io::Result<()> {
    let mut stack = vec![(src_path.to_path_buf(), true, false)];
    while let Some((src, is_dir, is_symlink)) = stack.pop() {
        let dest = path.join(src.strip_prefix(src_path).unwrap());

        #[cfg(feature = "runtime-async-std")]
        async fn check_is_dir(path: &Path) -> bool {
            path.is_dir().await
        }
        #[cfg(feature = "runtime-tokio")]
        async fn check_is_dir(path: &Path) -> bool {
            fs::metadata(path)
                .await
                .map(|m| m.is_dir())
                .unwrap_or(false)
        }

        // In case of a symlink pointing to a directory, is_dir is false, but src.is_dir() will return true
        if is_dir || (is_symlink && follow && check_is_dir(&src).await) {
            #[cfg(feature = "runtime-async-std")]
            let mut entries = fs::read_dir(&src).await?;
            #[cfg(feature = "runtime-tokio")]
            let mut entries = tokio_stream::wrappers::ReadDirStream::new(fs::read_dir(&src).await?);
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                let file_type = entry.file_type().await?;
                stack.push((entry.path(), file_type.is_dir(), file_type.is_symlink()));
            }
            if dest != Path::new("") {
                append_dir(dst, &dest, &src, mode).await?;
            }
        } else if !follow && is_symlink {
            let stat = fs::symlink_metadata(&src).await?;
            let link_name = fs::read_link(&src).await?;
            append_fs(dst, &dest, &stat, &mut io::empty(), mode, Some(&link_name)).await?;
        } else {
            append_file(dst, &dest, &mut fs::File::open(src).await?, mode).await?;
        }
    }
    Ok(())
}

#[cfg(feature = "runtime-async-std")]
impl<W: Write + Unpin + Send + Sync> Drop for Builder<W> {
    fn drop(&mut self) {
        async_std::task::block_on(async move {
            let _ = self.finish().await;
        });
    }
}

#[cfg(feature = "runtime-tokio")]
impl<W: Write + Unpin + Send + Sync> Drop for Builder<W> {
    fn drop(&mut self) {
        if !self.finished && !std::thread::panicking() && self.obj.is_some() {
            panic!("Builder dropped without finalizing; call finish() or into_inner()");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_impl_all!(fs::File: Send, Sync);
    assert_impl_all!(Builder<fs::File>: Send, Sync);
}
