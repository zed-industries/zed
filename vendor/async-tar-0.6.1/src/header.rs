#[cfg(any(unix, target_os = "redox"))]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::{borrow::Cow, fmt, iter, iter::repeat, iter::repeat_n, mem, str};

#[cfg(all(windows, feature = "runtime-async-std"))]
use async_std::fs;
#[cfg(feature = "runtime-async-std")]
use async_std::{
    fs::Metadata,
    io,
    path::{Component, Path, PathBuf},
};
#[cfg(feature = "runtime-tokio")]
use std::{
    fs::Metadata,
    path::{Component, Path, PathBuf},
};
#[cfg(all(windows, feature = "runtime-tokio"))]
use tokio::fs;
#[cfg(feature = "runtime-tokio")]
use tokio::io;

use crate::{EntryType, other};

/// Representation of the header of an entry in an archive
#[repr(C)]
#[allow(missing_docs)]
pub struct Header {
    bytes: [u8; 512],
}

/// Declares the information that should be included when filling a Header
/// from filesystem metadata.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum HeaderMode {
    /// All supported metadata, including mod/access times and ownership will
    /// be included.
    Complete,

    /// Only metadata that is directly relevant to the identity of a file will
    /// be included. In particular, ownership and mod/access times are excluded.
    Deterministic,
}

/// Representation of the header of an entry in an archive
#[repr(C)]
#[allow(missing_docs)]
pub struct OldHeader {
    pub name: [u8; 100],
    pub mode: [u8; 8],
    pub uid: [u8; 8],
    pub gid: [u8; 8],
    pub size: [u8; 12],
    pub mtime: [u8; 12],
    pub cksum: [u8; 8],
    pub linkflag: [u8; 1],
    pub linkname: [u8; 100],
    pub pad: [u8; 255],
}

/// Representation of the header of an entry in an archive
#[repr(C)]
#[allow(missing_docs)]
pub struct UstarHeader {
    pub name: [u8; 100],
    pub mode: [u8; 8],
    pub uid: [u8; 8],
    pub gid: [u8; 8],
    pub size: [u8; 12],
    pub mtime: [u8; 12],
    pub cksum: [u8; 8],
    pub typeflag: [u8; 1],
    pub linkname: [u8; 100],

    // UStar format
    pub magic: [u8; 6],
    pub version: [u8; 2],
    pub uname: [u8; 32],
    pub gname: [u8; 32],
    pub dev_major: [u8; 8],
    pub dev_minor: [u8; 8],
    pub prefix: [u8; 155],
    pub pad: [u8; 12],
}

/// Representation of the header of an entry in an archive
#[repr(C)]
#[allow(missing_docs)]
pub struct GnuHeader {
    pub name: [u8; 100],
    pub mode: [u8; 8],
    pub uid: [u8; 8],
    pub gid: [u8; 8],
    pub size: [u8; 12],
    pub mtime: [u8; 12],
    pub cksum: [u8; 8],
    pub typeflag: [u8; 1],
    pub linkname: [u8; 100],

    // GNU format
    pub magic: [u8; 6],
    pub version: [u8; 2],
    pub uname: [u8; 32],
    pub gname: [u8; 32],
    pub dev_major: [u8; 8],
    pub dev_minor: [u8; 8],
    pub atime: [u8; 12],
    pub ctime: [u8; 12],
    pub offset: [u8; 12],
    pub longnames: [u8; 4],
    pub unused: [u8; 1],
    pub sparse: [GnuSparseHeader; 4],
    pub isextended: [u8; 1],
    pub realsize: [u8; 12],
    pub pad: [u8; 17],
}

/// Description of the header of a spare entry.
///
/// Specifies the offset/number of bytes of a chunk of data in octal.
#[repr(C)]
#[allow(missing_docs)]
pub struct GnuSparseHeader {
    pub offset: [u8; 12],
    pub numbytes: [u8; 12],
}

/// Representation of the entry found to represent extended GNU sparse files.
///
/// When a `GnuHeader` has the `isextended` flag set to `1` then the contents of
/// the next entry will be one of these headers.
#[repr(C)]
#[allow(missing_docs)]
#[derive(Debug)]
pub struct GnuExtSparseHeader {
    pub sparse: [GnuSparseHeader; 21],
    pub isextended: [u8; 1],
    pub padding: [u8; 7],
}

impl Header {
    /// Creates a new blank GNU header.
    ///
    /// The GNU style header is the default for this library and allows various
    /// extensions such as long path names, long link names, and setting the
    /// atime/ctime metadata attributes of files.
    pub fn new_gnu() -> Header {
        let mut header = Header { bytes: [0; 512] };
        unsafe {
            let gnu = cast_mut::<_, GnuHeader>(&mut header);
            gnu.magic = *b"ustar ";
            gnu.version = *b" \0";
        }
        header.set_mtime(0);
        header
    }

    /// Creates a new blank UStar header.
    ///
    /// The UStar style header is an extension of the original archive header
    /// which enables some extra metadata along with storing a longer (but not
    /// too long) path name.
    ///
    /// UStar is also the basis used for pax archives.
    pub fn new_ustar() -> Header {
        let mut header = Header { bytes: [0; 512] };
        unsafe {
            let gnu = cast_mut::<_, UstarHeader>(&mut header);
            gnu.magic = *b"ustar\0";
            gnu.version = *b"00";
        }
        header.set_mtime(0);
        header
    }

    /// Creates a new blank old header.
    ///
    /// This header format is the original archive header format which all other
    /// versions are compatible with (e.g. they are a superset). This header
    /// format limits the path name limit and isn't able to contain extra
    /// metadata like atime/ctime.
    pub fn new_old() -> Header {
        let mut header = Header { bytes: [0; 512] };
        header.set_mtime(0);
        header
    }

    fn is_ustar(&self) -> bool {
        let ustar = unsafe { cast::<_, UstarHeader>(self) };
        ustar.magic[..] == b"ustar\0"[..] && ustar.version[..] == b"00"[..]
    }

    fn is_gnu(&self) -> bool {
        let ustar = unsafe { cast::<_, UstarHeader>(self) };
        ustar.magic[..] == b"ustar "[..] && ustar.version[..] == b" \0"[..]
    }

    /// View this archive header as a raw "old" archive header.
    ///
    /// This view will always succeed as all archive header formats will fill
    /// out at least the fields specified in the old header format.
    pub fn as_old(&self) -> &OldHeader {
        unsafe { cast(self) }
    }

