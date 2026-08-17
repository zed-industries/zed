use crate::sealing;
use rustix::fs::{MemfdFlags, SealFlags};
use std::fs;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

/// A `Memfd` builder, providing advanced options and flags for specifying its behavior.
#[derive(Clone, Debug)]
pub struct MemfdOptions {
    allow_sealing: bool,
    cloexec: bool,
    hugetlb: Option<HugetlbSize>,
}

impl MemfdOptions {
    /// Default set of options for `Memfd` creation.
    ///
    /// The default options are:
    ///  * [`FileSeal::SealSeal`] (i.e. no further sealing);
    ///  * close-on-exec is enabled;
    ///  * hugetlb is disabled.
    ///
    /// [`FileSeal::SealSeal`]: sealing::FileSeal::SealSeal
    pub const fn new() -> Self {
        Self {
            allow_sealing: false,
            cloexec: true,
            hugetlb: None,
        }
    }

    /// Whether to allow adding seals to the created `Memfd`.
    pub const fn allow_sealing(mut self, value: bool) -> Self {
        self.allow_sealing = value;
        self
    }

    /// Whether to set the `FD_CLOEXEC` flag on the created `Memfd`.
    pub const fn close_on_exec(mut self, value: bool) -> Self {
        self.cloexec = value;
        self
    }

    /// Optional hugetlb support and page size for the created `Memfd`.
    pub const fn hugetlb(mut self, size: Option<HugetlbSize>) -> Self {
        self.hugetlb = size;
        self
    }

    /// Translate the current options into a bitflags value for `memfd_create`.
    fn bitflags(&self) -> MemfdFlags {
        let mut bits = MemfdFlags::empty();
        if self.allow_sealing {
            bits |= MemfdFlags::ALLOW_SEALING;
        }
        if self.cloexec {
            bits |= MemfdFlags::CLOEXEC;
        }
        if let Some(ref hugetlb) = self.hugetlb {
            bits |= hugetlb.bitflags();
            bits |= MemfdFlags::HUGETLB;
        }
        bits
    }

    /// Create a [`Memfd`] according to configuration.
    ///
    /// [`Memfd`]: Memfd
    pub fn create<T: AsRef<str>>(&self, name: T) -> Result<Memfd, crate::Error> {
        let flags = self.bitflags();
        let fd = rustix::fs::memfd_create(name.as_ref(), flags)
            .map_err(Into::into)
            .map_err(crate::Error::Create)?;
        Ok(Memfd { file: fd.into() })
    }
}

impl Default for MemfdOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Page size for a hugetlb anonymous file.
#[derive(Copy, Clone, Debug)]
pub enum HugetlbSize {
    /// 64KB hugetlb page.
    Huge64KB,
    /// 64KB hugetlb page.
    Huge512KB,
    /// 1MB hugetlb page.
    Huge1MB,
    /// 2MB hugetlb page.
    Huge2MB,
    /// 8MB hugetlb page.
    Huge8MB,
    /// 16MB hugetlb page.
    Huge16MB,
    /// 256MB hugetlb page.
    Huge256MB,
    /// 1GB hugetlb page.
    Huge1GB,
    /// 2GB hugetlb page.
    Huge2GB,
    /// 16GB hugetlb page.
    Huge16GB,
}

impl HugetlbSize {
    const fn bitflags(self) -> MemfdFlags {
        match self {
            Self::Huge64KB => MemfdFlags::HUGE_64KB,
            Self::Huge512KB => MemfdFlags::HUGE_512KB,
            Self::Huge1MB => MemfdFlags::HUGE_1MB,
            Self::Huge2MB => MemfdFlags::HUGE_2MB,
            Self::Huge8MB => MemfdFlags::HUGE_8MB,
            Self::Huge16MB => MemfdFlags::HUGE_16MB,
            Self::Huge256MB => MemfdFlags::HUGE_256MB,
            Self::Huge1GB => MemfdFlags::HUGE_1GB,
            Self::Huge2GB => MemfdFlags::HUGE_2GB,
            Self::Huge16GB => MemfdFlags::HUGE_16GB,
        }
    }
}

/// An anonymous volatile file, with sealing capabilities.
#[derive(Debug)]
pub struct Memfd {
    file: fs::File,
}

