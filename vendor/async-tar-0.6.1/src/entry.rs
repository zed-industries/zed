use std::{
    borrow::Cow,
    cmp, fmt, marker,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "runtime-async-std")]
use async_std::{
    fs,
    fs::{OpenOptions, Permissions},
    io::{self, Error, ErrorKind, SeekFrom, prelude::*},
    path::{Component, Path, PathBuf},
};
use futures_core::ready;
#[cfg(all(unix, feature = "runtime-tokio"))]
use std::fs::Permissions;
#[cfg(feature = "runtime-tokio")]
use std::path::{Component, Path, PathBuf};
#[cfg(feature = "runtime-tokio")]
use tokio::{
    fs,
    fs::OpenOptions,
    io::{self, AsyncRead as Read, AsyncReadExt, AsyncSeekExt, Error, ErrorKind, SeekFrom},
};

use filetime::{self, FileTime};

use crate::{
    Archive, Header, PaxExtensions, error::TarError, fs_canonicalize, header::bytes2path, other,
    pax::pax_extensions, symlink_metadata,
};

/// A read-only view into an entry of an archive.
///
/// This structure is a window into a portion of a borrowed archive which can
/// be inspected. It acts as a file handle by implementing the Reader trait. An
/// entry cannot be rewritten once inserted into an archive.
pub struct Entry<R: Read + Unpin> {
    fields: EntryFields<R>,
    _ignored: marker::PhantomData<Archive<R>>,
}

impl<R: Read + Unpin> fmt::Debug for Entry<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("fields", &self.fields)
            .finish()
    }
}

// private implementation detail of `Entry`, but concrete (no type parameters)
// and also all-public to be constructed from other modules.
pub struct EntryFields<R: Read + Unpin> {
    pub long_pathname: Option<Vec<u8>>,
    pub long_linkname: Option<Vec<u8>>,
    pub pax_extensions: Option<Vec<u8>>,
    pub header: Header,
    pub size: u64,
    pub header_pos: u64,
    pub file_pos: u64,
    pub data: Vec<EntryIo<R>>,
    pub unpack_xattrs: bool,
    pub preserve_permissions: bool,
    pub preserve_mtime: bool,
    pub(crate) read_state: Option<EntryIo<R>>,
}

impl<R: Read + Unpin> fmt::Debug for EntryFields<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntryFields")
            .field("long_pathname", &self.long_pathname)
            .field("long_linkname", &self.long_linkname)
            .field("pax_extensions", &self.pax_extensions)
            .field("header", &self.header)
            .field("size", &self.size)
            .field("header_pos", &self.header_pos)
            .field("file_pos", &self.file_pos)
            .field("data", &self.data)
            .field("unpack_xattrs", &self.unpack_xattrs)
            .field("preserve_permissions", &self.preserve_permissions)
            .field("preserve_mtime", &self.preserve_mtime)
            .field("read_state", &self.read_state)
            .finish()
    }
}

pub enum EntryIo<R: Read + Unpin> {
    Pad(io::Take<io::Repeat>),
    Data(io::Take<R>),
}

impl<R: Read + Unpin> fmt::Debug for EntryIo<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryIo::Pad(_) => write!(f, "EntryIo::Pad"),
            EntryIo::Data(_) => write!(f, "EntryIo::Data"),
        }
    }
}

/// When unpacking items the unpacked thing is returned to allow custom
/// additional handling by users. Today the File is returned, in future
/// the enum may be extended with kinds for links, directories etc.
#[derive(Debug)]
#[non_exhaustive]
pub enum Unpacked {
    /// A file was unpacked.
    File(fs::File),
    /// A directory, hardlink, symlink, or other node was unpacked.
    Other,
}

