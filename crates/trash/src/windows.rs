use std::{
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use scopeguard::defer;
use windows::{
    Win32::{
        Foundation::PROPERTYKEY,
        System::Com::{
            CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
            CoUninitialize, StructuredStorage::PropVariantToBSTR,
        },
        UI::Shell::{
            BHID_EnumItems, FOF_ALLOWUNDO, FOF_NO_UI, FOF_WANTNUKEWARNING, FOFX_EARLYFAILURE,
            FOLDERID_RecycleBinFolder, FileOperation, IEnumShellItems, IFileOperation, IShellItem,
            IShellItem2, KF_FLAG_DEFAULT, PID_DISPLACED_FROM, PSGUID_DISPLACED,
            SHCreateItemFromParsingName, SHGetKnownFolderItem, SIGDN_DESKTOPABSOLUTEPARSING,
        },
    },
    core::{Interface, PCWSTR},
};

use crate::Platform;

const SCID_ORIGINAL_LOCATION: PROPERTYKEY = PROPERTYKEY {
    fmtid: PSGUID_DISPLACED,
    pid: PID_DISPLACED_FROM,
};

pub struct WindowsPlatform;

impl Platform for WindowsPlatform {
    fn trash_file(&self, path: &Path) -> Result<PathBuf> {
        trash_item(path)
    }

    fn trash_dir(&self, path: &Path) -> Result<PathBuf> {
        trash_item(path)
    }

    fn restore_file(&self, path_in_trash: &Path, original_path: &Path) -> Result<()> {
        restore_item(path_in_trash, original_path)
    }

    fn restore_dir(&self, path_in_trash: &Path, original_path: &Path) -> Result<()> {
        restore_item(path_in_trash, original_path)
    }
}

fn trash_item(path: &Path) -> Result<PathBuf> {
    let path = path.canonicalize()?;

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .context("failed to initialize COM")?;
    }

    defer! {
        unsafe { CoUninitialize(); }
    }

    unsafe {
        let pfo: IFileOperation = CoCreateInstance(&FileOperation, None, CLSCTX_ALL)
            .context("failed to create IFileOperation")?;

        pfo.SetOperationFlags(FOF_NO_UI | FOF_ALLOWUNDO | FOF_WANTNUKEWARNING)
            .context("failed to set operation flags")?;

        let wide_path = to_wide_path(path);
        let shell_item: IShellItem = SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)
            .context("failed to create shell item")?;

        pfo.DeleteItem(&shell_item, None)
            .context("failed to queue delete")?;

        pfo.PerformOperations()
            .context("failed to perform delete operation")?;

        if pfo.GetAnyOperationsAborted()?.as_bool() {
            return Err(anyhow!("trash operation was aborted"));
        }

        find_in_recycle_bin(path)
    }
}

fn restore_item(path_in_trash: &Path, original_path: &Path) -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .context("failed to initialize COM")?;
    }

    defer! {
        unsafe { CoUninitialize(); }
    }

    unsafe {
        let pfo: IFileOperation = CoCreateInstance(&FileOperation, None, CLSCTX_ALL)
            .context("failed to create IFileOperation")?;

        pfo.SetOperationFlags(FOF_NO_UI | FOFX_EARLYFAILURE)
            .context("failed to set operation flags")?;

        let wide_trash_path = to_wide_path(path_in_trash);
        let trash_item: IShellItem =
            SHCreateItemFromParsingName(PCWSTR(wide_trash_path.as_ptr()), None)
                .context("failed to create shell item for trash item")?;

        let original_parent = original_path
            .parent()
            .ok_or_else(|| anyhow!("original path has no parent"))?;
        let wide_parent = to_wide_path(original_parent);
        let parent_item: IShellItem =
            SHCreateItemFromParsingName(PCWSTR(wide_parent.as_ptr()), None)
                .context("failed to create shell item for parent")?;

        let file_name = original_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow!("invalid file name"))?;
        let wide_name = to_wide_string(file_name);

        pfo.MoveItem(&trash_item, &parent_item, PCWSTR(wide_name.as_ptr()), None)
            .context("failed to queue move")?;

        pfo.PerformOperations()
            .context("failed to perform restore operation")?;

        Ok(())
    }
}

unsafe fn find_in_recycle_bin(original_path: &Path) -> Result<PathBuf> {
    unsafe {
        let recycle_bin: IShellItem =
            SHGetKnownFolderItem(&FOLDERID_RecycleBinFolder, KF_FLAG_DEFAULT, None)
                .context("failed to get recycle bin")?;

        let enumerator: IEnumShellItems = recycle_bin
            .BindToHandler(None, &BHID_EnumItems)
            .context("failed to enumerate recycle bin")?;

        let original_str = original_path.to_string_lossy();

        loop {
            let mut items: [Option<IShellItem>; 1] = [None];
            let mut fetched: u32 = 0;

            enumerator.Next(&mut items, Some(&mut fetched))?;

            if fetched == 0 {
                break;
            }

            if let Some(item) = items[0].take() {
                if let Ok(location) = get_original_location(&item) {
                    if location
                        .to_string_lossy()
                        .eq_ignore_ascii_case(&original_str)
                    {
                        let display_name = item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING)?;
                        let path = PathBuf::from(wstr_to_os_string(display_name.0));
                        CoTaskMemFree(Some(display_name.0 as *const _));
                        return Ok(path);
                    }
                }
            }
        }

        Err(anyhow!(
            "could not find trashed item in recycle bin for: {:?}",
            original_path
        ))
    }
}

unsafe fn get_original_location(item: &IShellItem) -> Result<PathBuf> {
    unsafe {
        let item2: IShellItem2 = item.cast()?;
        let variant = item2.GetProperty(&SCID_ORIGINAL_LOCATION)?;
        let bstr = PropVariantToBSTR(&variant)?;
        let path = PathBuf::from(OsString::from_wide(bstr.as_ref()));
        Ok(path)
    }
}

unsafe fn wstr_to_os_string(wstr: *mut u16) -> OsString {
    unsafe {
        let mut len = 0;
        while *wstr.offset(len) != 0 {
            len += 1;
        }
        let slice = std::slice::from_raw_parts(wstr, len as usize);
        OsString::from_wide(slice)
    }
}

fn to_wide_path(path: &Path) -> Vec<u16> {
    path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
