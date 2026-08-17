// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::HandleWrapper;
use crate::{Disk, DiskKind, DiskRefreshKind, DiskUsage};

use std::ffi::{OsStr, OsString};
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem::{
    FindFirstVolumeW, FindNextVolumeW, FindVolumeClose, GetDiskFreeSpaceExW, GetDriveTypeW,
    GetVolumeInformationW, GetVolumePathNamesForVolumeNameW,
};
use windows::Win32::System::IO::DeviceIoControl;
use windows::Win32::System::Ioctl::{
    DEVICE_SEEK_PENALTY_DESCRIPTOR, DISK_PERFORMANCE, IOCTL_DISK_PERFORMANCE,
    IOCTL_STORAGE_QUERY_PROPERTY, PropertyStandardQuery, STORAGE_PROPERTY_QUERY,
    StorageDeviceSeekPenaltyProperty,
};
use windows::Win32::System::SystemServices::FILE_READ_ONLY_VOLUME;
use windows::Win32::System::WindowsProgramming::{DRIVE_FIXED, DRIVE_REMOVABLE};
use windows::core::{Error, HRESULT, PCWSTR};

/// Creates a copy of the first zero-terminated wide string in `buf`.
/// The copy includes the zero terminator.
fn from_zero_terminated(buf: &[u16]) -> Vec<u16> {
    let end = buf.iter().position(|&x| x == 0).unwrap_or(buf.len());
    buf[..=end].to_vec()
}

// Realistically, volume names are probably not longer than 44 characters,
// but the example in the Microsoft documentation uses MAX_PATH as well.
// https://learn.microsoft.com/en-us/windows/win32/fileio/displaying-volume-paths
const VOLUME_NAME_SIZE: usize = MAX_PATH as usize + 1;

const ERROR_NO_MORE_FILES: HRESULT = windows::Win32::Foundation::ERROR_NO_MORE_FILES.to_hresult();
const ERROR_MORE_DATA: HRESULT = windows::Win32::Foundation::ERROR_MORE_DATA.to_hresult();

/// Returns a list of zero-terminated wide strings containing volume GUID paths.
/// Volume GUID paths have the form `\\?\{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}\`.
///
/// Rather confusingly, the Win32 API _also_ calls these "volume names".
pub(crate) fn get_volume_guid_paths() -> Vec<Vec<u16>> {
    let mut volume_names = Vec::new();
    unsafe {
        let mut buf = Box::new([0u16; VOLUME_NAME_SIZE]);
        let Ok(handle) = FindFirstVolumeW(&mut buf[..]) else {
            sysinfo_debug!(
                "Error: FindFirstVolumeW() = {:?}",
                Error::from_win32().code()
            );
            return Vec::new();
        };
        volume_names.push(from_zero_terminated(&buf[..]));
        loop {
            if FindNextVolumeW(handle, &mut buf[..]).is_err() {
                if Error::from_win32().code() != ERROR_NO_MORE_FILES {
                    sysinfo_debug!("Error: FindNextVolumeW = {}", Error::from_win32().code());
                }
                break;
            }
            volume_names.push(from_zero_terminated(&buf[..]));
        }
        if FindVolumeClose(handle).is_err() {
            sysinfo_debug!("Error: FindVolumeClose = {:?}", Error::from_win32().code());
        };
    }
    volume_names
}

/// Given a volume GUID path (`\\?\{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}\`), returns all
/// volume paths (drive letters and mount paths) associated with it
/// as zero terminated wide strings.
///
/// # Safety
/// `volume_name` must contain a zero-terminated wide string.
pub(crate) unsafe fn get_volume_path_names_for_volume_name(
    volume_guid_path: &[u16],
) -> Vec<Vec<u16>> {
    let volume_guid_path = PCWSTR::from_raw(volume_guid_path.as_ptr());

    // Initial buffer size is just a guess. There is no direct connection between MAX_PATH
    // the output of GetVolumePathNamesForVolumeNameW.
    let mut path_names_buf = vec![0u16; MAX_PATH as usize];
    let mut path_names_output_size = 0u32;
    for _ in 0..10 {
        let volume_path_names = unsafe {
            GetVolumePathNamesForVolumeNameW(
                volume_guid_path,
                Some(path_names_buf.as_mut_slice()),
                &mut path_names_output_size,
            )
        };
        let code = volume_path_names.map_err(|_| Error::from_win32().code());
        match code {
            Ok(()) => break,
            Err(ERROR_MORE_DATA) => {
                // We need a bigger buffer. path_names_output_size contains the required buffer size.
                path_names_buf = vec![0u16; path_names_output_size as usize];
                continue;
            }
            Err(_e) => {
                sysinfo_debug!("Error: GetVolumePathNamesForVolumeNameW() = {}", _e);
                return Vec::new();
            }
        }
    }

    // path_names_buf contains multiple zero terminated wide strings.
    // An additional zero terminates the list.
    let mut path_names = Vec::new();
    let mut buf = &path_names_buf[..];
    while !buf.is_empty() && buf[0] != 0 {
        let path = from_zero_terminated(buf);
        buf = &buf[path.len()..];
        path_names.push(path);
    }
    path_names
}