impl<R: Read + Unpin> Entry<R> {
    /// Returns the path name for this entry.
    ///
    /// This method may fail if the pathname is not valid Unicode and this is
    /// called on a Windows platform.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators, and it will not always return the same value as
    /// `self.header().path()` as some archive formats have support for longer
    /// path names described in separate entries.
    ///
    /// It is recommended to use this method instead of inspecting the `header`
    /// directly to ensure that various archive formats are handled correctly.
    pub fn path(&self) -> io::Result<Cow<'_, Path>> {
        self.fields.path()
    }

    /// Returns the raw bytes listed for this entry.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators, and it will not always return the same value as
    /// `self.header().path_bytes()` as some archive formats have support for
    /// longer path names described in separate entries.
    pub fn path_bytes(&self) -> Cow<'_, [u8]> {
        self.fields.path_bytes()
    }

    /// Returns the link name for this entry, if any is found.
    ///
    /// This method may fail if the pathname is not valid Unicode and this is
    /// called on a Windows platform. `Ok(None)` being returned, however,
    /// indicates that the link name was not present.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators, and it will not always return the same value as
    /// `self.header().link_name()` as some archive formats have support for
    /// longer path names described in separate entries.
    ///
    /// It is recommended to use this method instead of inspecting the `header`
    /// directly to ensure that various archive formats are handled correctly.
    pub fn link_name(&self) -> io::Result<Option<Cow<'_, Path>>> {
        self.fields.link_name()
    }

    /// Returns the link name for this entry, in bytes, if listed.
    ///
    /// Note that this will not always return the same value as
    /// `self.header().link_name_bytes()` as some archive formats have support for
    /// longer path names described in separate entries.
    pub fn link_name_bytes(&self) -> Option<Cow<'_, [u8]>> {
        self.fields.link_name_bytes()
    }

    /// Returns an iterator over the pax extensions contained in this entry.
    ///
    /// Pax extensions are a form of archive where extra metadata is stored in
    /// key/value pairs in entries before the entry they're intended to
    /// describe. For example this can be used to describe long file name or
    /// other metadata like atime/ctime/mtime in more precision.
    ///
    /// The returned iterator will yield key/value pairs for each extension.
    ///
    /// `None` will be returned if this entry does not indicate that it itself
    /// contains extensions, or if there were no previous extensions describing
    /// it.
    ///
    /// Note that global pax extensions are intended to be applied to all
    /// archive entries.
    ///
    /// Also note that this function will read the entire entry if the entry
    /// itself is a list of extensions.
    pub async fn pax_extensions(&mut self) -> io::Result<Option<PaxExtensions<'_>>> {
        self.fields.pax_extensions().await
    }

    /// Returns access to the header of this entry in the archive.
    ///
    /// This provides access to the metadata for this entry in the archive.
    pub fn header(&self) -> &Header {
        &self.fields.header
    }

    /// Returns the starting position, in bytes, of the header of this entry in
    /// the archive.
    ///
    /// The header is always a contiguous section of 512 bytes, so if the
    /// underlying reader implements `Seek`, then the slice from `header_pos` to
    /// `header_pos + 512` contains the raw header bytes.
    pub fn raw_header_position(&self) -> u64 {
        self.fields.header_pos
    }

    /// Returns the starting position, in bytes, of the file of this entry in
    /// the archive.
    ///
    /// If the file of this entry is continuous (e.g. not a sparse file), and
    /// if the underlying reader implements `Seek`, then the slice from
    /// `file_pos` to `file_pos + entry_size` contains the raw file bytes.
    pub fn raw_file_position(&self) -> u64 {
        self.fields.file_pos
    }

    /// Writes this file to the specified location.
    ///
    /// This function will write the entire contents of this file into the
    /// location specified by `dst`. Metadata will also be propagated to the
    /// path `dst`.
    ///
    /// This function will create a file at the path `dst`, and it is required
    /// that the intermediate directories are created. Any existing file at the
    /// location `dst` will be overwritten.
    ///
    /// > **Note**: This function does not have as many sanity checks as
    /// > `Archive::unpack` or `Entry::unpack_in`. As a result if you're
    /// > thinking of unpacking untrusted tarballs you may want to review the
    /// > implementations of the previous two functions and perhaps implement
    /// > similar logic yourself.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "runtime-async-std", doc = "```no_run")]
    #[cfg_attr(feature = "runtime-tokio", doc = "```ignore")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::prelude::*;
    /// use async_tar::Archive;
    ///
    /// let mut ar = Archive::new(File::open("foo.tar").await?);
    /// let mut entries = ar.entries()?;
    /// let mut i = 0;
    /// while let Some(file) = entries.next().await {
    ///     let mut file = file?;
    ///     file.unpack(format!("file-{}", i)).await?;
    ///     i += 1;
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn unpack<P: AsRef<Path>>(&mut self, dst: P) -> io::Result<Unpacked> {
        self.fields.unpack(None, dst.as_ref()).await
    }

    /// Extracts this file under the specified path, avoiding security issues.
    ///
    /// This function will write the entire contents of this file into the
    /// location obtained by appending the path of this file in the archive to
    /// `dst`, creating any intermediate directories if needed. Metadata will
    /// also be propagated to the path `dst`. Any existing file at the location
    /// `dst` will be overwritten.
    ///
    /// This function carefully avoids writing outside of `dst`. If the file has
    /// a '..' in its path, this function will skip it and return false.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "runtime-async-std", doc = "```no_run")]
    #[cfg_attr(feature = "runtime-tokio", doc = "```ignore")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_tar::Archive;
    /// use async_std::prelude::*;
    ///
    /// let mut ar = Archive::new(File::open("foo.tar").await?);
    /// let mut entries = ar.entries()?;
    /// let mut i = 0;
    /// while let Some(file) = entries.next().await {
    ///     let mut file = file.unwrap();
    ///     file.unpack_in("target").await?;
    ///     i += 1;
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn unpack_in<P: AsRef<Path>>(&mut self, dst: P) -> io::Result<bool> {
        self.fields.unpack_in(dst.as_ref()).await
    }

    /// Indicate whether extended file attributes (xattrs on Unix) are preserved
    /// when unpacking this entry.
    ///
    /// This flag is disabled by default and is currently only implemented on
    /// Unix using xattr support. This may eventually be implemented for
    /// Windows, however, if other archive implementations are found which do
    /// this as well.
    pub fn set_unpack_xattrs(&mut self, unpack_xattrs: bool) {
        self.fields.unpack_xattrs = unpack_xattrs;
    }

    /// Indicate whether extended permissions (like suid on Unix) are preserved
    /// when unpacking this entry.
    ///
    /// This flag is disabled by default and is currently only implemented on
    /// Unix.
    pub fn set_preserve_permissions(&mut self, preserve: bool) {
        self.fields.preserve_permissions = preserve;
    }

    /// Indicate whether access time information is preserved when unpacking
    /// this entry.
    ///
    /// This flag is enabled by default.
    pub fn set_preserve_mtime(&mut self, preserve: bool) {
        self.fields.preserve_mtime = preserve;
    }
}

