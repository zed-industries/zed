// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::ptr::{NonNull, null_mut};
use std::sync::OnceLock;

use libc::{c_void, devstat, devstat_getversion};

use super::ffi::{
    DEVSTAT_READ, DEVSTAT_WRITE, geom_stats_open, geom_stats_snapshot_free,
    geom_stats_snapshot_get, geom_stats_snapshot_next, geom_stats_snapshot_reset,
};
use super::utils::{c_buf_to_utf8_str, get_sys_value_str_by_name};
use crate::{Disk, DiskKind, DiskRefreshKind, DiskUsage};

#[derive(Debug)]
pub(crate) struct DiskInner {
    name: OsString,
    c_mount_point: Vec<libc::c_char>,
    dev_id: Option<String>,
    mount_point: PathBuf,
    total_space: u64,
    available_space: u64,
    file_system: OsString,
    is_removable: bool,
    is_read_only: bool,
    read_bytes: u64,
    old_read_bytes: u64,
    written_bytes: u64,
    old_written_bytes: u64,
    updated: bool,
}

impl DiskInner {
    pub(crate) fn kind(&self) -> DiskKind {
        // Currently don't know how to retrieve this information on FreeBSD.
        DiskKind::Unknown(-1)
    }

    pub(crate) fn name(&self) -> &OsStr {
        &self.name
    }

    pub(crate) fn file_system(&self) -> &OsStr {
        &self.file_system
    }

    pub(crate) fn mount_point(&self) -> &Path {
        &self.mount_point
    }

    pub(crate) fn total_space(&self) -> u64 {
        self.total_space
    }

    pub(crate) fn available_space(&self) -> u64 {
        self.available_space
    }

    pub(crate) fn is_removable(&self) -> bool {
        self.is_removable
    }

    pub(crate) fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub(crate) fn refresh_specifics(&mut self, refresh_kind: DiskRefreshKind) -> bool {
        refresh_disk(self, refresh_kind)
    }

    pub(crate) fn usage(&self) -> DiskUsage {
        DiskUsage {
            read_bytes: self.read_bytes.saturating_sub(self.old_read_bytes),
            total_read_bytes: self.read_bytes,
            written_bytes: self.written_bytes.saturating_sub(self.old_written_bytes),
            total_written_bytes: self.written_bytes,
        }
    }
}

impl crate::DisksInner {
    pub(crate) fn new() -> Self {
        Self {
            disks: Vec::with_capacity(2),
        }
    }

    pub(crate) fn refresh_specifics(
        &mut self,
        remove_not_listed_disks: bool,
        refresh_kind: DiskRefreshKind,
    ) {
        unsafe { get_all_list(&mut self.disks, remove_not_listed_disks, refresh_kind) }
    }

    pub(crate) fn list(&self) -> &[Disk] {
        &self.disks
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Disk] {
        &mut self.disks
    }
}

trait GetValues {
    fn update_old(&mut self);
    fn get_read(&mut self) -> &mut u64;
    fn get_written(&mut self) -> &mut u64;
    fn dev_id(&self) -> Option<&String>;
}

impl GetValues for crate::Disk {
    fn update_old(&mut self) {
        self.inner.update_old()
    }
    fn get_read(&mut self) -> &mut u64 {
        self.inner.get_read()
    }
    fn get_written(&mut self) -> &mut u64 {
        self.inner.get_written()
    }
    fn dev_id(&self) -> Option<&String> {
        self.inner.dev_id()
    }
}

impl GetValues for &mut DiskInner {
    fn update_old(&mut self) {
        self.old_read_bytes = self.read_bytes;
        self.old_written_bytes = self.written_bytes;
    }
    fn get_read(&mut self) -> &mut u64 {
        &mut self.read_bytes
    }
    fn get_written(&mut self) -> &mut u64 {
        &mut self.written_bytes
    }
    fn dev_id(&self) -> Option<&String> {
        self.dev_id.as_ref()
    }
}
impl GetValues for DiskInner {
    fn update_old(&mut self) {
        self.old_read_bytes = self.read_bytes;
        self.old_written_bytes = self.written_bytes;
    }
    fn get_read(&mut self) -> &mut u64 {
        &mut self.read_bytes
    }
    fn get_written(&mut self) -> &mut u64 {
        &mut self.written_bytes
    }
    fn dev_id(&self) -> Option<&String> {
        self.dev_id.as_ref()
    }
}