    /// Same as `as_old`, but the mutable version.
    pub fn as_old_mut(&mut self) -> &mut OldHeader {
        unsafe { cast_mut(self) }
    }

    /// View this archive header as a raw UStar archive header.
    ///
    /// The UStar format is an extension to the tar archive format which enables
    /// longer pathnames and a few extra attributes such as the group and user
    /// name.
    ///
    /// This cast may not succeed as this function will test whether the
    /// magic/version fields of the UStar format have the appropriate values,
    /// returning `None` if they aren't correct.
    pub fn as_ustar(&self) -> Option<&UstarHeader> {
        if self.is_ustar() {
            Some(unsafe { cast(self) })
        } else {
            None
        }
    }

    /// Same as `as_ustar_mut`, but the mutable version.
    pub fn as_ustar_mut(&mut self) -> Option<&mut UstarHeader> {
        if self.is_ustar() {
            Some(unsafe { cast_mut(self) })
        } else {
            None
        }
    }

    /// View this archive header as a raw GNU archive header.
    ///
    /// The GNU format is an extension to the tar archive format which enables
    /// longer pathnames and a few extra attributes such as the group and user
    /// name.
    ///
    /// This cast may not succeed as this function will test whether the
    /// magic/version fields of the GNU format have the appropriate values,
    /// returning `None` if they aren't correct.
    pub fn as_gnu(&self) -> Option<&GnuHeader> {
        if self.is_gnu() {
            Some(unsafe { cast(self) })
        } else {
            None
        }
    }

    /// Same as `as_gnu`, but the mutable version.
    pub fn as_gnu_mut(&mut self) -> Option<&mut GnuHeader> {
        if self.is_gnu() {
            Some(unsafe { cast_mut(self) })
        } else {
            None
        }
    }

    /// Treats the given byte slice as a header.
    ///
    /// Panics if the length of the passed slice is not equal to 512.
    pub fn from_byte_slice(bytes: &[u8]) -> &Header {
        assert_eq!(bytes.len(), mem::size_of::<Header>());
        assert_eq!(mem::align_of_val(bytes), mem::align_of::<Header>());
        unsafe { &*(bytes.as_ptr() as *const Header) }
    }

    /// Returns a view into this header as a byte array.
    pub fn as_bytes(&self) -> &[u8; 512] {
        &self.bytes
    }

