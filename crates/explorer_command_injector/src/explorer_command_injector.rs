#![cfg(target_os = "windows")]

use std::{os::windows::ffi::OsStringExt, path::PathBuf};

use windows::{
    Win32::{
        Foundation::{
            CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_INVALIDARG, E_NOTIMPL, ERROR_INSUFFICIENT_BUFFER,
            GetLastError, HINSTANCE, MAX_PATH,
        },
        Globalization::u_strlen,
        System::{
            Com::{IBindCtx, IClassFactory, IClassFactory_Impl},
            LibraryLoader::GetModuleFileNameW,
            SystemServices::DLL_PROCESS_ATTACH,
        },
        UI::Shell::{
            ECF_DEFAULT, ECS_ENABLED, IEnumExplorerCommand, IExplorerCommand,
            IExplorerCommand_Impl, IShellItemArray, SHStrDupW, SIGDN_FILESYSPATH,
        },
    },
    core::{BOOL, GUID, HRESULT, HSTRING, Interface, Ref, Result, implement},
};

static mut DLL_INSTANCE: HINSTANCE = HINSTANCE(std::ptr::null_mut());

#[unsafe(no_mangle)]
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
    fn GetTitle(&self, _: Ref<IShellItemArray>) -> Result<windows_core::PWSTR> {
        let command_description =
            retrieve_command_description().unwrap_or(HSTRING::from("Open with Zed"));
        unsafe { SHStrDupW(&command_description) }
    }

    fn GetIcon(&self, _: Ref<IShellItemArray>) -> Result<windows_core::PWSTR> {
        let Some(zed_exe) = get_zed_exe_path() else {
            return Err(E_FAIL.into());
        };
        unsafe { SHStrDupW(&HSTRING::from(zed_exe)) }
    }

    fn GetToolTip(&self, _: Ref<IShellItemArray>) -> Result<windows_core::PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetCanonicalName(&self) -> Result<windows_core::GUID> {
        Ok(GUID::zeroed())
    }

    fn GetState(&self, _: Ref<IShellItemArray>, _: BOOL) -> Result<u32> {
        Ok(ECS_ENABLED.0 as _)
    }

    fn Invoke(&self, psiitemarray: Ref<IShellItemArray>, _: Ref<IBindCtx>) -> Result<()> {
        let items = psiitemarray.ok()?;
        let Some(zed_exe) = get_zed_exe_path() else {
            return Ok(());
        };

        let count = unsafe { items.GetCount()? };
        for idx in 0..count {
            let item = unsafe { items.GetItemAt(idx)? };
            let item_path = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH)?.to_string()? };
            #[allow(clippy::disallowed_methods, reason = "no async context in sight..")]
            std::process::Command::new(&zed_exe)
                .arg(&item_path)
                .spawn()
                .map_err(|_| E_INVALIDARG)?;
        }

        Ok(())
    }

    fn GetFlags(&self) -> Result<u32> {
        Ok(ECF_DEFAULT.0 as _)
    }

    fn EnumSubCommands(&self) -> Result<IEnumExplorerCommand> {
        Err(E_NOTIMPL.into())
    }
}

#[implement(IClassFactory)]
struct ExplorerCommandInjectorFactory;

impl IClassFactory_Impl for ExplorerCommandInjectorFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Ref<windows_core::IUnknown>,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
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

    fn LockServer(&self, _: BOOL) -> Result<()> {
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

#[unsafe(no_mangle)]
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
    unsafe { GetModuleFileNameW(Some(DLL_INSTANCE.into()), &mut buf) };

    while unsafe { GetLastError() } == ERROR_INSUFFICIENT_BUFFER {
        buf = vec![0u16; buf.len() * 2];
        unsafe { GetModuleFileNameW(Some(DLL_INSTANCE.into()), &mut buf) };
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
    get_zed_install_folder().map(|path| path.join("Zed.exe").to_string_lossy().into_owned())
}

#[inline]
fn retrieve_command_description() -> Result<HSTRING> {
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
