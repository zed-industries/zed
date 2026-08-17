// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::{get_all_utf8_data, to_cpath};
use crate::{Disk, DiskKind, DiskRefreshKind, DiskUsage};

use libc::statvfs;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Copied from [`psutil`]:
///
/// "man iostat" states that sectors are equivalent with blocks and have
/// a size of 512 bytes. Despite this value can be queried at runtime
/// via /sys/block/{DISK}/queue/hw_sector_size and results may vary
/// between 1k, 2k, or 4k... 512 appears to be a magic constant used
/// throughout Linux source code:
/// * <https://stackoverflow.com/a/38136179/376587>
/// * <https://lists.gt.net/linux/kernel/2241060>
/// * <https://github.com/giampaolo/psutil/issues/1305>
/// * <https://github.com/torvalds/linux/blob/4f671fe2f9523a1ea206f63fe60a7c7b3a56d5c7/include/linux/bio.h#L99>
/// * <https://lkml.org/lkml/2015/8/17/234>
///
/// [`psutil`]: <https://github.com/giampaolo/psutil/blob/master/psutil/_pslinux.py#L103>
const SECTOR_SIZE: u64 = 512;

macro_rules! cast {
    ($x:expr) => {
        u64::from($x)
    };
}

pub(crate) struct DiskInner {
    type_: DiskKind,
    device_name: OsString,
    actual_device_name: Option<String>,
    file_system: OsString,
    mount_point: PathBuf,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
    is_read_only: bool,
    old_written_bytes: u64,
    old_read_bytes: u64,
    written_bytes: u64,
    read_bytes: u64,
    updated: bool,
}

impl DiskInner {
    pub(crate) fn kind(&self) -> DiskKind {
        self.type_
    }

    pub(crate) fn name(&self) -> &OsStr {
        &self.device_name
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
        self.efficient_refresh(refresh_kind, &disk_stats(&refresh_kind), false)
    }

    fn efficient_refresh(
        &mut self,
        refresh_kind: DiskRefreshKind,
        procfs_disk_stats: &HashMap<String, DiskStat>,
        first: bool,
    ) -> bool {
        if refresh_kind.io_usage() {
            if self.actual_device_name.is_none() {
                self.actual_device_name = Some(get_actual_device_name(&self.device_name));
            }
            if let Some(stat) = self
                .actual_device_name
                .as_ref()
                .and_then(|actual_device_name| procfs_disk_stats.get(actual_device_name))
            {
                self.old_read_bytes = self.read_bytes;
                self.old_written_bytes = self.written_bytes;
                self.read_bytes = stat.sectors_read * SECTOR_SIZE;
                self.written_bytes = stat.sectors_written * SECTOR_SIZE;
            } else {
                sysinfo_debug!("Failed to update disk i/o stats");
            }
        }

        if refresh_kind.kind() && self.type_ == DiskKind::Unknown(-1) {
            self.type_ = find_type_for_device_name(&self.device_name);
        }

        if refresh_kind.storage()
            && let Some((total_space, available_space, is_read_only)) =
                unsafe { load_statvfs_values(&self.mount_point) }
        {
            self.total_space = total_space;
            self.available_space = available_space;
            if first {
                self.is_read_only = is_read_only;
            }
        }

        true
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
        get_all_list(
            &mut self.disks,
            &get_all_utf8_data("/proc/mounts", 16_385).unwrap_or_default(),
            refresh_kind,
        );

        if remove_not_listed_disks {
            self.disks.retain_mut(|disk| {
                if !disk.inner.updated {
                    return false;
                }
                disk.inner.updated = false;
                true
            });
        } else {
            for c in self.disks.iter_mut() {
                c.inner.updated = false;
            }
        }
    }

    pub(crate) fn list(&self) -> &[Disk] {
        &self.disks
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Disk] {
        &mut self.disks
    }
}