#[cfg(feature = "runtime-async-std")]
impl<R: Read + Unpin> Read for Entry<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        into: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.fields).poll_read(cx, into)
    }
}

#[cfg(feature = "runtime-tokio")]
impl<R: Read + Unpin> Read for Entry<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        into: &mut tokio::io::ReadBuf,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.fields).poll_read(cx, into)
    }
}

impl<R: Read + Unpin> EntryFields<R> {
    pub fn from(entry: Entry<R>) -> Self {
        entry.fields
    }

    pub fn into_entry(self) -> Entry<R> {
        Entry {
            fields: self,
            _ignored: marker::PhantomData,
        }
    }

    pub(crate) fn poll_read_all(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Vec<u8>>> {
        // Preallocate some data but don't let ourselves get too crazy now.
        let cap = cmp::min(self.size, 128 * 1024);
        let mut buf = Vec::with_capacity(cap as usize);

        // Copied from futures::ReadToEnd
        match ready!(poll_read_all_internal(self, cx, &mut buf)) {
            Ok(_) => Poll::Ready(Ok(buf)),
            Err(err) => Poll::Ready(Err(err)),
        }
    }

    pub async fn read_all(&mut self) -> io::Result<Vec<u8>> {
        // Preallocate some data but don't let ourselves get too crazy now.
        let cap = cmp::min(self.size, 128 * 1024);
        let mut v = Vec::with_capacity(cap as usize);
        self.read_to_end(&mut v).await.map(|_| v)
    }

    fn path(&self) -> io::Result<Cow<'_, Path>> {
        bytes2path(self.path_bytes())
    }