    /// Returns a view into this header as a byte array.
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 512] {
        &mut self.bytes
    }

    /// Blanket sets the metadata in this header from the metadata argument
    /// provided.
    ///
    /// This is useful for initializing a `Header` from the OS's metadata from a
    /// file. By default, this will use `HeaderMode::Complete` to include all
    /// metadata.
    pub fn set_metadata(&mut self, meta: &Metadata) {
        self.fill_from(meta, HeaderMode::Complete);
    }

    /// Sets only the metadata relevant to the given HeaderMode in this header
    /// from the metadata argument provided.
    pub fn set_metadata_in_mode(&mut self, meta: &Metadata, mode: HeaderMode) {
        self.fill_from(meta, mode);
    }

    /// Returns the size of entry's data this header represents.
    ///
    /// This is different from `Header::size` for sparse files, which have
    /// some longer `size()` but shorter `entry_size()`. The `entry_size()`
    /// listed here should be the number of bytes in the archive this header
    /// describes.
    ///
    /// May return an error if the field is corrupted.
    pub fn entry_size(&self) -> io::Result<u64> {
        num_field_wrapper_from(&self.as_old().size).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting size for {}", err, self.path_lossy()),
            )
        })
    }

    /// Returns the file size this header represents.
    ///
    /// May return an error if the field is corrupted.
    pub fn size(&self) -> io::Result<u64> {
        if self.entry_type().is_gnu_sparse() {
            self.as_gnu()
                .ok_or_else(|| other("sparse header was not a gnu header"))
                .and_then(GnuHeader::real_size)
        } else {
            self.entry_size()
        }
    }

    /// Encodes the `size` argument into the size field of this header.
    pub fn set_size(&mut self, size: u64) {
        num_field_wrapper_into(&mut self.as_old_mut().size, size);
    }

    /// Returns the raw path name stored in this header.
    ///
    /// This method may fail if the pathname is not valid Unicode and this is
    /// called on a Windows platform.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators.
    pub fn path(&self) -> io::Result<Cow<'_, Path>> {
        bytes2path(self.path_bytes())
    }

    /// Returns the pathname stored in this header as a byte array.
    ///
    /// This function is guaranteed to succeed, but you may wish to call the
    /// `path` method to convert to a `Path`.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators.
    pub fn path_bytes(&self) -> Cow<'_, [u8]> {
        if let Some(ustar) = self.as_ustar() {
            ustar.path_bytes()
        } else {
            let name = truncate(&self.as_old().name);
            Cow::Borrowed(name)
        }
    }

    /// Gets the path in a "lossy" way, used for error reporting ONLY.
    fn path_lossy(&self) -> String {
        String::from_utf8_lossy(&self.path_bytes()).to_string()
    }

    /// Sets the path name for this header.
    ///
    /// This function will set the pathname listed in this header, encoding it
    /// in the appropriate format. May fail if the path is too long or if the
    /// path specified is not Unicode and this is a Windows platform.
    pub fn set_path<P: AsRef<Path>>(&mut self, p: P) -> io::Result<()> {
        self.set_path_inner(p.as_ref(), false)
    }

    // Sets the truncated path for GNU header
    //
    // Same as `set_path`` but skips some validations.
    pub(crate) fn set_truncated_path_for_gnu_header<P: AsRef<Path>>(
        &mut self,
        p: P,
    ) -> io::Result<()> {
        self.set_path_inner(p.as_ref(), true)
    }

    fn set_path_inner(&mut self, path: &Path, is_truncated_gnu_long_path: bool) -> io::Result<()> {
        if let Some(ustar) = self.as_ustar_mut() {
            return ustar.set_path(path);
        }
        if is_truncated_gnu_long_path {
            copy_path_into_gnu_long(&mut self.as_old_mut().name, path, false)
        } else {
            copy_path_into(&mut self.as_old_mut().name, path, false)
        }
        .map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when setting path for {}", err, self.path_lossy()),
            )
        })
    }

    /// Returns the link name stored in this header, if any is found.
    ///
    /// This method may fail if the pathname is not valid Unicode and this is
    /// called on a Windows platform. `Ok(None)` being returned, however,
    /// indicates that the link name was not present.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators.
    pub fn link_name(&self) -> io::Result<Option<Cow<'_, Path>>> {
        match self.link_name_bytes() {
            Some(bytes) => bytes2path(bytes).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the link name stored in this header as a byte array, if any.
    ///
    /// This function is guaranteed to succeed, but you may wish to call the
    /// `link_name` method to convert to a `Path`.
    ///
    /// Note that this function will convert any `\` characters to directory
    /// separators.
    pub fn link_name_bytes(&self) -> Option<Cow<'_, [u8]>> {
        let old = self.as_old();
        if old.linkname[0] == 0 {
            None
        } else {
            Some(Cow::Borrowed(truncate(&old.linkname)))
        }
    }

    /// Sets the link name for this header.
    ///
    /// This function will set the linkname listed in this header, encoding it
    /// in the appropriate format. May fail if the link name is too long or if
    /// the path specified is not Unicode and this is a Windows platform.
    pub fn set_link_name<P: AsRef<Path>>(&mut self, p: P) -> io::Result<()> {
        self._set_link_name(p.as_ref())
    }

    fn _set_link_name(&mut self, path: &Path) -> io::Result<()> {
        copy_path_into(&mut self.as_old_mut().linkname, path, true).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when setting link name for {}", err, self.path_lossy()),
            )
        })
    }

    /// Returns the mode bits for this file
    ///
    /// May return an error if the field is corrupted.
    pub fn mode(&self) -> io::Result<u32> {
        octal_from(&self.as_old().mode)
            .map(|u| u as u32)
            .map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!("{} when getting mode for {}", err, self.path_lossy()),
                )
            })
    }

    /// Encodes the `mode` provided into this header.
    pub fn set_mode(&mut self, mode: u32) {
        octal_into(&mut self.as_old_mut().mode, mode);
    }

    /// Returns the value of the owner's user ID field
    ///
    /// May return an error if the field is corrupted.
    pub fn uid(&self) -> io::Result<u64> {
        num_field_wrapper_from(&self.as_old().uid).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting uid for {}", err, self.path_lossy()),
            )
        })
    }

    /// Encodes the `uid` provided into this header.
    pub fn set_uid(&mut self, uid: u64) {
        num_field_wrapper_into(&mut self.as_old_mut().uid, uid);
    }

    /// Returns the value of the group's user ID field
    pub fn gid(&self) -> io::Result<u64> {
        num_field_wrapper_from(&self.as_old().gid).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting gid for {}", err, self.path_lossy()),
            )
        })
    }

    /// Encodes the `gid` provided into this header.
    pub fn set_gid(&mut self, gid: u64) {
        num_field_wrapper_into(&mut self.as_old_mut().gid, gid);
    }

    /// Returns the last modification time in Unix time format
    pub fn mtime(&self) -> io::Result<u64> {
        num_field_wrapper_from(&self.as_old().mtime).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting mtime for {}", err, self.path_lossy()),
            )
        })
    }

    /// Encodes the `mtime` provided into this header.
    ///
    /// Note that this time is typically a number of seconds passed since
    /// January 1, 1970.
    pub fn set_mtime(&mut self, mtime: u64) {
        num_field_wrapper_into(&mut self.as_old_mut().mtime, mtime);
    }

    /// Return the user name of the owner of this file.
    ///
    /// A return value of `Ok(Some(..))` indicates that the user name was
    /// present and was valid utf-8, `Ok(None)` indicates that the user name is
    /// not present in this archive format, and `Err` indicates that the user
    /// name was present but was not valid utf-8.
    pub fn username(&self) -> Result<Option<&str>, str::Utf8Error> {
        match self.username_bytes() {
            Some(bytes) => str::from_utf8(bytes).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the user name of the owner of this file, if present.
    ///
    /// A return value of `None` indicates that the user name is not present in
    /// this header format.
    pub fn username_bytes(&self) -> Option<&[u8]> {
        if let Some(ustar) = self.as_ustar() {
            Some(ustar.username_bytes())
        } else if let Some(gnu) = self.as_gnu() {
            Some(gnu.username_bytes())
        } else {
            None
        }
    }

    /// Sets the username inside this header.
    ///
    /// This function will return an error if this header format cannot encode a
    /// user name or the name is too long.
    pub fn set_username(&mut self, name: &str) -> io::Result<()> {
        if let Some(ustar) = self.as_ustar_mut() {
            return ustar.set_username(name);
        }
        if let Some(gnu) = self.as_gnu_mut() {
            gnu.set_username(name)
        } else {
            Err(other("not a ustar or gnu archive, cannot set username"))
        }
    }

    /// Return the group name of the owner of this file.
    ///
    /// A return value of `Ok(Some(..))` indicates that the group name was
    /// present and was valid utf-8, `Ok(None)` indicates that the group name is
    /// not present in this archive format, and `Err` indicates that the group
    /// name was present but was not valid utf-8.
    pub fn groupname(&self) -> Result<Option<&str>, str::Utf8Error> {
        match self.groupname_bytes() {
            Some(bytes) => str::from_utf8(bytes).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the group name of the owner of this file, if present.
    ///
    /// A return value of `None` indicates that the group name is not present in
    /// this header format.
    pub fn groupname_bytes(&self) -> Option<&[u8]> {
        if let Some(ustar) = self.as_ustar() {
            Some(ustar.groupname_bytes())
        } else if let Some(gnu) = self.as_gnu() {
            Some(gnu.groupname_bytes())
        } else {
            None
        }
    }

    /// Sets the group name inside this header.
    ///
    /// This function will return an error if this header format cannot encode a
    /// group name or the name is too long.
    pub fn set_groupname(&mut self, name: &str) -> io::Result<()> {
        if let Some(ustar) = self.as_ustar_mut() {
            return ustar.set_groupname(name);
        }
        if let Some(gnu) = self.as_gnu_mut() {
            gnu.set_groupname(name)
        } else {
            Err(other("not a ustar or gnu archive, cannot set groupname"))
        }
    }

    /// Returns the device major number, if present.
    ///
    /// This field may not be present in all archives, and it may not be
    /// correctly formed in all archives. `Ok(Some(..))` means it was present
    /// and correctly decoded, `Ok(None)` indicates that this header format does
    /// not include the device major number, and `Err` indicates that it was
    /// present and failed to decode.
    pub fn device_major(&self) -> io::Result<Option<u32>> {
        if let Some(ustar) = self.as_ustar() {
            ustar.device_major().map(Some)
        } else if let Some(gnu) = self.as_gnu() {
            gnu.device_major().map(Some)
        } else {
            Ok(None)
        }
    }

    /// Encodes the value `major` into the dev_major field of this header.
    ///
    /// This function will return an error if this header format cannot encode a
    /// major device number.
    pub fn set_device_major(&mut self, major: u32) -> io::Result<()> {
        if let Some(ustar) = self.as_ustar_mut() {
            ustar.set_device_major(major);
            Ok(())
        } else if let Some(gnu) = self.as_gnu_mut() {
            gnu.set_device_major(major);
            Ok(())
        } else {
            Err(other("not a ustar or gnu archive, cannot set dev_major"))
        }
    }

    /// Returns the device minor number, if present.
    ///
    /// This field may not be present in all archives, and it may not be
    /// correctly formed in all archives. `Ok(Some(..))` means it was present
    /// and correctly decoded, `Ok(None)` indicates that this header format does
    /// not include the device minor number, and `Err` indicates that it was
    /// present and failed to decode.
    pub fn device_minor(&self) -> io::Result<Option<u32>> {
        if let Some(ustar) = self.as_ustar() {
            ustar.device_minor().map(Some)
        } else if let Some(gnu) = self.as_gnu() {
            gnu.device_minor().map(Some)
        } else {
            Ok(None)
        }
    }

    /// Encodes the value `minor` into the dev_minor field of this header.
    ///
    /// This function will return an error if this header format cannot encode a
    /// minor device number.
    pub fn set_device_minor(&mut self, minor: u32) -> io::Result<()> {
        if let Some(ustar) = self.as_ustar_mut() {
            ustar.set_device_minor(minor);
            Ok(())
        } else if let Some(gnu) = self.as_gnu_mut() {
            gnu.set_device_minor(minor);
            Ok(())
        } else {
            Err(other("not a ustar or gnu archive, cannot set dev_minor"))
        }
    }

    /// Returns the type of file described by this header.
    pub fn entry_type(&self) -> EntryType {
        EntryType::new(self.as_old().linkflag[0])
    }

    /// Sets the type of file that will be described by this header.
    pub fn set_entry_type(&mut self, ty: EntryType) {
        self.as_old_mut().linkflag = [ty.as_byte()];
    }

    /// Returns the checksum field of this header.
    ///
    /// May return an error if the field is corrupted.
    pub fn cksum(&self) -> io::Result<u32> {
        octal_from(&self.as_old().cksum)
            .map(|u| u as u32)
            .map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!("{} when getting cksum for {}", err, self.path_lossy()),
                )
            })
    }

    /// Sets the checksum field of this header based on the current fields in
    /// this header.
    pub fn set_cksum(&mut self) {
        let cksum = self.calculate_cksum();
        octal_into(&mut self.as_old_mut().cksum, cksum);
    }

    fn calculate_cksum(&self) -> u32 {
        let old = self.as_old();
        let start = old as *const _ as usize;
        let cksum_start = old.cksum.as_ptr() as *const _ as usize;
        let offset = cksum_start - start;
        let len = old.cksum.len();
        self.bytes[0..offset]
            .iter()
            .chain(iter::repeat_n(&b' ', len))
            .chain(&self.bytes[offset + len..])
            .fold(0, |a, b| a + (*b as u32))
    }

    fn fill_from(&mut self, meta: &Metadata, mode: HeaderMode) {
        self.fill_platform_from(meta, mode);
        // Set size of directories to zero
        self.set_size(if meta.is_dir() || meta.file_type().is_symlink() {
            0
        } else {
            meta.len()
        });
        if let Some(ustar) = self.as_ustar_mut() {
            ustar.set_device_major(0);
            ustar.set_device_minor(0);
        }
        if let Some(gnu) = self.as_gnu_mut() {
            gnu.set_device_major(0);
            gnu.set_device_minor(0);
        }
    }

    #[cfg(target_arch = "wasm32")]
    #[allow(unused_variables)]
    fn fill_platform_from(&mut self, meta: &fs::Metadata, mode: HeaderMode) {
        unimplemented!();
    }

    #[cfg(any(unix, target_os = "redox"))]
    fn fill_platform_from(&mut self, meta: &Metadata, mode: HeaderMode) {
        match mode {
            HeaderMode::Complete => {
                self.set_mtime(meta.mtime() as u64);
                self.set_uid(meta.uid() as u64);
                self.set_gid(meta.gid() as u64);
                self.set_mode(meta.mode());
            }
            HeaderMode::Deterministic => {
                self.set_mtime(0);
                self.set_uid(0);
                self.set_gid(0);

                // Use a default umask value, but propagate the (user) execute bit.
                let fs_mode = if meta.is_dir() || (0o100 & meta.mode() == 0o100) {
                    0o755
                } else {
                    0o644
                };
                self.set_mode(fs_mode);
            }
        }

        // Note that if we are a GNU header we *could* set atime/ctime, except
        // the `tar` utility doesn't do that by default and it causes problems
        // with 7-zip [1].
        //
        // It's always possible to fill them out manually, so we just don't fill
        // it out automatically here.
        //
        // [1]: https://github.com/alexcrichton/tar-rs/issues/70

        // TODO: need to bind more file types
        self.set_entry_type(entry_type(meta.mode()));

        #[cfg(not(target_os = "redox"))]
        fn entry_type(mode: u32) -> EntryType {
            match mode as libc::mode_t & libc::S_IFMT {
                libc::S_IFREG => EntryType::file(),
                libc::S_IFLNK => EntryType::symlink(),
                libc::S_IFCHR => EntryType::character_special(),
                libc::S_IFBLK => EntryType::block_special(),
                libc::S_IFDIR => EntryType::dir(),
                libc::S_IFIFO => EntryType::fifo(),
                _ => EntryType::new(b' '),
            }
        }

        #[cfg(target_os = "redox")]
        fn entry_type(mode: u32) -> EntryType {
            use syscall;
            match mode as u16 & syscall::MODE_TYPE {
                syscall::MODE_FILE => EntryType::file(),
                syscall::MODE_SYMLINK => EntryType::symlink(),
                syscall::MODE_DIR => EntryType::dir(),
                _ => EntryType::new(b' '),
            }
        }
    }

    #[cfg(windows)]
    fn fill_platform_from(&mut self, meta: &Metadata, mode: HeaderMode) {
        // There's no concept of a file mode on Windows, so do a best approximation here.
        match mode {
            HeaderMode::Complete => {
                self.set_uid(0);
                self.set_gid(0);
                // The dates listed in tarballs are always seconds relative to
                // January 1, 1970. On Windows, however, the timestamps are returned as
                // dates relative to January 1, 1601 (in 100ns intervals), so we need to
                // add in some offset for those dates.
                let mtime = (meta.last_write_time() / (1_000_000_000 / 100)) - 11644473600;
                self.set_mtime(mtime);
                let fs_mode = {
                    const FILE_ATTRIBUTE_READONLY: u32 = 0x00000001;
                    let readonly = meta.file_attributes() & FILE_ATTRIBUTE_READONLY;
                    match (meta.is_dir(), readonly != 0) {
                        (true, false) => 0o755,
                        (true, true) => 0o555,
                        (false, false) => 0o644,
                        (false, true) => 0o444,
                    }
                };
                self.set_mode(fs_mode);
            }
            HeaderMode::Deterministic => {
                self.set_uid(0);
                self.set_gid(0);
                self.set_mtime(0);
                let fs_mode = if meta.is_dir() { 0o755 } else { 0o644 };
                self.set_mode(fs_mode);
            }
        }

        let ft = meta.file_type();
        self.set_entry_type(if ft.is_dir() {
            EntryType::dir()
        } else if ft.is_file() {
            EntryType::file()
        } else if ft.is_symlink() {
            EntryType::symlink()
        } else {
            EntryType::new(b' ')
        });
    }

    fn debug_fields(&self, b: &mut fmt::DebugStruct) {
        if let Ok(entry_size) = self.entry_size() {
            b.field("entry_size", &entry_size);
        }
        if let Ok(size) = self.size() {
            b.field("size", &size);
        }
        if let Ok(path) = self.path() {
            b.field("path", &path);
        }
        if let Ok(link_name) = self.link_name() {
            b.field("link_name", &link_name);
        }
        if let Ok(mode) = self.mode() {
            b.field("mode", &DebugAsOctal(mode));
        }
        if let Ok(uid) = self.uid() {
            b.field("uid", &uid);
        }
        if let Ok(gid) = self.gid() {
            b.field("gid", &gid);
        }
        if let Ok(mtime) = self.mtime() {
            b.field("mtime", &mtime);
        }
        if let Ok(username) = self.username() {
            b.field("username", &username);
        }
        if let Ok(groupname) = self.groupname() {
            b.field("groupname", &groupname);
        }
        if let Ok(device_major) = self.device_major() {
            b.field("device_major", &device_major);
        }
        if let Ok(device_minor) = self.device_minor() {
            b.field("device_minor", &device_minor);
        }
        if let Ok(cksum) = self.cksum() {
            b.field("cksum", &cksum);
            b.field("cksum_valid", &(cksum == self.calculate_cksum()));
        }
    }
}

