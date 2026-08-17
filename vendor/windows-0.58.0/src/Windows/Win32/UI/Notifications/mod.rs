windows_core::imp::define_interface!(INotificationActivationCallback, INotificationActivationCallback_Vtbl, 0x53e31837_6600_4a81_9395_75cffe746f94);
impl core::ops::Deref for INotificationActivationCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INotificationActivationCallback, windows_core::IUnknown);
impl INotificationActivationCallback {
    pub unsafe fn Activate<P0, P1>(&self, appusermodelid: P0, invokedargs: P1, data: &[NOTIFICATION_USER_INPUT_DATA]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), appusermodelid.param().abi(), invokedargs.param().abi(), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct INotificationActivationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const NOTIFICATION_USER_INPUT_DATA, u32) -> windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: windows_core::PCWSTR,
    pub Value: windows_core::PCWSTR,
}
impl windows_core::TypeKind for NOTIFICATION_USER_INPUT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NOTIFICATION_USER_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
