#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
windows_core::imp::define_interface!(IInputActivationListenerPreviewStatics, IInputActivationListenerPreviewStatics_Vtbl, 0xf0551ce5_0de6_5be0_a589_f737201a4582);
impl windows_core::RuntimeType for IInputActivationListenerPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListenerPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateForApplicationWindow: usize,
}
pub struct InputActivationListenerPreview;
impl InputActivationListenerPreview {
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateForApplicationWindow<P0>(window: P0) -> windows_core::Result<super::InputActivationListener>
    where
        P0: windows_core::Param<super::super::WindowManagement::AppWindow>,
    {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForApplicationWindow)(windows_core::Interface::as_raw(this), window.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}