struct DebugAsOctal<T>(T);

impl<T: fmt::Octal> fmt::Debug for DebugAsOctal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

unsafe fn cast<T, U>(a: &T) -> &U {
    assert_eq!(mem::size_of_val(a), mem::size_of::<U>());
    assert_eq!(mem::align_of_val(a), mem::align_of::<U>());
    unsafe { &*(a as *const T as *const U) }
}

unsafe fn cast_mut<T, U>(a: &mut T) -> &mut U {
    assert_eq!(mem::size_of_val(a), mem::size_of::<U>());
    assert_eq!(mem::align_of_val(a), mem::align_of::<U>());
    unsafe { &mut *(a as *mut T as *mut U) }
}

impl Clone for Header {
    fn clone(&self) -> Header {
        Header { bytes: self.bytes }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(me) = self.as_ustar() {
            me.fmt(f)
        } else if let Some(me) = self.as_gnu() {
            me.fmt(f)
        } else {
            self.as_old().fmt(f)
        }
    }
}

impl OldHeader {
    /// Views this as a normal `Header`
    pub fn as_header(&self) -> &Header {
        unsafe { cast(self) }
    }

    /// Views this as a normal `Header`
    pub fn as_header_mut(&mut self) -> &mut Header {
        unsafe { cast_mut(self) }
    }
}