/// Returns `(total_space, available_space, is_read_only)`.
unsafe fn get_statvfs(
    c_mount_point: &[libc::c_char],
    vfs: &mut libc::statvfs,
) -> Option<(u64, u64, bool)> {
    if unsafe { libc::statvfs(c_mount_point.as_ptr() as *const _, vfs as *mut _) < 0 } {
        sysinfo_debug!("statvfs failed");
        None
    } else {
        let block_size: u64 = vfs.f_frsize as _;
        Some((
            vfs.f_blocks.saturating_mul(block_size),
            vfs.f_favail.saturating_mul(block_size),
            (vfs.f_flag & libc::ST_RDONLY) != 0,
        ))
    }
}

fn refresh_disk(disk: &mut DiskInner, refresh_kind: DiskRefreshKind) -> bool {
    if refresh_kind.storage() {
        unsafe {
            let mut vfs: libc::statvfs = std::mem::zeroed();
            if let Some((total_space, available_space, is_read_only)) =
                get_statvfs(&disk.c_mount_point, &mut vfs)
            {
                disk.total_space = total_space;
                disk.available_space = available_space;
                disk.is_read_only = is_read_only;
            }
        }
    }

    if refresh_kind.io_usage() {
        unsafe {
            refresh_disk_io(&mut [disk]);
        }
    }

    true
}

unsafe fn initialize_geom() -> Result<(), ()> {
    let version = unsafe { devstat_getversion(null_mut()) };
    if version != 6 {
        // For now we only handle the devstat 6 version.
        sysinfo_debug!("version {version} of devstat is not supported");
        return Err(());
    }
    let r = unsafe { geom_stats_open() };
    if r != 0 {
        sysinfo_debug!("`geom_stats_open` failed: {r}");
        Err(())
    } else {
        Ok(())
    }
}

unsafe fn refresh_disk_io<T: GetValues>(disks: &mut [T]) {
    static GEOM_STATS: OnceLock<Result<(), ()>> = OnceLock::new();

    if GEOM_STATS
        .get_or_init(|| unsafe { initialize_geom() })
        .is_err()
    {
        return;
    }
    let snap = unsafe { GeomSnapshot::new() };
    let Some(mut snap) = snap else {
        return;
    };
    for device in snap.iter() {
        let device = unsafe { device.devstat.as_ref() };
        let Some(device_name) = c_buf_to_utf8_str(&device.device_name) else {
            continue;
        };
        let dev_stat_name = format!("{device_name}{}", device.unit_number);

        for disk in disks
            .iter_mut()
            .filter(|d| d.dev_id().is_some_and(|id| *id == dev_stat_name))
        {
            disk.update_old();
            *disk.get_read() = device.bytes[DEVSTAT_READ];
            *disk.get_written() = device.bytes[DEVSTAT_WRITE];
        }
    }

    // thread_local! {
    //     static DEV_INFO: RefCell<DevInfoWrapper> = RefCell::new(DevInfoWrapper::new());
    // }

    // DEV_INFO.with_borrow_mut(|dev_info| {
    //     let Some(stat_info) = dev_info.get_devs() else { return };
    //     let dinfo = (*stat_info).dinfo;

    //     let numdevs = (*dinfo).numdevs;
    //     if numdevs < 0 {
    //         return;
    //     }
    //     let devices: &mut [devstat] = std::slice::from_raw_parts_mut((*dinfo).devices, numdevs as _);
    //     for device in devices {
    //         let Some(device_name) = c_buf_to_utf8_str(&device.device_name) else { continue };
    //         let dev_stat_name = format!("{device_name}{}", device.unit_number);

    //         for disk in disks.iter_mut().filter(|d| d.dev_id().is_some_and(|id| *id == dev_stat_name)) {
    //             disk.update_old();
    //             let mut read = 0u64;
    //             // This code cannot work because `devstat_compute_statistics` expects a
    //             // `long double` as 3rd argument, making it impossible for rust to call it...
    //             devstat_compute_statistics(
    //                 device,
    //                 null_mut(),
    //                 0,
    //                 DSM_TOTAL_BYTES_READ,
    //                 &mut read,
    //                 DSM_TOTAL_BYTES_WRITE,
    //                 disk.get_written(),
    //                 DSM_NONE,
    //             );
    //             *disk.get_read() = read;
    //         }
    //     }
    // });
}

