// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "disk")]
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ACCESS_RIGHTS, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};

#[cfg(any(feature = "user", feature = "system"))]
pub(crate) unsafe fn to_utf8_str(p: windows::core::PWSTR) -> String {
    if p.is_null() {
        return String::new();
    }

    unsafe {
        p.to_string().unwrap_or_else(|_e| {
            sysinfo_debug!("Failed to convert to UTF-16 string: {}", _e);
            String::new()
        })
    }
}

cfg_if! {
    if #[cfg(any(feature = "disk", feature = "system"))] {
        use windows::Win32::Foundation::{CloseHandle, HANDLE};
        use std::ops::Deref;

        pub(crate) struct HandleWrapper(pub(crate) HANDLE);

        impl HandleWrapper {
            #[cfg(feature = "system")]
            pub(crate) fn new(handle: HANDLE) -> Option<Self> {
                if handle.is_invalid() {
                    None
                } else {
                    Some(Self(handle))
                }
            }

            #[cfg(feature = "disk")]
            pub(crate) unsafe fn new_from_file(
                drive_name: &[u16],
                open_rights: FILE_ACCESS_RIGHTS,
            ) -> Option<Self> {
                let lpfilename = windows::core::PCWSTR::from_raw(drive_name.as_ptr());
                let handle = unsafe { CreateFileW(
                    lpfilename,
                    open_rights.0,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    None,
                    OPEN_EXISTING,
                    Default::default(),
                    Some(HANDLE::default()),
                ) }
                .ok()?;
                if handle.is_invalid() {
                    sysinfo_debug!(
                        "Expected handle to {:?} to be valid",
                        String::from_utf16_lossy(drive_name)
                    );
                    None
                } else {
                    Some(Self(handle))
                }
            }
        }

        impl Deref for HandleWrapper {
            type Target = HANDLE;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Drop for HandleWrapper {
            fn drop(&mut self) {
                let _err = unsafe { CloseHandle(self.0) };
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "system")] {
        use windows::Win32::System::SystemInformation::{FIRMWARE_TABLE_PROVIDER, GetSystemFirmwareTable};
        use super::ffi::SMBIOSType;

        // Get the SMBIOS table using the WinAPI.
        pub(crate) fn get_smbios_table() -> Option<Vec<u8>> {
            const PROVIDER: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(u32::from_be_bytes(*b"RSMB"));

            let size = unsafe { GetSystemFirmwareTable(PROVIDER, 0, None) };
            if size == 0 {
                return None;
            }

            let mut buffer = vec![0u8; size as usize];

            let res = unsafe { GetSystemFirmwareTable(PROVIDER, 0, Some(&mut buffer)) };
            if res == 0 {
                return None;
            }

            Some(buffer)
        }

        // Parses the SMBIOS table to get mainboard information (type number).
        // Returns a part of struct with its associated strings.
        // The parsing format is described here: https://wiki.osdev.org/System_Management_BIOS
        // and here: https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.6.0.pdf
        pub(crate) fn parse_smbios<T: SMBIOSType>(table: &[u8], number: u8) -> Option<(T, Vec<&str>)> {
            // Skip SMBIOS types until type `number` is reached.
            // All indexes provided by the structure start at 1.
            // If the index is 0, the value has not been filled in.
            // At index i:
            //      table[i] is the current SMBIOS type.
            //      table[i + 1] is the length of the current SMBIOS table header
            //      Strings section starts immediately after the SMBIOS header,
            //      and is a list of null-terminated strings, terminated with two \0.
            let mut i = 0;
            while i + 1 < table.len() {
                if table[i] == number {
                    break;
                }
                i += table[i + 1] as usize;
                // Skip strings table (terminated itself by \0)
                while i < table.len() {
                    if table[i] == 0 && table[i + 1] == 0 {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            }

            let info: T = unsafe { std::ptr::read_unaligned(table[i..].as_ptr() as *const _) };

            // As said in the SMBIOS 3 standard: https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.6.0.pdf,
            // the strings are necessarily in UTF-8. But sometimes virtual machines may return non-compliant data.
            let values = table[(i + info.length() as usize)..]
                .split(|&b| b == 0)
                .filter_map(|s| std::str::from_utf8(s).ok())
                .take_while(|s| !s.is_empty())
                .collect();

            Some((info, values))
        }
    }
}