impl fmt::Debug for OldHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("OldHeader");
        self.as_header().debug_fields(&mut f);
        f.finish()
    }
}

impl UstarHeader {
    /// See `Header::path_bytes`
    pub fn path_bytes(&self) -> Cow<'_, [u8]> {
        if self.prefix[0] == 0 && !self.name.contains(&b'\\') {
            Cow::Borrowed(truncate(&self.name))
        } else {
            let mut bytes = Vec::new();
            let prefix = truncate(&self.prefix);
            if !prefix.is_empty() {
                bytes.extend_from_slice(prefix);
                bytes.push(b'/');
            }
            bytes.extend_from_slice(truncate(&self.name));
            Cow::Owned(bytes)
        }
    }

    /// Gets the path in a "lossy" way, used for error reporting ONLY.
    fn path_lossy(&self) -> String {
        String::from_utf8_lossy(&self.path_bytes()).to_string()
    }

    /// See `Header::set_path`
    pub fn set_path<P: AsRef<Path>>(&mut self, p: P) -> io::Result<()> {
        self._set_path(p.as_ref())
    }

    fn _set_path(&mut self, path: &Path) -> io::Result<()> {
        // This can probably be optimized quite a bit more, but for now just do
        // something that's relatively easy and readable.
        //
        // First up, if the path fits within `self.name` then we just shove it
        // in there. If not then we try to split it between some existing path
        // components where it can fit in name/prefix. To do that we peel off
        // enough until the path fits in `prefix`, then we try to put both
        // halves into their destination.
        let bytes = path2bytes(path)?;
        let (maxnamelen, maxprefixlen) = (self.name.len(), self.prefix.len());
        if bytes.len() <= maxnamelen {
            copy_path_into(&mut self.name, path, false).map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!("{} when setting path for {}", err, self.path_lossy()),
                )
            })?;
        } else {
            let mut prefix = path;
            let mut prefixlen;
            loop {
                match prefix.parent() {
                    Some(parent) => prefix = parent,
                    None => {
                        return Err(other(&format!(
                            "path cannot be split to be inserted into archive: {}",
                            path.display()
                        )));
                    }
                }
                prefixlen = path2bytes(prefix)?.len();
                if prefixlen <= maxprefixlen {
                    break;
                }
            }
            copy_path_into(&mut self.prefix, prefix, false).map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!("{} when setting path for {}", err, self.path_lossy()),
                )
            })?;
            let path = bytes2path(Cow::Borrowed(&bytes[prefixlen + 1..]))?;
            copy_path_into(&mut self.name, &path, false).map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!("{} when setting path for {}", err, self.path_lossy()),
                )
            })?;
        }
        Ok(())
    }

    /// See `Header::username_bytes`
    pub fn username_bytes(&self) -> &[u8] {
        truncate(&self.uname)
    }

    /// See `Header::set_username`
    pub fn set_username(&mut self, name: &str) -> io::Result<()> {
        copy_into(&mut self.uname, name.as_bytes()).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when setting username for {}", err, self.path_lossy()),
            )
        })
    }

    /// See `Header::groupname_bytes`
    pub fn groupname_bytes(&self) -> &[u8] {
        truncate(&self.gname)
    }

    /// See `Header::set_groupname`
    pub fn set_groupname(&mut self, name: &str) -> io::Result<()> {
        copy_into(&mut self.gname, name.as_bytes()).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when setting groupname for {}", err, self.path_lossy()),
            )
        })
    }

    /// See `Header::device_major`
    pub fn device_major(&self) -> io::Result<u32> {
        octal_from(&self.dev_major)
            .map(|u| u as u32)
            .map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!(
                        "{} when getting device_major for {}",
                        err,
                        self.path_lossy()
                    ),
                )
            })
    }

    /// See `Header::set_device_major`
    pub fn set_device_major(&mut self, major: u32) {
        octal_into(&mut self.dev_major, major);
    }

    /// See `Header::device_minor`
    pub fn device_minor(&self) -> io::Result<u32> {
        octal_from(&self.dev_minor)
            .map(|u| u as u32)
            .map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!(
                        "{} when getting device_minor for {}",
                        err,
                        self.path_lossy()
                    ),
                )
            })
    }

    /// See `Header::set_device_minor`
    pub fn set_device_minor(&mut self, minor: u32) {
        octal_into(&mut self.dev_minor, minor);
    }

    /// Views this as a normal `Header`
    pub fn as_header(&self) -> &Header {
        unsafe { cast(self) }
    }

    /// Views this as a normal `Header`
    pub fn as_header_mut(&mut self) -> &mut Header {
        unsafe { cast_mut(self) }
    }
}