fn get_disks_mapping() -> HashMap<String, String> {
    let mut disk_mapping = HashMap::new();
    let Some(mapping) = get_sys_value_str_by_name(b"kern.geom.conftxt\0") else {
        return disk_mapping;
    };

    let mut last_id = String::new();

    for line in mapping.lines() {
        let mut parts = line.split_whitespace();
        let Some(kind) = parts.next() else { continue };

        #[allow(clippy::collapsible_if)]
        if kind == "0" {
            if let Some("DISK") = parts.next()
                && let Some(id) = parts.next()
            {
                last_id.clear();
                last_id.push_str(id);
            }
        } else if kind == "2" && !last_id.is_empty() {
            if let Some("LABEL") = parts.next()
                && let Some(path) = parts.next()
            {
                disk_mapping.insert(format!("/dev/{path}"), last_id.clone());
            }
        }
    }
    disk_mapping
}

pub unsafe fn get_all_list(
    container: &mut Vec<Disk>,
    remove_not_listed_disks: bool,
    refresh_kind: DiskRefreshKind,
) {
    let mut fs_infos: *mut libc::statfs = null_mut();

    let count = unsafe { libc::getmntinfo(&mut fs_infos, libc::MNT_WAIT) };

    if count < 1 {
        return;
    }
    let disk_mapping = get_disks_mapping();

    let fs_infos: &[libc::statfs] =
        unsafe { std::slice::from_raw_parts(fs_infos as _, count as _) };

    for fs_info in fs_infos {
        if fs_info.f_mntfromname[0] == 0 || fs_info.f_mntonname[0] == 0 {
            // If we have missing information, no need to look any further...
            continue;
        }
        let fs_type: Vec<u8> = {
            let len = fs_info
                .f_fstypename
                .iter()
                .position(|x| *x == 0)
                .unwrap_or(fs_info.f_fstypename.len());
            fs_info.f_fstypename[..len]
                .iter()
                .map(|c| *c as u8)
                .collect()
        };
        match &fs_type[..] {
            b"autofs" | b"devfs" | b"linprocfs" | b"procfs" | b"fdesckfs" | b"tmpfs"
            | b"linsysfs" => {
                sysinfo_debug!(
                    "Memory filesystem `{:?}`, ignoring it.",
                    c_buf_to_utf8_str(&fs_info.f_fstypename).unwrap(),
                );
                continue;
            }
            _ => {}
        }

        let mount_point = match c_buf_to_utf8_str(&fs_info.f_mntonname) {
            Some(m) => m,
            None => {
                sysinfo_debug!("Cannot get disk mount point, ignoring it.");
                continue;
            }
        };

        if mount_point == "/boot/efi" {
            continue;
        }
        let name = if mount_point == "/" {
            OsString::from("root")
        } else {
            OsString::from(mount_point)
        };

        if let Some(disk) = container.iter_mut().find(|d| {
            d.inner.name == name
                && d.inner
                    .file_system
                    .as_encoded_bytes()
                    .iter()
                    .zip(fs_type.iter())
                    .all(|(a, b)| a == b)
        }) {
            // I/O usage is updated for all disks at once at the end.
            refresh_disk(&mut disk.inner, refresh_kind.without_io_usage());
            disk.inner.updated = true;
        } else {
            let dev_mount_point = c_buf_to_utf8_str(&fs_info.f_mntfromname).unwrap_or("");

            // USB keys and CDs are removable.
            let is_removable = if refresh_kind.storage() {
                [b"USB", b"usb"].iter().any(|b| *b == &fs_type[..])
                    || fs_type.starts_with(b"/dev/cd")
            } else {
                false
            };

            let mut disk = DiskInner {
                name,
                c_mount_point: fs_info.f_mntonname.to_vec(),
                mount_point: PathBuf::from(mount_point),
                dev_id: disk_mapping.get(dev_mount_point).map(ToString::to_string),
                total_space: 0,
                available_space: 0,
                file_system: OsString::from_vec(fs_type),
                is_removable,
                is_read_only: false,
                read_bytes: 0,
                old_read_bytes: 0,
                written_bytes: 0,
                old_written_bytes: 0,
                updated: true,
            };
            // I/O usage is updated for all disks at once at the end.
            refresh_disk(&mut disk, refresh_kind.without_io_usage());
            container.push(Disk { inner: disk });
        }
    }

    if remove_not_listed_disks {
        container.retain_mut(|disk| {
            if !disk.inner.updated {
                return false;
            }
            disk.inner.updated = false;
            true
        });
    } else {
        for c in container.iter_mut() {
            c.inner.updated = false;
        }
    }
    if refresh_kind.io_usage() {
        unsafe {
            refresh_disk_io(container.as_mut_slice());
        }
    }
}