/// Resolves the actual device name for a specified `device` from `/proc/mounts`
///
/// This function is inspired by the [`bottom`] crate implementation and essentially does the following:
///     1. Canonicalizes the specified device path to its absolute form
///     2. Strips the "/dev" prefix from the canonicalized path
///
/// [`bottom`]: <https://github.com/ClementTsang/bottom/blob/main/src/data_collection/disks/unix/linux/partition.rs#L44>
fn get_actual_device_name(device: &OsStr) -> String {
    let device_path = PathBuf::from(device);

    std::fs::canonicalize(&device_path)
        .ok()
        .and_then(|path| path.strip_prefix("/dev").ok().map(Path::to_path_buf))
        .unwrap_or(device_path)
        .to_str()
        .map(str::to_owned)
        .unwrap_or_default()
}

unsafe fn load_statvfs_values(mount_point: &Path) -> Option<(u64, u64, bool)> {
    let mount_point_cpath = to_cpath(mount_point);
    let mut stat: MaybeUninit<statvfs> = MaybeUninit::uninit();
    if unsafe {
        retry_eintr!(statvfs(
            mount_point_cpath.as_ptr() as *const _,
            stat.as_mut_ptr()
        ))
    } == 0
    {
        let stat = unsafe { stat.assume_init() };

        let bsize = cast!(stat.f_bsize);
        let blocks = cast!(stat.f_blocks);
        let bavail = cast!(stat.f_bavail);
        let total = bsize.saturating_mul(blocks);
        if total == 0 {
            return None;
        }
        let available = bsize.saturating_mul(bavail);
        let is_read_only = (stat.f_flag & libc::ST_RDONLY) != 0;

        Some((total, available, is_read_only))
    } else {
        None
    }
}

fn new_disk(
    device_name: &OsStr,
    mount_point: &Path,
    file_system: &OsStr,
    removable_entries: &[PathBuf],
    procfs_disk_stats: &HashMap<String, DiskStat>,
    refresh_kind: DiskRefreshKind,
) -> Disk {
    let is_removable = removable_entries
        .iter()
        .any(|e| e.as_os_str() == device_name);

    let mut disk = Disk {
        inner: DiskInner {
            type_: DiskKind::Unknown(-1),
            device_name: device_name.to_owned(),
            actual_device_name: None,
            file_system: file_system.to_owned(),
            mount_point: mount_point.to_owned(),
            total_space: 0,
            available_space: 0,
            is_removable,
            is_read_only: false,
            old_read_bytes: 0,
            old_written_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
            updated: true,
        },
    };
    disk.inner
        .efficient_refresh(refresh_kind, procfs_disk_stats, true);
    disk
}

