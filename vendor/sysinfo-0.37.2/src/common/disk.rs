// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::OsStr;
use std::fmt;
use std::path::Path;

use crate::DiskUsage;
use crate::common::impl_get_set::impl_get_set;

/// Struct containing a disk information.
///
/// ```no_run
/// use sysinfo::Disks;
///
/// let disks = Disks::new_with_refreshed_list();
/// for disk in disks.list() {
///     println!("{:?}: {:?}", disk.name(), disk.kind());
/// }
/// ```
pub struct Disk {
    pub(crate) inner: crate::DiskInner,
}

impl Disk {
    /// Returns the kind of disk.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] {:?}", disk.name(), disk.kind());
    /// }
    /// ```
    pub fn kind(&self) -> DiskKind {
        self.inner.kind()
    }

    /// Returns the disk name.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("{:?}", disk.name());
    /// }
    /// ```
    pub fn name(&self) -> &OsStr {
        self.inner.name()
    }

    /// Returns the file system used on this disk (so for example: `EXT4`, `NTFS`, etc...).
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] {:?}", disk.name(), disk.file_system());
    /// }
    /// ```
    pub fn file_system(&self) -> &OsStr {
        self.inner.file_system()
    }

    /// Returns the mount point of the disk (`/` for example).
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] {:?}", disk.name(), disk.mount_point());
    /// }
    /// ```
    pub fn mount_point(&self) -> &Path {
        self.inner.mount_point()
    }

    /// Returns the total disk size, in bytes.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] {}B", disk.name(), disk.total_space());
    /// }
    /// ```
    pub fn total_space(&self) -> u64 {
        self.inner.total_space()
    }

    /// Returns the available disk size, in bytes.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] {}B", disk.name(), disk.available_space());
    /// }
    /// ```
    pub fn available_space(&self) -> u64 {
        self.inner.available_space()
    }

    /// Returns `true` if the disk is removable.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] {}", disk.name(), disk.is_removable());
    /// }
    /// ```
    pub fn is_removable(&self) -> bool {
        self.inner.is_removable()
    }

    /// Returns `true` if the disk is read-only.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] is read-only: {}", disk.name(), disk.is_read_only());
    /// }
    /// ```
    pub fn is_read_only(&self) -> bool {
        self.inner.is_read_only()
    }

    /// Updates the disk' information with everything loaded.
    ///
    /// Equivalent to <code>[Disk::refresh_specifics]\([DiskRefreshKind::everything]\())</code>.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let mut disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list_mut() {
    ///     disk.refresh();
    /// }
    /// ```
    pub fn refresh(&mut self) -> bool {
        self.refresh_specifics(DiskRefreshKind::everything())
    }

    /// Updates the disk's information corresponding to the given [`DiskRefreshKind`].
    ///
    /// ```no_run
    /// use sysinfo::{Disks, DiskRefreshKind};
    ///
    /// let mut disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list_mut() {
    ///     disk.refresh_specifics(DiskRefreshKind::nothing());
    /// }
    /// ```
    pub fn refresh_specifics(&mut self, refreshes: DiskRefreshKind) -> bool {
        self.inner.refresh_specifics(refreshes)
    }

    /// Returns number of bytes read and written by the disk
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("[{:?}] disk usage: {:?}", disk.name(), disk.usage());
    /// }
    /// ```
    pub fn usage(&self) -> DiskUsage {
        self.inner.usage()
    }
}

/// Disks interface.
///
/// ```no_run
/// use sysinfo::Disks;
///
/// let disks = Disks::new_with_refreshed_list();
/// for disk in disks.list() {
///     println!("{disk:?}");
/// }
/// ```
///
/// ⚠️ Note that tmpfs mounts are excluded by default under Linux.
/// To display tmpfs mount points, the `linux-tmpfs` feature must be enabled.
///
/// ⚠️ Note that network devices are excluded by default under Linux.
/// To display mount points using the CIFS and NFS protocols, the `linux-netdevs`
/// feature must be enabled. Note, however, that sysinfo may hang under certain
/// circumstances. For example, if a CIFS or NFS share has been mounted with
/// the _hard_ option, but the connection has an error, such as the share server has stopped.
pub struct Disks {
    inner: crate::DisksInner,
}

impl Default for Disks {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Disks> for Vec<Disk> {
    fn from(disks: Disks) -> Vec<Disk> {
        disks.inner.into_vec()
    }
}

impl From<Vec<Disk>> for Disks {
    fn from(disks: Vec<Disk>) -> Self {
        Self {
            inner: crate::DisksInner::from_vec(disks),
        }
    }
}

impl<'a> IntoIterator for &'a Disks {
    type Item = &'a Disk;
    type IntoIter = std::slice::Iter<'a, Disk>;

    fn into_iter(self) -> Self::IntoIter {
        self.list().iter()
    }
}

impl<'a> IntoIterator for &'a mut Disks {
    type Item = &'a mut Disk;
    type IntoIter = std::slice::IterMut<'a, Disk>;