pub(crate) struct DiskInner {
    type_: DiskKind,
    name: OsString,
    file_system: OsString,
    mount_point: Vec<u16>,
    s_mount_point: OsString,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
    is_read_only: bool,
    device_path: Vec<u16>,
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
        &self.name
    }

    pub(crate) fn file_system(&self) -> &OsStr {
        &self.file_system
    }

    pub(crate) fn mount_point(&self) -> &Path {
        self.s_mount_point.as_ref()
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

    pub(crate) fn refresh_specifics(&mut self, refreshes: DiskRefreshKind) -> bool {
        if refreshes.kind() || refreshes.io_usage() {
            unsafe {
                if let Some(handle) =
                    HandleWrapper::new_from_file(&self.device_path, Default::default())
                {
                    if refreshes.kind() && self.type_ == DiskKind::Unknown(-1) {
                        self.type_ = get_disk_kind(&handle);
                    }

                    if refreshes.io_usage() {
                        if let Some((read_bytes, written_bytes)) = get_disk_io(handle) {
                            self.old_read_bytes = self.read_bytes;
                            self.old_written_bytes = self.written_bytes;
                            self.read_bytes = read_bytes;
                            self.written_bytes = written_bytes;
                        } else {
                            sysinfo_debug!("Failed to update disk i/o stats");
                        }
                    }
                }
            }
        }

        if refreshes.storage()
            && let Some((total_space, available_space)) =
                unsafe { get_drive_size(&self.mount_point) }
        {
            self.total_space = total_space;
            self.available_space = available_space;
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

pub(crate) struct DisksInner {
    pub(crate) disks: Vec<Disk>,
}

impl DisksInner {
    pub(crate) fn new() -> Self {
        Self {
            disks: Vec::with_capacity(2),
        }
    }

    pub(crate) fn from_vec(disks: Vec<Disk>) -> Self {
        Self { disks }
    }

    pub(crate) fn into_vec(self) -> Vec<Disk> {
        self.disks
    }

    pub(crate) fn refresh_specifics(
        &mut self,
        remove_not_listed_disks: bool,
        refreshes: DiskRefreshKind,
    ) {
        unsafe {
            get_list(&mut self.disks, remove_not_listed_disks, refreshes);
        }
    }

    pub(crate) fn list(&self) -> &[Disk] {
        &self.disks
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Disk] {
        &mut self.disks
    }
}

unsafe fn get_drive_size(mount_point: &[u16]) -> Option<(u64, u64)> {
    let mut total_size = 0;
    let mut available_space = 0;
    let lpdirectoryname = PCWSTR::from_raw(mount_point.as_ptr());
    if unsafe {
        GetDiskFreeSpaceExW(
            lpdirectoryname,
            None,
            Some(&mut total_size),
            Some(&mut available_space),
        )
    }
    .is_ok()
    {
        Some((total_size, available_space))
    } else {
        None
    }
}

pub(crate) unsafe fn get_list(
    disks: &mut Vec<Disk>,
    remove_not_listed_disks: bool,
    refreshes: DiskRefreshKind,
) {
    for volume_name in get_volume_guid_paths() {
        let mount_paths = unsafe { get_volume_path_names_for_volume_name(&volume_name[..]) };
        if mount_paths.is_empty() {
            continue;
        }

        let raw_volume_name = PCWSTR::from_raw(volume_name.as_ptr());
        let drive_type = unsafe { GetDriveTypeW(raw_volume_name) };

        if drive_type != DRIVE_FIXED && drive_type != DRIVE_REMOVABLE {
            continue;
        }

        let is_removable = drive_type == DRIVE_REMOVABLE;

        let mut name = [0u16; MAX_PATH as usize + 1];
        let mut file_system = [0u16; 32];
        let mut flags = 0;
        let volume_info_res = unsafe {
            GetVolumeInformationW(
                raw_volume_name,
                Some(&mut name),
                None,
                None,
                Some(&mut flags),
                Some(&mut file_system),
            )
        }
        .is_ok();
        if !volume_info_res {
            sysinfo_debug!(
                "Error: GetVolumeInformationW = {:?}",
                Error::from_win32().code()
            );
            continue;
        }
        let is_read_only = (flags & FILE_READ_ONLY_VOLUME) != 0;

        // The device path is the volume name without the trailing backslash.
        let device_path = volume_name[..(volume_name.len() - 2)]
            .iter()
            .copied()
            .chain([0])
            .collect::<Vec<_>>();

        let name = os_string_from_zero_terminated(&name);
        let file_system = os_string_from_zero_terminated(&file_system);
        for mount_path in mount_paths {
            if let Some(disk) = disks
                .iter_mut()
                .find(|d| d.inner.mount_point == mount_path && d.inner.file_system == file_system)
            {
                disk.refresh_specifics(refreshes);
                disk.inner.updated = true;
                continue;
            }
            let mut disk = DiskInner {
                type_: DiskKind::Unknown(-1),
                name: name.clone(),
                file_system: file_system.clone(),
                s_mount_point: OsString::from_wide(&mount_path[..mount_path.len() - 1]),
                mount_point: mount_path,
                total_space: 0,
                available_space: 0,
                is_removable,
                is_read_only,
                device_path: device_path.clone(),
                old_read_bytes: 0,
                old_written_bytes: 0,
                read_bytes: 0,
                written_bytes: 0,
                updated: true,
            };
            disk.refresh_specifics(refreshes);
            disks.push(Disk { inner: disk });
        }
    }

    if remove_not_listed_disks {
        disks.retain_mut(|disk| {
            if !disk.inner.updated {
                return false;
            }
            disk.inner.updated = false;
            true
        });
    } else {
        for c in disks.iter_mut() {
            c.inner.updated = false;
        }
    }
}

fn os_string_from_zero_terminated(name: &[u16]) -> OsString {
    let len = name.iter().position(|&x| x == 0).unwrap_or(name.len());
    OsString::from_wide(&name[..len])
}

unsafe fn get_disk_kind(handle: &HandleWrapper) -> DiskKind {
    let spq_trim = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceSeekPenaltyProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    let mut result: DEVICE_SEEK_PENALTY_DESCRIPTOR = unsafe { std::mem::zeroed() };

    let mut dw_size = 0;
    let device_io_control = unsafe {
        DeviceIoControl(
            handle.0,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some(&spq_trim as *const STORAGE_PROPERTY_QUERY as *const _),
            size_of::<STORAGE_PROPERTY_QUERY>() as _,
            Some(&mut result as *mut DEVICE_SEEK_PENALTY_DESCRIPTOR as *mut _),
            size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as _,
            Some(&mut dw_size),
            None,
        )
        .is_ok()
    };

    if !device_io_control || dw_size != size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32 {
        DiskKind::Unknown(-1)
    } else {
        let is_hdd = result.IncursSeekPenalty;
        if is_hdd { DiskKind::HDD } else { DiskKind::SSD }
    }
}

/// Returns a tuple consisting of the total number of bytes read and written by the volume with the
/// specified device path
fn get_disk_io(handle: HandleWrapper) -> Option<(u64, u64)> {
    let mut disk_perf = DISK_PERFORMANCE::default();
    let mut bytes_returned = 0;

    // SAFETY: the handle is checked for validity above
    unsafe {
        // See <https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-ioctl_disk_performance> for reference
        DeviceIoControl(
            handle.0,
            IOCTL_DISK_PERFORMANCE,
            None, // Must be None as per docs
            0,
            Some(&mut disk_perf as *mut _ as _),
            size_of::<DISK_PERFORMANCE>() as u32,
            Some(&mut bytes_returned),
            None,
        )
    }
    .inspect_err(|_err| {
        sysinfo_debug!(
            "Error: DeviceIoControl(IOCTL_DISK_PERFORMANCE) = {:?}",
            _err
        );
    })
    .ok()?;

    Some((
        disk_perf.BytesRead.try_into().ok()?,
        disk_perf.BytesWritten.try_into().ok()?,
    ))
}
