use std::{
    ffi::{OsStr, OsString},
    process::Command,
};

use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn commands<T: AsRef<OsStr>>(path: T) -> Vec<Command> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/c")
        .arg("start")
        .raw_arg("\"\"")
        .raw_arg(wrap_in_quotes(path))
        .creation_flags(CREATE_NO_WINDOW);
    vec![cmd]
}

pub fn with_command<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/c")
        .arg("start")
        .raw_arg("\"\"")
        .raw_arg(wrap_in_quotes(app.into()))
        .raw_arg(wrap_in_quotes(path))
        .creation_flags(CREATE_NO_WINDOW);
    cmd
}

fn wrap_in_quotes<T: AsRef<OsStr>>(path: T) -> OsString {
    let mut result = OsString::from("\"");
    result.push(path);
    result.push("\"");

    result
}

#[cfg(feature = "shellexecute-on-windows")]
pub fn that_detached<T: AsRef<OsStr>>(path: T) -> std::io::Result<()> {
    use std::path::Path;

    let path = path.as_ref();
    let is_dir = std::fs::metadata(path).map(|f| f.is_dir()).unwrap_or(false);

    if is_dir {
        let path = dunce::simplified(Path::new(path));
        let path = wide(path);
        unsafe { ffi::CoInitialize(std::ptr::null()) };
        let folder = unsafe { ffi::ILCreateFromPathW(path.as_ptr()) };
        if unsafe { SHOpenFolderAndSelectItems(folder, Some(&[folder]), 0) }.is_ok() {
            return Ok(());
        }
    };

    let path = wide(path);

    let (verb, class) = if is_dir {
        (ffi::EXPLORE, ffi::FOLDER)
    } else {
        (std::ptr::null(), std::ptr::null())
    };

    let mut info = ffi::SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<ffi::SHELLEXECUTEINFOW>() as _,
        nShow: ffi::SW_SHOWNORMAL,
        lpVerb: verb,
        lpClass: class,
        lpFile: path.as_ptr(),
        ..unsafe { std::mem::zeroed() }
    };

    unsafe { ShellExecuteExW(&mut info) }
}

#[cfg(feature = "shellexecute-on-windows")]
pub fn with_detached<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> std::io::Result<()> {
    let app = wide(app.into());
    let path = wide(path);

    let mut info = ffi::SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<ffi::SHELLEXECUTEINFOW>() as _,
        nShow: ffi::SW_SHOWNORMAL,
        lpFile: app.as_ptr(),
        lpParameters: path.as_ptr(),
        ..unsafe { std::mem::zeroed() }
    };

    unsafe { ShellExecuteExW(&mut info) }
}

/// Encodes as wide and adds a null character.
#[cfg(feature = "shellexecute-on-windows")]
#[inline]
fn wide<T: AsRef<OsStr>>(input: T) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    input
        .as_ref()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Performs an operation on a specified file.
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecuteexw>
#[allow(non_snake_case)]
#[cfg(feature = "shellexecute-on-windows")]
pub unsafe fn ShellExecuteExW(info: *mut ffi::SHELLEXECUTEINFOW) -> std::io::Result<()> {
    // ShellExecuteExW returns TRUE (i.e 1) on success
    // https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecuteexw#remarks
    if ffi::ShellExecuteExW(info) == 1 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

// Taken from https://microsoft.github.io/windows-docs-rs/doc/windows/
/// Opens a Windows Explorer window with specified items in a particular folder selected.
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shopenfolderandselectitems>
#[allow(non_snake_case)]
#[cfg(feature = "shellexecute-on-windows")]
pub unsafe fn SHOpenFolderAndSelectItems(
    pidlfolder: *const ffi::ITEMIDLIST,
    apidl: Option<&[*const ffi::ITEMIDLIST]>,
    dwflags: u32,
) -> std::io::Result<()> {
    use std::convert::TryInto;

    match ffi::SHOpenFolderAndSelectItems(
        pidlfolder,
        apidl
            .as_deref()
            .map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(
            apidl
                .as_deref()
                .map_or(core::ptr::null(), |slice| slice.as_ptr()),
        ),
        dwflags,
    ) {
        0 => Ok(()),
        error_code => Err(std::io::Error::from_raw_os_error(error_code)),
    }
}

#[cfg(feature = "shellexecute-on-windows")]
#[allow(non_snake_case)]
mod ffi {
    /// Activates and displays a window.
    /// If the window is minimized, maximized, or arranged, the system restores it to its original size and position.
    /// An application should specify this flag when displaying the window for the first time.
    ///
    /// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow>
    pub const SW_SHOWNORMAL: i32 = 1;

    /// Null-terminated UTF-16 encoding of `explore`.  
    pub const EXPLORE: *const u16 = [101, 120, 112, 108, 111, 114, 101, 0].as_ptr();

    /// Null-terminated UTF-16 encoding of `folder`.  
    pub const FOLDER: *const u16 = [102, 111, 108, 100, 101, 114, 0].as_ptr();

    // Taken from https://docs.rs/windows-sys/latest/windows_sys/
    #[cfg_attr(not(target_arch = "x86"), repr(C))]
    #[cfg_attr(target_arch = "x86", repr(C, packed(1)))]
    pub struct SHELLEXECUTEINFOW {
        pub cbSize: u32,
        pub fMask: u32,
        pub hwnd: isize,
        pub lpVerb: *const u16,
        pub lpFile: *const u16,
        pub lpParameters: *const u16,
        pub lpDirectory: *const u16,
        pub nShow: i32,
        pub hInstApp: isize,
        pub lpIDList: *mut core::ffi::c_void,
        pub lpClass: *const u16,
        pub hkeyClass: isize,
        pub dwHotKey: u32,
        pub Anonymous: SHELLEXECUTEINFOW_0,
        pub hProcess: isize,
    }

    // Taken from https://docs.rs/windows-sys/latest/windows_sys/
    #[cfg_attr(not(target_arch = "x86"), repr(C))]
    #[cfg_attr(target_arch = "x86", repr(C, packed(1)))]
    pub union SHELLEXECUTEINFOW_0 {
        pub hIcon: isize,
        pub hMonitor: isize,
    }

    // Taken from https://microsoft.github.io/windows-docs-rs/doc/windows/
    #[repr(C, packed(1))]
    pub struct SHITEMID {
        pub cb: u16,
        pub abID: [u8; 1],
    }

    // Taken from https://microsoft.github.io/windows-docs-rs/doc/windows/
    #[repr(C, packed(1))]
    pub struct ITEMIDLIST {
        pub mkid: SHITEMID,
    }

    #[link(name = "shell32")]
    extern "system" {
        pub fn ShellExecuteExW(info: *mut SHELLEXECUTEINFOW) -> isize;
        pub fn ILCreateFromPathW(pszpath: *const u16) -> *mut ITEMIDLIST;
        pub fn SHOpenFolderAndSelectItems(
            pidlfolder: *const ITEMIDLIST,
            cidl: u32,
            apidl: *const *const ITEMIDLIST,
            dwflags: u32,
        ) -> i32;
    }

    #[link(name = "ole32")]
    extern "system" {
        pub fn CoInitialize(pvreserved: *const core::ffi::c_void) -> i32;
    }
}