    fn into_iter(self) -> Self::IntoIter {
        self.list_mut().iter_mut()
    }
}

impl Disks {
    /// Creates a new empty [`Disks`][crate::Disks] type.
    ///
    /// If you want it to be filled directly, take a look at [`Disks::new_with_refreshed_list`].
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let mut disks = Disks::new();
    /// disks.refresh(false);
    /// for disk in disks.list() {
    ///     println!("{disk:?}");
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            inner: crate::DisksInner::new(),
        }
    }

    /// Creates a new [`Disks`][crate::Disks] type with the disk list loaded.
    ///
    /// Equivalent to <code>[Disks::new_with_refreshed_list_specifics]\([DiskRefreshKind::everything]\())</code>.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let mut disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("{disk:?}");
    /// }
    /// ```
    pub fn new_with_refreshed_list() -> Self {
        Self::new_with_refreshed_list_specifics(DiskRefreshKind::everything())
    }

    /// Creates a new [`Disks`][crate::Disks] type with the disk list loaded
    /// and refreshed according to the given [`DiskRefreshKind`].
    ///
    /// ```no_run
    /// use sysinfo::{Disks, DiskRefreshKind};
    ///
    /// let mut disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing());
    /// for disk in disks.list() {
    ///     println!("{disk:?}");
    /// }
    /// ```
    pub fn new_with_refreshed_list_specifics(refreshes: DiskRefreshKind) -> Self {
        let mut disks = Self::new();
        disks.refresh_specifics(false, refreshes);
        disks
    }

    /// Returns the disks list.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list() {
    ///     println!("{disk:?}");
    /// }
    /// ```
    pub fn list(&self) -> &[Disk] {
        self.inner.list()
    }

    /// Returns the disks list.
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let mut disks = Disks::new_with_refreshed_list();
    /// for disk in disks.list_mut() {
    ///     disk.refresh();
    ///     println!("{disk:?}");
    /// }
    /// ```
    pub fn list_mut(&mut self) -> &mut [Disk] {
        self.inner.list_mut()
    }

    /// Refreshes the listed disks' information.
    ///
    /// Equivalent to <code>[Disks::refresh_specifics]\([DiskRefreshKind::everything]\())</code>.
    pub fn refresh(&mut self, remove_not_listed_disks: bool) {
        self.inner
            .refresh_specifics(remove_not_listed_disks, DiskRefreshKind::everything());
    }

    /// Refreshes the disks' information according to the given [`DiskRefreshKind`].
    ///
    /// ```no_run
    /// use sysinfo::Disks;
    ///
    /// let mut disks = Disks::new_with_refreshed_list();
    /// // We wait some time...?
    /// disks.refresh(true);
    /// ```
    pub fn refresh_specifics(&mut self, remove_not_listed_disks: bool, refreshes: DiskRefreshKind) {
        self.inner
            .refresh_specifics(remove_not_listed_disks, refreshes);
    }
}

impl std::ops::Deref for Disks {
    type Target = [Disk];

    fn deref(&self) -> &Self::Target {
        self.list()
    }
}

impl std::ops::DerefMut for Disks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.list_mut()
    }
}

/// Enum containing the different supported kinds of disks.
///
/// This type is returned by [`Disk::kind`](`crate::Disk::kind`).
///
/// ```no_run
/// use sysinfo::Disks;
///
/// let disks = Disks::new_with_refreshed_list();
/// for disk in disks.list() {
///     println!("{:?}: {:?}", disk.name(), disk.kind());
/// }
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum DiskKind {
    /// HDD type.
    HDD,
    /// SSD type.
    SSD,
    /// Unknown type.
    Unknown(isize),
}

impl fmt::Display for DiskKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            DiskKind::HDD => "HDD",
            DiskKind::SSD => "SSD",
            _ => "Unknown",
        })
    }
}

/// Used to determine what you want to refresh specifically on the [`Disk`] type.
///
/// * `kind` is about refreshing the [`Disk::kind`] information.
/// * `storage` is about refreshing the [`Disk::available_space`] and [`Disk::total_space`] information.
/// * `io_usage` is about refreshing the [`Disk::usage`] information.
///
/// ```no_run
/// use sysinfo::{Disks, DiskRefreshKind};
///
/// let mut disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::everything());
///
/// for disk in disks.list() {
///     assert!(disk.total_space() != 0);
/// }
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct DiskRefreshKind {
    kind: bool,
    storage: bool,
    io_usage: bool,
}

impl DiskRefreshKind {
    /// Creates a new `DiskRefreshKind` with every refresh set to false.
    ///
    /// ```
    /// use sysinfo::DiskRefreshKind;
    ///
    /// let r = DiskRefreshKind::nothing();
    ///
    /// assert_eq!(r.kind(), false);
    /// assert_eq!(r.storage(), false);
    /// assert_eq!(r.io_usage(), false);
    /// ```
    pub fn nothing() -> Self {
        Self::default()
    }

    /// Creates a new `DiskRefreshKind` with every refresh set to true.
    ///
    /// ```
    /// use sysinfo::DiskRefreshKind;
    ///
    /// let r = DiskRefreshKind::everything();
    ///
    /// assert_eq!(r.kind(), true);
    /// assert_eq!(r.storage(), true);
    /// assert_eq!(r.io_usage(), true);
    /// ```
    pub fn everything() -> Self {
        Self {
            kind: true,
            storage: true,
            io_usage: true,
        }
    }

    impl_get_set!(DiskRefreshKind, kind, with_kind, without_kind);
    impl_get_set!(DiskRefreshKind, storage, with_storage, without_storage);
    impl_get_set!(DiskRefreshKind, io_usage, with_io_usage, without_io_usage);
}

#[cfg(test)]
mod tests {
    /// This first doctest ensure that we can create a new `Disks`.
    ///
    /// ```
    /// let x = sysinfo::Disks::new();
    /// ```
    ///
    /// This second doctest ensures that `Disks` doesn't implement `Clone`.
    ///
    /// ```compile_fail
    /// let x = sysinfo::Disks::new();
    /// x.clone();
    /// ```
    #[test]
    fn check_if_disks_is_send() {
        fn is_send<T: Send>(_: &T) {}

        let disks = crate::Disks::new();
        is_send(&disks);
    }
}
