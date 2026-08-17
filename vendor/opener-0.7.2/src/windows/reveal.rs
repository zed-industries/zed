#![allow(non_camel_case_types, non_snake_case)]

use super::convert_path;
use crate::OpenError;
use normpath::PathExt;
use std::path::Path;
use std::{io, ptr, thread};
use windows_sys::core::{HRESULT, PCWSTR};
use windows_sys::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

pub(crate) fn reveal(path: &Path) -> Result<(), OpenError> {
    let path = path.to_owned();
    thread::Builder::new()
        .spawn(move || reveal_in_thread(&path).map_err(OpenError::Io))
        .map_err(OpenError::Io)?
        .join()
        .expect("COM worker thread should not panic")
}

fn reveal_in_thread(path: &Path) -> io::Result<()> {
    unsafe {
        to_io_result(CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED as u32))?;
    }

    let item_id_list = {
        // The ILCreateFromPathW function expects a canonicalized path.
        // Unfortunately it does not support NT UNC paths (which std::path::canonicalize returns)
        // so we use the normpath crate instead.
        let path = convert_path(path.normalize()?.as_os_str())?;
        let result = unsafe { ILCreateFromPathW(path.as_ptr()) };
        if result.is_null() {
            return Err(io::Error::last_os_error());
        } else {
            result
        }
    };

    unsafe {
        // Because the cidl argument is zero, SHOpenFolderAndSelectItems opens the singular item
        // in our item id list in its containing folder and selects it.
        let result = to_io_result(SHOpenFolderAndSelectItems(item_id_list, 0, ptr::null(), 0));
        ILFree(item_id_list);
        CoUninitialize();

        result
    }
}

fn to_io_result(hresult: HRESULT) -> io::Result<()> {
    if hresult >= 0 {
        Ok(())
    } else {
        // See the HRESULT_CODE macro in winerror.h
        Err(io::Error::from_raw_os_error(hresult & 0xFFFF))
    }
}

//
//
//

#[repr(C)]
#[repr(packed)]
struct ITEMIDLIST {
    mkid: SHITEMID,
}

#[repr(C)]
#[repr(packed)]
struct SHITEMID {
    cb: u16,
    abID: [u8; 1],
}

#[link(name = "Shell32")]
extern "C" {
    fn ILCreateFromPathW(pszPath: PCWSTR) -> *mut ITEMIDLIST;

    fn ILFree(pidl: *mut ITEMIDLIST);

    fn SHOpenFolderAndSelectItems(
        pidlFolder: *const ITEMIDLIST,
        cidl: u32,
        apidl: *const *const ITEMIDLIST,
        dwFlags: u32,
    ) -> HRESULT;
}
