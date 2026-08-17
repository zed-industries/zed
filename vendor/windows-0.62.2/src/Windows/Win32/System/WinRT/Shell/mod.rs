pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CreateProcessMethod(pub i32);
windows_core::imp::define_interface!(IDDEInitializer, IDDEInitializer_Vtbl, 0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
windows_core::imp::interface_hierarchy!(IDDEInitializer, windows_core::IUnknown);
impl IDDEInitializer {
    #[cfg(feature = "Win32_UI_Shell")]
    pub unsafe fn Initialize<P0, P2, P3, P4, P5, P6, P7, P8>(&self, fileextensionorprotocol: P0, method: CreateProcessMethod, currentdirectory: P2, exectarget: P3, site: P4, application: P5, targetfile: P6, arguments: P7, verb: P8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::super::UI::Shell::IShellItem>,
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
        P8: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fileextensionorprotocol.param().abi(), method, currentdirectory.param().abi(), exectarget.param().abi(), site.param().abi(), application.param().abi(), targetfile.param().abi(), arguments.param().abi(), verb.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, CreateProcessMethod, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_UI_Shell")]
pub trait IDDEInitializer_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, fileextensionorprotocol: &windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: &windows_core::PCWSTR, exectarget: windows_core::Ref<super::super::super::UI::Shell::IShellItem>, site: windows_core::Ref<windows_core::IUnknown>, application: &windows_core::PCWSTR, targetfile: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, verb: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell")]
impl IDDEInitializer_Vtbl {
    pub const fn new<Identity: IDDEInitializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDDEInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileextensionorprotocol: windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: windows_core::PCWSTR, exectarget: *mut core::ffi::c_void, site: *mut core::ffi::c_void, application: windows_core::PCWSTR, targetfile: windows_core::PCWSTR, arguments: windows_core::PCWSTR, verb: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDDEInitializer_Impl::Initialize(this, core::mem::transmute(&fileextensionorprotocol), core::mem::transmute_copy(&method), core::mem::transmute(&currentdirectory), core::mem::transmute_copy(&exectarget), core::mem::transmute_copy(&site), core::mem::transmute(&application), core::mem::transmute(&targetfile), core::mem::transmute(&arguments), core::mem::transmute(&verb)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDDEInitializer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell")]
impl windows_core::RuntimeName for IDDEInitializer {}
