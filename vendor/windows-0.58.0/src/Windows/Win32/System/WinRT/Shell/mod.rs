windows_core::imp::define_interface!(IDDEInitializer, IDDEInitializer_Vtbl, 0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
impl core::ops::Deref for IDDEInitializer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDDEInitializer, windows_core::IUnknown);
impl IDDEInitializer {
    #[cfg(feature = "Win32_UI_Shell")]
    pub unsafe fn Initialize<P0, P1, P2, P3, P4, P5, P6, P7>(&self, fileextensionorprotocol: P0, method: CreateProcessMethod, currentdirectory: P1, exectarget: P2, site: P3, application: P4, targetfile: P5, arguments: P6, verb: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::UI::Shell::IShellItem>,
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fileextensionorprotocol.param().abi(), method, currentdirectory.param().abi(), exectarget.param().abi(), site.param().abi(), application.param().abi(), targetfile.param().abi(), arguments.param().abi(), verb.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDDEInitializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, CreateProcessMethod, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell"))]
    Initialize: usize,
}
pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CreateProcessMethod(pub i32);
impl windows_core::TypeKind for CreateProcessMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CreateProcessMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CreateProcessMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
