windows_core::imp::define_interface!(IWindowManagementPreview, IWindowManagementPreview_Vtbl, 0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
impl windows_core::RuntimeType for IWindowManagementPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowManagementPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IWindowManagementPreviewStatics, IWindowManagementPreviewStatics_Vtbl, 0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
impl windows_core::RuntimeType for IWindowManagementPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowManagementPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPreferredMinSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Size) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowManagementPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowManagementPreview, windows_core::IUnknown, windows_core::IInspectable);
impl WindowManagementPreview {
    pub fn SetPreferredMinSize<P0>(window: P0, preferredframeminsize: super::super::super::Foundation::Size) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AppWindow>,
    {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetPreferredMinSize)(windows_core::Interface::as_raw(this), window.param().abi(), preferredframeminsize).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowManagementPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowManagementPreview>();
}
unsafe impl windows_core::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: windows_core::GUID = <IWindowManagementPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
unsafe impl Send for WindowManagementPreview {}
unsafe impl Sync for WindowManagementPreview {}
