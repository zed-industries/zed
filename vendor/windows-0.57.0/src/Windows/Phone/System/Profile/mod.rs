#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IRetailModeStatics, IRetailModeStatics_Vtbl, 0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IRetailModeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IRetailModeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RetailModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RetailModeEnabled: usize,
}
#[cfg(feature = "deprecated")]
pub struct RetailMode;
#[cfg(feature = "deprecated")]
impl RetailMode {
    #[cfg(feature = "deprecated")]
    pub fn RetailModeEnabled() -> windows_core::Result<bool> {
        Self::IRetailModeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetailModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IRetailModeStatics<R, F: FnOnce(&IRetailModeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RetailMode, IRetailModeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