    fn path_bytes(&self) -> Cow<'_, [u8]> {
        if let Some(ref bytes) = self.long_pathname {
            if let Some(&0) = bytes.last() {
                Cow::Borrowed(&bytes[..bytes.len() - 1])
            } else {
                Cow::Borrowed(bytes)
            }
        } else {
            if let Some(ref pax) = self.pax_extensions {
                let pax = pax_extensions(pax)
                    .filter_map(Result::ok)
                    .find(|f| f.key_bytes() == b"path")
                    .map(|f| f.value_bytes());
                if let Some(field) = pax {
                    return Cow::Borrowed(field);
                }
            }
            self.header.path_bytes()
        }
    }

    /// Gets the path in a "lossy" way, used for error reporting ONLY.
    fn path_lossy(&self) -> String {
        String::from_utf8_lossy(&self.path_bytes()).to_string()
    }

    fn link_name(&self) -> io::Result<Option<Cow<'_, Path>>> {
        match self.link_name_bytes() {
            Some(bytes) => bytes2path(bytes).map(Some),
            None => Ok(None),
        }
    }

    fn link_name_bytes(&self) -> Option<Cow<'_, [u8]>> {
        match self.long_linkname {
            Some(ref bytes) => {
                if let Some(&0) = bytes.last() {
                    Some(Cow::Borrowed(&bytes[..bytes.len() - 1]))
                } else {
                    Some(Cow::Borrowed(bytes))
                }
            }
            None => self.header.link_name_bytes(),
        }
    }

    async fn pax_extensions(&mut self) -> io::Result<Option<PaxExtensions<'_>>> {
        if self.pax_extensions.is_none() {
            if !self.header.entry_type().is_pax_global_extensions()
                && !self.header.entry_type().is_pax_local_extensions()
            {
                return Ok(None);
            }
            self.pax_extensions = Some(self.read_all().await?);
        }
        Ok(Some(pax_extensions(self.pax_extensions.as_ref().unwrap())))
    }

    async fn unpack_in(&mut self, dst: &Path) -> io::Result<bool> {
        // Notes regarding bsdtar 2.8.3 / libarchive 2.8.3:
        // * Leading '/'s are trimmed. For example, `///test` is treated as
        //   `test`.
        // * If the filename contains '..', then the file is skipped when
        //   extracting the tarball.
        // * '//' within a filename is effectively skipped. An error is
        //   logged, but otherwise the effect is as if any two or more
        //   adjacent '/'s within the filename were consolidated into one
        //   '/'.
        //
        // Most of this is handled by the `path` module of the standard
        // library, but we specially handle a few cases here as well.

        let mut file_dst = dst.to_path_buf();
        {
            let path = self.path().map_err(|e| {
                TarError::new(
                    &format!("invalid path in entry header: {}", self.path_lossy()),
                    e,
                )
            })?;
            for part in path.components() {
                match part {
                    // Leading '/' characters, root paths, and '.'
                    // components are just ignored and treated as "empty
                    // components"
                    Component::Prefix(..) | Component::RootDir | Component::CurDir => continue,

                    // If any part of the filename is '..', then skip over
                    // unpacking the file to prevent directory traversal
                    // security issues.  See, e.g.: CVE-2001-1267,
                    // CVE-2002-0399, CVE-2005-1918, CVE-2007-4131
                    Component::ParentDir => return Ok(false),

                    Component::Normal(part) => file_dst.push(part),
                }
            }
        }

        // Skip cases where only slashes or '.' parts were seen, because
        // this is effectively an empty filename.
        if *dst == *file_dst {
            return Ok(true);
        }

        // Skip entries without a parent (i.e. outside of FS root)
        let parent = match file_dst.parent() {
            Some(p) => p,
            None => return Ok(false),
        };

        self.ensure_dir_created(dst, parent)
            .await
            .map_err(|e| TarError::new(&format!("failed to create `{}`", parent.display()), e))?;

        let canon_target = self.validate_inside_dst(dst, parent).await?;

        self.unpack(Some(&canon_target), &file_dst)
            .await
            .map_err(|e| TarError::new(&format!("failed to unpack `{}`", file_dst.display()), e))?;

        Ok(true)
    }

    /// Unpack as destination directory `dst`.
    async fn unpack_dir(&mut self, dst: &Path) -> io::Result<()> {
        // If the directory already exists just let it slide
        match fs::create_dir(dst).await {
            Ok(()) => Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    let prev = fs::metadata(dst).await;
                    if prev.map(|m| m.is_dir()).unwrap_or(false) {
                        return Ok(());
                    }
                }
                Err(Error::new(
                    err.kind(),
                    format!("{} when creating dir {}", err, dst.display()),
                ))
            }
        }
    }

    /// Returns access to the header of this entry in the archive.
    async fn unpack(&mut self, target_base: Option<&Path>, dst: &Path) -> io::Result<Unpacked> {
        let kind = self.header.entry_type();

        if kind.is_dir() {
            self.unpack_dir(dst).await?;
            if let Ok(mode) = self.header.mode() {
                set_perms(dst, None, mode, self.preserve_permissions).await?;
            }
            return Ok(Unpacked::Other);
        } else if kind.is_hard_link() || kind.is_symlink() {
            let src = match self.link_name()? {
                Some(name) => name,
                None => {
                    return Err(other(&format!(
                        "hard link listed for {} but no link name found",
                        String::from_utf8_lossy(self.header.as_bytes())
                    )));
                }
            };

            if src.iter().count() == 0 {
                return Err(other(&format!(
                    "symlink destination for {} is empty",
                    String::from_utf8_lossy(self.header.as_bytes())
                )));
            }

            if kind.is_hard_link() {
                let link_src = match target_base {
                    // If we're unpacking within a directory then ensure that
                    // the destination of this hard link is both present and
                    // inside our own directory. This is needed because we want
                    // to make sure to not overwrite anything outside the root.
                    //
                    // Note that this logic is only needed for hard links
                    // currently. With symlinks the `validate_inside_dst` which
                    // happens before this method as part of `unpack_in` will
                    // use canonicalization to ensure this guarantee. For hard
                    // links though they're canonicalized to their existing path
                    // so we need to validate at this time.
                    Some(p) => {
                        let link_src = p.join(src);
                        self.validate_inside_dst(p, &link_src).await?;
                        link_src
                    }
                    None => src.into_owned(),
                };
                fs::hard_link(&link_src, dst).await.map_err(|err| {
                    Error::new(
                        err.kind(),
                        format!(
                            "{} when hard linking {} to {}",
                            err,
                            link_src.display(),
                            dst.display()
                        ),
                    )
                })?;
            } else {
                symlink(&src, dst).await.map_err(|err| {
                    Error::new(
                        err.kind(),
                        format!(
                            "{} when symlinking {} to {}",
                            err,
                            src.display(),
                            dst.display()
                        ),
                    )
                })?;
            };
            return Ok(Unpacked::Other);

            #[cfg(target_arch = "wasm32")]
            #[allow(unused_variables)]
            async fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
                Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
            }

            #[cfg(windows)]
            async fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
                #[cfg(feature = "runtime-async-std")]
                {
                    async_std::os::windows::fs::symlink_file(src, dst).await
                }
                #[cfg(feature = "runtime-tokio")]
                {
                    tokio::fs::symlink_file(src, dst).await
                }
            }

            #[cfg(any(unix, target_os = "redox"))]
            async fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
                #[cfg(feature = "runtime-async-std")]
                async_std::os::unix::fs::symlink(src, dst).await?;
                #[cfg(feature = "runtime-tokio")]
                tokio::fs::symlink(src, dst).await?;

                Ok(())
            }
        } else if kind.is_pax_global_extensions()
            || kind.is_pax_local_extensions()
            || kind.is_gnu_longname()
            || kind.is_gnu_longlink()
        {
            return Ok(Unpacked::Other);
        };

        // Old BSD-tar compatibility.
        // Names that have a trailing slash should be treated as a directory.
        // Only applies to old headers.
        if self.header.as_ustar().is_none() && self.path_bytes().ends_with(b"/") {
            self.unpack_dir(dst).await?;
            if let Ok(mode) = self.header.mode() {
                set_perms(dst, None, mode, self.preserve_permissions).await?;
            }
            return Ok(Unpacked::Other);
        }

        // Note the lack of `else` clause above. According to the FreeBSD
        // documentation:
        //
        // > A POSIX-compliant implementation must treat any unrecognized
        // > typeflag value as a regular file.
        //
        // As a result if we don't recognize the kind we just write out the file
        // as we would normally.

        // Ensure we write a new file rather than overwriting in-place which
        // is attackable; if an existing file is found unlink it.
        async fn open(dst: &Path) -> io::Result<fs::File> {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dst)
                .await
        }
        let mut f = async {
            let mut f = match open(dst).await {
                Ok(f) => Ok(f),
                Err(err) => {
                    if err.kind() == ErrorKind::AlreadyExists {
                        match fs::remove_file(dst).await {
                            Ok(()) => open(dst).await,
                            Err(ref e) if e.kind() == io::ErrorKind::NotFound => open(dst).await,
                            Err(e) => Err(e),
                        }
                    } else {
                        Err(err)
                    }
                }
            }?;
            for io in self.data.drain(..) {
                match io {
                    EntryIo::Data(mut d) => {
                        let expected = d.limit();
                        if io::copy(&mut d, &mut f).await? != expected {
                            return Err(other("failed to write entire file"));
                        }
                    }
                    EntryIo::Pad(d) => {
                        // TODO: checked cast to i64
                        let to = SeekFrom::Current(d.limit() as i64);
                        let size = f.seek(to).await?;
                        f.set_len(size).await?;
                    }
                }
            }
            Ok::<fs::File, io::Error>(f)
        }
        .await
        .map_err(|e| {
            let header = self.header.path_bytes();
            TarError::new(
                &format!(
                    "failed to unpack `{}` into `{}`",
                    String::from_utf8_lossy(&header),
                    dst.display()
                ),
                e,
            )
        })?;

        if self.preserve_mtime {
            if let Ok(mtime) = self.header.mtime() {
                let mtime = FileTime::from_unix_time(mtime as i64, 0);
                filetime::set_file_times(dst, mtime, mtime).map_err(|e| {
                    TarError::new(&format!("failed to set mtime for `{}`", dst.display()), e)
                })?;
            }
        }
        if let Ok(mode) = self.header.mode() {
            set_perms(dst, Some(&mut f), mode, self.preserve_permissions).await?;
        }
        if self.unpack_xattrs {
            set_xattrs(self, dst).await?;
        }
        return Ok(Unpacked::File(f));

        async fn set_perms(
            dst: &Path,
            f: Option<&mut fs::File>,
            mode: u32,
            preserve: bool,
        ) -> Result<(), TarError> {
            _set_perms(dst, f, mode, preserve).await.map_err(|e| {
                TarError::new(
                    &format!(
                        "failed to set permissions to {:o} \
                         for `{}`",
                        mode,
                        dst.display()
                    ),
                    e,
                )
            })
        }

        #[cfg(any(unix, target_os = "redox"))]
        async fn _set_perms(
            dst: &Path,
            f: Option<&mut fs::File>,
            mode: u32,
            preserve: bool,
        ) -> io::Result<()> {
            use std::os::unix::prelude::*;

            let mode = if preserve { mode } else { mode & 0o777 };
            let perm = Permissions::from_mode(mode as _);
            match f {
                Some(f) => f.set_permissions(perm).await,
                None => fs::set_permissions(dst, perm).await,
            }
        }

        #[cfg(windows)]
        async fn _set_perms(
            dst: &Path,
            f: Option<&mut fs::File>,
            mode: u32,
            _preserve: bool,
        ) -> io::Result<()> {
            if mode & 0o200 == 0o200 {
                return Ok(());
            }
            match f {
                Some(f) => {
                    let mut perm = f.metadata().await?.permissions();
                    perm.set_readonly(true);
                    f.set_permissions(perm).await
                }
                None => {
                    let mut perm = fs::metadata(dst).await?.permissions();
                    perm.set_readonly(true);
                    fs::set_permissions(dst, perm).await
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        #[allow(unused_variables)]
        async fn _set_perms(
            dst: &Path,
            f: Option<&mut fs::File>,
            mode: u32,
            _preserve: bool,
        ) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
        }

        #[cfg(all(unix, feature = "xattr"))]
        async fn set_xattrs<R: Read + Unpin>(
            me: &mut EntryFields<R>,
            dst: &Path,
        ) -> io::Result<()> {
            use std::{ffi::OsStr, os::unix::prelude::*};

            let exts = match me.pax_extensions().await {
                Ok(Some(e)) => e,
                _ => return Ok(()),
            };
            let exts = exts
                .filter_map(Result::ok)
                .filter_map(|e| {
                    let key = e.key_bytes();
                    let prefix = b"SCHILY.xattr.";
                    if key.starts_with(prefix) {
                        Some((&key[prefix.len()..], e))
                    } else {
                        None
                    }
                })
                .map(|(key, e)| (OsStr::from_bytes(key), e.value_bytes()));

            for (key, value) in exts {
                xattr::set(dst, key, value).map_err(|e| {
                    TarError::new(
                        &format!(
                            "failed to set extended \
                             attributes to {}. \
                             Xattrs: key={:?}, value={:?}.",
                            dst.display(),
                            key,
                            String::from_utf8_lossy(value)
                        ),
                        e,
                    )
                })?;
            }

            Ok(())
        }
        // Windows does not completely support posix xattrs
        // https://en.wikipedia.org/wiki/Extended_file_attributes#Windows_NT
        #[cfg(any(
            windows,
            target_os = "redox",
            not(feature = "xattr"),
            target_arch = "wasm32"
        ))]
        async fn set_xattrs<R: Read + Unpin>(_: &mut EntryFields<R>, _: &Path) -> io::Result<()> {
            Ok(())
        }
    }

    async fn ensure_dir_created(&self, dst: &Path, dir: &Path) -> io::Result<()> {
        let mut ancestor = dir;
        let mut dirs_to_create = Vec::new();

        while symlink_metadata(ancestor).await.is_err() {
            dirs_to_create.push(ancestor);
            if let Some(parent) = ancestor.parent() {
                ancestor = parent;
            } else {
                break;
            }
        }
        for ancestor in dirs_to_create.into_iter().rev() {
            if let Some(parent) = ancestor.parent() {
                self.validate_inside_dst(dst, parent).await?;
            }
            fs::create_dir(ancestor).await?;
        }
        Ok(())
    }

    async fn validate_inside_dst(&self, dst: &Path, file_dst: &Path) -> io::Result<PathBuf> {
        // Abort if target (canonical) parent is outside of `dst`
        let canon_parent = fs_canonicalize(file_dst).await.map_err(|err| {
            Error::new(
                err.kind(),
                format!("{} while canonicalizing {}", err, file_dst.display()),
            )
        })?;
        let canon_target = fs_canonicalize(dst).await.map_err(|err| {
            Error::new(
                err.kind(),
                format!("{} while canonicalizing {}", err, dst.display()),
            )
        })?;
        if !canon_parent.starts_with(&canon_target) {
            let err = TarError::new(
                &format!(
                    "trying to unpack outside of destination path: {}",
                    canon_target.display()
                ),
                // TODO: use ErrorKind::InvalidInput here? (minor breaking change)
                Error::other("Invalid argument"),
            );
            return Err(err.into());
        }
        Ok(canon_target)
    }
}

