#![cfg(target_os = "windows")]

use std::path::PathBuf;

use windows::{
    core::implement,
    Win32::{
        Foundation::{
            GetLastError, BOOL, ERROR_INSUFFICIENT_BUFFER, E_FAIL, E_NOTIMPL, HINSTANCE, MAX_PATH,
        },
        Globalization::u_strlen,
        System::{
            Com::IBindCtx, LibraryLoader::GetModuleFileNameW, SystemServices::DLL_PROCESS_ATTACH,
        },
        UI::Shell::{
            IEnumExplorerCommand, IExplorerCommand, IExplorerCommand_Impl, IShellItemArray,
            SHStrDupW, ECF_DEFAULT, ECS_ENABLED, SIGDN_FILESYSPATH,
        },
    },
};
use windows_core::{GUID, HSTRING};

static mut DLL_INSTANCE: HINSTANCE = HINSTANCE(std::ptr::null_mut());

#[no_mangle]
extern "system" fn DllMain(
    hinstdll: HINSTANCE,
    fdwreason: u32,
    _lpvreserved: *mut core::ffi::c_void,
) -> bool {
    if fdwreason == DLL_PROCESS_ATTACH {
        unsafe { DLL_INSTANCE = hinstdll };
    }

    true
}

fn get_zed_path() -> Option<String> {
    let mut buf = vec![0u16; MAX_PATH as usize];
    unsafe { GetModuleFileNameW(DLL_INSTANCE, &mut buf) };

    while unsafe { GetLastError() } == ERROR_INSUFFICIENT_BUFFER {
        buf = vec![0u16; buf.len() * 2];
        unsafe { GetModuleFileNameW(DLL_INSTANCE, &mut buf) };
    }
    let len = unsafe { u_strlen(buf.as_ptr()) };
    let path: PathBuf = String::from_utf16_lossy(&buf[..len as usize]).into();
    Some(
        path.parent()?
            .parent()?
            .join("zed.exe")
            .to_string_lossy()
            .to_string(),
    )
}

#[implement(IExplorerCommand)]
struct ContextMenuHandler;

#[allow(non_snake_case)]
impl IExplorerCommand_Impl for ContextMenuHandler_Impl {
    fn GetTitle(&self, _: Option<&IShellItemArray>) -> windows_core::Result<windows_core::PWSTR> {
        unsafe { SHStrDupW(windows::core::w!("Open with Zed")) }
    }

    fn GetIcon(&self, _: Option<&IShellItemArray>) -> windows_core::Result<windows_core::PWSTR> {
        let Some(zed_exe) = get_zed_path() else {
            return Err(E_FAIL.into());
        };
        unsafe { SHStrDupW(&HSTRING::from(zed_exe)) }
    }

    fn GetToolTip(&self, _: Option<&IShellItemArray>) -> windows_core::Result<windows_core::PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetCanonicalName(&self) -> windows_core::Result<windows_core::GUID> {
        Ok(GUID::zeroed())
    }

    fn GetState(&self, _: Option<&IShellItemArray>, _: BOOL) -> windows_core::Result<u32> {
        Ok(ECS_ENABLED.0 as _)
    }

    fn Invoke(
        &self,
        psiitemarray: Option<&IShellItemArray>,
        _: Option<&IBindCtx>,
    ) -> windows_core::Result<()> {
        let Some(items) = psiitemarray else {
            return Ok(());
        };

        let count = unsafe { items.GetCount()? };
        for idx in 0..count {
            let item = unsafe { items.GetItemAt(idx)? };
            let item_path = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH)? };
            let string = unsafe { item_path.to_string()? };
        }

        Ok(())
    }

    fn GetFlags(&self) -> windows_core::Result<u32> {
        Ok(ECF_DEFAULT.0 as _)
    }

    fn EnumSubCommands(&self) -> windows_core::Result<IEnumExplorerCommand> {
        Err(E_NOTIMPL.into())
    }
}
