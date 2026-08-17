// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::HandleWrapper;
use crate::{Disk, DiskKind};

use std::ffi::{c_void, OsStr, OsString};
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

use windows::core::{Error, HRESULT, PCWSTR};
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem::{
    FindFirstVolumeW, FindNextVolumeW, FindVolumeClose, GetDiskFreeSpaceExW, GetDriveTypeW,
    GetVolumeInformationW, GetVolumePathNamesForVolumeNameW,
};
use windows::Win32::System::Ioctl::{
    PropertyStandardQuery, StorageDeviceSeekPenaltyProperty, DEVICE_SEEK_PENALTY_DESCRIPTOR,
    IOCTL_STORAGE_QUERY_PROPERTY, STORAGE_PROPERTY_QUERY,
};
use windows::Win32::System::WindowsProgramming::{DRIVE_FIXED, DRIVE_REMOVABLE};
use windows::Win32::System::IO::DeviceIoControl;

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
        let volume_path_names = GetVolumePathNamesForVolumeNameW(
            volume_guid_path,
            Some(path_names_buf.as_mut_slice()),
            &mut path_names_output_size,
        );
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

    pub(crate) fn refresh(&mut self) -> bool {
        if self.total_space != 0 {
            unsafe {
                let mut tmp = 0;
                let lpdirectoryname = PCWSTR::from_raw(self.mount_point.as_ptr());
                if GetDiskFreeSpaceExW(lpdirectoryname, None, None, Some(&mut tmp)).is_ok() {
                    self.available_space = tmp;
                    return true;
                }
            }
        }
        false
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

    pub(crate) fn refresh_list(&mut self) {
        unsafe {
            self.disks = get_list();
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
    if GetDiskFreeSpaceExW(
        lpdirectoryname,
        None,
        Some(&mut total_size),
        Some(&mut available_space),
    )
    .is_ok()
    {
        Some((total_size, available_space))
    } else {
        None
    }
}

pub(crate) unsafe fn get_list() -> Vec<Disk> {
    #[cfg(feature = "multithread")]
    use rayon::iter::ParallelIterator;

    crate::utils::into_iter(get_volume_guid_paths())
        .flat_map(|volume_name| {
            let raw_volume_name = PCWSTR::from_raw(volume_name.as_ptr());
            let drive_type = GetDriveTypeW(raw_volume_name);

            let is_removable = drive_type == DRIVE_REMOVABLE;

            if drive_type != DRIVE_FIXED && drive_type != DRIVE_REMOVABLE {
                return Vec::new();
            }
            let mut name = [0u16; MAX_PATH as usize + 1];
            let mut file_system = [0u16; 32];
            let volume_info_res = GetVolumeInformationW(
                raw_volume_name,
                Some(&mut name),
                None,
                None,
                None,
                Some(&mut file_system),
            )
            .is_ok();
            if !volume_info_res {
                sysinfo_debug!(
                    "Error: GetVolumeInformationW = {:?}",
                    Error::from_win32().code()
                );
                return Vec::new();
            }

            let mount_paths = get_volume_path_names_for_volume_name(&volume_name[..]);
            if mount_paths.is_empty() {
                return Vec::new();
            }

            // The device path is the volume name without the trailing backslash.
            let device_path = volume_name[..(volume_name.len() - 2)]
                .iter()
                .copied()
                .chain([0])
                .collect::<Vec<_>>();
            let Some(handle) = HandleWrapper::new_from_file(&device_path[..], Default::default())
            else {
                return Vec::new();
            };
            let Some((total_space, available_space)) = get_drive_size(&mount_paths[0][..]) else {
                return Vec::new();
            };
            if total_space == 0 {
                sysinfo_debug!("total_space == 0");
                return Vec::new();
            }
            let spq_trim = STORAGE_PROPERTY_QUERY {
                PropertyId: StorageDeviceSeekPenaltyProperty,
                QueryType: PropertyStandardQuery,
                AdditionalParameters: [0],
            };
            let mut result: DEVICE_SEEK_PENALTY_DESCRIPTOR = std::mem::zeroed();

            let mut dw_size = 0;
            let device_io_control = DeviceIoControl(
                handle.0,
                IOCTL_STORAGE_QUERY_PROPERTY,
                Some(&spq_trim as *const STORAGE_PROPERTY_QUERY as *const c_void),
                size_of::<STORAGE_PROPERTY_QUERY>() as u32,
                Some(&mut result as *mut DEVICE_SEEK_PENALTY_DESCRIPTOR as *mut c_void),
                size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32,
                Some(&mut dw_size),
                None,
            )
            .is_ok();
            let type_ = if !device_io_control
                || dw_size != size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32
            {
                DiskKind::Unknown(-1)
            } else {
                let is_hdd = result.IncursSeekPenalty.as_bool();
                if is_hdd {
                    DiskKind::HDD
                } else {
                    DiskKind::SSD
                }
            };

            let name = os_string_from_zero_terminated(&name);
            let file_system = os_string_from_zero_terminated(&file_system);
            mount_paths
                .into_iter()
                .map(move |mount_path| Disk {
                    inner: DiskInner {
                        type_,
                        name: name.clone(),
                        file_system: file_system.clone(),
                        s_mount_point: OsString::from_wide(&mount_path[..mount_path.len() - 1]),
                        mount_point: mount_path,
                        total_space,
                        available_space,
                        is_removable,
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn os_string_from_zero_terminated(name: &[u16]) -> OsString {
    let len = name.iter().position(|&x| x == 0).unwrap_or(name.len());
    OsString::from_wide(&name[..len])
}