#[allow(clippy::manual_range_contains)]
fn find_type_for_device_name(device_name: &OsStr) -> DiskKind {
    // The format of devices are as follows:
    //  - device_name is symbolic link in the case of /dev/mapper/
    //     and /dev/root, and the target is corresponding device under
    //     /sys/block/
    //  - In the case of /dev/sd, the format is /dev/sd[a-z][1-9],
    //     corresponding to /sys/block/sd[a-z]
    //  - In the case of /dev/nvme, the format is /dev/nvme[0-9]n[0-9]p[0-9],
    //     corresponding to /sys/block/nvme[0-9]n[0-9]
    //  - In the case of /dev/mmcblk, the format is /dev/mmcblk[0-9]p[0-9],
    //     corresponding to /sys/block/mmcblk[0-9]
    let device_name_path = device_name.to_str().unwrap_or_default();
    let real_path = fs::canonicalize(device_name).unwrap_or_else(|_| PathBuf::from(device_name));
    let mut real_path = real_path.to_str().unwrap_or_default();
    if device_name_path.starts_with("/dev/mapper/") {
        // Recursively solve, for example /dev/dm-0
        if real_path != device_name_path {
            return find_type_for_device_name(OsStr::new(&real_path));
        }
    } else if device_name_path.starts_with("/dev/sd") || device_name_path.starts_with("/dev/vd") {
        // Turn "sda1" into "sda" or "vda1" into "vda"
        real_path = real_path.trim_start_matches("/dev/");
        real_path = real_path.trim_end_matches(|c| c >= '0' && c <= '9');
    } else if device_name_path.starts_with("/dev/nvme") {
        // Turn "nvme0n1p1" into "nvme0n1"
        real_path = match real_path.find('p') {
            Some(idx) => &real_path["/dev/".len()..idx],
            None => &real_path["/dev/".len()..],
        };
    } else if device_name_path.starts_with("/dev/root") {
        // Recursively solve, for example /dev/mmcblk0p1
        if real_path != device_name_path {
            return find_type_for_device_name(OsStr::new(&real_path));
        }
    } else if device_name_path.starts_with("/dev/mmcblk") {
        // Turn "mmcblk0p1" into "mmcblk0"
        real_path = match real_path.find('p') {
            Some(idx) => &real_path["/dev/".len()..idx],
            None => &real_path["/dev/".len()..],
        };
    } else {
        // Default case: remove /dev/ and expects the name presents under /sys/block/
        // For example, /dev/dm-0 to dm-0
        real_path = real_path.trim_start_matches("/dev/");
    }

    let trimmed: &OsStr = OsStrExt::from_bytes(real_path.as_bytes());

    let path = Path::new("/sys/block/")
        .to_owned()
        .join(trimmed)
        .join("queue/rotational");
    // Normally, this file only contains '0' or '1' but just in case, we get 8 bytes...
    match get_all_utf8_data(path, 8)
        .unwrap_or_default()
        .trim()
        .parse()
        .ok()
    {
        // The disk is marked as rotational so it's a HDD.
        Some(1) => DiskKind::HDD,
        // The disk is marked as non-rotational so it's very likely a SSD.
        Some(0) => DiskKind::SSD,
        // Normally it shouldn't happen but welcome to the wonderful world of IT! :D
        Some(x) => DiskKind::Unknown(x),
        // The information isn't available...
        None => DiskKind::Unknown(-1),
    }
}

