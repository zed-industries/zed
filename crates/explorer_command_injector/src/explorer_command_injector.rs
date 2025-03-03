#![cfg(target_os = "windows")]

use std::{os::windows::ffi::OsStringExt, path::PathBuf};

use windows::{
    core::implement,
    Win32::{
        Foundation::{
            GetLastError, BOOL, CLASS_E_CLASSNOTAVAILABLE, ERROR_INSUFFICIENT_BUFFER, E_FAIL,
            E_INVALIDARG, E_NOTIMPL, HINSTANCE, MAX_PATH,
        },
        Globalization::u_strlen,
        System::{
            Com::{IBindCtx, IClassFactory, IClassFactory_Impl},
            LibraryLoader::GetModuleFileNameW,
            SystemServices::DLL_PROCESS_ATTACH,
        },
        UI::Shell::{
            IEnumExplorerCommand, IExplorerCommand, IExplorerCommand_Impl, IShellItemArray,
            SHStrDupW, ECF_DEFAULT, ECS_ENABLED, SIGDN_FILESYSPATH,
        },
    },
};
use windows_core::{Interface, GUID, HRESULT, HSTRING};

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

#[implement(IExplorerCommand)]
struct ExplorerCommandInjector;

#[allow(non_snake_case)]
impl IExplorerCommand_Impl for ExplorerCommandInjector_Impl {
    fn GetTitle(&self, _: Option<&IShellItemArray>) -> windows_core::Result<windows_core::PWSTR> {
        let command_description =
            retrieve_command_description().unwrap_or(HSTRING::from("Open with Zed"));
        unsafe { SHStrDupW(&command_description) }
    }

    fn GetIcon(&self, _: Option<&IShellItemArray>) -> windows_core::Result<windows_core::PWSTR> {
        let Some(zed_exe) = get_zed_exe_path() else {
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
        let Some(zed_exe) = get_zed_exe_path() else {
            return Ok(());
        };

        let count = unsafe { items.GetCount()? };
        for idx in 0..count {
            let item = unsafe { items.GetItemAt(idx)? };
            let item_path = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH)?.to_string()? };
            std::process::Command::new(&zed_exe)
                .arg(&item_path)
                .spawn()
                .map_err(|_| E_INVALIDARG)?;
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

#[implement(IClassFactory)]
struct ExplorerCommandInjectorFactory;

impl IClassFactory_Impl for ExplorerCommandInjectorFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Option<&windows_core::IUnknown>,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            *ppvobject = std::ptr::null_mut();
        }
        if punkouter.is_none() {
            let factory: IExplorerCommand = ExplorerCommandInjector {}.into();
            let ret = unsafe { factory.query(riid, ppvobject).ok() };
            if ret.is_ok() {
                unsafe {
                    *ppvobject = factory.into_raw();
                }
            }
            ret
        } else {
            Err(E_INVALIDARG.into())
        }
    }

    fn LockServer(&self, _: BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}

#[cfg(all(feature = "stable", not(feature = "preview"), not(feature = "nightly")))]
const MODULE_ID: GUID = GUID::from_u128(0x6a1f6b13_3b82_48a1_9e06_7bb0a6d0bffd);
#[cfg(all(feature = "preview", not(feature = "stable"), not(feature = "nightly")))]
const MODULE_ID: GUID = GUID::from_u128(0xaf8e85ea_fb20_4db2_93cf_56513c1ec697);
#[cfg(all(feature = "nightly", not(feature = "stable"), not(feature = "preview")))]
const MODULE_ID: GUID = GUID::from_u128(0x266f2cfe_1653_42af_b55c_fe3590c83871);

// Make cargo clippy happy
#[cfg(all(feature = "nightly", feature = "stable", feature = "preview"))]
const MODULE_ID: GUID = GUID::from_u128(0x685f4d49_6718_4c55_b271_ebb5c6a48d6f);

#[no_mangle]
extern "system" fn DllGetClassObject(
    class_id: *const GUID,
    iid: *const GUID,
    out: *mut *mut std::ffi::c_void,
) -> HRESULT {
    unsafe {
        *out = std::ptr::null_mut();
    }
    let class_id = unsafe { *class_id };
    if class_id == MODULE_ID {
        let instance: IClassFactory = ExplorerCommandInjectorFactory {}.into();
        let ret = unsafe { instance.query(iid, out) };
        if ret.is_ok() {
            unsafe {
                *out = instance.into_raw();
            }
        }
        ret
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}

fn get_zed_install_folder() -> Option<PathBuf> {
    let mut buf = vec![0u16; MAX_PATH as usize];
    unsafe { GetModuleFileNameW(DLL_INSTANCE, &mut buf) };

    while unsafe { GetLastError() } == ERROR_INSUFFICIENT_BUFFER {
        buf = vec![0u16; buf.len() * 2];
        unsafe { GetModuleFileNameW(DLL_INSTANCE, &mut buf) };
    }
    let len = unsafe { u_strlen(buf.as_ptr()) };
    let path: PathBuf = std::ffi::OsString::from_wide(&buf[..len as usize])
        .into_string()
        .ok()?
        .into();
    Some(path.parent()?.parent()?.to_path_buf())
}

#[inline]
fn get_zed_exe_path() -> Option<String> {
    get_zed_install_folder().map(|path| path.join("Zed.exe").to_string_lossy().to_string())
}

#[inline]
fn retrieve_command_description() -> windows_core::Result<HSTRING> {
    #[cfg(all(feature = "stable", not(feature = "preview"), not(feature = "nightly")))]
    const REG_PATH: &str = "Software\\Classes\\ZedEditorContextMenu";
    #[cfg(all(feature = "preview", not(feature = "stable"), not(feature = "nightly")))]
    const REG_PATH: &str = "Software\\Classes\\ZedEditorPreviewContextMenu";
    #[cfg(all(feature = "nightly", not(feature = "stable"), not(feature = "preview")))]
    const REG_PATH: &str = "Software\\Classes\\ZedEditorNightlyContextMenu";

    // Make cargo clippy happy
    #[cfg(all(feature = "nightly", feature = "stable", feature = "preview"))]
    const REG_PATH: &str = "Software\\Classes\\ZedEditorClippyContextMenu";

    let key = windows_registry::CURRENT_USER.open(REG_PATH)?;
    key.get_hstring("Title")
}