impl Memfd {
    /// Try to convert an object that owns a file descriptor into a `Memfd`.
    ///
    /// This function consumes the ownership of the specified object. If the underlying
    /// file-descriptor is compatible with memfd/sealing, a `Memfd` object is returned.
    /// Otherwise the supplied object is returned as error.
    pub fn try_from_fd<F>(fd: F) -> Result<Self, F>
    where
        F: AsRawFd + IntoRawFd,
    {
        if is_memfd(&fd) {
            // SAFETY: from_raw_fd requires a valid, uniquely owned file descriptor.
            // The IntoRawFd trait guarantees both conditions.
            let file = unsafe { fs::File::from_raw_fd(fd.into_raw_fd()) };
            Ok(Self { file })
        } else {
            Err(fd)
        }
    }

    /// Try to convert a [`File`] object into a `Memfd`.
    ///
    /// This function consumes the ownership of the specified `File`.  If the underlying
    /// file-descriptor is compatible with memfd/sealing, a `Memfd` object is returned.
    /// Otherwise the supplied `File` is returned for further usage.
    ///
    /// [`File`]: fs::File
    pub fn try_from_file(file: fs::File) -> Result<Self, fs::File> {
        Self::try_from_fd(file)
    }

    /// Return a reference to the backing [`File`].
    ///
    /// [`File`]: fs::File
    pub const fn as_file(&self) -> &fs::File {
        &self.file
    }

    /// Convert `Memfd` to the backing [`File`].
    ///
    /// [`File`]: fs::File
    pub fn into_file(self) -> fs::File {
        self.file
    }

    /// Obtain the current set of seals for the `Memfd`.
    pub fn seals(&self) -> Result<sealing::SealsHashSet, crate::Error> {
        let flags = Self::file_get_seals(&self.file)?;
        Ok(sealing::bitflags_to_seals(flags))
    }

    /// Add a seal to the existing set of seals.
    pub fn add_seal(&self, seal: sealing::FileSeal) -> Result<(), crate::Error> {
        let flags = seal.bitflags();
        self.add_seal_flags(flags)
    }

    /// Add some seals to the existing set of seals.
    pub fn add_seals<'a>(
        &self,
        seals: impl IntoIterator<Item = &'a sealing::FileSeal>,
    ) -> Result<(), crate::Error> {
        let flags = sealing::seals_to_bitflags(seals);
        self.add_seal_flags(flags)
    }

    fn add_seal_flags(&self, flags: rustix::fs::SealFlags) -> Result<(), crate::Error> {
        rustix::fs::fcntl_add_seals(&self.file, flags)
            .map_err(Into::into)
            .map_err(crate::Error::AddSeals)?;
        Ok(())
    }

    /// Return the current sealing bitflags.
    fn file_get_seals(fp: &fs::File) -> Result<SealFlags, crate::Error> {
        let r = rustix::fs::fcntl_get_seals(fp)
            .map_err(Into::into)
            .map_err(crate::Error::GetSeals)?;
        Ok(r)
    }
}

impl FromRawFd for Memfd {
    /// Convert a raw file-descriptor to a [`Memfd`].
    ///
    /// This function consumes ownership of the specified file descriptor. `Memfd` will take
    /// responsibility for closing it when the object goes out of scope.
    ///
    /// # Safety
    ///
    /// `fd` must be a valid file descriptor representing a memfd file.
    ///
    /// [`Memfd`]: Memfd
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        let file = fs::File::from_raw_fd(fd);
        Self { file }
    }
}

impl AsRawFd for Memfd {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl IntoRawFd for Memfd {
    fn into_raw_fd(self) -> RawFd {
        self.into_file().into_raw_fd()
    }
}

/// Check if a file descriptor is a memfd.
///
/// Implemented by trying to retrieve the seals.
/// If that fails, the fd is not a memfd.
fn is_memfd<F: AsRawFd>(fd: &F) -> bool {
    // SAFETY: For now, we trust the file descriptor returned by `as_raw_fd()`
    // is valid. Once `AsFd` is stabilized in std, we can use that instead of
    // `AsRawFd`, and eliminate this `unsafe` block.
    let fd = unsafe { rustix::fd::BorrowedFd::borrow_raw(fd.as_raw_fd()) };
    rustix::fs::fcntl_get_seals(fd).is_ok()
}
