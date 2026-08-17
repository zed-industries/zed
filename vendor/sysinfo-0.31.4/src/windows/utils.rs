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

    p.to_string().unwrap_or_else(|_e| {
        sysinfo_debug!("Failed to convert to UTF-16 string: {}", _e);
        String::new()
    })
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
                let handle = CreateFileW(
                    lpfilename,
                    open_rights.0,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    None,
                    OPEN_EXISTING,
                    Default::default(),
                    HANDLE::default(),
                )
                .ok()?;
                Some(Self(handle))
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