fn get_all_list(container: &mut Vec<Disk>, content: &str, refresh_kind: DiskRefreshKind) {
    // The goal of this array is to list all removable devices (the ones whose name starts with
    // "usb-").
    let removable_entries = match fs::read_dir("/dev/disk/by-id/") {
        Ok(r) => r
            .filter_map(|res| Some(res.ok()?.path()))
            .filter_map(|e| {
                if e.file_name()
                    .and_then(|x| Some(x.to_str()?.starts_with("usb-")))
                    .unwrap_or_default()
                {
                    e.canonicalize().ok()
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>(),
        _ => Vec::new(),
    };

    let procfs_disk_stats = disk_stats(&refresh_kind);

    for (fs_spec, fs_file, fs_vfstype) in content
        .lines()
        .map(|line| {
            let line = line.trim_start();
            // mounts format
            // http://man7.org/linux/man-pages/man5/fstab.5.html
            // fs_spec<tab>fs_file<tab>fs_vfstype<tab>other fields
            let mut fields = line.split_whitespace();
            let fs_spec = fields.next().unwrap_or("");
            let fs_file = fields
                .next()
                .unwrap_or("")
                .replace("\\134", "\\")
                .replace("\\040", " ")
                .replace("\\011", "\t")
                .replace("\\012", "\n");
            let fs_vfstype = fields.next().unwrap_or("");
            (fs_spec, fs_file, fs_vfstype)
        })
        .filter(|(fs_spec, fs_file, fs_vfstype)| {
            // Check if fs_vfstype is one of our 'ignored' file systems.
            let filtered = match *fs_vfstype {
                "rootfs" | // https://www.kernel.org/doc/Documentation/filesystems/ramfs-rootfs-initramfs.txt
                "sysfs" | // pseudo file system for kernel objects
                "proc" |  // another pseudo file system
                "devtmpfs" |
                "cgroup" |
                "cgroup2" |
                "pstore" | // https://www.kernel.org/doc/Documentation/ABI/testing/pstore
                "squashfs" | // squashfs is a compressed read-only file system (for snaps)
                "rpc_pipefs" | // The pipefs pseudo file system service
                "iso9660" | // optical media
                "devpts" | // https://www.kernel.org/doc/Documentation/filesystems/devpts.txt
                "hugetlbfs" | // https://www.kernel.org/doc/Documentation/vm/hugetlbfs_reserv.txt
                "mqueue" // https://man7.org/linux/man-pages/man7/mq_overview.7.html
                => true,
                "tmpfs" => !cfg!(feature = "linux-tmpfs"),
                // calling statvfs on a mounted CIFS or NFS or through autofs may hang, when they are mounted with option: hard
                "cifs" | "nfs" | "nfs4" | "autofs" => !cfg!(feature = "linux-netdevs"),
                _ => false,
            };

            !(filtered ||
               fs_file.starts_with("/sys") || // check if fs_file is an 'ignored' mount point
               fs_file.starts_with("/proc") ||
               (fs_file.starts_with("/run") && !fs_file.starts_with("/run/media")) ||
               fs_spec.starts_with("sunrpc"))
        })
    {
        let mount_point = Path::new(&fs_file);
        if let Some(disk) = container.iter_mut().find(|d| {
            d.inner.mount_point == mount_point
                && d.inner.device_name == fs_spec
                && d.inner.file_system == fs_vfstype
        }) {
            disk.inner
                .efficient_refresh(refresh_kind, &procfs_disk_stats, false);
            disk.inner.updated = true;
            continue;
        }
        container.push(new_disk(
            fs_spec.as_ref(),
            mount_point,
            fs_vfstype.as_ref(),
            &removable_entries,
            &procfs_disk_stats,
            refresh_kind,
        ));
    }
}

/// Disk IO stat information from `/proc/diskstats` file.
///
/// To fully understand these fields, please see the
/// [iostats.txt](https://www.kernel.org/doc/Documentation/iostats.txt) kernel documentation.
///
/// This type only contains the value `sysinfo` is interested into.
///
/// The fields of this file are:
/// 1. major number
/// 2. minor number
/// 3. device name
/// 4. reads completed successfully
/// 5. reads merged
/// 6. sectors read
/// 7. time spent reading (ms)
/// 8. writes completed
/// 9. writes merged
/// 10. sectors written
/// 11. time spent writing (ms)
/// 12. I/Os currently in progress
/// 13. time spent doing I/Os (ms)
/// 14. weighted time spent doing I/Os (ms)
///
/// Doc reference: https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats
///
/// Doc reference: https://www.kernel.org/doc/Documentation/iostats.txt
#[derive(Debug, PartialEq)]
struct DiskStat {
    sectors_read: u64,
    sectors_written: u64,
}

impl DiskStat {
    /// Returns the name and the values we're interested into.
    fn new_from_line(line: &str) -> Option<(String, Self)> {
        let mut iter = line.split_whitespace();
        // 3rd field
        let name = iter.nth(2).map(ToString::to_string)?;
        // 6th field
        let sectors_read = iter.nth(2).and_then(|v| u64::from_str(v).ok()).unwrap_or(0);
        // 10th field
        let sectors_written = iter.nth(3).and_then(|v| u64::from_str(v).ok()).unwrap_or(0);
        Some((
            name,
            Self {
                sectors_read,
                sectors_written,
            },
        ))
    }
}

fn disk_stats(refresh_kind: &DiskRefreshKind) -> HashMap<String, DiskStat> {
    if refresh_kind.io_usage() {
        let path = "/proc/diskstats";
        match fs::read_to_string(path) {
            Ok(content) => disk_stats_inner(&content),
            Err(_error) => {
                sysinfo_debug!("failed to read {path:?}: {_error:?}");
                HashMap::new()
            }
        }
    } else {
        Default::default()
    }
}

// We split this function out to make it possible to test it.
fn disk_stats_inner(content: &str) -> HashMap<String, DiskStat> {
    let mut data = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((name, stats)) = DiskStat::new_from_line(line) {
            data.insert(name, stats);
        }
    }
    data
}

#[cfg(test)]
mod test {
    use super::{DiskStat, disk_stats_inner};
    use std::collections::HashMap;

    #[test]
    fn test_disk_stat_parsing() {
        // Content of a (very nicely formatted) `/proc/diskstats` file.
        let file_content = "\
 259       0 nvme0n1   571695 101559 38943220 165643 9824246  1076193 462375378 4140037  0  1038904 4740493  254020 0  1436922320 68519 306875 366293
 259       1 nvme0n1p1 240    2360   15468    48     2        0       2         0        0  21      50       8      0  2373552    2     0      0
 259       2 nvme0n1p2 243    10     11626    26     63       39      616       125      0  84      163      44     0  1075280    11    0      0
 259       3 nvme0n1p3 571069 99189  38910302 165547 9824180  1076154 462374760 4139911  0  1084855 4373964  253968 0  1433473488 68505 0      0
 253       0 dm-0      670206 0      38909056 259490 10900330 0       462374760 12906518 0  1177098 13195902 253968 0  1433473488 29894 0      0
 252       0 zram0     2382   0      20984    11     260261   0       2082088   2063     0  1964    2074     0      0  0          0     0      0
 1         2 bla       4      5      6        7      8        9       10        11       12 13      14       15     16 17         18    19     20
";

        let data = disk_stats_inner(file_content);
        let expected_data: HashMap<String, DiskStat> = HashMap::from([
            (
                "nvme0n1".to_string(),
                DiskStat {
                    sectors_read: 38943220,
                    sectors_written: 462375378,
                },
            ),
            (
                "nvme0n1p1".to_string(),
                DiskStat {
                    sectors_read: 15468,
                    sectors_written: 2,
                },
            ),
            (
                "nvme0n1p2".to_string(),
                DiskStat {
                    sectors_read: 11626,
                    sectors_written: 616,
                },
            ),
            (
                "nvme0n1p3".to_string(),
                DiskStat {
                    sectors_read: 38910302,
                    sectors_written: 462374760,
                },
            ),
            (
                "dm-0".to_string(),
                DiskStat {
                    sectors_read: 38909056,
                    sectors_written: 462374760,
                },
            ),
            (
                "zram0".to_string(),
                DiskStat {
                    sectors_read: 20984,
                    sectors_written: 2082088,
                },
            ),
            // This one ensures that we read the correct fields.
            (
                "bla".to_string(),
                DiskStat {
                    sectors_read: 6,
                    sectors_written: 10,
                },
            ),
        ]);

        assert_eq!(data, expected_data);
    }

    #[test]
    fn disk_entry_with_less_information() {
        let file_content = "\
 systemd-1      /efi autofs rw,relatime,fd=181,pgrp=1,timeout=120,minproto=5,maxproto=5,direct,pipe_ino=8311 0 0
 /dev/nvme0n1p1 /efi vfat   rw,nosuid,nodev,noexec,relatime,nosymfollow,fmask=0077,dmask=0077                0 0
";

        let data = disk_stats_inner(file_content);
        let expected_data: HashMap<String, DiskStat> = HashMap::from([
            (
                "autofs".to_string(),
                DiskStat {
                    sectors_read: 0,
                    sectors_written: 0,
                },
            ),
            (
                "vfat".to_string(),
                DiskStat {
                    sectors_read: 0,
                    sectors_written: 0,
                },
            ),
        ]);

        assert_eq!(data, expected_data);
    }
}