impl fmt::Debug for UstarHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("UstarHeader");
        self.as_header().debug_fields(&mut f);
        f.finish()
    }
}

impl GnuHeader {
    /// See `Header::username_bytes`
    pub fn username_bytes(&self) -> &[u8] {
        truncate(&self.uname)
    }

    /// Gets the fullname (group:user) in a "lossy" way, used for error reporting ONLY.
    fn fullname_lossy(&self) -> String {
        format!(
            "{}:{}",
            String::from_utf8_lossy(self.groupname_bytes()),
            String::from_utf8_lossy(self.username_bytes()),
        )
    }

    /// See `Header::set_username`
    pub fn set_username(&mut self, name: &str) -> io::Result<()> {
        copy_into(&mut self.uname, name.as_bytes()).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!(
                    "{} when setting username for {}",
                    err,
                    self.fullname_lossy()
                ),
            )
        })
    }

    /// See `Header::groupname_bytes`
    pub fn groupname_bytes(&self) -> &[u8] {
        truncate(&self.gname)
    }

    /// See `Header::set_groupname`
    pub fn set_groupname(&mut self, name: &str) -> io::Result<()> {
        copy_into(&mut self.gname, name.as_bytes()).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!(
                    "{} when setting groupname for {}",
                    err,
                    self.fullname_lossy()
                ),
            )
        })
    }

    /// See `Header::device_major`
    pub fn device_major(&self) -> io::Result<u32> {
        octal_from(&self.dev_major)
            .map(|u| u as u32)
            .map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!(
                        "{} when getting device_major for {}",
                        err,
                        self.fullname_lossy()
                    ),
                )
            })
    }

    /// See `Header::set_device_major`
    pub fn set_device_major(&mut self, major: u32) {
        octal_into(&mut self.dev_major, major);
    }

    /// See `Header::device_minor`
    pub fn device_minor(&self) -> io::Result<u32> {
        octal_from(&self.dev_minor)
            .map(|u| u as u32)
            .map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!(
                        "{} when getting device_minor for {}",
                        err,
                        self.fullname_lossy()
                    ),
                )
            })
    }

    /// See `Header::set_device_minor`
    pub fn set_device_minor(&mut self, minor: u32) {
        octal_into(&mut self.dev_minor, minor);
    }

    /// Returns the last modification time in Unix time format
    pub fn atime(&self) -> io::Result<u64> {
        num_field_wrapper_from(&self.atime).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting atime for {}", err, self.fullname_lossy()),
            )
        })
    }

    /// Encodes the `atime` provided into this header.
    ///
    /// Note that this time is typically a number of seconds passed since
    /// January 1, 1970.
    pub fn set_atime(&mut self, atime: u64) {
        num_field_wrapper_into(&mut self.atime, atime);
    }

    /// Returns the last modification time in Unix time format
    pub fn ctime(&self) -> io::Result<u64> {
        num_field_wrapper_from(&self.ctime).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting ctime for {}", err, self.fullname_lossy()),
            )
        })
    }

    /// Encodes the `ctime` provided into this header.
    ///
    /// Note that this time is typically a number of seconds passed since
    /// January 1, 1970.
    pub fn set_ctime(&mut self, ctime: u64) {
        num_field_wrapper_into(&mut self.ctime, ctime);
    }

    /// Returns the "real size" of the file this header represents.
    ///
    /// This is applicable for sparse files where the returned size here is the
    /// size of the entire file after the sparse regions have been filled in.
    pub fn real_size(&self) -> io::Result<u64> {
        octal_from(&self.realsize).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!(
                    "{} when getting real_size for {}",
                    err,
                    self.fullname_lossy()
                ),
            )
        })
    }

    /// Indicates whether this header will be followed by additional
    /// sparse-header records.
    ///
    /// Note that this is handled internally by this library, and is likely only
    /// interesting if a `raw` iterator is being used.
    pub fn is_extended(&self) -> bool {
        self.isextended[0] == 1
    }

    /// Views this as a normal `Header`
    pub fn as_header(&self) -> &Header {
        unsafe { cast(self) }
    }

    /// Views this as a normal `Header`
    pub fn as_header_mut(&mut self) -> &mut Header {
        unsafe { cast_mut(self) }
    }
}

