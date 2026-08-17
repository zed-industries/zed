// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Disk, DiskKind};

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use super::utils::c_buf_to_utf8_str;

pub(crate) struct DiskInner {
    name: OsString,
    c_mount_point: Vec<libc::c_char>,
    mount_point: PathBuf,
    total_space: u64,
    available_space: u64,
    file_system: OsString,
    is_removable: bool,
}

impl DiskInner {
    pub(crate) fn kind(&self) -> DiskKind {
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

    pub(crate) fn refresh(&mut self) -> bool {
        unsafe {
            let mut vfs: libc::statvfs = std::mem::zeroed();
            refresh_disk(self, &mut vfs)
        }
    }
}

impl crate::DisksInner {
    pub(crate) fn new() -> Self {
        Self {
            disks: Vec::with_capacity(2),
        }
    }

    pub(crate) fn refresh_list(&mut self) {
        unsafe { get_all_list(&mut self.disks) }
    }

    pub(crate) fn list(&self) -> &[Disk] {
        &self.disks
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Disk] {
        &mut self.disks
    }
}

// FIXME: if you want to get disk I/O usage:
// statfs.[f_syncwrites, f_asyncwrites, f_syncreads, f_asyncreads]

unsafe fn refresh_disk(disk: &mut DiskInner, vfs: &mut libc::statvfs) -> bool {
    if libc::statvfs(disk.c_mount_point.as_ptr() as *const _, vfs) < 0 {
        return false;
    }
    let f_frsize: u64 = vfs.f_frsize as _;

    disk.total_space = vfs.f_blocks.saturating_mul(f_frsize);
    disk.available_space = vfs.f_favail.saturating_mul(f_frsize);
    true
}

pub unsafe fn get_all_list(container: &mut Vec<Disk>) {
    container.clear();

    let mut fs_infos: *mut libc::statfs = std::ptr::null_mut();

    let count = libc::getmntinfo(&mut fs_infos, libc::MNT_WAIT);

    if count < 1 {
        return;
    }
    let mut vfs: libc::statvfs = std::mem::zeroed();
    let fs_infos: &[libc::statfs] = std::slice::from_raw_parts(fs_infos as _, count as _);

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

        if libc::statvfs(fs_info.f_mntonname.as_ptr(), &mut vfs) != 0 {
            continue;
        }

        let mount_point = match c_buf_to_utf8_str(&fs_info.f_mntonname) {
            Some(m) => m,
            None => {
                sysinfo_debug!("Cannot get disk mount point, ignoring it.");
                continue;
            }
        };

        let name = if mount_point == "/" {
            OsString::from("root")
        } else {
            OsString::from(mount_point)
        };

        // USB keys and CDs are removable.
        let is_removable =
            [b"USB", b"usb"].iter().any(|b| *b == &fs_type[..]) || fs_type.starts_with(b"/dev/cd");

        let f_frsize: u64 = vfs.f_frsize as _;

        container.push(Disk {
            inner: DiskInner {
                name,
                c_mount_point: fs_info.f_mntonname.to_vec(),
                mount_point: PathBuf::from(mount_point),
                total_space: vfs.f_blocks.saturating_mul(f_frsize),
                available_space: vfs.f_favail.saturating_mul(f_frsize),
                file_system: OsString::from_vec(fs_type),
                is_removable,
            },
        });
    }
}
