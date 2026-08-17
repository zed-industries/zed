windows_core::imp::define_interface!(IWindowsProtectedPrintInfoStatics, IWindowsProtectedPrintInfoStatics_Vtbl, 0xa7d212f3_4168_5485_98ab_d89d04603b40);
impl windows_core::RuntimeType for IWindowsProtectedPrintInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsProtectedPrintInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsProtectedPrintEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct WindowsProtectedPrintInfo;
impl WindowsProtectedPrintInfo {
    pub fn IsProtectedPrintEnabled() -> windows_core::Result<bool> {
        Self::IWindowsProtectedPrintInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsProtectedPrintEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IWindowsProtectedPrintInfoStatics<R, F: FnOnce(&IWindowsProtectedPrintInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsProtectedPrintInfo, IWindowsProtectedPrintInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WindowsProtectedPrintInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.ProtectedPrint.WindowsProtectedPrintInfo";
}