impl fmt::Debug for GnuHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("GnuHeader");
        self.as_header().debug_fields(&mut f);
        if let Ok(atime) = self.atime() {
            f.field("atime", &atime);
        }
        if let Ok(ctime) = self.ctime() {
            f.field("ctime", &ctime);
        }
        f.field("is_extended", &self.is_extended())
            .field("sparse", &DebugSparseHeaders(&self.sparse))
            .finish()
    }
}

struct DebugSparseHeaders<'a>(&'a [GnuSparseHeader]);

impl<'a> fmt::Debug for DebugSparseHeaders<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_list();
        for header in self.0 {
            if !header.is_empty() {
                f.entry(header);
            }
        }
        f.finish()
    }
}

impl GnuSparseHeader {
    /// Returns true if block is empty
    pub fn is_empty(&self) -> bool {
        self.offset[0] == 0 || self.numbytes[0] == 0
    }

    /// Offset of the block from the start of the file
    ///
    /// Returns `Err` for a malformed `offset` field.
    pub fn offset(&self) -> io::Result<u64> {
        octal_from(&self.offset).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting offset from sparse header", err),
            )
        })
    }

    /// Length of the block
    ///
    /// Returns `Err` for a malformed `numbytes` field.
    pub fn length(&self) -> io::Result<u64> {
        octal_from(&self.numbytes).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when getting length from sparse header", err),
            )
        })
    }
}

impl fmt::Debug for GnuSparseHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("GnuSparseHeader");
        if let Ok(offset) = self.offset() {
            f.field("offset", &offset);
        }
        if let Ok(length) = self.length() {
            f.field("length", &length);
        }
        f.finish()
    }
}

impl GnuExtSparseHeader {
    /// Crates a new zero'd out sparse header entry.
    pub fn new() -> GnuExtSparseHeader {
        unsafe { mem::zeroed() }
    }

    /// Returns a view into this header as a byte array.
    pub fn as_bytes(&self) -> &[u8; 512] {
        debug_assert_eq!(mem::size_of_val(self), 512);
        unsafe { &*(self as *const GnuExtSparseHeader as *const [u8; 512]) }
    }

    /// Returns a view into this header as a byte array.
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 512] {
        debug_assert_eq!(mem::size_of_val(self), 512);
        unsafe { &mut *(self as *mut GnuExtSparseHeader as *mut [u8; 512]) }
    }

    /// Returns a slice of the underlying sparse headers.
    ///
    /// Some headers may represent empty chunks of both the offset and numbytes
    /// fields are 0.
    pub fn sparse(&self) -> &[GnuSparseHeader; 21] {
        &self.sparse
    }

    /// Indicates if another sparse header should be following this one.
    pub fn is_extended(&self) -> bool {
        self.isextended[0] == 1
    }
}

impl Default for GnuExtSparseHeader {
    fn default() -> Self {
        Self::new()
    }
}

fn octal_from(slice: &[u8]) -> io::Result<u64> {
    let trun = truncate(slice);
    let num = match str::from_utf8(trun) {
        Ok(n) => n,
        Err(_) => {
            return Err(other(&format!(
                "numeric field did not have utf-8 text: {}",
                String::from_utf8_lossy(trun)
            )));
        }
    };
    match u64::from_str_radix(num.trim(), 8) {
        Ok(n) => Ok(n),
        Err(_) => Err(other(&format!("numeric field was not a number: {}", num))),
    }
}

fn octal_into<T: fmt::Octal>(dst: &mut [u8], val: T) {
    let o = format!("{:o}", val);
    let value = o.bytes().rev().chain(repeat(b'0'));
    for (slot, value) in dst.iter_mut().rev().skip(1).zip(value) {
        *slot = value;
    }
}

// Wrapper to figure out if we should fill the header field using tar's numeric
// extension (binary) or not (octal).
fn num_field_wrapper_into(dst: &mut [u8], src: u64) {
    if src >= 8_589_934_592 || (src >= 2_097_152 && dst.len() == 8) {
        numeric_extended_into(dst, src);
    } else {
        octal_into(dst, src);
    }
}

// Wrapper to figure out if we should read the header field in binary (numeric
// extension) or octal (standard encoding).
fn num_field_wrapper_from(src: &[u8]) -> io::Result<u64> {
    if src[0] & 0x80 == 0 {
        octal_from(src)
    } else {
        Ok(numeric_extended_from(src))
    }
}

// When writing numeric fields with is the extended form, the high bit of the
// first byte is set to 1 and the remainder of the field is treated as binary
// instead of octal ascii.
// This handles writing u64 to 8 (uid, gid) or 12 (size, *time) bytes array.
fn numeric_extended_into(dst: &mut [u8], src: u64) {
    let len: usize = dst.len();
    for (slot, val) in dst.iter_mut().zip(
        repeat_n(0, len - 8) // to zero init extra bytes
            .chain((0..8).rev().map(|x| ((src >> (8 * x)) & 0xff) as u8)),
    ) {
        *slot = val;
    }
    dst[0] |= 0x80;
}

fn numeric_extended_from(src: &[u8]) -> u64 {
    let mut dst: u64 = 0;
    let mut b_to_skip = 1;
    if src.len() == 8 {
        // read first byte without extension flag bit
        dst = (src[0] ^ 0x80) as u64;
    } else {
        // only read last 8 bytes
        b_to_skip = src.len() - 8;
    }
    for byte in src.iter().skip(b_to_skip) {
        dst <<= 8;
        dst |= *byte as u64;
    }
    dst
}

fn truncate(slice: &[u8]) -> &[u8] {
    match slice.iter().position(|i| *i == 0) {
        Some(i) => &slice[..i],
        None => slice,
    }
}

/// Copies `bytes` into the `slot` provided, returning an error if the `bytes`
/// array is too long or if it contains any nul bytes.
fn copy_into(slot: &mut [u8], bytes: &[u8]) -> io::Result<()> {
    if bytes.len() > slot.len() {
        Err(other("provided value is too long"))
    } else if bytes.contains(&0) {
        Err(other("provided value contains a nul byte"))
    } else {
        for (slot, val) in slot.iter_mut().zip(bytes.iter().chain(Some(&0))) {
            *slot = *val;
        }
        Ok(())
    }
}