#[cfg(feature = "runtime-async-std")]
impl<R: Read + Unpin> Read for EntryFields<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        into: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            if self.read_state.is_none() {
                if self.data.is_empty() {
                    self.read_state = None;
                } else {
                    self.read_state = Some(self.data.remove(0));
                }
            }

            if let Some(io) = &mut self.read_state {
                let ret = Pin::new(io).poll_read(cx, into);
                match ret {
                    Poll::Ready(Ok(0)) => {
                        self.read_state = None;
                        if self.data.is_empty() {
                            return Poll::Ready(Ok(0));
                        }
                        continue;
                    }
                    Poll::Ready(Ok(val)) => {
                        return Poll::Ready(Ok(val));
                    }
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Err(err));
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                }
            }
            // Unable to pull another value from `data`, so we are done.
            return Poll::Ready(Ok(0));
        }
    }
}

#[cfg(feature = "runtime-tokio")]
impl<R: Read + Unpin> Read for EntryFields<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        into: &mut tokio::io::ReadBuf,
    ) -> Poll<io::Result<()>> {
        loop {
            if self.read_state.is_none() {
                if self.data.is_empty() {
                    self.read_state = None;
                } else {
                    self.read_state = Some(self.data.remove(0));
                }
            }

            if let Some(io) = &mut self.read_state {
                let start = into.filled().len();
                let ret = Pin::new(io).poll_read(cx, into);
                match ret {
                    Poll::Ready(Ok(())) => {
                        let diff = into.filled().len() - start;
                        if diff == 0 {
                            self.read_state = None;
                            if self.data.is_empty() {
                                return Poll::Ready(Ok(()));
                            }
                            continue;
                        } else {
                            return Poll::Ready(Ok(()));
                        }
                    }
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Err(err));
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                }
            }
            // Unable to pull another value from `data`, so we are done.
            return Poll::Ready(Ok(()));
        }
    }
}

