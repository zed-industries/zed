#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[cfg(feature = "System_RemoteDesktop_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(IInteractiveSessionStatics, IInteractiveSessionStatics_Vtbl, 0x60884631_dd3a_4576_9c8d_e8027618bdce);
impl windows_core::RuntimeType for IInteractiveSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInteractiveSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRemote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct InteractiveSession;
impl InteractiveSession {
    pub fn IsRemote() -> windows_core::Result<bool> {
        Self::IInteractiveSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRemote)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractiveSessionStatics<R, F: FnOnce(&IInteractiveSessionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractiveSession, IInteractiveSessionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for InteractiveSession {
    const NAME: &'static str = "Windows.System.RemoteDesktop.InteractiveSession";
}
