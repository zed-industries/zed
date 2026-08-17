use crate::OpenError;
use normpath::PathExt;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::{io, ptr};
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

#[cfg(feature = "reveal")]
mod reveal;
#[cfg(feature = "reveal")]
pub(crate) use self::reveal::reveal;

pub(crate) fn open(path: &OsStr) -> Result<(), OpenError> {
    let Err(first_error) = open_helper(path) else {
        return Ok(());
    };

    match PathBuf::from(path).normalize() {
        Ok(normalized) => match open_helper(normalized.as_os_str()) {
            Ok(()) => Ok(()),
            Err(_second_error) => Err(first_error),
        },
        Err(_) => Err(first_error),
    }
}

pub(crate) fn open_helper(path: &OsStr) -> Result<(), OpenError> {
    let path = convert_path(path).map_err(OpenError::Io)?;
    let operation: Vec<u16> = OsStr::new("open\0").encode_wide().collect();
    let result = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            operation.as_ptr(),
            path.as_ptr(),
            ptr::null(),
            ptr::null(),
            SW_SHOW,
        )
    };
    if result as usize as isize > 32 {
        Ok(())
    } else {
        Err(OpenError::Io(io::Error::last_os_error()))
    }
}

fn convert_path(path: &OsStr) -> io::Result<Vec<u16>> {
    let mut maybe_result: Vec<u16> = path.encode_wide().collect();
    if maybe_result.iter().any(|&u| u == 0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path contains NUL byte(s)",
        ));
    }

    maybe_result.push(0);
    Ok(maybe_result)
}
