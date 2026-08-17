//! FFI to [Jon Blow's VS discovery script](https://pastebin.com/3YvWQa5c).
//!
//! See the associated funxions on `VsFindResult` for more specific info,
//! but I'll copy over some usage information from the original file:
//!
//! The purpose of this file is to find the folders that contain libraries
//! you may need to link against, on Windows, if you are linking with any
//! compiled C or C++ code. This will be necessary for many non-C++ programming
//! language environments that want to provide compatibility.
//!
//! We find the place where the Visual Studio libraries live (for example,
//! libvcruntime.lib), where the linker and compiler executables live
//! (for example, link.exe), and where the Windows SDK libraries reside
//! (kernel32.lib, libucrt.lib).
//!
//! One other shortcut I took is that this is hardcoded to return the
//! folders for x64 libraries. If you want x86 or arm, you can make
//! slight edits to the code below, or, if enough people want this,
//! I can work it in here.


extern crate vswhom_sys;
#[cfg(target_os = "windows")]
extern crate libc;

#[cfg(target_os = "windows")]
use vswhom_sys::{vswhom_find_visual_studio_and_windows_sdk, vswhom_free_resources};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use vswhom_sys::Find_Result;
use std::num::NonZeroU8;
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use libc::wcslen;
#[cfg(target_os = "windows")]
use std::slice;


/// The result of looking for Visual Studio and Windows SDK.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VsFindResult {
    pub windows_sdk_version: NonZeroU8,

    pub windows_sdk_root: Option<OsString>,
    pub windows_sdk_um_library_path: Option<OsString>,
    pub windows_sdk_ucrt_library_path: Option<OsString>,

    pub vs_exe_path: Option<OsString>,
    pub vs_library_path: Option<OsString>,
}

impl VsFindResult {
    /// Use `vswhom-sys` to find Visual Studio and Windows SDK and parse it.
    ///
    /// Always returns `None` on non-Windows.
    pub fn search() -> Option<VsFindResult> {
        #[cfg(target_os = "windows")]
        unsafe {
            let mut res = vswhom_find_visual_studio_and_windows_sdk();
            let ret = VsFindResult::from_raw_result(&res);
            vswhom_free_resources(&mut res);
            ret
        }

        #[cfg(not(target_os = "windows"))]
        None
    }

    /// Parse a result from `vswhom_sys`.
    ///
    /// Returns `None` if `windows_sdk_version` is `0`.
    ///
    /// Allocates fresh Rust `OsString`s where non-null.
    ///
    /// Always returns `None` on non-Windows.
    pub unsafe fn from_raw_result(res: &Find_Result) -> Option<VsFindResult> {
        #[cfg(target_os = "windows")]
        {
            if res.windows_sdk_version != 0 {
                Some(VsFindResult {
                    windows_sdk_version: NonZeroU8::new_unchecked(res.windows_sdk_version as u8),

                    windows_sdk_root: osfpo(res.windows_sdk_root),
                    windows_sdk_um_library_path: osfpo(res.windows_sdk_um_library_path),
                    windows_sdk_ucrt_library_path: osfpo(res.windows_sdk_ucrt_library_path),

                    vs_exe_path: osfpo(res.vs_exe_path),
                    vs_library_path: osfpo(res.vs_library_path),
                })
            } else {
                None
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = res;
            None
        }
    }
}


#[cfg(target_os = "windows")]
unsafe fn osfpo(s: *const u16) -> Option<OsString> {
    if !s.is_null() {
        Some(OsString::from_wide(slice::from_raw_parts(s, wcslen(s))))
    } else {
        None
    }
}