fn copy_path_into_inner(
    mut slot: &mut [u8],
    path: &Path,
    is_link_name: bool,
    is_truncated_gnu_long_path: bool,
) -> io::Result<()> {
    let mut emitted = false;
    let mut needs_slash = false;
    let mut iter = path.components().peekable();
    while let Some(component) = iter.next() {
        let bytes = path2bytes(Path::new(component.as_os_str()))?;
        match (component, is_link_name) {
            (Component::Prefix(..), false) | (Component::RootDir, false) => {
                return Err(other("paths in archives must be relative"));
            }
            (Component::ParentDir, false) => {
                if is_truncated_gnu_long_path && iter.peek().is_none() {
                    // If it's last component of a gnu long path we know that there might be more
                    // to the component than .. (the rest is stored elsewhere)
                    {}
                } else {
                    return Err(other("paths in archives must not have `..`"));
                }
            }
            // Allow "./" as the path
            (Component::CurDir, false) if path.components().count() == 1 => {}
            (Component::CurDir, false) => continue,
            (Component::Normal(_), _) | (_, true) => {}
        };
        if needs_slash {
            copy(&mut slot, b"/")?;
        }
        if bytes.contains(&b'/') {
            if let Component::Normal(..) = component {
                return Err(other("path component in archive cannot contain `/`"));
            }
        }
        copy(&mut slot, &bytes)?;
        if &*bytes != b"/" {
            needs_slash = true;
        }
        emitted = true;
    }
    if !emitted {
        return Err(other("paths in archives must have at least one component"));
    }
    if ends_with_slash(path) {
        copy(&mut slot, b"/")?;
    }
    return Ok(());

    fn copy(slot: &mut &mut [u8], bytes: &[u8]) -> io::Result<()> {
        copy_into(slot, bytes)?;
        let tmp = std::mem::take(slot);
        *slot = &mut tmp[bytes.len()..];
        Ok(())
    }
}

/// Copies `path` into the `slot` provided
///
/// Returns an error if:
///
/// * the path is too long to fit
/// * a nul byte was found
/// * an invalid path component is encountered (e.g. a root path or parent dir)
/// * the path itself is empty
fn copy_path_into(slot: &mut [u8], path: &Path, is_link_name: bool) -> io::Result<()> {
    copy_path_into_inner(slot, path, is_link_name, false)
}

/// Copies `path` into the `slot` provided
///
/// Returns an error if:
///
/// * the path is too long to fit
/// * a nul byte was found
/// * an invalid path component is encountered (e.g. a root path or parent dir)
/// * the path itself is empty
///
/// This is less restrictive version meant to be used for truncated GNU paths.
fn copy_path_into_gnu_long(slot: &mut [u8], path: &Path, is_link_name: bool) -> io::Result<()> {
    copy_path_into_inner(slot, path, is_link_name, true)
}

#[cfg(target_arch = "wasm32")]
fn ends_with_slash(p: &Path) -> bool {
    p.to_string_lossy().ends_with('/')
}

#[cfg(windows)]
fn ends_with_slash(p: &Path) -> bool {
    let last = p.as_os_str().encode_wide().last();
    last == Some(b'/' as u16) || last == Some(b'\\' as u16)
}

#[cfg(any(unix, target_os = "redox"))]
fn ends_with_slash(p: &Path) -> bool {
    p.as_os_str().as_bytes().ends_with(b"/")
}

#[cfg(any(windows, target_arch = "wasm32"))]
pub fn path2bytes(p: &Path) -> io::Result<Cow<[u8]>> {
    p.as_os_str()
        .to_str()
        .map(|s| s.as_bytes())
        .ok_or_else(|| other(&format!("path {} was not valid Unicode", p.display())))
        .map(|bytes| {
            if bytes.contains(&b'\\') {
                // Normalize to Unix-style path separators
                let mut bytes = bytes.to_owned();
                for b in &mut bytes {
                    if *b == b'\\' {
                        *b = b'/';
                    }
                }
                Cow::Owned(bytes)
            } else {
                Cow::Borrowed(bytes)
            }
        })
}

#[cfg(any(unix, target_os = "redox"))]
/// On unix this will never fail
pub fn path2bytes(p: &Path) -> io::Result<Cow<'_, [u8]>> {
    Ok(Cow::Borrowed(p.as_os_str().as_bytes()))
}

#[cfg(windows)]
/// On windows we cannot accept non-Unicode bytes because it
/// is impossible to convert it to UTF-16.
pub fn bytes2path(bytes: Cow<[u8]>) -> io::Result<Cow<Path>> {
    return match bytes {
        Cow::Borrowed(bytes) => {
            let s = str::from_utf8(bytes).map_err(|_| not_unicode(bytes))?;
            Ok(Cow::Borrowed(Path::new(s)))
        }
        Cow::Owned(bytes) => {
            let s = String::from_utf8(bytes).map_err(|uerr| not_unicode(&uerr.into_bytes()))?;
            Ok(Cow::Owned(PathBuf::from(s)))
        }
    };

    fn not_unicode(v: &[u8]) -> io::Error {
        other(&format!(
            "only Unicode paths are supported on Windows: {}",
            String::from_utf8_lossy(v)
        ))
    }
}

#[cfg(any(unix, target_os = "redox"))]
/// On unix this operation can never fail.
pub fn bytes2path(bytes: Cow<'_, [u8]>) -> io::Result<Cow<'_, Path>> {
    use std::ffi::{OsStr, OsString};

    Ok(match bytes {
        Cow::Borrowed(bytes) => Cow::Borrowed(Path::new(OsStr::from_bytes(bytes))),
        Cow::Owned(bytes) => Cow::Owned(PathBuf::from(OsString::from_vec(bytes))),
    })
}

#[cfg(target_arch = "wasm32")]
pub fn bytes2path(bytes: Cow<[u8]>) -> io::Result<Cow<Path>> {
    Ok(match bytes {
        Cow::Borrowed(bytes) => {
            Cow::Borrowed({ Path::new(str::from_utf8(bytes).map_err(invalid_utf8)?) })
        }
        Cow::Owned(bytes) => {
            Cow::Owned({ PathBuf::from(String::from_utf8(bytes).map_err(invalid_utf8)?) })
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn invalid_utf8<T>(_: T) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "Invalid utf-8")
}
