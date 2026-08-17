windows_core::imp::define_interface!(IToastOcclusionManagerPreviewStatics, IToastOcclusionManagerPreviewStatics_Vtbl, 0x507e5c83_50f9_5412_8953_b65c18cfab12);
impl windows_core::RuntimeType for IToastOcclusionManagerPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastOcclusionManagerPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetToastWindowMargin: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowId, f64) -> windows_core::HRESULT,
}
pub struct ToastOcclusionManagerPreview;
impl ToastOcclusionManagerPreview {
    pub fn SetToastWindowMargin(appwindowid: super::super::WindowId, margin: f64) -> windows_core::Result<()> {
        Self::IToastOcclusionManagerPreviewStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetToastWindowMargin)(windows_core::Interface::as_raw(this), appwindowid, margin).ok() })
    }
    #[doc(hidden)]
    pub fn IToastOcclusionManagerPreviewStatics<R, F: FnOnce(&IToastOcclusionManagerPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastOcclusionManagerPreview, IToastOcclusionManagerPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ToastOcclusionManagerPreview {
    const NAME: &'static str = "Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview";
}