#[cfg(feature = "runtime-async-std")]
impl<R: Read + Unpin> Read for EntryIo<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        into: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            EntryIo::Pad(io) => Pin::new(io).poll_read(cx, into),
            EntryIo::Data(io) => Pin::new(io).poll_read(cx, into),
        }
    }
}

#[cfg(feature = "runtime-tokio")]
impl<R: Read + Unpin> Read for EntryIo<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        into: &mut tokio::io::ReadBuf,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            EntryIo::Pad(io) => Pin::new(io).poll_read(cx, into),
            EntryIo::Data(io) => Pin::new(io).poll_read(cx, into),
        }
    }
}

struct Guard<'a> {
    buf: &'a mut Vec<u8>,
    len: usize,
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        unsafe {
            self.buf.set_len(self.len);
        }
    }
}

#[cfg(feature = "runtime-async-std")]
fn poll_read_all_internal<R: Read + ?Sized>(
    mut rd: Pin<&mut R>,
    cx: &mut Context<'_>,
    buf: &mut Vec<u8>,
) -> Poll<io::Result<usize>> {
    let mut g = Guard {
        len: buf.len(),
        buf,
    };
    let ret;
    loop {
        if g.len == g.buf.len() {
            unsafe {
                g.buf.reserve(32);
                let capacity = g.buf.capacity();
                g.buf.set_len(capacity);

                let buf = &mut g.buf[g.len..];
                std::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
            }
        }

        match ready!(rd.as_mut().poll_read(cx, &mut g.buf[g.len..])) {
            Ok(0) => {
                ret = Poll::Ready(Ok(g.len));
                break;
            }
            Ok(n) => g.len += n,
            Err(e) => {
                ret = Poll::Ready(Err(e));
                break;
            }
        }
    }

    ret
}

#[cfg(feature = "runtime-tokio")]
fn poll_read_all_internal<R: Read + ?Sized>(
    mut rd: Pin<&mut R>,
    cx: &mut Context<'_>,
    buf: &mut Vec<u8>,
) -> Poll<io::Result<usize>> {
    let mut g = Guard {
        len: buf.len(),
        buf,
    };
    let ret;
    loop {
        if g.len == g.buf.len() {
            unsafe {
                g.buf.reserve(32);
                let capacity = g.buf.capacity();
                g.buf.set_len(capacity);

                let buf = &mut g.buf[g.len..];
                std::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
            }
        }

        let mut read_buf = io::ReadBuf::new(&mut g.buf[g.len..]);
        let start = read_buf.filled().len();
        match ready!(rd.as_mut().poll_read(cx, &mut read_buf)) {
            Ok(()) => {
                let diff = read_buf.filled().len() - start;
                if diff == 0 {
                    ret = Poll::Ready(Ok(g.len));
                    break;
                } else {
                    g.len += diff;
                }
            }
            Err(e) => {
                ret = Poll::Ready(Err(e));
                break;
            }
        }
    }

    ret
}