// struct DevInfoWrapper {
//     info: statinfo,
// }

// impl DevInfoWrapper {
//     fn new() -> Self {
//         Self {
//             info: unsafe { std::mem::zeroed() },
//         }
//     }

//     unsafe fn get_devs(&mut self) -> Option<&statinfo> {
//         let version = devstat_getversion(null_mut());
//         if version != 6 {
//             // For now we only handle the devstat 6 version.
//             sysinfo_debug!("version {version} of devstat is not supported");
//             return None;
//         }
//         if self.info.dinfo.is_null() {
//             self.info.dinfo = libc::calloc(1, std::mem::size_of::<devinfo>()) as *mut _;
//             if self.info.dinfo.is_null() {
//                 return None;
//             }
//         }
//         if devstat_getdevs(null_mut(), &mut self.info as *mut _) != -1 {
//             Some(&self.info)
//         } else {
//             None
//         }
//     }
// }

// impl Drop for DevInfoWrapper {
//     fn drop(&mut self) {
//         if !self.info.dinfo.is_null() {
//             unsafe { libc::free(self.info.dinfo as *mut _); }
//         }
//     }
// }

// Most of this code was adapted from `gstat-rs` (https://github.com/asomers/gstat-rs).
struct GeomSnapshot(NonNull<c_void>);

impl GeomSnapshot {
    unsafe fn new() -> Option<Self> {
        match NonNull::new(unsafe { geom_stats_snapshot_get() }) {
            Some(n) => Some(Self(n)),
            None => {
                sysinfo_debug!("geom_stats_snapshot_get failed");
                None
            }
        }
    }

    fn iter(&mut self) -> GeomSnapshotIter<'_> {
        GeomSnapshotIter(self)
    }

    fn reset(&mut self) {
        unsafe { geom_stats_snapshot_reset(self.0.as_mut()) }
    }
}

impl Drop for GeomSnapshot {
    fn drop(&mut self) {
        unsafe { geom_stats_snapshot_free(self.0.as_mut()) };
    }
}

#[repr(transparent)]
struct Devstat<'a> {
    devstat: NonNull<devstat>,
    phantom: PhantomData<&'a devstat>,
}

struct GeomSnapshotIter<'a>(&'a mut GeomSnapshot);

impl<'a> Iterator for GeomSnapshotIter<'a> {
    type Item = Devstat<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = unsafe { geom_stats_snapshot_next(self.0.0.as_mut()) };
        NonNull::new(raw).map(|devstat| Devstat {
            devstat,
            phantom: PhantomData,
        })
    }
}

impl Drop for GeomSnapshotIter<'_> {
    fn drop(&mut self) {
        self.0.reset();
    }
}
