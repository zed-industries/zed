use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::ptr;

use windows_sys::Win32::UI::Shell::SHGetFileInfoW;
use windows_sys::Win32::UI::Shell::SHFILEINFOW;
use windows_sys::Win32::UI::Shell::SHGFI_DISPLAYNAME;

pub(crate) fn name(path: &Path) -> Option<OsString> {
    let mut path: Vec<_> = path.as_os_str().encode_wide().collect();
    if path.contains(&0) {
        return None;
    }
    path.push(0);

    let mut path_info = SHFILEINFOW {
        hIcon: ptr::null_mut(),
        iIcon: 0,
        dwAttributes: 0,
        szDisplayName: [0; 260],
        szTypeName: [0; 80],
    };
    let result = unsafe {
        SHGetFileInfoW(
            path.as_ptr(),
            0,
            &mut path_info,
            size_of_val(&path_info)
                .try_into()
                .expect("path information too large for WinAPI"),
            SHGFI_DISPLAYNAME,
        )
    };
    if result == 0 {
        return None;
    }

    // The display name buffer has a fixed length, so it must be truncated at
    // the first null character.
    Some(OsString::from_wide(
        path_info
            .szDisplayName
            .split(|&x| x == 0)
            .next()
            .expect("missing null byte in display name"),
    ))
}
